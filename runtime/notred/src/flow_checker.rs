use crate::common::*;
use crate::errors::Error;
use crate::node_util::node_by_name;

pub fn check_flow(nodes: &Vec<Box<dyn Node>>, connections: &Vec<Connection>) -> Result<(), Error> {
    // Check that each connection's inputs and outputs exist
    for c in connections {
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
