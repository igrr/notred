use std::any::Any;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::common::*;
use crate::node::*;
use crate::MessageType;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CaptureNode {
    #[serde(flatten)]
    common: NodeCommon,
    #[serde(skip)]
    captured_messages: Vec<Message>,
}

#[typetag::serde(name = "capture")]
impl Node for CaptureNode {
    fn common(&self) -> &NodeCommon {
        &self.common
    }

    fn create(&mut self, _event_sender: Option<Arc<Mutex<dyn EventSender>>>) {}

    fn run(&mut self, msg: &Message, index: usize) -> NodeFunctionResult {
        assert_eq!(index, 0);
        self.captured_messages.push(msg.clone());
        Ok(None)
    }

    fn destroy(&mut self) {
        todo!()
    }

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
        unreachable!("node has no outputs")
    }
}

impl CaptureNode {
    #[cfg(test)]
    pub fn get_captured_messages(&self) -> &Vec<Message> {
        &self.captured_messages
    }
}
