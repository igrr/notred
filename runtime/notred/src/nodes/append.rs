use std::any::Any;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::common::*;
use crate::node::NodeFunctionResult;
use crate::node::*;
use crate::MessageType;
use crate::TextContentType::Plain;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AppendNode {
    #[serde(flatten)]
    common: NodeCommon,
    what_to_append: String,
}

static APPEND_MESSAGE_TYPE: MessageType = MessageType::Text(Plain);

#[typetag::serde(name = "append")]
impl Node for AppendNode {
    fn common(&self) -> &NodeCommon {
        &self.common
    }
    fn create(&mut self, _event_sender: Option<Arc<Mutex<dyn EventSender>>>) {}
    fn run(&mut self, msg: &Message, _index: usize) -> NodeFunctionResult {
        if let MessageData::Text(text) = msg {
            Ok(Some(MessageData::from_string(
                &(text.value.clone() + &self.what_to_append),
            )))
        } else {
            unimplemented!();
        }
    }

    fn destroy(&mut self) {}

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn num_inputs(&self) -> usize {
        1
    }

    fn num_outputs(&self) -> usize {
        1
    }

    fn input_type(&self, index: usize) -> Option<&MessageType> {
        assert_eq!(index, 0);
        Some(&APPEND_MESSAGE_TYPE)
    }

    fn output_type(&self, index: usize) -> &MessageType {
        assert_eq!(index, 0);
        &APPEND_MESSAGE_TYPE
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_append_node() {
        let node: Box<dyn Node> = serde_json::from_str(
            r#"{"class":"append", "name":"node1", "what_to_append": " test"}"#,
        )
        .unwrap();
        assert_eq!(node.common().name, "node1");
        // assert_eq!(
        //     n.run(&MessageData::from_str("this is"), 0)
        //         .as_message()
        //         .unwrap()
        //         .as_text()
        //         .unwrap(),
        //     "this is test"
        // );
    }
}
