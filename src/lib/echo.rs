use std::io::{self, Write};

use serde::{Deserialize, Serialize};

use crate::message::{AppState, Body, Message, PayloadType};

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
        app_state: &mut AppState,
    ) -> io::Result<String> {
        let new_message: Message = match &message.body.payload {
            PayloadType::Echo(EchoPayload::Init {
                node_id,
                node_ids: _,
            }) => {
                app_state.src_id = Some(node_id.clone());

                Message {
                    src: app_state.src_id.clone().expect("src id already assigned"),
                    dest: message.src,
                    body: Body {
                        payload: PayloadType::Echo(EchoPayload::InitOk),
                        msg_id: message.body.msg_id,
                        in_reply_to: message.body.msg_id,
                    },
                }
            }
            PayloadType::Echo(EchoPayload::Echo { echo }) => Message {
                src: app_state.src_id.clone().expect("src id already assigned"),
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

        //TODO: change write so it writes in generic buffer (stdout or variable injected)
        //Ex: if Stdout is passed via param, print output. Else, if a variable (some char buffer) is passed via param, write output in said buffer.

        let output = serde_json::to_string(&new_message).unwrap();
        writer.write_all(output.as_bytes())?;
        writer.write_all(b"\n")?;

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::message::{AppState, Body, Message, PayloadType};

    use super::EchoPayload;

    #[test]
    fn init_message() {
        let src_id: Option<String> = None;
        let mut app_state = AppState {
            src_id,
            neighbours: BTreeMap::new(),
        };
        let mut writer = std::io::stdout().lock();
        let message = Message {
            src: "c1".into(),
            dest: "n1".into(),
            body: Body {
                msg_id: Some(1),
                in_reply_to: None,
                payload: PayloadType::Echo(EchoPayload::Init {
                    node_id: "n3".into(),
                    node_ids: vec!["n1".into(), "n2".into(), "n3".into()],
                }),
            },
        };

        let output = EchoPayload::handle(message, &mut writer, &mut app_state);

        assert_eq!(
            output.unwrap(),
            r#"{"src":"n3","dest":"c1","body":{"type":"init_ok","msg_id":1,"in_reply_to":1}}"#
                .to_string()
        );
        assert_eq!(Some("n3".to_string()), app_state.src_id);
        //let init_input = "{"src":"c1","dest":"n1","body":{"type":"init","msg_id":1, "node_id":"n3","node_ids":["n1","n2","n3"]}}"
    }

    #[test]
    fn echo_message() {
        let src_id: Option<String> = Some("n3".to_string());
        let mut app_state = AppState {
            src_id,
            neighbours: BTreeMap::new(),
        };

        let mut writer = std::io::stdout().lock();
        let message = Message {
            src: "c1".into(),
            dest: "n1".into(),
            body: Body {
                msg_id: Some(1),
                in_reply_to: None,
                payload: PayloadType::Echo(EchoPayload::Echo {
                    echo: "Please echo 35".to_string(),
                }),
            },
        };

        let output = EchoPayload::handle(message, &mut writer, &mut app_state);

        assert_eq!(
            output.unwrap(),
            r#"{"src":"n3","dest":"c1","body":{"type":"echo_ok","echo":"Please echo 35","msg_id":1,"in_reply_to":1}}"#
                .to_string()
        );
    }
}
