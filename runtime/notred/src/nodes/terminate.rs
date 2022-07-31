use std::any::Any;
use std::sync::{Arc, Mutex};

use crate::common::*;

#[derive(Debug)]
pub struct TerminateNode {
    common: NodeCommonData,
    event_sender: Arc<Mutex<dyn EventSender>>,
}

fn make_node(
    common: NodeCommonData,
    _opt_provider: &dyn NodeOptionsProvider,
    event_sender: Option<Arc<Mutex<dyn EventSender>>>,
) -> Result<Box<dyn Node>, NodeOptionsError> {
    let event_sender = event_sender.expect("event_sender must be specified");
    Ok(Box::new(TerminateNode { common, event_sender }))
}

pub static TERMINATE_NODE_CLASS: NodeClass = NodeClass {
    name: "terminate",
    constructor: make_node,
    has_input: true,
    num_outputs: 0,
};

impl Node for TerminateNode {
    fn get_common(&self) -> &NodeCommonData {
        &self.common
    }

    fn class(&self) -> &NodeClass {
        &TERMINATE_NODE_CLASS
    }

    fn run(&mut self, _msg: &Message, _input: usize) -> NodeFunctionResult {
        self.event_sender.lock().unwrap().dispatch(Event::Terminate());
        NodeFunctionResult::NoResult()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
