use crate::message::*;

pub fn find(src: &MessageType, dst: &MessageType) -> FindConversionResult {
    match dst {
        MessageType::Text(dst_content_type) => to_text(src, dst_content_type),
        MessageType::Binary(dst_content_type) => to_binary(src, dst_content_type),
        MessageType::Int => to_int(src),
        MessageType::Float => to_float(src),
        MessageType::Dict(dst_schema) => to_dict(src, dst_schema),
    }
}

fn to_text(src: &MessageType, _dst_content_type: &TextContentType) -> FindConversionResult {
    match src {
        MessageType::Text(_) => Some(identity),
        MessageType::Binary(_) => None,
        MessageType::Int => Some(int_to_text),
        MessageType::Float => Some(float_to_text),
        MessageType::Dict(_) => None,
    }
}

fn to_binary(src: &MessageType, _dst_content_type: &BinaryContentType) -> FindConversionResult {
    match src {
        MessageType::Text(_) => None,
        MessageType::Binary(_) => Some(identity),
        MessageType::Int => None,
        MessageType::Float => None,
        MessageType::Dict(_) => None,
    }
}

fn to_int(src: &MessageType) -> FindConversionResult {
    match src {
        MessageType::Text(_) => None,
        MessageType::Binary(_) => None,
        MessageType::Int => Some(identity),
        MessageType::Float => Some(float_to_int),
        MessageType::Dict(_) => None,
    }
}

fn to_float(src: &MessageType) -> FindConversionResult {
    match src {
        MessageType::Text(_) => None,
        MessageType::Binary(_) => None,
        MessageType::Int => Some(int_to_float),
        MessageType::Float => Some(identity),
        MessageType::Dict(_) => None,
    }
}

fn to_dict(src: &MessageType, dst_schema: &DictSchema) -> FindConversionResult {
    match src {
        MessageType::Text(_) => None,
        MessageType::Binary(_) => None,
        MessageType::Int => None,
        MessageType::Float => None,
        MessageType::Dict(src_schema) => from_dict_to_dict(src_schema, dst_schema),
    }
}

fn from_dict_to_dict(src: &DictSchema, dst: &DictSchema) -> FindConversionResult {
    for (key, mt_dst) in dst {
        match src.get(key) {
            // FIXME: return a user-readable explanation when no key is found.
            // FIXME: Should we default-initialize the missing values?
            None => return FindConversionResult::None,
            Some(mt_src) => {
                // FIXME: for deeply nested dictionaries, this recursion may overflow the stack.
                match find_conversion(mt_src, mt_dst) {
                    // found some conversion, continue with the next key
                    Some(_) => continue,
                    // FIXME: return a user-readable explanation
                    None => return FindConversionResult::None,
                }
            }
        }
    }
    // All the keys of the destination dict can be obtained from the keys in the source dict.
    // TODO: Should there be a warning if some key from the source dict is dropped?
    Some(dict_to_dict)
}

fn identity(src: &MessageData, _dst: &MessageType) -> ConversionResult {
    Ok(src.clone())
}

fn dict_to_dict(_src: &MessageData, _dst: &MessageType) -> ConversionResult {
    unimplemented!();
}

fn int_to_text(src: &MessageData, _dst: &MessageType) -> ConversionResult {
    if let MessageData::Int(val) = src {
        return Ok(MessageData::Text(Text {
            value: val.to_string(),
            content_type: TextContentType::Plain,
        }));
    }
    unreachable!("src should be an Int");
}

fn int_to_float(src: &MessageData, _dst: &MessageType) -> ConversionResult {
    if let MessageData::Int(val) = src {
        return Ok(MessageData::Float(*val as f32));
    }
    unreachable!("src should be a Int")
}

fn float_to_text(src: &MessageData, _dst: &MessageType) -> ConversionResult {
    if let MessageData::Float(val) = src {
        return Ok(MessageData::Text(Text {
            value: val.to_string(),
            content_type: TextContentType::Plain,
        }));
    }
    unreachable!("src should be a Float");
}

fn float_to_int(src: &MessageData, _dst: &MessageType) -> ConversionResult {
    if let MessageData::Float(val) = src {
        return Ok(MessageData::Int(*val as i64));
    }
    unreachable!("src should be a Float")
}

#[cfg(test)]
mod test {
    use crate::conversion;
    use crate::message::*;
    use crate::BinaryContentType::*;
    use crate::TextContentType::*;

    fn assert_bidirectional_conversion(src: MessageType, dst: MessageType) {
        assert!(conversion::find(&src, &dst).is_some());
        assert!(conversion::find(&dst, &src).is_some());
    }

    fn assert_has_conversion(src: MessageType, dst: MessageType) {
        assert!(conversion::find(&src, &dst).is_some())
    }

    fn assert_no_conversion(src: MessageType, dst: MessageType) {
        assert!(conversion::find(&src, &dst).is_none())
    }

    #[test]
    fn test_conversion_basic() {
        assert_bidirectional_conversion(MessageType::Int, MessageType::Int);
        assert_bidirectional_conversion(MessageType::Float, MessageType::Float);
        assert_bidirectional_conversion(MessageType::Text(Plain), MessageType::Text(Plain));
        assert_bidirectional_conversion(MessageType::Binary(Unknown), MessageType::Binary(Unknown));

        let empty_schema = DictSchema::new();

        assert_has_conversion(MessageType::Int, MessageType::Float);
        assert_has_conversion(MessageType::Int, MessageType::Text(Plain));
        assert_no_conversion(MessageType::Int, MessageType::Binary(Unknown));
        assert_no_conversion(MessageType::Int, MessageType::Dict(empty_schema.clone()));
        assert_has_conversion(MessageType::Float, MessageType::Int);
        assert_has_conversion(MessageType::Float, MessageType::Text(Plain));
        assert_no_conversion(MessageType::Float, MessageType::Binary(Unknown));
        assert_no_conversion(MessageType::Float, MessageType::Dict(empty_schema.clone()));
    }

    #[test]
    fn test_conversion_dict_simple() {
        let schema_src = DictSchema::from([("key".to_string(), MessageType::Int)]);
        let schema_dst = DictSchema::from([("key".to_string(), MessageType::Int)]);
        assert_bidirectional_conversion(
            MessageType::Dict(schema_src),
            MessageType::Dict(schema_dst),
        );
    }

    #[test]
    fn test_conversion_dict_mismatched_keys() {
        let schema_src = DictSchema::from([("key1".to_string(), MessageType::Int)]);
        let schema_dst = DictSchema::from([("key2".to_string(), MessageType::Int)]);
        assert_no_conversion(MessageType::Dict(schema_src), MessageType::Dict(schema_dst));
    }

    #[test]
    fn test_conversion_dict_implicit() {
        let schema_src = DictSchema::from([("key".to_string(), MessageType::Int)]);
        let schema_dst = DictSchema::from([("key".to_string(), MessageType::Float)]);
        assert_bidirectional_conversion(
            MessageType::Dict(schema_src),
            MessageType::Dict(schema_dst),
        );
    }

    #[test]
    fn test_conversion_dict_nested() {
        let schema_nested_src = DictSchema::from([("nested_key".to_string(), MessageType::Int)]);

        let schema_nested_dst = DictSchema::from([("nested_key".to_string(), MessageType::Float)]);

        let schema_src =
            DictSchema::from([("nested".to_string(), MessageType::Dict(schema_nested_src))]);
        let schema_dst =
            DictSchema::from([("nested".to_string(), MessageType::Dict(schema_nested_dst))]);

        assert_bidirectional_conversion(
            MessageType::Dict(schema_src),
            MessageType::Dict(schema_dst),
        );
    }
}
