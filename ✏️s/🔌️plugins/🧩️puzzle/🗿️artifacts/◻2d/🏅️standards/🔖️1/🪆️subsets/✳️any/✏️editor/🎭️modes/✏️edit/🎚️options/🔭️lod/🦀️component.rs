//! 🔭️ Edit-mode window option — the per-pane level-of-detail select: "Automatic" plus every board
//! scale tier (minimap…micro), persisted through `setLodModeForPane`. Shared by all three canvas
//! windows, which only differ in which pane id they bind it to.

use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{puzzle2d_action, PUZZLE2D_LOD_MODE_AUTOMATIC};
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};
use serde_json::{json, Value};

async fn puzzle2d_lod_tier_ids() -> Vec<String> {
    serde_json::from_str::<Vec<Value>>(&crate::editor::puzzle2d::engine::puzzle_2d_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|row| row.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
}

/// 📶️ Per-pane LOD select measure, persisted via `setLodModeForPane`.
pub async fn measure(pane: &str, current_mode: &str, labels: &Puzzle2dLabels) -> WindowMeasure {
    let mut items = vec![MeasureSelectItem { id: PUZZLE2D_LOD_MODE_AUTOMATIC.into(), value: PUZZLE2D_LOD_MODE_AUTOMATIC.into(), label: labels.automatic.into() }];
    items.extend(puzzle2d_lod_tier_ids().into_iter().map(|tier| MeasureSelectItem { id: tier.clone(), value: tier.clone(), label: tier }));
    WindowMeasure::Select { id: format!("{pane}-lod"), label: Some(labels.lod.into()), value: current_mode.into(), items, on_change: puzzle2d_action("setLodModeForPane", Some(json!({ "pane": pane }))) }
}
