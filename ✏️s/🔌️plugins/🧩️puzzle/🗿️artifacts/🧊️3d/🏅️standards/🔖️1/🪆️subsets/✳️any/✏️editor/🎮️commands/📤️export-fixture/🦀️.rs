//! 📤 Downloads the live fixture as round-trippable JSON.

use crate::editor::puzzle3d::{puzzle3d_projection_value, Puzzle3dActionCtx};
use dsl::os_pack::json::to_json_string;
use semio_framework_plugin::kernel::Effect;

/// 📤 The download filename for one export: the active example's own id — `concrete-forest.json`,
/// `nakagin-capsule-tower.json` — so exporting two different examples never lands as two files with
/// one name. A document that came from no example (blank, or already imported from elsewhere) keeps
/// the app-generic `puzzle-3d.json`.
pub fn puzzle3d_export_filename(active_example_id: &str) -> String {
    if active_example_id.is_empty() {
        return "puzzle-3d.json".into();
    }
    format!("{active_example_id}.json")
}

/// 📤 Emits a host download of the current fixture JSON.
pub fn export_fixture(ctx: &mut Puzzle3dActionCtx<'_>) {
    let data = to_json_string(&puzzle3d_projection_value(dsl::ToValue::to_value(&ctx.scene.fixture)));
    ctx.effects.push(Effect::DownloadMediaExport {
        filename: puzzle3d_export_filename(&ctx.scene.runtime.active_example_id),
        mime_type: "application/json".into(),
        data,
        encoding: Some("utf-8".into()),
    });
}
