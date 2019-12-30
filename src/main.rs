use serde_cbor::Value;
use serde_cbor::to_vec as cbor_to_vec;
use crate::operators::{RadonOpCodes, RadonReducers, RadonFilters};

pub mod operators;

pub fn dr() -> Vec<u8> {
    cbor_to_vec(&Value::Array(vec![
        Value::Integer(RadonOpCodes::StringParseJSONMap as i128),
        Value::Array(vec![
            Value::Integer(RadonOpCodes::MapGetMap as i128),
            Value::Text(String::from("bpi")),
        ]),
        Value::Array(vec![
            Value::Integer(RadonOpCodes::MapGetMap as i128),
            Value::Text(String::from("USD")),
        ]),
        Value::Array(vec![
            Value::Integer(RadonOpCodes::MapGetFloat as i128),
            Value::Text(String::from("rate_float")),
        ]),
    ]))
        .unwrap()
}

fn main() {
    println!("{:?}", dr());
}
