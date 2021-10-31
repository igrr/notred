use crate::common::*;
use crate::errors::Error;
use crate::node_util::node_by_name;

pub fn check_flow(nodes: &Vec<Box<dyn Node>>, connections: &Vec<Connection>) -> Result<(), Error> {
    // Check that each connection's inputs and outputs exist
    for c in connections {
        match node_by_name(nodes, c.source.as_str()) {
            None => { return Result::Err(Error::InvalidNodeName(c.source.clone())); }
            Some(node) => {
                if c.source_output_index >= node.class().num_outputs {
                    return Result::Err(Error::InvalidOutputIndex(c.source.clone(), c.source_output_index));
                }
            }
        }
        match node_by_name(nodes, c.dest.as_str()) {
            None => { return Result::Err(Error::InvalidNodeName(c.source.clone())); }
            Some(node) => {
                if !node.class().has_input {
                    return Result::Err(Error::InvalidInput(c.source.clone(), c.dest.clone()));
                }
            }
        }
    }
    // TODO: warn if inputs or outputs are not connected?
    return Result::Ok(());
}

