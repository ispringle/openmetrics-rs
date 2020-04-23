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
pub struct Metric( Vec<Label> );

#[derive(Default, Debug)]
pub struct MetricGroup {
    pub help: String,
    pub r#type: MetricType,
    pub label: Label,
    pub metric: Vec<Metric>,
}

impl MetricGroup {
    pub fn new_with_help(help_string: &str) -> Self {
        MetricGroup {
            help: help_string.to_string(),
            r#type: Default::default(),
            label: Default::default(),
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
            label: Default::default(),
            metric: Default::default()
        }
    }

    pub fn new_with_label(label_key: &str, label_value: &str) -> Self {
        MetricGroup {
            help: Default::default(),
            r#type: Default::default(),
            label: Default::default(),
            metric: Default::default()
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
