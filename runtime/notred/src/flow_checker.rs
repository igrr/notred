use crate::common::*;
use crate::errors::Error;
use crate::node_util::node_by_name;
use crate::{find_conversion, no_conversion};

pub fn check_flow(nodes: &Vec<Box<dyn Node>>, connections: &Vec<Connection>) -> Result<(), Error> {
    for c in connections {
        // Check that each connection's inputs and outputs exist
        match node_by_name(nodes, c.source.name.as_str()) {
            None => {
                return Result::Err(Error::InvalidNodeName(c.source.name.clone()));
            }
            Some(node) => {
                if c.source.index >= node.class().num_outputs {
                    return Result::Err(Error::InvalidPortIndex(
                        c.source.name.clone(),
                        c.source.index,
                    ));
                }
            }
        }
        match node_by_name(nodes, c.dest.name.as_str()) {
            None => {
                return Result::Err(Error::InvalidNodeName(c.dest.name.clone()));
            }
            Some(node) => {
                if c.dest.index >= node.class().num_inputs {
                    return Result::Err(Error::InvalidPortIndex(c.dest.name.clone(), c.dest.index));
                }
            }
        }
    }
    // TODO: warn if inputs or outputs are not connected?
    return Result::Ok(());
}

pub fn find_conversions(
    nodes: &Vec<Box<dyn Node>>,
    connections: &mut Vec<Connection>,
) -> Result<(), Error> {
    for c in connections {
        let source_node = node_by_name(nodes, c.source.name.as_str()).unwrap();
        let source_index = c.source.index;
        let source_message_type = source_node.output_type(source_index);

        let dest_node = node_by_name(nodes, c.dest.name.as_str()).unwrap();
        let dest_index = c.dest.index;
        if dest_node.input_type(dest_index).is_none() {
            c.conversion = Some(no_conversion);
            c.dest_type = Some(source_message_type.clone());
            continue;
        }
        let dest_message_type = dest_node.input_type(dest_index).as_ref().unwrap();

        let res = find_conversion(&source_message_type, &dest_message_type);
        match res {
            Ok(conv) => {
                c.conversion = Some(conv);
                c.dest_type = Some(dest_message_type.clone())
            }
            Err(e) => return Err(Error::ConversionError(e.to_string())),
        }
    }
    return Ok(());
}
