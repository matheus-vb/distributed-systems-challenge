extern crate ds_challenge;
use std::{
    collections::BTreeMap,
    io::{self, BufRead, Write},
};

use ds_challenge::message::{AppState, Message};

fn main() -> io::Result<()> {
    let stdin = std::io::stdin();
    let reader = stdin.lock();
    let mut writer = std::io::stdout().lock();

    let mut app_state = AppState {
        src_id: None,
        neighbours: BTreeMap::new(),
        record: vec![],
    };

    for line in reader.lines() {
        let line = line?;
        let message: Message = serde_json::from_str(&line).expect("Failed to deserialize");

        message.handle(&mut writer, &mut app_state)?;
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
//Echo:
//{"src":"c1","dest":"n1","body":{"type":"echo","msg_id":1,"echo":"Please echo 35"}}
//Generate:
//{"src":"c1","dest":"n1","body":{"type":"generate","msg_id":1}}
