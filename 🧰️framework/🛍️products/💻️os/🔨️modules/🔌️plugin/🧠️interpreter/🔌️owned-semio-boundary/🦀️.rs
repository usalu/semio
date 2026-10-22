//! 🔌️ Owned semio boundary: the checkpoint constants, the semio export names and the owned export enum, the owned artifact, instance and checkpoint layout. See [`super`].

use super::*;

//#region 🧠️OwnedSemioBoundary
const SEMIO_ACTOR_CHECKPOINT_MAGIC: &[u8; 8] = b"SEMIOACT";

const SEMIO_ACTOR_CHECKPOINT_VERSION: u8 = 1;

pub const SEMIO_OWNED_ALLOC_EXPORT: &str = "semio_owned_alloc_v1";

pub const SEMIO_OWNED_DEALLOC_EXPORT: &str = "semio_owned_dealloc_v1";

pub const SEMIO_OWNED_CHECKPOINT_EXPORT: &str = "semio_owned_checkpoint_v1";

pub const SEMIO_OWNED_RESTORE_EXPORT: &str = "semio_owned_restore_v1";

pub const SEMIO_OWNED_DESCRIBE_EXPORT: &str = "semio_owned_describe_v1";

pub const SEMIO_OWNED_CANCEL_JOB_EXPORT: &str = "semio_owned_cancel_job_v1";

pub const SEMIO_OWNED_START_JOB_EXPORT: &str = "semio_owned_start_job_v1";

pub const SEMIO_OWNED_STEP_JOB_EXPORT: &str = "semio_owned_step_job_v1";

pub const SEMIO_OWNED_READ_JOB_OUTPUT_EXPORT: &str = "semio_owned_read_job_output_v1";

pub const SEMIO_OWNED_ACKNOWLEDGE_JOB_OUTPUT_EXPORT: &str = "semio_owned_acknowledge_job_output_v1";

pub const SEMIO_OWNED_CLOSE_JOBS_EXPORT: &str = "semio_owned_close_jobs_v1";

pub const SEMIO_OWNED_BORROW_JOB_OUTPUT_EXPORT:&str="semio_owned_borrow_job_output_v1";

pub const SEMIO_OWNED_ACKNOWLEDGE_JOB_OUTPUT_CHUNK_EXPORT:&str="semio_owned_acknowledge_job_output_chunk_v1";

pub const SEMIO_OWNED_POLL_EXPORT: &str = "semio_owned_poll_v1";

pub const SEMIO_OWNED_ADMIT_ACTOR_CONTEXT_EXPORT:&str="semio_owned_admit_actor_context_v1";

pub const SEMIO_OWNED_CLOSE_ACTOR_CONTEXT_EXPORT:&str="semio_owned_close_actor_context_v1";

/// 🤝️ The owned twin of `reactor.channel-version`: the guest's compiled app-channel version, which a host admits before any frame.
pub const SEMIO_OWNED_CHANNEL_VERSION_EXPORT: &str = "semio_owned_channel_version_v1";

/// 🧬️ OS-HUB-COLLABORATION-AI-END-TO-END (TC3b): the owned twin of `world actor`'s `codec`
/// interface — the four pure document-kind functions a headless host calls on a package whose Rust
/// codec it does not link.
pub const SEMIO_OWNED_PACK_SCHEMA_HASH_EXPORT: &str = "semio_owned_pack_schema_hash_v1";

pub const SEMIO_OWNED_GENESIS_EXPORT: &str = "semio_owned_genesis_v1";

pub const SEMIO_OWNED_PRINT_MIRROR_EXPORT: &str = "semio_owned_print_mirror_v1";

pub const SEMIO_OWNED_APPLY_OPS_EXPORT: &str = "semio_owned_apply_ops_v1";

/// 📜️ The owned twin of `codec.replay-envelopes`, the hub's Check In fold.
pub const SEMIO_OWNED_REPLAY_ENVELOPES_EXPORT: &str = "semio_owned_replay_envelopes_v1";

pub const SEMIO_OWNED_SQLITE_SCHEMA_EXPORT: &str = "semio_owned_sqlite_schema_v1";

pub const SEMIO_OWNED_SQLITE_EXPORT_EXPORT: &str = "semio_owned_sqlite_export_v1";

pub const SEMIO_OWNED_SQLITE_IMPORT_EXPORT: &str = "semio_owned_sqlite_import_v1";

pub const SEMIO_OWNED_SQLITE_TAKE_PAYLOAD_EXPORT:&str="semio_owned_sqlite_take_payload_v1";

pub const SEMIO_OWNED_SQLITE_TAKE_FILE_EXPORT:&str="semio_owned_sqlite_take_file_v1";

pub const SEMIO_OWNED_SQLITE_CLOSE_EXPORT:&str="semio_owned_sqlite_close_v1";

pub const SEMIO_OWNED_SQLITE_RETIREMENT_EXPORT:&str="semio_owned_sqlite_retirement_v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OwnedSemioExport {
    Allocate,
    Deallocate,
    Checkpoint,
    Restore,
    Describe,
    ChannelVersion,
    CancelJob,
    StartJob,
    StepJob,
    ReadJobOutput,
    AcknowledgeJobOutput,
    CloseJobs,
    BorrowJobOutput,
    AcknowledgeJobOutputChunk,
    Poll,
    AdmitActorContext,
    CloseActorContext,
    PackSchemaHash,
    Genesis,
    PrintMirror,
    ApplyOps,
    ReplayEnvelopes,
    SqliteSchema,
    SqliteExport,
    SqliteImport,
    SqliteRetirement,
    SqliteClose,
    SqliteTakeFile,
    SqliteTakePayload,
}

impl OwnedSemioExport {
    pub const ALL: [Self; 29] = [
        Self::Allocate,
        Self::Deallocate,
        Self::Checkpoint,
        Self::Restore,
        Self::Describe,
        Self::ChannelVersion,
        Self::CancelJob,
        Self::StartJob,
        Self::StepJob,
        Self::ReadJobOutput,
        Self::AcknowledgeJobOutput,
        Self::CloseJobs,
        Self::BorrowJobOutput,
        Self::AcknowledgeJobOutputChunk,
        Self::Poll,
        Self::AdmitActorContext,
        Self::CloseActorContext,
        Self::PackSchemaHash,
        Self::Genesis,
        Self::PrintMirror,
        Self::ApplyOps,
        Self::ReplayEnvelopes,
        Self::SqliteSchema,
        Self::SqliteExport,
        Self::SqliteImport,
        Self::SqliteRetirement,
        Self::SqliteClose,
        Self::SqliteTakeFile,
        Self::SqliteTakePayload,
    ];

    pub fn core_name(self) -> &'static str {
        match self {
            Self::Allocate => SEMIO_OWNED_ALLOC_EXPORT,
            Self::Deallocate => SEMIO_OWNED_DEALLOC_EXPORT,
            Self::Checkpoint => SEMIO_OWNED_CHECKPOINT_EXPORT,
            Self::Restore => SEMIO_OWNED_RESTORE_EXPORT,
            Self::Describe => SEMIO_OWNED_DESCRIBE_EXPORT,
            Self::ChannelVersion => SEMIO_OWNED_CHANNEL_VERSION_EXPORT,
            Self::CancelJob => SEMIO_OWNED_CANCEL_JOB_EXPORT,
            Self::StartJob => SEMIO_OWNED_START_JOB_EXPORT,
            Self::StepJob => SEMIO_OWNED_STEP_JOB_EXPORT,
            Self::ReadJobOutput => SEMIO_OWNED_READ_JOB_OUTPUT_EXPORT,
            Self::AcknowledgeJobOutput => SEMIO_OWNED_ACKNOWLEDGE_JOB_OUTPUT_EXPORT,
            Self::CloseJobs => SEMIO_OWNED_CLOSE_JOBS_EXPORT,
            Self::BorrowJobOutput=>SEMIO_OWNED_BORROW_JOB_OUTPUT_EXPORT,
            Self::AcknowledgeJobOutputChunk=>SEMIO_OWNED_ACKNOWLEDGE_JOB_OUTPUT_CHUNK_EXPORT,
            Self::Poll => SEMIO_OWNED_POLL_EXPORT,
            Self::AdmitActorContext=>SEMIO_OWNED_ADMIT_ACTOR_CONTEXT_EXPORT,
            Self::CloseActorContext=>SEMIO_OWNED_CLOSE_ACTOR_CONTEXT_EXPORT,
            Self::PackSchemaHash => SEMIO_OWNED_PACK_SCHEMA_HASH_EXPORT,
            Self::Genesis => SEMIO_OWNED_GENESIS_EXPORT,
            Self::PrintMirror => SEMIO_OWNED_PRINT_MIRROR_EXPORT,
            Self::ApplyOps => SEMIO_OWNED_APPLY_OPS_EXPORT,
            Self::ReplayEnvelopes => SEMIO_OWNED_REPLAY_ENVELOPES_EXPORT,
            Self::SqliteSchema => SEMIO_OWNED_SQLITE_SCHEMA_EXPORT,
            Self::SqliteExport => SEMIO_OWNED_SQLITE_EXPORT_EXPORT,
            Self::SqliteImport => SEMIO_OWNED_SQLITE_IMPORT_EXPORT,
            Self::SqliteRetirement => SEMIO_OWNED_SQLITE_RETIREMENT_EXPORT,
            Self::SqliteClose => SEMIO_OWNED_SQLITE_CLOSE_EXPORT,
            Self::SqliteTakeFile => SEMIO_OWNED_SQLITE_TAKE_FILE_EXPORT,
            Self::SqliteTakePayload => SEMIO_OWNED_SQLITE_TAKE_PAYLOAD_EXPORT,

        }
    }

    pub(super) fn core_type(self) -> FunctionType {
        match self {
            Self::Allocate => FunctionType { parameters: vec![ValueType::I32], results: vec![ValueType::I32] },
            Self::Deallocate => FunctionType { parameters: vec![ValueType::I32, ValueType::I32], results: vec![] },
            Self::Checkpoint | Self::Describe | Self::ChannelVersion => FunctionType { parameters: vec![], results: vec![ValueType::I64] },
            Self::StepJob | Self::ReadJobOutput | Self::AcknowledgeJobOutput | Self::CancelJob => FunctionType{parameters:vec![ValueType::I64;20],results:vec![ValueType::I64]},
            Self::CloseJobs => FunctionType{parameters:vec![ValueType::I64;19],results:vec![ValueType::I64]},
            Self::BorrowJobOutput=>FunctionType{parameters:vec![ValueType::I64;5],results:vec![ValueType::I64]},
            Self::AcknowledgeJobOutputChunk=>FunctionType{parameters:vec![ValueType::I64;22],results:vec![ValueType::I64]},
            Self::Restore | Self::StartJob | Self::Poll | Self::AdmitActorContext | Self::CloseActorContext | Self::PackSchemaHash | Self::Genesis | Self::PrintMirror | Self::ApplyOps | Self::ReplayEnvelopes | Self::SqliteSchema | Self::SqliteExport | Self::SqliteImport | Self::SqliteRetirement | Self::SqliteClose | Self::SqliteTakeFile | Self::SqliteTakePayload => {
                FunctionType { parameters: vec![ValueType::I32, ValueType::I32], results: vec![ValueType::I64] }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct OwnedSemioArtifact {
    pub(super) component: Arc<ComponentArtifact>,
    actor_module: Arc<CoreModule>,
}

impl OwnedSemioArtifact {
    pub fn parse(bytes: &[u8]) -> Result<Self, CoreError> {
        Self::from_component(Arc::new(ComponentArtifact::parse(bytes)?))
    }

    pub fn from_component(component: Arc<ComponentArtifact>) -> Result<Self, CoreError> {
        let mut candidates = component.core_modules.iter().filter(|module| {
            OwnedSemioExport::ALL.iter().all(|export| match module.export(export.core_name()) {
                Some(ExportKind::Function(function)) => module.function_type(*function).is_ok_and(|function_type| function_type == &export.core_type()),
                _ => false,
            })
        });
        let actor_module = Arc::clone(candidates.next().ok_or_else(|| CoreError::Validation("component has no core module implementing the owned Semio actor ABI".into()))?);
        if candidates.next().is_some() {
            return Err(CoreError::Validation("component has multiple core modules implementing the owned Semio actor ABI".into()));
        }
        if !matches!(actor_module.export("memory"), Some(ExportKind::Memory(_))) {
            return Err(CoreError::Validation("owned Semio actor core module does not export memory".into()));
        }
        Ok(Self { component, actor_module })
    }

    /// 🪢️ The immutable module witness used by original phased opening.
    pub fn original_actor_module(&self)->&Arc<CoreModule>{&self.actor_module}

    pub fn fingerprint(&self) -> u64 {
        self.component.fingerprint()
    }

    pub fn instantiate(&self) -> Result<OwnedSemioInstance, CoreError> {
        Ok(OwnedSemioInstance { component_fingerprint: self.fingerprint(), core: CoreInstance::instantiate(Arc::clone(&self.actor_module))? })
    }

    pub fn restore(&self, checkpoint: &[u8]) -> Result<OwnedSemioInstance, CoreError> {
        if checkpoint.get(..8) != Some(SEMIO_ACTOR_CHECKPOINT_MAGIC) || checkpoint.get(8) != Some(&SEMIO_ACTOR_CHECKPOINT_VERSION) {
            return Err(CoreError::State("invalid owned Semio actor checkpoint header".into()));
        }
        let component_fingerprint = checkpoint.get(9..17).and_then(|bytes| bytes.try_into().ok()).map(u64::from_le_bytes).ok_or_else(|| CoreError::State("truncated owned Semio actor checkpoint fingerprint".into()))?;
        if component_fingerprint != self.fingerprint() {
            return Err(CoreError::State("owned Semio actor checkpoint component fingerprint mismatch".into()));
        }
        let core_length =
            checkpoint.get(17..25).and_then(|bytes| bytes.try_into().ok()).map(u64::from_le_bytes).and_then(|length| usize::try_from(length).ok()).ok_or_else(|| CoreError::State("invalid owned Semio actor core checkpoint length".into()))?;
        let core_checkpoint = checkpoint.get(25..).filter(|bytes| bytes.len() == core_length).ok_or_else(|| CoreError::State("owned Semio actor core checkpoint length mismatch".into()))?;
        Ok(OwnedSemioInstance { component_fingerprint, core: CoreInstance::restore(Arc::clone(&self.actor_module), core_checkpoint)? })
    }
}

#[derive(Clone, Debug)]
pub struct OwnedSemioInstance {
    pub(super) component_fingerprint: u64,
    pub(super) core: CoreInstance,
}

pub struct OwnedSemioCheckpointLayout<'a>{pub(super) actor:&'a OwnedSemioInstance,core_bytes:usize}

impl OwnedSemioCheckpointLayout<'_>{
    /// 👓️ Writes the same retained borrowed VM after its original observed measurement.
    pub fn visit(&self,visitor:&mut dyn FnMut(&[u8])->Result<(),semio_framework_value::ValueError>)->Result<(),semio_framework_value::ValueError>{
        visitor(SEMIO_ACTOR_CHECKPOINT_MAGIC)?;visitor(&[SEMIO_ACTOR_CHECKPOINT_VERSION])?;visitor(&self.actor.component_fingerprint.to_le_bytes())?;visitor(&(self.core_bytes as u64).to_le_bytes())?;
        let mut written=0usize;self.actor.core.visit_checkpoint(&mut |bytes|{written=written.checked_add(bytes.len()).filter(|written|*written<=self.core_bytes).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"borrowed actor checkpoint changed after measurement"))?;visitor(bytes)})?;
        if written!=self.core_bytes{return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"borrowed actor checkpoint changed after measurement"));}Ok(())
    }
}

impl OwnedSemioInstance {
    /// 📏️ Retains the original VM borrow and scalar extent without allocating checkpoint storage.
    pub fn checkpoint_layout<'a>(&'a self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<OwnedSemioCheckpointLayout<'a>,semio_framework_value::ValueError>{
        control.scoped_stage(|control|{control.begin_stage(0)?;let mut core_bytes=0usize;self.core.visit_checkpoint(&mut |bytes|{core_bytes=core_bytes.checked_add(bytes.len()).filter(|count|*count<=isize::MAX as usize-25).ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"borrowed actor checkpoint extent exceeds address space"))?;control.advance(bytes.len())})?;Ok(OwnedSemioCheckpointLayout{actor:self,core_bytes})})
    }

    pub fn startup_active(&self) -> bool {
        self.core.active()
    }

    pub fn begin(&mut self, export: OwnedSemioExport, arguments: Vec<Value>) -> Result<(), CoreError> {
        self.core.begin_export(export.core_name(), arguments)
    }

    pub fn step(&mut self, fuel: u64, control: StepControl) -> CoreStepOutcome {
        self.core.step(fuel, control)
    }

    pub fn resume_host(&mut self, call_id: u64, result: Result<Vec<Value>, String>) -> Result<(), CoreError> {
        self.core.resume_host(call_id, result)
    }

    pub fn memory(&self) -> Option<&[u8]> {
        let Some(ExportKind::Memory(memory)) = self.core.module.export("memory") else { return None };
        self.core.memory(*memory)
    }

    pub fn memory_mut(&mut self) -> Option<&mut [u8]> {
        let memory = match self.core.module.export("memory") {
            Some(ExportKind::Memory(memory)) => *memory,
            _ => return None,
        };
        self.core.memory_mut(memory)
    }

    pub fn read_bytes_result(&self, values: &[Value], maximum_bytes: usize) -> Result<Vec<u8>, CoreError> {
        let [Value::I64(pair)] = values else { return Err(CoreError::Host("owned Semio export returned an invalid pointer/length pair".into())) };
        let pair = *pair as u64;
        let start = pair as u32 as usize;
        let length = (pair >> 32) as u32 as usize;
        if length > maximum_bytes {
            return Err(CoreError::Host(format!("owned Semio export length {length} exceeds limit {maximum_bytes}")));
        }
        let memory = self.memory().ok_or_else(|| CoreError::State("owned Semio actor memory is unavailable".into()))?;
        let end = checked_end(start, length, memory.len(), "owned Semio export")?;
        Ok(memory[start..end].to_vec())
    }

    pub fn checkpoint(&self) -> Vec<u8> {
        let core = self.core.checkpoint();
        let mut checkpoint = Vec::with_capacity(25 + core.len());
        checkpoint.extend_from_slice(SEMIO_ACTOR_CHECKPOINT_MAGIC);
        checkpoint.push(SEMIO_ACTOR_CHECKPOINT_VERSION);
        checkpoint.extend_from_slice(&self.component_fingerprint.to_le_bytes());
        checkpoint.extend_from_slice(&(core.len() as u64).to_le_bytes());
        checkpoint.extend_from_slice(&core);
        checkpoint
    }
}
//#endregion 🧠️OwnedSemioBoundary
