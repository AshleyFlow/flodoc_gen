use pest::Parser;
use pest_derive::Parser;

use crate::parser;

#[derive(Parser)]
#[grammar = ".pest"]
pub struct FlodocParser;

pub fn parse_content(content: String) {
    println!("{:#?}", FlodocParser::parse(parser::Rule::Doc, &content));
}
