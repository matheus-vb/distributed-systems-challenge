use std::{
    collections::BTreeMap,
    io::{self, Write},
};

use serde::{Deserialize, Serialize};

use crate::{
    gossip::GossipPayload,
    message::{AppState, Body, Message, PayloadType},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TopologyData {
    #[serde(flatten)]
    pub nodes: BTreeMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BroadcastPayload {
    Topology { topology: TopologyData },
    TopologyOk,
    Broadcast { message: usize },
    BroadcastOk,
    Read,
    ReadOk { messages: Vec<usize> },
}

impl BroadcastPayload {
    pub fn handle(
        &self,
        message_input: Message,
        writer: &mut std::io::StdoutLock,
        app_state: &mut AppState,
    ) -> io::Result<String> {
        let new_message: Message = match self {
            Self::Topology { topology } => {
                let _ = topology
                    .nodes
                    .iter()
                    .map(|(key, value)| app_state.neighbours.insert(key.clone(), value.to_vec()));

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
                BroadcastPayload::broadcast(
                    app_state,
                    writer,
                    *message,
                    message_input.body.msg_id,
                )?;

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

    fn broadcast(
        app_state: &mut AppState,
        writer: &mut std::io::StdoutLock,
        message: usize,
        msg_id: Option<usize>,
    ) -> io::Result<()> {
        let src_id = &app_state.src_id.clone().unwrap();
        if let Some(nodes) = app_state.neighbours.get(src_id.as_str()) {
            for node in nodes {
                let new_message = Message {
                    src: src_id.to_string(),
                    dest: node.to_string(),
                    body: Body {
                        payload: PayloadType::Gossip(GossipPayload { message: message }),
                        msg_id,
                        in_reply_to: msg_id,
                    },
                };

                let output = serde_json::to_string(&new_message).unwrap();
                writer.write_all(output.as_bytes())?;
                writer.write_all(b"\n")?;
            }
        }

        app_state.record.push(message);

        Ok(())
    }
}
