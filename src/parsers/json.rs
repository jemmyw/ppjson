use super::parser::JSONValue;
use pest::iterators::Pair;
use pest::Parser;

#[derive(Parser)]
#[grammar = "parsers/json.pest"]
struct JSONParser;

fn unwrap_string_pair(pair: Pair<Rule>) -> &str {
    match pair.into_inner().next() {
        Some(inner) => inner.as_str(),
        None => unreachable!(),
    }
}

fn parse_json_file(input: &str) -> Result<JSONValue, pest::error::Error<Rule>> {
    let json = JSONParser::parse(Rule::value, input)?.next().unwrap();

    fn parse_value(pair: Pair<Rule>) -> JSONValue {
        match pair.as_rule() {
            Rule::object => JSONValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut inner_rules = pair.into_inner();

                        match (inner_rules.next(), inner_rules.next()) {
                            (Some(key_pair), Some(value_pair)) => {
                                let name =
                                    unwrap_string_pair(key_pair.into_inner().next().unwrap());
                                let value = parse_value(value_pair);
                                (name, value)
                            }
                            _ => ("none", JSONValue::Null),
                        }
                    })
                    .collect(),
            ),
            Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
            Rule::string => JSONValue::String(unwrap_string_pair(pair)),
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
