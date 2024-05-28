use std::io::{self, Write};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::message::{AppState, Body, Message, PayloadType};

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
        app_state: &mut AppState,
    ) -> io::Result<String> {
        let new_id = Uuid::now_v7();

        let new_message = Message {
            src: app_state.src_id.clone().expect("src id already assigned"),
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::Value;
    use uuid::Uuid;

    use crate::message::{AppState, Body, Message, PayloadType};

    use super::GeneratePayload;

    #[test]
    fn generate_uuid() {
        let src_id = Some("n3".to_string());

        let mut app_state = AppState {
            src_id,
            neighbours: BTreeMap::new(),
            record: vec![],
        };

        let mut writer = std::io::stdout().lock();

        let message = Message {
            src: "c1".into(),
            dest: "n1".into(),
            body: Body {
                msg_id: Some(1),
                in_reply_to: None,
                payload: PayloadType::Generate(GeneratePayload::Generate),
            },
        };

        let output = GeneratePayload::handle(message, &mut writer, &mut app_state).unwrap();

        let des_output: Value = serde_json::from_str(output.as_str()).unwrap();

        assert_eq!(des_output["src"], "n3");
        assert_eq!(des_output["dest"], "c1");
        assert_eq!(des_output["body"]["type"], "generate_ok");
        assert_eq!(des_output["body"]["msg_id"], 1);
        assert_eq!(des_output["body"]["in_reply_to"], 1);

        let uuid_str = des_output["body"]["id"]
            .as_str()
            .expect("should be a string");
        assert!(Uuid::parse_str(uuid_str).is_ok());
    }
}
