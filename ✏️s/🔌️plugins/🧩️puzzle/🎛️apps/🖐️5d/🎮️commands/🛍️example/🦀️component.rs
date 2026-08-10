//! 🛍️ Puzzle 5d play app commands — whole-document loads: the raw fixture-json setter, the example
//! picker (concrete-forest / nakagin / empty).

use crate::apps::puzzle5d::config::{Puzzle5dRuntime, Puzzle5dSelection};
use crate::apps::puzzle5d::{
    default_document, document_from_json, empty_document, CAPSULE_DREAM_EXAMPLE_JSON, CONCRETE_FOREST_EXAMPLE_JSON, NAKAGIN_EXAMPLE_JSON, Puzzle5dActionCtx, Puzzle5dDocument, Puzzle5dFastener, Puzzle5dPart, PUZZLE5D_EXAMPLE_CAPSULE_DREAM, PUZZLE5D_EXAMPLE_CONCRETE_FOREST, PUZZLE5D_EXAMPLE_NAKAGIN,
};
use serde_json::Value;

/// 🧾️ Replaces the whole document from a raw JSON payload; an unparseable payload is a no-op.
pub fn set_fixture_json(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
        if let Ok(document) = serde_json::from_str::<Puzzle5dDocument>(json_text) {
            ctx.scene.document = document;
        }
    }
}

/// 📚️ Loads one of the two shipped examples (or the empty document), resetting the runtime with it.
pub fn set_active_example(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
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

