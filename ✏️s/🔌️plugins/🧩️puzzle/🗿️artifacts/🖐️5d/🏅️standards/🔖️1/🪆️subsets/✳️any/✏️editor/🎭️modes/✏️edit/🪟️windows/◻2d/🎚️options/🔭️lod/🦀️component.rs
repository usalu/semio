//! 🔭️ 2D-window option — the level-of-detail select: "Automatic" plus every board scale tier
//! (minimap…micro), persisted through `setLodMode`. Genuinely window-specific: only the Board2d
//! surface has LOD tiers, so this stays under the 2D window rather than at mode level.

use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::{puzzle5d_action, PUZZLE5D_LOD_MODE_AUTOMATIC, PUZZLE5D_PLAY_CONTROLLER_ID};
use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use semio_framework_plugin::{MeasureSelectItem, WindowMeasure};
use serde_json::Value;

fn puzzle5d_lod_tier_ids() -> Vec<String> {
    serde_json::from_str::<Vec<Value>>(&infinite_board_port_directed_normal::puzzle_2d_lod_scale_json()).unwrap_or_default().into_iter().filter_map(|row| row.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect()
}

/// 📶️ The LOD select measure, persisted via `setLodMode`.
pub fn measure(runtime: &Puzzle5dRuntime, labels: &Puzzle5dLabels) -> WindowMeasure {
    let mut items = vec![MeasureSelectItem { id: PUZZLE5D_LOD_MODE_AUTOMATIC.into(), value: PUZZLE5D_LOD_MODE_AUTOMATIC.into(), label: labels.automatic.into() }];
    items.extend(puzzle5d_lod_tier_ids().into_iter().map(|tier| MeasureSelectItem { id: tier.clone(), value: tier.clone(), label: tier }));
    WindowMeasure::Select { id: format!("{PUZZLE5D_PLAY_CONTROLLER_ID}-lod"), label: Some(labels.lod.into()), value: runtime.lod_mode.clone(), items, on_change: puzzle5d_action("setLodMode", None) }
}
