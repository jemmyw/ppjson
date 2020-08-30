use super::parser::JSONValue;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "parsers/hash.pest"]
struct HASHParser;

fn unwrap_string_pair(pair: Pair<Rule>) -> Option<&str> {
    pair.into_inner().next().map(|inner| inner.as_str())
}

fn parse_hash(input: &str) -> Result<JSONValue, pest::error::Error<Rule>> {
    let hash = HASHParser::parse(Rule::value, input)?.next().unwrap();

    fn parse_value(pair: Pair<Rule>) -> JSONValue {
        match pair.as_rule() {
            Rule::object => JSONValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();
                        let key_pair = inner_rules.next();
                        let value_pair = inner_rules.next();

                        key_pair
                            .and_then(|key_pair| match key_pair.as_rule() {
                                Rule::rocket_key => {
                                    key_pair.into_inner().next().and_then(|key_inner| {
                                        match key_inner.as_rule() {
                                            Rule::string => unwrap_string_pair(key_inner),
                                            Rule::symbol => Some(key_inner.into_inner().as_str()),
                                            _ => None,
                                        }
                                    })
                                }
                                Rule::symbol_inner => Some(key_pair.as_str()),
                                _ => None,
                            })
                            .zip(value_pair.map(parse_value))
                            .unwrap_or(("none", JSONValue::Null))
                    })
                    .collect(),
            ),
            Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string => unwrap_string_pair(pair).map_or(JSONValue::Null, JSONValue::String),
            Rule::symbol => JSONValue::String(pair.as_str()),
            Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
            Rule::boolean => JSONValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::nil => JSONValue::Null,
            _ => JSONValue::Null,
        }
    }

    Ok(parse_value(hash))
}

pub struct HashParser {}

impl super::parser::Parser for HashParser {
    fn parse<'a>(&'a self, input: &'a str) -> Result<JSONValue, std::string::String> {
        parse_hash(input).map_err(|err| err.to_string())
    }
}
