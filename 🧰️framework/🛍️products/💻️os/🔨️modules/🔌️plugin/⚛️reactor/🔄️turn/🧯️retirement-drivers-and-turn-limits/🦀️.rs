//! 🧯️ Retirement drivers and turn limits of the reactor turn: the retire-until-complete drivers and the close, patch, executor hold and measured guest turn limits.

/// 🏃️ Upper bound on process-pool pumps per reactor turn on wasm.
/// 🧹️ Drives one per-turn retirement ladder entry for a bounded RUN of units instead of the single
/// unit it used to get, stopping as soon as the entry reports itself complete, its run is spent, or
/// the turn's own wall-clock budget is gone.
///
/// A whole-document swap fills every one of these arenas with DOCUMENT-scaled garbage (the replaced
/// window bodies, their patches, their built-node pages), and each `…_one()` retires one item. At one
/// unit per turn that is one host↔guest round trip per retired item, because the same turn answers
/// `MoreWork` while any of it is outstanding — measured 2026-09-10 (ticket 26/09/02 W-S2) as 8 799
/// worker messages and 24.3 s for the 180-object Nakagin switch, of which only 5.4 s was main-thread
/// work. Retirement is bounded work per TURN, not per item.
// 🚫️async: E1 pure bounded retirement driver called from the turn's own close ladder — see R9.
pub fn retire_until_complete(cx:&mut semio_framework_job::StepContext<'_>, mut unit: impl FnMut() -> bool) {
    for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
        if cx.should_yield(){return}
        cx.consume_fuel(1);
        if unit() {
            return;
        }
    }
}

/// 🧹️ [`retire_until_complete`] for a ladder entry that reports PROGRESS rather than completion.
// 🚫️async: E1 pure bounded retirement driver called from the turn's own close ladder — see R9.
pub fn retire_while_progress(cx:&mut semio_framework_job::StepContext<'_>, mut unit: impl FnMut() -> bool) {
    for _ in 0..PATCH_CLOSE_UNITS_PER_TURN {
        if cx.should_yield(){return}
        cx.consume_fuel(1);
        if !unit() {
            return;
        }
    }
}

/// 🧹️ [`retire_while_progress`] with its own unit ceiling — for a ladder whose fixed slot geometry is
/// wider than the patch ladder's 256 units.
// 🚫️async: E1 pure bounded retirement driver called from the turn's own close ladder — see R9.
pub fn retire_while_progress_bounded(cx:&mut semio_framework_job::StepContext<'_>, units: usize, mut unit: impl FnMut() -> bool) {
    for _ in 0..units {
        if cx.should_yield(){return ;}
        cx.consume_fuel(1);
        if !unit() {
            return;
        }
    }
}

/// 🧹️ [`retire_while_progress_bounded`] for a ladder unit that may fault — the fault propagates out
/// of the turn instead of being swallowed by the driver.
// 🚫️async: E1 pure bounded retirement driver called from the turn's own close ladder — see R9.
pub fn retire_while_progress_fallible(cx:&mut semio_framework_job::StepContext<'_>, units: usize, mut unit: impl FnMut(&mut semio_framework_job::StepContext<'_>) -> Result<bool, semio_framework_job::OriginalTurnFailure>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
    for _ in 0..units {
        if cx.should_yield(){return Ok(())}
        cx.consume_fuel(1);
        if !unit(cx)? {
            return Ok(());
        }
    }
    Ok(())
}

/// 🚪️ Instance-close ladder units per reactor turn. The reactor close walks FIXED slot geometry —
/// `REACTOR_TASK_SLOTS` task slots, the request registry's own slot array, the armed-timer
/// array — one slot per unit, and until 2026-09-12 it took exactly ONE of those units per reactor
/// turn. Measured (`🧫️fixtures/🚪️close-ladder`, ticket 26/09/09): a generation3d editor reached
/// `Retired` after **2 052** close turns whether the session had loaded zero documents or eight with
/// thirty-two preview renders — the cost is the geometry, not the retained data. In the browser each
/// of those turns is one worker round trip, so the ⌘⌥V role switch spent 62–87 s inside
/// `retireInstanceLifecycle` and then failed `plugin-ui.lifecycle-close-budget-exhausted`. Driving the
/// whole geometry in ONE turn, cut short by the turn's own wall-clock budget every
/// [`PATCH_CLOSE_DEADLINE_STRIDE`] units, makes a close cost a bounded handful of round trips — the
/// same correction [`PATCH_CLOSE_UNITS_PER_TURN`] applied to the patch ladder.
pub const REACTOR_CLOSE_UNITS_PER_TURN: usize = 4_096;

/// 🧹️ Retained terminal retirement units per reactor turn, bounded so one turn stays inside its
/// interactive ceiling — [`PATCH_CLOSE_DEADLINE_STRIDE`] cuts the run short on the turn's own
/// wall-clock budget, exactly like the reconcile loop above it.
///
/// This is NOT background cleanup that a slow drip can be indifferent to: an acknowledged publication
/// keeps `PendingPatchAuthority::has_retiring` — and therefore the whole turn — in `MoreWork` until
/// it is retired, and the host answers every `MoreWork` with another turn ROUND TRIP. One 180-object
/// world-3d publication takes 1 092 retirement units; at the previous 8 units per turn that is 137
/// round trips per published surface, and it is why the Nakagin example switch took 24.3 s and 8 799
/// worker messages with only 5.4 s of main-thread work in it (measured 2026-09-10, ticket 26/09/02
/// W-S2). At 256 it is 5. The turn stays interactive because [`PATCH_CLOSE_DEADLINE_STRIDE`] re-reads
/// the clock every 8 units against the turn's own budget — the unit count is the ceiling, the
/// deadline is the bound.
pub const PATCH_CLOSE_UNITS_PER_TURN: usize = 256;

/// ⏱️ How often the retirement run re-reads the clock — the same 64-opportunity stride the reconcile
/// loop uses, scaled to this shorter run.
const PATCH_CLOSE_DEADLINE_STRIDE: usize = 8;

/// 🧹️ Items one retirement unit may retire. Priced per PAGE like every other stage of the turn
/// (`SURFACE_RECONCILE_PAGE_BYTES` for bytes, this for items) instead of per item, so a
/// document-scaled patch retires in a bounded handful of turns instead of one turn per item.
pub const PATCH_RETIREMENT_ITEMS_PER_UNIT: usize = 1_024;

/// 🧹️ Bytes one retirement unit may retire — the publication's own page budget.
pub const PATCH_RETIREMENT_BYTES_PER_UNIT: usize = semio_framework_ui_runtime::SURFACE_RECONCILE_PAGE_BYTES;

/// ⏱️ The reactor's OWN slice of one turn: the wall the executor may spend running ready tasks before
/// it hands control back to whoever polled it. It is the reactor's slice, not the turn's — the turn's
/// own wall is `semio_framework::kernel::Budget::deadline_us`, which the reconcile drive and the
/// retirement passes read.
///
/// 🐛️ It was a literal `8` at its only call site, and the browser worker's silent-turn hold copied
/// that literal as its OWN wall budget (`📓️reactor-reconcile-spin-2026-09-14.md` §3.5). That was the
/// incoherence: one whole guest turn measures [`MEASURED_GUEST_TURN_COST_MS`] on the procedural 3d
/// React door — the executor slice plus everything else a turn does — so a drive bounded by this
/// number as a WALL admits `floor(hold / cost) = 0` further turns and degenerates into "one extra
/// poll". A drive is therefore bounded by a STEP CEILING derived from the measured cost
/// (`more_work_drive_steps`), and this constant stays what it always was: the reactor's slice.
pub const REACTOR_TURN_EXECUTOR_HOLD_MS: u64 = 8;

/// ⏱️ What one whole reactor turn costs on the renderer this budget was measured against — the
/// `worker.guest` mean of `🐍️react-hop-cost-probe.mjs` over a 10-step run of the procedural 3d React
/// door (12.9 ms over 1 154 crossings, 2026-09-15; 12.8 ms over 1 165 the day before). It is a
/// MEASUREMENT carried as a declaration, so the derivation below is a law rather than a guess, and it
/// is the number to re-measure when the guest's own turn cost moves.
pub const MEASURED_GUEST_TURN_COST_MS: u64 = 13;
