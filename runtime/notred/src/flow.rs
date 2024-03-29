use std::sync::{Arc, Mutex};
use std::time::Duration;

use log::*;

use crate::common::*;
use crate::errors::Error;
use crate::flow_checker::{check_flow, find_conversions};
use crate::loader;
use crate::node::Node;
use crate::node_util::{node_by_name, node_by_name_mut};

#[derive(Debug)]
pub struct FlowState {
    nodes: Vec<Box<dyn Node>>,
    connections: Vec<Connection>,
    message_queue_rx: std::sync::mpsc::Receiver<Event>,
    event_sender: Arc<Mutex<dyn EventSender>>,
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
    pub fn new(text: &str) -> Result<FlowState, Error> {
        let (sender, receiver): (
            std::sync::mpsc::SyncSender<Event>,
            std::sync::mpsc::Receiver<Event>,
        ) = std::sync::mpsc::sync_channel(10); // FIXME
        let event_sender = Arc::new(Mutex::new(FlowAsyncMessageDispatcher {
            message_queue_tx: sender,
        }));

        let lfd = loader::LoadedFlowDescription::new(text)?;
        let mut nodes = lfd.nodes;
        for n in &mut nodes {
            n.create(Some(event_sender.clone()));
        }
        let mut connections = lfd.connections;
        check_flow(&nodes, &connections)?;
        find_conversions(&nodes, &mut connections)?;

        Ok(FlowState {
            nodes,
            connections,
            message_queue_rx: receiver,
            event_sender: event_sender.clone(),
        })
    }

    fn handle_message_to(&mut self, mt: MessageTo) {
        let dst_node = node_by_name_mut(&mut self.nodes, mt.to.name.as_str()).unwrap();
        if dst_node.common().log_outputs {
            if dst_node.num_inputs() == 1 {
                info!("Input to {}: {}", mt.to.name, mt.message);
            } else {
                info!("Input to {}[{}]: {}", mt.to.name, mt.to.index, mt.message);
            }
        }
        let node_res = dst_node.run(&mt.message, mt.to.index);
        if let Ok(Some(msg)) = node_res {
            self.event_sender
                .lock()
                .unwrap()
                .dispatch(Event::MessageFrom(MessageFrom {
                    from: NodePort {
                        name: mt.to.name.clone(),
                        index: 0,
                    },
                    message: msg,
                }));
        }
    }

    fn handle_message_from(&mut self, mf: MessageFrom) {
        if let Some(src_node) = node_by_name_mut(&mut self.nodes, mf.from.name.as_str()) {
            if src_node.common().log_outputs {
                if src_node.num_outputs() == 1 {
                    info!("Output from {}: {}", mf.from.name, mf.message)
                } else {
                    info!(
                        "Output from {}[{}]: {}",
                        mf.from.name, mf.from.index, mf.message
                    )
                }
            }
        }

        for c in &self.connections {
            if c.source != mf.from {
                continue;
            }

            match c.conversion.unwrap()(&mf.message, &c.dest_type.clone().unwrap()) {
                Ok(converted_message) => {
                    self.event_sender
                        .lock()
                        .unwrap()
                        .dispatch(Event::MessageTo(MessageTo {
                            message: converted_message,
                            to: c.dest.clone(),
                        }))
                }
                Err(e) => self
                    .event_sender
                    .lock()
                    .unwrap()
                    .dispatch(Event::Log(e.to_string())),
            }
        }
    }

    fn handle_log(&mut self, log_msg: String) {
        info!("{log_msg}");
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
}

#[cfg(test)]
mod test {
    use crate::nodes::capture::CaptureNode;

    use super::*;

    #[test]
    fn test_create_flow() {
        let json_str = r#"
            {
                "nodes": [
                    {"class": "ticker", "name":"ticker1", "period": 50, "limit":1},
                    {"class": "append", "name":"append1", "what_to_append":" test"},
                    {"class": "append", "name":"append2", "what_to_append":" test2"},
                    {"class": "capture", "name":"capture1"}
                ],
                "connections": [
                    {"source": {"name":"ticker1"}, "dest": {"name": "append1"}},
                    {"source": {"name":"ticker1"}, "dest": {"name":"append2"}},
                    {"source": {"name":"append1"}, "dest": {"name":"append2"}},
                    {"source": {"name":"append2"}, "dest": {"name":"capture1"}}
                ]
            }"#;

        let mut flow = FlowState::new(json_str).unwrap();
        assert_eq!(flow.connections.len(), 4);
        assert_eq!(flow.nodes.len(), 4);

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
        assert!(msgs.contains(&Message::from_str("0 test2")));
        assert!(msgs.contains(&Message::from_str("0 test test2")));
    }
}
