#[derive(Debug, Default)]
pub struct Message {
    pub value: String,
}

#[derive(Debug)]
pub enum NodeFunctionResult {
    Success(Message),
    NoResult(),
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
}

pub trait NodeCommon {
    fn get_common(&self) -> &NodeCommonData;
    fn get_name(&self) -> &str {
        self.get_common().name.as_str()
    }
}

pub trait Node: NodeCommon {
    fn class(&self) -> &NodeClass;
    fn create(&mut self) {}
    fn run(&mut self, msg: &Message) -> NodeFunctionResult;
    fn destroy(&mut self) {}
}

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
    ) -> Result<Box<dyn Node>, NodeOptionsError>,
    pub has_input: bool,
    pub num_outputs: usize,
}