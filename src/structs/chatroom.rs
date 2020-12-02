/*
 * Created on Wed Dec 02 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};

use super::{chat::Chatlog, openlink::OpenLinkId};

/// LOGINLIST chatroom list item.
/// Including eseential chatroom info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatroomListData {

    /// Chatroom id
    #[serde(rename = "c")]
    pub id: i64,

    /// Chatroom type
    /// * group = "MultiChat"
    /// * direct = "DirectChat"
    /// * pluschat = "PlusChat"
    /// * self = "MemoChat"
    /// * openchat group = "OM"
    /// * openchat direct = "OD"
    #[serde(rename = "t")]
    pub chatroom_type: String,

    /// Last chat log id
    #[serde(rename = "ll")]
    pub last_chat_log_id: i64,

    // /// Last Chatlog
    // #[serde(rename = "l")]
    // pub chatlog: Option<Chatlog>,

    /// Member count
    #[serde(rename = "a")]
    pub member_count: i32,

    /// Unread message count
    #[serde(rename = "n")]
    pub unread_count: i32,

    // /// Chatroom metadata(?)
    // #[serde(rename = "m")]
    // pub metadata: ()
    
    
    /// Push alert setting
    #[serde(rename = "p")]
    pub push_alert: bool,

    /// Only present if chatroom is Openchat
    #[serde(flatten)]
    pub link: Option<OpenLinkId>,

    /// Chatroom preview icon target user id list
    #[serde(rename = "i")]
    pub icon_user_ids: Option<Vec<i64>>,

    /// Chatroom preview icon target user name list
    #[serde(rename = "k")]
    pub icon_user_nicknames: Option<Vec<String>>,

    /// Unknown. Always 0 on openchat rooms.
    pub mmr: i64,

    /// Unknown
    pub s: i64,

    /// Unknown. Only appears on openchat rooms.
    pub o: Option<i32>,

    /// Unknown. Only appears on non openchat rooms.
    pub jn: Option<i32>
}