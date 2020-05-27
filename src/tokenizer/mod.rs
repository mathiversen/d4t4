use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "tokenizer/rules.pest"]
pub struct Tokenizer;
