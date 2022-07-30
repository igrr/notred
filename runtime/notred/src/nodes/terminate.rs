use std::any::Any;
use std::sync::{Arc, Mutex};

use crate::common::*;

#[derive(Debug)]
pub struct TerminateNode {
    common: NodeCommonData,
    dispatcher: Arc<Mutex<dyn EventSender>>,
}

fn make_node(
    common: NodeCommonData,
    _opt_provider: &dyn NodeOptionsProvider,
    async_dispatcher: Option<Arc<Mutex<dyn EventSender>>>,
) -> Result<Box<dyn Node>, NodeOptionsError> {
    let dispatcher = async_dispatcher.expect("dispatcher must be specified");
    Ok(Box::new(TerminateNode { common, dispatcher }))
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

    fn run(&mut self, _msg: &Message) -> NodeFunctionResult {
        self.dispatcher.lock().unwrap().dispatch(Event::Terminate());
        NodeFunctionResult::NoResult()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
