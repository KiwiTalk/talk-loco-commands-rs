/*
 * Created on Wed Dec 02 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};

/// Send before server disconnect connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kickout {

    /// Kicked reasoon
    ///
    /// * Change server = 2
    /// * Login another = 0
    /// * Account deleted = 1
    pub reason: i16

}