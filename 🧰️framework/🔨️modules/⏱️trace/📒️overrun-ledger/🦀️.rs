//! 🕵️ Overrun ledger of the trace primitives: the sustained-overrun quarantine steps, the step quarantine and the step overrun ledger with its counters. See [`super`].

use super::*;

//#region 📒️OverrunLedger
/// 🔁️ How many CONSECUTIVE steps of ONE session must each breach [`INTERACTIVE_STEP_CEILING_US`]
/// before the overrun is attributed to the step's own work instead of to the machine that scheduled
/// it. [`INTERACTIVE_STEP_CEILING_US`] itself is unchanged: it still says what "interactive" means
/// for one step, and every single breach is still recorded ([`Watchdog::violations`],
/// [`step_overrun_counts`]). What changes is who may be *quarantined* for it.
///
/// 🧮️ Why a consecutive count and not thread CPU time: `clock_gettime(CLOCK_THREAD_CPUTIME_ID)`
/// would attribute an overrun exactly, but it exists only off-`wasm32`. `wasi:clocks` publishes a
/// monotonic and a wall clock and NO per-thread CPU clock, and the browser has neither — so a
/// CPU-time law would leave the one target where descheduling is worst (a browser worker sharing a
/// core with ~20 booting WASM plugins) running the very law this constant replaces. Fuel-versus-
/// elapsed was rejected for the same reason in the other direction: fuel is job-declared, and the
/// steps that trip this ceiling are precisely the cheap ones that consume none. A consecutive count
/// needs only the monotonic clock the runtime already requires, so ONE law holds on every target.
///
/// 📏️ Sized at four: a descheduling burst has to survive four separate scheduler decisions in a row
/// to reach it, while a step that genuinely costs more than 8 ms overruns *every* step and is stopped
/// after at most `4 × INTERACTIVE_STEP_CEILING_US` = 32 ms — well inside the frame budget a stuck
/// operation would otherwise burn forever.
pub const SUSTAINED_OVERRUN_QUARANTINE_STEPS: u32 = 4;

static RECORDED_STEP_OVERRUNS: AtomicU64 = AtomicU64::new(0);

static SUSTAINED_OVERRUN_QUARANTINES: AtomicU64 = AtomicU64::new(0);

/// ⚖️ What one [`CallbackVerdict`] means for the session that produced it, once its own
/// [`StepOverrunLedger`] has seen it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepQuarantine {
    Admitted,
    RecordedOverrun { consecutive: u32 },
    SustainedOverrun { consecutive: u32 },
    ClockFault(CallbackClockFault),
}

impl StepQuarantine {
    /// 🛑️ Whether the owning session must be terminated. A single overrun never is; an unusable
    /// clock reading always is, because nothing about that step was measured at all.
    pub fn is_terminal(self) -> bool {
        matches!(self, StepQuarantine::SustainedOverrun { .. } | StepQuarantine::ClockFault(_))
    }
}

/// 📒️ One session's fixed-capacity overrun ledger — four counters, no allocation, `Copy`, usable
/// from any thread and from `wasm32`. Every [`CallbackVerdict`] a session mints is admitted here
/// exactly once, in step order, and the ledger answers whether that session has now overrun
/// [`SUSTAINED_OVERRUN_QUARANTINE_STEPS`] steps in a row. Any admitted step resets the run, so an
/// isolated descheduling spike is recorded and forgotten while a runaway step still quarantines.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StepOverrunLedger {
    pub(super) consecutive: u32,
    longest_run: u32,
    pub(super) total: u32,
    pub(super) worst_elapsed_us: u64,
}

impl StepOverrunLedger {
    pub const fn new() -> StepOverrunLedger {
        StepOverrunLedger { consecutive: 0, longest_run: 0, total: 0, worst_elapsed_us: 0 }
    }

    /// 🏛️ Admits one verdict and returns this session's quarantine authority for it.
    pub fn admit(&mut self, verdict: &CallbackVerdict) -> StepQuarantine {
        if let Some(fault) = verdict.clock_fault() {
            self.consecutive = 0;
            return StepQuarantine::ClockFault(fault);
        }
        let Some(violation) = verdict.violation() else {
            self.consecutive = 0;
            return StepQuarantine::Admitted;
        };
        self.consecutive = self.consecutive.saturating_add(1);
        self.longest_run = self.longest_run.max(self.consecutive);
        self.total = self.total.saturating_add(1);
        self.worst_elapsed_us = self.worst_elapsed_us.max(violation.elapsed_us);
        RECORDED_STEP_OVERRUNS.fetch_add(1, Ordering::Relaxed);
        if self.consecutive < SUSTAINED_OVERRUN_QUARANTINE_STEPS {
            return StepQuarantine::RecordedOverrun { consecutive: self.consecutive };
        }
        SUSTAINED_OVERRUN_QUARANTINES.fetch_add(1, Ordering::Relaxed);
        StepQuarantine::SustainedOverrun { consecutive: self.consecutive }
    }

    pub fn consecutive_overruns(&self) -> u32 {
        self.consecutive
    }

    pub fn longest_overrun_run(&self) -> u32 {
        self.longest_run
    }

    pub fn total_overruns(&self) -> u32 {
        self.total
    }

    pub fn worst_elapsed_us(&self) -> u64 {
        self.worst_elapsed_us
    }
}

/// 📊️ Process-wide `(recorded step overruns, sustained-overrun quarantines)` — the second number is
/// how often an overrun was ever attributed to a step rather than to the machine.
pub fn step_overrun_counts() -> (u64, u64) {
    (RECORDED_STEP_OVERRUNS.load(Ordering::Relaxed), SUSTAINED_OVERRUN_QUARANTINES.load(Ordering::Relaxed))
}
//#endregion 📒️OverrunLedger
