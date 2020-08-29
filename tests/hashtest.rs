use ppjson::parsers::hash::HashParser;
use ppjson::parsers::parser::{JSONValue, Parser};
use ppjson::serialize;

fn test_serialize(v: JSONValue) -> std::string::String {
    serialize(&v)
}

fn parse(input: &str) -> std::string::String {
    let parser = HashParser {};
    parser.parse(input).map(test_serialize).unwrap()
}

#[test]
fn correct_hashes() {
    assert_eq!(parse(r#"{"hello" => "world"}"#), r#"{"hello":"world"}"#);

    // let original = r#"{"hello" => "world","bool"=>true,"num"=>123,symbol_key_1: "a", :symbol_key_2 => "b", "symbol_val" => :c, "nested" => {"hello" => "world"},"array" => [1,2,3]}"#;

    // let value = parser.parse(&original).map(test_serialize).unwrap();

    // assert_eq!(
    //     value,
    //     r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"},"array":[1,2,3]}"#
    // );
}
