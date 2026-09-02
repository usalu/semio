//! 🛍️ `set-active-example` command.

use crate::editor::puzzle5d::capsule_dream_example_document;
use crate::editor::puzzle5d::concrete_forest_example_document;
use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::empty_document;
use crate::editor::puzzle5d::nakagin_example_document;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
use crate::editor::puzzle5d::PUZZLE5D_EXAMPLE_CAPSULE_DREAM;
use crate::editor::puzzle5d::PUZZLE5D_EXAMPLE_CONCRETE_FOREST;
use crate::editor::puzzle5d::PUZZLE5D_EXAMPLE_NAKAGIN;
use serde_json::Value;

/// 📚️ Loads one of the two shipped examples (or the empty document), resetting the runtime with it.
pub fn set_active_example(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
    let next = if example_id.is_empty() {
        Some(empty_document())
    } else if example_id == PUZZLE5D_EXAMPLE_CONCRETE_FOREST || example_id == "concrete" {
        Some(concrete_forest_example_document())
    } else if example_id == PUZZLE5D_EXAMPLE_NAKAGIN || example_id == "nakagin" {
        Some(nakagin_example_document())
    } else if example_id == PUZZLE5D_EXAMPLE_CAPSULE_DREAM || example_id == "capsule-dream" || example_id == "capsule" {
        Some(capsule_dream_example_document())
    } else {
        None
    };
    if let Some(document) = next {
        ctx.scene.document = document;
        ctx.scene.runtime = Puzzle5dRuntime::default();
    }
}
