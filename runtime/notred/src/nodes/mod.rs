pub use capture::CaptureNode;
use counted_array::counted_array;

use super::common::NodeClass;

mod append;
mod capture;
mod ticker;

/* for tests */
counted_array!(
    pub static NODE_CLASSES: [&NodeClass; _] = [
        &append::APPEND_NODE_CLASS,
        &ticker::TICKER_NODE_CLASS,
        &capture::CAPTURE_NODE_CLASS
]);
