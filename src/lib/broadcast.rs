use std::io;

use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BroadcastPayload {
    Topology {
        n1: Vec<String>,
        n2: Vec<String>,
        n3: Vec<String>,
    },
    TopologyOk,
    Broadcast {
        message: String,
    },
    BroadcaseOk,
    Read,
    ReadOk {
        messages: Vec<String>,
    },
}

impl BroadcastPayload {
    pub fn handle(
        message: Message,
        writer: &mut std::io::StdoutLock,
        str_id: &mut Option<String>,
    ) -> io::Result<String> {
        Ok(String::new())
    }
}
