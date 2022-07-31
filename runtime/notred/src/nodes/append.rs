use std::sync::{Arc, Mutex};

use crate::common::*;

#[derive(Debug)]
struct AppendNode {
    common: NodeCommonData,
    what_to_append: String,
}

fn make_append_node(
    common: NodeCommonData,
    opt_provider: &dyn NodeOptionsProvider,
    _event_sender: Option<Arc<Mutex<dyn EventSender>>>,
) -> Result<Box<dyn Node>, NodeOptionsError> {
    let what_to_append = match opt_provider.get_str("what_to_append") {
        Ok(s) => s.to_string(),
        Err(e) => return Err(e),
    };
    Ok(Box::new(AppendNode {
        common,
        what_to_append,
    }))
}

pub static APPEND_NODE_CLASS: NodeClass = NodeClass {
    name: "append",
    constructor: make_append_node,
    num_inputs: 1,
    num_outputs: 1,
};

impl Node for AppendNode {
    fn get_common(&self) -> &NodeCommonData {
        &self.common
    }

    fn class(&self) -> &NodeClass {
        &APPEND_NODE_CLASS
    }

    fn run(&mut self, msg: &Message, _index: usize) -> NodeFunctionResult {
        NodeFunctionResult::Success(Message {
            value: msg.value.clone() + &self.what_to_append,
        })
    }
}

#[cfg(test)]
mod test {
    use json;

    use crate::json_options_provider::JsonNodeOptionsProvider;

    use super::*;

    #[test]
    fn test_make_append_node() {
        let mut n = make_append_node(
            NodeCommonData::from_name("node1"),
            &JsonNodeOptionsProvider {
                data: &json::object! {"what_to_append": " test"},
            },
            None,
        )
        .unwrap();
        assert_eq!(n.get_name(), "node1");
        assert_eq!(
            n.run(
                &Message {
                    value: "this is".to_string()
                },
                0
            )
            .as_message()
            .unwrap()
            .value,
            "this is test"
        );
    }
}
