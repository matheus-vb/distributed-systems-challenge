use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
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

#[derive(Deserialize, Serialize, Debug)]
struct Body {
    #[serde(flatten)]
    payload: Payload,
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Message {
    src: String,
    dest: String,
    body: Body,
}

fn main() -> io::Result<()> {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut writer = std::io::stdout().lock();
    let mut src_id: Option<String> = None;

    for line in reader.lines() {
        let line = line?;
        let mut message: Message = serde_json::from_str(&line).expect("Failed to deserialize");

        let new_message: Message = match &message.body.payload {
            Payload::Init {
                node_id,
                node_ids: _,
            } => {
                src_id = Some(node_id.clone());

                Message {
                    src: src_id.clone().expect("src id already assigned"),
                    dest: message.src,
                    body: Body {
                        payload: Payload::InitOk,
                        msg_id: message.body.msg_id,
                        in_reply_to: message.body.msg_id,
                    },
                }
            }
            Payload::Echo { echo } => Message {
                src: src_id.clone().expect("src id already assigned"),
                dest: message.src,
                body: Body {
                    payload: Payload::EchoOk {
                        echo: echo.to_string(),
                    },
                    msg_id: message.body.msg_id,
                    in_reply_to: message.body.msg_id,
                },
            },
            _ => unreachable!(),
        };

        serde_json::to_writer(&mut writer, &new_message)?;
        writer.write_all(b"\n")?;
    }
    /* let message = Message {
        src: "c1".into(),
        dest: "n1".into(),
        body: Body {
            payload: Payload::Init {
                node_id: "n1".to_string(),
                node_ids: vec!["n2".to_string()],
            },
            msg_id: None,
            in_reply_to: None,
        },
    };

    serde_json::to_writer(&mut writer, &message)?;
    writer.write_all(b"\n")?; */

    Ok(())
}

//{"src":"c1","dest":"n1","body":{"type":"init","msg_id":1, "node_id":"n3","node_ids":["n1","n2","n3"]}}
//{"src":"c1","dest":"n1","body":{"type":"echo","msg_id":1,"echo":"Please echo 35"}}
