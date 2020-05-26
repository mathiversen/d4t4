use data_parser::{parse, Result};
use indoc::indoc;
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_reference() -> Result<()> {
    let markup = indoc!(
        r#"{
            "value": "10px",
            "border": "border-width: ${value}"
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_can_parse_multiple_references() -> Result<()> {
    let markup = indoc!(
        r#"{
            "values": {
                "r": "10",
                "g": "10",
                "b": "100"
            },
            "two": "rgba(${values.r}, ${values.g}, ${values.b}, 0.1)",
            "tree": "${values.r}px"
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}
