//! Serialize dag to stdio.png.
use crate::artifacts::dag::DagSnapshot;
use semio_s_plugin_stdio::artifacts::png::PngSnapshot;

pub fn register() {}

pub fn serialize(from: &DagSnapshot) -> Result<PngSnapshot, store::PackError> {
    let value = serde_json::to_value(from).map_err(|e| store::PackError::Schema(e.to_string()))?;
    serde_json::from_value(value).map_err(|e| store::PackError::Schema(e.to_string()))
}
