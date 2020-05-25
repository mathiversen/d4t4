use data_parser::{parse, Result};
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

const JSON_COMPLETE: &'static str = "tests/data_complete.json";
const JSON_SIMPLE: &'static str = "tests/data_simple.json";

// https://github.com/pest-parser/pest/blob/master/grammars/tests/examples.json

#[test]
fn it_can_parse_simple_json() -> Result<()> {
    let data = read_file_to_string(JSON_SIMPLE)?;
    let v: Value = serde_json::from_str(&data)?;
    let x = parse(&data)?;
    assert_eq!(v, x);
    Ok(())
}

#[ignore]
#[test]
fn it_can_parse_j() -> Result<()> {
    let data = read_file_to_string(JSON_COMPLETE)?;
    let v: Value = serde_json::from_str(&data)?;
    let x = parse(&data)?;
    assert_eq!(v, x);
    Ok(())
}

fn read_file_to_string(path: &'static str) -> Result<String> {
    let mut f = File::open(path)?;
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    Ok(data)
}
