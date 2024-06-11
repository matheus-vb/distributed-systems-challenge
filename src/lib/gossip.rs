use std::io::{self, Write};

use serde::{Deserialize, Serialize};

use crate::message::{AppState, Body, Message, PayloadType, SeenMessage};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub struct GossipPayload {
    pub message: usize,
    pub id: String,
}

impl GossipPayload {
    pub fn handle(
        &self,
        message: Message,
        writer: &mut std::io::StdoutLock,
        app_state: &mut AppState,
    ) -> io::Result<()> {
        let src_id = &app_state.src_id.clone().unwrap();

        if let Some(nodes) = app_state.neighbours.get(src_id.as_str()) {
            for node in nodes {
                let new_message = Message {
                    src: src_id.to_string(),
                    dest: node.to_string(),
                    body: Body {
                        payload: PayloadType::Gossip(GossipPayload {
                            message: self.message.clone(),
                            id: self.id.clone(),
                        }),
                        msg_id: message.body.msg_id,
                        in_reply_to: message.body.msg_id,
                    },
                };

                let output = serde_json::to_string(&new_message).unwrap();
                writer.write_all(output.as_bytes())?;
                writer.write_all(b"\n")?;
            }
        }

        if message.dest == app_state.src_id.clone().expect("src is already assigned") {
            app_state.record.push(SeenMessage {
                message: self.message.clone(),
                id: self.id.clone(),
            });
            println!("read message!");
        };

        Ok(())
    }
}
