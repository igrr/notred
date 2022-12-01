use std::any::Any;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

use serde::{Deserialize, Serialize};

use crate::common::*;
use crate::node::*;
use crate::MessageType;

#[derive(Serialize, Deserialize, Debug)]
struct TickerNode {
    #[serde(flatten)]
    common: NodeCommon,
    period: DurationMsec,
    limit: Option<usize>,

    #[serde(skip)]
    thread_handle: Option<JoinHandle<()>>,
    #[serde(skip)]
    terminate_tx: Option<std::sync::mpsc::SyncSender<()>>,
}

#[typetag::serde(name = "ticker")]
impl Node for TickerNode {
    fn common(&self) -> &NodeCommon {
        &self.common
    }

    fn create(&mut self, event_sender: Option<Arc<Mutex<dyn EventSender>>>) {
        let period = self.period.to_duration();
        let event_sender = event_sender.unwrap().clone();
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        let name = self.common.name.clone();
        let mut limit = self.limit.clone();
        let mut count: i64 = 0;
        self.terminate_tx = Some(sender);
        self.thread_handle = Some(std::thread::spawn(move || loop {
            if receiver.recv_timeout(period).is_ok() {
                return;
            }
            let mut r = event_sender.lock().unwrap();
            r.dispatch(Event::MessageFrom(MessageFrom {
                message: MessageData::Int(count),
                from: NodePort {
                    name: name.clone(),
                    index: 0,
                },
            }));

            if let Some(mut lim) = limit {
                lim -= 1;
                limit = Some(lim);
                if lim == 0 {
                    return;
                }
            }
            count += 1;
        }));
    }

    fn run(&mut self, _msg: &Message, _index: usize) -> NodeFunctionResult {
        unreachable!("node has no inputs");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn num_inputs(&self) -> usize {
        0
    }

    fn num_outputs(&self) -> usize {
        1
    }

    fn input_type(&self, _index: usize) -> Option<&MessageType> {
        unreachable!("node has no inputs");
    }

    fn output_type(&self, index: usize) -> &MessageType {
        assert_eq!(index, 0);
        static OUTPUT_TYPE: MessageType = MessageType::Int;
        &OUTPUT_TYPE
    }
}

impl Drop for TickerNode {
    fn drop(&mut self) {
        /* The thread might have already terminated because it has reached the 'limit' */
        if self.terminate_tx.take().unwrap().send(()).is_err() {
            return;
        }
        if self.thread_handle.take().unwrap().join().is_err() {}
    }
}

#[cfg(test)]
mod test {
    use std::fmt::{Debug, Formatter};
    use std::thread;
    use std::time::Duration;

    use super::*;

    struct TestDispatcher {
        pub count: usize,
    }

    impl Debug for TestDispatcher {
        fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    impl EventSender for TestDispatcher {
        fn dispatch(&mut self, _e: Event) {
            self.count += 1;
        }
    }

    #[test]
    fn test_make_ticker_node() {
        let event_sender = Arc::new(Mutex::new(TestDispatcher { count: 0 }));
        let mut n: Box<dyn Node> = serde_json::from_str(
            r#"{
            "name": "node1",
            "class": "ticker",
            "period": 500
        }"#,
        )
        .unwrap();

        assert_eq!(n.common().name, "node1");
        n.create(Some(event_sender.clone()));
        thread::sleep(Duration::from_millis(1200));
        assert_eq!(event_sender.lock().unwrap().count, 2);
    }
}
