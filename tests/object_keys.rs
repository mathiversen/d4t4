use data_parser::{parse, Result};
use indoc::indoc;
use insta::assert_json_snapshot;

#[test]
fn it_can_parse_different_keys() -> Result<()> {
    let markup = indoc!(
        r#"{
            _name: "Mr. Sunesson",
            $name: "Mr. Birgersson",
            0: "Mr. Karlsson",
            1-name: "Mr. Eriksson",
            2_name: "Mr. Fredriksson",
            name-1: "Mr. Andersson",
            name_2: "Mr. Johansson",
            "name-3": "Mr. Davidsson",
        }"#
    );
    let x = parse(markup)?;
    assert_json_snapshot!(x);
    Ok(())
}

#[test]
fn it_errors_on_same_key() {
    let markup = indoc!(
        r#"{
            name: "Mr. Karlsson",
            name: "Mr. Eriksson",
        }"#
    );
    let x = parse(markup).err().unwrap();
    assert_eq!(format!("{}", x), "Object already contains key: name");
}
