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
        let mut metric_hash = HashMap::new();
        metric_hash.insert("value".to_string(), metric_text.to_string());
        metric_hash
    }
}

#[derive(Default, Debug)]
pub struct MetricGroup {
    pub help: String,
    pub r#type: MetricType,
    pub metric: Metric,
}

#[derive(Default, Debug)]
pub struct MetricGroupBuilder {
    pub help: String,
    pub r#type: MetricType,
    pub metric: Metric,
}

impl MetricGroupBuilder {
    pub fn new() -> Self {
        Self {
            help: Default::default(),
            r#type: Default::default(),
            metric: Default::default(),
        }
    }
    pub fn help(mut self, help_string: &str) -> Self {
        self.help = help_string.to_string();
        self
    }

    pub fn r#type(mut self, type_string: &str) -> Self {
        self.r#type = match type_string {
            "counter" => MetricType::COUNTER,
            "gauge" => MetricType::GAUGE,
            "histogram" => MetricType::HISTOGRAM,
            "summary" => MetricType::SUMMARY,
            _ => MetricType::NONE,
        };
        self
    }

    pub fn metric(mut self, metric_name: &str, metric_text: &str) -> Self {
        self.metric = Default::default();
        self
    }

    pub fn build(self) -> MetricGroup {
        MetricGroup {
            help: self.help,
            r#type: self.r#type,
            metric: self.metric,
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
    let mut metric_hash = HashMap::new();
    metric_hash.insert("value".to_string(), metric_text.to_string());
    metric_hash
}
