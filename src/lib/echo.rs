use std::io::{self, Write};

use serde::{Deserialize, Serialize};

use crate::message::{Body, Message, PayloadType};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum EchoPayload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

impl EchoPayload {
    pub fn handle(
        message: Message,
        writer: &mut std::io::StdoutLock,
        src_id: &mut Option<String>,
    ) -> io::Result<()> {
        let new_message: Message = match &message.body.payload {
            PayloadType::Echo(EchoPayload::Init {
                node_id,
                node_ids: _,
            }) => {
                *src_id = Some(node_id.clone());

                Message {
                    src: src_id.clone().expect("src id already assigned"),
                    dest: message.src,
                    body: Body {
                        payload: PayloadType::Echo(EchoPayload::InitOk),
                        msg_id: message.body.msg_id,
                        in_reply_to: message.body.msg_id,
                    },
                }
            }
            PayloadType::Echo(EchoPayload::Echo { echo }) => Message {
                src: src_id.clone().expect("src id already assigned"),
                dest: message.src,
                body: Body {
                    payload: PayloadType::Echo(EchoPayload::EchoOk {
                        echo: echo.to_string(),
                    }),
                    msg_id: message.body.msg_id,
                    in_reply_to: message.body.msg_id,
                },
            },
            _ => unreachable!(),
        };

        serde_json::to_writer(&mut *writer, &new_message)?;
        writer.write_all(b"\n")?;

        Ok(())
    }
}
