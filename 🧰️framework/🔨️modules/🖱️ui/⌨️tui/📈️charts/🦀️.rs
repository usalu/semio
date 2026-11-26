//! 📈️ tui charts: the domain-neutral chart primitives of the terminal target. Sparkline, LineChart, Gauge and StatCell are pure
//! functions from data (multi-resolution buckets with min, max, mean and last) to a cell grid, with en/de labels from the
//! compile-checked label macro. They are wired here, outside the widgets crate, so that crate keeps its source boundary.

#[path = "../../🧱️elements/🧮️ChartScale/🎯️targets/⌨️tui/🦀️.rs"]
pub mod chart_scale;

#[path = "../../🧱️elements/🔢️StatCell/🎯️targets/⌨️tui/🦀️.rs"]
pub mod stat;

#[path = "../../🧱️elements/📈️Sparkline/🎯️targets/⌨️tui/🦀️.rs"]
pub mod sparkline;

#[path = "../../🧱️elements/📉️LineChart/🎯️targets/⌨️tui/🦀️.rs"]
pub mod line_chart;

#[path = "../../🧱️elements/🌡️Gauge/🎯️targets/⌨️tui/🦀️.rs"]
pub mod gauge;
