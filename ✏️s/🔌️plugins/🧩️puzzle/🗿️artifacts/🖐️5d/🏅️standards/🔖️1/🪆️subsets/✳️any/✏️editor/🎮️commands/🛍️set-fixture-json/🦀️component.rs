//! 🛍️ `set-fixture-json` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use crate::editor::puzzle5d::Puzzle5dDocument;
use serde_json::Value;

/// 🧾️ Replaces the whole document from a raw JSON payload; an unparseable payload is a no-op.
pub async fn set_fixture_json(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(json_text) = args.and_then(|value| value.get("json")).and_then(|value| value.as_str()) {
        if let Ok(document) = serde_json::from_str::<Puzzle5dDocument>(json_text) {
            ctx.scene.document = document;
        }
    }
}
