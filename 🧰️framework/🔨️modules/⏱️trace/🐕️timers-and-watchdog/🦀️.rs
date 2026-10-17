//! 🐕️ Timers and watchdog of the trace primitives: the step and callback timers, the contract violation records, the callback verdict and the watchdog with its violation ring. See [`super`].

use super::*;

//#region ⏱️Timers
/// ⏱️ RAII guard for a plain latency sample: records elapsed microseconds into `site`'s
/// [`PercentileRing`] when dropped. No operation context and no overrun reporting — that's
/// [`Watchdog`]'s job, layered on the same per-site ring. Use this for a step with no single owning
/// operation (a per-frame simulation tick, a render pass).
pub struct StepTimer {
    pub(super) site: &'static str,
    pub(super) start_us: Option<u64>,
}

impl StepTimer {
    /// ▶️ Starts timing `site` from [`now_us`].
    pub fn start(site: &'static str) -> StepTimer {
        StepTimer { site, start_us: try_now_us() }
    }
}

impl Drop for StepTimer {
    // 🚫️async: E1 external-trait impl — `Drop::drop`'s signature is fixed by std, so this can never
    // `.await`; same reasoning as `CancelToken`'s `Debug::fmt` impl in `⏳️async/🦀️.rs`.
    fn drop(&mut self) {
        if let Some(elapsed) = self.start_us.zip(try_now_us()).and_then(|(start, end)| end.checked_sub(start)) {
            record_site_sample(self.site, elapsed);
        } else {
            OMITTED_SITE_SAMPLES.fetch_add(1, Ordering::Relaxed);
        }
    }
}

/// ⏱️ Same shape as [`StepTimer`], named separately for callback/event-handler call sites (a UI event
/// dispatch, a promise resolution) so a latency report reads by call-site intent instead of forcing
/// every caller to squint at one generic timer type.
// 🔕 dead_code: this field is never read directly — it exists purely for its `Drop` side effect,
// same as any RAII guard whose value nobody inspects.
#[allow(dead_code)]
pub struct CallbackTimer(StepTimer);

impl CallbackTimer {
    pub fn start(site: &'static str) -> CallbackTimer {
        CallbackTimer(StepTimer::start(site))
    }
}
//#endregion ⏱️Timers

//#region 🐕️Watchdog
/// 🚨️ What [`Watchdog::violations`] reports once a wrapped site's elapsed time crosses
/// [`INTERACTIVE_STEP_CEILING_US`] — enough to name the offending site, correlate it back to the
/// operation/generation in flight, and feed it to that session's [`StepOverrunLedger`], which decides
/// whether a run of them is the step's own work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ContractViolation {
    pub site: &'static str,
    pub operation: OperationId,
    pub generation: Generation,
    pub stage: InteractiveStage,
    pub elapsed_us: u64,
}

/// 🕰️ A missing or backward monotonic reading denies callback publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CallbackClockFault {
    Missing,
    Backward,
}

/// 🔒️ Immutable verdict minted only by its exact callback guard, independent of telemetry storage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackVerdict {
    pub(super) site: &'static str,
    pub(super) operation: OperationId,
    pub(super) generation: Generation,
    pub(super) stage: InteractiveStage,
    pub(super) elapsed: Result<u64, CallbackClockFault>,
}

impl CallbackVerdict {
    /// 🔎️ Whether THIS ONE sample breached its clock authority — an unusable reading
    /// ([`CallbackVerdict::clock_fault`]) or an over-ceiling elapsed WALL time
    /// ([`CallbackVerdict::violation`]). A measurement, never on its own a quarantine authority: a
    /// single over-ceiling wall reading is as often the machine descheduling the thread as it is the
    /// step's own work (measured: a 19 µs unit read 14 571 µs on a load-average-90 box, and a
    /// browser worker is descheduled the same way). [`StepOverrunLedger`] is the authority that turns
    /// a run of these into a terminal verdict.
    pub fn is_fault(&self) -> bool {
        self.elapsed.is_err() || self.elapsed.is_ok_and(interactive_step_contract_violated)
    }
    pub fn elapsed_us(&self) -> Option<u64> {
        self.elapsed.ok()
    }
    pub fn clock_fault(&self) -> Option<CallbackClockFault> {
        self.elapsed.err()
    }
    pub fn operation(&self) -> OperationId {
        self.operation
    }
    pub fn generation(&self) -> Generation {
        self.generation
    }
    pub fn violation(&self) -> Option<ContractViolation> {
        self.elapsed.ok().filter(|elapsed| interactive_step_contract_violated(*elapsed)).map(|elapsed_us| ContractViolation { site: self.site, operation: self.operation, generation: self.generation, stage: self.stage, elapsed_us })
    }
}

/// 🚨️ Ring capacity for the process-wide violation store — bounded so a runaway-overrunning site
/// cannot grow this unboundedly; oldest violations are evicted first (see [`Watchdog::violation_count`]
/// for the never-shrinking total).
pub(super) const VIOLATION_RING_CAPACITY: usize = 128;

pub(super) fn violation_ring() -> &'static StaticTryGate<BoundedRing<ContractViolation, VIOLATION_RING_CAPACITY>> {
    static RING: StaticTryGate<BoundedRing<ContractViolation, VIOLATION_RING_CAPACITY>> = StaticTryGate::new(BoundedRing::new());
    &RING
}

/// 🐕️ RAII guard: wraps one [`InteractiveStage`] site for a specific operation/generation, records the
/// elapsed sample into the same per-site [`PercentileRing`] [`StepTimer`] uses, and reports a
/// [`ContractViolation`] at the strict ceiling. [`Watchdog::finish`] transfers the exact verdict;
/// global snapshots are optional diagnostics and must never authorize publication or quarantine.
pub struct Watchdog {
    pub(super) site: &'static str,
    pub(super) operation: OperationId,
    pub(super) generation: Generation,
    pub(super) stage: InteractiveStage,
    pub(super) start_us: Option<u64>,
    pub(super) finished: bool,
}

impl Watchdog {
    pub fn start(site: &'static str, operation: OperationId, generation: Generation, stage: InteractiveStage) -> Watchdog {
        Self::start_at(site, operation, generation, stage, try_now_us())
    }

    /// ⏱️ [`Watchdog::start`] from a reading its caller already made at the site's entry, so a
    /// driver that reads the clock to derive a step budget does not read it a second time.
    pub fn start_at(site: &'static str, operation: OperationId, generation: Generation, stage: InteractiveStage, start_us: Option<u64>) -> Watchdog {
        Watchdog { site, operation, generation, stage, start_us, finished: false }
    }

    pub fn finish(self) -> CallbackVerdict {
        self.finish_at(try_now_us())
    }

    /// 🏁️ [`Watchdog::finish`] at a reading its caller made at the site's exit and reuses.
    pub fn finish_at(mut self, end_us: Option<u64>) -> CallbackVerdict {
        self.finished = true;
        self.report(end_us)
    }

    pub fn is_admitted(&self) -> bool {
        self.start_us.is_some()
    }

    pub(super) fn verdict_at(&self, end_us: Option<u64>) -> CallbackVerdict {
        let elapsed = match (self.start_us, end_us) {
            (Some(start), Some(end)) => end.checked_sub(start).ok_or(CallbackClockFault::Backward),
            _ => Err(CallbackClockFault::Missing),
        };
        CallbackVerdict { site: self.site, operation: self.operation, generation: self.generation, stage: self.stage, elapsed }
    }

    pub(super) fn report(&self, end_us: Option<u64>) -> CallbackVerdict {
        let verdict = self.verdict_at(end_us);
        if let Some(elapsed) = verdict.elapsed_us() {
            record_site_sample(self.site, elapsed);
        }
        if let Some(violation) = verdict.violation() {
            if let Some(mut ring) = violation_ring().try_lock() {
                ring.push(violation);
                RECORDED_VIOLATIONS.fetch_add(1, Ordering::Relaxed);
            } else {
                OMITTED_VIOLATIONS.fetch_add(1, Ordering::Relaxed);
            }
        }
        verdict
    }

    /// 📸️ Optional nonblocking snapshot, oldest first, empty during another telemetry borrow.
    pub fn violations() -> Vec<ContractViolation> {
        violation_ring().try_lock().map_or_else(Vec::new, |ring| ring.snapshot())
    }

    /// 🔢️ Total violations ever recorded — unlike [`Watchdog::violations`], never shrunk by ring
    /// eviction, so a long-running host can still see "how bad has it been" after the ring wraps.
    pub fn violation_count() -> u64 {
        RECORDED_VIOLATIONS.load(Ordering::Relaxed)
    }

    /// 🧹️ Resets the violation store — for test isolation and for a host that has finished quarantining
    /// every reported offender and wants a clean slate.
    pub fn clear() {
        if let Some(mut ring) = violation_ring().try_lock() {
            ring.clear();
            RECORDED_VIOLATIONS.store(0, Ordering::Relaxed);
        }
    }
}

impl Drop for Watchdog {
    // 🚫️async: E1 external-trait impl, same reasoning as `StepTimer::drop` above.
    fn drop(&mut self) {
        if !self.finished {
            self.report(try_now_us());
        }
    }
}
//#endregion 🐕️Watchdog
