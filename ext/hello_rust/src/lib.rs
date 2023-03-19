use std;
use magnus::{DataTypeFunctions, typed_data::DataTypeBuilder, memoize, define_class, class, method, define_module, function, prelude::*, RHash, RClass, DataType, TypedData};
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

#[derive(Debug)]
#[derive(DataTypeFunctions)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(serde_json::value::Number),
    String(String),
    Array(Vec<serde_json::Value>),
    Object(serde_json::Map<String, serde_json::Value>),
}

unsafe impl TypedData for JsonValue {
    fn class() -> RClass {
        *memoize!(RClass: {
            let class = define_class("JsonValue", class::object()).unwrap();
            class.undef_alloc_func();
            class
        })
    }
    fn data_type() -> &'static DataType {
        memoize!(DataType: DataTypeBuilder::<JsonValue>::new("example").build())
    }
}

impl From<serde_json::value::Value> for JsonValue {
    fn from(value: serde_json::value::Value) -> Self {
        match value {
            serde_json::value::Value::Null => JsonValue::Null,
            serde_json::value::Value::Number(number) => JsonValue::Number(number),
            serde_json::value::Value::Bool(number) => JsonValue::Bool(number),
            serde_json::value::Value::String(number) => JsonValue::String(number),
            serde_json::value::Value::Array(number) => JsonValue::Array(number),
            serde_json::value::Value::Object(number) => JsonValue::Object(number),
        }
    }
}

impl MyHashMap {
    fn getByKey(&self, key: String) -> JsonValue {
        (*self).0[&key].clone().into()
    } 
}

#[magnus::init]
fn init() -> Result<(), magnus::Error> {
    let module = define_module("ParseJson")?;
    module.define_singleton_method("parse_json", function!(parse_json, 1))?;
    let value_class = define_class("Value", class::object()).unwrap();
    value_class.define_method("[]", method!(MyHashMap::getByKey, 1))?;

    Ok(())
}
