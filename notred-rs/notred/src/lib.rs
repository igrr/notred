use json;

struct Message {
    value: String,
}

enum NodeFunctionResult {
    Success(Message),
    NoResult(),
}

struct NodeClass {
    name: &'static str,
    constructor: fn() -> Box<dyn Node>,
    has_input: bool,
    num_outputs: usize,
}

trait Node {
    fn create(&mut self) {}
    fn set_options(&mut self, opts_json: &str) -> Result<(), NodeOptionsError>;
    /* how to get the list of options? */
    fn get_options(&self) -> String;
    fn run(&mut self, msg: &Message) -> NodeFunctionResult;
    fn class(&self) -> &NodeClass;
    fn destroy(&mut self) {}
}

struct AppendNode {
    what_to_append: String,
}

fn make_append_node() -> Box<dyn Node> {
    Box::new(AppendNode {
        what_to_append: "".to_string(),
    })
}

static APPEND_NODE_CLASS: NodeClass = NodeClass {
    name: "Append",
    constructor: make_append_node,
    has_input: false,
    num_outputs: 0,
};

#[derive(Debug, Clone)]
struct NodeOptionsError;

impl Node for AppendNode {
    fn set_options(&mut self, opts_json: &str) -> Result<(), NodeOptionsError> {
        let obj = json::parse(opts_json);
        match obj {
            Ok(val) => {
                self.what_to_append = val["what_to_append"].as_str().unwrap_or("").to_string();
                Result::Ok(())
            }
            Err(_e) => Result::Err(NodeOptionsError {}),
        }
    }

    fn get_options(&self) -> String {
        json::stringify(json::object! {
            what_to_append: self.what_to_append.as_str()
        })
    }

    fn run(&mut self, msg: &Message) -> NodeFunctionResult {
        NodeFunctionResult::Success(Message {
            value: msg.value.clone() + &self.what_to_append,
        })
    }

    fn class(&self) -> &NodeClass {
        &APPEND_NODE_CLASS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_node_set_options() {
        let mut node = AppendNode {
            what_to_append: "".to_string()
        };
        assert!(node.set_options("{}").is_ok());
        assert!(node.set_options("{").is_err());
        assert!(node.set_options("{\"foo\": 42}").is_ok());
        assert!(node.set_options("{\"what_to_append\": 42}").is_ok());
    }

    #[test]
    fn test_stuff() {
        let mut node = (APPEND_NODE_CLASS.constructor)();
        let res1 = (*node).set_options("{\"what_to_append\":\" foo\"}");
        match res1 {
            Ok(()) => {}
            Err(_e) => {
                assert!(false)
            }
        };
        let m1 = Message {
            value: "test".to_string(),
        };
        let res = (*node).run(&m1);
        match res {
            NodeFunctionResult::Success(m) => {
                assert_eq!(m.value.as_str(), "test foo")
            },
            NodeFunctionResult::NoResult() => {
                assert!(false);
            }
        }
    }
}
