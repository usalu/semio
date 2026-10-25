//! 🎭️ The guest runtime of the plugin host: the `GuestRuntime` trait, the scripted mock, the recording double, the owned interpreter runtime, the wasmtime runtime, snapshot calls and the component document codec.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

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
use wasmtime_runtime::WasmtimeInstanceState;

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

#[path = "💼️job-crossing/🦀️.rs"]
mod job_crossing;
pub use job_crossing::{self_issued_budget, JobCrossing, JobFunds};

#[cfg(test)]
#[path = "../🔁️lifecycle/🧪️tests/🔁️lifecycle/🦀️.rs"]
mod guest_fault_tests;

//#region 🔖️MockGuestRuntime

#[cfg(test)]
#[path = "../🧪️tests/🔬️mock-guest-runtime/🦀️.rs"]
mod mock_guest_runtime_tests;
//#endregion 🔖️MockGuestRuntime

//#region 🐎️WasmtimeRuntime
/// 🧬️ The real `impl GuestRuntime for WasmtimeRuntime` — `design-runtime.md` §2. Nested `mod
/// actor_bindings` (mirrors the file's own `mod extension_bindings` idiom below, "wasmtime's
/// `bindgen!` cannot be invoked twice at the same module scope") so this coexists with the OLD
/// `plugin-world`/`extension-world` `bindgen!` calls until the deletion pass — both of which are
/// now unconditionally broken independent of this packet (their `world` names no longer exist in
/// `📜️world.wit`, which declares only `world actor`), a finding recorded in
/// `📓️terra-B1-host-native-report.md`.

use semio_framework_plugin_host_engine::wasm;
pub(crate) use wasm::component::Accessor;
use semio_framework_actor::{ActorId, OriginalGuestExecutionBudget, OriginalGuestExecutionReceipt, OriginalJobBudget, OriginalJobCallFailure, OriginalJobCancelReply, OriginalJobOutcome, OriginalJobOutputAckReply, OriginalJobOutputChunk, OriginalJobOutputReply, OriginalJobOutputState, OriginalJobSource, OriginalJobStartReply, OriginalJobStepReply, PackageHash, PackageId, ActorId as RuntimeActorId};
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use semio_framework::kernel::{BrokerCapabilityGrant, Budget, Event, TurnResult, TurnStatus};
use semio_framework_plugin_host_engine::{BudgetLimiter, EpochDeadlineCell, EpochDeadlines, EpochDemand, IsolatedCompileTarget, SharedEngineConfig, build_shared_engine, compiled_cache_path, default_compiled_cache_root, hex_encode, load_compiled_component, plugin_host_worker_pool, shared_engine_config_hash, store_compiled_component};
use wasm::component::{Component, Linker, ResourceTable};
use wasm::{Engine, ResourceLimiter, Store};
use semio_framework_plugin_guest_contract::{GUEST_DIAGNOSTICS_CAPACITY_BYTES, GuestCallCancellation, GuestDocumentMirror, GuestDocumentPair, JobBudget, JobStep, PluginHostError, TurnFault, decode_guest_fault_bytes, identity_turn_fault, sqlite_wire};
use std::collections::{HashMap, VecDeque};
use semio_framework_plugin_host_engine::original_deadline::{OriginalDeadlineError, remaining_original_deadline_us};
use semio_framework_plugin_interpreter_runtime::{OriginalGuestJobCall, OwnedCompiledGuest, OwnedGuestInstance, OwnedRuntime};
use std::path::PathBuf;
use wasm::wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};
use semio_framework_plugin_host_wit::conversion::{kernel_turn_inputs_to_wit, wit_command_ingress_to_kernel, wit_effect_to_kernel, wit_lifecycle_receipt_to_kernel, wit_patch_receipt_to_kernel, wit_snapshot_grant, wit_snapshot_limits, wit_snapshot_rejection, wit_snapshot_retirement, wit_turn_status_to_kernel};
use semio_framework_plugin_host_wit::cold_pair::wit_cold_ingress_to_kernel;
use semio_framework_plugin_host_wit::wit_capabilities;
use semio_framework_plugin_host_wit::wit_effects;
use semio_framework_plugin_host_wit::wit_events;
use semio_framework_plugin_host_wit::wit_host_async;
use semio_framework_plugin_host_wit::wit_lifetime;
use semio_framework_plugin_host_wit::wit_reactor;
use semio_framework_plugin_host_wit::wit_types;
use semio_framework_plugin_host_wit::wit_ui;

//#endregion 🐎️WasmtimeRuntime
//#endregion 🎭️GuestRuntime
