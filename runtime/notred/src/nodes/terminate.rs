use std::any::Any;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::common::*;
use crate::node::*;
use crate::MessageType;

#[derive(Serialize, Deserialize, Debug)]
pub struct TerminateNode {
    #[serde(flatten)]
    common: NodeCommon,
    #[serde(skip)]
    event_sender: Option<Arc<Mutex<dyn EventSender>>>,
}

#[typetag::serde(name = "terminate")]
impl Node for TerminateNode {
    fn common(&self) -> &NodeCommon {
        &self.common
    }

    fn create(&mut self, event_sender: Option<Arc<Mutex<dyn EventSender>>>) {
        self.event_sender = event_sender
    }

    fn run(&mut self, _msg: &Message, _input: usize) -> NodeFunctionResult {
        self.event_sender
            .as_ref()
            .unwrap()
            .lock()
            .unwrap()
            .dispatch(Event::Terminate());
        Ok(None)
    }

    fn destroy(&mut self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn num_inputs(&self) -> usize {
        1
    }

    fn num_outputs(&self) -> usize {
        0
    }

    fn input_type(&self, index: usize) -> Option<&MessageType> {
        assert_eq!(index, 0);
        None
    }

    fn output_type(&self, _index: usize) -> &MessageType {
        unreachable!("node has no outputs");
    }
}
