use indoc::indoc;
use insta::assert_json_snapshot;
use sdf_parser::{Result, Sdf};

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
    let x = Sdf::parse(markup)?;
    assert_json_snapshot!(x);
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
    Sdf::parse(markup)?;
    Ok(())
}
