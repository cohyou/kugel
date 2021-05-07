extern crate pest;

#[macro_use]
extern crate pest_derive;

#[derive(Parser)]
#[grammar = "data.pest"]
pub struct DataParser;

fn main() {
    let src = "a b c";
    use pest::Parser;
    let data = DataParser::parse(Rule::data, src);
    let _ = dbg!(data);
}