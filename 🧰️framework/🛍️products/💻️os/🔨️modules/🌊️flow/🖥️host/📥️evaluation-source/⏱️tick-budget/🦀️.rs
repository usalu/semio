//! ⏱️ Dispatch and elapsed-time limits for a Flow evaluation turn.

use neural_engine::EvalStepBudget;

/// ⏱️ Max cache-missed neuron dispatches per `flowEvalTick` — keeps one dispatch from blocking the worker while still converging small graphs in a single tick.
pub const FLOW_EVAL_TICK_STEP_BUDGET: usize = 512;

/// ⏱️ Wall-clock ceiling for ONE `flowEvalTick` dag walk, in microseconds.
///
/// 🚨️ [`FLOW_EVAL_TICK_STEP_BUDGET`] alone cannot bound a tick: it counts NODES, and a seven-node
/// graph is far under 512 however expensive each node is. The reactor's own 8 ms budget cannot
/// bound it either — `run_until_deadline` checks its deadline BETWEEN two tasks' `step()` calls,
/// never inside one, and `flowEvalTick` is a plain synchronous `fn`, so once the executor enters it
/// nothing can interrupt it. That is how one browser turn ran for 3.5-18.4 s against an 8 ms budget
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️audit-guest-tick-cost-2026-09-12.md` §1.3).
///
/// 🎚️ Sized at three quarters of [`semio_framework_job::INTERACTIVE_STEP_CEILING_US`] so the rest of
/// the turn — publication, retirement, the effect the tick returns — still fits the same 8 ms the
/// dag walk is being held to, and so a tick that yields here has not ALREADY broken the contract it
/// exists to keep. A walk always dispatches at least one node before the deadline can stop it
/// ([`EvalStepBudget::exhausted`]), so a graph whose single cheapest node exceeds the
/// ceiling still converges — one node per tick — instead of re-arming forever with no progress.
pub const FLOW_EVAL_TICK_ELAPSED_CEILING_US: u64 = semio_framework_job::INTERACTIVE_STEP_CEILING_US / 4 * 3;

/// ⏱️ Wall a guest turn must still have UNSPENT of [`FLOW_EVAL_TICK_ELAPSED_CEILING_US`] before a
/// fold may run the next wave of its chain inline rather than park it for a host round trip.
///
/// 🔁️ A fold that continues inline saves a whole `flowEvalResolve` → `flowEvalTick` pair — measured
/// at 1 391 ms on React (`📓️react-hop-latency-2026-09-14.md` §2.1) — but it spends that saving
/// inside the answer's OWN turn, on top of whatever the fold already cost. Without a reserve the
/// two additions are exactly how an 8 ms hold becomes a 14 ms one. A third of the allowance is the
/// floor a walk needs to be worth starting at all: `EvalStepBudget::exhausted` always dispatches at
/// least one node before a deadline can stop it, so a continuation admitted with a sliver left
/// would overrun by one whole operator rather than yield (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub const FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US: u64 = FLOW_EVAL_TICK_ELAPSED_CEILING_US / 3;

/// ⏱️ The budget ONE `flowEvalTick` dag walk runs under: [`FLOW_EVAL_TICK_STEP_BUDGET`] nodes,
/// preempted after [`FLOW_EVAL_TICK_ELAPSED_CEILING_US`] on the process clock. Falls back to the
/// node count alone when no clock is installed (bare wasm), which is the pre-existing behaviour.
///
/// 🔁️ `turn_started_us` is when the GUEST TURN this walk belongs to began. A walk that opens its own
/// turn passes `None` and gets the whole allowance; one running inline inside a fold's turn passes
/// that turn's start and shares the SAME deadline, which is the entire reason an inline continuation
/// cannot stretch the interactive hold however many waves it chains.
pub fn flow_eval_tick_budget(turn_started_us: Option<u64>) -> EvalStepBudget {
    let started_us = turn_started_us.or_else(semio_framework_job::default_now_us);
    match started_us {
        Some(started_us) => EvalStepBudget::until(FLOW_EVAL_TICK_STEP_BUDGET, semio_framework_job::default_now_us, started_us.saturating_add(FLOW_EVAL_TICK_ELAPSED_CEILING_US)),
        None => EvalStepBudget::dispatches(FLOW_EVAL_TICK_STEP_BUDGET),
    }
}

/// ⏱️ Whether a turn that began at `turn_started_us` and is now at `now_us` still has
/// [`FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US`] of its evaluation allowance left — the WALL half of
/// `FlowEvalSession::inline_continuation_admitted`, pure in both instants so a law can state the
/// park boundary in microseconds instead of racing a clock.
///
/// 🕰️ An uninstrumented process (no clock at all, bare wasm) admits the continuation: the node
/// budget still bounds the walk, and that is precisely the fallback [`flow_eval_tick_budget`] takes.
pub fn flow_eval_inline_continuation_fits(turn_started_us: Option<u64>, now_us: Option<u64>) -> bool {
    match (turn_started_us, now_us) {
        (Some(turn_started_us), Some(now_us)) => now_us.saturating_sub(turn_started_us).saturating_add(FLOW_EVAL_INLINE_CONTINUATION_RESERVE_US) <= FLOW_EVAL_TICK_ELAPSED_CEILING_US,
        _ => true,
    }
}
