//! 🗂️ 🗂️ Drawing play app commands command — `patch-layers`.

use crate::artifacts::drawing::op::{drawing_op_for_layer_field, DrawingMutation};
use crate::artifacts::drawing::DrawingSnapshot;
use crate::editor::drawing::commands::canvas_pointer_down::DrawingSession;
use crate::editor::drawing::config::{DrawingConfig, DrawingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use dsl::{FromValue, ToValue};

//#region 🔖️DocumentHelpers
/// 🩹️ Parses a `PatchLayer`/`PatchLayers` wire `value` as JSON text (falling back to a plain JSON
/// string when it isn't valid JSON) so one `String` wire field covers every heterogeneous
/// `drawing_op_for_layer_field` value type (bool/number/string) — mirrors
/// `shooting_protocol::ShootingCommand`'s `PatchShots`/`PatchAssets` shape.
fn patch_value_json(value: &str) -> dsl::DslValue {
    dsl::json::parse(value).map(|parsed| dsl::json::to_dsl_value(&parsed)).unwrap_or_else(|_| dsl::DslValue::String(value.to_string()))
}
//#endregion 🔖️DocumentHelpers

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "patch-layers")]
pub struct PatchLayers {
    pub layer_ids: Vec<String>,
    pub field: String,
    pub value: String,
}

pub fn handle(payload: &PatchLayers, doc: &ArtifactView<'_, DrawingSnapshot>, _cfg: &ConfigView<'_, DrawingConfig>, _session: &mut DrawingSession) -> Result<Emit<DrawingMutation, DrawingConfigMutation>, Fault> {
    let document = doc.snapshot;
    let json_value = patch_value_json(&payload.value);
    let operations: Vec<DrawingMutation> = payload.layer_ids.iter().filter_map(|id| drawing_op_for_layer_field(document, id, &payload.field, &json_value)).collect();
    if operations.is_empty() {
        return Ok(Emit::default());
    }
    Ok(Emit::mutations(operations))
}
