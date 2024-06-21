use std::io::{self, Write};

use rand::prelude::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use crate::message::{AppState, Body, Message, PayloadType, SeenMessage};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub struct GossipPayload {
    pub message: usize,
    pub id: String,
}

// When a message arrives, the node evaluates if it is a new one
// If it is a new message, the node will randomly select a subset of nodes to send the message to.
impl GossipPayload {
    pub fn handle(
        &self,
        message: Message,
        writer: &mut std::io::StdoutLock,
        app_state: &mut AppState,
    ) -> io::Result<()> {
        //ignore message if already in the record
        if app_state.record.get(&self.id).is_some() {
            return Ok(());
        }

        let src_id = &app_state.src_id.clone().unwrap();

        let all_nodes: Vec<String> = app_state
            .neighbours
            .values()
            .flat_map(|n| n.iter().cloned())
            .collect();

        let num_gossip = all_nodes.len() / 5;

        let mut rng = thread_rng();
        let mut subset = all_nodes.clone();

        subset.shuffle(&mut rng);
        subset.truncate(num_gossip);

        for node in subset {
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

        app_state.record.insert(self.id.clone(), self.message);

        Ok(())
    }
}
