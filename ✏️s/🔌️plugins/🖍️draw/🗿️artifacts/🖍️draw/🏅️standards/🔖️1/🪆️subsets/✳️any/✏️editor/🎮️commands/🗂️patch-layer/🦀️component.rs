//! 🗂️ 🗂️ Draw play app commands command — `patch-layer`.

use crate::artifacts::draw::op::{draw_op_for_layer_field, DrawMutation};
use crate::artifacts::draw::DrawSnapshot;
use crate::editor::draw::commands::canvas_pointer_down::DrawSession;
use crate::editor::draw::config::{DrawConfig, DrawConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use serde::{Deserialize, Serialize};
use serde_json::Value;

//#region 🔖️DocumentHelpers
/// 🩹️ Parses a `PatchLayer`/`PatchLayers` wire `value` as JSON text (falling back to a plain JSON
/// string when it isn't valid JSON) so one `String` wire field covers every heterogeneous
/// `draw_op_for_layer_field` value type (bool/number/string) — mirrors
/// `shooting_protocol::ShootingCommand`'s `PatchShots`/`PatchAssets` shape.
async fn patch_value_json(value: &str) -> Value {
    serde_json::from_str(value).unwrap_or_else(|_| Value::String(value.to_string()))
}
//#endregion 🔖️DocumentHelpers

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[dsl(keyword = "patch-layer")]
pub struct PatchLayer {
    pub layer_id: String,
    pub field: String,
    pub value: String,
}

pub async fn handle(payload: &PatchLayer, doc: &ArtifactView<'_, DrawSnapshot>, _cfg: &ConfigView<'_, DrawConfig>, _session: &mut DrawSession) -> Result<Emit<DrawMutation, DrawConfigMutation>, Fault> {
    let document = doc.snapshot;
    let json_value = patch_value_json(&payload.value);
    match draw_op_for_layer_field(document, &payload.layer_id, &payload.field, &json_value) {
        Some(operation) => Ok(Emit::mutations(vec![operation])),
        None => Ok(Emit::default()),
    }
}
