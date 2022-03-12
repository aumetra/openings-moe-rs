//!
//! Entities of the list endpoint
//!

use crate::{
    entity::{Details, Song},
    CLIENT, DETAILS_URL,
};
use reqwest::Result;
use serde::{Deserialize, Serialize};

/// Entity returned by loading the data from the list endpoint
pub type List = Vec<Entry>;

/// Entry of the list
#[derive(Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Entry {
    /// Unique ID of the opening
    pub uid: String,

    /// Song of the opening
    pub song: Option<Song>,

    /// Name of the anime
    pub source: String,

    /// Creator of the subtitles
    pub subtitles: Option<String>,
}

impl Entry {
    /// Get details about the opening (by its UID)
    ///
    /// # Errors
    ///
    /// Refer to [reqwest's errors](::reqwest::Error)
    pub async fn details(&self) -> Result<Details> {
        let request = CLIENT
            .get(DETAILS_URL)
            .query(&[("uid", &self.uid)])
            .build()?;

        CLIENT.execute(request).await?.json().await
    }
}
