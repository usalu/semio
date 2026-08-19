//! 🧪️ terra-async-harness-spike (Q1-Q6). Guest half of a REDUCED, host-controlled copy of real
//! `world actor`'s `reactor`/`jobs`/`checkpoint` exports + a `pure`/`hostasync` import pair — see
//! `🧬️schema/📜️world.wit`'s own doc for exactly what was reduced and why.

wit_bindgen::generate!({
    path: "🧬️schema/📜️world.wit",
    world: "turnharness",
});

use exports::semio::turnharness::checkpoint::Guest as CheckpointGuest;
use exports::semio::turnharness::jobs::{Guest as JobsGuest, JobBudget, JobStep};
use exports::semio::turnharness::reactor::{Budget, Event, Guest as ReactorGuest, TurnResult, TurnStatus};
use semio::turnharness::hostasync;
use semio::turnharness::pure;

struct Component;

impl ReactorGuest for Component {
    async fn poll(events: Vec<Event>, _budget: Budget) -> Result<TurnResult, String> {
        let mut status = TurnStatus::Idle;
        for event in events {
            match event {
                Event::Tick => {}
                // 🧪️ Q1: the decisive case — a HOST-implemented async import awaited MID-`poll`.
                Event::AwaitSignal(id) => {
                    let value = hostasync::wait_signal(id).await;
                    pure::log("info".to_string(), format!("poll: wait-signal({id}) resolved = {value}")).await;
                    status = TurnStatus::MoreWork;
                }
                // 🧪️ Q2: `hang` never resolves on its own — this branch is only ever entered by a
                // test that deliberately drops the call/Store before it could complete.
                Event::AwaitHang(id) => {
                    let value = hostasync::hang(id).await;
                    pure::log("info".to_string(), format!("poll: hang({id}) unexpectedly resolved = {value}")).await;
                }
                // 🧪️ Q3/Q5: pure CPU-bound loop, ZERO host-import calls — S1c's own isolation
                // discipline, `black_box`'d per iteration so LLVM cannot strength-reduce the loop to
                // a closed-form sum (S1c's own self-inflicted confound, avoided here from the start).
                Event::Burn(iters) => {
                    let mut acc: u64 = 0;
                    let mut i: u64 = 0;
                    while i < iters {
                        let step = std::hint::black_box(i).wrapping_mul(2654435761);
                        acc = std::hint::black_box(acc.wrapping_add(step));
                        i = i.wrapping_add(1);
                    }
                    pure::log("info".to_string(), format!("poll: burn({iters}) acc={acc}")).await;
                    status = TurnStatus::MoreWork;
                }
            }
        }
        // 🚧️ `fuel-used` is ALWAYS 0 here, by design — the guest cannot observe its own fuel
        // consumption from inside the WIT ABI; the real `synthesize_turn_result` (⏳️runtime.rs)
        // never trusts a guest-reported value either, it computes `fuel_before - fuel_after` from
        // the Store's own fuel counter host-side. Mirrored here, not an oversight.
        Ok(TurnResult { status, fuel_used: 0 })
    }
}

impl JobsGuest for Component {
    async fn start_job(_job: u64, _kind: String, _input: u32) -> Result<(), String> {
        Ok(())
    }

    // 🧪️ Q4: the host calls this WHILE `poll` may be concurrently suspended on the same instance
    // (via `hostasync::hang`) — logs so host stdout proves the call actually reached the guest.
    async fn step_job(job: u64, _budget: JobBudget) -> Result<JobStep, String> {
        pure::log("info".to_string(), format!("step-job({job}) called")).await;
        Ok(JobStep::Done(job as u32))
    }

    async fn cancel_job(_job: u64) {}
}

impl CheckpointGuest for Component {
    async fn checkpoint() -> Vec<u8> {
        vec![1, 2, 3]
    }

    async fn restore(_state: Vec<u8>) -> Result<(), String> {
        Ok(())
    }
}

export!(Component);
