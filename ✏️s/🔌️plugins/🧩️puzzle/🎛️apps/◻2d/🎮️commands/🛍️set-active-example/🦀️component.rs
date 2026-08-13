//! 🛍️ `set-active-example` command.

use crate::apps::puzzle2d::config::Puzzle2dPlayRuntime;
use crate::apps::puzzle2d::{default_empty_fixture, Puzzle2dActionCtx, concrete_forest_example_json, nakagin_example_json, PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID};
use serde_json::Value;

pub fn set_active_example(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
    ctx.scene.fixture = if example_id.is_empty() {
        default_empty_fixture()
    } else if example_id == PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID || example_id == "concrete" {
        serde_json::from_str(concrete_forest_example_json().as_str()).unwrap_or_else(|_| default_empty_fixture())
    } else if example_id == PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID || example_id == "nakagin" {
        serde_json::from_str(nakagin_example_json().as_str()).unwrap_or_else(|_| default_empty_fixture())
    } else {
        default_empty_fixture()
    };
    ctx.scene.runtime = Puzzle2dPlayRuntime::default();
}
