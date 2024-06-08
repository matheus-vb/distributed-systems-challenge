use serde::{Deserialize, Serialize};

use crate::message::{AppState, Message, SeenMessage};

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
        app_state: &mut AppState,
    ) -> Result<String, &'static str> {
        if message.dest == app_state.src_id.clone().expect("src is already assigned") {
            app_state.record.push(SeenMessage {
                message: self.message.clone(),
                id: self.id.clone(),
            });
            println!("read message!");
            Ok("read message".to_string())
        } else {
            Err("wrong destination")
        }
    }
}
