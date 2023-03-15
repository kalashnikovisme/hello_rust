use std;
use std::io::Write;
use serde_json;
use magnus::{class, define_module, function, prelude::*, Error};
use std::collections::HashMap;

#[magnus::wrap(class = "Hash")]
struct MyHashMap {
    json: String
    // json: HashMap<std::string::String, serde_json::Value>
}

fn parse_json(file_path: String) -> magnus::RHash {
    let data = std::fs::read_to_string(file_path.clone()).expect("Unable to read file");
    let json: magnus::RHash = serde_json::from_str::<magnus::RHash>(&data).unwrap();
    json

    // let json: HashMap<String, serde_json::Value> =
    //     serde_json::from_str(&data).unwrap();
        // serde_json::from_str::<MyHashMap>(&data).unwrap().expect("JSON was not well-formatted");

    // println!("{0}", json["arr"][0]);

    // let hash = magnus::RHash::new();
    //     hash.aset("answer", 42);
    // hash
    // magnus::RHash(json)
    // let hash: magnus::RHash = json.clone()

    // format!("{}", json["arr"][0])
}

struct MagnusValue(magnus::Value);

impl From<MyHashMap> for MagnusValue {
    fn from(hash: MyHashMap) -> Self {
        todo!()
    }
}

// impl serde_json::de::Deserialize for MyHashMap {
//     fn class() -> Self { todo!() }
// }

// unsafe impl magnus::TypedData for MyHashMap {
//     fn class(data: dyn magnus::TypedData) -> Self { todo!() }
// }

#[magnus::init]
fn init() -> Result<(), magnus::Error> {
    let module = define_module("ParseJson")?;
    module.define_singleton_method("parse_json", function!(parse_json, 1))?;
    Ok(())
}
