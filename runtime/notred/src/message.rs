use crate::conversion;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

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

impl Display for MessageType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res: &str = match &self {
            MessageType::Text(tct) => match tct {
                TextContentType::Plain => "text/plain",
                TextContentType::Json => "text/json",
            },
            MessageType::Binary(bct) => match bct {
                BinaryContentType::Unknown => "binary/unknown",
            },
            MessageType::Int => "integer",
            MessageType::Float => "float",
            MessageType::Dict(_) => "dictionary",
        };
        f.write_str(res)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageData {
    Text(Text),
    Binary(Binary),
    Int(i64),
    Float(f32),
    Dict(Dict),
}

impl MessageData {
    pub fn as_text(&self) -> Option<&String> {
        if let MessageData::Text(t) = &self {
            Some(&t.value)
        } else {
            None
        }
    }
    pub fn from_str(text: &str) -> MessageData {
        MessageData::Text(Text {
            value: text.to_string(),
            content_type: TextContentType::Plain,
        })
    }

    pub fn from_string(text: &String) -> MessageData {
        MessageData::Text(Text {
            value: text.clone(),
            content_type: TextContentType::Plain,
        })
    }
}

impl Display for MessageData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = match &self {
            MessageData::Text(t) => format!("\"{}\"", t.value),
            MessageData::Binary(_) => format!("<binary data>"), // FIXME
            MessageData::Int(i) => format!("{i}"),
            MessageData::Float(f) => format!("{f}"),
            MessageData::Dict(_) => format!("<dictionary>"), // FIXME
        };
        f.write_str(res.as_str())
    }
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

#[derive(Debug, Clone)]
pub enum FindConversionError {
    NoImplicitConversion,
    ConversionNotImplemented,
    NoImplicitConversionDetailed(String),
}

impl Display for FindConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let res = match &self {
            FindConversionError::NoImplicitConversion => "no implicit conversion".to_string(),
            FindConversionError::ConversionNotImplemented => {
                "conversion not implemented".to_string()
            }
            FindConversionError::NoImplicitConversionDetailed(details) => {
                format!("no implicit conversion: {details}")
            }
        };
        f.write_str(res.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct ConversionError {
    // TODO:
    // src: MessageData,
    // dst_type: MessageType,
    // err_msg: String,
}

impl Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Failed to convert")
    }
}

pub type ConversionResult = Result<MessageData, ConversionError>;
pub type MessageConverter = fn(src: &MessageData, dst: &MessageType) -> ConversionResult;
pub type FindConversionResult = Result<MessageConverter, FindConversionError>;

pub fn find_conversion(src: &MessageType, dst: &MessageType) -> FindConversionResult {
    conversion::find(src, dst)
}

pub fn no_conversion(src: &MessageData, dst: &MessageType) -> ConversionResult {
    return conversion::identity(src, dst);
}
