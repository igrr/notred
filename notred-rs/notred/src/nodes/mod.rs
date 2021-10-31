mod append;
mod ticker;
mod capture;

use super::common::NodeClass;
use counted_array::counted_array;

/* for tests */
pub use capture::CaptureNode;

counted_array!(
    pub static NODE_CLASSES: [&NodeClass; _] = [
        &append::APPEND_NODE_CLASS,
        &ticker::TICKER_NODE_CLASS,
        &capture::CAPTURE_NODE_CLASS
]);
