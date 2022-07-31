use core::fmt;
use std::any::Any;
use std::fmt::Formatter;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NodePort {
    pub name: String,
    pub index: usize,
}

// FIXME: rename to MessageData
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Message {
    pub value: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

// FIXME: rename to Message
#[derive(Debug, Default, Clone, PartialEq)]
pub struct MessageTo {
    pub message: Message,
    pub to: NodePort,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MessageFrom {
    pub message: Message,
    pub from: NodePort,
}

#[derive(Debug)]
pub enum NodeFunctionResult {
    Success(Message),
    NoResult(),
    // FIXME: should node function be allowed to return an error?
}

impl NodeFunctionResult {
    pub fn as_message(&self) -> Option<&Message> {
        match self {
            NodeFunctionResult::Success(m) => Option::Some(m),
            NodeFunctionResult::NoResult() => Option::None,
        }
    }
}

#[derive(Debug, Default)]
pub struct NodeCommonData {
    pub name: String,
    pub log_inputs: bool,
    pub log_outputs: bool,
}

impl NodeCommonData {
    pub fn from_name(name: &str) -> NodeCommonData {
        NodeCommonData {
            name: name.to_string(),
            log_inputs: false,
            log_outputs: false,
        }
    }
}

pub trait Node: fmt::Debug {
    fn get_common(&self) -> &NodeCommonData;
    fn get_name(&self) -> &str {
        self.get_common().name.as_str()
    }
    // FIXME: should be get_class to be similar to get_common and get_name?
    fn class(&self) -> &NodeClass;
    fn create(&mut self) {}
    fn run(&mut self, _msg: &Message) -> NodeFunctionResult {
        unimplemented!();
    }
    fn destroy(&mut self) {}
    fn as_any(&self) -> &dyn Any {
        unimplemented!();
    }
    fn should_log_inputs(&self) -> bool {
        self.get_common().log_inputs
    }
    fn should_log_outputs(&self) -> bool {
        self.get_common().log_outputs
    }
}

// FIXME: make this useful ("key not found" error, "value type" error)
#[derive(Debug, Clone)]
pub struct NodeOptionsError;

pub trait NodeOptionsProvider {
    fn get_str(&self, key: &str) -> Result<&str, NodeOptionsError>;
    fn get_bool(&self, key: &str) -> Result<bool, NodeOptionsError>;
    fn get_usize(&self, key: &str) -> Result<usize, NodeOptionsError>;
    fn get_i32(&self, key: &str) -> Result<i32, NodeOptionsError>;
    fn get_f32(&self, key: &str) -> Result<f32, NodeOptionsError>;
}

pub struct NodeClass {
    pub name: &'static str,
    pub constructor: fn(
        common: NodeCommonData,
        opt_provider: &dyn NodeOptionsProvider,
        event_sender: Option<Arc<Mutex<dyn EventSender>>>,
    ) -> Result<Box<dyn Node>, NodeOptionsError>,
    pub has_input: bool,
    pub num_outputs: usize,
}

pub enum Event {
    MessageTo(MessageTo),
    MessageFrom(MessageFrom),
    Log(String),
    Terminate(),
}

pub trait EventSender: fmt::Debug + Send {
    fn dispatch(&mut self, e: Event);
}

pub trait NodeFactory {
    fn create_node(
        &self,
        class_name: &str,
        name: &str,
        opt_provider: &dyn NodeOptionsProvider,
        event_sender: Option<Arc<Mutex<dyn EventSender>>>,
    ) -> Option<Box<dyn Node>>;
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub source: String,
    pub dest: String,
    pub source_output_index: usize,
}
