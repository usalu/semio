//! 🛍️ `set-active-example` command.

use crate::editor::puzzle3d::config::Puzzle3dRuntime;
use crate::editor::puzzle3d::{default_scene_snapshot, empty_scene_snapshot, nakagin_scene_snapshot, resolve_puzzle3d_attractions, Puzzle3dActionCtx, PUZZLE3D_EXAMPLE_CONCRETE_FOREST, PUZZLE3D_EXAMPLE_NAKAGIN};
use semio_framework_pack_json::Value;

pub fn set_active_example(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
    let next = if example_id.is_empty() {
        Some((empty_scene_snapshot(), String::new()))
    } else if example_id == PUZZLE3D_EXAMPLE_CONCRETE_FOREST || example_id == "concrete" {
        Some((default_scene_snapshot(), PUZZLE3D_EXAMPLE_CONCRETE_FOREST.to_string()))
    } else if example_id == PUZZLE3D_EXAMPLE_NAKAGIN || example_id == "nakagin" {
        Some((nakagin_scene_snapshot(), PUZZLE3D_EXAMPLE_NAKAGIN.to_string()))
    } else {
        None
    };
    if let Some((scene_snapshot, canonical_id)) = next {
        ctx.scene.scene_snapshot = scene_snapshot;
        ctx.scene.runtime = Puzzle3dRuntime { active_example_id: canonical_id, ..Puzzle3dRuntime::default() };
    }
    resolve_puzzle3d_attractions(&mut ctx.scene.scene_snapshot);
}
