use indoc::indoc;
use insta::assert_json_snapshot;
use zen_parser::{Result, Zen};

#[test]
fn it_can_parse_object() -> Result<()> {
    let markup = indoc!(
        r#"{
            "values": {
                "r": 10,
                "g": 10,
                "b": 100
            },
            "two": "rgba(#values.r, #values.g, #values.g, 0.1)"
        }"#
    );
    Zen::parse(markup)?;
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
    Zen::parse(markup)?;
    Ok(())
}
