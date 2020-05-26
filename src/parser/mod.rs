use crate::ast::{Ast, Rule};
use crate::error::Error;
use anyhow::Result;
use pest::{iterators::Pair, iterators::Pairs, Parser as PestParser};
use serde_json::{map::Map, value::Value};
use std::collections::HashMap;
use std::default::Default;
use std::str::FromStr;

#[derive(Debug)]
pub struct Reference {
    name: String,
    value: Option<Value>,
}

#[derive(Default, Debug)]
pub struct Context {
    references: HashMap<String, Vec<Reference>>,
    location: Vec<String>,
}

pub fn parse(input: &str) -> Result<Value> {
    let mut ctx = Context::default();

    let ast = Ast::parse(Rule::ast, input)?
        .next()
        .expect("failed to parse the file");

    let mut json = match ast.as_rule() {
        Rule::object => parse_object(ast.into_inner(), &mut ctx)?,
        Rule::array => parse_array(ast.into_inner(), &mut ctx)?,
        _ => unreachable!("json can only be of type array or object"),
    };

    get_reference_values(&mut json, &mut ctx)?;
    set_reference_value(&mut json, &ctx)?;

    Ok(json)
}

fn get_reference_values(data: &Value, ctx: &mut Context) -> Result<()> {
    for (_key, references) in ctx.references.iter_mut() {
        for reference in references.iter_mut() {
            let value = get_object_value(data, &reference.name)?;
            reference.value = Some(value);
        }
    }
    Ok(())
}

fn set_reference_value(data: &mut Value, ctx: &Context) -> Result<()> {
    for (key, references) in ctx.references.iter() {
        for reference in references.iter() {
            // TODO: improve this logic
            if let Some(prev) = data.get_mut(key) {
                let new_value = match *prev {
                    Value::String(ref s) => Value::String(
                        s.clone().replace(
                            format!("${{{}}}", reference.name).as_str(),
                            reference
                                .value
                                .as_ref()
                                .expect("failed to get reference.value")
                                .as_str()
                                .expect("failed to translate reference value to string"),
                        ),
                    ),
                    _ => unreachable!("nej"), // TODO: references can only be string
                };
                *prev = new_value;
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
                    &ctx.location.push(key.clone().as_str().unwrap().to_string()); // TODO: Use str
                    key
                }
                1 => parse_value(key_value, ctx)?,
                _ => unreachable!("pair should only be two"),
            };
            key_value_pair.push(value);
        }
        object.insert(
            key_value_pair[0]
                .clone()
                .as_str()
                .expect("failed to translate value to str")
                .to_string(),
            key_value_pair[1].clone(),
        );
        ctx.location.pop();
    }
    Ok(Value::Object(object))
}

fn get_object_value(data: &Value, path: &str) -> Result<Value> {
    let mut keys = path.split(".").collect::<Vec<_>>();
    let x = data.get(&keys[0]).expect("no value was found in path");
    let value = match x {
        Value::Object(_) => {
            keys.remove(0);
            get_object_value(&x, keys.join(".").as_str())?
        }
        _ => x.clone(),
    };
    Ok(value)
}

fn parse_string(pair: Pair<Rule>, ctx: &mut Context, extract_refs: bool) -> Result<Value> {
    let mut string = pair.as_str().to_string();
    remove_wrapping_quotes(&mut string);
    for pair in pair.into_inner() {
        match pair.as_rule() {
            Rule::text => replace_escape_in_string_pair(pair, &mut string)?,
            Rule::reference => {
                if extract_refs {
                    add_reference_to_ctx(pair, ctx)?
                } else {
                    return Err(Error::Parsing("References are not allowed".to_string()).into());
                }
            }
            _ => unreachable!("strings can only have text or references"),
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
        .or_insert(Vec::new());
    entry.push(Reference {
        name: pair.as_str().to_string(),
        value: None,
    });
    Ok(())
}

fn remove_wrapping_quotes(string: &mut String) {
    string.remove(0);
    string.pop();
}
