use std;
use magnus::{DataTypeFunctions, typed_data::DataTypeBuilder, memoize, define_class, class, method, define_module, function, prelude::*, RClass, DataType, TypedData};
use std::collections::HashMap;
use serde_json;
use serde_magnus::serialize;

fn parse_json(file_path: String) -> UltimateJSON {
    let data = std::fs::read_to_string(file_path.clone()).expect("Unable to read file");
    let json: HashMap<String, serde_json::Value> = serde_json::from_str(&data).unwrap();
    UltimateJSON(json)
}

#[derive(Debug)]
#[magnus::wrap(class = "UltimateJSON")]
struct UltimateJSON(HashMap<String, serde_json::Value>);

#[derive(Debug)]
struct JsonArray(Vec<serde_json::Value>);

impl JsonArray {
    fn to_s(&self) -> String {
        let arr_str = self.0.iter()         // ChatGPT
            .map(|x| x.to_string())         // ChatGPT
            .collect::<Vec<String>>()       // ChatGPT
            .join(", ");                    // ChatGPT
        arr_str                             // ChatGPT
    }
}

#[derive(Debug)]
struct JsonObject(serde_json::Map<String, serde_json::Value>);

impl JsonObject {
    fn fetch(&self, key: String) -> JsonValue {
        let val: JsonValue = (*self).0[&key].clone().into();
        val
    } 

    fn to_s(&self) -> String {
         let json_str = serde_json::json!((*self).0).to_string();       // ChatGPT

         json_str
    }

    fn to_h(&self) -> magnus::RHash {
        serialize(&(*self).0).unwrap()
    }
}

#[derive(Debug)]
enum JsonValue {
    Null,
    Bool(bool),
    Number(serde_json::value::Number),
    String(String),
    Array(JsonArray),
    Object(JsonObject),
}

impl JsonValue {
    fn to_s(&self) -> String {
        match self {
            JsonValue::String(str) => (*str).clone(),
            JsonValue::Bool(str) => (*str).to_string().clone(),
            JsonValue::Number(str) => (*str).to_string().clone(),
            JsonValue::Array(str) => (*str).to_s().clone(),
            JsonValue::Object(str) => (*str).to_s().clone(),
            JsonValue::Null => "nil".to_string(),                   // ChatGPT
            _ => todo!(),
        }
    }

    fn to_h(&self) -> magnus::RHash {
        match self {
            JsonValue::Object(str) => serialize(&(str).0).unwrap(),
            _ => todo!(),
        }
    }
}

unsafe impl TypedData for JsonValue {
    fn class() -> RClass {
        *memoize!(RClass: {
            let class = define_class("JsonValue", class::object()).unwrap();
            class.define_method("to_s", method!(JsonValue::to_s, 0));
            class.define_method("to_h", method!(JsonValue::to_h, 0));
            class.undef_alloc_func();
            class
        })
    }
    fn data_type() -> &'static DataType {
        memoize!(DataType: DataTypeBuilder::<JsonValue>::new("example").build())
    }
}


impl DataTypeFunctions for JsonValue {}

impl From<serde_json::value::Value> for JsonValue {
    fn from(value: serde_json::value::Value) -> Self {
        match value {
            serde_json::value::Value::Null => JsonValue::Null,
            serde_json::value::Value::Number(number) => JsonValue::Number(number),
            serde_json::value::Value::Bool(number) => JsonValue::Bool(number),
            serde_json::value::Value::String(number) => JsonValue::String(number),
            serde_json::value::Value::Array(number) => JsonValue::Array(JsonArray(number)),
            serde_json::value::Value::Object(number) => JsonValue::Object(JsonObject(number)),
        }
    }
}

impl UltimateJSON {
    fn fetch(&self, key: String) -> JsonValue {
        let val: JsonValue = (*self).0[&key].clone().into();
        val
    } 
}

#[magnus::init]
fn init() -> Result<(), magnus::Error> {
    let value_class = define_class("UltimateJSON", class::object()).unwrap();
    value_class.define_singleton_method("parse_json", function!(parse_json, 1))?;
    value_class.define_method("[]", method!(UltimateJSON::fetch, 1))?;

    Ok(())
}
