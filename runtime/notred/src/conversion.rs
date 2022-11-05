use crate::message::FindConversionError::*;
use crate::message::MessageData as MD;
use crate::message::MessageType as MT;
use crate::message::*;

pub fn find(src: &MT, dst: &MT) -> FindConversionResult {
    match dst {
        MT::Text(dst_content_type) => to_text(src, dst_content_type),
        MT::Binary(dst_content_type) => to_binary(src, dst_content_type),
        MT::Int => to_int(src),
        MT::Float => to_float(src),
        MT::Dict(dst_schema) => to_dict(src, dst_schema),
    }
}

fn to_text(src: &MT, dst_content_type: &TextContentType) -> FindConversionResult {
    match src {
        MT::Text(_) => Ok(identity),
        MT::Binary(_) => Err(NoImplicitConversion),
        MT::Int => Ok(int_to_text),
        MT::Float => Ok(float_to_text),
        MT::Dict(_) => match dst_content_type {
            TextContentType::Json => Ok(dict_to_text_json),
            _ => Err(NoImplicitConversionDetailed(
                format!("Can't convert dictionary to text with content-type {}. Try setting content-type to JSON.", MT::Text(dst_content_type.clone()))
            ))
        },
    }
}

fn to_binary(src: &MT, _dst_content_type: &BinaryContentType) -> FindConversionResult {
    match src {
        MT::Text(_) => Err(NoImplicitConversion),
        MT::Binary(_) => Ok(identity),
        MT::Int => Err(NoImplicitConversion),
        MT::Float => Err(NoImplicitConversion),
        MT::Dict(_) => Err(NoImplicitConversion),
    }
}

fn to_int(src: &MT) -> FindConversionResult {
    match src {
        MT::Text(_) => Ok(text_to_int),
        MT::Binary(_) => Err(NoImplicitConversion),
        MT::Int => Ok(identity),
        MT::Float => Ok(float_to_int),
        MT::Dict(_) => Err(NoImplicitConversion),
    }
}

fn to_float(src: &MT) -> FindConversionResult {
    match src {
        MT::Text(_) => Ok(text_to_float),
        MT::Binary(_) => Err(NoImplicitConversion),
        MT::Int => Ok(int_to_float),
        MT::Float => Ok(identity),
        MT::Dict(_) => Err(NoImplicitConversion),
    }
}

fn to_dict(src: &MT, dst_schema: &DictSchema) -> FindConversionResult {
    match src {
        MT::Text(_) => Err(NoImplicitConversion),
        MT::Binary(_) => Err(NoImplicitConversion),
        MT::Int => Err(NoImplicitConversion),
        MT::Float => Err(NoImplicitConversion),
        MT::Dict(src_schema) => from_dict_to_dict(src_schema, dst_schema),
    }
}

fn from_dict_to_dict(src: &DictSchema, dst: &DictSchema) -> FindConversionResult {
    for (key, mt_dst) in dst {
        match src.get(key) {
            None => return Err(NoImplicitConversionDetailed(
                format!("Key '{key}' is present in the destination dictionary but not in the source dictionary")
            )),
            Some(mt_src) => {
                // FIXME: for deeply nested dictionaries, this recursion may overflow the stack.
                match find_conversion(mt_src, mt_dst) {
                    // found some conversion, continue with the next key
                    Ok(_) => continue,
                    Err(e) => return Err(NoImplicitConversionDetailed(
                        format!("Couldn't convert key {key} from type {mt_src} to type {mt_dst}: {e}")
                    )),
                }
            }
        }
    }
    // All the keys of the destination dict can be obtained from the keys in the source dict.
    // TODO: Should there be a warning if some key from the source dict is dropped?
    Ok(dict_to_dict)
}

fn identity(src: &MessageData, _dst: &MT) -> ConversionResult {
    Ok(src.clone())
}

fn dict_to_dict(_src: &MessageData, _dst: &MT) -> ConversionResult {
    unimplemented!();
}

fn int_to_text(src: &MessageData, _dst: &MT) -> ConversionResult {
    if let MD::Int(val) = src {
        return Ok(MD::Text(Text {
            value: val.to_string(),
            content_type: TextContentType::Plain,
        }));
    }
    unreachable!("src should be an Int");
}

fn int_to_float(src: &MessageData, _dst: &MT) -> ConversionResult {
    if let MD::Int(val) = src {
        return Ok(MD::Float(*val as f32));
    }
    unreachable!("src should be a Int")
}

fn float_to_text(src: &MessageData, _dst: &MT) -> ConversionResult {
    if let MD::Float(val) = src {
        return Ok(MD::Text(Text {
            value: val.to_string(),
            content_type: TextContentType::Plain,
        }));
    }
    unreachable!("src should be a Float");
}

fn float_to_int(src: &MessageData, _dst: &MT) -> ConversionResult {
    if let MD::Float(val) = src {
        return Ok(MD::Int(*val as i64));
    }
    unreachable!("src should be a Float")
}

fn text_to_int(src: &MessageData, _dst: &MT) -> ConversionResult {
    if let MD::Text(text) = src {
        return match text.value.parse::<i64>() {
            Ok(res) => Ok(MD::Int(res)),
            Err(_) => Err(ConversionError {}), // FIXME: pass the error
        };
    }
    unreachable!("src should be Text")
}

fn text_to_float(src: &MessageData, _dst: &MT) -> ConversionResult {
    if let MD::Text(text) = src {
        return match text.value.parse::<f32>() {
            Ok(res) => Ok(MD::Float(res)),
            Err(_) => Err(ConversionError {}), // FIXME: pass the error
        };
    }
    unreachable!("src should be Text")
}

fn dict_to_text_json(_src: &MessageData, _dst: &MT) -> ConversionResult {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use crate::conversion;
    use crate::message::MessageData as MD;
    use crate::message::MessageType as MT;
    use crate::message::*;
    use crate::BinaryContentType::*;
    use crate::TextContentType::*;

    fn assert_has_bidirectional_conversion(src: MT, dst: MT) {
        assert!(conversion::find(&src, &dst).is_ok());
        assert!(conversion::find(&dst, &src).is_ok());
    }

    fn assert_has_conversion(src: MT, dst: MT) {
        assert!(conversion::find(&src, &dst).is_ok())
    }

    fn assert_has_no_conversion(src: MT, dst: MT) {
        assert!(conversion::find(&src, &dst).is_err())
    }

    #[test]
    fn test_conversion_basic() {
        assert_has_bidirectional_conversion(MT::Int, MT::Int);
        assert_has_bidirectional_conversion(MT::Float, MT::Float);
        assert_has_bidirectional_conversion(MT::Text(Plain), MT::Text(Plain));
        assert_has_bidirectional_conversion(MT::Binary(Unknown), MT::Binary(Unknown));

        let empty_schema = DictSchema::new();

        assert_has_bidirectional_conversion(MT::Int, MT::Float);
        assert_has_bidirectional_conversion(MT::Int, MT::Text(Plain));
        assert_has_bidirectional_conversion(MT::Float, MT::Text(Plain));
        assert_has_no_conversion(MT::Int, MT::Binary(Unknown));
        assert_has_no_conversion(MT::Int, MT::Dict(empty_schema.clone()));
        assert_has_no_conversion(MT::Float, MT::Binary(Unknown));
        assert_has_no_conversion(MT::Float, MT::Dict(empty_schema.clone()));
        assert_has_no_conversion(MT::Text(Plain), MT::Binary(Unknown));
        assert_has_no_conversion(MT::Binary(Unknown), MT::Text(Plain));

        assert_has_conversion(MT::Dict(empty_schema), MT::Text(Json));
    }

    fn assert_conversion_result(src_type: MT, src: MD, dst_type: MT, dst_expected: MD) {
        let conv = conversion::find(&src_type, &dst_type);
        assert!(conv.is_ok());
        let dst_actual = conv.unwrap()(&src, &dst_type);
        assert!(dst_actual.is_ok());
        assert_eq!(dst_actual.unwrap(), dst_expected);
    }

    fn assert_conversion_error(src_type: MT, src: MD, dst_type: MT) {
        let conv = conversion::find(&src_type, &dst_type);
        assert!(conv.is_ok());
        let dst_actual = conv.unwrap()(&src, &dst_type);
        assert!(dst_actual.is_err());
    }

    fn make_md_text_plain(text: &str) -> MD {
        MD::Text(Text {
            value: text.to_string(),
            content_type: Plain,
        })
    }

    #[test]
    fn test_conversion_result() {
        assert_conversion_result(
            MT::Int,
            MD::Int(42),
            MT::Text(Plain),
            make_md_text_plain("42"),
        );

        assert_conversion_error(MT::Text(Plain), make_md_text_plain("aaa"), MT::Int);
    }

    #[test]
    fn test_conversion_dict_simple() {
        let schema_src = DictSchema::from([("key".to_string(), MT::Int)]);
        let schema_dst = DictSchema::from([("key".to_string(), MT::Int)]);
        assert_has_bidirectional_conversion(MT::Dict(schema_src), MT::Dict(schema_dst));
    }

    #[test]
    fn test_conversion_dict_mismatched_keys() {
        let schema_src = DictSchema::from([("key1".to_string(), MT::Int)]);
        let schema_dst = DictSchema::from([("key2".to_string(), MT::Int)]);
        assert_has_no_conversion(MT::Dict(schema_src), MT::Dict(schema_dst));
    }

    #[test]
    fn test_conversion_dict_implicit() {
        let schema_src = DictSchema::from([("key".to_string(), MT::Int)]);
        let schema_dst = DictSchema::from([("key".to_string(), MT::Float)]);
        assert_has_bidirectional_conversion(MT::Dict(schema_src), MT::Dict(schema_dst));
    }

    #[test]
    fn test_conversion_dict_nested() {
        let schema_nested_src = DictSchema::from([("nested_key".to_string(), MT::Int)]);

        let schema_nested_dst = DictSchema::from([("nested_key".to_string(), MT::Float)]);

        let schema_src = DictSchema::from([("nested".to_string(), MT::Dict(schema_nested_src))]);
        let schema_dst = DictSchema::from([("nested".to_string(), MT::Dict(schema_nested_dst))]);

        assert_has_bidirectional_conversion(MT::Dict(schema_src), MT::Dict(schema_dst));
    }
}
