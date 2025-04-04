use reqwest::Method;
use reqwest::multipart::{Form, Part};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

impl Clone for Request {
    /// Creates a clone of the `Request` instance.
    ///
    /// Note: The `multipart_form_data` field is not cloned.
    fn clone(&self) -> Self {
        Request {
            url: self.url.clone(),
            method: self.method.clone(),
            post_data: self.post_data.clone(),
            headers: self.headers.clone(),
            options: self.options.clone(),
            extra_info: self.extra_info.clone(),
            response_text: self.response_text.clone(),
            response_info: self.response_info.clone(),
            response_error: self.response_error.clone(),
            response_errno: self.response_errno,
            multipart_form_data: None, // Multipart data is not cloned
        }
    }
}

/// Represents an HTTP request with customizable parameters.
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
    /// Optional multipart form data.
    pub multipart_form_data: Option<Form>,
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
            multipart_form_data: None,
        }
    }

    /// Sets extra information for the request.
    ///
    /// #### Arguments
    ///
    /// * `extra_info` - Additional information to associate with the request.
    pub fn set_extra_info(&mut self, extra_info: &str) -> &mut Self {
        self.extra_info = Some(extra_info.to_string());
        self
    }

    /// Retrieves extra information for the request.
    pub fn get_extra_info(&self) -> Option<&String> {
        self.extra_info.as_ref()
    }

    /// Sets HTTP headers for the request.
    ///
    /// #### Arguments
    ///
    /// * `headers` - A map of header names and values.
    pub fn set_headers(&mut self, headers: HashMap<String, String>) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    /// Retrieves the HTTP headers for the request.
    pub fn get_headers(&self) -> Option<&HashMap<String, String>> {
        self.headers.as_ref()
    }

    /// Sets the HTTP method for the request.
    ///
    /// #### Arguments
    ///
    /// * `method` - The HTTP method to set.
    pub fn set_method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }

    /// Retrieves the HTTP method for the request.
    pub fn get_method(&self) -> &Method {
        &self.method
    }

    /// Sets additional options for the request.
    ///
    /// #### Arguments
    ///
    /// * `options` - A map of option names and values.
    pub fn set_options(&mut self, options: HashMap<String, String>) -> &mut Self {
        self.options = options;
        self
    }

    /// Adds additional options to the existing ones.
    ///
    /// #### Arguments
    ///
    /// * `options` - A map of option names and values to add.
    pub fn add_options(&mut self, options: HashMap<String, String>) -> &mut Self {
        self.options.extend(options);
        self
    }

    /// Retrieves the options for the request.
    pub fn get_options(&self) -> &HashMap<String, String> {
        &self.options
    }

    /// Sets the POST data for the request.
    ///
    /// #### Arguments
    ///
    /// * `post_data` - The data to include in the POST request body.
    pub fn set_post_data(&mut self, post_data: Option<&str>) -> &mut Self {
        self.post_data = post_data.map(|s| s.to_string());
        self
    }

    /// Retrieves the POST data for the request.
    pub fn get_post_data(&self) -> Option<&String> {
        self.post_data.as_ref()
    }

    /// Sets the error number from the response.
    ///
    /// #### Arguments
    ///
    /// * `response_errno` - The error number to set.
    pub fn set_response_errno(&mut self, response_errno: i32) -> &mut Self {
        self.response_errno = Some(response_errno);
        self
    }

    /// Retrieves the error number from the response.
    pub fn get_response_errno(&self) -> Option<i32> {
        self.response_errno
    }

    /// Sets the error message from the response.
    ///
    /// #### Arguments
    ///
    /// * `response_error` - The error message to set.
    pub fn set_response_error(&mut self, response_error: &str) -> &mut Self {
        self.response_error = Some(response_error.to_string());
        self
    }

    /// Retrieves the error message from the response.
    pub fn get_response_error(&self) -> Option<&String> {
        self.response_error.as_ref()
    }

    /// Sets additional response information.
    ///
    /// #### Arguments
    ///
    /// * `response_info` - Additional response information to set.
    pub fn set_response_info(&mut self, response_info: &str) -> &mut Self {
        self.response_info = Some(response_info.to_string());
        self
    }

    /// Retrieves additional response information.
    pub fn get_response_info(&self) -> Option<&String> {
        self.response_info.as_ref()
    }

    /// Sets the response text from the server.
    ///
    /// #### Arguments
    ///
    /// * `response_text` - The response text to set.
    pub fn set_response_text(&mut self, response_text: &str) -> &mut Self {
        self.response_text = Some(response_text.to_string());
        self
    }

    /// Retrieves the response text from the server.
    pub fn get_response_text(&self) -> Option<&String> {
        self.response_text.as_ref()
    }

    /// Sets the URL for the request.
    ///
    /// #### Arguments
    ///
    /// * `url` - The URL to set for the request.
    pub fn set_url(&mut self, url: &str) -> &mut Self {
        self.url = url.to_string();
        self
    }

    /// Retrieves the URL for the request.
    pub fn get_url(&self) -> &String {
        &self.url
    }

    /// Adds a text field to the multipart form data.
    ///
    /// #### Arguments
    ///
    /// * `name` - The name of the form field.
    /// * `value` - The value of the form field.
    pub fn add_form_text(&mut self, name: &str, value: &str) -> &mut Self {
        let mut form = self.multipart_form_data.take().unwrap_or_else(Form::new);
        form = form.text(name.to_string(), value.to_string());
        self.multipart_form_data = Some(form);
        self
    }

    /// Adds a file to the multipart form data.
    ///
    /// #### Arguments
    ///
    /// * `name` - The name of the form field.
    /// * `file_path` - The path to the file to add.
    pub fn add_form_file(&mut self, name: &str, file_path: &Path) -> &mut Self {
        let mut form = self.multipart_form_data.take().unwrap_or_else(Form::new);
        let file_content = fs::read(file_path).expect("Failed to read file");
        let file_part = Part::bytes(file_content).file_name(
            file_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
        );
        form = form.part(name.to_string(), file_part);
        self.multipart_form_data = Some(form);
        self
    }

    /// Sets the multipart form data for the request.
    ///
    /// #### Arguments
    ///
    /// * `form_data` - The multipart form data to set.
    pub fn set_multipart_form_data(&mut self, form_data: Form) -> &mut Self {
        self.multipart_form_data = Some(form_data);
        self
    }
}
