//! 📥 Loads an imported puzzle 5d JSON file as the whole document (`Effect::LoadDocument`, outside history). The host's chunked inbound lane
//! (`Effect::RequestFileOpen` → one `importSnapshot {payload, name, chunk, chunkCount}` per `IMPORT_CHUNK_BYTES` page)
//! is reassembled by the framework before this action runs (`semio_framework::kernel::ImportStaging`, admitted in the
//! SDK's `dispatch_action`), so the action reads ONE whole `payload`; an agent may hand the document inline as a
//! inline `payload` object instead.

use crate::editor::puzzle5d::{Puzzle5dActionCtx, Puzzle5dDocument, PUZZLE5D_SCHEMA};
use crate::retained_command::PUZZLE_IMPORT_TOTAL_BYTES;
use semio_framework_pack_json::Value;
use semio_framework::kernel::{Effect, IMPORT_ARGUMENT_PAYLOAD};
use semio_framework_value::FromValue;

//#region 🔖️Vocabulary
/// 🚫️ Why one import was refused. Every arm becomes a localized shell notice — never a silent no-op.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle5dImportFault {
    /// 📦️ The file is larger than [`PUZZLE_IMPORT_TOTAL_BYTES`], the budget one export may stream — so a file this
    /// app wrote is always a file this app can read back. Capsule Dream is above it and is refused with a notice, in
    /// both directions.
    Capacity,
    /// 🔤️ The file is not one JSON object, or not a puzzle 5d document.
    Payload,
}

impl Puzzle5dImportFault {
    /// 🏷️ Stable code, mirrored by the unit laws.
    pub fn code(self) -> &'static str {
        match self {
            Self::Capacity => "puzzle5d-import-capacity",
            Self::Payload => "puzzle5d-import-payload",
        }
    }
}
//#endregion 🔖️Vocabulary

//#region 🔖️Command
/// 🔎️ A puzzle 5d document: one JSON object carrying its `schema`, or a `parts` array.
fn is_document(value: &Value) -> bool {
    value.is_object() && (value.get("schema").and_then(Value::as_str) == Some(PUZZLE5D_SCHEMA) || value.get("parts").is_some_and(Value::is_array))
}

/// 📥 Decodes one whole file into a puzzle 5d document value — the decode every import runs, whatever its size.
pub fn puzzle5d_decode_document(text: &str) -> Result<Value, Puzzle5dImportFault> {
    semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().filter(is_document).ok_or(Puzzle5dImportFault::Payload)
}

/// 📥 The document an import carries: the whole picked file as `payload` (within [`PUZZLE_IMPORT_TOTAL_BYTES`]), or
/// an inline `payload` object — admitted only when it really is a puzzle 5d document.
pub fn puzzle5d_import_value(args: &Value) -> Result<Value, Puzzle5dImportFault> {
    match args.get(IMPORT_ARGUMENT_PAYLOAD).and_then(Value::as_str) {
        Some(text) if text.len() > PUZZLE_IMPORT_TOTAL_BYTES => Err(Puzzle5dImportFault::Capacity),
        Some(text) => puzzle5d_decode_document(text),
        None => {
            let inline = args.get(IMPORT_ARGUMENT_PAYLOAD).ok_or(Puzzle5dImportFault::Payload)?;
            Some(inline.clone()).filter(is_document).ok_or(Puzzle5dImportFault::Payload)
        }
    }
}

/// 🌱️ The sanctioned whole-document replacement: the imported `document` as a pack plus a fresh, edit-free op log. A
/// natural-file import is the load path, not a mutation — it carries no inverse and journals no history row.
pub fn puzzle5d_load_document_effect(document: &crate::Puzzle5dSnapshot) -> Effect {
    let pack = <crate::Puzzle5dSnapshot as store::ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::empty_document_spr(crate::editor::puzzle5d::PUZZLE5D_PLAY_APP_ID, crate::PUZZLE_5D_SCHEMA));
    Effect::LoadDocument { pack, spr }
}

/// 📥 Loads the imported document as the whole live document; every refusal publishes a named, localized notice and
/// leaves the document and the history untouched.
pub fn import_snapshot(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let outcome = args.ok_or(Puzzle5dImportFault::Payload).and_then(puzzle5d_import_value);
    let document: Puzzle5dDocument = match outcome.and_then(|value| Puzzle5dDocument::from_value(semio_framework_pack_json::to_dsl_value(&value)).map_err(|_| Puzzle5dImportFault::Payload)) {
        Ok(document) => document,
        Err(fault) => return refuse(ctx, fault),
    };
    let Ok(typed) = crate::editor::puzzle5d::puzzle5d_snapshot_from_document(&document) else {
        return refuse(ctx, Puzzle5dImportFault::Payload);
    };
    ctx.effects.push(puzzle5d_load_document_effect(&typed));
}

/// 📥️ The one localized notice a refused import publishes, then the abort that keeps the document and the history
/// untouched.
fn refuse(ctx: &mut Puzzle5dActionCtx<'_>, fault: Puzzle5dImportFault) {
    match fault {
        Puzzle5dImportFault::Capacity => ctx.notice(|labels| labels.import_too_large.as_str()),
        Puzzle5dImportFault::Payload => ctx.notice(|labels| labels.import_invalid.as_str()),
    }
    ctx.abort = true;
}
//#endregion 🔖️Command
