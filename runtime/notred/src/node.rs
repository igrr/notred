use std::any::Any;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::common::{EventSender, Message};
use crate::{Error, MessageType};

pub type NodeFunctionResult = Result<Option<Message>, Error>;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NodeCommon {
    pub name: String,
    #[serde(default)]
    pub log_inputs: bool,
    #[serde(default)]
    pub log_outputs: bool,
}

impl NodeCommon {
    pub fn new(name: &str) -> NodeCommon {
        NodeCommon {
            name: name.to_string(),
            log_inputs: false,
            log_outputs: false,
        }
    }
}

#[typetag::serde(tag = "class")]
pub trait Node: Debug + Any {
    fn common(&self) -> &NodeCommon;
    fn create(&mut self, event_sender: Option<Arc<Mutex<dyn EventSender>>>);
    fn run(&mut self, msg: &Message, _input: usize) -> NodeFunctionResult;
    fn destroy(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn num_inputs(&self) -> usize;
    fn num_outputs(&self) -> usize;
    fn input_type(&self, index: usize) -> Option<&MessageType>;
    fn output_type(&self, index: usize) -> &MessageType;
}
