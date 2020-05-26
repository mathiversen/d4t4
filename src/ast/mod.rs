use pest_derive::Parser as PestMacro;

#[derive(PestMacro)]
#[grammar = "ast/rules.pest"]
pub struct Ast;
