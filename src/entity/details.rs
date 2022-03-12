//!
//! Entities returned by the details API
//!

use crate::{entity::Song, BASE_URL};
use mime::{FromStrError, Mime};
use serde::{Deserialize, Serialize};

/// Data field of the details response
#[derive(Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Data {
    /// Title of the video
    pub title: String,

    /// Video's filename without an extension
    pub file: String,

    /// List of possible encodings
    ///
    /// (This should ideally use the `mime` crate but the crate is unmaintained and doesn't have published serde support)
    pub mime: Vec<String>,

    /// Song of the opening
    pub song: Song,

    /// Name of the subtitle group
    pub subtitles: Option<String>,

    /// Path to get the video file(s)
    pub path: String,

    /// Whether the video is hidden
    #[serde(default)]
    pub hidden: bool,
}

/// Response of the details endpoint
#[derive(Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Details {
    /// Actual data about the opening
    pub data: Data,

    /// Next data
    pub next: Option<Next>,

    /// Comment or error message
    pub comment: Option<String>,
}

impl Details {
    /// Return a list of all possible URLs to stream this opening
    ///
    /// # Errors
    ///
    /// Check [mime's `FromStrError`](::mime::FromStrError) for details
    pub fn videos(&self) -> Result<Vec<String>, FromStrError> {
        let mut videos = Vec::new();
        for mime in &self.data.mime {
            let mime: Mime = mime.parse()?;
            let url = format!(
                "{BASE_URL}/{}/{}.{}",
                self.data.path,
                self.data.file,
                mime.subtype()
            );
            videos.push(url);
        }

        Ok(videos)
    }
}

/// Next field of the details response
#[derive(Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Next {
    /// First seed for the RNG
    pub seed: u64,

    /// Second seed for the RNG
    pub seed_b: u64,

    /// Whether strict mode was used
    pub strict: bool,
}
