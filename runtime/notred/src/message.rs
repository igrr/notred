use crate::conversion;
use std::collections::HashMap;

/// MessageType encodes various types of messages.
///
/// MessageType holds the _type_ of the message, while the very similar MessageData enum holds
/// the actual message. MessageType and MessageData have similarly named variants, although there is
/// no relationship between them in the type system.
///
/// The following overview of the message types applies to both MessageType and MessageData:
///
/// - Int: 64-bit signed integer
/// - Float: 32-bit floating point number
/// - Text: Unicode string (i.e. String); has an additional property "content_type" which is a hint
///   about the nature of the text.
/// - Binary: vector of bytes; similar to Text, has a "content_type" property to hint what kind of
///   binary data is held inside.
/// - Dict: in MessageType, a dictionary (map) from string keys to MessageType values. This map is
///   also known as "schema". In MessageData, the Dict is a map from string keys to MessageData
///   values. The type of data for the given key matches the type indicated in the schema.
///
///
#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    Text(TextContentType),
    Binary(BinaryContentType),
    Int,
    Float,
    Dict(DictSchema),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageData {
    Text(Text),
    Binary(Binary),
    Int(i64),
    Float(f32),
    Dict(Dict),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextContentType {
    Plain,
    Json,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryContentType {
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
    pub value: String,
    pub content_type: TextContentType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binary {
    pub value: Vec<u8>,
    pub content_type: BinaryContentType,
}

pub type DictSchema = HashMap<String, MessageType>;

#[derive(Debug, Clone, PartialEq)]
pub struct Dict {
    pub data: HashMap<String, MessageData>,
    pub schema: DictSchema,
}

pub struct ConversionError {
    // TODO:
    // src: MessageData,
    // dst_type: MessageType,
    // err_msg: String,
}

pub type ConversionResult = Result<MessageData, ConversionError>;
pub type MessageConverter = fn(src: &MessageData, dst: &MessageType) -> ConversionResult;
pub type FindConversionResult = Option<MessageConverter>;

pub fn find_conversion(src: &MessageType, dst: &MessageType) -> FindConversionResult {
    conversion::find(src, dst)
}
