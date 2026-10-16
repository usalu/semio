//! 🛎️ Host-side async SERVICES built ON TOP of `semio-framework-async`'s `HostAsyncRuntime` — the
//! ONE crate in this tree allowed to name `tokio`. Every other framework/os/plugin crate reaches
//! this functionality only through `semio-framework-async` vocabulary (`OperationContext`,
//! `ScopeHandle`, `ChannelPolicy`, `ThreadPlan`) or this crate's own domain types — never a tokio
//! type — exactly as `wasmtime` is confined behind `GuestRuntime` in the plugin-host crate. Verify
//! this holds with `grep -rnE 'tokio' --include='🦀️.rs' . | grep -v '^\s*[0-9]*://'` against every `pub
//! fn`/`pub struct` signature (see the packet report's `## tokio-containment evidence`).
//!
//! 🚂️ [`TokioHostRuntime`] is the one `HostAsyncRuntime` implementation. Every other public type
//! here ([`TimerWheel`], [`ComputePool`], [`HttpPool`], [`StorageScheduler`], [`EventRouter`]) is a
//! SERVICE built on top of that trait, reaching around it into raw tokio only where explicitly
//! noted (the timer driver task, the HTTP bucket refill driver, the storage dispatcher) — never
//! around `semio-framework-async`.
//!
//! 🧾️ Re-entry into the kernel from any of these services happens ONLY through [`CompletionSink`].
//! No type in this crate holds or calls a `Kernel` directly.
//!
//! 🏷️ Naming: per-plugin quota accounting ([`TimerWheel`]'s armed-timer count,
//! [`StorageScheduler`]'s byte budget, [`HttpPool`]'s byte bucket) and per-actor accounting
//! ([`HttpPool`]'s outstanding cap, [`EventRouter`]'s subscribers) use `semio_framework_actor`'s
//! own [`PackageId`]/[`ActorId`] directly rather than a parallel local newtype for the same
//! concept — unlike `semio-framework-async` (which stays domain-neutral and keeps
//! `OperationContext.actor` a bare `u64` on purpose), this crate IS the OS-tier host serving this
//! kernel, and the actor crate is pure (no tokio, no threads, builds for `wasm32-unknown-unknown`),
//! so depending on it costs no platform coupling. `🔌️plugin/🖥️host` already depends on it the same
//! way. `CompletionSink::complete` and [`TimerFired`]'s `actor`/`generation` fields stay bare
//! `u64`/`u16`, unrelated to this: they mirror `OperationContext`'s own untyped re-entry shape, and
//! `OperationContext.generation` is a different concept from `ActorId`'s packed 14-bit
//! restart-generation bits — converting one into the other would be a category error, not a typing
//! improvement.
//!
//! See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️design-runtime.md`.

//#region 🧮️ComputePool
#[path="🧮️compute/🎟️invocation/📋️parameters/🦀️.rs"]
mod original_compute_parameters;
pub use original_compute_parameters::ComputeJobParamsPreparation;
//#endregion 🧮️ComputePool

#[path = "🚂️host-runtime/🦀️.rs"]
mod host_runtime;
#[path = "⏲️timer-wheel/🦀️.rs"]
mod timer_wheel;
#[path = "🧰️compute-pool/🦀️.rs"]
mod compute_pool;
#[path = "🌐️http-contract/🦀️.rs"]
mod http_contract;
#[path = "🔌️http-socket/🦀️.rs"]
mod http_socket;
#[path = "🪣️http-pool/🦀️.rs"]
mod http_pool;
#[path = "📄️fixed-file-storage/🦀️.rs"]
mod fixed_file_storage;
#[path = "💾️storage-scheduler/🦀️.rs"]
mod storage_scheduler;
#[path = "📮️event-router/🦀️.rs"]
mod event_router;
#[path = "🧾️completion-sink/🦀️.rs"]
mod completion_sink;

pub use completion_sink::{CompletionRecord, CompletionSink, MockCompletionSink};
pub use compute_pool::{ComputeError, ComputeJobCompletion, ComputePool, ComputeRetainedRecipient};
pub use event_router::{EventRouter, PublishOutcome, Topic};
pub use fixed_file_storage::{
    FixedFileDocumentWriteStep,
    FixedFileDocumentWriteToken,
    STORAGE_FIXED_FILE_DOCUMENT_MAX_BYTES,
    STORAGE_FIXED_FILE_DOCUMENT_MAX_WRITE_STEPS,
    STORAGE_FIXED_FILE_PAGE_BYTES,
};
pub use host_runtime::TokioHostRuntime;
pub use http_contract::{
    AsyncHttpTransport,
    BlockingHttpTransport,
    HttpBody,
    HttpBodyCancellationHandle,
    HttpBodyCancellationStep,
    HttpPoolError,
    HttpRequest,
    HttpResponse,
    HttpResponseHead,
    HttpTransport,
    HttpTransportStart,
    HttpTransportTerminalGuard,
    HttpTransportTerminalHandle,
    StartedTransport,
    UnwiredHttpTransport,
};
pub use http_pool::{HTTP_BUCKET_REFILL_INTERVAL_MS, HttpPackageBudget, HttpPool, HttpPoolBody};
pub use http_socket::{SocketHttpBody, SocketHttpTransport};
pub use storage_scheduler::{StorageError, StorageScheduler, StorageTicket};
pub use timer_wheel::{TimerError, TimerFired, TimerId, TimerWheel, WheelCore};
#[cfg(not(target_arch = "wasm32"))]
pub use fixed_file_storage::{
    RetainedFixedFileDocuments,
    storage_worker_read_fixed_file_document,
    storage_worker_read_fixed_file_page,
    storage_worker_write_fixed_file_page,
};

//#region 🧬️Tests
/// 🪝️ Test hook of the companion crate `semio-framework-os-services-tests`: the private service modules, public only behind the `testkit` feature.
#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod testkit {
    pub mod completion_sink {
        pub use super::super::completion_sink::*;
    }
    pub mod compute_pool {
        pub use super::super::compute_pool::*;
    }
    pub mod event_router {
        pub use super::super::event_router::*;
    }
    pub mod fixed_file_storage {
        pub use super::super::fixed_file_storage::*;
    }
    pub mod host_runtime {
        pub use super::super::host_runtime::*;
    }
    pub mod http_contract {
        pub use super::super::http_contract::*;
    }
    pub mod http_pool {
        pub use super::super::http_pool::*;
    }
    pub mod http_socket {
        pub use super::super::http_socket::*;
    }
    pub mod storage_scheduler {
        pub use super::super::storage_scheduler::*;
    }
    pub mod timer_wheel {
        pub use super::super::timer_wheel::*;
    }
}
//#endregion 🧬️Tests
