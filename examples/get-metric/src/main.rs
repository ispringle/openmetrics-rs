extern crate openmetrics;
use std::fs;

fn main() {
    let unparsed_file = fs::read_to_string("test.prom").expect("cannot read file!");
    openmetrics::parse_metrics(unparsed_file);
}
