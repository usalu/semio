//! 📤 Downloads the live fixture as round-trippable JSON.

use crate::editor::puzzle3d::{puzzle3d_projection_value, Puzzle3dActionCtx};
use dsl::os_pack::json::to_json_string;
use semio_framework_plugin::kernel::Effect;

/// 📤 Emits a host download of the current fixture JSON.
pub fn export_fixture(ctx: &mut Puzzle3dActionCtx<'_>) {
    let data = to_json_string(&puzzle3d_projection_value(dsl::ToValue::to_value(&ctx.scene.fixture)));
    ctx.effects.push(Effect::DownloadMediaExport {
        filename: "puzzle-3d.json".into(),
        mime_type: "application/json".into(),
        data,
        encoding: Some("utf-8".into()),
    });
}
