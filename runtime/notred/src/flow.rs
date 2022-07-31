use std::sync::{Arc, Mutex};
use std::time::Duration;

use json;
use log::*;

use crate::common::*;
use crate::errors::Error;
use crate::flow_checker::check_flow;
use crate::loader::JsonNodeLoader;
use crate::node_util::{node_by_name, node_by_name_mut};

#[derive(Debug)]
pub struct FlowState {
    nodes: Vec<Box<dyn Node>>,
    connections: Vec<Connection>,
    message_queue_rx: std::sync::mpsc::Receiver<Event>,
    event_sender: Arc<Mutex<FlowAsyncMessageDispatcher>>,
}

#[derive(Debug)]
pub struct FlowAsyncMessageDispatcher {
    message_queue_tx: std::sync::mpsc::SyncSender<Event>,
}

impl EventSender for FlowAsyncMessageDispatcher {
    fn dispatch(&mut self, e: Event) {
        self.message_queue_tx.send(e).unwrap();
    }
}

impl FlowState {
    pub fn from_json(
        json: &json::JsonValue,
        node_factory: &dyn NodeFactory,
    ) -> Result<FlowState, Error> {
        let (sender, receiver): (
            std::sync::mpsc::SyncSender<Event>,
            std::sync::mpsc::Receiver<Event>,
        ) = std::sync::mpsc::sync_channel(10); // FIXME
        let event_sender = Arc::new(Mutex::new(FlowAsyncMessageDispatcher {
            message_queue_tx: sender,
        }));

        let jl = JsonNodeLoader {};
        let nodes = jl.load_nodes(json, node_factory, Some(event_sender.clone()))?;
        let connections = jl.load_connections(&json)?;
        check_flow(&nodes, &connections)?;

        Ok(FlowState {
            nodes,
            connections,
            message_queue_rx: receiver,
            event_sender: event_sender.clone(),
        })
    }

    fn handle_message_to(&mut self, mt: MessageTo) {
        let dst_node = node_by_name_mut(&mut self.nodes, mt.to.name.as_str()).unwrap();
        if dst_node.should_log_inputs() {
            info!("Input to {}:{}: '{}'", mt.to.name, mt.to.index, mt.message);
        }
        let node_res = dst_node.run(&mt.message);
        if let NodeFunctionResult::Success(msg) = node_res {
            self.event_sender
                .lock()
                .unwrap()
                .dispatch(Event::MessageFrom(MessageFrom {
                    from: NodeIO {
                        name: mt.to.name.clone(),
                        index: 0,
                    },
                    message: msg,
                }));
        }
    }

    fn handle_message_from(&mut self, mf: MessageFrom) {
        if let Some(src_node) = node_by_name_mut(&mut self.nodes, mf.from.name.as_str()) {
            if src_node.should_log_outputs() {
                info!(
                    "Output from {}:{}: '{}'",
                    mf.from.name, mf.from.index, mf.message
                )
            }
        }

        for c in &self.connections {
            if c.source != mf.from.name || c.source_output_index != mf.from.index {
                continue;
            }

            self.event_sender
                .lock()
                .unwrap()
                .dispatch(Event::MessageTo(MessageTo {
                    message: mf.message.clone(),
                    to: NodeIO {
                        name: c.dest.clone(),
                        index: 0, // FIXME
                    },
                }));
        }
    }

    fn handle_log(&mut self, _log: String) {
        unimplemented!();
    }

    pub fn run_once(&mut self, timeout: Duration) -> Result<(), Error> {
        let e = self.message_queue_rx.recv_timeout(timeout)?;

        match e {
            Event::MessageTo(mt) => {
                self.handle_message_to(mt);
            }
            Event::MessageFrom(mf) => {
                self.handle_message_from(mf);
            }
            Event::Log(log) => {
                self.handle_log(log);
            }
            Event::Terminate() => {
                return Result::Err(Error::Terminate("received termination message".to_string()));
            }
        };

        Ok(())
    }

    pub fn get_node_by_name(&self, name: &str) -> Option<&Box<dyn Node>> {
        return node_by_name(&self.nodes, name);
    }

    pub fn get_node_by_name_mut(&mut self, name: &str) -> Option<&mut Box<dyn Node>> {
        return node_by_name_mut(&mut self.nodes, name);
    }

    pub fn create(&mut self) {
        for n in &mut self.nodes {
            n.create();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::node_factory;
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
        let factory = node_factory::DefaultNodeFactory::default();
        let mut flow = FlowState::from_json(&j, &factory).unwrap();
        assert_eq!(flow.connections.len(), 4);
        assert_eq!(flow.nodes.len(), 4);
        flow.create();

        for _i in 1..10 {
            let res = flow.run_once(Duration::from_millis(100));
            if let Ok(()) = res {
                continue;
            }
            if let Err(Error::Timeout(_)) = res {
                continue;
            }
            res.unwrap();
        }

        let capture_node = flow
            .get_node_by_name("capture1")
            .unwrap()
            .as_any()
            .downcast_ref::<CaptureNode>()
            .unwrap();
        let msgs = capture_node.get_captured_messages();
        assert_eq!(msgs.len(), 2);
        assert!(msgs.contains(&Message {
            value: " test2".to_string()
        }));
        assert!(msgs.contains(&Message {
            value: " test test2".to_string()
        }));
    }
}
