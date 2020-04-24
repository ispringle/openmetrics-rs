# OpenMetrics-rs
![Rust](https://github.com/pard68/openmetrics-rs/workflows/Rust/badge.svg)

An OpenMetrics parser in Rust 🦀

## Example

```
extern crate openmetrics;
use std::fs;

fn main() {
    let unparsed_file = fs::read_to_string("test.prom").expect("cannot read file!");
    openmetrics::parse_metrics(unparsed_file);
}
```

# TODO
- [X] bin -> lib
- [ ] consume URLs
- [X] Add label and metric logic to convert parsed pest data into usable data
- [ ] parse labels within each metricLine
- [ ] reconsider best way to handle labels and special labels (such as quartile)
