//! 📥 Replaces the document with an imported snapshot JSON file. The host's chunked inbound lane
//! (`Effect::RequestFileOpen` → one `importSnapshot {payload, name, chunk, chunkCount}` per
//! `IMPORT_CHUNK_BYTES` page) is reassembled by the framework before this action runs
//! (`semio_framework::kernel::ImportStaging`, admitted in the SDK's `dispatch_action`), so the action reads
//! one whole `payload`, which may contain picked JSON text or an explicit inline snapshot object.

use crate::editor::puzzle2d::{Puzzle2dActionCtx, PUZZLE2D_BOARD_SNAPSHOT_SCHEMA};
use crate::retained_command::PUZZLE_IMPORT_TOTAL_BYTES;
use semio_framework::kernel::{UiDirtyScope, IMPORT_ARGUMENT_PAYLOAD};
use serde_json::Value;

//#region 🔖️Vocabulary
/// 🚫️ Why one import was refused — every arm becomes a localized shell notice, never a silent no-op.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle2dImportFault {
    /// 📦️ The file is larger than [`PUZZLE_IMPORT_TOTAL_BYTES`], the budget one export may stream.
    Capacity,
    /// 🔤️ The file is not one JSON object, or not a puzzle 2d snapshot.
    Payload,
}

impl Puzzle2dImportFault {
    /// 🏷️ Stable code, mirrored by the unit laws.
    pub fn code(self) -> &'static str {
        match self {
            Self::Capacity => "puzzle2d-import-capacity",
            Self::Payload => "puzzle2d-import-payload",
        }
    }
}
//#endregion 🔖️Vocabulary

//#region 🔖️Command
/// 📥 The snapshot an import carries: the picked JSON text or an inline object in `payload` —
/// accepted only when it is a puzzle 2d snapshot (its `schema` or a `nodes` array).
pub fn puzzle2d_import_value(args: &Value) -> Result<Value, Puzzle2dImportFault> {
    let value = match args.get(IMPORT_ARGUMENT_PAYLOAD).and_then(Value::as_str) {
        Some(text) if text.len() > PUZZLE_IMPORT_TOTAL_BYTES => return Err(Puzzle2dImportFault::Capacity),
        Some(text) => serde_json::from_str::<Value>(text).map_err(|_| Puzzle2dImportFault::Payload)?,
        None => args.get(IMPORT_ARGUMENT_PAYLOAD).cloned().filter(Value::is_object).ok_or(Puzzle2dImportFault::Payload)?,
    };
    let is_snapshot = value.get("schema").and_then(Value::as_str) == Some(PUZZLE2D_BOARD_SNAPSHOT_SCHEMA) || value.get("nodes").is_some_and(Value::is_array);
    if !is_snapshot {
        return Err(Puzzle2dImportFault::Payload);
    }
    Ok(value)
}

/// 📥 Replaces the document with the imported snapshot as one document edit; every refusal publishes a named,
/// localized notice and changes nothing.
pub fn import_snapshot(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let mut snapshot = match args.ok_or(Puzzle2dImportFault::Payload).and_then(puzzle2d_import_value) {
        Ok(snapshot) => snapshot,
        Err(fault) => {
            match fault {
                Puzzle2dImportFault::Capacity => ctx.notice(|labels| labels.import_too_large.as_str()),
                Puzzle2dImportFault::Payload => ctx.notice(|labels| labels.import_invalid.as_str()),
            }
            *ctx.ui_scope = UiDirtyScope::None;
            return;
        }
    };
    if let Some(object) = snapshot.as_object_mut() {
        object.entry("schema").or_insert_with(|| Value::String(PUZZLE2D_BOARD_SNAPSHOT_SCHEMA.into()));
        object.entry("edges").or_insert_with(|| Value::Array(Vec::new()));
    }
    ctx.scene.board_snapshot = snapshot;
}
//#endregion 🔖️Command
