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
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
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

/// Configuration for `RollingRequests`.
pub struct RollingRequestsConfig {
    pub simultaneous_limit: usize,
    pub timeout: Duration,
    pub force_http2: bool,
}

impl Default for RollingRequestsConfig {
    fn default() -> Self {
        RollingRequestsConfig {
            simultaneous_limit: 1,            // Default limit
            timeout: Duration::from_secs(30), // Default timeout
            force_http2: false,               // Default false
        }
    }
}

/// Builder for `RollingRequests`.
pub struct RollingRequestsBuilder {
    config: RollingRequestsConfig,
}

impl RollingRequestsBuilder {
    /// Creates a new builder with default configuration.
    ///
    /// #### Examples
    ///
    /// ```
    /// use rollingrequests::rolling::RollingRequestsBuilder;
    ///
    /// let builder = RollingRequestsBuilder::new();
    /// ```
    pub fn new() -> Self {
        RollingRequestsBuilder {
            config: RollingRequestsConfig::default(),
        }
    }

    /// Sets the simultaneous request limit.
    ///
    /// #### Arguments
    ///
    /// * `limit` - The maximum number of requests to execute simultaneously.
    ///
    /// #### Examples
    ///
    /// ```
    /// use rollingrequests::rolling::RollingRequestsBuilder;
    ///
    /// let builder = RollingRequestsBuilder::new().simultaneous_limit(5);
    /// ```
    pub fn simultaneous_limit(mut self, limit: usize) -> Self {
        self.config.simultaneous_limit = limit;
        self
    }

    /// Sets the request timeout duration.
    ///
    /// #### Arguments
    ///
    /// * `timeout` - The duration to wait before a request times out.
    ///
    /// #### Examples
    ///
    /// ```
    /// use rollingrequests::rolling::RollingRequestsBuilder;
    /// use std::time::Duration;
    ///
    /// let builder = RollingRequestsBuilder::new().timeout(Duration::from_secs(10));
    /// ```
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Forces the use of HTTP/2 for requests.
    ///
    /// #### Arguments
    ///
    /// * `force` - A boolean indicating whether to force HTTP/2.
    ///
    /// #### Examples
    ///
    /// ```
    /// use rollingrequests::rolling::RollingRequestsBuilder;
    ///
    /// let builder = RollingRequestsBuilder::new().force_http2(true);
    /// ```
    pub fn force_http2(mut self, force: bool) -> Self {
        self.config.force_http2 = force;
        self
    }

    /// Builds the `RollingRequests` instance.
    ///
    /// #### Examples
    ///
    /// ```
    /// use rollingrequests::rolling::RollingRequestsBuilder;
    ///
    /// let rolling_requests = RollingRequestsBuilder::new().build();
    /// ```
    pub fn build(self) -> RollingRequests {
        RollingRequests::new(self.config)
    }
}

impl RollingRequests {
    /// Creates a new `RollingRequests` instance with the specified configuration.
    ///
    /// #### Arguments
    ///
    /// * `config` - The configuration for the requests.
    ///
    /// #### Examples
    ///
    /// ```
    /// use rollingrequests::rolling::RollingRequestsBuilder;
    /// use std::time::Duration;
    ///
    /// let rolling_requests = RollingRequestsBuilder::new()
    ///     .simultaneous_limit(5)
    ///     .timeout(Duration::from_secs(10))
    ///     .build();
    /// ```
    pub fn new(config: RollingRequestsConfig) -> Self {
        let client_builder = Client::builder().timeout(config.timeout);

        let client = if config.force_http2 {
            client_builder.http2_prior_knowledge().build().unwrap()
        } else {
            client_builder.build().unwrap()
        };

        RollingRequests {
            simultaneous_limit: config.simultaneous_limit,
            pending_requests: Arc::new(Mutex::new(Vec::new())),
            client,
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
    /// use rollingrequests::rolling::RollingRequestsBuilder;
    /// use rollingrequests::request::Request;
    /// use reqwest::Method;
    /// use std::time::Duration;
    ///
    /// let mut rolling_requests = RollingRequestsBuilder::new().build();
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
    /// use rollingrequests::rolling::RollingRequestsBuilder;
    /// use reqwest::Method;
    /// use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut rolling_requests = RollingRequestsBuilder::new()
    ///         .simultaneous_limit(2)
    ///         .build();
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

        for req in &requests_to_process {
            let client = self.client.clone();
            let req = req.clone();

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

        // Automatically clear processed requests
        let count = requests_to_process.len();
        let mut pending = self.pending_requests.lock().unwrap();
        pending.drain(0..count);

        responses
    }
}
