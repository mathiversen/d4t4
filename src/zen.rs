use crate::parser::{Pest, Rule};
use anyhow::Result;
use pest::{iterators::Pairs, Parser as PestParser};
use serde_json::{map::Map, value::Value};

pub struct Zen {
    value: Value,
}

impl Zen {
    pub fn parse(input: &str) -> Result<Self> {
        let root = Pest::parse(Rule::json, input)?.next().unwrap();
        let json = match root.as_rule() {
            Rule::object => Self::build_object(root.into_inner())?,
            Rule::array => Self::build_array(root.into_inner())?,
            _ => todo!("handle edge case"),
        };
        Ok(Self { value: json })
    }

    fn build_object(pairs: Pairs<Rule>) -> Result<Value> {
        let mut object = Map::new();
        dbg!(&pairs);
        Ok(Value::Object(object))
    }

    fn build_array(pairs: Pairs<Rule>) -> Result<Value> {
        let mut array = Vec::new();
        dbg!(&pairs);
        Ok(Value::Array(array))
    }
}
