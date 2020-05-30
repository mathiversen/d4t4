use d4t4::{parse, Result};
use indoc::indoc;
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_object_with_comment() -> Result<()> {
    let markup = indoc!(
        r#"[
            {
                /* This is a comment */
                "values": "10"
            }
        ]"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_can_parse_object_with_multiline_comment() -> Result<()> {
    let markup = indoc!(
        r#"[
            {
                /*
                    This is a comment that spans
                    across multiple lines
                */
                "values": "10"
            }
        ]"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}
