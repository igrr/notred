mod append;

use super::common::NodeClass;
use append::APPEND_NODE_CLASS;
use counted_array::counted_array;

counted_array!(
    pub static NODE_CLASSES: [&NodeClass; _] = [
        &APPEND_NODE_CLASS
]);
