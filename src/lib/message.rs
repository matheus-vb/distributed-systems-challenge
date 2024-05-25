use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, StdoutLock, Write};
use uuid::Uuid;

use crate::{echo::EchoPayload, generate::GeneratePayload};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PayloadType {
    Generate(GeneratePayload),
    Echo(EchoPayload),
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
        src_id: &mut Option<String>,
    ) -> io::Result<()> {
        match &self.body.payload {
            PayloadType::Echo(_) => {
                EchoPayload::handle(self, writer, src_id)?;
            }
            PayloadType::Generate(_) => {
                GeneratePayload::handle(self, writer, src_id)?;
            }
        };

        Ok(())
    }
}
