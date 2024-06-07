extern crate ds_challenge;
use std::{
    collections::BTreeMap,
    io::{self, BufRead, Write},
};

use ds_challenge::{
    broadcast::{BroadcastPayload, TopologyData},
    echo::EchoPayload,
    message::{AppState, Body, Message, PayloadType},
};

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
        let message: Message =
            serde_json::from_str(&line).expect(format!("Failed to parse:\n{line}\n").as_str());
        message.handle(&mut writer, &mut app_state)?;
    }

    /* let message = Message {
        src: "c1".into(),
        dest: "n1".into(),
        body: Body {
            msg_id: Some(1),
            in_reply_to: None,
            payload: PayloadType::Broadcast(BroadcastPayload::Topology {
                topology: TopologyData {
                    n1: vec!["n2".to_string()],
                    n2: vec!["n1".to_string(), "n3".to_string()],
                    n3: vec!["n2".to_string()],
                },
            }),
        },
    };

    serde_json::to_writer_pretty(&mut writer, &message)?; */

    /*
    serde_json::to_writer(&mut writer, &message)?;
    writer.write_all(b"\n")?;
    */

    Ok(())
}

//{"src":"c1","dest":"n1","body":{"type":"init","msg_id":1, "node_id":"n3","node_ids":["n1","n2","n3"]}}
//Echo:
//{"src":"c1","dest":"n1","body":{"type":"echo","msg_id":1,"echo":"Please echo 35"}}
//Generate:
//{"src":"c1","dest":"n1","body":{"type":"generate","msg_id":1}}
//Topology:
//{"src":"c1","dest":"n1","body":{"type":"topology","msg_id":1,"topology":{"n1":["n2","n3"],"n2":["n1"],"n3":["n1"]}}}
//Broadcast:
//{"src":"c1","dest":"n1","body":{"type":"broadcast","msg_id":1,"message":10}}
//Gossip:
//{"src":"c1","dest":"n3","body":{"type":"gossip","msg_id":1,"message":10}}
//Read:
//{"src":"c1","dest":"n3","body":{"type":"read","msg_id":1}}
