//! 🏊️ Kernel pool: the kernel pool state and its implementation, the pool runner and the kernel client.
//!
//! 🧭️ Native only. The module files stay in the renderer tree; this crate root mounts them and re-exports their surface.

extern crate semio_framework_os_kernel as protocol;
use semio_framework::kernel::{BrokerCapabilityGrant, Effect, Event, JobPlacement, MessageEndpoint, QuotaSchema, TurnResult, UiPatch as KernelUiPatch};
use semio_framework_actor::{
    intersect_capabilities, ActivationEvent, ActorId, ActorKind, Backpressure, Envelope, JobProgressIdentity, JobProgressKind, JobProgressLiveAuthority, JobProgressOverlayStore, JobProgressRejected, JobPublication, JobReplayFault, JobReplayLog,
    JobReplayPublicationKind, JobReplayPublicationPolicy, JobReplayRequest, JobReplayStep, JobTurn, Lane, Origin, PackageHash, PackageId, Payload, JOB_PROGRESS_ACTIVE_CAPACITY,
};
use semio_framework_os_renderer_wgpu_contracts::ProgramFault;
use semio_framework_os_renderer_wgpu_host_base::{actor_budget_from_turn_budget, log_debug, renderer_worker_pool, run_renderer_io, KernelPoolFuture};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_os_renderer_wgpu_kernel_protocol::CANCELLED_KERNEL_TURNS;
use semio_framework_os_renderer_wgpu_kernel_protocol::{
    decode_actor_turn_result, extension_index, find_wasm_artifact, install_job_progress_acknowledge, job_progress_presentation_bridge, kernel_request_interleavable, mounted_product_replay_profile, mounted_product_replay_recovery_registry,
    mounted_product_replay_refusal_registry, mounted_product_replay_worker_slot, native_shard_count, next_seq, parallel_runtime, read_native_component, reserve_seq, run_turn_continuation_turns, run_turn_quiescent_continuations,
    CreateAppRequestOwner, ExchangeOutcome, ExchangeSurfaceDocument, JobProgressPresentationToken, KernelCloseHandle, KernelCloseStatus, KernelCloseSubmission, KernelCloseSubmissionRegistry, KernelOutcome, KernelRequest, KernelRequestQueue,
    MountedProductReplayAdmissionPermit, MountedProductReplayAuthority, MountedProductReplayCheckpoint, MountedProductReplayClaim, MountedProductReplayExpected, MountedProductReplayRequest, QueuedKernelEvent, RefusedProductReplay,
    RejectedKernelEvents, ResponseSlot, TypedOperationResultPage, COMPILED_COMPONENT_CAPACITY, JOB_PROGRESS_PRESENTATION_CAPACITY, JOB_REPLAY_STEP_GRANT, KERNEL_CLOSE_READY, KERNEL_CLOSE_UNADMITTED, KERNEL_REQUEST_QUEUE_CAPACITY, PRODUCT_REPLAY_PROFILE_COUNT,
    RUN_TURN_OUTCOME_TIMEOUT, RUN_TURN_SETTLE_BUDGET, TURN_BUDGET, TURN_REQUESTS_BETWEEN_SLICES,
};
use semio_framework_os_renderer_wgpu_kernel_retained::{
    mounted_replay_recovery_registry, ClosingKernelApp, CommandDocumentRetirementRegistry, MountedJobReplay, MountedReplayRouteSeed, PendingJobProgressPresentation, PendingSurfaceRejection, PendingSurfaceRejectionRegistry, ReservedToolJob,
    RetainedSurfaceRegistry,
};
use semio_framework_plugin_host::shard::ShardOutcome;
use semio_framework_plugin_host::{GuestRuntimes, PackageRef};
use semio_framework_plugin_interpreter_runtime::OwnedRuntime;
use std::collections::HashMap;
use std::future::Future;
use std::mem::ManuallyDrop;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex, OnceLock};
use std::task::{Context, Poll, Waker};
use ui_contract::{SurfaceId, UiFixedList, UiRevision, UI_DOCUMENT_LEASE_SLOTS};

#[path = "../🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs"]
mod kernel_pool;
pub use kernel_pool::*;

#[path = "../🧊️renderer/📦️kernel-runtime/📬️kernel-replay-client/🦀️.rs"]
mod kernel_client;
pub use kernel_client::*;

#[path = "📥️lifted/🗃️kernel-retained-state/🦀️.rs"]
mod pool_state;
pub use pool_state::*;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
