use serde;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::common::Connection;
use crate::node::Node;
use crate::Error;

#[derive(Serialize, Deserialize)]
pub(crate) struct LoadedFlowDescription {
    pub(crate) nodes: Vec<Box<dyn Node>>,
    pub(crate) connections: Vec<Connection>,
}

impl LoadedFlowDescription {
    pub(crate) fn new(text: &str) -> Result<LoadedFlowDescription, Error> {
        let res: LoadedFlowDescription = serde_json::from_str(text)?;
        Ok(res)
    }
}
