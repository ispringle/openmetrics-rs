# OpenMetrics-rs
![Rust](https://github.com/pard68/openmetrics-rs/workflows/Rust/badge.svg?branch=master&event=push)
[![Docs](https://docs.rs/openmetrics/badge.svg)](https://docs.rs/openmetrics)
[![Crates.io](https://img.shields.io/crates/v/openmetrics)](https://crates.io/crates/openmetrics)

An OpenMetrics parser in Rust 🦀

## Example

```rust
extern crate openmetrics;
use std::fs;

fn main() {
    let unparsed_file = fs::read_to_string("test.prom").expect("cannot read file!");
    openmetrics::parse_metrics(unparsed_file);
}
```

Sample metrics data:
```shell
{
    "go_goroutines": MetricGroup {
        help: "Number of goroutines that currently exist.",
        type: GAUGE,
        metric: Metric(
            [
                {
                    "value": "9",
                },
            ],
        ),
    },
}
```

# TODO
- [X] bin -> lib
- [ ] consume URLs
- [X] Add label and metric logic to convert parsed pest data into usable data
- [ ] parse labels within each metricLine
- [ ] reconsider best way to handle labels and special labels (such as quartile)
