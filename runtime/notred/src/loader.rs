use json;

use crate::common::*;
use crate::errors::Error;
use crate::JsonNodeOptionsProvider;

pub struct JsonNodeLoader {}

// FIXME: figure out how to avoid passing 'factory' and 'event_sender' here.
// Passing 'create_node' closure didn't work well since the closure captured event_sender
// before cloning it, so was FnOnce instead of Fn.

impl JsonNodeLoader {
    pub fn load_nodes<T>(
        &self,
        j: &json::JsonValue,
        create_node: T,
    ) -> Result<Vec<Box<dyn Node>>, Error>
    where
        T: Fn(&str, &str, &dyn NodeOptionsProvider) -> Option<Box<dyn Node>>,
    {
        let nodes_array = &j["nodes"];
        let mut nodes: Vec<Box<dyn Node>> = Vec::new();

        for e in nodes_array.members() {
            let class_name = match e["class"].as_str() {
                Some(n) => n,
                None => {
                    return Result::Err(Error::FieldMissing("class"));
                }
            };
            let name = match e["name"].as_str() {
                Some(n) => n,
                None => {
                    return Result::Err(Error::FieldMissing("name"));
                }
            };
            let res = create_node(class_name, name, &JsonNodeOptionsProvider { data: &e })
                .expect("failed to load node");
            nodes.push(res);
        }
        Ok(nodes)
    }

    pub fn load_connections(&self, j: &json::JsonValue) -> Result<Vec<Connection>, Error> {
        let connections_array = &j["connections"];
        let mut connections: Vec<Connection> = Vec::new();

        for c in connections_array.members() {
            let source_str = match c["source"].as_str() {
                Some(n) => n,
                None => {
                    return Result::Err(Error::FieldMissing("source"));
                }
            };

            let source = JsonNodeLoader::parse_port(source_str)?;

            let dest_str = match c["dest"].as_str() {
                Some(n) => n,
                None => {
                    return Result::Err(Error::FieldMissing("dest"));
                }
            };

            let dest = JsonNodeLoader::parse_port(dest_str)?;

            connections.push(Connection {
                source,
                dest,
                conversion: None,
                dest_type: None,
            })
        }

        Ok(connections)
    }

    fn parse_port(str_to_parse: &str) -> Result<NodePort, Error> {
        let name: &str;
        let idx: usize;

        match str_to_parse.rsplit_once('.') {
            None => {
                name = str_to_parse;
                idx = 0;
            }
            Some(p) => {
                name = p.0;
                idx = match p.1.parse::<usize>() {
                    Ok(i) => i,
                    Err(_) => {
                        return Result::Err(Error::ValueError(String::from(str_to_parse)));
                    }
                };
            }
        };

        Ok(NodePort {
            name: name.to_string(),
            index: idx,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::node_factory::DefaultNodeFactory;
    use crate::Text;
    use crate::TextContentType::Plain;

    use super::*;

    #[test]
    fn test_load_nodes() {
        let json_str = "{\"nodes\":[{\"class\": \"append\", \"name\":\"append1\", \"what_to_append\":\" test\"}]}";
        let j = json::parse(json_str).unwrap();
        let jl = JsonNodeLoader {};
        let factory = DefaultNodeFactory::default();
        let create_node = |class_name: &str,
                           name: &str,
                           jop: &dyn NodeOptionsProvider|
         -> Option<Box<dyn Node>> {
            factory.create_node(class_name, name, jop, None)
        };
        let mut v = jl.load_nodes(&j, create_node).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].get_name(), "append1");
        assert_eq!(
            v[0].run(
                &MessageData::Text(Text {
                    value: String::new(),
                    content_type: Plain
                }),
                0
            )
            .as_message()
            .unwrap()
            .as_text()
            .unwrap(),
            " test"
        );
    }

    #[test]
    fn test_load_connections() {
        let json_str = "{\"connections\":[{\"source\":\"foo1.2\",\"dest\":\"bar1\"}]}";
        let j = json::parse(json_str).unwrap();
        let jl = JsonNodeLoader {};
        let v = jl.load_connections(&j).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].source.name, "foo1");
        assert_eq!(v[0].source.index, 2);
        assert_eq!(v[0].dest.name, "bar1");
        assert_eq!(v[0].dest.index, 0);
    }
}
