//! ⏱️ Measures the actual actor's running time across the bounded execution clock.
use std::cell::Cell;

/// ⏱️ One turn's EXECUTING microseconds — wall time spent inside the guest's own `Future::poll`
/// calls, with every `Pending` gap excluded. The strict lifecycle authority
/// ([`semio_framework_trace::GUEST_LIFECYCLE_TURN_CEILING_US`]) must bound the guest's own work; a
/// plain `now_us() - started_us` also bills the guest for time it is not running at all — a JSPI
/// suspension parked on a host import, a hidden browser tab whose timer/microtask continuation is
/// throttled, or an OS deschedule of a loaded box. Measured 2026-09-10: generation3d's whole
/// `InstanceOpen` turn executes in 13.9 ms natively, yet its browser first step was killed at the
/// 5 s ceiling, because the wall clock kept running across the descheduled continuations.
#[derive(Clone, Copy)]
struct TurnExecution {
    accumulated_us: u64,
    poll_started_us: Option<u64>,
    clock_lost: bool,
    clock: fn() -> Option<u64>,
}

const TURN_EXECUTION_IDLE: TurnExecution = TurnExecution { accumulated_us: 0, poll_started_us: None, clock_lost: false, clock: semio_framework_job::default_now_us };

thread_local! {
    static TURN_EXECUTION: Cell<TurnExecution> = const { Cell::new(TURN_EXECUTION_IDLE) };
}

/// ⏱️ Executing microseconds of the turn in flight, including the poll currently on the stack.
/// `None` means the host clock was missing or ran backward, which the lifecycle authority retains
/// its receipt for exactly like an overrun.
#[doc(hidden)]
pub fn guest_turn_executing_us() -> Option<u64> {
    let execution = TURN_EXECUTION.with(Cell::get);
    if execution.clock_lost {
        return None;
    }
    let Some(poll_started_us) = execution.poll_started_us else { return Some(execution.accumulated_us) };
    Some(execution.accumulated_us.saturating_add((execution.clock)()?.checked_sub(poll_started_us)?))
}

/// ⏱️ Runs one turn under [`TURN_EXECUTION`] accounting; the gaps between polls never accrue.
///
/// 📏️ Takes the turn future ALREADY PINNED, by pointer. Taking it `impl Future` by value cost an
/// exact duplicate of the largest generator in the guest: a parameter owns a coroutine slot from
/// `Unresumed` onward, `pin!` then moved it into a second local that lives across the `.await`, and
/// the moved-from parameter slot is never reused — `with_turn_execution` measured 243 184 B around
/// a 121 584 B `poll_kernel_turn`. `Pin<&mut …>` is 8 bytes and the caller owns the one slot. See
/// `📓️poll-task-leak-2026-09-10.md` §3.1.
#[doc(hidden)]
pub async fn with_turn_execution<T>(future: std::pin::Pin<&mut (impl std::future::Future<Output = T> + ?Sized)>) -> T {
    with_turn_execution_clock(future, semio_framework_job::default_now_us).await
}

/// 🎛️ Measures using the actual executor supplied for this turn.
#[doc(hidden)]
pub async fn with_turn_execution_clock<T>(mut future: std::pin::Pin<&mut (impl std::future::Future<Output = T> + ?Sized)>, clock: fn() -> Option<u64>) -> T {
    TURN_EXECUTION.with(|execution| execution.set(TurnExecution { clock, ..TURN_EXECUTION_IDLE }));
    std::future::poll_fn(move |context| {
        TURN_EXECUTION.with(|execution| {
            let mut current = execution.get();
            current.poll_started_us = (current.clock)();
            current.clock_lost = current.clock_lost || current.poll_started_us.is_none();
            execution.set(current);
        });
        let polled = future.as_mut().poll(context);
        TURN_EXECUTION.with(|execution| {
            let mut current = execution.get();
            match current.poll_started_us.take().zip((current.clock)()).map(|(started_us, now_us)| now_us.checked_sub(started_us)) {
                Some(Some(spent_us)) => current.accumulated_us = current.accumulated_us.saturating_add(spent_us),
                _ => current.clock_lost = true,
            }
            execution.set(current);
        });
        polled
    })
    .await
}
