use data_parser::{parse, Result};
use indoc::indoc;
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_double_quotes() -> Result<()> {
    let markup = indoc!(
        r#"{
            "name": "Mr. Anderson",
            "greeting": "We meet again &{name}",
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_can_parse_single_quotes() -> Result<()> {
    let markup = indoc!(
        r#"{
            'name': 'Mr. Anderson',
            'greeting': 'We meet again &{name}',
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_can_parse_mixed_quotations() -> Result<()> {
    let markup = indoc!(
        r#"{
            "name": 'Mr. Anderson',
            "greeting": 'We "meet" again &{name}',
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_can_parse_double_inside_single_quotes() -> Result<()> {
    let markup = indoc!(
        r#"{
            'name': 'Mr. Anderson',
            'greeting': 'We "meet" again &{name}',
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_can_parse_single_inside_double_quotes() -> Result<()> {
    let markup = indoc!(
        r#"{
            "name": "Mr. Anderson",
            "greeting": "We 'meet' again &{name}",
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_can_parse_double_with_escaped_double() -> Result<()> {
    let markup = indoc!(
        r#"{
            "name": "Mr. Anderson",
            "greeting": "We \"meet\" again &{name}",
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_can_parse_single_with_escaped_single() -> Result<()> {
    let markup = indoc!(
        r#"{
            'name': 'Mr. Anderson',
            'greeting': 'We \'meet\' again &{name}',
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}
