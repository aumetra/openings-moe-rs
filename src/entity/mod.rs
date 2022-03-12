//!
//! Entities returned by the API
//!

use serde::{Deserialize, Serialize};

pub use self::{details::Details, list::List};

pub mod details;
pub mod list;

/// Song information
#[derive(Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Song {
    /// Title of the song
    pub title: String,

    /// Artist of the song
    pub artist: String,
}
