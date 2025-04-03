use reqwest::Method;
use std::collections::HashMap;

/// Represents an HTTP request with customizable parameters.
#[derive(Clone)]
pub struct Request {
    /// The URL of the request.
    pub url: String,
    /// The HTTP method (e.g., GET, POST).
    pub method: Method,
    /// Optional data for POST requests.
    pub post_data: Option<String>,
    /// Optional HTTP headers.
    pub headers: Option<HashMap<String, String>>,
    /// Additional options for the request.
    pub options: HashMap<String, String>,
    /// Extra information for custom use.
    pub extra_info: Option<String>,
    /// The response text from the server.
    pub response_text: Option<String>,
    /// Additional response information.
    pub response_info: Option<String>,
    /// Any error message from the response.
    pub response_error: Option<String>,
    /// Error number from the response.
    pub response_errno: Option<i32>,
}

impl Request {
    /// Creates a new `Request` with the specified URL and method.
    ///
    /// #### Arguments
    ///
    /// * `url` - The URL for the request.
    /// * `method` - The HTTP method to use.
    ///
    /// #### Examples
    ///
    /// ```
    /// use rollingrequests::request::Request;
    /// use reqwest::Method;
    /// 
    /// let request = Request::new("http://example.com", Method::GET);
    /// ```
    pub fn new(url: &str, method: Method) -> Self {
        Request {
            url: url.to_string(),
            method,
            post_data: None,
            headers: None,
            options: HashMap::new(),
            extra_info: None,
            response_text: None,
            response_info: None,
            response_error: None,
            response_errno: None,
        }
    }

    /// Sets extra information for the request.
    pub fn set_extra_info(&mut self, extra_info: &str) -> &mut Self {
        self.extra_info = Some(extra_info.to_string());
        self
    }

    /// Retrieves extra information for the request.
    pub fn get_extra_info(&self) -> Option<&String> {
        self.extra_info.as_ref()
    }

    /// Sets HTTP headers for the request.
    pub fn set_headers(&mut self, headers: HashMap<String, String>) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    /// Retrieves the HTTP headers for the request.
    pub fn get_headers(&self) -> Option<&HashMap<String, String>> {
        self.headers.as_ref()
    }

    /// Sets the HTTP method for the request.
    pub fn set_method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }

    /// Retrieves the HTTP method for the request.
    pub fn get_method(&self) -> &Method {
        &self.method
    }

    /// Sets additional options for the request.
    pub fn set_options(&mut self, options: HashMap<String, String>) -> &mut Self {
        self.options = options;
        self
    }

    /// Adds additional options to the existing ones.
    pub fn add_options(&mut self, options: HashMap<String, String>) -> &mut Self {
        self.options.extend(options);
        self
    }

    /// Retrieves the options for the request.
    pub fn get_options(&self) -> &HashMap<String, String> {
        &self.options
    }

    /// Sets the POST data for the request.
    pub fn set_post_data(&mut self, post_data: &str) -> &mut Self {
        self.post_data = Some(post_data.to_string());
        self
    }

    /// Retrieves the POST data for the request.
    pub fn get_post_data(&self) -> Option<&String> {
        self.post_data.as_ref()
    }

    /// Sets the error number from the response.
    pub fn set_response_errno(&mut self, response_errno: i32) -> &mut Self {
        self.response_errno = Some(response_errno);
        self
    }

    /// Retrieves the error number from the response.
    pub fn get_response_errno(&self) -> Option<i32> {
        self.response_errno
    }

    /// Sets the error message from the response.
    pub fn set_response_error(&mut self, response_error: &str) -> &mut Self {
        self.response_error = Some(response_error.to_string());
        self
    }

    /// Retrieves the error message from the response.
    pub fn get_response_error(&self) -> Option<&String> {
        self.response_error.as_ref()
    }

    /// Sets additional response information.
    pub fn set_response_info(&mut self, response_info: &str) -> &mut Self {
        self.response_info = Some(response_info.to_string());
        self
    }

    /// Retrieves additional response information.
    pub fn get_response_info(&self) -> Option<&String> {
        self.response_info.as_ref()
    }

    /// Sets the response text from the server.
    pub fn set_response_text(&mut self, response_text: &str) -> &mut Self {
        self.response_text = Some(response_text.to_string());
        self
    }

    /// Retrieves the response text from the server.
    pub fn get_response_text(&self) -> Option<&String> {
        self.response_text.as_ref()
    }

    /// Sets the URL for the request.
    pub fn set_url(&mut self, url: &str) -> &mut Self {
        self.url = url.to_string();
        self
    }

    /// Retrieves the URL for the request.
    pub fn get_url(&self) -> &String {
        &self.url
    }
}
