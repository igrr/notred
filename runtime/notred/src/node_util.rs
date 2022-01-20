use crate::common::*;

pub fn node_by_name<'a>(nodes: &'a Vec<Box<dyn Node>>, name: &str) -> Option<&'a Box<dyn Node>> {
    for n in nodes {
        if n.get_name() == name {
            return Option::Some(n);
        }
    }
    return Option::None;
}

pub fn node_by_name_mut<'a>(
    nodes: &'a mut Vec<Box<dyn Node>>,
    name: &str,
) -> Option<&'a mut Box<dyn Node>> {
    for n in nodes {
        if n.get_name() == name {
            return Option::Some(n);
        }
    }
    return Option::None;
}
