use json;
use crate::common::*;
use crate::nodes::NODE_CLASSES;
use crate::JsonNodeOptionsProvider;
use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum JsonNodeLoaderError {
        Json(err: json::Error) {
            from()
        }
        NodeOptions(err: NodeOptionsError) {
            from()
        }
        ClassNotFound(classname: String) {
            display("Class not found: {}", classname)
        }
        FieldMissing(field: &'static str) {
            display("Field missing: {}", field)
        }
    }
}

pub struct JsonNodeLoader {

}

impl JsonNodeLoader {
    fn class_by_name(class_name: &str) -> Option<&NodeClass> {
        for nc in NODE_CLASSES {
            match nc.name == class_name {
                true => return Option::Some(nc),
                false => continue
            }
        }
        Option::None
    }

    pub fn load(&mut self, j: &json::JsonValue) -> Result<Vec<Box<dyn Node>>,JsonNodeLoaderError> {
        let nodes_array = &j["nodes"];
        let mut nodes : Vec<Box<dyn Node>> = Vec::new();

        for e in nodes_array.members() {
            let class_name = match e["class"].as_str() {
                Some(n) => n,
                None => { return Result::Err(JsonNodeLoaderError::FieldMissing("class")) }
            };
            let name = match e["name"].as_str() {
                Some(n) => n,
                None => { return Result::Err(JsonNodeLoaderError::FieldMissing("name")) }
            };
            let class = match JsonNodeLoader::class_by_name(class_name) {
                Some(c) => c,
                None => { return Result::Err(JsonNodeLoaderError::ClassNotFound(class_name.to_string())) }
            };
            let res = (class.constructor)(NodeCommonData{
                name: name.to_string()
            }, &JsonNodeOptionsProvider {
                data: &e
            })?;
            nodes.push(res);

        }
        Ok(nodes)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_loader() {
        let json_str = "{\"nodes\":[{\"class\": \"append\", \"name\":\"append1\", \"what_to_append\":\" test\"}]}";
        let j = json::parse(json_str).unwrap();
        let mut jl = JsonNodeLoader{};
        let mut v = jl.load(&j).unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].get_name(), "append1");
        assert_eq!(v[0].run(&Default::default()).as_message().unwrap().value, " test");

    }
}