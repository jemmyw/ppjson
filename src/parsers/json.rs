use super::parser::JSONValue;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "parsers/json.pest"]
struct JSONParser;

fn unwrap_string_pair(pair: Pair<Rule>) -> Option<&str> {
    pair.into_inner().next().map(|inner| inner.as_str())
}

fn parse_json_file(input: &str) -> Result<JSONValue, pest::error::Error<Rule>> {
    let json = JSONParser::parse(Rule::value, input)?.next().unwrap();

    fn parse_value(pair: Pair<Rule>) -> JSONValue {
        match pair.as_rule() {
            Rule::object => JSONValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let key_pair = inner_rules.next();
                        let value_pair = inner_rules.next();

                        key_pair
                            .and_then(|kp| kp.into_inner().next())
                            .and_then(unwrap_string_pair)
                            .zip(value_pair.map(parse_value))
                            .unwrap_or(("none", JSONValue::Null))
                    })
                    .collect(),
            ),
            Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string => unwrap_string_pair(pair).map_or(JSONValue::Null, JSONValue::String),
            Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => JSONValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::null => JSONValue::Null,
            _ => JSONValue::Null,
        }
    }

    Ok(parse_value(json))
}

pub struct JsonParser;

impl super::parser::Parser for JsonParser {
    fn parse<'a>(&'a self, input: &'a str) -> Result<JSONValue, std::string::String> {
        parse_json_file(input).map_err(|err| err.to_string())
    }
}
