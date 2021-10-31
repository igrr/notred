use std::sync::{Arc, Mutex};
use std::time::Duration;

use json;

use crate::common::*;
use crate::errors::Error;
use crate::flow_checker::check_flow;
use crate::loader::JsonNodeLoader;
use crate::node_factory::DefaultNodeFactory;
use crate::node_util::{dest_node_of, node_by_name, node_by_name_mut};

pub struct MessageQueueItem {
    message: Message,
    from_node: String,
    output_index: usize,
}


#[derive(Debug)]
pub struct FlowState {
    nodes: Vec<Box<dyn Node>>,
    connections: Vec<Connection>,
    message_queue_rx: std::sync::mpsc::Receiver<MessageQueueItem>,
    dispatcher: Arc<Mutex<FlowAsyncMessageDispatcher>>,
}

#[derive(Debug)]
struct FlowAsyncMessageDispatcher {
    message_queue_tx: std::sync::mpsc::SyncSender<MessageQueueItem>,
}

impl AsyncMessageDispatcher for FlowAsyncMessageDispatcher {
    fn dispatch(&mut self, message: &Message, from_node: &str, source_output_index: usize) {
        self.message_queue_tx.send(MessageQueueItem {
            message: message.clone(),
            from_node: from_node.to_string(),
            output_index: source_output_index,
        }).unwrap();
    }
}

impl FlowState {
    pub fn from_json(json: &json::JsonValue) -> Result<FlowState, Error> {
        let (sender, receiver): (std::sync::mpsc::SyncSender<MessageQueueItem>, std::sync::mpsc::Receiver<MessageQueueItem>)
            = std::sync::mpsc::sync_channel(10); // FIXME
        let dispatcher = Arc::new(Mutex::new(FlowAsyncMessageDispatcher {
            message_queue_tx: sender
        }));

        let jl = JsonNodeLoader {};
        let factory = DefaultNodeFactory { async_dispatcher: Some(dispatcher.clone()) };
        let nodes = jl.load_nodes(&factory, json)?;
        let connections = jl.load_connections(&json)?;
        check_flow(&nodes, &connections)?;

        Ok(FlowState {
            nodes,
            connections,
            message_queue_rx: receiver,
            dispatcher: dispatcher.clone(),
        })
    }

    pub fn send(&mut self, message: &Message, from_node: &str, output_index: usize) -> Result<(), Error> {
        self.dispatcher.lock().unwrap().dispatch(message, from_node, output_index);
        Result::Ok(())
    }

    pub fn run_once(&mut self, timeout: Duration) -> Result<(), Error> {
        let mqi = self.message_queue_rx.recv_timeout(timeout)?;

        for c in &self.connections {
            if c.source != mqi.from_node || c.source_output_index != mqi.output_index {
                continue;
            }
            let dst_node = node_by_name_mut(&mut self.nodes, c.dest.as_str()).unwrap();
            let node_res = dst_node.run(&mqi.message);
            if let NodeFunctionResult::Success(msg) = node_res {
                self.dispatcher.lock().unwrap().dispatch(&msg, c.dest.as_str(), 0);
            }
        }
        Ok(())
    }

    pub fn get_node_by_name(&self, name: &str) -> Option<&Box<dyn Node>> {
        return node_by_name(&self.nodes, name);
    }

    pub fn create(&mut self) {
        for n in &mut self.nodes {
            n.create();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::nodes::CaptureNode;

    use super::*;

    #[test]
    fn test_create_flow() {
        let json_str = r#"
            {
                "nodes": [
                    {"class": "ticker", "name":"ticker1", "period":50, "limit":1},
                    {"class": "append", "name":"append1", "what_to_append":" test"},
                    {"class": "append", "name":"append2", "what_to_append":" test2"},
                    {"class": "capture", "name":"capture1"}
                ],
                "connections": [
                    {"source": "ticker1", "dest": "append1"},
                    {"source": "ticker1", "dest": "append2"},
                    {"source": "append1", "dest": "append2"},
                    {"source": "append2", "dest": "capture1"}
                ]
            }"#;
        let j = json::parse(json_str).unwrap();
        let mut flow = FlowState::from_json(&j).unwrap();
        assert_eq!(flow.connections.len(), 4);
        assert_eq!(flow.nodes.len(), 4);
        flow.create();

        for _i in 1..5 {
            let res = flow.run_once(Duration::from_millis(100));
            if let Ok(()) = res {
                continue;
            }
            if let Err(Error::Timeout(_)) = res {
                continue;
            }
            res.unwrap();
        }

        let capture_node = flow.get_node_by_name("capture1").unwrap().as_any().downcast_ref::<CaptureNode>().unwrap();
        let msgs = capture_node.get_captured_messages();
        assert_eq!(msgs.len(), 2);
        assert!(msgs.contains(&Message { value: " test2".to_string() }));
        assert!(msgs.contains(&Message { value: " test test2".to_string() }));
    }
}