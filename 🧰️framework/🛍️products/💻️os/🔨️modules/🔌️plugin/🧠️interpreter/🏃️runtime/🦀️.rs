//! 🏃️ The owned interpreter as a guest runtime ([`OwnedRuntime`]): compiles a `world actor` component for the
//! repository-owned interpreter ([`crate::interpreter`]), runs its actor turns, jobs, checkpoints and pure `codec` exports
//! under instruction fuel, and answers every host import a guest can observe. It also owns the guest-runtime vocabulary every
//! backend shares — [`TurnFault`], [`PluginHostError`], [`JobBudget`], [`JobStep`], [`GuestCallCancellation`] — so the
//! Wasmtime host builds on it, and a tool that only describes or verifies components links no JIT.
//! @see https://webassembly.github.io/spec/core/exec/index.html
use crate::interpreter::{CoreStepOutcome, HostCall, OwnedSemioArtifact, OwnedSemioExport, OwnedSemioInstance, StepControl, Value};
use crate::{operation_authority, original_actor_context, sqlite_wire};
use semio_framework::kernel::{Budget, Event, TurnResult};
use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceiving;
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

#[path = "🎛️operation/🦀️.rs"]
pub mod operation;

//#region 🧯️GuestFaults
/// 🗝️ Conflicting ownership of one composer route.
#[derive(Debug)]
pub struct IoRouteConflict {
    pub key: semio_framework_os_kernel::io::IoKey,
    pub existing_plugin: String,
    pub incoming_plugin: String,
}

/// 🛤️ Conflicting ownership of one dialect conversion route.
#[derive(Debug)]
pub struct IoEntryRouteConflict {
    pub from: semio_framework_artifact_reference::ArtifactDialect,
    pub into: semio_framework_artifact_reference::ArtifactDialect,
    pub existing_plugin: String,
    pub incoming_plugin: String,
}

/// 🧯️ Errors from a guest runtime's host plumbing: the engine, the compiled component, the call boundary and the
/// host routers that admit what a guest declares.
#[derive(Debug)]
pub enum PluginHostError {
    Io(std::io::Error),
    Json(String),
    NativeIo(semio_framework_value::ValueError),
    Wasmtime(String),
    Plugin(String),
    IoRouteConflict(Box<IoRouteConflict>),
    /// 🌉️ CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (W1-D): the NEW `(ArtifactDialect,
    /// ArtifactDialect)`-keyed graph's own conflict — separate from `IoRouteConflict` above (OLD
    /// `IoKey`-keyed graph), since the two mechanisms are additive and independently registered.
    IoEntryRouteConflict(Box<IoEntryRouteConflict>),
    PluginRuntimeConflict {
        plugin_id: String,
    },
    LockPoisoned(&'static str),
    /// 🤝️ A guest the host refused to admit, with the framework fault a shell tells as its notice (`plugin.channel-mismatch`).
    Refused(Box<semio_framework::Fault>),
}

impl std::fmt::Display for PluginHostError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "io: {error}"),
            Self::Json(error) => write!(formatter, "json: {error}"),
            Self::NativeIo(error)=>write!(formatter,"native io: {error}"),
            Self::Wasmtime(message) => write!(formatter, "wasmtime: {message}"),
            Self::Plugin(message) => write!(formatter, "plugin: {message}"),
            Self::IoRouteConflict(conflict) => write!(formatter, "io route conflict for {:?}: {} already owns it; {} cannot replace it", conflict.key, conflict.existing_plugin, conflict.incoming_plugin),
            Self::IoEntryRouteConflict(conflict) => write!(formatter, "io entry route conflict for {:?} -> {:?}: {} already owns it; {} cannot replace it", conflict.from, conflict.into, conflict.existing_plugin, conflict.incoming_plugin),
            Self::PluginRuntimeConflict { plugin_id } => write!(formatter, "plugin runtime conflict for {plugin_id}"),
            Self::LockPoisoned(name) => write!(formatter, "{name} lock poisoned"),
            Self::Refused(fault) => write!(formatter, "{}", fault.describe()),
        }
    }
}

impl std::error::Error for PluginHostError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::NativeIo(error)=>Some(error),
            _ => None,
        }
    }
}

impl From<std::io::Error> for PluginHostError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

/// ⛽️ `jobs.wit`'s `job-budget` record, mirrored field-for-field (design-abi.md §1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
pub struct JobBudget {
    pub fuel: u64,
    pub deadline_ms: u32,
}

/// 🪜️ `jobs.wit`'s `job-step` variant, mirrored field-for-field. `Serialize`/`Deserialize` added
/// (past `design-runtime.md` §2's literal trait listing) so `🧵️shard/🦀️.rs`'s `ShardOutcome`
/// can carry one over a `ShardTransport` — every other outcome shape (`TurnResult`) already derives
/// both, so this was the one gap.
///
/// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (J1): `Running` is a STRUCT variant, not the newtype
/// `Running(Option<Vec<u8>>)` this used to be — serde's internal tagging (`#[serde(tag = "kind")]`)
/// cannot serialize a newtype variant whose payload is itself an `Option<T>` ("cannot serialize
/// tagged newtype variant ... containing an optional" — a real serde limitation, not a typo).
/// `ShardLoop::pump`'s job-stepping path never actually SENT a `Running` outcome over a transport
/// before this packet (no test drove a job past its first step, so a `Running` step never reached
/// `send_outcome`'s `serde_json::to_vec` — the exact "a contract that compiles is not a contract
/// that runs" shape this whole packet exists to close), which is how this stayed latent. Every
/// OTHER `Option`-carrying variant in this crate's kernel types already uses a struct variant for
/// exactly this reason (e.g. `kernel::Event::JobProgress { job, progress: Option<Vec<u8>> }`).
///
/// 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (K1/registrar): `Done`/`Failed` are struct variants
/// for the SAME reason, found one wave later. J1 fixed only `Running`, because serde's internal
/// tagging rejects a newtype variant carrying an `Option` — but it rejects one carrying ANY
/// sequence, and `Vec<u8>` is a sequence: `serde_json` errors with "cannot serialize tagged newtype
/// variant JobStep::Done containing a sequence". So every *successful* job completion failed to
/// serialize, on the one path a job must survive to be useful. J1's resumability test drove three
/// `step_job` calls but asserted on the in-process `JobStep` values rather than on bytes that had
/// crossed `send_outcome`, so the completion path was proven in memory and never on the wire.
/// The lesson generalises past serde: **fixing one variant of a defect is not fixing the defect** —
/// the sibling variants must be re-derived from the rule, not from the symptom that was reported.
#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum JobStep {
    Running { progress: Option<Vec<u8>> },
    Done { output: Vec<u8> },
    Failed { error: Vec<u8> },
}

/// 🧯️ Why a turn or job step didn't produce a result — distinct from [`PluginHostError`] (host-side
/// plumbing faults): every `GuestRuntime::execute_turn`/`step_job` failure is one of these.
#[derive(Debug)]
pub enum TurnFault {
    ActorCall(original_actor_context::OriginalActorFixedCall),
    Host(PluginHostError),
    Guest(semio_framework::Fault),
    Exhausted,
    Trapped(String),
    DeadlineExceeded,
    FuelExhausted,
    /// ⛔️ The caller's [`GuestCallCancellation`] ended the call between two interpreter steps.
    Cancelled,
}

/// 🧯️ A guest codec call's cancellation, observed by the owned interpreter between every two steps
/// ([`OWNED_STEP_FUEL`] instructions): a caller that is gone — a cancelled creation, the hub's background catalog
/// verification aborted by a SIGTERM — ends the interpretation at once instead of letting it run on to its own fuel
/// bound (measured before: a verification kept the hub process alive 17.9–25.2 s after its `server.shutdown`).
#[derive(Clone, Debug, Default)]
pub struct GuestCallCancellation(Arc<AtomicBool>);

impl GuestCallCancellation {
    /// 🛑️ Ends the call at the interpreter's next step.
    pub fn cancel(&self) {
        self.0.store(true, std::sync::atomic::Ordering::Release);
    }

    /// 🔎️ Whether the call was cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Acquire)
    }
}

impl std::fmt::Display for TurnFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Host(error) => std::fmt::Display::fmt(error, formatter),
            Self::ActorCall(call)=>write!(formatter,"original actor call {}/{}/{} refused after {:?}",call.input.operation,call.input.generation,call.input.epoch,call.spent),
            Self::Guest(fault) => write!(formatter, "guest fault {}: {}", fault.code.0, fault.message),
            Self::Exhausted => formatter.write_str("guest instance has no more scripted/actual turns"),
            Self::Trapped(message) => write!(formatter, "guest trapped: {message}"),
            Self::DeadlineExceeded => formatter.write_str("epoch deadline exceeded"),
            Self::FuelExhausted => formatter.write_str("fuel exhausted"),
            Self::Cancelled => formatter.write_str("guest call cancelled by its caller"),
        }
    }
}

impl std::error::Error for TurnFault {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Host(error) => Some(error),
            _ => None,
        }
    }
}

/// 📏️ The largest encoded guest fault a host decodes; a larger one is refused as a host fault, never trusted.
pub const GUEST_FAULT_MAXIMUM_BYTES: usize = 65_536;

/// 🧯️ Decodes the bounded fault bytes a guest export returned into its typed [`TurnFault::Guest`].
pub fn decode_guest_fault_bytes(bytes: &[u8]) -> TurnFault {
    if bytes.len() > GUEST_FAULT_MAXIMUM_BYTES {
        return TurnFault::Host(PluginHostError::Plugin("guest fault exceeds bounded wire capacity".into()));
    }
    TurnFault::Guest(semio_framework_diagnostic::decode_fault_bytes(bytes))
}

impl From<PluginHostError> for TurnFault {
    fn from(error: PluginHostError) -> Self {
        Self::Host(error)
    }
}
//#endregion 🧯️GuestFaults

/// 🩺️ How much guest stderr one instance retains while diagnostics are armed. Bounded because the
/// pipe TRAPS the guest once it overflows, and a perf run's trace volume is byte-proportional to the
/// document — 4 MiB is far more than any measured boot produces and far less than the guest's own
/// 512 MiB linear-memory budget.
pub const GUEST_DIAGNOSTICS_CAPACITY_BYTES: usize = 4 * 1024 * 1024;

/// 🧊️ Admits one native cold-pair page: its length is the one its header assigns to its index and its baseline frontier
/// carries a verified chain. Both runtimes run it before a page reaches a guest.
pub fn validate_cold_page(page: &semio_framework::kernel::ColdDocumentPairPage) -> Result<(), TurnFault> {
    let expected = page.header.page_length(page.page_index).map_err(|reason| TurnFault::Trapped(reason.to_owned()))?;
    if page.bytes.len() != expected || page.header.baseline_frontier.chain_sha256 == [0; 32] {
        return Err(TurnFault::Trapped("cold-pair page length or verified frontier is invalid".to_owned()));
    }
    Ok(())
}

//#region 🧠️OwnedRuntime
/// 📦️ The largest result bytes one owned export may return.
const OWNED_RESULT_LIMIT: usize = 64 * 1024 * 1024;
/// 🪪️ The header of an owned host checkpoint frame.
const OWNED_CHECKPOINT_MAGIC: &[u8; 8] = b"SMOWNH01";
/// ⛽️ The instructions one interpreter step may run before the host looks at its deadline, cancellation and progress.
pub const OWNED_STEP_FUEL: u64 = 4_096;
/// ⏱️ The wall bound of one owned job cancellation slice.
const OWNED_SLICE_DEADLINE_MS: u32 = 8;

/// 🧊️ The artifact schema a codec origin's assembly asks `codec.pack-schema-hash` for: one no app declares, so the
/// export assembles the bundle, answers its refusal and touches no app's codec.
const OWNED_CODEC_ORIGIN_SCHEMA: &str = "";

/// ⏲️ How often a codec call waiting for another call's origin assembly relays that assembly's fuel and looks at its
/// own cancellation.
const OWNED_CODEC_ORIGIN_WAIT_POLL: std::time::Duration = std::time::Duration::from_millis(50);

/// 🧠️ One component compiled for the owned interpreter: its parsed artifact, and the origin its codec calls start from
/// once one of them assembled it. A clone shares both, so every holder of one compiled guest starts its codec calls from the
/// same origin.
#[derive(Clone)]
pub struct OwnedCompiledGuest {
    artifact: Arc<OwnedSemioArtifact>,
    codec_origin: Arc<OwnedCodecOriginCell>,
}

impl OwnedCompiledGuest {
    /// 📏️ The bytes this compiled guest holds beyond its parsed component: its codec origin once a codec call assembled it
    /// ([`OwnedCodecOriginCell`]), else none. A residency budget charges a compiled guest this on top of its component.
    pub fn codec_origin_bytes(&self) -> u64 {
        self.codec_origin.resident_bytes()
    }
}

/// 🧊️ A compiled guest's codec origin: its owned instance right after a first codec export assembled the plugin bundle
/// (`__semio_ensure_plugin_runtime`: every artifact declared, every mutation roster, the built bundle) and completed.
/// Measured on catalog B3 (H12, 2026-09-27): a fresh instance spends 521.5 M of a puzzle codec call's 524.7 M fuel on that
/// assembly and the operation itself ≈ 3.2 M, and every codec call used to pay it on a throwaway instance. Each codec call
/// now starts from a copy of this state, so every call still sees exactly the same guest — the assembled bundle and
/// nothing another call did — and answers byte for byte what a fresh instance answers.
struct OwnedCodecOrigin {
    actor: OwnedSemioInstance,
    diagnostics: Vec<u8>,
    context: i32,
    next_resource: i32,
    assembly_fuel: u64,
}

impl OwnedCodecOrigin {
    /// 📏️ What the origin holds: the guest's linear memory and what it printed while assembling.
    fn resident_bytes(&self) -> u64 {
        u64::try_from(self.actor.memory().map_or(0, <[u8]>::len).saturating_add(self.diagnostics.len())).unwrap_or(u64::MAX)
    }
}

/// 🚦️ Where one compiled guest's codec origin stands; `Assembling` carries the fuel the assembly reported so far.
enum OwnedCodecOriginState {
    Absent,
    Assembling { fuel: u64 },
    Ready(Arc<OwnedCodecOrigin>),
}

/// 🧊️ The codec origin of one compiled guest, assembled once, by the first codec call that needs it. A call that arrives
/// while another assembles waits for that assembly instead of running a second one, relays its fuel as its own progress —
/// a stall bound watching the waiter sees the assembly move, and sees it stop — and observes its own cancellation. An
/// assembly that faults, is cancelled or panics leaves the origin absent, and the next call assembles it itself.
struct OwnedCodecOriginCell {
    state: Mutex<OwnedCodecOriginState>,
    settled: std::sync::Condvar,
}

impl OwnedCodecOriginCell {
    fn new() -> Self {
        Self { state: Mutex::new(OwnedCodecOriginState::Absent), settled: std::sync::Condvar::new() }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, OwnedCodecOriginState> {
        self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn resident_bytes(&self) -> u64 {
        match &*self.lock() {
            OwnedCodecOriginState::Ready(origin) => origin.resident_bytes(),
            OwnedCodecOriginState::Absent | OwnedCodecOriginState::Assembling { .. } => 0,
        }
    }

    /// 📈️ Records the running assembly's fuel for the calls waiting on it.
    fn assembled_so_far(&self, fuel: u64) {
        if let OwnedCodecOriginState::Assembling { fuel: reported } = &mut *self.lock() {
            *reported = fuel;
        }
        self.settled.notify_all();
    }
}

/// 🧊️ A codec call's use of its compiled guest's origin: the origin, the fuel THIS call spent assembling it (the part of its
/// budget it used), and the fuel it already reported as progress — the assembly's own, or the one it relayed while it waited
/// for another call's — so its operation's progress continues from there instead of starting over.
struct OwnedCodecOriginUse {
    origin: Arc<OwnedCodecOrigin>,
    spent_fuel: u64,
    reported_fuel: u64,
}

/// 🧯️ One origin assembly in flight: however it ends — with an origin, a fault, a cancellation or a panic — the cell
/// settles (ready, or absent again) and every waiting call wakes.
struct OwnedCodecOriginAssembly<'a> {
    cell: &'a OwnedCodecOriginCell,
    origin: Option<Arc<OwnedCodecOrigin>>,
}

impl Drop for OwnedCodecOriginAssembly<'_> {
    fn drop(&mut self) {
        *self.cell.lock() = self.origin.take().map_or(OwnedCodecOriginState::Absent, OwnedCodecOriginState::Ready);
        self.cell.settled.notify_all();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
enum OwnedOperation {
    Describe,
    ChannelVersion,
    Poll,
    AdmitActorContext,
    CloseActorContext,
    StartJob,
    StepJob,
    CancelJob,
    Checkpoint,
    Restore,
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

impl OwnedOperation {
    fn export(self) -> OwnedSemioExport {
        match self {
            Self::Describe => OwnedSemioExport::Describe,
            Self::ChannelVersion => OwnedSemioExport::ChannelVersion,
            Self::Poll => OwnedSemioExport::Poll,
            Self::AdmitActorContext=>OwnedSemioExport::AdmitActorContext,
            Self::CloseActorContext=>OwnedSemioExport::CloseActorContext,
            Self::StartJob => OwnedSemioExport::StartJob,
            Self::StepJob => OwnedSemioExport::StepJob,
            Self::CancelJob => OwnedSemioExport::CancelJob,
            Self::Checkpoint => OwnedSemioExport::Checkpoint,
            Self::Restore => OwnedSemioExport::Restore,
            Self::PackSchemaHash => OwnedSemioExport::PackSchemaHash,
            Self::Genesis => OwnedSemioExport::Genesis,
            Self::PrintMirror => OwnedSemioExport::PrintMirror,
            Self::ApplyOps => OwnedSemioExport::ApplyOps,
            Self::ReplayEnvelopes => OwnedSemioExport::ReplayEnvelopes,
            Self::SqliteSchema => OwnedSemioExport::SqliteSchema,
            Self::SqliteExport => OwnedSemioExport::SqliteExport,
            Self::SqliteImport => OwnedSemioExport::SqliteImport,
            Self::SqliteRetirement => OwnedSemioExport::SqliteRetirement,
            Self::SqliteClose => OwnedSemioExport::SqliteClose,
            Self::SqliteTakeFile => OwnedSemioExport::SqliteTakeFile,
            Self::SqliteTakePayload => OwnedSemioExport::SqliteTakePayload,

        }
    }
}

#[derive(Clone, Debug, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
enum OwnedStage {
    Allocate { input: Vec<u8> },
    Call,
    Deallocate { output: Vec<u8> },
}

#[derive(Clone, Debug, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
struct OwnedPending {
    retained_input: Option<Vec<u8>>,
    operation: OwnedOperation,
    stage: OwnedStage,
    fuel_used: u64,
}

#[derive(Debug, serde::Deserialize, FromValue)]
struct OwnedCheckpointMetadata {
    pending: Option<OwnedPending>,
    original_receipt: Option<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceipt>,
    guest_checkpoint: Option<Vec<u8>>,
    context: i32,
    next_resource: i32,
    instance_id: u32,
}

#[derive(serde::Serialize)]
struct OwnedCheckpointView<'a>{pending:&'a Option<OwnedPending>,original_receipt:&'a Option<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceipt>,guest_checkpoint:&'a Option<Vec<u8>>,context:i32,next_resource:i32,instance_id:u32}

/// 🏃️ One instance of an owned compiled guest: its interpreter state, the operation it has in flight, the original receipt
/// that operation keeps, and what the guest printed.
pub struct OwnedGuestInstance {
    original_job_clock:Option<fn()->Option<u64>>,
    artifact: Arc<OwnedSemioArtifact>,
    actor: OwnedSemioInstance,
    pending: Option<OwnedPending>,
    original_receipt: Option<semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceipt>,
    /// ☠️ Set by the first guest trap. A trapped guest keeps whatever linear memory, shadow-stack
    /// pointer and allocator state the trap left behind, so every later call starts from a lower
    /// stack pointer and traps again 112 bytes further down — ticket 26/09/18 slice A1 read that
    /// drift as the defect itself. There is no recovery short of re-instantiation, so this refuses.
    poisoned: bool,
    /// 🩺️ Everything this guest has written to `wasi:cli/stdout`/`stderr`, bounded by
    /// [`GUEST_DIAGNOSTICS_CAPACITY_BYTES`] — see [`retain_owned_guest_diagnostics`]. Unlike the
    /// wasmtime path this is ALWAYS armed: the owned interpreter serves the stream shim itself at
    /// no cost, and a guest panic message that is thrown away is a whole investigation.
    diagnostics: Vec<u8>,
    context: i32,
    next_resource: i32,
    instance_id: u32,
}

impl OwnedGuestInstance {
    /// 🔁️ True while an owned operation has not completed — the guest yielded on its fuel or deadline and owns the host's next
    /// call.
    pub fn turn_in_flight(&self) -> bool {
        self.pending.is_some()
    }

    /// ☠️ True once a guest trap left this instance unusable; every later call refuses.
    pub fn poisoned(&self) -> bool {
        self.poisoned
    }

    /// 🩺️ Everything this guest has written to `wasi:cli/stdout`/`stderr`, bounded by [`GUEST_DIAGNOSTICS_CAPACITY_BYTES`].
    pub fn diagnostics(&self) -> &[u8] {
        &self.diagnostics
    }

    /// ⏸️ The snapshot operation this instance has in flight and the fuel it ran so far; refused when another owned operation
    /// is in flight.
    pub fn pending_snapshot(&self) -> Result<Option<(OwnedSnapshotOperation, u64)>, TurnFault> {
        self.pending.as_ref().map(|pending| OwnedSnapshotOperation::from_operation(pending.operation).map(|operation| (operation, pending.fuel_used)).ok_or_else(|| TurnFault::Trapped(format!("owned operation {:?} is not a snapshot operation", pending.operation)))).transpose()
    }

    /// 🌱️ Begins `operation` on the input bytes `input` holds, taking them only once the begin is admitted.
    pub fn begin_snapshot(&mut self, operation: OwnedSnapshotOperation, input: &mut Option<Vec<u8>>) -> Result<(), TurnFault> {
        if self.poisoned {
            return Err(TurnFault::Trapped("owned snapshot instance is poisoned".into()));
        }
        if self.pending.is_some() {
            return Err(TurnFault::Trapped("owned snapshot operation remains pending".into()));
        }
        let bytes = input.as_ref().ok_or_else(|| identity_turn_fault(operation_authority::refusal(4)))?;
        let length = i32::try_from(bytes.len()).map_err(|_| identity_turn_fault(operation_authority::refusal(2)))?;
        self.pending = Some(OwnedPending { retained_input: None, operation: operation.operation(), stage: OwnedStage::Allocate { input: input.take().unwrap_or_default() }, fuel_used: 0 });
        self.actor.begin(OwnedSemioExport::Allocate, vec![Value::I32(length)])?;
        Ok(())
    }

    /// ▶️ Resumes the in-flight `operation` under `fuel`, bounding the gap between fuel observations by `deadline_ms`
    /// (`OwnedDeadline::NoFuelProgress`), with every original operation import answered by `receiver`.
    pub fn resume_snapshot(&mut self, operation: OwnedSnapshotOperation, fuel: u64, deadline_ms: u32, progress: impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>, receiver: &mut dyn OriginalOperationReceiving) -> Result<OwnedInvocation, TurnFault> {
        resume_owned_operation_bound(self, operation.operation(), fuel, deadline_ms, OwnedDeadline::NoFuelProgress, progress, cancellation, Some(receiver))
    }
}

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
    fn operation(self) -> OwnedOperation {
        match self {
            Self::Export => OwnedOperation::SqliteExport,
            Self::Import => OwnedOperation::SqliteImport,
            Self::Retirement => OwnedOperation::SqliteRetirement,
            Self::Close => OwnedOperation::SqliteClose,
            Self::TakeFile => OwnedOperation::SqliteTakeFile,
            Self::TakePayload => OwnedOperation::SqliteTakePayload,
        }
    }

    fn from_operation(operation: OwnedOperation) -> Option<Self> {
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
struct OwnedPollInput<'a> {
    events: &'a [&'a Event],
    command_page: Option<(&'a semio_framework::kernel::CommandPageCursor, &'a semio_framework::kernel::FixedCommandPage)>,
    cold_pair_page: Option<&'a semio_framework::kernel::ColdDocumentPairPage>,
    budget: Budget,
}

#[derive(serde::Serialize, ToValue)]
struct OwnedStartJobInput<'a> {
    job: u64,
    kind: &'a str,
    input: Vec<u8>,
}

#[derive(serde::Serialize, ToValue)]
struct OwnedStepJobInput {
    job: u64,
    budget: JobBudget,
}

#[derive(serde::Serialize, ToValue)]
struct OwnedCancelJobInput {
    job: u64,
}

#[derive(serde::Serialize, ToValue)]
struct OwnedRestoreInput {
    state: Vec<u8>,
}

/// 🧬️ OS-HUB-COLLABORATION-AI-END-TO-END (TC3b): the input envelope of the four owned `codec`
/// exports — the mirror of the guest's `owned_abi::CodecInput`.
#[derive(serde::Serialize)]
struct OwnedCodecInput<'a> {
    artifact_schema: &'a str,
    document_id: &'a str,
    pack: &'a [u8],
    spr: &'a [u8],
    ops: &'a [u8],
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

/// 📦️ One document's authoritative binary pair as the owned `codec` exports return it.
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, FromValue)]
pub struct GuestDocumentPair {
    pub pack: Vec<u8>,
    pub spr: Vec<u8>,
}

/// 📥️ The `(dsl, ops)` text mirror the owned `codec.print-mirror` export returns.
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, FromValue)]
pub struct GuestDocumentMirror {
    pub dsl: String,
    pub ops: String,
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
        let interpreter = include_str!("../🦀️.rs").as_bytes();
        let host = include_str!("🦀️.rs").as_bytes();
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
    next_instance_id: std::sync::atomic::AtomicU32,
}

impl Default for OwnedRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedRuntime {
    /// 🌱️ A runtime whose first instance is id 1.
    pub fn new() -> Self {
        Self { next_instance_id: std::sync::atomic::AtomicU32::new(1) }
    }

    /// 🧩️ Parses `bytes` as an owned-ABI component, ready to instantiate; no codec origin is assembled yet.
    pub fn compile_component(&self, bytes: &[u8]) -> Result<OwnedCompiledGuest, PluginHostError> {
        let artifact = OwnedSemioArtifact::parse(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        Ok(OwnedCompiledGuest { artifact: Arc::new(artifact), codec_origin: Arc::new(OwnedCodecOriginCell::new()) })
    }

    /// 🏃️ A fresh instance of `compiled` whose app channel is admitted ([`admit_owned_channel`]).
    pub fn instantiate_actor(&self, compiled: &OwnedCompiledGuest) -> Result<OwnedGuestInstance, PluginHostError> {
        let artifact = &compiled.artifact;
        let owned = artifact.instantiate().map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        if owned.startup_active() {
            return Err(PluginHostError::Plugin("owned actor has an undriven start function".to_string()));
        }
        let instance_id = self.next_instance_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut state = OwnedGuestInstance { original_job_clock:None, artifact: Arc::clone(artifact), actor: owned, pending: None, original_receipt: None, poisoned: false, diagnostics: Vec::new(), context: 0, next_resource: 1, instance_id };
        admit_owned_channel(&mut state).map_err(turn_fault_host)?;
        Ok(state)
    }

    /// 🧊️ `owned`'s codec origin for one call ([`OwnedCodecOriginUse`]). While another call assembles it, this one waits,
    /// relays that assembly's fuel through `progress` and observes `cancellation`; if that assembly ends without an origin,
    /// this call assembles it.
    fn codec_origin(&self, owned: &OwnedCompiledGuest, budget: Budget, progress: &mut impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>) -> Result<OwnedCodecOriginUse, TurnFault> {
        let cell: &OwnedCodecOriginCell = &owned.codec_origin;
        let started = std::time::Instant::now();
        let mut relayed = 0;
        let mut state = cell.lock();
        loop {
            match &*state {
                OwnedCodecOriginState::Ready(origin) => return Ok(OwnedCodecOriginUse { origin: Arc::clone(origin), spent_fuel: 0, reported_fuel: relayed }),
                OwnedCodecOriginState::Absent => break,
                OwnedCodecOriginState::Assembling { fuel } => {
                    if *fuel > relayed {
                        relayed = *fuel;
                        progress(relayed, started.elapsed());
                    }
                    if cancellation.is_some_and(GuestCallCancellation::is_cancelled) {
                        return Err(TurnFault::Cancelled);
                    }
                    state = cell.settled.wait_timeout(state, OWNED_CODEC_ORIGIN_WAIT_POLL).unwrap_or_else(std::sync::PoisonError::into_inner).0;
                }
            }
        }
        *state = OwnedCodecOriginState::Assembling { fuel: 0 };
        drop(state);
        let mut assembly = OwnedCodecOriginAssembly { cell, origin: None };
        let origin = Arc::new(self.assemble_codec_origin(
            owned,
            budget,
            |fuel, elapsed| {
                cell.assembled_so_far(fuel);
                progress(fuel, elapsed);
            },
            cancellation,
        )?);
        assembly.origin = Some(Arc::clone(&origin));
        let fuel = origin.assembly_fuel;
        Ok(OwnedCodecOriginUse { origin, spent_fuel: fuel, reported_fuel: fuel })
    }

    /// 🏗️ Admits a fresh instance's channel ([`admit_owned_channel`]: the codec decodes channel-versioned `.spr`, envelope
    /// and native bytes), then runs `codec.pack-schema-hash` for [`OWNED_CODEC_ORIGIN_SCHEMA`] on it — every owned codec
    /// export first assembles the plugin bundle — and keeps the instance it completed on. Its answer, a refusal, is read only
    /// to know the export completed as the ABI says.
    fn assemble_codec_origin(&self, owned: &OwnedCompiledGuest, budget: Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>) -> Result<OwnedCodecOrigin, TurnFault> {
        let actor = owned.artifact.instantiate()?;
        if actor.startup_active() {
            return Err(TurnFault::Host(PluginHostError::Plugin("owned actor has an undriven start function".to_string())));
        }
        let instance_id = self.next_instance_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let mut state = OwnedGuestInstance { original_job_clock:None, artifact: Arc::clone(&owned.artifact), actor, pending: None, original_receipt: None, poisoned: false, diagnostics: Vec::new(), context: 0, next_resource: 1, instance_id };
        admit_owned_channel(&mut state)?;
        let encoded = serde_json::to_vec(&OwnedCodecInput { artifact_schema: OWNED_CODEC_ORIGIN_SCHEMA, document_id: "", pack: &[], spr: &[], ops: &[] }).map_err(|error| PluginHostError::Json(error.to_string()))?;
        begin_owned_operation(&mut state, OwnedOperation::PackSchemaHash, Some(encoded))?;
        let invocation = resume_owned_operation_observed(&mut state, OwnedOperation::PackSchemaHash, budget.fuel, budget.deadline_ms, OwnedDeadline::NoFuelProgress, progress, cancellation)?;
        match decode_owned_result::<Vec<u8>>(&invocation.output) {
            Ok(_) | Err(TurnFault::Guest(_)) => {}
            Err(fault) => return Err(fault),
        }
        Ok(OwnedCodecOrigin { actor: state.actor, diagnostics: state.diagnostics, context: state.context, next_resource: state.next_resource, assembly_fuel: invocation.fuel_used })
    }

    /// 🧬️ A throwaway instance of `owned` in its codec origin's state.
    fn origin_instance(&self, owned: &OwnedCompiledGuest, origin: &OwnedCodecOrigin) -> OwnedGuestInstance {
        let instance_id = self.next_instance_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        OwnedGuestInstance { original_job_clock:None, artifact: Arc::clone(&owned.artifact), actor: origin.actor.clone(), pending: None, original_receipt: None, poisoned: false, diagnostics: origin.diagnostics.clone(), context: origin.context, next_resource: origin.next_resource, instance_id }
    }

    /// 🧬️ A throwaway instance of `compiled` in its codec origin's state, for a caller that drives its own codec operation on
    /// it (a retained SQLite snapshot call), with the fuel this call spent assembling the origin ([`Self::codec_origin`]).
    pub fn codec_instance(&self, compiled: &OwnedCompiledGuest, budget: Budget, progress: &mut impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>) -> Result<(OwnedGuestInstance, u64), TurnFault> {
        let used = self.codec_origin(compiled, budget, progress, cancellation)?;
        Ok((self.origin_instance(compiled, &used.origin), used.spent_fuel))
    }

    fn actor_context_call(&self,state:&mut OwnedGuestInstance,operation:OwnedOperation,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<original_actor_context::OriginalActorFixedCall,TurnFault>{
        if state.pending.is_none(){let encoded=identity.encode(|control|operation::input::encode_json_input(&input,control)).map_err(identity_turn_fault)?;identity.encode(|control|control.charge(encoded.len())).map_err(identity_turn_fault)?;begin_owned_operation(state,operation,Some(encoded))?;}
        let invocation=resume_owned_operation(state,operation,budget.fuel,budget.deadline_ms)?;
        original_actor_context::OriginalActorFixedReceipt::read(&invocation.output).map_err(identity_turn_fault)
    }

    /// 🎬️ Runs one actor turn over `events`, or resumes the turn `state` has in flight when `events` is empty.
    pub fn execute_actor_turn(&self, state: &mut OwnedGuestInstance, events: &[Event], budget: Budget, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TurnResult, TurnFault> {
        if state.pending.is_some() && !events.is_empty() {
            return Err(TurnFault::Trapped(format!("owned turn is mid-flight and cannot admit {} more event(s) — resume it with no events until it settles", events.len())));
        }
        let fresh=state.pending.is_none();
        if fresh{
        if state.original_receipt.is_some(){return Err(identity_turn_fault(operation_authority::refusal(4)));}
        let mut ordinary_events = identity.encode(|control|control.allocate_vec::<&Event>(events.len())).map_err(identity_turn_fault)?;
        let mut command_page = None;
        let mut cold_pair_page = None;
        for event in events {
            match event {
                Event::CommandIngressPage { cursor, bytes } if command_page.is_none() && bytes.len() <= semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES && (!bytes.is_empty() || (cursor.kind == 28 && cursor.item_count == 0)) => {
                    command_page = Some((cursor, bytes));
                }
                Event::CommandIngressPage { .. } => return Err(TurnFault::Trapped("turn carries more than one command page or an invalid page size".to_string())),
                Event::ColdDocumentPairPage(page) => {
                    if cold_pair_page.is_some() {
                        return Err(TurnFault::Trapped("turn carries duplicate cold-pair page".into()));
                    }
                    validate_cold_page(page)?;
                    cold_pair_page = Some(page);
                }
                event => ordinary_events.push(event),
            }
        }
        let input = identity.encode(|control|operation::input::encode_json_input(&OwnedPollInput { events: &ordinary_events, command_page, cold_pair_page, budget },control)).map_err(identity_turn_fault)?;
        identity.encode(|control|control.charge(input.len())).map_err(identity_turn_fault)?;
        begin_owned_operation(state, OwnedOperation::Poll, Some(input))?;
        }
        let mut receiver=if fresh{semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceiver::new(identity)}else{let receipt=state.original_receipt.take().ok_or_else(||identity_turn_fault(operation_authority::refusal(4)))?;match semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceiver::resume(receipt,identity){Ok(receiver)=>receiver,Err((error,receipt))=>{state.original_receipt=Some(receipt);return Err(identity_turn_fault(error));}}};
        let invocation = resume_owned_operation_authored(state, OwnedOperation::Poll, budget.fuel, budget.deadline_ms, OwnedDeadline::TotalWall, |_,_|{}, None, &mut receiver);
        let receipt=receiver.pause();
        state.original_receipt=if state.pending.is_some()||invocation.is_err(){Some(receipt)}else{None};
        let invocation=invocation?;
        if invocation.output.starts_with(b"SAC1") {let original=original_actor_context::OriginalActorFixedReceipt::read(&invocation.output).map_err(identity_turn_fault)?;return Err(TurnFault::ActorCall(original));}
        let mut result: TurnResult = decode_owned_result(&invocation.output)?;
        result.fuel_used = invocation.fuel_used;
        Ok(result)
    }

    /// 🛂️ Executes owned `describe` on a fresh instance of `compiled`.
    pub async fn describe(&self, compiled: &OwnedCompiledGuest, budget: Budget) -> Result<Vec<u8>, TurnFault> {
        self.describe_observed(compiled, budget, |_, _| {}).await
    }

    /// 🌱️ Runs one of the component's pure `codec` functions on a throwaway instance. This is
    /// the host half of ticket 26/09/18 slice TC3b: a headless server resolves a document kind's
    /// schema fingerprint, its canonical empty document, its pair-validation mirror and its edit
    /// apply from the component itself, so a package whose Rust codec the server does not link is
    /// still fully creatable and editable. The instance is a copy of the compiled guest's codec
    /// origin ([`OwnedCodecOrigin`]), created and dropped per call — nothing here observes or mutates
    /// live actor state, and no call sees what another did. `budget.fuel` bounds the call including
    /// the origin assembly it may run first; `progress` reports both as one count.
    fn codec_call<T: serde::de::DeserializeOwned, I: serde::Serialize>(&self, compiled: &OwnedCompiledGuest, operation: OwnedOperation, input: &I, budget: Budget, mut progress: impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>) -> Result<T, TurnFault> {
        let used = self.codec_origin(compiled, budget, &mut progress, cancellation)?;
        let state = &mut self.origin_instance(compiled, &used.origin);
        let encoded = serde_json::to_vec(input).map_err(|error| PluginHostError::Json(error.to_string()))?;
        begin_owned_operation(state, operation, Some(encoded))?;
        let invocation = resume_owned_operation_observed(state, operation, budget.fuel.saturating_sub(used.spent_fuel), budget.deadline_ms, OwnedDeadline::NoFuelProgress, |fuel, elapsed| progress(used.reported_fuel.saturating_add(fuel), elapsed), cancellation)?;
        decode_owned_result(&invocation.output)
    }

    fn codec_call_authored<T:serde::de::DeserializeOwned,I:serde::Serialize,O:FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool+?Sized>(&self,compiled:&OwnedCompiledGuest,operation:OwnedOperation,input:&I,budget:Budget,mut progress:impl FnMut(u64,std::time::Duration),cancellation:Option<&GuestCallCancellation>,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>)->Result<T,TurnFault>{
        let used=self.codec_origin(compiled,budget,&mut progress,cancellation)?;
        let state=&mut self.origin_instance(compiled,&used.origin);
        let input=identity.encode(|control|operation::input::encode_json_input(input,control)).map_err(identity_turn_fault)?;
        identity.encode(|control|control.charge(input.len())).map_err(identity_turn_fault)?;
        begin_owned_operation(state,operation,Some(input))?;
        let mut receiver=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceiver::new(identity);
        let invocation=resume_owned_operation_authored(state,operation,budget.fuel.saturating_sub(used.spent_fuel),budget.deadline_ms,OwnedDeadline::NoFuelProgress,|fuel,elapsed|progress(used.reported_fuel.saturating_add(fuel),elapsed),cancellation,&mut receiver)?;
        decode_owned_result(&invocation.output)
    }

    /// 🏛️ Resolves only the exact declared dialect's handwritten SQLite schema.
    pub async fn codec_sqlite_schema(&self, compiled: &OwnedCompiledGuest, dialect: &str, budget: Budget) -> Result<String, TurnFault> {
        self.codec_call(compiled, OwnedOperation::SqliteSchema, &OwnedSnapshotInput { dialect, encoding: "", payload: &[], limits: semio_framework::sqlite_snapshot::SqliteDatabaseLimits::default().into(), native: semio_framework_value::RetainedCloneGrant::default().into() }, budget, |_, _| {}, None)
    }

    /// 📤️ Projects native values using the component's own semantic provider under observed fuel.
    pub async fn codec_sqlite_export<O:FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool+?Sized>(&self, compiled: &OwnedCompiledGuest, dialect: &str, encoding: &str, payload: &[u8], limits: semio_framework::sqlite_snapshot::SqliteDatabaseLimits, budget: Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation, native_grant: semio_framework_value::RetainedCloneGrant, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>) -> Result<sqlite_wire::SnapshotFileResult, TurnFault> {
        self.codec_call_authored(compiled, OwnedOperation::SqliteExport, &OwnedSnapshotInput { dialect, encoding, payload, limits: limits.into(), native: native_grant.into() }, budget, progress, Some(cancellation), identity)
    }

    /// 📥️ Reconstructs native values after the component validates exact dialect and domain DDL.
    pub async fn codec_sqlite_import<O:FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool+?Sized>(&self, compiled: &OwnedCompiledGuest, dialect: &str, payload: &[u8], limits: semio_framework::sqlite_snapshot::SqliteDatabaseLimits, budget: Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation, native_grant: semio_framework_value::RetainedCloneGrant, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_,O>) -> Result<sqlite_wire::SnapshotPayloadResult, TurnFault> {
        self.codec_call_authored(compiled, OwnedOperation::SqliteImport, &OwnedSnapshotInput { dialect, encoding: "", payload, limits: limits.into(), native: native_grant.into() }, budget, progress, Some(cancellation), identity)
    }

    /// 🧬️ `codec.pack-schema-hash` — the kind's 32-byte structural snapshot fingerprint.
    pub async fn codec_pack_schema_hash(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, budget: Budget) -> Result<[u8; 32], TurnFault> {
        self.codec_pack_schema_hash_observed(compiled, artifact_schema, budget, |_, _| {}, &GuestCallCancellation::default()).await
    }

    /// 🧬️ `codec.pack-schema-hash` with the same stall-bound fuel observations as [`Self::codec_genesis_observed`].
    /// Catalog startup verifies every unlinked package through this call under
    /// `OperationContext::stall_bounded`; without guest fuel reaching that context a 30 s quiet span
    /// on a loaded machine looks like a wedged load (`trusted-catalog-load-stalled-before-it-finished`).
    pub async fn codec_pack_schema_hash_observed(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, budget: Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation) -> Result<[u8; 32], TurnFault> {
        let bytes: Vec<u8> = self.codec_call(compiled, OwnedOperation::PackSchemaHash, &OwnedCodecInput { artifact_schema, document_id: "", pack: &[], spr: &[], ops: &[] }, budget, progress, Some(cancellation))?;
        <[u8; 32]>::try_from(bytes.as_slice()).map_err(|_| TurnFault::Trapped("guest pack schema hash is not 32 bytes".to_string()))
    }

    /// 🌱️ `codec.genesis` — the canonical empty document of `artifact_schema` at `document_id`.
    pub async fn codec_genesis(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, document_id: &str, budget: Budget) -> Result<GuestDocumentPair, TurnFault> {
        self.codec_genesis_observed(compiled, artifact_schema, document_id, budget, |_, _| {}, &GuestCallCancellation::default()).await
    }

    /// 📈️ `codec.genesis` with bounded fuel-progress observations, for a caller whose own bound is a
    /// STALL bound. A hub creates a document on a detached task under
    /// `OperationContext::stall_bounded`, and the guest call is the single longest thing that task
    /// does — without the guest's fuel progress reaching it, the hub cannot tell an interpreter that
    /// is stepping from one that is wedged, and has to pick between refusing honest work and never
    /// refusing anything. The observations are the same ones `describe_observed` reports: one per
    /// 25 M fuel or per 5 s, whichever comes first.
    pub async fn codec_genesis_observed(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, document_id: &str, budget: Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation) -> Result<GuestDocumentPair, TurnFault> {
        self.codec_call(compiled, OwnedOperation::Genesis, &OwnedCodecInput { artifact_schema, document_id, pack: &[], spr: &[], ops: &[] }, budget, progress, Some(cancellation))
    }

    /// 📥️ `codec.print-mirror` — the host's pair-validation fence for an unlinked package.
    pub async fn codec_print_mirror(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, pack: &[u8], spr: &[u8], budget: Budget) -> Result<GuestDocumentMirror, TurnFault> {
        self.codec_call(compiled, OwnedOperation::PrintMirror, &OwnedCodecInput { artifact_schema, document_id: "", pack, spr, ops: &[] }, budget, |_, _| {}, None)
    }

    /// 📥️ `codec.print-mirror` with the same stall-bound fuel observations as [`Self::codec_genesis_observed`]:
    /// validating a large document's pair interprets for as long as a genesis does.
    pub async fn codec_print_mirror_observed(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, pack: &[u8], spr: &[u8], budget: Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation) -> Result<GuestDocumentMirror, TurnFault> {
        self.codec_call(compiled, OwnedOperation::PrintMirror, &OwnedCodecInput { artifact_schema, document_id: "", pack, spr, ops: &[] }, budget, progress, Some(cancellation))
    }

    /// 🧩️ `codec.apply-ops` — the host-authoritative edit apply for an unlinked package.
    pub async fn codec_apply_ops(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, pack: &[u8], spr: &[u8], ops: &[u8], budget: Budget, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<GuestDocumentPair, TurnFault> {
        self.codec_call_authored(compiled, OwnedOperation::ApplyOps, &OwnedCodecInput { artifact_schema, document_id: "", pack, spr, ops }, budget, |_, _| {}, None, identity)
    }

    /// 🧩️ `codec.apply-ops` with the same stall-bound fuel observations as [`Self::codec_genesis_observed`].
    pub async fn codec_apply_ops_observed(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, pack: &[u8], spr: &[u8], ops: &[u8], budget: Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<GuestDocumentPair, TurnFault> {
        self.codec_call_authored(compiled, OwnedOperation::ApplyOps, &OwnedCodecInput { artifact_schema, document_id: "", pack, spr, ops }, budget, progress, Some(cancellation), identity)
    }

    /// 📜️ `codec.replay-envelopes` — the hub's Check In fold, with the same stall-bound fuel
    /// observations as [`Self::codec_genesis_observed`]: a long ledger tail is the longest guest call
    /// a Check In makes, so its fuel progress is what its stall bound watches.
    pub async fn codec_replay_envelopes_observed(&self, compiled: &OwnedCompiledGuest, artifact_schema: &str, pack: &[u8], spr: &[u8], envelopes: &[u8], budget: Budget, progress: impl FnMut(u64, std::time::Duration), cancellation: &GuestCallCancellation) -> Result<GuestDocumentPair, TurnFault> {
        self.codec_call(compiled, OwnedOperation::ReplayEnvelopes, &OwnedCodecInput { artifact_schema, document_id: "", pack, spr, ops: envelopes }, budget, progress, Some(cancellation))
    }

    /// 📈️ Executes owned `describe` with bounded fuel-progress observations for build tooling.
    pub async fn describe_observed(&self, compiled: &OwnedCompiledGuest, budget: Budget, progress: impl FnMut(u64, std::time::Duration)) -> Result<Vec<u8>, TurnFault> {
        let state = &mut self.instantiate_actor(compiled).map_err(TurnFault::Host)?;
        begin_owned_operation(state, OwnedOperation::Describe, None)?;
        resume_owned_operation_observed(state, OwnedOperation::Describe, budget.fuel, budget.deadline_ms, OwnedDeadline::NoFuelProgress, progress, None).map(|invocation| invocation.output)
    }

    /// 🎟️ Admits the original actor context of `state` for one retained turn.
    pub fn admit_context(&self,state:&mut OwnedGuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<original_actor_context::OriginalActorAdmissionReply,TurnFault>{let call=self.actor_context_call(state,OwnedOperation::AdmitActorContext,input,budget,identity)?;match call.kind{original_actor_context::OriginalActorFixedKind::Admission{ready,refusal}=>Ok(original_actor_context::OriginalActorAdmissionReply{input:call.input,spent:call.spent,ready,refusal}),_=>Err(TurnFault::ActorCall(call))}}
    /// ♻️ Runs one closing step of the original actor context of `state`.
    pub fn close_context(&self,state:&mut OwnedGuestInstance,input:semio_framework::kernel::RetainedTurnInput,budget:Budget,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<original_actor_context::OriginalActorCloseReply,TurnFault>{let call=self.actor_context_call(state,OwnedOperation::CloseActorContext,input,budget,identity)?;match call.kind{original_actor_context::OriginalActorFixedKind::Close{complete,blocked,refusal}=>Ok(original_actor_context::OriginalActorCloseReply{input:call.input,spent:call.spent,complete,blocked,refusal}),_=>Err(TurnFault::ActorCall(call))}}

    /// 👷️ Starts guest job `job` of `kind` on `state`.
    pub fn start_actor_job(&self, state: &mut OwnedGuestInstance, job: u64, kind: &str, input: Vec<u8>) -> Result<(), TurnFault> {
        let input = semio_framework_pack_json::to_json_string(&OwnedStartJobInput { job, kind, input }).into_bytes();
        begin_owned_operation(state, OwnedOperation::StartJob, Some(input))?;
        let invocation = resume_owned_operation(state, OwnedOperation::StartJob, u64::MAX, 1_000)?;
        decode_owned_result(&invocation.output)
    }

    /// 🪜️ Steps guest job `job` of `state` once under `budget`.
    pub fn step_actor_job(&self, state: &mut OwnedGuestInstance, job: u64, budget: JobBudget) -> Result<JobStep, TurnFault> {
        let input = semio_framework_pack_json::to_json_string(&OwnedStepJobInput { job, budget }).into_bytes();
        begin_owned_operation(state, OwnedOperation::StepJob, Some(input))?;
        let invocation = resume_owned_operation(state, OwnedOperation::StepJob, budget.fuel, budget.deadline_ms)?;
        decode_owned_result(&invocation.output)
    }

    /// 🛑️ Cancels guest job `job` of `state`, abandoning whatever owned operation is in flight first.
    pub fn cancel_actor_job(&self, state: &mut OwnedGuestInstance, job: u64) -> Result<(), TurnFault> {
        if state.pending.as_ref().is_some_and(|pending| pending.operation != OwnedOperation::CancelJob) {
            cancel_owned_operation(state)?;
        }
        let input = semio_framework_pack_json::to_json_string(&OwnedCancelJobInput { job }).into_bytes();
        begin_owned_operation(state, OwnedOperation::CancelJob, Some(input))?;
        let invocation = resume_owned_operation(state, OwnedOperation::CancelJob, u64::MAX, OWNED_SLICE_DEADLINE_MS)?;
        decode_owned_result(&invocation.output)
    }

    /// 📸️ Frames `state` — its in-flight operation, original receipt, guest checkpoint and interpreter memory — as one owned
    /// host checkpoint.
    pub fn checkpoint_actor(&self, state: &mut OwnedGuestInstance, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<u8>, PluginHostError> {
        let guest_checkpoint = if state.pending.is_none() {
            begin_owned_operation(state, OwnedOperation::Checkpoint, None).map_err(turn_fault_host)?;
            let invocation = match resume_owned_operation(state, OwnedOperation::Checkpoint, 100_000_000, 1_000) {
                Ok(invocation) => invocation,
                Err(fault) => {
                    cancel_owned_operation(state).map_err(turn_fault_host)?;
                    return Err(turn_fault_host(fault));
                }
            };
            Some(decode_owned_result::<Vec<u8>>(&invocation.output).map_err(turn_fault_host)?)
        } else {
            None
        };
        let metadata = OwnedCheckpointView { pending: &state.pending, original_receipt:&state.original_receipt, guest_checkpoint:&guest_checkpoint, context: state.context, next_resource: state.next_resource, instance_id: state.instance_id };
        identity.encode(|control|{
            let layout=state.actor.checkpoint_layout(control)?;
            operation::checkpoint::encode_checkpoint_frame(&metadata,&mut|sink|layout.visit(sink),control)
        }).map_err(identity_turn_fault).map_err(turn_fault_host)
    }

    /// ⏮️ Restores `state` from an owned host checkpoint [`Self::checkpoint_actor`] framed.
    pub fn restore_actor(&self, state: &mut OwnedGuestInstance, checkpoint: &[u8]) -> Result<(), PluginHostError> {
        if checkpoint.get(..8) != Some(OWNED_CHECKPOINT_MAGIC) {
            return Err(PluginHostError::Plugin("invalid owned host checkpoint header".to_string()));
        }
        let metadata_length =
            checkpoint.get(8..12).and_then(|bytes| bytes.try_into().ok()).map(u32::from_le_bytes).map(|length| length as usize).ok_or_else(|| PluginHostError::Plugin("truncated owned host checkpoint metadata length".to_string()))?;
        let actor_length =
            checkpoint.get(12..20).and_then(|bytes| bytes.try_into().ok()).map(u64::from_le_bytes).and_then(|length| usize::try_from(length).ok()).ok_or_else(|| PluginHostError::Plugin("invalid owned host checkpoint actor length".to_string()))?;
        let metadata_end = 20usize.checked_add(metadata_length).ok_or_else(|| PluginHostError::Plugin("owned host checkpoint metadata length overflow".to_string()))?;
        let actor_end = metadata_end.checked_add(actor_length).ok_or_else(|| PluginHostError::Plugin("owned host checkpoint actor length overflow".to_string()))?;
        if actor_end != checkpoint.len() {
            return Err(PluginHostError::Plugin("owned host checkpoint length mismatch".to_string()));
        }
        let metadata_text = std::str::from_utf8(&checkpoint[20..metadata_end]).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let metadata: OwnedCheckpointMetadata = semio_framework_pack_json::from_json_str(metadata_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginHostError::Json(error.to_string()))?;
        state.actor = state.artifact.restore(&checkpoint[metadata_end..]).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        state.pending = metadata.pending;
        state.original_receipt=metadata.original_receipt;
        state.context = metadata.context;
        state.next_resource = metadata.next_resource;
        state.instance_id = metadata.instance_id;
        if let Some(guest_checkpoint) = metadata.guest_checkpoint {
            let input = semio_framework_pack_json::to_json_string(&OwnedRestoreInput { state: guest_checkpoint }).into_bytes();
            begin_owned_operation(state, OwnedOperation::Restore, Some(input)).map_err(turn_fault_host)?;
            let invocation = resume_owned_operation(state, OwnedOperation::Restore, 100_000_000, 1_000).map_err(turn_fault_host)?;
            decode_owned_result::<()>(&invocation.output).map_err(turn_fault_host)?;
        }
        Ok(())
    }
}

fn begin_owned_operation(state: &mut OwnedGuestInstance, operation: OwnedOperation, input: Option<Vec<u8>>) -> Result<(), TurnFault> {
    if state.poisoned {
        return Err(TurnFault::Trapped("owned actor instance is poisoned by an earlier guest trap and must be re-instantiated".to_string()));
    }
    if let Some(pending) = &state.pending {
        return if pending.operation == operation { Ok(()) } else { Err(TurnFault::Trapped(format!("owned operation {:?} is still active", pending.operation))) };
    }
    let stage = match input {
        Some(input) => {
            state.actor.begin(OwnedSemioExport::Allocate, vec![Value::I32(i32::try_from(input.len()).map_err(|_| TurnFault::Trapped("owned input exceeds wasm32 address space".to_string()))?)])?;
            OwnedStage::Allocate { input }
        }
        None => {
            state.actor.begin(operation.export(), Vec::new())?;
            OwnedStage::Call
        }
    };
    state.pending = Some(OwnedPending { retained_input:None, operation, stage, fuel_used: 0 });
    Ok(())
}

/// ⏱️ What `deadline_ms` bounds, chosen per call site because the two callers want opposite things.
///
/// A live turn owes the shell a frame, so its bound is TOTAL wall time: a turn that overruns is late
/// whether or not the guest is healthy, and `Poll` therefore keeps [`Self::TotalWall`].
///
/// A build-time `describe()` and a `codec` call owe nobody a frame, and their runaway bound is the
/// deterministic fuel cap (`DESCRIBE_FUEL_BUDGET`; for `codec` the caller's own `budget.fuel`). Measured on 2026-09-21 (slice CE2): the SAME `🗒️note` guest ran its
/// describe at 1 320 k fuel/s at 20:36 and at 520 k fuel/s at 22:10 — a 2.5× swing from fleet load
/// alone, with nothing about the guest changed. Under a saturated fleet the owned interpreter
/// sustains ≈ 210 k fuel/s, so a 1 800 000 ms TOTAL cap buys only ≈ 380 M of the 8 G fuel budget and
/// the WALL clock, not the guest, decides whether a build succeeds — which is how `🀄️wfc` (373 M
/// fuel, 4.6 % of its budget, a flat rate and no degradation) and `🧩️puzzle` both died
/// `DeadlineExceeded` while still progressing normally.
///
/// [`Self::NoFuelProgress`] bounds the gap BETWEEN fuel observations instead, so a describe that
/// keeps making progress runs to its fuel cap on any machine while a guest that stops making
/// progress still dies on a finite wall clock. Nothing is lost by this: the loop already traps a
/// zero-fuel yield outright, and a host call that never returns blocks inside `reply_owned_host`
/// where no deadline check can run — so a total-wall cap here never caught a runaway the fuel cap
/// would have missed.
///
/// 🌱️ `codec` joined `describe` on 2026-09-22 (ticket 26/09/18 slice HC1) for the same reason, one
/// layer further out: a hub arms `GUEST_CODEC_BUDGET` (4 G fuel, 30 000 ms) and creates the
/// document on a DETACHED task no socket is waiting on, so nothing there owes a frame either. Under
/// a total-wall bound the biggest staged component — `🌍️gis`, ≈ 48 MB of `wasm-release` — could not
/// be created at all: hub 7671 failed every `POST …/artifact-creations` in 32.0 s at machine load
/// 120, 33 and 21 alike, with the hub process burning 130–145 % CPU for the whole window
/// (`🗑️generated/hc1-create-sampled-1.txt`). A guest running flat out and cut by the calendar is
/// precisely what this enum exists to stop calling a fault.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum OwnedDeadline {
    TotalWall,
    NoFuelProgress,
}

/// 🛂️ A refused original native admission, as the host fault every runtime reports it as.
pub fn identity_turn_fault(error:semio_framework_value::ValueError)->TurnFault{TurnFault::Host(PluginHostError::NativeIo(error))}

fn resume_owned_operation(state: &mut OwnedGuestInstance, operation: OwnedOperation, fuel: u64, deadline_ms: u32) -> Result<OwnedInvocation, TurnFault> {
    resume_owned_operation_observed(state, operation, fuel, deadline_ms, OwnedDeadline::TotalWall, |_, _| {}, None)
}

fn resume_owned_operation_observed(state: &mut OwnedGuestInstance, operation: OwnedOperation, fuel: u64, deadline_ms: u32, deadline: OwnedDeadline, progress: impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>) -> Result<OwnedInvocation, TurnFault> {
    resume_owned_operation_bound(state, operation, fuel, deadline_ms, deadline, progress, cancellation, None)
}

fn resume_owned_operation_authored(state: &mut OwnedGuestInstance, operation: OwnedOperation, fuel: u64, deadline_ms: u32, deadline: OwnedDeadline, progress: impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>, receiver: &mut dyn OriginalOperationReceiving) -> Result<OwnedInvocation, TurnFault> {
    let result=resume_owned_operation_bound(state, operation, fuel, deadline_ms, deadline, progress, cancellation, Some(receiver));
    result.and_then(|output|{receiver.ensure_finished().map_err(identity_turn_fault)?;Ok(output)})
}

fn resume_owned_operation_bound(state: &mut OwnedGuestInstance, operation: OwnedOperation, fuel: u64, deadline_ms: u32, deadline: OwnedDeadline, mut progress: impl FnMut(u64, std::time::Duration), cancellation: Option<&GuestCallCancellation>, mut receiver: Option<&mut dyn OriginalOperationReceiving>) -> Result<OwnedInvocation, TurnFault> {
    let started = std::time::Instant::now();
    let mut fuel_moved_at = started;
    let mut remaining = fuel;
    let mut next_progress_fuel = 25_000_000;
    let mut next_progress_elapsed = std::time::Duration::from_secs(5);
    loop {
        let elapsed = started.elapsed();
        if cancellation.is_some_and(GuestCallCancellation::is_cancelled) {
            progress(fuel.saturating_sub(remaining), elapsed);
            return Err(TurnFault::Cancelled);
        }
        let against_deadline = match deadline {
            OwnedDeadline::TotalWall => elapsed,
            OwnedDeadline::NoFuelProgress => fuel_moved_at.elapsed(),
        };
        if against_deadline >= std::time::Duration::from_millis(u64::from(deadline_ms)) {
            progress(fuel.saturating_sub(remaining), elapsed);
            return Err(TurnFault::DeadlineExceeded);
        }
        if remaining == 0 {
            progress(fuel, elapsed);
            return Err(TurnFault::FuelExhausted);
        }
        let mut pending = state.pending.take().ok_or_else(|| TurnFault::Trapped("owned operation has no resumable state".to_string()))?;
        if pending.operation != operation {
            state.pending = Some(pending);
            return Err(TurnFault::Trapped("owned operation resume type mismatch".to_string()));
        }
        let grant = remaining.min(OWNED_STEP_FUEL);
        let remaining_before = remaining;
        let completed_fuel;
        match state.actor.step(grant, StepControl::default()) {
            CoreStepOutcome::Yield { fuel_used } => {
                pending.fuel_used = pending.fuel_used.saturating_add(fuel_used);
                remaining = remaining.saturating_sub(fuel_used);
                completed_fuel = pending.fuel_used;
                state.pending = Some(pending);
                if fuel_used == 0 {
                    return Err(TurnFault::Trapped("owned interpreter yielded without consuming fuel".to_string()));
                }
            }
            CoreStepOutcome::HostCall { fuel_used, call } => {
                pending.fuel_used = pending.fuel_used.saturating_add(fuel_used);
                remaining = remaining.saturating_sub(fuel_used);
                completed_fuel = pending.fuel_used;
                let result=reply_owned_host(state,&call,receiver.as_deref_mut()).map_err(|error|TurnFault::Trapped(error.to_string())).and_then(|results|state.actor.resume_host(call.id,Ok(results)).map_err(TurnFault::from));
                state.pending=Some(pending);
                result?;
            }
            CoreStepOutcome::Complete { fuel_used, values } => {
                pending.fuel_used = pending.fuel_used.saturating_add(fuel_used);
                remaining = remaining.saturating_sub(fuel_used);
                completed_fuel=pending.fuel_used;
                let result=(||->Result<Option<OwnedInvocation>,TurnFault>{match &mut pending.stage {
                    OwnedStage::Allocate { input } => {
                        let [Value::I32(pointer)] = values.as_slice() else { return Err(TurnFault::Trapped("owned allocator returned an invalid pointer".to_string())) };
                        write_owned_memory(&mut state.actor,*pointer,input)?;
                        let length=input.len()as i32;
                        if matches!(operation,OwnedOperation::SqliteExport|OwnedOperation::SqliteImport|OwnedOperation::SqliteRetirement|OwnedOperation::SqliteClose|OwnedOperation::SqliteTakeFile|OwnedOperation::SqliteTakePayload){pending.retained_input=Some(std::mem::take(input));}
                        state.actor.begin(operation.export(),vec![Value::I32(*pointer),Value::I32(length)])?;
                        pending.stage=OwnedStage::Call;
                        Ok(None)
                    }
                    OwnedStage::Call => {
                        let [Value::I64(pair)] = values.as_slice() else { return Err(TurnFault::Trapped("owned call returned an invalid pointer/length pair".to_string())) };
                        let pair=*pair as u64;
                        match operation_authority::owned_result::decode(pair).map_err(identity_turn_fault)?{operation_authority::owned_result::OwnedReturn::Bytes{..}=>{},operation_authority::owned_result::OwnedReturn::Refusal(kind)=>return Err(identity_turn_fault(semio_framework_value::ValueError::literal(kind,"original guest operation returned scalar refusal"))),}
                        if let Some(receiver)=receiver.as_deref_mut(){receiver.receive_output((pair>>32)as u32 as usize).map_err(identity_turn_fault)?;}
                        let output=state.actor.read_bytes_result(&values,OWNED_RESULT_LIMIT)?;
                        pending.stage=OwnedStage::Deallocate{output};
                        state.actor.begin(OwnedSemioExport::Deallocate,vec![Value::I32(pair as u32 as i32),Value::I32((pair>>32)as u32 as i32)])?;
                        Ok(None)
                    }
                    OwnedStage::Deallocate { output } => Ok(Some(OwnedInvocation{retained_input:pending.retained_input.take(),output:std::mem::take(output),fuel_used:pending.fuel_used})),
                }})();
                match result{Ok(Some(invocation))=>{progress(invocation.fuel_used,started.elapsed());return Ok(invocation)},Ok(None)=>state.pending=Some(pending),Err(error)=>{state.pending=Some(pending);return Err(error)}}
            }
            CoreStepOutcome::Cancelled { fuel_used } => {
                pending.fuel_used=pending.fuel_used.saturating_add(fuel_used);
                progress(pending.fuel_used,started.elapsed());
                state.pending=Some(pending);
                return Err(TurnFault::Trapped(format!("owned operation cancelled after {fuel_used} instructions")));
            }
            CoreStepOutcome::Fault { fuel_used, error } => {
                pending.fuel_used=pending.fuel_used.saturating_add(fuel_used);
                progress(pending.fuel_used,started.elapsed());
                state.pending=Some(pending);
                state.poisoned=true;
                return Err(TurnFault::Trapped(owned_fault_text(state,&error.to_string())));
            }
        }
        if remaining < remaining_before {
            fuel_moved_at = std::time::Instant::now();
        }
        let elapsed = started.elapsed();
        if completed_fuel >= next_progress_fuel || elapsed >= next_progress_elapsed {
            progress(completed_fuel, elapsed);
            next_progress_fuel = completed_fuel.saturating_add(25_000_000);
            next_progress_elapsed = elapsed.saturating_add(std::time::Duration::from_secs(5));
        }
    }
}

/// 🩺️ A guest fault, spoken in the guest's own words. `unreachable executed` from a
/// `panic = "abort"` component IS a Rust panic, and the panic message it printed a moment earlier is
/// the only thing that names the failing frame — so the retained tail of guest stderr travels with
/// the trap instead of being thrown away with the instance.
fn owned_fault_text(state: &OwnedGuestInstance, error: &str) -> String {
    let printed = String::from_utf8_lossy(&state.diagnostics);
    let printed = printed.trim();
    if printed.is_empty() {
        return error.to_string();
    }
    let tail = printed.char_indices().rev().nth(OWNED_FAULT_DIAGNOSTICS_TAIL_CHARS).map_or(printed, |(index, _)| &printed[index..]);
    format!("{error} — the guest printed: {tail}")
}

/// 🩺️ How much of the guest's own output a trap carries. A panic line plus its location is well
/// under this; the cap keeps a guest that logged a megabyte from burying the fault it ends with.
const OWNED_FAULT_DIAGNOSTICS_TAIL_CHARS: usize = 4_000;

fn cancel_owned_operation(state: &mut OwnedGuestInstance) -> Result<(), TurnFault> {
    match state.actor.step(1, StepControl { cancelled: true }) {
        CoreStepOutcome::HostCall { call, .. } => {
            state.actor.resume_host(call.id, Err("owned operation cancelled".to_string()))?;
            if !matches!(state.actor.step(1, StepControl { cancelled: true }), CoreStepOutcome::Cancelled { .. }) {
                return Err(TurnFault::Trapped("owned interpreter did not acknowledge cancellation".to_string()));
            }
        }
        CoreStepOutcome::Cancelled { .. } => {}
        _ => return Err(TurnFault::Trapped("owned interpreter did not acknowledge cancellation".to_string())),
    }
    state.pending = None;
    state.original_receipt=None;
    Ok(())
}

fn owned_wasi_nanoseconds(duration: std::time::Duration) -> Result<u64, PluginHostError> {
    u64::try_from(duration.as_nanos()).map_err(|_| PluginHostError::Plugin("owned WASI monotonic clock exhausted its unsigned nanosecond range".to_string()))
}

fn owned_wasi_monotonic_clock() -> Result<Value, PluginHostError> {
    static ORIGIN: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    let nanoseconds = owned_wasi_nanoseconds(ORIGIN.get_or_init(std::time::Instant::now).elapsed())?;
    Ok(Value::I64(i64::from_ne_bytes(nanoseconds.to_ne_bytes())))
}

/// 🕰️ `wasi:clocks/wall-clock.now`'s `datetime` record (`seconds: u64` at 0, `nanoseconds: u32` at 8,
/// padded to 16), read from the host's real wall clock — a guest Store stamps history with it.
fn owned_wasi_wall_clock() -> Result<[u8; 16], PluginHostError> {
    let since_epoch = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|_| PluginHostError::Plugin("owned WASI wall clock reads before the Unix epoch".to_string()))?;
    let mut datetime = [0u8; 16];
    datetime[..8].copy_from_slice(&since_epoch.as_secs().to_le_bytes());
    datetime[8..12].copy_from_slice(&since_epoch.subsec_nanos().to_le_bytes());
    Ok(datetime)
}

/// 🎲️ One platform-entropy word for `wasi:random` — a guest Store's replica identity comes from it.
fn owned_wasi_entropy() -> Result<u64, PluginHostError> {
    semio_framework_os_kernel::os_identity::entropy_u64().map_err(|error| PluginHostError::Plugin(format!("owned WASI random: {error}")))
}

fn reply_owned_host<'loan,'source>(state: &mut OwnedGuestInstance, call: &HostCall, receiver: Option<&'loan mut (dyn OriginalOperationReceiving+'source)>) -> Result<Vec<Value>, PluginHostError> {
    let values = match (call.module.as_str(), call.name.as_str()) {
        ("semio:framework/pure@1.0.0", "operation-begin") => {
            let outcome = receiver.ok_or_else(|| operation_authority::refusal(6)).and_then(|receiver| receiver.begin());
            let mut result = [0u8;16];
            match outcome { Ok(maximum) => result[8..].copy_from_slice(&(maximum as u64).to_le_bytes()), Err(error) => { result[..4].copy_from_slice(&1u32.to_le_bytes()); result[8..12].copy_from_slice(&operation_authority::refusal_code(error.kind).to_le_bytes()); } }
            write_owned_memory(&mut state.actor, owned_argument_i32(call, 0)?, &result).map_err(turn_fault_host)?;
            Vec::new()
        }
        ("semio:framework/pure@1.0.0", "operation-progress" | "operation-allocation" | "operation-finish") => {
            let outcome = receiver.ok_or_else(|| operation_authority::refusal(6)).and_then(|receiver| {
                let number = |index:usize| match call.arguments.get(index) { Some(Value::I64(value)) => usize::try_from(u64::from_ne_bytes(value.to_ne_bytes())).map_err(|_|operation_authority::refusal(2)), _ => Err(operation_authority::refusal(4)) };
                match call.name.as_str() {
                    "operation-progress" => receiver.progress(semio_framework_value::native_encoding::NativeEncodeProgress{completed:number(0)?,total:number(1)?,owned_bytes:number(2)?}),
                    "operation-allocation" => receiver.allocation(semio_framework_value::native_encoding::NativeEncodeAllocation{bytes:number(0)?,owned_bytes:number(1)?,next_owned_bytes:number(2)?,maximum_bytes:number(3)?}),
                    "operation-finish" => receiver.finish(number(0)?),
                    _ => Err(operation_authority::refusal(4)),
                }
            });
            vec![Value::I32(outcome.map_or_else(|error|operation_authority::refusal_code(error.kind) as i32,|()|0))]
        }
        ("semio:framework/pure@1.0.0", "operation-clock-us") => {
            let mut result=[0u8;16];if let Some(now)=state.original_job_clock.and_then(|clock|clock()){result[..4].copy_from_slice(&1u32.to_le_bytes());result[8..].copy_from_slice(&now.to_le_bytes());}
            write_owned_memory(&mut state.actor,owned_argument_i32(call,0)?,&result).map_err(turn_fault_host)?;Vec::new()
        }
        ("semio:framework/pure@1.0.0", "now-ms") => vec![Value::I64(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as i64))],
        ("semio:framework/pure@1.0.0", "log") | ("semio:framework/pure@1.0.0", "trace-span") => Vec::new(),
        ("$root", "[context-get-0]") => vec![Value::I32(state.context)],
        ("$root", "[context-set-0]") => {
            let [Value::I32(context)] = call.arguments.as_slice() else { return Err(PluginHostError::Plugin("invalid owned context-set argument".to_string())) };
            state.context = *context;
            Vec::new()
        }
        ("$root", "[waitable-set-new]")
        | ("wasi:io/streams@0.2.0", "[method]output-stream.subscribe")
        | ("wasi:clocks/monotonic-clock@0.2.0", "subscribe-duration")
        | ("wasi:cli/stdin@0.2.0", "get-stdin")
        | ("wasi:cli/stdout@0.2.0", "get-stdout")
        | ("wasi:cli/stderr@0.2.0", "get-stderr") => {
            let resource = state.next_resource;
            state.next_resource = state.next_resource.wrapping_add(1).max(1);
            vec![Value::I32(resource)]
        }
        ("$root", "[waitable-set-poll]") => vec![Value::I32(0)],
        ("$root", "[waitable-join]")
        | ("$root", "[waitable-set-drop]")
        | ("[export]$root", "[task-cancel]")
        | ("wasi:io/poll@0.2.0", "[method]pollable.block")
        | ("wasi:io/error@0.2.0", "[resource-drop]error")
        | ("wasi:io/poll@0.2.0", "[resource-drop]pollable")
        | ("wasi:io/streams@0.2.0", "[resource-drop]input-stream")
        | ("wasi:io/streams@0.2.0", "[resource-drop]output-stream") => Vec::new(),
        ("wasi:clocks/monotonic-clock@0.2.0", "now") => vec![owned_wasi_monotonic_clock()?],
        ("wasi:random/insecure-seed@0.2.9", "insecure-seed") => {
            let seed = [owned_wasi_entropy()?.to_le_bytes(), owned_wasi_entropy()?.to_le_bytes()].concat();
            write_owned_memory(&mut state.actor, owned_argument_i32(call, 0)?, &seed).map_err(turn_fault_host)?;
            Vec::new()
        }
        ("wasi:random/random@0.2.9", "get-random-u64") => vec![Value::I64(i64::from_le_bytes(owned_wasi_entropy()?.to_le_bytes()))],
        ("wasi:cli/environment@0.2.0", "get-environment") => {
            write_owned_zeroes(&mut state.actor, owned_argument_i32(call, 0)?, 8)?;
            Vec::new()
        }
        ("wasi:clocks/wall-clock@0.2.0", "now") => {
            write_owned_memory(&mut state.actor, owned_argument_i32(call, 0)?, &owned_wasi_wall_clock()?).map_err(turn_fault_host)?;
            Vec::new()
        }
        ("wasi:cli/terminal-stdin@0.2.0", "get-terminal-stdin") | ("wasi:cli/terminal-stdout@0.2.0", "get-terminal-stdout") | ("wasi:cli/terminal-stderr@0.2.0", "get-terminal-stderr") => {
            write_owned_zeroes(&mut state.actor, owned_argument_i32(call, 0)?, 8)?;
            Vec::new()
        }
        ("wasi:io/poll@0.2.0", "poll") => {
            write_owned_zeroes(&mut state.actor, owned_argument_i32(call, 2)?, 8)?;
            Vec::new()
        }
        ("wasi:io/streams@0.2.0", "[method]output-stream.check-write") => {
            let pointer = owned_argument_i32(call, 1)?;
            write_owned_zeroes(&mut state.actor, pointer, 16)?;
            write_owned_memory(&mut state.actor, pointer.wrapping_add(8), &65_536u64.to_le_bytes()).map_err(turn_fault_host)?;
            Vec::new()
        }
        ("wasi:io/streams@0.2.0", "[method]output-stream.write") => {
            let pointer = owned_argument_i32(call, 1)?;
            let length = owned_argument_i32(call, 2)?;
            retain_owned_guest_diagnostics(state, pointer, length)?;
            write_owned_zeroes(&mut state.actor, owned_argument_i32(call, 3)?, 16)?;
            Vec::new()
        }
        ("wasi:io/streams@0.2.0", "[method]output-stream.blocking-flush") => {
            write_owned_zeroes(&mut state.actor, owned_argument_i32(call, 1)?, 16)?;
            Vec::new()
        }
        _ => return Err(PluginHostError::Plugin(format!("owned host import {}::{} is unavailable", call.module, call.name))),
    };
    if values.len() != call.results.len() || values.iter().zip(&call.results).any(|(value, expected)| value.value_type() != *expected) {
        return Err(PluginHostError::Plugin(format!("owned host import {}::{} returned the wrong shape", call.module, call.name)));
    }
    Ok(values)
}

fn owned_argument_i32(call: &HostCall, index: usize) -> Result<i32, PluginHostError> {
    match call.arguments.get(index) {
        Some(Value::I32(value)) => Ok(*value),
        _ => Err(PluginHostError::Plugin(format!("owned host import {}::{} argument {index} is not i32", call.module, call.name))),
    }
}

fn write_owned_zeroes(actor: &mut OwnedSemioInstance, pointer: i32, length: usize) -> Result<(), PluginHostError> {
    write_owned_memory(actor, pointer, &vec![0; length]).map_err(turn_fault_host)
}

/// 🩺️ Retains what the owned guest writes to `wasi:cli/stdout`/`stderr`, bounded by
/// [`GUEST_DIAGNOSTICS_CAPACITY_BYTES`]. The owned shim used to answer `output-stream.write` with a
/// zeroed result and DISCARD the bytes, so a guest panic — which `panic = "abort"` prints to stderr
/// immediately before its `unreachable` — reached the host as the bare text `wasm trap: unreachable
/// executed` and nothing else. That is what made ticket 26/09/18 slice TC3c's `codec.genesis` trap
/// undiagnosable from the outside: the guest had already said exactly what was wrong. The wasmtime
/// path has kept this since [`guest_wasi_ctx`]; this is the owned interpreter's half of it.
fn retain_owned_guest_diagnostics(state: &mut OwnedGuestInstance, pointer: i32, length: i32) -> Result<(), PluginHostError> {
    let start = pointer as u32 as usize;
    let length = length as u32 as usize;
    let memory = state.actor.memory_mut().ok_or_else(|| PluginHostError::Plugin("owned actor memory is unavailable".to_string()))?;
    let end = start.checked_add(length).filter(|end| *end <= memory.len()).ok_or_else(|| PluginHostError::Plugin("owned guest diagnostics are outside guest memory".to_string()))?;
    let budget = GUEST_DIAGNOSTICS_CAPACITY_BYTES.saturating_sub(state.diagnostics.len());
    let bytes = &memory[start..end.min(start.saturating_add(budget))];
    state.diagnostics.extend_from_slice(bytes);
    Ok(())
}

fn write_owned_memory(actor: &mut OwnedSemioInstance, pointer: i32, bytes: &[u8]) -> Result<(), TurnFault> {
    let start = pointer as u32 as usize;
    let memory = actor.memory_mut().ok_or_else(|| TurnFault::Trapped("owned actor memory is unavailable".to_string()))?;
    let end = start.checked_add(bytes.len()).filter(|end| *end <= memory.len()).ok_or_else(|| TurnFault::Trapped("owned input is outside guest memory".to_string()))?;
    memory[start..end].copy_from_slice(bytes);
    Ok(())
}

/// 🤝️ Fuel and wall bound of the owned `semio_owned_channel_version_v1` read at instantiation: one JSON integer, never a turn.
const OWNED_CHANNEL_VERSION_FUEL: u64 = 1_000_000;
const OWNED_CHANNEL_VERSION_DEADLINE_MS: u32 = 5_000;

/// 🔖️ Reads a fresh owned instance's `semio_owned_channel_version_v1` once and admits it through the one kernel admission
/// ([`semio_framework_os_kernel::admit_guest_channel_version`]) before the instance serves anything — an actor turn or a codec call —
/// so a guest of another channel is refused as `PluginHostError::Refused` (`plugin.channel-mismatch`).
fn admit_owned_channel(state: &mut OwnedGuestInstance) -> Result<(), TurnFault> {
    begin_owned_operation(state, OwnedOperation::ChannelVersion, None)?;
    let invocation = resume_owned_operation(state, OwnedOperation::ChannelVersion, OWNED_CHANNEL_VERSION_FUEL, OWNED_CHANNEL_VERSION_DEADLINE_MS)?;
    let guest: u32 = serde_json::from_slice(&invocation.output).map_err(|error| PluginHostError::Json(error.to_string()))?;
    semio_framework_os_kernel::admit_guest_channel_version(guest, semio_framework_os_kernel::CHANNEL_VERSION).map_err(|fault| PluginHostError::Refused(Box::new(fault)))?;
    Ok(())
}

/// 📥️ Decodes an owned export's JSON `Result` envelope; its error arm is the guest's typed fault bytes.
pub fn decode_owned_result<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, TurnFault> {
    let result: Result<T, Vec<u8>> = serde_json::from_slice(bytes).map_err(|error| PluginHostError::Json(error.to_string()))?;
    result.map_err(|error| decode_guest_fault_bytes(&error))
}

fn turn_fault_host(fault: TurnFault) -> PluginHostError {
    match fault {
        TurnFault::Host(error) => error,
        other => PluginHostError::Plugin(other.to_string()),
    }
}

impl From<crate::interpreter::CoreError> for TurnFault {
    fn from(error: crate::interpreter::CoreError) -> Self {
        Self::Trapped(error.to_string())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️owned-runtime/🦀️.rs"]
mod owned_runtime_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️microsecond-clock/🦀️.rs"]
mod microsecond_clock_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️owned-instance-open/🦀️.rs"]
mod owned_instance_open_tests;

//#endregion 🧠️OwnedRuntime
