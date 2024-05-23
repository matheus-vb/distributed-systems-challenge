use serde::{Deserialize, Serialize};
use std::io::{self, BufRead, Write};

#[derive(Deserialize, Serialize, Debug)]
struct Body {
    #[serde(rename = "type")]
    kind: String,
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

    for line in reader.lines() {
        let line = line?;
        let mut message: Message = serde_json::from_str(&line).expect("Failed to deserialize");

        match message.body.kind.as_str() {
            "init" => {
                message.body.kind = String::from("init_ok");
            }
            _ => {
                message.body.kind = String::from("echo_ok");
            }
        }

        serde_json::to_writer(&mut writer, &message)?;
        writer.write_all(b"\n")?;
    }

    Ok(())
}
