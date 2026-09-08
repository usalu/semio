//! 📄️ 📄️ Sourcing curation app commands command — `set-artifact-json`.

use crate::op::SourcingMutation;
use crate::schema::snapshot::decode_curation_snapshot_json;
use crate::schema::sourcing_json_envelope_is_bounded;
use crate::CurationSnapshot;
use crate::editor::sourcing::config::{SourcingCurationConfig, SourcingCurationConfigMutation};
use crate::editor::sourcing::reset_document_effect;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "document-json")]
pub struct SetArtifactJson {
    pub json: String,
}

/// 🛠️ Dev-only whole-document import — kept out of the command palette.
pub fn handle(payload: &SetArtifactJson, _doc: &ArtifactView<'_, CurationSnapshot>, _cfg: &ConfigView<'_, SourcingCurationConfig>) -> Result<Emit<SourcingMutation, SourcingCurationConfigMutation>, Fault> {
    if !sourcing_json_envelope_is_bounded(&payload.json) {
        return Err(Fault::from("sourcing.invalid-payload: document JSON exceeds byte, depth, string, or cardinality limit"));
    }
    match decode_curation_snapshot_json(&payload.json) {
        Ok(document) => Ok(Emit { effects: vec![reset_document_effect(&document)], ..Default::default() }),
        Err(_) => Err(Fault::from("sourcing.invalid-payload: document schema mismatch")),
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
