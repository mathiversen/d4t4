use d4t4::{parse, Result};
use indoc::indoc;
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_references_at_root() -> Result<()> {
    let markup = indoc!(
        r#"{
            "value": "10px",
            "border": "border-width: &{value}"
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
            "color": "rgba(&{r}, &{g}, &{b}, 0.1)"
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
                "width": "&{value}px"
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
                        "width": "&{value}px"
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
            "color": "rgba(&{value.r}, &{value.g}, &{value.b}, 0.1)"
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
            "color": "rgba(&{value.color.black.r}, &{value.color.black.g}, &{value.color.black.b}, 0.1)"
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
                    "name": "&{variables.name}"
                },
                {
                    "surname": "&{variables.surname}"
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
            "key &{value}": "border-width: &{value}"
        }"#
    );
    assert!(parse(markup).is_err());
}

#[test]
fn it_thows_error_when_referencing_arrays() {
    let markup = indoc!(
        r#"{
            reference: [1, 2, 3],
            key: "&{reference}",
        }"#
    );
    let x = parse(markup).err().unwrap();
    assert_eq!(
        format!("{}", x),
        "Referencing arrays are not supported, failed at key: reference"
    );
}

#[test]
fn it_thows_error_when_key_is_not_found() {
    let markup = indoc!(
        r#"{
            hej: [1, 2, 3],
            hey: "&{tjena}",
        }"#
    );
    let x = parse(markup).err().unwrap();
    assert_eq!(format!("{}", x), "No data was found in: tjena at tjena");
}

#[test]
fn it_thows_error_when_key_is_not_found_in_nested() {
    let markup = indoc!(
        r#"{
            variables: {
                colors: {
                    r: "100",
                    g: "100",
                    b: "100"
                }
            },
            card: {
                color: "&{variables.colors.d}",
            },
        }"#
    );
    let x = parse(markup).err().unwrap();
    assert_eq!(
        format!("{}", x),
        "No data was found in: variables.colors.d at d"
    );
}

#[test]
#[ignore]
fn it_can_chain_references() -> Result<()> {
    let markup = indoc!(
        r#"{
            "key1": "1px",
            "key2": "&{key1} 2px",
            "key3": "&{key2} 3px",
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[ignore]
#[test]
fn it_thows_error_on_recursion() {
    let markup = indoc!(
        r#"{
            0: "&{1}",
            1: "&{0}",
        }"#
    );
    assert!(parse(markup).is_err());
}

#[test]
#[ignore]
fn it_can_translate_int_to_string() -> Result<()> {
    let markup = indoc!(
        r#"{
            "key1": 1,
            "key2": "&{key1}px",
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
#[ignore]
fn it_can_translate_float_to_string() -> Result<()> {
    let markup = indoc!(
        r#"{
            "key1": 1.1,
            "key2": "&{key1}px",
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
#[ignore]
fn it_can_translate_bool_to_string() -> Result<()> {
    let markup = indoc!(
        r#"{
            "key1": true,
            "key2": false,
            "key3": "&{key1} &{key2}",
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
#[ignore]
fn it_can_translate_null_to_string() -> Result<()> {
    let markup = indoc!(
        r#"{
            "key1": null,
            "key2": "&{key1}",
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
#[ignore]
fn it_can_reference_arrays() -> Result<()> {
    let markup = indoc!(
        r#"{
            "key1": [1,2,3],
            "key2": &key1,
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
#[ignore]
fn it_can_reference_objects() -> Result<()> {
    let markup = indoc!(
        r#"{
            key1: {
                colors: [1,2,3]
            },
            key2: &key1,
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
#[ignore]
fn it_can_extend_objects() -> Result<()> {
    let markup = indoc!(
        r#"{
            base-color: {
                r: "10",
                g: "10",
                b: "10",
            },
            extended-color: {
                &base-color,
                b: "20",
            },
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}
