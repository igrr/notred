use std::sync::{Arc, Mutex};

use crate::common::*;
use crate::NODE_CLASSES;

pub struct DefaultNodeFactory<'a> {
    pub node_classes: Option<&'a [&'a NodeClass]>,
}

impl Default for DefaultNodeFactory<'_> {
    fn default() -> Self {
        DefaultNodeFactory {
            node_classes: Some(&NODE_CLASSES),
        }
    }
}

impl DefaultNodeFactory<'_> {
    fn class_by_name(&self, class_name: &str) -> Option<&NodeClass> {
        let ncs = self.node_classes.unwrap_or(&NODE_CLASSES);
        for nc in ncs {
            match nc.name == class_name {
                true => return Option::Some(nc),
                false => continue,
            }
        }
        Option::None
    }
}

impl NodeFactory for DefaultNodeFactory<'_> {
    fn create_node(
        &self,
        class_name: &str,
        name: &str,
        opt_provider: &dyn NodeOptionsProvider,
        async_dispatcher: Option<Arc<Mutex<dyn EventSender>>>,
    ) -> Option<Box<dyn Node>> {
        let class = self.class_by_name(class_name)?;
        let log_outputs = opt_provider.get_bool("log_outputs").ok().unwrap_or(false);
        let log_inputs = opt_provider.get_bool("log_inputs").ok().unwrap_or(false);

        let res = (class.constructor)(
            NodeCommonData {
                name: name.to_string(),
                log_inputs,
                log_outputs,
            },
            opt_provider,
            async_dispatcher.clone(),
        );
        return res.ok();
    }
}
