#![warn(missing_docs)]

//! Datadog API client implementation.
//!
//! Example:
//! ```
//! let client = Client::new().set_api_key()
//!     api_key: Some("46f8ef1aa69a9cf1a1847e6857e94bab".to_string()),
//!     site: None,
//!     application_key: None,
//! });
//!
//!
//!
//!

/// This module implements the endpoints for the API.
mod api;

/// Implements the base client to build and interact with Datadog's API.
pub mod client;

/// Custom error implementation. Based on [thiserror] crate.
pub mod error;

/// Models most of the structures.
pub mod models;
