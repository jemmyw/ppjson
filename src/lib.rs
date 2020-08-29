extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod parsers;
use parsers::parser;

pub fn serialize(json_value: &parser::JSONValue) -> std::string::String {
    parser::serialize_jsonvalue(json_value)
}

// fn main() {
//     let unparsed_file = fs::read_to_string("fixtures/data.json").expect("cannot read file");

//     let parser = parsers::json::JsonParser {};
//     let value = parser.parse(&unparsed_file);

//     match value {
//         Ok(json) => println!("{}", serialize_jsonvalue(&json)),
//         Err(err) => println!("{}", err),
//     }
// }
