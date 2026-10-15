//#region 🎠️KernelRuntime
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
use semio_framework::kernel::{BrokerCapabilityGrant, Budget as TurnBudget, Effect, Event, JobPlacement, MessageEndpoint, QuotaSchema, TurnResult, UiPatch as KernelUiPatch};
use semio_framework_actor::{
    intersect_capabilities, ActivationEvent, ActorId, ActorKind, Backpressure, CapabilityGrant, Envelope, JobOperation, JobProgressIdentity, JobProgressKind, JobProgressLiveAuthority, JobProgressOverlayStore, JobProgressReceipt,
    JobProgressRejected, JobPublication, JobReplayFault, JobReplayLog, JobReplayPublicationKind, JobReplayPublicationPolicy, JobReplayRecordHeader, JobReplayRequest, JobReplayRoute, JobReplayStep, JobTurn, Lane, Origin, PackageHash, PackageId,
    Payload, JOB_PROGRESS_ACTIVE_CAPACITY,
};
use semio_framework_plugin_host::shard::ShardOutcome;
use semio_framework_plugin_host::{GuestRuntime, GuestRuntimes, PackageRef};
use semio_framework_plugin_interpreter_runtime::OwnedRuntime;
use std::collections::HashMap;
use std::future::Future;
use std::mem::ManuallyDrop;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::{Arc, Mutex, OnceLock};
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use ui_contract::{SurfaceId, UiDocumentBuilder, UiDocumentLease, UiDocumentLimits, UiFixedList, UiPatchApplyOutcome, UiPatchApplyProducer, UiPatchApplyRejected, UiPatchApplyStep, UiRevision, UiSnapshotState, UI_DOCUMENT_LEASE_SLOTS};

#[path = "🧮️kernel-core/🦀️.rs"]
mod kernel_core;
pub(crate) use kernel_core::*;

#[path = "📬️kernel-replay-client/🦀️.rs"]
mod kernel_replay_client;
pub(crate) use kernel_replay_client::*;

#[path = "🗃️kernel-retained-state/🦀️.rs"]
mod kernel_retained_state;
pub(crate) use kernel_retained_state::*;

#[path = "🏊️kernel-pool/🦀️.rs"]
mod kernel_pool;
pub(crate) use kernel_pool::*;

/// 🧩️ Reads one completed component into the compiler's contiguous input page by page. The
/// mounted native I/O authority answers at most one `JOB_PAYLOAD_PAGE_BYTES` page per request, so a
/// whole-file read refused every real guest. Local unoptimized components use the same
/// 256 MiB input ceiling as descriptor emission; network execution-target admission is separate.
pub(super) async fn read_native_component(path: &std::path::Path) -> Result<Vec<u8>, String> {
    let bound = 256 * 1024 * 1024;
    let mut component = Vec::new();
    let mut offset = 0u64;
    loop {
        let value = crate::run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadPage { path: path.to_path_buf(), offset, max_bytes: semio_framework_job::JOB_PAYLOAD_PAGE_BYTES }).await?;
        let semio_framework_os_services::NativeIoValue::Page { mut bytes, eof } = value else {
            return Err("kernel: native I/O returned the wrong value for a component page".into());
        };
        let count = bytes.len() as u64;
        offset = offset.saturating_add(count);
        if offset <= bound {
            (0..bytes.page_count()).filter_map(|index| bytes.page(index)).for_each(|page| component.extend_from_slice(page));
        }
        while !bytes.terminal_is_empty() {
            let demand = bytes.retirement_demands().map_err(|error| format!("kernel: component page demand refused: {error}"))?;
            let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            bytes.close_step(grant).map_err(|error| format!("kernel: component page close refused: {error}"))?;
        }
        if offset > bound || (count == 0 && !eof) {
            return Err(format!("kernel: component {} exceeds {bound} bytes or produced an empty nonterminal page", path.display()));
        }
        if eof {
            return Ok(component);
        }
    }
}
//#endregion 🎠️KernelRuntime

