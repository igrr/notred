use notred::*;
use std::any::Any;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct ExitNode {
    common: NodeCommonData,
    should_exit: bool,
}

fn make_exit_node(
    common: NodeCommonData,
    _opt_provider: &dyn NodeOptionsProvider,
    _async_dispatcher: Option<Arc<Mutex<dyn AsyncMessageDispatcher>>>,
) -> Result<Box<dyn Node>, NodeOptionsError> {
    Ok(Box::new(ExitNode {
        common,
        should_exit: false,
    }))
}

pub static EXIT_NODE_CLASS: NodeClass = NodeClass {
    name: "exit",
    constructor: make_exit_node,
    has_input: true,
    num_outputs: 0,
};

impl Node for ExitNode {
    fn get_common(&self) -> &NodeCommonData {
        &self.common
    }

    fn class(&self) -> &NodeClass {
        &EXIT_NODE_CLASS
    }

    fn run(&mut self, msg: &Message) -> NodeFunctionResult {
        self.should_exit = true;
        NodeFunctionResult::NoResult()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ExitNode {
    pub fn get_should_exit(&self) -> bool {
        self.should_exit
    }
}
