use data_parser::{Data, Result};
use indoc::indoc;
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_double_quotes() -> Result<()> {
    let markup = indoc!(
        r#"{
            "name": "Mr. Anderson",
            "greeting": "We meet again ${name}",
        }"#
    );
    let x = Data::parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_can_parse_single_quotes() -> Result<()> {
    let markup = indoc!(
        r#"{
            'name': 'Mr. Anderson',
            'greeting': 'We meet again ${name}',
        }"#
    );
    let x = Data::parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_can_parse_backticks_quotes() -> Result<()> {
    let markup = indoc!(
        r#"{
            `name`: `Mr. Anderson`,
            `greeting`: `We meet again ${name}`,
        }"#
    );
    let x = Data::parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_can_parse_mixed_quotations() -> Result<()> {
    let markup = indoc!(
        r#"{
            "name": 'Mr. Anderson',
            "greeting": `We meet again ${name}`,
        }"#
    );
    let x = Data::parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}
