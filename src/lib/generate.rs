use std::io::{self, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::message::{Body, Message, PayloadType};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum GeneratePayload {
    Generate,
    GenerateOk { id: String },
}

impl GeneratePayload {
    pub fn handle(
        message: Message,
        writer: &mut std::io::StdoutLock,
        src_id: &mut Option<String>,
    ) -> io::Result<String> {
        let new_id = Uuid::now_v7();

        let new_message = Message {
            src: src_id.clone().expect("src id is already assigned"),
            dest: message.src,
            body: Body {
                payload: PayloadType::Generate(GeneratePayload::GenerateOk {
                    id: new_id.to_string(),
                }),
                msg_id: message.body.msg_id,
                in_reply_to: message.body.msg_id,
            },
        };

        let output = serde_json::to_string(&new_message).unwrap();
        writer.write_all(output.as_bytes())?;
        writer.write_all(b"\n")?;

        Ok(output)
    }
}
