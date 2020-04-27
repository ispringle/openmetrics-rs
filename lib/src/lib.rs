//! # Openmetrics-rs
//!
//! ## Usage
//!
//! `Cargo.toml`
//! ```toml
//! ...
//! [dependencies]
//! openmetrics = "0.2"
//! ...
//! ```
//!
//! `main.rs`
//! ```rust
//! extern crate openmetrics;
//!
//! use std::fs;
//! use openmetrics::parse_metrics;
//! use openmetrics::metrics::Metrics;
//!
//! fn get_metrics(metric_string: String) -> Metrics {
//!     parse_metrics(metric_string)
//! }
//!
//! fn main() {
//! let unparsed_file = fs::read_to_string("test.prom").expect("cannot read file!");
//! println!("{:#?}", get_metrics(unparsed_file));
//! }
//! ```

extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use pest::Parser;

pub mod metrics;
use metrics::{MetricGroupBuilder, Metrics, MetricType};

#[derive(Parser)]
#[grammar = "metric_grammar.pest"]
pub struct MetricParser;

pub fn parse_metrics(unparsed_metrics: String) -> Metrics {
    let raw_metrics = MetricParser::parse(Rule::metrics, &unparsed_metrics)
        .expect("unsuccessful parse 😥")
        .next().unwrap();

    let mut metrics = Metrics::new();
    for raw_metric in raw_metrics.into_inner() {
        let mut base_metric_name = String::new();
        for line in raw_metric.into_inner() {
            match line.as_rule() {
                Rule::helpLine => {
                    let mut inner = line.into_inner();
                    let metric_name = inner.next().unwrap().as_str().to_string();
                    base_metric_name = metric_name.clone();
                    let help_text = match inner.next() {
                        Some(t) => t.as_str(),
                        _ => "",
                    };
                    metrics.entry(metric_name)
                        .and_modify(|m| m.help = help_text.to_string())
                        .or_insert(MetricGroupBuilder::new()
                                    .help(help_text)
                                    .build());
                }
                Rule::typeLine => {
                    let mut inner = line.into_inner();
                    let metric_name = inner.next().unwrap().as_str().to_string();
                    base_metric_name = metric_name.clone();
                    let type_text = match inner.next() {
                        Some(t) => t.as_str(),
                        _ => "",
                    };
                    metrics.entry(metric_name)
                        .and_modify(
                            |m| m.r#type = match type_text {
                                "counter" => MetricType::COUNTER,
                                "gauge" => MetricType::GAUGE,
                                "histogram" => MetricType::HISTOGRAM,
                                "summary" => MetricType::SUMMARY,
                                _ => MetricType::NONE,
                            })
                        .or_insert(MetricGroupBuilder::new()
                                    .r#type(type_text)
                                    .build());
                }
                Rule::metricLine => {
                    let mut inner = line.into_inner();
                    let metric_name = inner.next().unwrap().as_str();
                    let metric_text = match inner.next() {
                        Some(t) => t.as_str(),
                        _ => "",
                    };
                    metrics.entry(base_metric_name.clone())
                        .and_modify(
                            |m| m.labels.0.push(
                                metrics::add(metric_name, metric_text)))
                        .or_insert(MetricGroupBuilder::new()
                                    .label(metric_name, metric_text)
                                    .build());
                }
                _ => { () }
            }
        }
    }
    metrics
}
