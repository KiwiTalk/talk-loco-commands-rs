/*
 * Created on Tue Dec 01 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::BsonData;
 
/// Answer loco server information
#[derive(Debug, Clone, Serialize, Deserialize, BsonData)]
pub struct Checkin {

    /// Loco server ip
    pub host: String,

    /// Loco server ip(v6)
    pub host6: String,

    /// Loco server port
    pub port: i32,

    /// Info cache expire time(?)
    #[serde(rename = "cacheExpire")]
    pub cache_expire: i32,

    /// Call server ip
    #[serde(rename = "cshost")]
    pub cs_host: String,

    /// Call server ip(v6)
    #[serde(rename = "cshost6")]
    pub cs_host6: String,

    /// Call server port
    #[serde(rename = "csport")]
    pub cs_port: i32,

    /// Unknown server ip
    #[serde(rename = "vsshost")]
    pub vss_host: String,

    /// Unknown server ip(v6)
    #[serde(rename = "vsshost6")]
    pub vss_host6: String,

    /// Unknown server port
    #[serde(rename = "vssport")]
    pub vss_port: i32,
  
}