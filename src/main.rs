extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "metric.pest"]
pub struct MetricParser;

fn main() {
    let unparsed_file = fs::read_to_string("test.prom").expect("cannot read file!");
    let metrics = MetricParser::parse(Rule::metrics, &unparsed_file)
        .expect("unsuccessful parse 😥")
        .next().unwrap();

    println!("{:?}", metrics);
}
