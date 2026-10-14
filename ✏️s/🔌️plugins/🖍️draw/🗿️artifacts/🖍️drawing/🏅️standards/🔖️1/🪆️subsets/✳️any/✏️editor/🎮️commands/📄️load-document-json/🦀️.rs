//! 📄️ 📄️ Drawing play app commands command — `load-document-json`.

use semio_framework_plugin::{NoConfig, NoConfigMutation};
use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::DRAWING_DOCUMENT_SCHEMA;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
#[dsl(keyword = "document-json")]
pub struct LoadDocumentJson {
    pub json: String,
}

/// 🌡 Parsed as JSON (falling back to a no-op when it isn't valid or doesn't carry the drawing schema)
/// — mirrors every other plugin's fixture-injection command.
pub fn handle(
    payload: &LoadDocumentJson,
    _doc: &ArtifactView<'_, DrawingSnapshot>,
    _cfg: &ConfigView<'_, NoConfig>,
    _session: &mut semio_s_artifact_draw_drawing_editor_tool::DrawingSession,
) -> Result<Emit<DrawingMutation, NoConfigMutation>, Fault> {
    if payload.json.contains(DRAWING_DOCUMENT_SCHEMA) {
        if let Ok(snapshot) = semio_framework_pack_json::from_json_str::<DrawingSnapshot>(&payload.json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
            return Ok(Emit { effects: vec![semio_s_artifact_draw_drawing_editor_interaction::drawing_reset_document_effect(&snapshot)], ..Default::default() });
        }
    }
    Ok(Emit::default())
}
