pub fn notred_foo() -> i32 {
    42
}

use json;

struct Message {
    value: String,
}

enum NodeFunctionResult {
    Success(Message),
    NoResult,
}

// type NodeFunction = fn(&Message) -> NodeFunctionResult;

struct NodeClass {
    name: &'static str,
    constructor: fn() -> Box<dyn Node>,
    has_input: bool,
    num_outputs: usize
}

trait Node {
    fn create(&mut self) {}
    fn set_options(&mut self, opts_json: &str);
    fn get_options(&self) -> String;
    /* how to parse the options? */
    /* how to get the list of options? */
    /* how to save the options? */
    fn run(&mut self, msg: &Message) -> NodeFunctionResult;
    fn class(&self) -> &NodeClass;
    fn destroy(&mut self) {}
}

struct AppendNode {
    what_to_append: String
}

fn make_append_node() -> Box<dyn Node> {
    Box::new(AppendNode { what_to_append: "".to_string() })
}

static APPEND_NODE_CLASS: NodeClass = NodeClass {
    name: "Append",
    constructor: make_append_node,
    has_input: false,
    num_outputs: 0
};

impl Node for AppendNode {
    fn set_options(&mut self, opts_json: &str) {
        let obj = json::parse(opts_json);
        match obj {
            Ok(val) => {
                self.what_to_append = val["what_to_append"].as_str().unwrap_or("").to_string();
            },
            Err(e) => {}
        }
    }

    fn get_options(&self) -> String {
        json::stringify(json::object! {
            what_to_append: self.what_to_append.as_str()
        })
    }

    fn run(&mut self, msg: &Message) -> NodeFunctionResult {
        NodeFunctionResult::Success(Message {
                value: msg.value.clone() + &self.what_to_append
            }
        )
    }

    fn class(&self) -> &NodeClass {
        &APPEND_NODE_CLASS
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stuff() {
        let node = (APPEND_NODE_CLASS.constructor)();

    }

}
