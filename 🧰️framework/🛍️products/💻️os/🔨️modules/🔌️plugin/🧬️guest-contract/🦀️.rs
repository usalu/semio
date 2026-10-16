//! 🧬️ `semio-framework-plugin-guest-contract`: the vocabulary every plugin guest runtime shares — the owned interpreter's
//! runtime and the Wasmtime component host alike. Guest call faults ([`TurnFault`], [`PluginHostError`]), job budgets and
//! steps, codec document pairs, caller cancellation, the original operation authority wire, the original actor context and
//! the SQLite snapshot wire. It links no engine.
//! @see https://component-model.bytecodealliance.org/design/wit.html
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub use semio_framework_plugin_abi::{operation_authority, return_walk_template, sqlite_wire};
#[path = "🎟️original-actor-context/🦀️.rs"]
pub mod original_actor_context;

/// 🧮️ The heap witness the crate's allocation laws read through `semio_framework_trace::observe_heap_allocations_on_this_thread`.
#[cfg(test)]
#[global_allocator]
static GUEST_CONTRACT_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

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
    OriginalTurn(semio_framework_job::OriginalTurnFailure),
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
            Self::OriginalTurn(error)=>formatter.write_str(error.primary.message()),
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
/// (`OWNED_STEP_FUEL` instructions of the owned runtime): a caller that is gone — a cancelled creation, the hub's background catalog
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

/// 🛂️ A refused original native admission, as the host fault every runtime reports it as.
pub fn identity_turn_fault(error:semio_framework_value::ValueError)->TurnFault{TurnFault::Host(PluginHostError::NativeIo(error))}
