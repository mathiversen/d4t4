use data_parser::{Data, Result};
use indoc::indoc;
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_array() -> Result<()> {
    let markup = indoc!(
        r#"[
            "JSON Test Pattern pass1",
            {
                "object with 1 member": ["array with 1 element"]
            },
            {},
            [],
            -42,
            true,
            false,
            null,
        ]"#
    );
    let x = Data::parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}
