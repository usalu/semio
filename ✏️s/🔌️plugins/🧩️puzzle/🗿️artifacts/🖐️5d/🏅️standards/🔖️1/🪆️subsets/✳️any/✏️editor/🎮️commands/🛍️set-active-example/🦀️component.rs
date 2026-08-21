//! 🛍️ `set-active-example` command.

use crate::editor::puzzle5d::config::Puzzle5dRuntime;
use crate::editor::puzzle5d::document_from_json;
use crate::editor::puzzle5d::empty_document;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
use crate::editor::puzzle5d::CAPSULE_DREAM_EXAMPLE_JSON;
use crate::editor::puzzle5d::CONCRETE_FOREST_EXAMPLE_JSON;
use crate::editor::puzzle5d::NAKAGIN_EXAMPLE_JSON;
use crate::editor::puzzle5d::PUZZLE5D_EXAMPLE_CAPSULE_DREAM;
use crate::editor::puzzle5d::PUZZLE5D_EXAMPLE_CONCRETE_FOREST;
use crate::editor::puzzle5d::PUZZLE5D_EXAMPLE_NAKAGIN;
use serde_json::Value;

/// 📚️ Loads one of the two shipped examples (or the empty document), resetting the runtime with it.
pub async fn set_active_example(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let example_id = args.and_then(|value| value.get("exampleId")).and_then(|value| value.as_str()).unwrap_or("");
    let next = if example_id.is_empty() {
        Some(empty_document())
    } else if example_id == PUZZLE5D_EXAMPLE_CONCRETE_FOREST || example_id == "concrete" {
        Some(document_from_json(&CONCRETE_FOREST_EXAMPLE_JSON))
    } else if example_id == PUZZLE5D_EXAMPLE_NAKAGIN || example_id == "nakagin" {
        Some(document_from_json(&NAKAGIN_EXAMPLE_JSON))
    } else if example_id == PUZZLE5D_EXAMPLE_CAPSULE_DREAM || example_id == "capsule-dream" || example_id == "capsule" {
        Some(document_from_json(&CAPSULE_DREAM_EXAMPLE_JSON))
    } else {
        None
    };
    if let Some(document) = next {
        ctx.scene.document = document;
        ctx.scene.runtime = Puzzle5dRuntime::default();
    }
    ctx.app.drive_precompute(ctx.scene);
}
