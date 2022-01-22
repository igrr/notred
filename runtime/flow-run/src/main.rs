mod exit;

use crate::exit::ExitNode;
use clap::{app_from_crate, arg};
use env_logger;
use exit::EXIT_NODE_CLASS;
use json;
use notred::*;
use std::env;
use std::fs;
use std::time::Duration;

pub static NODE_CLASSES: [&NodeClass; 4] = [
    &APPEND_NODE_CLASS,
    &TICKER_NODE_CLASS,
    &CAPTURE_NODE_CLASS,
    &EXIT_NODE_CLASS,
];

fn main() {
    env_logger::init();

    let app = clap::app_from_crate!().arg(arg!(-f --flow <NAME>));
    let matches = app.get_matches();
    let flow_name = matches.value_of("flow").expect("Missing --flow argument");

    let flow_json = fs::read_to_string(flow_name).expect("Failed to read input flow file");

    let j = json::parse(flow_json.as_str()).expect("Failed to parse flow as JSON");

    let mut factory = notred::DefaultNodeFactory::default();
    factory.node_classes = Some(&NODE_CLASSES);

    let mut flow = notred::FlowState::from_json(&j, &factory).expect("Failed to build the flow");
    flow.create();

    loop {
        let res = flow.run_once(Duration::from_millis(100));
        if let Ok(()) = res {
            continue;
        }
        let exit_node: Option<&ExitNode> = if let Some(node) = flow.get_node_by_name("exit") {
            // FIXME any node of class 'exit' should trigger an exit
            node.clone().as_any().downcast_ref::<ExitNode>()
        } else {
            None
        };
        if exit_node.is_some() && exit_node.unwrap().get_should_exit() {
            break;
        }
        if let Err(Error::Timeout(_)) = res {
            continue;
        }
        res.expect("Failure while running flow");
    }
}
