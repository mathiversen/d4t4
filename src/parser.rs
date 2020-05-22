use pest_derive::Parser as PestMacro;

#[derive(PestMacro)]
#[grammar = "grammar.pest"]
pub struct Pest;
