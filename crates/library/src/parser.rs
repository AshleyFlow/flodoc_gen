use crate::{parser, util::Error};
use pest::{
    error::{Error as PestErr, ErrorVariant},
    iterators::Pair,
    Parser,
};
use pest_derive::Parser;
use serde_json::json;

#[derive(Parser)]
#[grammar = ".pest"]
pub struct FlodocParser;

macro_rules! next_pair {
    ($pairs:expr) => {
        match $pairs.next() {
            None => panic!("Reached the end of pairs"),
            Some(pair) => pair,
        }
    };
}

pub fn parse_pair(item: Pair<'_, Rule>) -> Result<serde_json::Value, Error> {
    match item.as_rule() {
        Rule::Tag => {
            let mut item_inner = item.into_inner();

            next_pair!(item_inner); // eat '<'
            let tag = next_pair!(item_inner).as_str();
            let string = next_pair!(item_inner).as_str();
            next_pair!(item_inner); // eat '>'

            Ok(json!({
                "tag": tag,
                "innerHTML": string
            }))
        }
        Rule::AdvTag => {
            let mut item_inner = item.into_inner();

            next_pair!(item_inner); // eat '-'
            let tag = next_pair!(item_inner).as_str();
            let string = next_pair!(item_inner)
                .as_str()
                .trim_start_matches('{')
                .trim_start_matches('\n')
                .trim_end_matches('\n')
                .trim_end_matches('}');

            Ok(json!({
                "tag": tag,
                "innerHTML": string
            }))
        }
        Rule::CodeBlock => {
            let mut item_inner = item.into_inner();

            next_pair!(item_inner); // eat '```'
            let lang = next_pair!(item_inner).as_str();
            let code = next_pair!(item_inner)
                .as_str()
                .trim_start_matches('{')
                .trim_start_matches('\n')
                .trim_end_matches('\n')
                .trim_end_matches('}');
            next_pair!(item_inner); // eat '```'

            Ok(json!({
                "tag": "codeblock",
                "lang": lang,
                "code": code
            }))
        }
        _ => {
            return Err(Error::Parser(Box::new(PestErr::new_from_span(
                ErrorVariant::CustomError {
                    message: "Got unexpected".into(),
                },
                item.as_span(),
            ))))
        }
    }
}

pub fn parse_content(content: String) -> Result<serde_json::Value, Error> {
    let doc = next_pair!(FlodocParser::parse(parser::Rule::Doc, &content)?);
    let mut json_values = vec![];

    match doc.as_rule() {
        Rule::Doc => {
            let doc_inner = doc.into_inner();

            for item in doc_inner {
                if matches!(item.as_rule(), Rule::EOI) {
                    break;
                }

                json_values.push(parse_pair(item)?);
            }
        }
        _ => {
            return Err(Error::Parser(Box::new(PestErr::new_from_span(
                ErrorVariant::CustomError {
                    message: "Expected document".into(),
                },
                doc.as_span(),
            ))))
        }
    }

    Ok(serde_json::Value::from(json_values))
}
