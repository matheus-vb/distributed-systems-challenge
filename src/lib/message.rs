use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    error::Error,
    io::{self, Error as IoError, ErrorKind},
};

use crate::{
    broadcast::BroadcastPayload, echo::EchoPayload, generate::GeneratePayload,
    gossip::GossipPayload,
};

pub struct AppState {
    pub src_id: Option<String>,
    pub neighbours: BTreeMap<String, Vec<String>>,
    pub record: Vec<SeenMessage>,
}

#[derive(Clone)]
pub struct SeenMessage {
    pub message: usize,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PayloadType {
    Generate(GeneratePayload),
    Echo(EchoPayload),
    Broadcast(BroadcastPayload),
    Gossip(GossipPayload),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Body {
    #[serde(flatten)]
    pub payload: PayloadType,
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

impl Message {
    pub fn handle(
        self,
        writer: &mut std::io::StdoutLock,
        app_state: &mut AppState,
    ) -> io::Result<String> {
        match &self.body.payload {
            PayloadType::Echo(_) => EchoPayload::handle(self, writer, app_state),
            PayloadType::Generate(_) => GeneratePayload::handle(self, writer, app_state),
            PayloadType::Broadcast(payload) => {
                BroadcastPayload::handle(&payload.clone(), self, writer, app_state)
            }
            PayloadType::Gossip(payload) => {
                GossipPayload::handle(&payload.clone(), self, writer, app_state)
                    .map(|()| String::new())
            }
        }
    }
}
