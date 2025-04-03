# RollingRequests

RollingRequests is a Rust library for handling and executing HTTP requests with concurrency control. It allows you to create and manage HTTP requests efficiently, with a specified limit on simultaneous requests.

>[!NOTE]
> This repository is still a work in progress and not yet released.

## Features

- Create and manage individual HTTP requests.
- Execute multiple requests concurrently with a specified limit.
- Easily add, execute, and clear requests.

## Usage

```rust
use rollingrequests::rolling::RollingRequests;
use rollingrequests::request::Request;
use reqwest::Method;
use tokio;

#[tokio::main]
async fn main() {
    let mut rolling_requests = RollingRequests::new(2);

    let url = "http://example.com";

    // Add requests to the queue
    for _ in 0..5 {
        let request = Request::new(url, Method::GET);
        rolling_requests.add_request(request);
    }

    // Execute requests
    let responses = rolling_requests.execute_requests().await;
    for response in responses {
        match response {
            Ok(res) => println!("Response: {:?}", res),
            Err(err) => eprintln!("Error: {:?}", err),
        }
    }
}
```

## License

This project is licensed under the BSD 3-Clause License. See the [LICENSE](LICENSE) file for details.
