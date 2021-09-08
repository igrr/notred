use crate::common::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;

#[derive(Debug)]
struct TickerNode {
    common: NodeCommonData,
    period: Duration,
    dispatcher: Arc<Mutex<dyn AsyncMessageDispatcher>>,
    thread_handle: Option<JoinHandle<()>>,
    terminate_tx: Option<std::sync::mpsc::SyncSender<()>>
}


fn make_ticker_node(
    common: NodeCommonData,
    opt_provider: &dyn NodeOptionsProvider,
    dispatcher: Option<Arc<Mutex<dyn AsyncMessageDispatcher>>>
) -> Result<Box<dyn Node>, NodeOptionsError> {
    let period = Duration::from_millis(opt_provider.get_usize("period")? as u64);
    let dispatcher = dispatcher.expect("dispatcher must be specified");
    Ok(Box::new(TickerNode {
        common,
        period,
        dispatcher,
        thread_handle: None,
        terminate_tx: None
    }))
}

pub static TICKER_NODE_CLASS: NodeClass = NodeClass {
    name: "ticker",
    constructor: make_ticker_node,
    has_input: false,
    num_outputs: 1,
};

impl NodeCommon for TickerNode {
    fn get_common(&self) -> &NodeCommonData {
        &self.common
    }
}

impl Node for TickerNode {
    fn class(&self) -> &NodeClass {
        &TICKER_NODE_CLASS
    }

    fn create(&mut self) {
        let period = self.period;
        let dispatcher = self.dispatcher.clone();
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        let name = self.common.name.clone();
        self.terminate_tx = Some(sender);
        self.thread_handle = Some(std::thread::spawn(move || {
            loop {
                if receiver.recv_timeout(period).is_ok() {
                    return;
                }
                let mut r = dispatcher.lock().unwrap();
                let msg = &Default::default();
                r.dispatch(msg, name.as_str());
            }
        }));
    }

    fn run(&mut self, _msg: &Message) -> NodeFunctionResult {
        panic!("node has no inputs, run shouldn't get called");
    }

    fn destroy(&mut self) {
        self.terminate_tx.take().unwrap().send(()).unwrap();
        self.thread_handle.take().unwrap().join().unwrap();
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::json_options_provider::JsonNodeOptionsProvider;
    use json;
    use std::fmt::{Debug, Formatter};

    struct TestDispatcher {
        pub count: usize
    }

    impl Debug for TestDispatcher {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    impl AsyncMessageDispatcher for TestDispatcher {
        fn dispatch(&mut self, _msg: &Message, _src_node_name: &str) {
            self.count += 1;
        }
    }


    #[test]
    fn test_make_ticker_node() {
        let dispatcher = Arc::new(Mutex::new(TestDispatcher { count: 0 }));
        let mut n = make_ticker_node(
            NodeCommonData {
                name: "node1".to_string(),
            },
            &JsonNodeOptionsProvider {
                data: &json::object! {"period": 500},
            },
            Some(dispatcher.clone())
        )
            .unwrap();
        assert_eq!(n.get_name(), "node1");
        n.create();
        thread::sleep(Duration::from_millis(1200));
        n.destroy();
        assert_eq!(dispatcher.lock().unwrap().count, 2);
    }
}

