//! 📥 Loads an imported scene_snapshot JSON file as the whole document (`Effect::LoadDocument`, outside history). The host's chunked inbound lane
//! (`Effect::RequestFileOpen` → one `importSnapshot {payload, name, chunk, chunkCount}` per `IMPORT_CHUNK_BYTES` page)
//! is reassembled by the framework before this action runs (`semio_framework::kernel::ImportStaging`, admitted in the
//! SDK's `dispatch_action`), so the action reads ONE whole `payload`; an agent may hand the scene_snapshot inline as a
//! inline `payload` object instead.
//!
//! 🧾️ Before ticket 26/09/02 wave B59 a 145 924-byte import produced no edit, no row and no notice (the retained job's
//! wire ladder spent one host step per byte, `PUZZLE_COMMAND_WIRE_SCAN_STRIDE_BYTES`), so every refusal here answers
//! with a named, localized notice.

use crate::editor::puzzle3d::{puzzle3d_snapshot_from_host_snapshot, Puzzle3dActionCtx, Puzzle3dSceneSnapshot, PUZZLE3D_SCENE_SNAPSHOT_SCHEMA};
use crate::retained_command::PUZZLE_IMPORT_TOTAL_BYTES;

use semio_framework_pack_json::{parse, Value};
use semio_framework_plugin::kernel::Effect;
use semio_framework_value::FromValue;
use semio_framework::kernel::IMPORT_ARGUMENT_PAYLOAD;

/// 🚫️ Why one import was refused. Every arm becomes a localized shell notice — never a silent no-op.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Puzzle3dImportFault {
    /// 📦️ The file is larger than [`PUZZLE_IMPORT_TOTAL_BYTES`], the budget one export may stream — so a file this
    /// app wrote is always a file this app can read back.
    Capacity,
    /// 🔤️ The file is not one JSON object, or not a puzzle 3D document.
    Payload,
}

impl Puzzle3dImportFault {
    /// 🏷️ Stable code, mirrored by the unit laws.
    pub fn code(self) -> &'static str {
        match self {
            Self::Capacity => "puzzle3d-import-capacity",
            Self::Payload => "puzzle3d-import-payload",
        }
    }
}

/// 📥️ The JSON object an import carries: the whole picked file as `payload`, or an inline `payload` object.
pub fn puzzle3d_import_value(args: &Value) -> Result<Value, Puzzle3dImportFault> {
    match args.get(IMPORT_ARGUMENT_PAYLOAD).and_then(Value::as_str) {
        Some(text) if text.len() > PUZZLE_IMPORT_TOTAL_BYTES => Err(Puzzle3dImportFault::Capacity),
        Some(text) => parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().filter(|value| value.as_object().is_some()).ok_or(Puzzle3dImportFault::Payload),
        None => args.get(IMPORT_ARGUMENT_PAYLOAD).cloned().filter(|value| value.as_object().is_some()).ok_or(Puzzle3dImportFault::Payload),
    }
}

/// 📥️ The localized notice one refused import publishes, then the abort that keeps the document and the history
/// untouched.
fn refuse(ctx: &mut Puzzle3dActionCtx<'_>, fault: Puzzle3dImportFault) {
    match fault {
        Puzzle3dImportFault::Capacity => ctx.notice(|labels| labels.import_too_large.as_str()),
        Puzzle3dImportFault::Payload => ctx.notice(|labels| labels.import_invalid.as_str()),
    }
    ctx.abort = true;
}

/// 🌱️ The sanctioned whole-document replacement: the imported `document` as a pack plus a fresh, edit-free op log. A
/// natural-file import is the load path, not a mutation — it carries no inverse and journals no history row.
pub fn puzzle3d_load_document_effect(document: &crate::Puzzle3dSnapshot) -> Effect {
    let pack = <crate::Puzzle3dSnapshot as store::ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::empty_document_spr(crate::editor::puzzle3d::PUZZLE3D_PLAY_APP_ID, crate::PUZZLE_3D_SCHEMA));
    Effect::LoadDocument { pack, spr }
}

/// 📥 Loads the imported scene_snapshot as the whole document.
pub fn import_snapshot(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let value = match args.ok_or(Puzzle3dImportFault::Payload).and_then(puzzle3d_import_value) {
        Ok(value) => value,
        Err(fault) => return refuse(ctx, fault),
    };
    let Ok(mut scene_snapshot) = Puzzle3dSceneSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&value)) else {
        return refuse(ctx, Puzzle3dImportFault::Payload);
    };
    if scene_snapshot.schema.is_empty() {
        scene_snapshot.schema = PUZZLE3D_SCENE_SNAPSHOT_SCHEMA.into();
    }
    let Ok(document) = puzzle3d_snapshot_from_host_snapshot(&scene_snapshot) else {
        return refuse(ctx, Puzzle3dImportFault::Payload);
    };
    ctx.effects.push(puzzle3d_load_document_effect(&document));
}
