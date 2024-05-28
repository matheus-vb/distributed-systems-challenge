use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    io::{self, BufRead, StdoutLock, Write},
};
use uuid::Uuid;

use crate::{broadcast::BroadcastPayload, echo::EchoPayload, generate::GeneratePayload};

pub struct AppState {
    pub src_id: Option<String>,
    pub neighbours: BTreeMap<&'static str, Vec<String>>,
    pub record: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PayloadType {
    Generate(GeneratePayload),
    Echo(EchoPayload),
    Broadcast(BroadcastPayload),
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
            PayloadType::Echo(_) => Ok(EchoPayload::handle(self, writer, app_state)?),
            PayloadType::Generate(_) => Ok(GeneratePayload::handle(self, writer, app_state)?),
            PayloadType::Broadcast(_) => todo!(),
        }
    }
}
