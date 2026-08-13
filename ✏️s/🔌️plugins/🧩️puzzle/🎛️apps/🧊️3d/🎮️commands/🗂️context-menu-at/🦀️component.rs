//! 🗂️ `context-menu-at` command.

use crate::apps::puzzle3d::config::Puzzle3dSelection;
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;
use crate::apps::puzzle3d::Puzzle3dActionCtx;

/// 🖱️ Right-click on an unselected entity selects it and opens its menu in one round trip, instead of
/// requiring a separate pick action before the menu items become available.
pub fn context_menu_at(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("");
    let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).unwrap_or("");
    ctx.scene.runtime.selection = Puzzle3dSelection::default();
    match kind {
        "object" => ctx.scene.runtime.selection.object_ids = SelectionSet::from(vec![id.to_string()]),
        "vortex" => ctx.scene.runtime.selection.vortex_ids = SelectionSet::from(vec![id.to_string()]),
        "attraction" => ctx.scene.runtime.selection.attraction_ids = SelectionSet::from(vec![id.to_string()]),
        "targetVolume" => ctx.scene.runtime.selection.target_volume_ids = SelectionSet::from(vec![id.to_string()]),
        "reference" => ctx.scene.runtime.selection.reference_ids = SelectionSet::from(vec![id.to_string()]),
        _ => {}
    }
}
