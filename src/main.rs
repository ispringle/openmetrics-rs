extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use pest::Parser;

mod metrics;

#[derive(Parser)]
#[grammar = "metric_grammar.pest"]
pub struct MetricParser;

fn main() {
    let unparsed_file = fs::read_to_string("test.prom").expect("cannot read file!");
    let raw_metrics = MetricParser::parse(Rule::metrics, &unparsed_file)
        .expect("unsuccessful parse 😥")
        .next().unwrap();

    //let mut metrics: HashMap<String, metrics::MetricGroup> = HashMap::new();
    let mut metrics = metrics::Metrics::new();

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
                        .or_insert(metrics::MetricGroup::new_with_help(help_text));
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
                                "counter" => metrics::MetricType::COUNTER,
                                "gauge" => metrics::MetricType::GAUGE,
                                "histogram" => metrics::MetricType::HISTOGRAM,
                                "summary" => metrics::MetricType::SUMMARY,
                                _ => metrics::MetricType::NONE,
                            })
                        .or_insert(
                            metrics::MetricGroup::new_with_type(type_text));
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
                            |m| m.metric.0.push(
                                metrics::add(metric_name, metric_text)))
                        .or_insert(
                            metrics::MetricGroup::new_with_metric(
                                metric_name, metric_text));
                }
                _ => {
        println!("{:#?}", line.into_inner());
                }

            }
        }
    }
    println!("{:#?}", metrics);
}
