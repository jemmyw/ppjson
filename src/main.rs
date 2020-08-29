extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error;
use pest::iterators::Pair;
use pest::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "json.pest"]
struct JSONParser;

enum JSONValue<'a> {
    Object(Vec<(&'a str, JSONValue<'a>)>),
    Array(Vec<JSONValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

fn unwrap_string_pair(pair: Pair<Rule>) -> &str {
    match pair.into_inner().next() {
        Some(inner) => inner.as_str(),
        None => unreachable!(),
    }
}

fn parse_json_file(file: &str) -> Result<JSONValue, Error<Rule>> {
    let json = JSONParser::parse(Rule::value, file)?.next().unwrap();

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

fn serialize_jsonvalue(val: &JSONValue) -> String {
    use JSONValue::*;

    match val {
        Object(o) => {
            let contents: Vec<_> = o
                .iter()
                .map(|(name, value)| format!("\"{}\":{}", name, serialize_jsonvalue(value)))
                .collect();
            format!("{{{}}}", contents.join(","))
        }
        Array(a) => {
            let contents: Vec<_> = a.iter().map(serialize_jsonvalue).collect();
            format!("[{}]", contents.join(","))
        }
        String(s) => format!("\"{}\"", s),
        Number(n) => format!("{}", n),
        Boolean(b) => format!("{}", b),
        Null => format!("null"),
    }
}

fn main() {
    let unparsed_file = fs::read_to_string("data.json").expect("cannot read file");
    let value = parse_json_file(&unparsed_file);

    match value {
        Ok(json) => println!("{}", serialize_jsonvalue(&json)),
        Err(err) => println!("{}", err),
    }
}
