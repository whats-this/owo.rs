//! # owo.rs
//!
//! A Rust client for the owo.whats-th.is API, offering implementations for both
//! asynchronous hyper (v0.11) and synchronous reqwest (v0.7).
//!
//! ### Compile Features
//!
//! - **hyper-support**: Compiles with `hyper` support
//! - **reqwest-support**: Compiles with `reqwest` support (*default*)

//! **note**: `hyper` support is minimal due to lack of existing ecosystem
//! multipart support, and is currently restricted to URL shortening
//!
//! ### Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! owo = "~0.2"
//! ```
//!
//! To enable both `hyper` and `reqwest` support:
//!
//! ```toml
//! [dependencies.owo]
//! features = ["hyper", "reqwest"]
//! version = "~0.2"
//! ```
//!
//! To enable `reqwest` but not `hyper` support:
//!
//! ```toml
//! [dependencies.owo]
//! default-features = false
//! features = ["reqwest"]
//! version = "~0.2"
//! ```
//!
//! ### Examples
//!
//! Using reqwest, upload a file by its filepath as a string taken from user
//! input, with a key taken from an environment variable:
//!
//! ```rust,no_run
//! extern crate owo;
//! # #[cfg(feature = "reqwest")]
//! extern crate reqwest;
//!
//! # #[cfg(feature = "reqwest")]
//! # fn main() {
//! #
//! use owo::OwoReqwestRequester;
//! use reqwest::Client;
//! use std::fs::File;
//! use std::io::{self, Read, Write};
//! use std::env;
//!
//! // Read a filepath to upload from user input:
//! let mut path = String::new();
//! print!("Enter the path on the filesystem to upload:\n>");
//! let _ = io::stdout().flush();
//! let _ = io::stdin().read_line(&mut path);
//!
//! // Read the file to a buffer of bytes
//! let mut buffer = vec![];
//! let mut file = File::open(path).expect("Error opening file");
//! file.read_to_end(&mut buffer).expect("Error reading file");
//!
//! // Retrieve the key from an environment variable named "OWO_KEY".
//! let key = env::var("OWO_KEY").expect("OWO_KEY env missing");
//!
//! // Create the reqwest Client.
//! //
//! // In this case, we're using the client for a oneshot request, but in
//! // normal cases a client can be re-used.
//! let client = Client::new();
//!
//! let response = client.upload_file(&key, buffer).expect("Err in request");
//!
//! println!("Response: {:?}", response);
//! # }
//! #
//! # #[cfg(not(feature = "reqwest"))]
//! # fn main() { }
//! ```
//!
//! For more examples, refer to the [examples] folder or the [documentation].
//!
//! ### License
//!
//! ISC.
//!
//! [documentation]: https://docs.rs/owo
//! [examples]: https://github.com/whats-this/owo.rs/blob/master/examples
#![deny(missing_docs)]

#[cfg(feature = "hyper")]
extern crate hyper;
#[cfg(feature = "hyper-tls")]
extern crate hyper_tls;
#[cfg(feature = "multipart")]
extern crate multipart;
#[cfg(feature = "multipart-async")]
extern crate multipart_async;
#[cfg(feature = "reqwest")]
extern crate reqwest;
#[cfg(feature = "serde")]
extern crate serde;
#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "serde_json")]
extern crate serde_json;

pub mod bridge;
pub mod constants;

#[cfg(feature = "serde_derive")]
pub mod model;

mod error;

pub use error::{Error, Result};

#[cfg(feature = "hyper")]
pub use bridge::hyper::OwoRequester as OwoHyperRequester;
#[cfg(feature = "reqwest")]
pub use bridge::reqwest::OwoRequester as OwoReqwestRequester;
