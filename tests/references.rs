use data_parser::{parse, Result};
use indoc::indoc;
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_references_at_root() -> Result<()> {
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
            "r": "10",
            "g": "10",
            "b": "100",
            "color": "rgba(${r}, ${g}, ${b}, 0.1)"
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_can_parse_nested_references() -> Result<()> {
    let markup = indoc!(
        r#"{
            "value": "10",
            "object": {
                "width": "${value}px"
            }
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}
#[test]
fn it_can_parse_deeply_nested_references() -> Result<()> {
    let markup = indoc!(
        r#"{
            "value": "10",
            "one": {
                "two": {
                    "three": {
                        "width": "${value}px"
                    }
                }
            }
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_can_parse_nested_targets() -> Result<()> {
    let markup = indoc!(
        r#"{
            "value": {
                "r": "10",
                "g": "10",
                "b": "10",
            },
            "color": "rgba(${value.r}, ${value.g}, ${value.b}, 0.1)"
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}
#[test]
fn it_can_parse_deeply_nested_targets() -> Result<()> {
    let markup = indoc!(
        r#"{
            "value": {
                "color": {
                    "black": {
                        "r": "0",
                        "g": "0",
                        "b": "0",
                    }
                }
            },
            "color": "rgba(${value.color.black.r}, ${value.color.black.g}, ${value.color.black.b}, 0.1)"
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_can_reference_from_within_array() -> Result<()> {
    let markup = indoc!(
        r#"{
            "variables": {
                "name": "kalle",
                "surname": "anka"
            },
            "person": [
                {
                    "name": "${variables.name}"
                },
                {
                    "surname": "${variables.surname}"
                }
            ]
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_thows_error_when_reference_in_key() {
    let markup = indoc!(
        r#"{
            "value": "10px",
            "key ${value}": "border-width: ${value}"
        }"#
    );
    assert!(parse(markup).is_err());
}
