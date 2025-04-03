//! A library for handling and executing HTTP requests with concurrency control.
//!
//! This crate provides modules to create and manage HTTP requests efficiently,
//! allowing for concurrent execution with a specified limit on simultaneous requests.
//!
//! #### Modules
//!
//! - `request`: Defines the `Request` struct and its associated methods for creating
//!   and managing individual HTTP requests.
//! - `rolling`: Provides the `RollingRequests` struct for managing and executing
//!   multiple requests concurrently.

pub mod request;
pub mod rolling;
