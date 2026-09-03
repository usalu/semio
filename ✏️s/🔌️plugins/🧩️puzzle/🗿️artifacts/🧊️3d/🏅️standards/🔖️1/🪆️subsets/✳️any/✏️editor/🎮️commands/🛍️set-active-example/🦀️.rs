//! 🛍️ `set-active-example` command.

use crate::editor::puzzle3d::config::Puzzle3dRuntime;
use crate::editor::puzzle3d::{default_fixture, empty_fixture, nakagin_fixture, resolve_puzzle3d_attractions, Puzzle3dActionCtx, PUZZLE3D_EXAMPLE_CONCRETE_FOREST, PUZZLE3D_EXAMPLE_NAKAGIN};
use dsl::os_pack::json::Value;

pub fn set_active_example(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
    let next = if example_id.is_empty() {
        Some(empty_fixture())
    } else if example_id == PUZZLE3D_EXAMPLE_CONCRETE_FOREST || example_id == "concrete" {
        Some(default_fixture())
    } else if example_id == PUZZLE3D_EXAMPLE_NAKAGIN || example_id == "nakagin" {
        Some(nakagin_fixture())
    } else {
        None
    };
    if let Some(fixture) = next {
        ctx.scene.fixture = fixture;
        ctx.scene.runtime = Puzzle3dRuntime::default();
    }
    resolve_puzzle3d_attractions(&mut ctx.scene.fixture);
}
