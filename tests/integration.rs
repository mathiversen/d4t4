use indoc::indoc;
use insta::assert_json_snapshot;
use myson_parser::{Myson, Result};

#[test]
fn it_can_parse_object() -> Result<()> {
    let markup = indoc!(
        r#"{
            "values": {
                "r": 10,
                "g": 10,
                "b": 100
            },
            "two": "rgba(${values.r}, ${values.g}, ${values.b}, 0.1)",
            "tree": "${values.r}px"
        }"#
    );
    Myson::parse(markup)?;
    Ok(())
}

#[test]
fn it_can_parse_array() -> Result<()> {
    let markup = indoc!(
        r#"[
            {
                "key": "value"
            }
        ]"#
    );
    Myson::parse(markup)?;
    Ok(())
}
