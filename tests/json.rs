use data_parser::{Data, Result};
use indoc::indoc;
use insta::assert_json_snapshot;

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
