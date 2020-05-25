use data_parser::{parse, Result};
use indoc::indoc;
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_object_with_comments() -> Result<()> {
    let markup = indoc!(
        r##"{
            # This is a comment
            "values": {
                "r": "10",
                "g": "10",
                "b": "100" # this is also a comment
            },
            "two": "rgba(${values.r}, ${values.g}, ${values.b}, 0.1)",
            "tree": "${values.r}px"
        }"##
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}
