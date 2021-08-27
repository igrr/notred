use crate::common::{NodeOptionsError, NodeOptionsProvider};

pub struct JsonNodeOptionsProvider<'a> {
    pub data: &'a json::JsonValue,
}

impl NodeOptionsProvider for JsonNodeOptionsProvider<'_> {
    fn get_str(&self, key: &str) -> Result<&str, NodeOptionsError> {
        match self.data[key].as_str() {
            Some(v) => Ok(v),
            None => Err(NodeOptionsError {}),
        }
    }

    fn get_bool(&self, key: &str) -> Result<bool, NodeOptionsError> {
        match self.data[key].as_bool() {
            Some(v) => Ok(v),
            None => Err(NodeOptionsError {}),
        }
    }

    fn get_usize(&self, key: &str) -> Result<usize, NodeOptionsError> {
        match self.data[key].as_usize() {
            Some(v) => Ok(v),
            None => Err(NodeOptionsError {}),
        }
    }

    fn get_i32(&self, key: &str) -> Result<i32, NodeOptionsError> {
        match self.data[key].as_i32() {
            Some(v) => Ok(v),
            None => Err(NodeOptionsError {}),
        }
    }

    fn get_f32(&self, key: &str) -> Result<f32, NodeOptionsError> {
        match self.data[key].as_f32() {
            Some(v) => Ok(v),
            None => Err(NodeOptionsError {}),
        }
    }
}
