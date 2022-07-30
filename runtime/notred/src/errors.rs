use std::sync::mpsc::RecvTimeoutError;

use json;
use quick_error::quick_error;

use crate::common::*;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Json(err: json::Error) {
            from()
        }
        NodeOptions(err: NodeOptionsError) {
            from()
        }
        ClassNotFound(classname: String) {
            display("Class not found: {}", classname)
        }
        FieldMissing(field: &'static str) {
            display("Field missing: {}", field)
        }
        ValueError(value: String) {
            display("Invalid value: {}", value)
        }
        InvalidNodeName(name: String) {
            display("Invalid node name: {}", name)
        }
        InvalidOutputIndex(name: String, index: usize) {
            display("Invalid output index {} for node {}", index, name)
        }
        InvalidInput(from: String, to: String) {
            display("Can't connect {} to {}, which has no inputs", from, to)
        }
        Timeout(err: RecvTimeoutError) {
            from()
        }
        Terminate(reason: String) {
            display("Execution terminated: {}", reason)
        }

    }
}
