use std;
use magnus::{define_class, class, method, define_module, function, prelude::*, RHash, RClass, DataType, TypedData};
use std::collections::HashMap;
use tinyjson;

fn parse_json(file_path: String) -> Value {
    let data = std::fs::read_to_string(file_path.clone()).expect("Unable to read file");
    let json_value: tinyjson::JsonValue = data.parse().unwrap();
    let value: Value = json_value.into();

    // std::println!("{:?}", value.get("5".to_string()));
    value
}

#[derive(Debug)]
#[magnus::wrap(class = "Value")]
enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    Null,
    Array(Vec<tinyjson::JsonValue>),
    Object(HashMap<String, tinyjson::JsonValue>),
}

impl Value {
    fn get(&self, key: String) -> String {
        key
    } 
}

impl From<tinyjson::JsonValue> for Value {
    fn from(value: tinyjson::JsonValue) -> Self {
        match value {
            tinyjson::JsonValue::Number(number) => Value::Number(number),
            tinyjson::JsonValue::Boolean(number) => Value::Boolean(number),
            tinyjson::JsonValue::String(number) => Value::String(number),
            tinyjson::JsonValue::Null => Value::Null,
            tinyjson::JsonValue::Array(number) => Value::Array(number),
            tinyjson::JsonValue::Object(number) => Value::Object(number),
        }
    }
}


#[magnus::init]
fn init() -> Result<(), magnus::Error> {
    let module = define_module("ParseJson")?;
    module.define_singleton_method("parse_json", function!(parse_json, 1))?;
    let value_class = define_class("Value", class::object()).unwrap();
    value_class.define_method("[]", method!(Value::get, 1))?;

    Ok(())
}
