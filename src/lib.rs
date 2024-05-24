use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, StdoutLock, Write};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum PayloadType {
    Generate(GeneratePayload),
    Echo(EchoPayload),
}

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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum GeneratePayload {
    Generate,
    GenerateOk { id: String },
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Body {
    #[serde(flatten)]
    pub payload: PayloadType,
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

impl Message {
    pub fn handle(
        self,
        writer: &mut std::io::StdoutLock,
        src_id: &mut Option<String>,
    ) -> io::Result<()> {
        match &self.body.payload {
            PayloadType::Echo(_) => {
                self.echo(writer, src_id)?;
            }
            PayloadType::Generate(_) => {
                self.generate(writer, src_id)?;
            }
        };

        Ok(())
    }
}

impl Message {
    fn echo(self, writer: &mut std::io::StdoutLock, src_id: &mut Option<String>) -> io::Result<()> {
        let new_message: Message = match &self.body.payload {
            PayloadType::Echo(EchoPayload::Init {
                node_id,
                node_ids: _,
            }) => {
                *src_id = Some(node_id.clone());

                Message {
                    src: src_id.clone().expect("src id already assigned"),
                    dest: self.src,
                    body: Body {
                        payload: PayloadType::Echo(EchoPayload::InitOk),
                        msg_id: self.body.msg_id,
                        in_reply_to: self.body.msg_id,
                    },
                }
            }
            PayloadType::Echo(EchoPayload::Echo { echo }) => Message {
                src: src_id.clone().expect("src id already assigned"),
                dest: self.src,
                body: Body {
                    payload: PayloadType::Echo(EchoPayload::EchoOk {
                        echo: echo.to_string(),
                    }),
                    msg_id: self.body.msg_id,
                    in_reply_to: self.body.msg_id,
                },
            },
            _ => unreachable!(),
        };

        serde_json::to_writer(&mut *writer, &new_message)?;
        writer.write_all(b"\n")?;

        Ok(())
    }

    fn generate(
        self,
        writer: &mut std::io::StdoutLock,
        src_id: &mut Option<String>,
    ) -> io::Result<()> {
        todo!();
    }
}

