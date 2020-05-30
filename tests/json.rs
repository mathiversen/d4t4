use d4t4::{parse, Result};
use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;

// https://github.com/pest-parser/pest/blob/master/grammars/tests/examples.json

const JSON: &'static str = "tests/data/data.json";

#[test]
fn it_can_parse_json() -> Result<()> {
    let data = read_file_to_string(JSON)?;
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
