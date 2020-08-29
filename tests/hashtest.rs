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
    assert_eq!(parse(r#"{:hello => "world"}"#), r#"{"hello":"world"}"#);
    assert_eq!(parse(r#"{hello: "world"}"#), r#"{"hello":"world"}"#);
    assert_eq!(
        parse(r#"{"hello" => "world", :how_are => "you", doing: "well"}"#),
        r#"{"hello":"world","how_are":"you","doing":"well"}"#
    );
    assert_eq!(parse(r#"{"a" => 1}"#), r#"{"a":1}"#);
    assert_eq!(parse(r#"{"a" => nil}"#), r#"{"a":null}"#);
    assert_eq!(parse(r#"{"a" => true}"#), r#"{"a":true}"#);
    assert_eq!(
        parse(r#"{"a" => [1,"2",'3',nil,true,false,[1],{"b"=>"c"}]}"#),
        r#"{"a":[1,"2","3",null,true,false,[1],{"b":"c"}]}"#
    );
    assert_eq!(parse(r#"{"a" => {"b" => "c"}}"#), r#"{"a":{"b":"c"}}"#);
}

#[test]
fn partial_json() {
    let original = r#"{"h" => "w", "bool"=>true, "num"=>123, sym_1: "a", :sym_2 => "b", "sym" => :c, "nested" => {"h" => "w"}, "array" => [1,2]}"#;
    let outputs = [
        r#"{"h":"w","bool":true,"num":123,"sym_1":"a","sym_2":"b","sym":":c","nested":{"h":"w"},"array":[1,2]}"#,
        r#"{"h":"w","bool":true,"num":123,"sym_1":"a","sym_2":"b","sym":":c","nested":{"h":"w"},"array":[1,2]}"#,
        r#"{"h":"w","bool":true,"num":123,"sym_1":"a","sym_2":"b","sym":":c","nested":{"h":"w"},"array":[1,null]}"#,
        r#"{"h":"w","bool":true,"num":123,"sym_1":"a","sym_2":"b","sym":":c","nested":{"h":"w"},"array":[1]}"#,
        r#"{"h":"w","bool":true,"num":123,"sym_1":"a","sym_2":"b","sym":":c","nested":{"h":"w"},"array":[null]}"#,
        r#"{"h":"w","bool":true,"num":123,"sym_1":"a","sym_2":"b","sym":":c","nested":{"h":"w"},"array":null}"#,
        r#"{"h":"w","bool":true,"num":123,"sym_1":"a","sym_2":"b","sym":":c","nested":{"h":"w"},"array":null}"#,
        r#"{"h":"w","bool":true,"num":123,"sym_1":"a","sym_2":"b","sym":":c","nested":{"h":"w"}}"#,
    ];

    for (i, output) in outputs.iter().enumerate() {
        let input = original.split_at(original.len() - (i + 1)).0;
        let value = parse(input);
        assert_eq!(
            value,
            output.to_string(),
            "\n input: {}\n line: {}",
            input,
            i + 1
        );
    }
}
