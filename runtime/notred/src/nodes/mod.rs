mod append;
mod capture;
mod ticker;

use counted_array::counted_array;

use super::common::NodeClass;

pub use append::APPEND_NODE_CLASS;
pub use capture::CAPTURE_NODE_CLASS;
pub use ticker::TICKER_NODE_CLASS;

pub use capture::CaptureNode;

/* for tests */
counted_array!(
    pub static NODE_CLASSES: [&NodeClass; _] = [
        &APPEND_NODE_CLASS,
        &TICKER_NODE_CLASS,
        &CAPTURE_NODE_CLASS
]);
