//! Deserialize dag via stdio.png.
use crate::artifacts::dag::io::{dag_from_wire, pack_err_as_text};
use crate::artifacts::dag::DagSnapshot;
use semio_s_plugin_stdio::artifacts::png::{PngSnapshot, STDIO_PNG_DOCUMENT_SCHEMA};

pub fn register() {}

pub fn deserialize(from: &PngSnapshot) -> Result<DagSnapshot, store::TextError> {
    let _ = STDIO_PNG_DOCUMENT_SCHEMA;
    let value = serde_json::to_value(from).map_err(|e| store::TextError::new(e.to_string(), dsl::TextSpan::at(1, 1)))?;
    serde_json::from_value(value).map_err(|e| store::TextError::new(format!("dag<-png: {e}"), dsl::TextSpan::at(1, 1)))
}
