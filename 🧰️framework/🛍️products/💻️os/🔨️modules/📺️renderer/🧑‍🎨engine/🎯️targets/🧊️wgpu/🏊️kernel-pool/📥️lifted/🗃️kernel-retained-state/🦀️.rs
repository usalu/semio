//! 🗃️ The kernel pool state: the thirty fields the pool implementation owns.
//!
//! 🧭️ Lifted out of the retained state into kernel-pool, so the struct shares a crate with its inherent implementation.

use super::*;

//#region 🎠️KernelRuntime
//#region 🔖️KernelPoolState
pub struct KernelPoolState {
    pub request_queue: Arc<KernelRequestQueue>,
    pub guest_runtime: Arc<GuestRuntimes>,
    /// 🎠️ terra-kernel-loop: the real multi-shard engine — replaces the single physical
    /// `ShardLoop`/`Kernel::new(.., 1, 0, ..)` this host used to run. `Kernel::new(Native, K, 2,
    /// 64)` (`exclusive_reserve: 2` — item 3 of the packet brief — makes `request_exclusive`
    /// real; no caller in this file exercises it yet, but the reserve pool now genuinely exists).
    /// P1e (INTERACTIVE-JOB-RUNTIME-REFACTOR, one-pool-worker-runtime): every shard now runs as a
    /// pool-scheduled job on `crate::renderer_worker_pool()` — no `ShardExecutor`/forwarder OS
    /// threads — see `🎠️runtime.rs`'s own module doc.
    pub runtime: crate::parallel_runtime::ParallelRuntime,
    pub worker_count: u16,
    /// ⏱️ Monotonic milliseconds this host's own `Kernel::tick` calls are stamped with — this
    /// crate's purity-respecting clock source (`Kernel` itself takes no clock, per `🎭️actor`'s
    /// own rule), incremented once per `run_turn`-internal tick, never wall-clock-read.
    pub now_ms: u64,
    pub plugin_ordinals: HashMap<String, u16>,
    /// 📇️ `instance_id` (the `u32` `ProgramBridgeEntry`'s callers already address plugin apps
    /// by) → the kernel's own bit-packed `ActorId`, minted by `Kernel::activate`.
    pub instances: HashMap<u32, ActorId>,
    pub next_instance_id: u32,
    /// 🖼️ One retained semantic [`UiSnapshotState`] plus its last published generation-qualified
    /// document lease per `(instance, surface)`. Every [`UiPatchOp`] is applied through the
    /// contract's retained, quota-bounded [`UiPatchApplyProducer`]; a rejection preserves both
    /// the document revision and the previously published document.
    pub retained: RetainedSurfaceRegistry,
    /// 🔁️ Surfaces whose next turn must carry an `Event::PatchRejected`, retaining both
    /// the receiver revision and the contract rejection reason.
    pub pending_rejections: PendingSurfaceRejectionRegistry,
    pub retained_command_closes: semio_framework::kernel::CommandDriverRegistry<1>,
    pub queued_command_closes: semio_framework::kernel::CommandDriverRegistry<1>,
    pub command_document_closes: CommandDocumentRetirementRegistry,
    pub rejected_command_builds: semio_framework::kernel::RejectedCommandBuildRegistry<1>,
    pub rejected_events: Option<RejectedKernelEvents>,
    pub job_progress: JobProgressOverlayStore,
    pub replay_routes: [Option<MountedReplayRouteSeed>; JOB_PROGRESS_ACTIVE_CAPACITY],
    pub job_replays: [Option<MountedJobReplay>; JOB_PROGRESS_ACTIVE_CAPACITY],
    pub reserved_jobs: [Option<ReservedToolJob>; JOB_PROGRESS_ACTIVE_CAPACITY],
    pub product_replay_claims: [Option<MountedProductReplayClaim>; JOB_PROGRESS_ACTIVE_CAPACITY],
    pub product_replay_authorities: [Option<MountedProductReplayAuthority>; JOB_PROGRESS_ACTIVE_CAPACITY],
    pub rejected_job_progress: [Option<JobProgressRejected>; 64],
    pub pending_job_progress_presentations: [Option<PendingJobProgressPresentation>; JOB_PROGRESS_PRESENTATION_CAPACITY],
    pub closing_apps: [Option<ClosingKernelApp>; JOB_PROGRESS_ACTIVE_CAPACITY],
    pub fault_closing_actors: [Option<ActorId>; JOB_PROGRESS_ACTIVE_CAPACITY],
    pub realm_progress_close_started: bool,
    pub semantic_close_document_lane: bool,
    /// 🎛️ The response slots of the requests being served, innermost last, each with whether its owner
    /// may cancel it mid-turn ([`Self::turn_cancelled`]).
    pub serving: Vec<(Arc<ResponseSlot>, bool)>,
    /// ⚖️ True while a request runs in another actor's slice gap ([`Self::yield_slice`]): a gap never nests.
    pub interleaving: bool,
    /// 🧩️ The components compiled most recently, newest first, by content hash ([`Self::compile_serving_others`]).
    pub compiled_components: Vec<(PackageHash, semio_framework_plugin_host::CompiledHandle)>,
}
//#endregion 🔖️KernelPoolState
//#endregion 🎠️KernelRuntime
