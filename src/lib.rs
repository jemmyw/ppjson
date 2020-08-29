extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parsers;
use parsers::parser::{serialize_jsonvalue, Parser};
use std::fs;

fn main() {
    let unparsed_file = fs::read_to_string("fixtures/data.json").expect("cannot read file");

    let parser = parsers::json::JsonParser {};
    let value = parser.parse(&unparsed_file);

    match value {
        Ok(json) => println!("{}", serialize_jsonvalue(&json)),
        Err(err) => println!("{}", err),
    }
}
