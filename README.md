# RollingRequests

RollingRequests is a Rust library designed for handling and executing HTTP requests with concurrency control. It allows you to efficiently manage HTTP requests with a specified limit on simultaneous executions.

>[!NOTE]
> This repository is still a work in progress and not yet released.

## Features

- **Create and Manage Requests**: Easily create HTTP requests with customizable parameters.
- **Concurrent Execution**: Execute multiple requests concurrently with a configurable limit.
- **Flexible Request Management**: Add, execute, and clear requests seamlessly.

## Usage

```rust
use rollingrequests::rolling::RollingRequestsBuilder;
use rollingrequests::request::Request;
use reqwest::Method;
use tokio;

#[tokio::main]
async fn main() {
    let mut rolling_requests = RollingRequestsBuilder::new()
        .simultaneous_limit(2)
        .build();

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
