//! 🔀️ The post-turn relay of the plugin host: guest relay pool, instance slots and leases, cold relay jobs, the mounted registry and the plugin instance handle.

//#region 🔀️PostTurnRelay

#[path = "🏊️pool-future-requests-and-completion-slot/🦀️.rs"]
mod pool_future_requests_and_completion_slot;
pub use pool_future_requests_and_completion_slot::RELAY_JOB_BUDGET;
use pool_future_requests_and_completion_slot::{
    GuestRelayCompletionSender, GuestRelayCompletionSlot, GuestRelayInlineRequest,
    GuestRelayOwnedBytes, GuestRelayPoolFailure, GuestRelayPoolFuture, GuestRelayRequest,
    GuestRelayRequestKind, GuestRelayStepCompletion,
};
pub(crate) use pool_future_requests_and_completion_slot::GuestRelayCompletion;

#[path = "🎟️instance-slots-leases-and-cancel-helpers/🦀️.rs"]
mod instance_slots_leases_and_cancel_helpers;
use instance_slots_leases_and_cancel_helpers::{
    cancel_guest_job_once, foreground_cancel_completion, guest_relay_cleanup_pending_detail,
    mark_guest_cleanup_pending, quarantine_guest_instance, GuestInstanceLease, GuestInstanceSlot,
};

#[path = "📡️relay-attempts-and-publication/🦀️.rs"]
mod relay_attempts_and_publication;
use relay_attempts_and_publication::{
    receive_relay_close, recover_guest_relay_failure, run_guest_relay_request, GuestRelayAttempt,
    GuestRelayPublication, GuestRelayPublicationKind,
};

#[path = "🥶️cold-relay-job/🦀️.rs"]
mod cold_relay_job;
use cold_relay_job::GuestColdRelayJob;

#[path = "🔬️lifecycle-probe-and-refused-source/🦀️.rs"]
mod lifecycle_probe_and_refused_source;
use lifecycle_probe_and_refused_source::{
    GuestRelayLifecycleProbeControl, GuestRelayLifecycleProbeJob, GuestRelayRefusedSource,
};

#[path = "🧾️mounted-session-types-and-registry-struct/🦀️.rs"]
mod mounted_session_types_and_registry_struct;
use mounted_session_types_and_registry_struct::{
    admit_guest_relay_session, GuestRelayMountedClose, GuestRelayMountedLifecycle,
    GuestRelayMountedOutput, GuestRelayMountedOwner, GuestRelayMountedRegistry,
    GuestRelayMountedSession, GuestRelayMountedSlot, GuestRelayMountedTerminal,
    GuestRelayOutcomeKind, GuestRelayOutcomeStage, GuestRelayWakeCustody, GUEST_RELAY_MOUNTED_SLOTS,
    GUEST_RELAY_REAPER_FALLBACK_MS,
};
pub use mounted_session_types_and_registry_struct::GuestRelayWakeAuthority;

#[path = "🗂️mounted-registry-lifecycle/🦀️.rs"]
mod mounted_registry_lifecycle;

#[path = "⚙️mounted-registry-pump-and-reaper/🦀️.rs"]
mod mounted_registry_pump_and_reaper;
#[cfg(test)]
use mounted_registry_pump_and_reaper::{
    read_guest_relay_outcome_page, step_guest_relay_owner, GuestRelayStep,
};
use mounted_registry_pump_and_reaper::{
    GuestRelayMountedFuture, GuestRelayMountedReap, GuestRelayMountedReaper,
};

#[path = "🧪️lifecycle-trace-exercises/🦀️.rs"]
mod lifecycle_trace_exercises;
pub use lifecycle_trace_exercises::exercise_relay_lifecycle_trace;

#[path = "🔌️plugin-instance-handle/🦀️.rs"]
mod plugin_instance_handle;
pub use plugin_instance_handle::PluginInstanceHandle;
#[cfg(any(test,feature="testkit"))]
pub use plugin_instance_handle::test_relay_wake_authority;
#[cfg(any(test,feature="testkit"))]
use plugin_instance_handle::wait_for_scripted_guest_relay_release;

#[cfg(test)]
#[path="../♻️publication/🧪️tests/🦀️.rs"]
mod original_publication_source_tests;

use semio_framework_plugin_host_wake::{GuestRelayWakeDriver, GuestRelayWakeReceiptLedger, GuestRelayWakeTurnGrant};
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use semio_framework_plugin_host_guest_runtime::{GuestInstance, GuestRuntimes};
use semio_framework_plugin_host_wake::GuestRelayWakeReceipt;
use semio_framework_plugin_guest_contract::{JobBudget, JobStep, PluginHostError, TurnFault};
use semio_framework_async::{Lane, ProcessKind, WorkerPool, WorkerPoolConfig};
use semio_framework_actor::ActorId as RuntimeActorId;
use semio_framework_value_derive::ToValue;
use semio_framework_plugin_host_engine::plugin_host_worker_pool;
use semio_framework_plugin_host_guest_runtime::GuestRuntime as _;
use semio_framework_plugin_host_wake::GuestRelayWakeReceiptReceiver as _;
#[cfg(any(test, feature = "testkit"))]
#[global_allocator]
static HOST_HEAP_WITNESS:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

#[cfg(test)]
#[path = "../🧪️tests/🔬️guest-cold-relay/🦀️.rs"]
mod guest_cold_relay_tests;
//#endregion 🔀️PostTurnRelay
