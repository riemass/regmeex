// use super::*;

use crate::character_set::CharacterSet;

use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;

#[derive(Debug)]
pub enum RegexAST {
    Exprs(Vec<RegexAST>),
    Alterations(Vec<RegexAST>),
    Quantifier(QuantifierType, Box<RegexAST>),
    Match(CharacterSet),
}

#[derive(Debug)]
pub enum QuantifierType {
    // Number(std::ops::Range<i32>),
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
}

#[derive(Parser)]
#[grammar = "regex.pest"]
struct RegexParser;

fn partial_match(matched_rule: Pair<Rule>) -> Result<RegexAST, Error<Rule>> {
    match matched_rule.as_rule() {
        Rule::Exprs => {
            let mut exprs: Vec<_> = matched_rule
                .into_inner()
                .filter_map(|rule| partial_match(rule).ok())
                .collect();
            if exprs.len() == 1 {
                Ok(exprs.pop().unwrap())
            } else {
                Ok(RegexAST::Exprs(exprs))
            }
        }
        Rule::Alterations => {
            let mut exprs: Vec<_> = matched_rule
                .into_inner()
                .filter_map(|rule| partial_match(rule).ok())
                .collect();
            if exprs.len() == 1 {
                Ok(exprs.pop().unwrap())
            } else {
                Ok(RegexAST::Alterations(exprs))
            }
        }
        Rule::Expr => {
            let mut inner_rules = matched_rule.into_inner();
            let expr = inner_rules
                .next()
                .and_then(|x| partial_match(x).ok())
                .unwrap();
            if let Some(x) = inner_rules.next() {
                match x.as_rule() {
                    Rule::ZeroOrOne => Ok(RegexAST::Quantifier(
                        QuantifierType::ZeroOrOne,
                        Box::new(expr),
                    )),
                    Rule::ZeroOrMore => Ok(RegexAST::Quantifier(
                        QuantifierType::ZeroOrMore,
                        Box::new(expr),
                    )),
                    Rule::OneOrMore => Ok(RegexAST::Quantifier(
                        QuantifierType::OneOrMore,
                        Box::new(expr),
                    )),
                    _ => todo!(""),
                }
            } else {
                Ok(expr)
            }
        }
        Rule::Matcher => {
            let mut charset = CharacterSet::new();
            let ch = matched_rule.as_str().chars().next().unwrap() as u8;
            charset.set(ch);
            Ok(RegexAST::Match(charset))
        }
        _ => {
            unimplemented!()
        }
    }
}

pub fn parse_regex(file: &str) -> Result<RegexAST, Error<Rule>> {
    let mut value = RegexParser::parse(Rule::regex, file)?;

    println!("Begin parsing: {:?}", value);

    return partial_match(value.next().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_test() {
        let regs = [
            "a", "ab", "abc", "a|b", "a|b|c", "ab|b", "ab|ba", "a*",
        ];

        for r in regs {
            let x = RegexParser::parse(Rule::regex, r);
            println!("{r} parses into \n{:#?}", x);
            assert!(x.is_ok());
        }

    }
}
