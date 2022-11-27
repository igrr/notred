use core::fmt;
use std::fmt::{Debug, Formatter};
use std::time::Duration;

use serde::{Deserialize, Serialize};

pub use crate::message::{MessageConverter, MessageData};
use crate::MessageType;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct DurationMsec(u64);

impl DurationMsec {
    pub fn to_duration(&self) -> Duration {
        Duration::from_millis(self.0)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct NodePort {
    pub name: String,
    #[serde(default)]
    pub index: usize,
}

pub type Message = MessageData;

// FIXME: rename to Message
#[derive(Debug, Clone, PartialEq)]
pub struct MessageTo {
    pub message: Message,
    pub to: NodePort,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageFrom {
    pub message: Message,
    pub from: NodePort,
}

pub enum Event {
    MessageTo(MessageTo),
    MessageFrom(MessageFrom),
    Log(String),
    Terminate(),
}

pub trait EventSender: fmt::Debug + Send {
    fn dispatch(&mut self, e: Event);
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Connection {
    #[serde(skip_serializing)]
    pub source: NodePort,
    #[serde(skip_serializing)]
    pub dest: NodePort,
    #[serde(skip)]
    pub conversion: Option<MessageConverter>,
    #[serde(skip)]
    pub dest_type: Option<MessageType>,
}

impl Debug for Connection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection")
            .field("source", &self.source)
            .field("dest", &self.dest)
            .finish()
    }
}
