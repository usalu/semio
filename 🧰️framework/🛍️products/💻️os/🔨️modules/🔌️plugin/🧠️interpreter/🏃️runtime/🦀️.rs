//! 🏃️ `semio-framework-plugin-interpreter-runtime`: the owned interpreter as a guest runtime ([`OwnedRuntime`]). It compiles a
//! `world actor` component for the repository-owned interpreter (`semio-framework-plugin-interpreter`), runs its actor turns,
//! jobs, checkpoints and pure `codec` exports under instruction fuel, and answers every host import a guest can observe, in
//! the vocabulary of `semio-framework-plugin-guest-contract`. A tool that only describes or verifies components links this and
//! no JIT.
//! @see https://webassembly.github.io/spec/core/exec/index.html
use semio_framework_plugin_guest_contract::{decode_guest_fault_bytes, identity_turn_fault, operation_authority, original_actor_context, sqlite_wire, validate_cold_page, GuestCallCancellation, GuestDocumentMirror, GuestDocumentPair, JobBudget, JobStep, PluginHostError, TurnFault};
use semio_framework_plugin_interpreter::{CoreError, CoreStepOutcome, HostCall, OwnedSemioArtifact, OwnedSemioExport, OwnedSemioInstance, StepControl, Value};
use semio_framework::kernel::{Budget, Event, TurnResult};
use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceiving;
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::{Arc, Mutex};

/// 🪪️ The authored source of the owned runtime: the root and every child module, one text that the engine identity digests.
const OWNED_RUNTIME_SOURCE: &str = concat!(
    include_str!("🦀️.rs"),
    include_str!("🧮️owned-constants-and-codec-origin/🦀️.rs"),
    include_str!("🛰️owned-operation-and-guest-instance/🦀️.rs"),
    include_str!("📥️snapshot-operation-and-input-records/🦀️.rs"),
    include_str!("🔄️owned-runtime/🦀️.rs"),
    include_str!("⏯️operation-begin-resume-and-cancel/🦀️.rs"),
    include_str!("📣️host-replies-wasi-and-channel-faults/🦀️.rs"),
);

/// 🛂️ The neutral test caller admission the plugin host's runtime laws share; this crate's laws drive part of it.
#[cfg(test)]
pub(crate) use semio_framework_plugin_host_operation::test_native_authority;

#[cfg(any(test, feature = "testkit"))]
#[path = "🧪️tests/🧫️fixtures/🌱️channel/🦀️.rs"]
pub mod channel_fixture;

/// 🧮️ The heap witness the crate's allocation laws read through `semio_framework_trace::observe_heap_allocations_on_this_thread`.
#[cfg(test)]
#[global_allocator]
static OWNED_RUNTIME_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[path = "🎛️operation/🦀️.rs"]
pub mod operation;
#[path="💼️jobs/🦀️.rs"]
mod original_jobs;
pub use original_jobs::OriginalGuestJobCall;

#[path="🎟️call/🦀️.rs"]
mod original_call;
pub use original_call::{OwnedOriginalCallStatus,OwnedOriginalCallProgress,OwnedOriginalCallResult};

#[path="🏗️startup/🦀️.rs"]
mod original_startup;
pub use original_startup::{OwnedActorStartup,OwnedActorStartupCall,OwnedActorStartupStatus};

//#region 🧠️OwnedRuntime

#[path = "🧮️owned-constants-and-codec-origin/🦀️.rs"]
mod owned_constants_and_codec_origin;
pub use owned_constants_and_codec_origin::{OWNED_STEP_FUEL, OwnedCompiledGuest};
use owned_constants_and_codec_origin::{OWNED_CHECKPOINT_MAGIC, OWNED_CODEC_ORIGIN_SCHEMA, OWNED_CODEC_ORIGIN_WAIT_POLL, OWNED_RESULT_LIMIT, OWNED_SLICE_DEADLINE_MS, OwnedCodecOrigin, OwnedCodecOriginAssembly, OwnedCodecOriginCell, OwnedCodecOriginState, OwnedCodecOriginUse};

#[path = "🛰️owned-operation-and-guest-instance/🦀️.rs"]
mod owned_operation_and_guest_instance;
pub use owned_operation_and_guest_instance::OwnedGuestInstance;
use owned_operation_and_guest_instance::{OwnedCheckpointMetadata, OwnedCheckpointView, OwnedOperation, OwnedPending, OwnedStage};

#[path = "📥️snapshot-operation-and-input-records/🦀️.rs"]
mod snapshot_operation_and_input_records;
pub use snapshot_operation_and_input_records::{OwnedInvocation, OwnedRuntime, OwnedSnapshotInput, OwnedSnapshotOperation, owned_engine_identity};
use snapshot_operation_and_input_records::{OwnedCodecInput, OwnedPollInput, OwnedRestoreInput, OwnedStartJobInput};

#[path = "🔄️owned-runtime/🦀️.rs"]
mod owned_runtime;

#[path = "⏯️operation-begin-resume-and-cancel/🦀️.rs"]
mod operation_begin_resume_and_cancel;
use operation_begin_resume_and_cancel::{OwnedDeadline, begin_owned_operation, begin_owned_scalar_operation, cancel_owned_operation, original_deadline_at, resume_owned_operation, resume_owned_operation_authored, resume_owned_operation_bound, resume_owned_operation_observed};

#[path = "📣️host-replies-wasi-and-channel-faults/🦀️.rs"]
mod host_replies_wasi_and_channel_faults;
pub use host_replies_wasi_and_channel_faults::decode_owned_result;
use host_replies_wasi_and_channel_faults::{admit_owned_channel, core_fault, reply_owned_host, turn_fault_host, write_owned_memory};
#[cfg(test)]
use host_replies_wasi_and_channel_faults::{owned_wasi_monotonic_clock, owned_wasi_nanoseconds};

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
