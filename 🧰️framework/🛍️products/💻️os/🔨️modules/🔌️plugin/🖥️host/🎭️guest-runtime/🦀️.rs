//! 🎭️ The guest runtime of the plugin host: the `GuestRuntime` trait, the scripted mock, the recording double, the owned interpreter runtime, the wasmtime runtime, snapshot calls and the component document codec.

#![allow(async_fn_in_trait)]


extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;

#[path = "../🚪️io/🪶️snapshot/🦀️.rs"]
mod snapshot_call;
pub use snapshot_call::{SnapshotCall, SnapshotCallOutput, SnapshotCallStatus, SnapshotComponentReceipt};
#[path = "../🧬️component-codec/🦀️.rs"]
mod component_codec;
pub use component_codec::{OwnedComponentDocumentCodec, GUEST_CODEC_BUDGET};

//#region 🎭️GuestRuntime

#[path = "🧱️guest-types-and-runtime-trait/🦀️.rs"]
mod guest_types_and_runtime_trait;
pub use guest_types_and_runtime_trait::{CompiledHandle, GuestInstance, GuestInstanceState, GuestRuntime, PackageRef, retryable_lifecycle_turn};
use guest_types_and_runtime_trait::{decode_guest_plugin_error, receiving_turn_fault};

#[path = "🤖️mock-guest-runtime/🦀️.rs"]
mod mock_guest_runtime;
pub use mock_guest_runtime::MockInstanceState;
#[cfg(any(test, feature = "testkit"))]
pub use mock_guest_runtime::{MockGuestRuntime, MockJobStepGate};

#[path = "🧠️owned-runtime-adapter/🦀️.rs"]
mod owned_runtime_adapter;
use owned_runtime_adapter::{owned_compiled, snapshot_component_transport};

#[path = "🏠️wasmtime-host-state-and-wit-hosts/🦀️.rs"]
mod wasmtime_host_state_and_wit_hosts;
pub use wasmtime_host_state_and_wit_hosts::{GUEST_JOB_WATCHDOG_MS, classify_guest_trap};
use wasmtime_host_state_and_wit_hosts::{ActorHostState, arm_store_fuel, guest_wasi_ctx};

#[path = "🐎️wasmtime-runtime/🦀️.rs"]
mod wasmtime_runtime;
pub use wasmtime_runtime::{WasmtimeRuntime, observe_sqlite_guest};
use wasmtime_runtime::{ActorStore, WasmtimeInstanceState};

#[path = "🗂️wasmtime-guest-runtime-and-codec-dispatch/🦀️.rs"]
mod wasmtime_guest_runtime_and_codec_dispatch;

#[path = "🎛️guest-runtimes-and-test-doubles/🦀️.rs"]
mod guest_runtimes_and_test_doubles;
pub use guest_runtimes_and_test_doubles::GuestRuntimes;
#[cfg(any(test, feature = "testkit"))]
pub use guest_runtimes_and_test_doubles::RecordingRuntime;

#[path="../🏗️instance/🦀️.rs"]
mod original_instance_opening;
pub use original_instance_opening::GuestActorOpening;
#[path="🎟️unit/🦀️.rs"]
mod original_unit;
#[path="🪙️escrow/🦀️.rs"]
mod original_escrow;
pub use original_escrow::{GuestBootstrapHeaderIdentity,GuestBootstrapHeaderDeallocateReturned,GuestFrozenBootstrapBorrow,GuestActorFailureCallReference,GuestHeaderDeallocateReturned,GuestActorFailureAcknowledged,GuestFrozenEscrowBorrow,GuestFrozenEscrowIdentity};

#[path = "💼️job-crossing/🦀️.rs"]
mod job_crossing;
pub use job_crossing::{self_issued_budget, JobCrossing, JobFunds};

#[cfg(test)]
#[path = "../🔁️lifecycle/🧪️tests/🔁️lifecycle/🦀️.rs"]
mod guest_fault_tests;

//#region 🔖️MockGuestRuntime


//#endregion 🔖️MockGuestRuntime

//#region 🐎️WasmtimeRuntime
use semio_plugin_host_engine::wasm;
pub(crate) use wasm::component::Accessor;
use semio_framework_actor::{ActorId, OriginalGuestExecutionBudget, OriginalGuestExecutionReceipt, OriginalJobBudget, OriginalJobCallFailure, OriginalJobCancelReply, OriginalJobOutcome, OriginalJobOutputAckReply, OriginalJobOutputChunk, OriginalJobOutputReply, OriginalJobOutputState, OriginalJobSource, OriginalJobStartReply, OriginalJobStepReply, PackageHash, PackageId, ActorId as RuntimeActorId};
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use semio_framework::kernel::{BrokerCapabilityGrant, Budget, Event, TurnResult, TurnStatus};
use semio_plugin_host_engine::{BudgetLimiter, EpochDeadlineCell, EpochDeadlines, EpochDemand, IsolatedCompileTarget, SharedEngineConfig, build_shared_engine, compiled_cache_path, default_compiled_cache_root, hex_encode, load_compiled_component, plugin_host_worker_pool, shared_engine_config_hash, store_compiled_component};
use wasm::component::{Component, Linker, ResourceTable};
use wasm::{Engine, ResourceLimiter, Store};
use semio_plugin_guest_contract::{GUEST_DIAGNOSTICS_CAPACITY_BYTES, GuestCallCancellation, GuestDocumentMirror, GuestDocumentPair, JobBudget, JobStep, PluginHostError, TurnFault, decode_guest_fault_bytes, identity_turn_fault, sqlite_wire};
use std::collections::{HashMap, VecDeque};
use semio_plugin_host_engine::original_deadline::{OriginalDeadlineError, remaining_original_deadline_us};
pub use semio_plugin_interpreter_runtime::OriginalGuestJobCall;
pub use semio_plugin_interpreter_runtime::OriginalOperationReceiving;
pub use semio_plugin_interpreter_runtime::{OriginalCoreValue,OwnedOriginalCallResult};
pub use semio_plugin_interpreter_runtime::{OwnedActorStartupStatus,OwnedSemioExport,OwnedOriginalCallProgress,OwnedOriginalCallStatus};
pub use semio_plugin_interpreter_runtime::{OwnedCodecInput,OwnedSnapshotInput};
use semio_plugin_interpreter_runtime::{OwnedCompiledGuest, OwnedGuestInstance, OwnedRuntime};
use std::path::PathBuf;
use wasm::wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};
use semio_plugin_host_wit::conversion::{kernel_turn_inputs_to_wit, wit_command_ingress_to_kernel, wit_effect_to_kernel, wit_lifecycle_receipt_to_kernel, wit_patch_receipt_to_kernel, wit_snapshot_rejection, wit_snapshot_retirement, wit_turn_status_to_kernel};
use semio_plugin_host_wit::cold_pair::wit_cold_ingress_to_kernel;
use semio_plugin_host_wit::wit_capabilities;
use semio_plugin_host_wit::wit_effects;
use semio_plugin_host_wit::wit_events;
use semio_plugin_host_wit::wit_host_async;
use semio_plugin_host_wit::wit_lifetime;
use semio_plugin_host_wit::wit_reactor;
use semio_plugin_host_wit::wit_types;
use semio_plugin_host_wit::wit_ui;

//#endregion 🐎️WasmtimeRuntime
//#endregion 🎭️GuestRuntime

pub use semio_plugin_interpreter_runtime::{OwnedActorFailureAckFrame,OwnedActorFailureAcknowledged};
