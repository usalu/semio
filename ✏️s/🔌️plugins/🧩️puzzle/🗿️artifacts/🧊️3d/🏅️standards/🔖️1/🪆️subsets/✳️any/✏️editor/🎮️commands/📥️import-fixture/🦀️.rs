//! 📥 Replaces the document with an imported fixture JSON file. The host's chunked inbound lane
//! (`Effect::RequestFileOpen` → one `importFixture {payload, name, chunk, chunkCount}` per `IMPORT_CHUNK_BYTES` page)
//! is reassembled by the framework before this action runs (`semio_framework::kernel::ImportStaging`, admitted in the
//! SDK's `dispatch_action`), so the action reads ONE whole `payload`; an agent may hand the fixture inline as a
//! `json`/`fixture` object instead.
//!
//! 🧾️ Before ticket 26/09/02 wave B59 a 145 924-byte import produced no edit, no row and no notice (the retained job's
//! wire ladder spent one host step per byte, `PUZZLE_COMMAND_WIRE_SCAN_STRIDE_BYTES`), so every refusal here answers
//! with a named, localized notice.

use crate::editor::puzzle3d::{Puzzle3dActionCtx, Puzzle3dFixture, PUZZLE3D_FIXTURE_SCHEMA};
use crate::retained_command::PUZZLE_IMPORT_TOTAL_BYTES;

use semio_framework_pack_json::{parse, Value};
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

/// 📥️ The JSON object an import carries: the whole picked file as `payload`, or an inline `json`/`fixture` object.
pub fn puzzle3d_import_value(args: &Value) -> Result<Value, Puzzle3dImportFault> {
    match args.get(IMPORT_ARGUMENT_PAYLOAD).and_then(Value::as_str) {
        Some(text) if text.len() > PUZZLE_IMPORT_TOTAL_BYTES => Err(Puzzle3dImportFault::Capacity),
        Some(text) => parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok().filter(|value| value.as_object().is_some()).ok_or(Puzzle3dImportFault::Payload),
        None => args
            .get("json")
            .cloned()
            .filter(|value| value.as_object().is_some())
            .or_else(|| args.get("fixture").cloned().filter(|value| value.as_object().is_some()))
            .or_else(|| args.get(IMPORT_ARGUMENT_PAYLOAD).cloned().filter(|value| value.as_object().is_some()))
            .ok_or(Puzzle3dImportFault::Payload),
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

/// 📥 Replaces the live fixture with the imported document as one document edit.
pub fn import_fixture(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let value = match args.ok_or(Puzzle3dImportFault::Payload).and_then(puzzle3d_import_value) {
        Ok(value) => value,
        Err(fault) => return refuse(ctx, fault),
    };
    let Ok(mut fixture) = Puzzle3dFixture::from_value(semio_framework_pack_json::to_dsl_value(&value)) else {
        return refuse(ctx, Puzzle3dImportFault::Payload);
    };
    if fixture.schema.is_empty() {
        fixture.schema = PUZZLE3D_FIXTURE_SCHEMA.into();
    }
    ctx.scene.fixture = fixture;
}
