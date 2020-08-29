extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parsers;
use parsers::json::parse_json_file;
use parsers::serializer::serialize_jsonvalue;
use std::fs;

fn main() {
    let unparsed_file = fs::read_to_string("data.json").expect("cannot read file");
    let value = parse_json_file(&unparsed_file);

    match value {
        Ok(json) => println!("{}", serialize_jsonvalue(&json)),
        Err(err) => println!("{}", err),
    }
}
