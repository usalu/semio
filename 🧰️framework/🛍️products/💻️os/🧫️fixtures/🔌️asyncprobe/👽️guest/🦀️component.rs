//! 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-S1 spike). Guest half of the go/no-go probe
//! for component-model-async / WASI 0.3: an async export taking a scalar, an async import awaited
//! mid-call, and an async export reading a host-written `stream<u32>`.

wit_bindgen::generate!({
    path: "🧬️schema/📜️world.wit",
    world: "asyncprobe",
});

struct Component;

impl Guest for Component {
    async fn ping(n: u32) -> u32 {
        // 🎯️ the critical case: awaiting a HOST-implemented async import mid-call.
        let echoed = echo(format!("ping:{n}")).await;
        let _ = echoed;
        n + 1
    }

    async fn run(mut events: wit_bindgen::rt::async_support::StreamReader<u32>) -> u32 {
        let mut total: u32 = 0;
        while let Some(v) = events.next().await {
            total = total.wrapping_add(v);
        }
        total
    }
}

export!(Component);
