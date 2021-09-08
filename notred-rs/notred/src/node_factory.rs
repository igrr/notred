use crate::common::*;
use std::sync::{Arc, Mutex};
use crate::NODE_CLASSES;

#[derive(Debug)]
pub struct DefaultNodeFactory{
    pub async_dispatcher: Option<Arc<Mutex<dyn AsyncMessageDispatcher>>>
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
        let res = (class.constructor)(NodeCommonData{
            name: name.to_string()
        }, opt_provider, self.async_dispatcher.clone());
        return res.ok();
    }
}
