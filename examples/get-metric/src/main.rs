extern crate openmetrics;
use std::fs;
use openmetrics::parse_metrics;
use openmetrics::metrics::Metrics;

fn get_metrics(metric_string: String) -> Metrics {
    parse_metrics(metric_string)
}

fn main() {
    let unparsed_file = fs::read_to_string("test.prom").expect("cannot read file!");
    println!("{:#?}", get_metrics(unparsed_file));
}
