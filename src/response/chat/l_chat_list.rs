/*
 * Created on Thu Dec 03 2020
 *
 * Copyright (c) storycraft. Licensed under the MIT Licence.
 */

use serde::{Serialize, Deserialize};
use crate::{ structs::chatroom::ChatroomListData};

/// Request every chatroom list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LChatList {

    #[serde(rename = "chatDatas")]
    pub chat_datas: Vec<ChatroomListData>

}