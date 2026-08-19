//! 🧪️ terra-brep-probe Q5 micro-test: is the CURRENT guest `block_on` (⚙️engine/🦀️component.rs
//! lines 128-134: `pub async fn block_on<F>(future: F) -> F::Output { pollster::block_on(future) }`)
//! actually load-bearing under wasm32-wasip2, or does it only "work" today because — per this
//! probe's grep evidence (0 `.await` in the entire ✳️brep/🧬️schema tree, 1600+ `async fn`) —
//! every real kernel future resolves `Ready` on its FIRST poll, so `pollster::block_on`'s
//! `Condvar::wait()` path is NEVER actually reached in production?
//!
//! `pollster::block_on` (see vendored source at
//! ~/.cargo/registry/src/.../pollster-0.4.0/src/lib.rs) is a `Mutex`+`Condvar` loop, NOT
//! `thread::park`/`unpark`. In a real (native) multi-threaded host, a `Condvar::notify` from a
//! background thread can always eventually unblock `Condvar::wait`. wasm32-wasip2 guest
//! instances here run SINGLE-THREADED (no wasi-threads) — nothing else can ever run to call
//! `notify()` once the guest's one and only thread is blocked inside `Condvar::wait()`.
//!
//! Two modes, selected by argv[1]:
//!   `self-driving` — a future that calls `cx.waker().wake_by_ref()` SYNCHRONOUSLY, from within
//!                    its own `poll()`, before returning `Pending` (models a cooperative
//!                    multi-step CPU computation with no external dependency — the only shape a
//!                    guest-internal-only future can safely have). Expected: completes fast,
//!                    `Condvar::wait()` never truly blocks (already-`Notified` before it's
//!                    reached), proves Mutex/Condvar are at least FUNCTIONAL on wasm32-wasip2.
//!   `never-wakes`  — a future that returns `Pending` and stores the waker but NOTHING ever
//!                    calls it (models a future waiting on a completion this single thread could
//!                    never service while itself blocked). Expected: HANGS forever — the caller
//!                    must wrap this invocation in an external `timeout`.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct SelfDrivingCountdown {
    remaining: u32,
}

impl Future for SelfDrivingCountdown {
    type Output = u32;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        let this = self.get_mut();
        if this.remaining == 0 {
            return Poll::Ready(42);
        }
        this.remaining -= 1;
        cx.waker().wake_by_ref(); // synchronous self-wake, same call stack
        Poll::Pending
    }
}

struct NeverWakes;

impl Future for NeverWakes {
    type Output = u32;
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<u32> {
        // 🪤️ Deliberately drops the waker on the floor — nothing will ever call it. Models a
        // guest-internal future genuinely waiting on something this single thread cannot
        // service while blocked inside `pollster::block_on`'s `Condvar::wait()`.
        Poll::Pending
    }
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    match mode.as_str() {
        "self-driving" => {
            println!("[wasmpollster] mode=self-driving: pollster::block_on(SelfDrivingCountdown{{remaining:5}})");
            let result = pollster::block_on(SelfDrivingCountdown { remaining: 5 });
            println!("[wasmpollster] RESULT = {result}");
            println!("[wasmpollster] PASS: block_on completed under wasm32-wasip2 for a self-driving future");
        }
        "never-wakes" => {
            println!("[wasmpollster] mode=never-wakes: pollster::block_on(NeverWakes) -- expect this line is the LAST output, process must be killed by an external timeout");
            let _: u32 = pollster::block_on(NeverWakes);
            println!("[wasmpollster] UNREACHABLE: block_on returned, this must never print");
        }
        other => {
            eprintln!("unknown mode {other:?}, expected self-driving|never-wakes");
            std::process::exit(2);
        }
    }
}
