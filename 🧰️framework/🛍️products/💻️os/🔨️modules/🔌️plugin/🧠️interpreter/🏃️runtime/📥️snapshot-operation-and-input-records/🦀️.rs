//! 📥️ Snapshot operation and input records: the snapshot operation, the poll, job, restore, codec and snapshot inputs, the invocation, the engine identity and the owned runtime record. See [`super`].

use super::*;

//#region 🧠️OwnedRuntime
/// 🪶️ The six owned `codec` SQLite snapshot exports a retained snapshot call drives on one instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OwnedSnapshotOperation {
    Export,
    Import,
    Retirement,
    Close,
    TakeFile,
    TakePayload,
}

impl OwnedSnapshotOperation {
    pub(super) fn operation(self) -> OwnedOperation {
        match self {
            Self::Export => OwnedOperation::SqliteExport,
            Self::Import => OwnedOperation::SqliteImport,
            Self::Retirement => OwnedOperation::SqliteRetirement,
            Self::Close => OwnedOperation::SqliteClose,
            Self::TakeFile => OwnedOperation::SqliteTakeFile,
            Self::TakePayload => OwnedOperation::SqliteTakePayload,
        }
    }

    pub(super) fn from_operation(operation: OwnedOperation) -> Option<Self> {
        match operation {
            OwnedOperation::SqliteExport => Some(Self::Export),
            OwnedOperation::SqliteImport => Some(Self::Import),
            OwnedOperation::SqliteRetirement => Some(Self::Retirement),
            OwnedOperation::SqliteClose => Some(Self::Close),
            OwnedOperation::SqliteTakeFile => Some(Self::TakeFile),
            OwnedOperation::SqliteTakePayload => Some(Self::TakePayload),
            _ => None,
        }
    }
}

/// 🔀️ `serde`-only, not `ToValue`: `events`/`command_page`/`budget` embed `semio_framework::kernel::
/// {Event, CommandPageCursor, FixedCommandPage, Budget}`, none of which have `ToValue`/`FromValue`
/// yet (same orphan-rule/`rename_all_fields`-unsupported blocker as `owned_abi::PollInput`'s own
/// hand-written bridge in `🔌️plugin/🦀️.rs` — see that impl's docstring). Converting this
/// needs the `🎠️kernel` module's own conversion pass first.
#[derive(serde::Serialize)]
pub(super) struct OwnedPollInput<'a> {
    pub(super) events: &'a [&'a Event],
    pub(super) command_page: Option<(&'a semio_framework::kernel::CommandPageCursor, &'a semio_framework::kernel::FixedCommandPage)>,
    pub(super) cold_pair_page: Option<&'a semio_framework::kernel::ColdDocumentPairPage>,
    pub(super) budget: Budget,
}

/// 📥️ The mirror of the guest's `owned_abi::StartJobInput`: the prepared source copy and the original job budget.
#[derive(serde::Serialize)]
pub(super) struct OwnedStartJobInput<'a> {
    pub(super) source: &'a Option<semio_framework_actor::OriginalJobSource>,
    pub(super) budget: semio_framework_actor::OriginalJobBudget,
}

#[derive(serde::Serialize, ToValue)]
pub(super) struct OwnedRestoreInput {
    pub(super) state: Vec<u8>,
}

/// 🧬️ OS-HUB-COLLABORATION-AI-END-TO-END (TC3b): the input envelope of the four owned `codec`
/// exports — the mirror of the guest's `owned_abi::CodecInput`.
#[derive(serde::Serialize)]
pub(super) struct OwnedCodecInput<'a> {
    pub(super) artifact_schema: &'a str,
    pub(super) document_id: &'a str,
    pub(super) pack: &'a [u8],
    pub(super) spr: &'a [u8],
    pub(super) ops: &'a [u8],
}

/// 🪶️ The input envelope of the owned `codec.sqlite-export`/`sqlite-import` exports — the mirror of the guest's own snapshot
/// input.
#[derive(serde::Serialize)]
pub struct OwnedSnapshotInput<'a> {
    pub dialect: &'a str,
    pub encoding: &'a str,
    pub payload: &'a [u8],
    pub limits: sqlite_wire::SnapshotLimits,
    pub native: sqlite_wire::SnapshotGrant,
}

/// 🧾️ One completed owned export: the input bytes the call retained for its caller, the result bytes and the fuel it ran.
pub struct OwnedInvocation {
    pub retained_input: Option<Vec<u8>>,
    pub output: Vec<u8>,
    pub fuel_used: u64,
}

/// 🪪️ The owned engine's semantic identity: a digest of the interpreter's source and of this owned
/// host adapter's source — every instruction a guest executes and every host import answer it can
/// observe. A fact one build of the engine derived from a component (a verified pack-schema hash)
/// holds for every other build of the same engine — a copied, re-signed or cross-compiled binary —
/// and a changed engine never reuses one.
pub fn owned_engine_identity() -> &'static str {
    static IDENTITY: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    IDENTITY.get_or_init(|| {
        let interpreter = semio_framework_plugin_interpreter::INTERPRETER_SOURCE.as_bytes();
        let host = OWNED_RUNTIME_SOURCE.as_bytes();
        let mut engine = Vec::with_capacity(64 + interpreter.len() + host.len());
        engine.extend_from_slice(b"semio.owned-engine/v1\n");
        engine.extend_from_slice(&(interpreter.len() as u64).to_le_bytes());
        engine.extend_from_slice(interpreter);
        engine.extend_from_slice(&(host.len() as u64).to_le_bytes());
        engine.extend_from_slice(host);
        format!("owned-engine-v1:{}", semio_framework_hash::hex_lower(semio_framework_hash::hash(&engine).as_bytes()))
    })
}

/// 🏃️ The owned interpreter as a guest runtime: compiles components, instantiates actors and runs their turns, jobs,
/// checkpoints and pure `codec` exports under instruction fuel. Holds no engine; instance ids are its only state.
pub struct OwnedRuntime {
    pub(super) next_instance_id: std::sync::atomic::AtomicU32,
    pub(super) original_clock:fn()->Option<u64>,
}
//#endregion 🧠️OwnedRuntime
