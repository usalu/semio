//! 📦️ Native kernel runtime: the persistent bounded-poll kernel loop on the renderer worker pool. This root declares the kernel partitions and re-exports their surface.
//!
//! 🎭️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet H3-wgpu-native; upgraded by terra-kernel-loop):
//! `📓️design-runtime.md` §1's "wgpu native" host — `Kernel` runs as a persistent, bounded-poll state
//! machine on the renderer's shared worker pool; the winit thread only submits requests and drains
//! outbound results. terra-kernel-loop replaced the
//! original single-`ShardLoop` request-servant with `crate::parallel_runtime::ParallelRuntime`: real
//! `Kernel::submit`/`tick`/`complete` (DRR fairness, failure-ladder/metrics bookkeeping) dispatched
//! across K logical `ShardExecutor`s on the same pool, one per `ShardTable`-pinned shard — see
//! `📓️terra-kernel-loop-report.md` for what is (and, per that report's own honest-gaps section, is
//! NOT) wired all the way through. `ProgramBridgeBackend::Wasm` (in `🌉️ProgramBridge/`) holds a
//! [`KernelClient`] instead of the deleted `Arc<WasmPluginRuntime>`; every plugin turn now executes
//! through `Kernel` + `GuestRuntime`/`WasmtimeRuntime` + `ParallelRuntime` on pool workers, never
//! in-process on the winit thread.
//!
//! 🧭️ Native only. The facade of the four kernel crates plus the scale-bench scenario driver; it re-exports exactly the surface the renderer consumers name.

pub use semio_framework_os_renderer_wgpu_host_base::KernelPoolFuture;
pub use semio_framework_os_renderer_wgpu_kernel_pool::KernelClient;
#[cfg(any(test, feature = "testkit"))]
pub use semio_framework_os_renderer_wgpu_kernel_protocol::CANCELLED_KERNEL_TURNS;
pub use semio_framework_os_renderer_wgpu_kernel_protocol::{CreateAppBorrowed, CreateAppSourceFailure, CreateAppSourceFailureKind, open_create_app_source, read_native_component, take_job_progress_presentation, ExchangeOutcome, JobProgressPresentationLease, KernelCloseHandle, KernelCloseStatus, MountedProductReplayAdmission};

use semio_framework::kernel::TurnResult;
use semio_framework_actor::ActivationEvent;
use semio_framework_os_renderer_wgpu_contracts::ProgramFault;
use semio_framework_os_renderer_wgpu_host_base::{actor_budget_from_turn_budget, run_renderer_io};
use semio_framework_os_renderer_wgpu_kernel_protocol::parallel_runtime;

#[cfg(test)]
use semio_framework::kernel::{Effect, Event, JobPlacement, UiPatch as KernelUiPatch};
#[cfg(test)]
use semio_framework_actor::{
    ActorId, JobOperation, JobProgressIdentity, JobProgressKind, JobReplayLog, JobReplayPublicationKind, JobReplayPublicationPolicy, JobReplayRecordHeader, JobReplayRequest, JobReplayRoute, JobTurn, Lane, JOB_PROGRESS_ACTIVE_CAPACITY,
};
#[cfg(test)]
use semio_framework_os_renderer_wgpu_host_base::renderer_worker_pool;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_kernel_pool::KernelFuture;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_kernel_protocol::{
    kernel_request_interleavable, mounted_product_replay_profile, mounted_product_replay_recovery_registry, mounted_product_replay_refusal_registry, mounted_product_replay_worker_slot, CreateAppRequestOwner, ExchangeSurfaceDocument,
    JobProgressPresentationBridge, KernelCloseSubmission, KernelCloseSubmissionRegistry, KernelOutcome, KernelRequest, KernelRequestQueue, MountedProductReplayAdmissionPermit, MountedProductReplayAuthority, MountedProductReplayCheckpoint,
    MountedProductReplayClaim, MountedProductReplayExpected, MountedProductReplayRecoveryOwner, MountedProductReplayRecoveryToken, MountedProductReplayRefusalCause, MountedProductReplayRefusalToken, MountedProductReplayRequest, QueuedKernelEvent,
    QueuedKernelEventKind, RawSpawnJobDisposition, RejectedKernelEvents, RendererSequenceAuthority, ResponseSlot, JOB_PROGRESS_PRESENTATION_CAPACITY, KERNEL_CLOSE_QUEUED, KERNEL_CLOSE_SUBMISSION_CAPACITY, KERNEL_CLOSE_UNADMITTED,
    KERNEL_REQUEST_QUEUE_CAPACITY, PRODUCT_REPLAY_INPUT_BYTES, PRODUCT_REPLAY_KIND_BYTES, PRODUCT_REPLAY_PROFILE_COUNT, TURN_BUDGET, TURN_REQUESTS_BETWEEN_SLICES,
};
#[cfg(test)]
use semio_framework_os_renderer_wgpu_kernel_retained::{
    mounted_replay_recovery_registry, CommandDocumentPageReservation, CommandDocumentRetirementRegistry, CommandDocumentRetirementState, MountedJobReplay, MountedReplayRecoveryRegistry, PendingSurfaceRejection, PendingSurfaceRejectionRegistry,
    RetainedSurface, RetainedSurfaceRegistry, RetainedSurfaceSlot, COMMAND_DOCUMENT_RETIREMENT_CAPACITY, RETAINED_SURFACE_CAPACITY,
};
#[cfg(test)]
use std::future::Future;
#[cfg(test)]
use std::mem::ManuallyDrop;
#[cfg(test)]
use std::path::PathBuf;
#[cfg(test)]
use std::pin::Pin;
#[cfg(test)]
use std::sync::atomic::Ordering;
#[cfg(test)]
use std::sync::{Arc, Mutex};
#[cfg(test)]
use std::task::{Context, Poll, Waker};
#[cfg(test)]
use std::time::Duration;
#[cfg(test)]
use ui_contract::{SurfaceId, UiDocumentBuilder, UiDocumentLease, UiFixedList, UiRevision, UI_DOCUMENT_LEASE_SLOTS};

#[cfg(test)]
use semio_framework_os_renderer_wgpu_kernel_pool::*;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_kernel_protocol::*;
#[cfg(test)]
use semio_framework_os_renderer_wgpu_kernel_retained::*;

#[path = "../🧊️renderer/📊️scale-bench/🦀️.rs"]
pub mod scale_bench;

#[cfg(test)]
include!("../../../🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🦀️.rs");

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
