use data_parser::{Data, Result};
use indoc::indoc;
use insta::assert_json_snapshot;
use serde_json::Value;
use std::fs::File;
use std::io;
use std::io::prelude::*;

const JSON_FILE: &'static str = "tests/data.json";

// https://github.com/pest-parser/pest/blob/master/grammars/tests/examples.json
#[test]
fn it_can_parse_json() -> Result<()> {
    let markup = indoc!(
        r#"  {
            "null": null,
            "true": true,
            "false": false,
            "":  23456789012E66,
            "e": 0.123456789e-12,
            "E": 1.234567890E+34,
            "integer": 1234567890,
            "one": 1,
            "real": -9876.543210,
            "zero": 0,
        }"#
    );
    let x = Data::parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_can_parse_j() -> Result<()> {
    let json = parse_json_with_serde(JSON_FILE)?;
    assert_json_snapshot!(json);
    Ok(())
}

fn parse_json_with_serde(path: &str) -> Result<Value> {
    let mut f = File::open(JSON_FILE)?;
    let mut data = String::new();
    f.read_to_string(&mut data)?;
    Ok(serde_json::from_str(&data)?)
}
