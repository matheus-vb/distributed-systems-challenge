use std::io::{self, Write};

use serde::{Deserialize, Serialize};

use crate::message::{AppState, Body, Message, PayloadType};

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
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<String>,
    },
}

impl BroadcastPayload {
    pub fn handle(
        &self,
        message_input: Message,
        writer: &mut std::io::StdoutLock,
        app_state: &mut AppState,
    ) -> io::Result<String> {
        let new_message: Message = match self {
            Self::Topology { n1, n2, n3 } => {
                app_state.neighbours.insert("n1", n1.to_vec());
                app_state.neighbours.insert("n2", n2.to_vec());
                app_state.neighbours.insert("n3", n3.to_vec());

                Message {
                    src: app_state.src_id.clone().expect("src is already assigned"),
                    dest: message_input.src,
                    body: Body {
                        payload: PayloadType::Broadcast(BroadcastPayload::TopologyOk),
                        msg_id: message_input.body.msg_id,
                        in_reply_to: message_input.body.msg_id,
                    },
                }
            }
            Self::Broadcast { message } => {
                BroadcastPayload::broadcast(app_state, writer, message);

                Message {
                    src: app_state.src_id.clone().expect("src is already assigned"),
                    dest: message_input.src,
                    body: Body {
                        payload: PayloadType::Broadcast(BroadcastPayload::BroadcastOk),
                        msg_id: message_input.body.msg_id,
                        in_reply_to: message_input.body.msg_id,
                    },
                }
            }
            Self::Read => Message {
                src: app_state.src_id.clone().expect("src is already assigned"),
                dest: message_input.src,
                body: Body {
                    payload: PayloadType::Broadcast(BroadcastPayload::ReadOk {
                        messages: app_state.record.clone(),
                    }),
                    msg_id: message_input.body.msg_id,
                    in_reply_to: message_input.body.msg_id,
                },
            },
            _ => unreachable!(),
        };

        let output = serde_json::to_string(&new_message).unwrap();
        writer.write_all(output.as_bytes())?;
        writer.write_all(b"\n")?;

        Ok(output)
    }

    fn broadcast(app_state: &AppState, writer: &mut std::io::StdoutLock, message: &String) {
        let src_id = &app_state.src_id.clone().unwrap();
        if let Some(nodes) = app_state.neighbours.get(src_id.as_str()) {
            for node in nodes {
                //TODO: send message to other nodes
                /* let new_message = Message {
                    src: src_id,
                    dest: node,
                    body: Body { payload: PayloadType, msg_id: (), in_reply_to: () }
                } */
            }
        }

        /* let new_message = Message {
            src: app_state.src_id.clone().expect("src is already assigned"),
            dest:
        } */
    }
}