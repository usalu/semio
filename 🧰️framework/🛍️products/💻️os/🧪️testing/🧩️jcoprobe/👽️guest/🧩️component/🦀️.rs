//! 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-jco-spike). Minimal wasip2 guest component,
//! ALL `async func` (WASI 0.3 / component-model-async, callback ABI via
//! `wit_bindgen::generate!({ async: true })`), used to answer whether jco 1.27.0 can actually drive
//! an async-lifted-everywhere component from JS. See `👽️guest/📦️packages/🦀️rust/🧬️schema/📜️world.wit` for the
//! reduced WIT copy and `TICKET_DIR/📓️terra-jco-spike-report.md` for the full S1-S5 evidence.

wit_bindgen::generate!({
    path: "🧬️schema/📜️world.wit",
    world: "jcoprobe",
    async: true,
});

use exports::semio::jcoprobe::probe::Guest as ProbeGuest;
use semio::jcoprobe::probe_host;

struct Component;

impl ProbeGuest for Component {
    // #region S1 — trivial callable async export
    async fn poll(n: u32) -> u32 {
        n.wrapping_mul(2)
    }
    // #endregion

    // #region S2 — direct await of an async host import
    async fn await_echo(ms: u32, v: u32) -> u32 {
        probe_host::slow_echo(ms, v).await
    }
    // #endregion

    // #region S3 — spawn-detached: root future returns immediately, spawned sibling keeps running
    async fn spawn_detached(ms: u32) -> u32 {
        // 🧪️ `wit_bindgen::spawn` per its own doc: "this can be used to express
        // execution-after-returning in the component model" — the spawned future is polled inside
        // the SAME `FutureState::tasks` FuturesUnordered as this root future, but wit-bindgen's
        // codegen emits the `AsyncTaskReturn`/`task.return` call as soon as THIS function's own
        // future resolves (see `CallInterface`/`AsyncTaskReturn` in wit-bindgen-rust's bindgen.rs) —
        // not after `tasks` fully drains. So the caller's promise should resolve here, while
        // `slow-echo(ms, 0xDE7AC4ED)` is still in flight inside the spawned sibling.
        wit_bindgen::spawn(async move {
            let _ = probe_host::slow_echo(ms, 0xDE7AC4ED).await;
        });
        1
    }
    // #endregion

    // #region S4 — read a host-supplied byte stream chunk-by-chunk
    async fn read_body() -> u32 {
        let mut reader = probe_host::fetch_body().await;
        let mut total: u32 = 0;
        while let Some(_byte) = reader.next().await {
            total = total.wrapping_add(1);
        }
        total
    }
    // #endregion
}

export!(Component);
