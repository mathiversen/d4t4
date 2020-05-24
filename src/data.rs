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

        let root = Pest::parse(Rule::json, input)?.next().unwrap();

        let mut json = match root.as_rule() {
            Rule::object => Self::build_object(root.into_inner(), &mut ctx)?,
            Rule::array => Self::build_array(root.into_inner())?,
            _ => unreachable!("json can only be ofe type array or object"),
        };

        Self::get_reference_values(&mut json, &mut ctx)?;
        Self::set_reference_value(&mut json, &ctx)?;

        Ok(json)
    }

    fn get_reference_values(json: &Value, ctx: &mut Context) -> Result<()> {
        for (key, references) in ctx.references.iter_mut() {
            for reference in references.iter_mut() {
                let value = Self::get_object_value(json, &reference.name)?;
                reference.value = Some(value);
            }
        }
        Ok(())
    }

    fn set_reference_value(json: &mut Value, ctx: &Context) -> Result<()> {
        for (key, references) in ctx.references.iter() {
            for reference in references.iter() {
                if let Some(prev) = json.get_mut(key) {
                    let new_value = match *prev {
                        Value::String(ref s) => Value::String(s.clone().replace(
                            format!("${{{}}}", reference.name).as_str(),
                            reference.value.as_ref().unwrap().as_str().unwrap(),
                        )),
                        _ => unreachable!("nej"),
                    };
                    *prev = new_value;
                }
            }
        }
        Ok(())
    }

    fn get_object_value(json: &Value, path: &str) -> Result<Value> {
        let mut keys = path.split(".").collect::<Vec<_>>();
        let x = json.get(&keys[0]).unwrap();
        let value = match x {
            Value::Object(_) => {
                keys.remove(0);
                Self::get_object_value(&x, keys.join(".").as_str())?
            }
            _ => x.clone(),
        };
        Ok(value)
    }

    fn build_object(pairs: Pairs<Rule>, ctx: &mut Context) -> Result<Value> {
        let mut object = Map::new();
        for pair in pairs {
            let mut key_value_pair = Vec::new();
            for (index, key_value) in pair.into_inner().enumerate() {
                let value = match index {
                    0 => {
                        let mut key = key_value.as_str().to_string();
                        Self::replace_quote_symbol(&mut key);
                        ctx.location.push(key.clone());
                        Value::String(key)
                    }
                    1 => match key_value.as_rule() {
                        Rule::null => Value::Null,
                        Rule::number | Rule::bool => json!(key_value.as_str()),
                        Rule::string => Self::get_text_and_references(key_value.into_inner(), ctx)?,
                        Rule::object => Self::build_object(key_value.into_inner(), ctx)?,
                        Rule::array => Self::build_array(key_value.into_inner())?,
                        _ => unreachable!("unknown json value"),
                    },
                    _ => unreachable!("pair should only be two"),
                };
                key_value_pair.push(value);
            }
            object.insert(
                key_value_pair[0].clone().as_str().unwrap().to_string(),
                key_value_pair[1].clone(),
            );
            ctx.location.pop();
        }
        Ok(Value::Object(object))
    }

    fn get_text_and_references(pairs: Pairs<Rule>, ctx: &mut Context) -> Result<Value> {
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
        Ok(Value::String(string))
    }

    fn replace_quote_symbol(string: &mut String) {
        if string.starts_with("\"") {
            *string = string.replace("\"", "");
        } else if string.starts_with("\'") {
            *string = string.replace("\'", "");
        } else if string.starts_with("`") {
            *string = string.replace("`", "");
        }
    }

    fn build_array(pairs: Pairs<Rule>) -> Result<Value> {
        let array = Vec::new();
        dbg!(&pairs);
        Ok(Value::Array(array))
    }
}
