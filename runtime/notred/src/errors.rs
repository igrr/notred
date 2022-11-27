use std::sync::mpsc::RecvTimeoutError;

use quick_error::quick_error;
use serde_json;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        JsonLoad(err: serde_json::Error) {
            from()
        }
        InvalidNodeName(name: String) {
            display("Invalid node name: {}", name)
        }
        InvalidPortIndex(name: String, index: usize) {
            display("Invalid port index ({}.{})", name, index)
        }
        Timeout(err: RecvTimeoutError) {
            from()
        }
        Terminate(reason: String) {
            display("Execution terminated: {}", reason)
        }
        ConversionError(reason: String) {
            display("Conversion error: {}", reason)
        }

    }
}
