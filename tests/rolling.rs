#[cfg(test)]
mod tests {
    use mockito::mock;
    use reqwest::Method;
    use rollingrequests::{request::Request, rolling::RollingRequests};
    use tokio;

    #[tokio::test]
    async fn test_rolling_requests_batch_execution() {
        let _m1 = mock("GET", "/get")
            .with_status(200)
            .with_body(r#"{"url": "http://mockito.org/get"}"#)
            .create();

        let mut rolling_requests = RollingRequests::new(2);

        let url = &mockito::server_url();

        // Add 5 requests to simulate a batch process
        for _ in 0..5 {
            let request = Request::new(&format!("{}/get", url), Method::GET);
            rolling_requests.add_request(request);
        }

        // Execute requests in batches of 2
        let mut total_responses = 0;
        while total_responses < 5 {
            let responses = rolling_requests.execute_requests().await;
            let responses_len = responses.len(); // Store the length before moving

            assert!(responses_len <= 2);

            for response in responses {
                if let Ok(resp) = response {
                    let text = resp.text().await.unwrap();
                    assert!(text.contains("\"url\": \"http://mockito.org/get\""));
                    total_responses += 1;
                }
            }

            // Clear the processed requests
            rolling_requests.clear_processed_requests(responses_len);
        }

        assert_eq!(total_responses, 5);
    }

    #[tokio::test]
    async fn test_rolling_requests_add_and_execute() {
        let _m1 = mock("GET", "/get")
            .with_status(200)
            .with_body(r#"{"url": "http://mockito.org/get"}"#)
            .create();

        let mut rolling_requests = RollingRequests::new(2);

        let url = &mockito::server_url();
        let request1 = Request::new(&format!("{}/get", url), Method::GET);
        let request2 = Request::new(&format!("{}/get", url), Method::GET);

        rolling_requests.add_request(request1);
        rolling_requests.add_request(request2);

        let responses = rolling_requests.execute_requests().await;
        assert_eq!(responses.len(), 2);

        for response in responses {
            assert!(response.is_ok());
            let text = response.unwrap().text().await.unwrap();
            assert!(text.contains("\"url\": \"http://mockito.org/get\""));
        }
    }

    #[tokio::test]
    async fn test_rolling_requests_with_post_data() {
        let _m1 = mock("POST", "/post")
            .with_status(200)
            .match_body(r#"{"key": "value"}"#)
            .with_body(r#"{"status": "success"}"#)
            .create();

        let mut rolling_requests = RollingRequests::new(1);

        let url = &mockito::server_url();
        let mut request = Request::new(&format!("{}/post", url), Method::POST);
        request.set_post_data(Some(r#"{"key": "value"}"#));

        rolling_requests.add_request(request);

        let responses = rolling_requests.execute_requests().await;
        assert_eq!(responses.len(), 1);

        for response in responses {
            assert!(response.is_ok());
            let text = response.unwrap().text().await.unwrap();
            assert!(text.contains("\"status\": \"success\""));
        }
    }

    #[tokio::test]
    async fn test_rolling_requests_batch_post_execution() {
        let _m1 = mock("POST", "/post")
            .with_status(200)
            .match_body(r#"{"key": "value"}"#)
            .with_body(r#"{"status": "success"}"#)
            .create();

        let mut rolling_requests = RollingRequests::new(2);

        let url = &mockito::server_url();

        // Add 5 POST requests to simulate a batch process
        for _ in 0..5 {
            let mut request = Request::new(&format!("{}/post", url), Method::POST);
            request.set_post_data(Some(r#"{"key": "value"}"#));
            rolling_requests.add_request(request);
        }

        // Execute requests in batches of 2
        let mut total_responses = 0;
        while total_responses < 5 {
            let responses = rolling_requests.execute_requests().await;
            let responses_len = responses.len(); // Store the length before moving

            assert!(responses_len <= 2);

            for response in responses {
                if let Ok(resp) = response {
                    let text = resp.text().await.unwrap();
                    assert!(text.contains("\"status\": \"success\""));
                    total_responses += 1;
                }
            }

            // Clear the processed requests
            rolling_requests.clear_processed_requests(responses_len);
        }

        assert_eq!(total_responses, 5);
    }
}
