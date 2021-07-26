/*
 * Created on Wed Dec 02 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};

/// Signal server to keep connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ping {

}
