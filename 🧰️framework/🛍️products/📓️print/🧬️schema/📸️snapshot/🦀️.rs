//! 📸️ Authored chart snapshot shared by mutation replay and print inference.
use semio_framework_value::DslValue;


#[derive(Clone, Debug, semio_framework_value::RetireOwned)]
pub struct ChartSnapshot {
    pub chart: DslValue,
}

impl PartialEq for ChartSnapshot {fn eq(&self,other:&Self)->bool{crate::diff::chart_values_equal(&self.chart,&other.chart)}}

impl Default for ChartSnapshot {
    fn default() -> Self {
        Self { chart: DslValue::object([("width".into(), DslValue::uint(80)), ("height".into(), DslValue::uint(40)), ("layers".into(), DslValue::Array(Vec::new()))]) }
    }
}
