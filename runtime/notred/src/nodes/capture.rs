use std::any::Any;
use std::sync::{Arc, Mutex};

use crate::common::*;

#[derive(Debug)]
pub struct CaptureNode {
    common: NodeCommonData,
    captured_messages: Vec<Message>,
}

fn make_capture_node(
    mut common: NodeCommonData,
    _opt_provider: &dyn NodeOptionsProvider,
    _event_sender: Option<Arc<Mutex<dyn EventSender>>>,
) -> Result<Box<dyn Node>, NodeOptionsError> {
    common.input_types.push(None);
    Ok(Box::new(CaptureNode {
        common,
        captured_messages: Vec::new(),
    }))
}

pub static CAPTURE_NODE_CLASS: NodeClass = NodeClass {
    name: "capture",
    constructor: make_capture_node,
    num_inputs: 1,
    num_outputs: 0,
};

impl Node for CaptureNode {
    fn get_common(&self) -> &NodeCommonData {
        &self.common
    }

    fn class(&self) -> &NodeClass {
        &CAPTURE_NODE_CLASS
    }

    fn run(&mut self, msg: &Message, _index: usize) -> NodeFunctionResult {
        self.captured_messages.push(msg.clone());
        NodeFunctionResult::NoResult()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl CaptureNode {
    pub fn get_captured_messages(&self) -> &Vec<Message> {
        &self.captured_messages
    }
}
