//! ⏳️ Async-runtime interface layer AND the process-wide CPU substrate: `OperationContext`/
//! `CancelToken`/`Scope`/`ChannelPolicy` vocabulary, the [`HostAsyncRuntime`] trait that hides the
//! concrete future-polling executor from every other framework crate, and — as of the INTERACTIVE-
//! JOB-RUNTIME-REFACTOR Phase 1 packet P1a — [`WorkerPool`], the ONE process-wide work-stealing
//! thread pool every subsystem now schedules CPU-bound and blocking work onto. No `tokio` in this
//! crate; the future-*polling* executor (tokio today, packet R2's sibling crate) still owns turning
//! a `Future` into progress, but the OS threads it runs ON, and every other subsystem's OS threads
//! (shard executors, DB actors, epoch tickers, HTTP fetch threads — see Phase 0's thread census),
//! collapse into this one pool. `now_ms` stays caller/implementation-supplied everywhere in the
//! vocabulary types below (`OperationContext`, `HostAsyncRuntime::now_ms`); [`WorkerPool`]'s OWN
//! internal idle-park/timer-wheel bookkeeping is the one exception — parking a real OS thread for a
//! bounded interval is a mechanism, not a caller-facing clock reading, exactly the same way
//! `semio_framework_trace::now_us` owns a clock for its own instrumentation without becoming
//! `HostAsyncRuntime`'s clock.
//!
//! 🪡 **Where tokio actually lives**: the future-polling reactor only, in a sibling crate (design
//! ticket `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, packet R2; re-hosted onto
//! [`WorkerPool`] by Phase 1 packet P1b, `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services`).
//! `ManualRuntime` in the sibling crate `semio-framework-async-testkit` exists so downstream crates can
//! unit-test against [`HostAsyncRuntime`] without ever linking tokio.
//!
//! 🧭 **Scope discipline**: [`HostAsyncRuntime::spawn_scoped`] takes a `&ScopeHandle`, not an
//! ambient context — there is no detached-spawn entry point on this trait. Every unit of async work
//! belongs to a [`Scope`] that can be found, waited on and drained; "who owns this task" is always
//! answerable.
//!
//! 🚫 **Not on this trait**: raw I/O primitives (sockets, files). `run_blocking` was removed from
//! this trait in Phase 1 packet P1a — a caller that needs to run a blocking closure now submits it
//! to [`WorkerPool`] directly (on [`Lane::Io`] or [`Lane::Background`] as appropriate), the same
//! substrate every other lane uses, instead of a dedicated trait method.
//!
//! See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-runtime.md`
//! and `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📋️master.md` (Phase 1,
//! packet P1a) for the worker-pool redesign this file implements.

use std::collections::VecDeque;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex, PoisonError};
use std::task::Waker;

#[cfg(not(target_arch="wasm32"))]
#[path="🔔️wake/🧵️thread/🦀️.rs"]
pub mod original_thread_wake;

//#region 🌐️BrowserFutureBridge
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "⏱️clock/🦀️.rs"]
mod browser_clock;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use browser_clock::install_browser_monotonic_clock;

// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this module is a browser JS
// `Promise`/`JsValue` bridge with no meaning under WASI, so it is narrowed to exclude the
// component target. wasip2 task execution runs through `WorkerPool` above instead (its
// `wasm_pool` variant has no JS dependency and stays on the bare arch gate correctly).
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🌐️browser/🦀️.rs"]
pub mod browser;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use browser::{future_to_promise, spawn_local, JsFuture};
//#endregion 🌐️BrowserFutureBridge

//#region 🧬️SchemaMetadata
#[cfg(feature = "testkit")]
#[path = "🏷️schema-metadata/🦀️.rs"]
pub mod schema_metadata;
//#endregion 🧬️SchemaMetadata

#[path = "🪪️operation-context/🦀️.rs"]
mod operation_context;
pub use operation_context::{CapabilityTokenId, OperationContext, TraceId};

#[path = "⛔️cancel-token/🦀️.rs"]
mod cancel_token;
pub use cancel_token::{CancelState, CancelToken, Cancelled};
use cancel_token::CancelNode;
#[path="🛑️cancel/🚦️waiters/🦀️.rs"]
mod cancel_waiters;
use cancel_waiters::CancelWaiters;

#[path="🛑️cancel/♻️retirement/🦀️.rs"]
mod cancellation_retirement;
pub use cancellation_retirement::{CancelTokenRetirement,CancelTokenRetirementError,CancelTokenRetirementBlocked};
#[path="♻️cancel-return/📬️claim/🦀️.rs"]
mod publication_claim;
pub use publication_claim::{PublicationClaim,PublicationClaimHandle,PublicationPermit};

#[path = "🌳️scope/🦀️.rs"]
mod scope;
pub use scope::{ChannelPolicy, HostAsyncRuntime, HostFuture, ScopeDrainReport, ScopeHandle, ScopeId, ScopeOwner};

#[cfg(feature = "entrypoint")]
#[path = "🌉️block-on/🦀️.rs"]
mod block_on_entry;
#[cfg(feature = "entrypoint")]
pub use block_on_entry::block_on;

#[path = "🛣️lane/🦀️.rs"]
mod lane;
pub use lane::{Lane, PermitError, PermitGuard, PermitLedger, ProcessKind, worker_count_for};
use lane::LANE_COUNT;
#[cfg(not(target_arch = "wasm32"))]
use lane::is_low_priority;

use semio_framework_async_timer::{TIMER_ACTIONS_PER_POOL_TURN, TimerWheel};

#[path = "🔀️combinators/🦀️.rs"]
mod combinators;
pub use combinators::{Either, join2, race2, select2, yield_once};

#[path = "🧵️worker-pool/🦀️.rs"]
mod worker_pool;
pub use worker_pool::{Job, WORKER_JOBS_PER_LANE, WorkerPoolConfig, WorkerPoolShutdownError, WorkerPoolUseError, WorkerSubmitError, WorkerSubmitErrorKind, process_worker_pool};
use worker_pool::{PoolLifecycleState, admitted_job_queue};
#[path = "🔔️maintenance/🦀️.rs"]
mod maintenance;
use maintenance::{PoolWork, WorkerMaintenanceRegistry};
pub use maintenance::{WorkerMaintenanceCallback, WorkerMaintenanceError, WorkerMaintenanceRequest, WorkerMaintenanceStep, WorkerMaintenanceTicket, WORKER_MAINTENANCE_CAPACITY};

#[path = "🔔️deferred-wake/🦀️.rs"]
mod deferred_wake;
use deferred_wake::WorkerDeferredWakeRegistry;
pub use deferred_wake::{WorkerDeferredWakeError, WorkerDeferredWakeRejected, WorkerDeferredWakeTicket, WORKER_DEFERRED_WAKES_PER_PARTITION, WORKER_DEFERRED_WAKE_CAPACITY, WORKER_DEFERRED_WAKE_PARTITIONS};
#[cfg(not(target_arch = "wasm32"))]
#[path = "🔔️worker-parking/🦀️.rs"]
mod worker_parking;

//#region 🧵️WorkerPoolNative
#[cfg(not(target_arch = "wasm32"))]
#[path = "🖥️native-pool/🦀️.rs"]
mod native_pool;
//#endregion 🧵️WorkerPoolNative

//#region 🧵️WorkerPoolWasm
#[cfg(any(target_arch = "wasm32", feature = "wasm-pool-host"))]
#[path = "🕸️wasm-pool/🦀️.rs"]
mod wasm_pool;
//#endregion 🧵️WorkerPoolWasm

#[cfg(not(target_arch = "wasm32"))]
pub use native_pool::{WORKER_STACK_BYTES, WorkerPool, WorkerPoolUse};
#[cfg(target_arch = "wasm32")]
pub use wasm_pool::{CooperativePoolSnapshot, WorkerPool, WorkerPoolUse};


#[path = "🔂️poll/🦀️.rs"]
pub mod poll;

#[path = "🔐️publication/🦀️.rs"]
pub mod publication;

#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod internals {
    pub use super::publication_claim::internals as publication_claim;
    pub use super::deferred_wake::internals as deferred_wake;
    pub use super::maintenance::internals as maintenance;
    #[cfg(not(target_arch = "wasm32"))]
    pub use super::worker_parking::internals as worker_parking;
    pub const LANE_COUNT: usize = super::LANE_COUNT;
    pub type CancelNode = super::CancelNode;
    #[cfg(not(target_arch = "wasm32"))]
    pub use super::native_pool::internals as native_pool;
}

#[cfg(feature = "wasm-pool-host")]
#[doc(hidden)]
pub mod wasm_internals {
    pub use super::wasm_pool::internals::*;
}
