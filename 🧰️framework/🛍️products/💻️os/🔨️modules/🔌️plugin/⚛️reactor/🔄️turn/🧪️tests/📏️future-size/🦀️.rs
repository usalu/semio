// 📏️ The size of the future `wit_bindgen`'s `start_task` boxes once per `poll` export call.
//
// `__export_poll_cabi` → `async_support::start_task(fut)` → `Box::pin(fut)`: ONE heap allocation
// of `size_of_val(&fut)` bytes per guest turn, on wasm only. Boot #9b died on exactly that
// allocation at 189 328 bytes. The number is a property of the Rust generator layout, which is
// computed in MIR (`StateTransform`) and is therefore IDENTICAL natively — so this law weighs it
// without a wasm build.
//
// See `📓️poll-task-leak-2026-09-10.md`.

/// 📐️ `size_of_val` of the real turn future, exactly as `wit_bridge::poll` composes it:
/// `poll_kernel_output` → `with_turn_execution` → `poll_kernel_turn`, with the same generic
/// arguments the WIT bridge instantiates.
fn reactor_turn_future_bytes() -> usize {
    let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
    let budget = Budget { fuel: 64, deadline_ms: 1000, max_effects: 16, max_patch_bytes: 65536, max_frames: 16 };
    let future = crate::reactor::poll_kernel(&runtime, Vec::new(), None, None, budget);
    let bytes = size_of_val(&future);
    drop(future);
    bytes
}

/// 📐️ One wasm page. `start_task` allocates the turn future ONCE PER GUEST TURN, and it is the
/// largest single allocation the guest ever makes — boot #9b died on exactly that request at
/// 189 328 B (252 272 B here, the same generator at 64-bit pointer width). Measured after the fix:
/// 53 872 B. The ceiling is a ratchet against the two shapes that produced the quarter megabyte —
/// a future taken by value and re-pinned, and a large `async fn` awaited inline — not a target;
/// §8 of `📓️poll-task-leak-2026-09-10.md` carries the byte accounting for the next step.
const REACTOR_TURN_FUTURE_CEILING_BYTES: usize = 65_536;

#[test]
fn the_reactor_turn_future_fits_the_one_page_ceiling() {
    let bytes = reactor_turn_future_bytes();
    eprintln!("[DEBUG] reactor turn future size_of_val = {bytes} B");
    assert!(bytes <= REACTOR_TURN_FUTURE_CEILING_BYTES, "the turn future is {bytes} B — `wit_bindgen::rt::async_support::start_task` boxes exactly this much on every guest turn");
}

// 🧮️ …and what one turn RETAINS. `🖥️host/🧪️tests/🔬️poll-turn-memory` measured 4 437 B of guest
// linear memory per `poll` turn against a live instance on the real `wasm32-wasip2` component —
// identically for two different apps and with `max_frames = 0`, so the owner is a framework phase.
// This is the same measurement natively, through `PLUGIN_HEAP_WITNESS`, where it can be attributed
// to a phase with `SEMIO_RUNTIME_DIAGNOSTICS=1`.

/// ⏱️ Settled turns measured; growth is linear in the leak, so a short run extrapolates.
const SETTLED_TURNS: isize = 256;

/// 📐️ A settled turn may retain at most this much — one turn of a live actor holds NO new state, and
/// the guest's whole budget is one fixed linear memory with no process to restart.
const SETTLED_TURN_RETENTION_CEILING_BYTES: isize = 64;

#[semio_framework_async_macros::async_test]
async fn a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford() {
    let runtime = crate::plugin_runtime::PluginRuntime::<TestRuntimeApps>::new();
    crate::plugin_runtime::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
    let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(4_001, 8, "turn-retention".into())]).await.lifecycle_receipt.expect("Captured receipt");
    let semio_framework::kernel::ActorInstanceLifecycleReceipt::Captured { lifetime, .. } = captured else { panic!("open must emit Captured") };
    reactor_native_lifecycle_ack(&runtime, captured).await;
    for _ in 0..32 {
        reactor_native_lifecycle_poll(&runtime, Vec::new()).await;
    }
    let settled = semio_framework_trace::retained_heap_bytes();
    for _ in 0..SETTLED_TURNS {
        reactor_native_lifecycle_poll(&runtime, Vec::new()).await;
    }
    let after = semio_framework_trace::retained_heap_bytes();
    let per_turn = (after - settled) / SETTLED_TURNS;
    eprintln!("[DEBUG] settled reactor turn retention: settled={settled} after={after} per_turn={per_turn} B");
    assert!(per_turn <= SETTLED_TURN_RETENTION_CEILING_BYTES, "a settled reactor turn retains {per_turn} B — {} B over {SETTLED_TURNS} turns; the guest's linear memory is fixed", after - settled);
    reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
}
