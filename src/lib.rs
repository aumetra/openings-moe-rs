//!
//! [openings.moe](https://openings.moe) API library
//!
//! # Examples
//!
//! See the `examples/` directory in the Git repository
//!

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

use self::entity::List;
use const_format::concatcp;
use once_cell::sync::Lazy;
use reqwest::{Client, Result};

/// General ase URL
const BASE_URL: &str = "https://openings.moe";

/// Base API URL
const BASE_API_URL: &str = concatcp!(BASE_URL, "/api");

/// Details endpoint
const DETAILS_URL: &str = concatcp!(BASE_API_URL, "/details.php");

/// List endpoint
const LIST_URL: &str = concatcp!(BASE_API_URL, "/list.php");

/// Globally shared reqwest client
static CLIENT: Lazy<Client> = Lazy::new(|| {
    let client = Client::builder();

    // The `.user_agent()` function is not supported under WASM
    #[cfg(not(target_arch = "wasm32"))]
    let client = client.user_agent(concat!("openings-moe-rs", "/", env!("CARGO_PKG_VERSION")));

    client.build().expect("Failed to build client")
});

/// Fetch a list of all openings known to `openings.moe`
///
/// # Errors
///
/// Refer to [reqwest's errors](::reqwest::Error)
pub async fn all() -> Result<List> {
    let request = CLIENT.get(LIST_URL).build()?;

    CLIENT.execute(request).await?.json().await
}

pub mod entity;
