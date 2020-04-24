use std::collections::HashMap;

pub type LabelName = String;
pub type LabelValue = String;
pub type Label = HashMap<LabelName, LabelValue>;

#[derive(Debug)]
pub enum MetricType {
    COUNTER,
    GAUGE,
    HISTOGRAM,
    NONE,
    SUMMARY,
}

impl Default for MetricType {
    fn default() -> Self { MetricType::NONE }
}

#[derive(Default, Debug)]
pub struct Metric( pub Vec<Label> );

impl Metric {
    pub fn add(metric_name: &str, metric_text: &str) -> Label {
        let mut metricHash = HashMap::new();
        metricHash.insert("value".to_string(), metric_text.to_string());
        metricHash
    }
}

#[derive(Default, Debug)]
pub struct MetricGroup {
    pub help: String,
    pub r#type: MetricType,
    pub metric: Metric,
}

impl MetricGroup {
    pub fn new_with_help(help_string: &str) -> Self {
        MetricGroup {
            help: help_string.to_string(),
            r#type: Default::default(),
            metric: Default::default()
        }
    }

    pub fn new_with_type(type_string: &str) -> Self {
        MetricGroup {
            help: Default::default(),
            r#type: match type_string {
                "counter" => MetricType::COUNTER,
                "gauge" => MetricType::GAUGE,
                "histogram" => MetricType::HISTOGRAM,
                "summary" => MetricType::SUMMARY,
                _ => MetricType::NONE,
            },
            metric: Default::default()
        }
    }

    pub fn new_with_metric(metric_name: &str, metric_text: &str) -> Self {
        MetricGroup {
            help: Default::default(),
            r#type: Default::default(),
            metric: Default::default()
            //metric: metric_text.to_string()
        }
    }
}

pub type Metrics = HashMap<String, MetricGroup>;
//#[derive(Default, Debug)]
//pub struct Metrics( HashMap<String, MetricGroup> );
//impl Metrics {
//    pub fn new() -> Self {
//        Default::default()
//    }
//}

pub fn add(metric_name: &str, metric_text: &str) -> Label {
    let mut metricHash = HashMap::new();
    metricHash.insert("value".to_string(), metric_text.to_string());
    metricHash
}
