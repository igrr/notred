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
