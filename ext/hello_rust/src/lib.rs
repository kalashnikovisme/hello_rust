use std;
use magnus::{define_class, class, method, define_module, function, prelude::*, RHash, RClass, DataType, TypedData};
use std::collections::HashMap;
use serde_json;
// use tinyjson;

fn parse_json(file_path: String) -> MyHashMap {
    let data = std::fs::read_to_string(file_path.clone()).expect("Unable to read file");
    let json: HashMap<String, serde_json::Value> = serde_json::from_str(&data).unwrap();
    MyHashMap(json)
}

#[derive(Debug)]
#[magnus::wrap(class = "Value")]
struct MyHashMap(HashMap<String, serde_json::Value>);

impl MyHashMap {
    fn get(&self, key: String) -> String {
        (*self)[key]
    } 
}

#[magnus::init]
fn init() -> Result<(), magnus::Error> {
    let module = define_module("ParseJson")?;
    module.define_singleton_method("parse_json", function!(parse_json, 1))?;
    let value_class = define_class("Value", class::object()).unwrap();
    value_class.define_method("[]", method!(MyHashMap::get, 1))?;

    Ok(())
}
