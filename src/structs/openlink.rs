/*
 * Created on Wed Dec 02 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};

/// OpenLink info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenLinkId {

    /// OpenLink identifier
    #[serde(rename = "li")]
    pub link_id: i64,

    /// OpenLink token.
    /// Multiply to 1000 to convert to Date.
    #[serde(rename = "otk")]
    pub open_token: i32

}