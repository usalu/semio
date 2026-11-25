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

/// 🔀️ Serializes borrowed input in the tagged owned Core Poll envelope.
#[derive(serde::Serialize)]
pub(super) enum OwnedPollInput<'a> {
    Poll {
        events: &'a [&'a Event],
        command_page: Option<(&'a semio_framework::kernel::CommandPageCursor, &'a semio_framework::kernel::FixedCommandPage)>,
        cold_pair_page: Option<&'a semio_framework::kernel::ColdDocumentPairPage>,
        limits: semio_framework::kernel::OwnedCorePollLimits,
    },
}

impl<'a> OwnedPollInput<'a> {
    pub(super) fn with_budget(
        events: &'a [&'a Event],
        command_page: Option<(&'a semio_framework::kernel::CommandPageCursor, &'a semio_framework::kernel::FixedCommandPage)>,
        cold_pair_page: Option<&'a semio_framework::kernel::ColdDocumentPairPage>,
        budget: Budget,
    ) -> Result<Self, semio_framework_value::ValueError> {
        let limits = semio_framework::kernel::OwnedCorePollLimits::try_from(semio_framework::kernel::PollRequestLimits {
            retained_policy: budget.retained_policy,
            work_fuel: budget.work_fuel,
            fuel: budget.fuel,
            deadline_us: budget.deadline_us,
            max_effects: budget.max_effects,
            max_patch_bytes: budget.max_patch_bytes,
            max_frames: budget.max_frames,
        })?;
        Ok(Self::Poll { events, command_page, cold_pair_page, limits })
    }
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
pub struct OwnedCodecInput<'a> {
    pub artifact_schema: &'a str,
    pub document_id: &'a str,
    pub pack: &'a [u8],
    pub spr: &'a [u8],
    pub ops: &'a [u8],
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

/// 🪪️ Identifies the complete interpreter/runtime source and floating-point writer.
pub fn owned_engine_identity() -> &'static str {
    static IDENTITY: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    IDENTITY.get_or_init(|| {
        let sources = [semio_plugin_interpreter::INTERPRETER_SOURCE, semio_plugin_interpreter::NUMERIC_SOURCE, OWNED_RUNTIME_SOURCE, semio_plugin_interpreter_imports::ORIGINAL_HOST_SOURCE];
        let mut engine = Vec::with_capacity(72 + sources.iter().map(|source| source.len()).sum::<usize>());
        engine.extend_from_slice(b"semio.owned-engine/v1\n");
        for source in sources {
            engine.extend_from_slice(&(source.len() as u64).to_le_bytes());
            engine.extend_from_slice(source.as_bytes());
        }
        format!("owned-engine-v1:{}", semio_framework_hash::hex_lower(semio_framework_hash::hash(&engine).as_bytes()))
    })
}

/// 🏃️ The owned interpreter implements the guest runtime.
pub struct OwnedRuntime {
    pub(super) next_instance_id: std::sync::atomic::AtomicU32,
    pub(super) original_clock:fn()->Option<u64>,
    pub(super) input_retirement:semio_plugin_interpreter::CoreInputRetirementAuthority,
}
//#endregion 🧠️OwnedRuntime
