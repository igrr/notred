use std::sync::{Arc, Mutex};

use crate::common::*;
use crate::NODE_CLASSES;

#[derive(Debug)]
pub struct DefaultNodeFactory {
    pub async_dispatcher: Option<Arc<Mutex<dyn AsyncMessageDispatcher>>>,
}

impl DefaultNodeFactory {
    fn class_by_name(class_name: &str) -> Option<&NodeClass> {
        for nc in NODE_CLASSES {
            match nc.name == class_name {
                true => return Option::Some(nc),
                false => continue
            }
        }
        Option::None
    }
}

impl NodeFactory for DefaultNodeFactory {
    fn create_node(&self, class_name: &str, name: &str, opt_provider: &dyn NodeOptionsProvider) -> Option<Box<dyn Node>> {
        let class = DefaultNodeFactory::class_by_name(class_name)?;
        let log_outputs = opt_provider.get_bool("log_outputs").ok().unwrap_or(false);
        let log_inputs = opt_provider.get_bool("log_inputs").ok().unwrap_or(false);

        let res = (class.constructor)(NodeCommonData {
            name: name.to_string(),
            log_inputs,
            log_outputs
        }, opt_provider, self.async_dispatcher.clone());
        return res.ok();
    }
}
