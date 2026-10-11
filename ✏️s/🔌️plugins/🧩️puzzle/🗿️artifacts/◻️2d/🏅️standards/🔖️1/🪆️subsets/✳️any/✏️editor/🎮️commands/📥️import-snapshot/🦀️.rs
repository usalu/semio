//! 📥 Loads an imported snapshot JSON file as the whole document — the artifact's load path (`Effect::LoadDocument`, outside
//! history), never a mutation and never a difference of the imported board against the current one. The host's chunked inbound lane
//! (`Effect::RequestFileOpen` → one `importSnapshot {payload, name, chunk, chunkCount}` per
//! `IMPORT_CHUNK_BYTES` page) is reassembled by the framework before this action runs
//! (`semio_framework::kernel::ImportStaging`, admitted in the SDK's `dispatch_action`), so the action reads
//! one whole `payload`, which may contain picked JSON text or an explicit inline snapshot object.

use crate::editor::puzzle2d::{puzzle2d_load_document_effect, Puzzle2dActionCtx, PUZZLE2D_BOARD_SNAPSHOT_SCHEMA};
use crate::retained_command::PUZZLE_IMPORT_TOTAL_BYTES;
use semio_framework::kernel::{UiDirtyScope, IMPORT_ARGUMENT_PAYLOAD};
use semio_framework_pack_json::Value;

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
        Some(text) => semio_framework_pack_json::from_json_str::<Value>(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| Puzzle2dImportFault::Payload)?,
        None => args.get(IMPORT_ARGUMENT_PAYLOAD).cloned().filter(|payload| payload.as_object().is_some()).ok_or(Puzzle2dImportFault::Payload)?,
    };
    let is_snapshot = value.get("schema").and_then(Value::as_str) == Some(PUZZLE2D_BOARD_SNAPSHOT_SCHEMA) || value.get("nodes").is_some_and(|nodes| nodes.as_array().is_some());
    if !is_snapshot {
        return Err(Puzzle2dImportFault::Payload);
    }
    Ok(value)
}

/// 📥 Loads the imported snapshot as the whole document through the load path; every refusal — a file too large, not a
/// puzzle 2d snapshot, or one the typed model refuses — publishes a named, localized notice and changes nothing.
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
        if !object.contains_key("schema") {
            object.insert("schema", Value::String(PUZZLE2D_BOARD_SNAPSHOT_SCHEMA.into()));
        }
        if !object.contains_key("edges") {
            object.insert("edges", Value::Array(Vec::new()));
        }
    }
    let Ok(document) = <crate::Puzzle2dSnapshot as semio_framework_value::FromValue>::from_value(semio_framework_pack_json::to_dsl_value(&snapshot)) else {
        ctx.notice(|labels| labels.import_invalid.as_str());
        *ctx.ui_scope = UiDirtyScope::None;
        return;
    };
    ctx.effects.push(puzzle2d_load_document_effect(&document));
}
//#endregion 🔖️Command
