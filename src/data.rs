use crate::error::Error;
use crate::parser::{Pest, Rule};
use anyhow::Result;
use pest::{iterators::Pairs, Parser as PestParser};
use serde_json::{json, map::Map, value::Value};
use std::collections::HashMap;
use std::default::Default;

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

pub struct Data {}

impl Data {
    pub fn parse(input: &str) -> Result<Value> {
        let mut ctx = Context::default();

        let root = Pest::parse(Rule::data, input)?
            .next()
            .expect("failed to parse the file");

        let mut data = match root.as_rule() {
            Rule::object => Self::parse_value_object(root.into_inner(), &mut ctx)?,
            Rule::array => Self::parse_value_array(root.into_inner())?,
            _ => unreachable!("data can only be of type array or object"),
        };

        Self::get_reference_values(&mut data, &mut ctx)?;
        Self::set_reference_value(&mut data, &ctx)?;

        Ok(data)
    }

    fn get_reference_values(data: &Value, ctx: &mut Context) -> Result<()> {
        for (_key, references) in ctx.references.iter_mut() {
            for reference in references.iter_mut() {
                let value = Self::get_object_value(data, &reference.name)?;
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

    fn get_object_value(data: &Value, path: &str) -> Result<Value> {
        let mut keys = path.split(".").collect::<Vec<_>>();
        let x = data.get(&keys[0]).expect("no value was found in path");
        let value = match x {
            Value::Object(_) => {
                keys.remove(0);
                Self::get_object_value(&x, keys.join(".").as_str())?
            }
            _ => x.clone(),
        };
        Ok(value)
    }

    fn parse_value_object(pairs: Pairs<Rule>, ctx: &mut Context) -> Result<Value> {
        let mut object = Map::new();
        for pair in pairs {
            let mut key_value_pair = Vec::new();
            // TODO: There must be a better way to handle key value pairs than a loop
            for (index, key_value) in pair.into_inner().enumerate() {
                dbg!(&key_value);
                let value = match index {
                    0 => {
                        let mut key = key_value.as_str().to_string();
                        Self::replace_quote_symbol(&mut key);
                        ctx.location.push(key.clone());
                        Value::String(key)
                    }
                    1 => match key_value.as_rule() {
                        Rule::null => Value::Null,
                        Rule::bool => Self::parse_value_bool(key_value.as_str())?,
                        Rule::number => Self::parse_value_number(key_value.as_str())?,
                        Rule::string => Self::parse_value_text(key_value.into_inner(), ctx)?,
                        Rule::object => Self::parse_value_object(key_value.into_inner(), ctx)?,
                        Rule::array => Self::parse_value_array(key_value.into_inner())?,
                        _ => unreachable!("unknown json value"),
                    },
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

    fn parse_value_bool(value: &str) -> Result<Value> {
        Ok(json!(value.parse::<bool>()?))
    }

    fn parse_value_number(value: &str) -> Result<Value> {
        Ok(json!(value.parse::<f64>()?))
    }

    fn parse_value_text(pairs: Pairs<Rule>, ctx: &mut Context) -> Result<Value> {
        let mut string = String::new();
        let current_location = ctx.location.clone().join(".");

        for pair in pairs {
            if pair.as_rule() == Rule::reference {
                let entry = ctx
                    .references
                    .entry(current_location.clone())
                    .or_insert(Vec::new());
                entry.push(Reference {
                    name: pair.clone().as_str().to_string(),
                    value: None,
                });
                string.push_str(format!("${{{}}}", pair.as_str()).as_str());
            } else {
                string.push_str(pair.as_str());
            }
        }
        Self::replace_quote_symbol(&mut string);
        Self::replace_double_escape(&mut string);
        Ok(Value::String(string))
    }

    fn replace_quote_symbol(string: &mut String) {
        if string.starts_with("\"") {
            *string = string.replace("\"", "");
        } else if string.starts_with("\'") {
            *string = string.replace("\'", "");
        }
    }

    fn replace_double_escape(string: &mut String) {
        *string = string.replace(r"\\", r"\")
    }

    fn parse_value_array(pairs: Pairs<Rule>) -> Result<Value> {
        let array = Vec::new();
        dbg!(&pairs);
        Ok(Value::Array(array))
    }
}
