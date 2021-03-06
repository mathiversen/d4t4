use crate::error::Error;
use crate::tokenizer::{Rule, Tokenizer};
use anyhow::Result;
use pest::{iterators::Pair, iterators::Pairs, Parser};
use serde_json::{map::Map, value::Value};
use std::collections::{HashMap, VecDeque};
use std::default::Default;
use std::str::FromStr;

#[derive(Debug)]
pub struct Reference {
    target: String,
    value: Option<Value>,
    location: String,
}

#[derive(Default, Debug)]
pub struct Context {
    references: HashMap<String, Vec<Reference>>,
    location: Vec<String>,
}

pub fn parse(input: &str) -> Result<Value> {
    let mut ctx = Context::default();

    let tokenizer = Tokenizer::parse(Rule::root, input)?
        .next()
        .expect("failed to parse the file");

    let mut json = match tokenizer.as_rule() {
        Rule::object => parse_object(tokenizer.into_inner(), &mut ctx)?,
        Rule::array => parse_array(tokenizer.into_inner(), &mut ctx)?,
        _ => unreachable!("json can only be of type array or object"),
    };

    get_reference_values(&json, &mut ctx)?;
    set_reference_values(&mut json, &ctx)?;

    Ok(json)
}

fn get_reference_values(data: &Value, ctx: &mut Context) -> Result<()> {
    for (_target, references) in ctx.references.iter_mut() {
        for reference in references.iter_mut() {
            let value = get_reference_value(data, &reference, &reference.target)?;
            reference.value = Some(value);
        }
    }
    Ok(())
}

fn get_reference_value(data: &Value, reference: &Reference, path: &str) -> Result<Value> {
    let mut keys = path.split('.').collect::<Vec<_>>();
    let first_key = keys.first().expect("should have a key");
    if let Some(value) = data.get(&first_key) {
        let value = match value {
            Value::Object(_) => {
                keys.remove(0);
                get_reference_value(&value, reference, keys.join(".").as_str())?
            }
            Value::Array(_) => {
                return Err(Error::Parsing(format!(
                    "Referencing arrays are not supported, failed at key: {}",
                    &first_key
                ))
                .into())
            }
            _ => value.clone(),
        };
        Ok(value)
    } else {
        Err(Error::Parsing(format!(
            "No data was found in: {} at {}",
            reference.target, first_key
        ))
        .into())
    }
}

fn set_reference_values(data: &mut Value, ctx: &Context) -> Result<()> {
    for (target, references) in ctx.references.iter() {
        let mut path = target.split('.').collect::<VecDeque<_>>();
        set_reference_value_at_target(data, &mut path, references, true)?;
    }
    Ok(())
}

fn set_reference_value_at_target(
    data: &mut Value,
    path: &mut VecDeque<&str>,
    references: &[Reference],
    hard_error: bool,
) -> Result<()> {
    let key = path.pop_front();
    if let Some(key) = key {
        match data {
            Value::Array(array) => {
                for value in array {
                    path.push_front(key);
                    set_reference_value_at_target(value, path, references, false)?;
                }
            }
            Value::Object(_) => {
                if let Some(data) = data.get_mut(key) {
                    set_reference_value_at_target(data, path, references, true)?;
                } else {
                    // TODO: This is to prevent errors when looping over multiple elements inside of an
                    // array (logic above). We should probably improve this so that we index into the
                    // array instead of looping over every element.
                    if hard_error {
                        return Err(Error::Parsing(format!("unknown reference key {}", key)).into());
                    }
                }
            }
            _ => unimplemented!("only objects can have references"),
        }
    } else {
        for reference in references {
            if let Some(value) = &reference.value {
                match (&data, value) {
                    (Value::String(x), Value::String(ref y)) => {
                        let new_value = x
                            .clone()
                            .replace(format!("&{{{}}}", &reference.target).as_str(), &y);
                        *data = Value::String(new_value);
                    }
                    _ => {
                        return Err(Error::Parsing(
                            "only string references have been implemented".to_string(),
                        )
                        .into());
                    }
                };
            } else {
                println!(
                    "{}",
                    format!("reference {} doesnt have a value", reference.target)
                );
            }
        }
    }
    Ok(())
}

fn parse_value(pair: Pair<Rule>, ctx: &mut Context) -> Result<Value> {
    match pair.as_rule() {
        Rule::null => Ok(Value::Null),
        Rule::bool => parse_bool(pair.as_str()),
        Rule::number => parse_number(pair.as_str()),
        Rule::string => parse_string(pair, ctx, true),
        Rule::object => parse_object(pair.into_inner(), ctx),
        Rule::array => parse_array(pair.into_inner(), ctx),
        _ => unreachable!("unknown json value"),
    }
}

fn parse_bool(value: &str) -> Result<Value> {
    Ok(Value::from_str(value)?)
}

fn parse_number(value: &str) -> Result<Value> {
    Ok(Value::from_str(value)?)
}

fn parse_array(pairs: Pairs<Rule>, ctx: &mut Context) -> Result<Value> {
    let mut array = Vec::new();
    for pair in pairs {
        let value = parse_value(pair, ctx)?;
        array.push(value);
    }
    Ok(Value::Array(array))
}

fn parse_object(pairs: Pairs<Rule>, ctx: &mut Context) -> Result<Value> {
    let mut object = Map::new();
    for pair in pairs {
        // TODO: Improve this key value logic
        let mut key_value_pair = Vec::new();
        for (index, key_value) in pair.into_inner().enumerate() {
            let value = match index {
                0 => {
                    let key = parse_string(key_value, ctx, false)?;
                    ctx.location.push(key.clone().as_str().unwrap().to_string()); // TODO: Use str
                    key
                }
                1 => parse_value(key_value, ctx)?,
                _ => unreachable!("pair should only be two"),
            };
            key_value_pair.push(value);
        }
        let key = key_value_pair[0]
            .clone()
            .as_str()
            .expect("failed to translate value to str")
            .to_string();
        if object.contains_key(&key) {
            return Err(Error::Parsing(format!("Object already contains key: {}", key)).into());
        } else {
            object.insert(key, key_value_pair[1].clone());
        }
        ctx.location.pop();
    }
    Ok(Value::Object(object))
}

fn parse_string(pair: Pair<Rule>, ctx: &mut Context, extract_refs: bool) -> Result<Value> {
    let mut string = pair.as_str().to_string();
    if string.starts_with("\"") || string.starts_with("\'") {
        remove_wrapping_quotes(&mut string);
    }
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::text => replace_escape_in_string_pair(pair, &mut string)?,
            Rule::reference => {
                if extract_refs {
                    add_reference_to_ctx(pair, ctx)?
                } else {
                    return Err(Error::Parsing(
                        "References are not allowed inside of keys".to_string(),
                    )
                    .into());
                }
            }
            _ => unreachable!("strings can only consist of a text and/or a reference"),
        }
    }
    Ok(Value::String(string))
}

fn replace_escape_in_string_pair(pair: Pair<Rule>, string: &mut String) -> Result<()> {
    for pair in pair.into_inner() {
        let new_value = match pair.as_rule() {
            Rule::esc_slash => '/'.to_string(),
            Rule::esc_backslash => '\\'.to_string(),
            Rule::esc_carriage_return => '\r'.to_string(),
            Rule::esc_tab => '\t'.to_string(),
            Rule::esc_quote_double => '\"'.to_string(),
            Rule::esc_quote_single => '\''.to_string(),
            Rule::esc_backspace => '\u{8}'.to_string(),
            Rule::esc_form_feed => '\u{c}'.to_string(),
            Rule::esc_new_line => '\n'.to_string(),
            Rule::esc_unicode => parse_unicode(pair.as_str())?,
            _ => unimplemented!(),
        };
        *string = string.replacen(&pair.as_str(), &new_value, 1);
    }
    Ok(())
}

fn parse_unicode(string: &str) -> Result<String> {
    let unicode = &string[2..];
    if let Some(unicode) = u32::from_str_radix(unicode, 16)
        .ok()
        .and_then(std::char::from_u32)
    {
        Ok(unicode.to_string())
    } else {
        Err(Error::Parsing(format!("unknown unicode {}", unicode)).into())
    }
}

fn add_reference_to_ctx(pair: Pair<Rule>, ctx: &mut Context) -> Result<()> {
    let current_location = ctx.location.clone().join(".");
    let entry = ctx
        .references
        .entry(current_location.clone())
        .or_insert_with(Vec::new);
    entry.push(Reference {
        target: pair.as_str().to_string(),
        value: None,
        location: current_location,
    });
    Ok(())
}

fn remove_wrapping_quotes(string: &mut String) {
    string.remove(0);
    string.pop();
}
