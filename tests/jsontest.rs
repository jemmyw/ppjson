use ppjson::parsers::json::JsonParser;
use ppjson::parsers::parser::{JSONValue, Parser};
use ppjson::serialize;

fn test_serialize(v: JSONValue) -> std::string::String {
    serialize(&v)
}

#[test]
fn correct_json() {
    let original = r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"},"array":[1,2,3]}"#;

    let parser = JsonParser {};
    let value = parser.parse(&original).map(test_serialize).unwrap();

    assert_eq!(
        value,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"},"array":[1,2,3]}"#
    );
}

#[test]
fn partial_json() {
    let original = r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"},"array":[1,2,3]}"#;
    let outputs = [
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"},"array":[1,2,3]}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"},"array":[1,2,3]}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"},"array":[1,2,null]}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"},"array":[1,2]}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"},"array":[1,null]}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"},"array":[1]}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"},"array":[null]}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"},"array":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"world"}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"worl"}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"wor"}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"wo"}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":"w"}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":""}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":{"hello":null}}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null,"nested":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123}"#,
        r#"{"hello":"world","bool":true,"num":123}"#,
        r#"{"hello":"world","bool":true,"num":123}"#,
        r#"{"hello":"world","bool":true,"num":123,"null":null}"#,
        r#"{"hello":"world","bool":true,"num":123}"#,
        r#"{"hello":"world","bool":true,"num":123}"#,
        r#"{"hello":"world","bool":true,"num":123}"#,
        r#"{"hello":"world","bool":true,"num":123}"#,
        r#"{"hello":"world","bool":true,"num":123}"#,
        r#"{"hello":"world","bool":true,"num":123}"#,
        r#"{"hello":"world","bool":true,"num":123}"#,
        r#"{"hello":"world","bool":true,"num":123}"#,
        r#"{"hello":"world","bool":true,"num":12}"#,
        r#"{"hello":"world","bool":true,"num":1}"#,
        r#"{"hello":"world","bool":true,"num":null}"#,
        r#"{"hello":"world","bool":true}"#,
    ];
    let parser = JsonParser {};

    for (i, output) in outputs.iter().enumerate() {
        let input = original.split_at(original.len() - (i + 1)).0;
        let value = parser.parse(input).map(test_serialize).unwrap();
        assert_eq!(value, output.to_string());
    }
}
