//! A module for creating and managing HTTP requests.
//!
//! This module provides the `Request` struct, which allows you to define HTTP requests
//! with various parameters such as URL, method, headers, and body data. It also provides
//! methods to set and retrieve additional information related to the request and response.

mod request;

pub use request::Request;
