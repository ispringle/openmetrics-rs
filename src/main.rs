extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use pest::Parser;

mod metrics;

#[derive(Parser)]
#[grammar = "metric.pest"]
pub struct MetricParser;

fn main() {
    let unparsed_file = fs::read_to_string("test.prom").expect("cannot read file!");
    let raw_metrics = MetricParser::parse(Rule::metrics, &unparsed_file)
        .expect("unsuccessful parse 😥")
        .next().unwrap();

    //let mut metrics: HashMap<String, metrics::MetricGroup> = HashMap::new();
    let mut metrics = metrics::Metrics::new();

    for raw_metric in raw_metrics.into_inner() {
        for line in raw_metric.into_inner() {
            match line.as_rule() {
                Rule::helpLine => {
                    let mut inner = line.into_inner();
                    metrics.entry(inner.next().unwrap().as_str().to_string())
                        .and_modify(
                            |m| m.help = inner.next().unwrap().as_str().to_string()
                        )
                        .or_insert(
                            metrics::MetricGroup::new_with_help(
                                inner.next().unwrap().as_str()
                            )
                        );
                }
                Rule::typeLine => {
                    let mut inner = line.into_inner();
                    metrics.entry(inner.next().unwrap().as_str().to_string())
                        .and_modify(
                            |m| m.r#type = match inner.next().unwrap().as_str() {
                                "counter" => metrics::MetricType::COUNTER,
                                "gauge" => metrics::MetricType::GAUGE,
                                "histogram" => metrics::MetricType::HISTOGRAM,
                                "summary" => metrics::MetricType::SUMMARY,
                                _ => metrics::MetricType::NONE,
                            })
                        .or_insert(
                            match inner.next() {
                                Some(t) => metrics::MetricGroup::new_with_type(t.as_str()),
                                _ => metrics::MetricGroup::new_with_type(""),
                            });
                }
                _ => {
        println!("{:#?}", line.into_inner());
                }

            }
        }
    }

println!("{:#?}", metrics);
}
