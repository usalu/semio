//! 🛣️ Scheduling vocabulary: `ProcessKind`, the six weighted `Lane`s and the `PermitLedger`.

use std::sync::atomic::{AtomicU32, Ordering};

//#region 🏭️ProcessKind
/// 🏭️ Which sizing rule [`worker_count_for`] applies. Made explicit at construction — never
/// inferred by guessing whether a UI is present — because a headless batch process (CLI conversion,
/// CI worker) wants every core for throughput, while an interactive native process must always
/// leave the OS/UI thread its own core. No `Serialize`/`Deserialize`: same repo-wide
/// zero-call-site finding as [`CancelState`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProcessKind {
    InteractiveNative,
    HeadlessBatch,
}

/// 🔢️ `max(1, cores-1)` for [`ProcessKind::InteractiveNative`] (the OS/UI thread keeps the
/// remaining logical core), `cores` for [`ProcessKind::HeadlessBatch`]. Never zero — a single-core
/// interactive host still gets one worker (see [`WorkerPool`]'s doc for how that worker stays
/// cooperative rather than starving the OS/UI thread).
pub fn worker_count_for(process_kind: ProcessKind, cores: usize) -> usize {
    let cores = cores.max(1);
    match process_kind {
        ProcessKind::InteractiveNative => cores.saturating_sub(1).max(1),
        ProcessKind::HeadlessBatch => cores,
    }
}
//#endregion 🏭️ProcessKind

//#region 🛣️Lane
/// 🧮️ How many [`Lane`] variants exist — the fixed width of every per-worker queue array.
pub(super) const LANE_COUNT: usize = 6;

/// 🧵️ Logical scheduling lane on [`WorkerPool`], replacing the old per-role OS threads
/// (`ThreadPlan`'s `kernel`/`shards`/`io_workers`/`compute`/`epoch_ticker`, deleted in this packet).
/// `Interactive`/`UserVisible`/`Background`/`Maintenance` mirror `🎭️actor::Lane`'s discriminant
/// order and [`Lane::weight`] verbatim (this crate must not depend on the actor crate — see
/// [`OperationContext::lane`]'s doc — so the mirroring is a deliberate, documented duplication, not
/// a shared type); `Io` and `Timer` are new lanes for the OS threads Phase 0's census found with no
/// actor-crate analogue (HTTP fetch threads, DB storage blocking I/O, the epoch-ticker replacement).
/// No shipped `Serialize`/`Deserialize`: same repo-wide zero-production-call-site finding as
/// [`CancelState`] — the one exception is `Deserialize`, kept behind the `wasm-pool-host` feature because
/// `🤝️cooperative`'s fixture-driven suite parses `Lane` out of a JSON test corpus; `serde` is an
/// optional dependency of that feature only, so this never reaches the `wasm32-wasip2` guest link path.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "wasm-pool-host", derive(serde::Deserialize))]
pub enum Lane {
    Interactive,
    UserVisible,
    Background,
    Maintenance,
    Io,
    Timer,
}

impl Lane {
    /// 🗂️ Every lane, in a fixed order used both for per-worker queue storage indexing (`as usize`)
    /// and as the deficit-round-robin scan order — DRR fairness does not depend on this order being
    /// priority-sorted, only on it being STABLE across scans.
    pub const ALL: [Lane; LANE_COUNT] = [Lane::Interactive, Lane::UserVisible, Lane::Background, Lane::Maintenance, Lane::Io, Lane::Timer];

    /// ⚖️ Deficit-round-robin service weight — verbatim `🎭️actor::Lane::weight` for the four
    /// mirrored variants (8/4/2/1); `Io`=4 (as latency-sensitive as `UserVisible` — a stalled I/O
    /// lane stalls whatever's awaiting it) and `Timer`=3 (between `UserVisible` and `Background` —
    /// prompt but not as urgent as directly-observed interactive/user-visible work). A lane with
    /// pending work but insufficient accrued deficit is skipped THIS round, not starved forever —
    /// see [`WorkerPool`]'s scheduler doc for the accrual rule.
    pub const fn weight(self) -> u32 {
        match self {
            Lane::Interactive => 8,
            Lane::UserVisible => 4,
            Lane::Io => 4,
            Lane::Timer => 3,
            Lane::Background => 2,
            Lane::Maintenance => 1,
        }
    }

    /// 🗄️ Storage index into a `[T; LANE_COUNT]` per-worker queue array.
    #[doc(hidden)] pub const fn index(self) -> usize {
        self as usize
    }

    /// 🔄️ Maps `OperationContext.lane`'s bare `u8` (mirroring `🎭️actor::Lane`'s 0..3 discriminant
    /// order) onto this crate's [`Lane`]. Values outside `0..=3` (this crate's `Io`/`Timer`, which
    /// have no `🎭️actor::Lane` analogue) fall back to [`Lane::Background`] — a caller that wants
    /// `Io`/`Timer` submits to [`WorkerPool::submit`] with that [`Lane`] directly rather than via an
    /// `OperationContext`.
    pub const fn from_context_lane(value: u8) -> Lane {
        match value {
            0 => Lane::Interactive,
            1 => Lane::UserVisible,
            2 => Lane::Background,
            3 => Lane::Maintenance,
            _ => Lane::Background,
        }
    }
}

/// 🚦️ Whether `lane` is subject to [`WorkerPool`]'s interactive-reserve admission control —
/// `Background`/`Maintenance` are the two lanes that may never occupy every worker while the reserve
/// is active. `Io`/`Timer` are deliberately excluded: a stalled I/O or timer lane can itself block
/// interactive work waiting on it, so throttling them would be self-defeating. Native-only: the
/// wasm cooperative pool has exactly one logical worker, where admission control is a documented
/// no-op (see [`WorkerPool::pump`]'s doc), so this fn would otherwise be dead code on that target.
#[cfg(not(target_arch = "wasm32"))]
pub(super) const fn is_low_priority(lane: Lane) -> bool {
    matches!(lane, Lane::Background | Lane::Maintenance)
}
//#endregion 🛣️Lane

//#region ⚖️PermitLedger
/// 🚫️ What [`PermitLedger::checkout`] returns on over-allocation — CHECKED in every build
/// (including release): the counter never wraps. Replaces `ThreadBudget::checkout`'s release-mode
/// `debug_assert!`-only tripwire (Phase 0 gate report finding: `previous.wrapping_sub(n)` silently
/// wraps to a huge value in release), this packet's explicit exit-gate item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PermitError {
    pub requested: u32,
    pub remaining: u32,
}

impl std::fmt::Display for PermitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PermitLedger over-allocated: requested {} permits but only {} remained", self.requested, self.remaining)
    }
}

impl std::error::Error for PermitError {}

/// 📒️ Checked permit accounting, replacing `ThreadBudget`. Unlike the old per-role `ThreadBudget`
/// (five separate counters, one per OS-thread role), a [`PermitLedger`] tracks ONE pool of permits —
/// [`WorkerPool`] owns one sized to its worker count, and any other subsystem that wants a bounded
/// concurrent-admission gate (a future I/O connection pool, a plugin blocking-call limiter) can size
/// its own independently. [`PermitLedger::checkout`] uses a checked compare-exchange loop — never a
/// bare `fetch_sub` — so over-allocation is always observable as `Err`, in every build profile.
/// [`PermitLedger::occupancy`] feeds `semio_framework_trace::PermitLedger` so the trace module can
/// read live occupancy without this crate depending on trace's internals beyond that one type.
#[derive(Debug)]
pub struct PermitLedger {
    pub(super) remaining: AtomicU32,
    trace_permits: semio_framework_trace::PermitLedger,
}

impl PermitLedger {
    pub fn new(total: u32) -> PermitLedger {
        PermitLedger { remaining: AtomicU32::new(total), trace_permits: semio_framework_trace::PermitLedger::new() }
    }

    /// ✏️ Draws `n` permits, returning a [`PermitGuard`] that returns them on drop, or `Err` if
    /// fewer than `n` remain. A checked compare-exchange loop: a release build gets the exact same
    /// `Err` a debug build would, never a silently wrapped counter.
    pub fn checkout(&self, n: u32) -> Result<PermitGuard<'_>, PermitError> {
        loop {
            let current = self.remaining.load(Ordering::SeqCst);
            if n > current {
                return Err(PermitError { requested: n, remaining: current });
            }
            let next = current - n;
            if self.remaining.compare_exchange(current, next, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                for _ in 0..n {
                    self.trace_permits.acquire();
                }
                return Ok(PermitGuard { ledger: self, n });
            }
        }
    }

    pub fn remaining(&self) -> u32 {
        self.remaining.load(Ordering::SeqCst)
    }

    /// 📊️ Live occupancy — how many permits are currently checked out. Backed by
    /// `semio_framework_trace::PermitLedger`, so anything already reading that trace type sees the
    /// same number.
    pub fn occupancy(&self) -> u32 {
        self.trace_permits.occupancy()
    }
}

/// 🔒️ RAII guard returned by [`PermitLedger::checkout`] — returns its `n` permits to the ledger
/// (and releases the matching trace occupancy) on drop, whether that drop is an ordinary scope exit
/// or unwinding past a panic.
#[derive(Debug)]
pub struct PermitGuard<'a> {
    pub(super) ledger: &'a PermitLedger,
    n: u32,
}

impl Drop for PermitGuard<'_> {
    fn drop(&mut self) {
        self.ledger.remaining.fetch_add(self.n, Ordering::SeqCst);
        for _ in 0..self.n {
            self.ledger.trace_permits.release();
        }
    }
}
//#endregion ⚖️PermitLedger
