//! ⏱️ Interactivity tracing/observability primitives for the Semio interactive job runtime: latency
//! instrumentation ([`StepTimer`]/[`CallbackTimer`] RAII guards, a hard [`INTERACTIVE_STEP_CEILING_US`]
//! plus per-family soft targets, a [`Watchdog`] that reports [`ContractViolation`]s on overrun, and a
//! [`PercentileRing`] per labelled site), cheap process-wide thread-role identity
//! ([`ThreadRole::Ui`]/[`ThreadRole::Worker`]/[`ThreadRole::IoBoundary`]), lock-free active-worker/permit/queue counters
//! ([`WorkerCounters`]/[`PermitLedger`]/[`QueueCounter`]), and a bounded operation-[`TraceEvent`] ring
//! (started/stage-changed/preview-published/checkpoint/commit/cancelled/failed) with preview- and
//! cancellation-latency queries. Every later packet of INTERACTIVE-JOB-RUNTIME-REFACTOR (design ticket
//! `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR`, this is packet P0a) depends on this module to answer
//! "did this stay interactive" instead of instrumenting its own ad hoc timers.
//!
//! 🧬 **Zero dependencies**: pure `std`, no `serde`/`thiserror`/`owned schema exporter`/anything else — this is a leaf
//! instrumentation primitive every other crate may end up depending on, so it must never widen anyone
//! else's dependency graph.
//!
//! 🕰️ **Clock**: `std::time::Instant` doesn't exist (usably) on `wasm32-unknown-unknown` without WASI
//! p2. The embedding host installs one actual monotonic source shared by tracing and job deadlines.
//! Missing or backward readings produce an exact callback fault, never synthetic time. Telemetry is
//! optional and nonblocking; [`CallbackVerdict`] is the caller-owned MEASUREMENT of one step and
//! [`StepOverrunLedger`] the caller-owned quarantine authority over a run of them, because the only
//! clock any target guarantees measures WALL time and a single over-ceiling wall reading says as much
//! about the machine's scheduler as about the step.
//!
//! 🚫️async: this crate deliberately writes NO `async fn` anywhere, breaking with the rest of the
//! repo's universal-async convention (see `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/
//! MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📌️important.md`'s R9/E1/E5 exception classes). Every public
//! fn here is either reached from `Drop::drop` (a fixed sync external-trait signature — the same E1
//! class as `CancelToken`'s `Debug` impl and `ActorMetrics::wall_us_p95` in `⏳️async`/`🎭️actor`'s
//! `🦀️.rs`) or is itself a cheap hot-path identity/atomic primitive meant to be callable from
//! ANY context, including one with no executor running yet (the same R9 "pure, zero-I/O, sync-only
//! consumer" class, generalized: here the consumer is "arbitrary caller", not one specific closure).
//! Wrapping either shape in `async fn` would force every caller to own an executor just to check "am I
//! the UI thread" or drop a timer, defeating this module's entire purpose as a zero-overhead,
//! always-available probe.

use std::cell::Cell;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::OnceLock;

#[path = "🕰️clock/🦀️.rs"]
mod clock;
pub use clock::*;

#[path = "📏️ceilings-and-diagnostics/🦀️.rs"]
mod ceilings_and_diagnostics;
pub use ceilings_and_diagnostics::*;

#[path = "📊️sample-rings-and-sites/🦀️.rs"]
mod sample_rings_and_sites;
pub use sample_rings_and_sites::*;

#[path = "🐕️timers-and-watchdog/🦀️.rs"]
mod timers_and_watchdog;
pub use timers_and_watchdog::*;

#[path = "📒️overrun-ledger/🦀️.rs"]
mod overrun_ledger;
pub use overrun_ledger::*;

#[path = "🧵️thread-roles-and-counters/🦀️.rs"]
mod thread_roles_and_counters;
pub use thread_roles_and_counters::*;

#[path = "🛰️operations-and-trace/🦀️.rs"]
mod operations_and_trace;
pub use operations_and_trace::*;

//#region 🧮️GuestMemory
#[path = "🧮️memory/🦀️.rs"]
mod guest_memory;
pub use guest_memory::{
    guest_host_answer_pages, guest_linear_memory_bytes, guest_linear_memory_install_peak_ceiling_bytes, guest_linear_memory_percent, peak_heap_bytes, peak_heap_bytes_on_this_thread, reset_heap_peak, reset_heap_peak_on_this_thread,
    observe_heap_allocations_on_this_thread, heap_allocation_observation_on_this_thread, HeapAllocationObservation,
    retained_heap_bytes, retained_heap_bytes_on_this_thread, HeapWitness, GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, GUEST_HOST_ANSWER_CEILING_BYTES, GUEST_LINEAR_MEMORY_INSTALL_PEAK_PERCENT, GUEST_LINEAR_MEMORY_MAXIMUM_BYTES,
    GUEST_LINEAR_MEMORY_STACK_BYTES,
};
//#endregion 🧮️GuestMemory

//#region 📝️SpanRecord
/// 📝️ Structured request-scoped span records with an injectable sink — the server half of this
/// module's observability budget, kept in its own file because it is the only part that ever
/// formats or writes anything. See `📝️record/🦀️.rs`.
#[path = "📝️record/🦀️.rs"]
pub mod record;
//#endregion 📝️SpanRecord

//#region 🐕️Watchdog

#[path = "⏱️clock/🏁️tail/🦀️.rs"]
mod watchdog_tail;
pub use watchdog_tail::WatchdogAdmission;
//#endregion 🐕️Watchdog

//#region 🧪️Tests
#[cfg(test)]
/// 🚧️ Serializes every test that reads, holds or counts the process-wide telemetry rings, which share one static registry.
pub(crate) fn telemetry_test_gate() -> std::sync::MutexGuard<'static, ()> {
    static GATE: std::sync::Mutex<()> = std::sync::Mutex::new(());
    GATE.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[cfg(test)]
#[path = "⏱️clock/🧪️tests/🔬️standalone/🦀️.rs"]
mod microsecond_clock_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "⏱️clock/🧪️tests/🧪️contention/🦀️.rs"]
mod microsecond_telemetry_contention_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "⏱️clock/🏁️tail/🧪️tests/🏁️tail/🦀️.rs"]
mod watchdog_tail_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path="📈️percentile/🔐️gate/🦀️.rs"]
mod static_telemetry_gate;
use static_telemetry_gate::StaticTryGate;
