//! A library for managing and executing multiple HTTP requests concurrently.
//!
//! This module provides the `RollingRequests` struct, which allows you to manage
//! a collection of HTTP requests and execute them with a limit on the number
//! of simultaneous requests.

use crate::request::Request;
use reqwest::{
    Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use std::sync::{Arc, Mutex};
use tokio::task;

/// A struct to manage and execute HTTP requests with a concurrency limit.
pub struct RollingRequests {
    /// The maximum number of requests to execute simultaneously.
    simultaneous_limit: usize,
    /// A thread-safe collection of pending requests.
    pending_requests: Arc<Mutex<Vec<Request>>>,
    /// The HTTP client used to send requests.
    client: Client,
}

impl RollingRequests {
    /// Creates a new `RollingRequests` instance with a specified concurrency limit.
    ///
    /// #### Arguments
    ///
    /// * `simultaneous_limit` - The maximum number of requests to execute at the same time.
    ///
    /// #### Examples
    ///
    /// ```
    /// use rollingrequests::rolling::RollingRequests;
    ///
    /// let rolling_requests = RollingRequests::new(5);
    /// ```
    pub fn new(simultaneous_limit: usize) -> Self {
        RollingRequests {
            simultaneous_limit,
            pending_requests: Arc::new(Mutex::new(Vec::new())),
            client: Client::new(),
        }
    }

    /// Adds a new request to the collection of pending requests.
    ///
    /// #### Arguments
    ///
    /// * `request` - The `Request` to add.
    ///
    /// #### Examples
    ///
    /// ```
    /// use rollingrequests::rolling::RollingRequests;
    /// use rollingrequests::request::Request;
    /// use reqwest::Method;
    ///
    /// let mut rolling_requests = RollingRequests::new(5);
    /// let request = Request::new("http://example.com", Method::GET);
    /// rolling_requests.add_request(request);
    /// ```
    pub fn add_request(&mut self, request: Request) {
        let mut pending = self.pending_requests.lock().unwrap();
        pending.push(request);
    }

    /// Executes the pending requests up to the concurrency limit.
    ///
    /// Returns a vector of results for each request, either a successful response
    /// or an error.
    ///
    /// #### Examples
    ///
    /// ```
    /// use rollingrequests::request::Request;
    /// use rollingrequests::rolling::RollingRequests;
    /// use reqwest::Method;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut rolling_requests = RollingRequests::new(2);
    ///
    ///     let url = "http://example.com";
    ///
    ///     // Add requests to the queue
    ///     for _ in 0..5 {
    ///         let request = Request::new(url, Method::GET);
    ///         rolling_requests.add_request(request);
    ///     }
    ///
    ///     // Execute requests
    ///     let responses = rolling_requests.execute_requests().await;
    ///     for response in responses {
    ///         match response {
    ///             Ok(res) => println!("Response: {:?}", res),
    ///             Err(err) => eprintln!("Error: {:?}", err),
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn execute_requests(&self) -> Vec<Result<reqwest::Response, reqwest::Error>> {
        let mut handles = vec![];
        let mut responses = vec![];

        let requests_to_process: Vec<Request> = {
            let pending = self.pending_requests.lock().unwrap();
            pending
                .iter()
                .take(self.simultaneous_limit)
                .cloned()
                .collect()
        };

        for req in requests_to_process {
            let client = self.client.clone();

            let handle = task::spawn(async move {
                let mut req_builder = client.request(req.method.clone(), &req.url);

                if let Some(headers) = &req.headers {
                    let mut header_map = HeaderMap::new();
                    for (key, value) in headers {
                        if let (Ok(header_name), Ok(header_value)) = (
                            HeaderName::from_bytes(key.as_bytes()),
                            HeaderValue::from_str(value),
                        ) {
                            header_map.insert(header_name, header_value);
                        }
                    }
                    req_builder = req_builder.headers(header_map);
                }

                if let Some(data) = &req.post_data {
                    req_builder = req_builder.body(data.clone());
                }

                req_builder.send().await
            });

            handles.push(handle);
        }

        for handle in handles {
            match handle.await {
                Ok(response) => responses.push(response),
                Err(e) => eprintln!("Task failed: {:?}", e),
            }
        }

        responses
    }

    /// Removes a specified number of processed requests from the pending list.
    ///
    /// This method is used to clear requests that have already been executed,
    /// allowing new requests to be added and executed in subsequent batches.
    ///
    /// #### Arguments
    ///
    /// * `count` - The number of requests to remove from the start of the pending list.
    ///
    /// #### Examples
    ///
    /// ```
    /// use rollingrequests::request::Request;
    /// use rollingrequests::rolling::RollingRequests;
    /// use reqwest::Method;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut rolling_requests = RollingRequests::new(2);
    ///
    ///     let url = "http://example.com";
    ///
    ///     // Add requests to the queue
    ///     for _ in 0..5 {
    ///         let request = Request::new(url, Method::GET);
    ///         rolling_requests.add_request(request);
    ///     }
    ///
    ///     // Execute requests
    ///     let responses = rolling_requests.execute_requests().await;
    ///
    ///     // Clear processed requests
    ///     rolling_requests.clear_processed_requests(responses.len());
    /// }
    /// ```
    ///
    /// This ensures that the pending list only contains requests that have not yet been processed,
    /// maintaining an accurate queue for execution.
    pub fn clear_processed_requests(&self, count: usize) {
        let mut pending = self.pending_requests.lock().unwrap();
        pending.drain(0..count);
    }
}
