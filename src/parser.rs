extern crate pest;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "data.pest"]
pub struct DataParser;

use kugel::Inst;

fn main() {
    let src = "a b c";
    use pest::Parser;
    let mut parse_result = DataParser::parse(Rule::data, src).unwrap();
    let data = parse_result.next().unwrap();

    for pair_def in data.into_inner() {
        let s = pair_def.as_str();
        let _one = Inst::one(s);
    }
}