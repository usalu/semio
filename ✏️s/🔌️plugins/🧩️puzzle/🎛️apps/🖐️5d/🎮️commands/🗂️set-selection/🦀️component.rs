//! 🗂️ `set-selection` command.

use crate::apps::puzzle5d::config::{puzzle5d_clear_non_part_selection, puzzle5d_clear_selection, Puzzle5dSelection};
use crate::apps::puzzle5d::{classify_selection, Puzzle5dActionCtx};
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;

/// 🎯️ `setSelection`/`documentSelect`: a flat `ids` list is classified against the document, otherwise
/// the three typed bags are read directly.
pub fn set_selection(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(ids) = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok()) {
        ctx.scene.runtime.selection = classify_selection(&ctx.scene.document, &ids);
    } else {
        let read = |key: &str| args.and_then(|value| value.get(key)).and_then(|value| serde_json::from_value::<Vec<String>>(value.clone()).ok());
        ctx.scene.runtime.selection = Puzzle5dSelection {
            part_ids: SelectionSet::from_ids(read("partIds").unwrap_or_default()),
            grip_ids: SelectionSet::from_ids(read("gripIds").unwrap_or_default()),
            fastener_ids: SelectionSet::from_ids(read("fastenerIds").unwrap_or_default()),
        };
    }
}
