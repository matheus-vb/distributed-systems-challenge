mod lib;
use lib::*;

use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut writer = std::io::stdout().lock();
    let mut src_id: Option<String> = None;

    for line in reader.lines() {
        let line = line?;
        let message: Message = serde_json::from_str(&line).expect("Failed to deserialize");

        message.handle(&mut writer, &mut src_id)?;
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
