//! Serialize dag to stdio.svg.
use crate::artifacts::dag::DagSnapshot;
use semio_s_plugin_stdio::artifacts::svg::SvgSnapshot;

pub fn register() {}

pub fn serialize(from: &DagSnapshot) -> Result<SvgSnapshot, store::PackError> {
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    serde_json::from_value(value).map_err(|e| store::PackError::Schema(e.to_string()))
}
