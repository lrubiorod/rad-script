use serde_cbor::Value;
use serde_cbor::to_vec as cbor_to_vec;
use crate::operators::{RadonOpCodes, RadonReducers, RadonFilters};

pub mod operators;

pub fn dr() -> Vec<u8> {
    cbor_to_vec(&Value::Array(vec![
        Value::Integer(RadonOpCodes::StringParseXMLMap as i128),
        Value::Array(vec![
            Value::Integer(RadonOpCodes::MapGetMap as i128),
            Value::Text(String::from("dwml")),
        ]),
        Value::Array(vec![
            Value::Integer(RadonOpCodes::MapGetArray as i128),
            Value::Text(String::from("data")),
        ]),
        Value::Array(vec![
            Value::Integer(RadonOpCodes::ArrayGetMap as i128),
            Value::Integer(0),
        ]),
        Value::Array(vec![
            Value::Integer(RadonOpCodes::MapGetMap as i128),
            Value::Text(String::from("parameters")),
        ]),
        Value::Array(vec![
            Value::Integer(RadonOpCodes::MapGetMap as i128),
            Value::Text(String::from("weather")),
        ]),
        Value::Array(vec![
            Value::Integer(RadonOpCodes::MapGetArray as i128),
            Value::Text(String::from("weather-conditions")),
        ]),
        Value::Array(vec![
            Value::Integer(RadonOpCodes::ArrayGetMap as i128),
            Value::Integer(2),
        ]),
        Value::Array(vec![
            Value::Integer(RadonOpCodes::MapGetString as i128),
            Value::Text(String::from("@weather-summary")),
        ]),
    ]))
        .unwrap()
}

fn main() {
    println!("{:?}", dr());
}
