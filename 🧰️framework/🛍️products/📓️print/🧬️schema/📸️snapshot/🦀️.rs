//! 📸️ Authored chart snapshot shared by mutation replay and print inference.
use protocol::{DslValue, FromValue, ToValue};

#[path="🪶️sqlite/🦀️.rs"]
mod sqlite;
#[path="🚦️native/🦀️.rs"]
mod sqlite_native;

#[derive(Clone, Debug, ToValue, FromValue,semio_framework_dsl_record_derive::DslRecord)]
#[value(deny_unknown_fields)]
pub struct ChartSnapshot {
    pub chart: DslValue,
}

impl PartialEq for ChartSnapshot {fn eq(&self,other:&Self)->bool{crate::diff::chart_values_equal(&self.chart,&other.chart)}}

impl Default for ChartSnapshot {
    fn default() -> Self {
        Self { chart: DslValue::object([("width".into(), DslValue::uint(80)), ("height".into(), DslValue::uint(40)), ("layers".into(), DslValue::Array(Vec::new()))]) }
    }
}
