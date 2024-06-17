use crate::{parser, util::Error};
use pest::{
    error::{Error as PestErr, ErrorVariant},
    Parser,
};
use pest_derive::Parser;

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

pub fn parse_content(content: String) -> Result<(), Error> {
    let doc = next_pair!(FlodocParser::parse(parser::Rule::Doc, &content)?);

    match doc.as_rule() {
        Rule::Doc => {
            let doc_inner = doc.into_inner();

            for item in doc_inner {
                if matches!(item.as_rule(), Rule::EOI) {
                    break;
                }

                match item.as_rule() {
                    Rule::Tag => {
                        let mut item_inner = item.into_inner();

                        next_pair!(item_inner); // eat '<'
                        let tag = next_pair!(item_inner).as_str();
                        let string = next_pair!(item_inner).as_str();
                        next_pair!(item_inner); // eat '>'

                        println!("{tag}: {string}");
                    }
                    Rule::AdvTag => {
                        let mut item_inner = item.into_inner();

                        next_pair!(item_inner); // eat '-'
                        let tag = next_pair!(item_inner).as_str();
                        let string = next_pair!(item_inner)
                            .as_str()
                            .trim_start_matches("{")
                            .trim_start_matches("\n")
                            .trim_end_matches("\n")
                            .trim_end_matches("}");

                        println!("{tag}: {string}");
                    }
                    Rule::CodeBlock => {
                        let mut item_inner = item.into_inner();

                        next_pair!(item_inner); // eat '```'
                        let lang = next_pair!(item_inner).as_str();
                        let code = next_pair!(item_inner)
                            .as_str()
                            .trim_start_matches("{")
                            .trim_start_matches("\n")
                            .trim_end_matches("\n")
                            .trim_end_matches("}");
                        next_pair!(item_inner); // eat '```'

                        println!("```{lang}\n{code}\n```");
                    }
                    _ => {}
                }

                // println!("{:?}", item.as_rule());
            }
        }
        _ => {
            return Err(Error::Parser(PestErr::new_from_span(
                ErrorVariant::CustomError {
                    message: "Expected document".into(),
                },
                doc.as_span(),
            )))
        }
    }

    Ok(())
}
