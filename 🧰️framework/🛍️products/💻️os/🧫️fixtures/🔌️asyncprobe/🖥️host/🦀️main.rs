//! 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-S1 spike). Host half: instantiates the guest
//! component built from `👽️guest/`, drives its async exports, answers its async import, and writes
//! a host-owned `stream<u32>` into the guest's `run` export.

use anyhow::Result;
use wasmtime::component::{Accessor, Component, HasSelf, Linker, ResourceTable, StreamReader};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

wasmtime::component::bindgen!({
    path: "../👽️guest/🧬️schema/📜️world.wit",
    world: "asyncprobe",
});

struct HostState {
    wasi: WasiCtx,
    table: ResourceTable,
}

impl WasiView for HostState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

// 🎯️ G3a wiring: the async `echo` import is implemented via the `HasSelf<T>`/`Accessor` pattern
// wasmtime 47.0.3's concurrent-call model requires for host imports on an async world.
impl AsyncprobeImportsWithStore<HostState> for HasSelf<HostState> {
    async fn echo(_accessor: &Accessor<HostState, Self>, s: String) -> String {
        println!("[host] echo import called from inside guest await: {s}");
        format!("echo:{s}")
    }
}
impl AsyncprobeImports for HostState {}

fn main() -> Result<()> {
    futures::executor::block_on(async_main())
}

async fn async_main() -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model_async(true);
    config.concurrency_support(true);
    let engine = Engine::new(&config)?;

    let component_path = std::env::var("ASYNCPROBE_WASM").unwrap_or_else(|_| {
        "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/🎯️target-s1/wasm32-wasip2/release/semio_asyncprobe_guest.wasm".to_string()
    });
    let component = Component::from_file(&engine, &component_path)?;

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_async(&mut linker)?;
    Asyncprobe::add_to_linker::<HostState, HasSelf<HostState>>(&mut linker, |state| state)?;

    let wasi = WasiCtxBuilder::new().inherit_stdio().build();
    let table = ResourceTable::new();
    let mut store = Store::new(&engine, HostState { wasi, table });

    let instance = Asyncprobe::instantiate_async(&mut store, &component, &linker).await?;

    store
        .run_concurrent(async move |accessor: &Accessor<HostState>| -> Result<()> {
            // 🎯️ G3a: ping(41) == 42 — exercises the async export that awaits the async import
            // (`echo`) mid-call.
            let ping_result = instance.call_ping(accessor, 41).await?;
            println!("[host] ping(41) = {ping_result}");
            assert_eq!(ping_result, 42, "ping(41) should equal 42");

            // 🎯️ G3b: host writes a `stream<u32>`, guest sums it. `Vec<u32>` implements
            // `StreamProducer` directly (one-shot delivery, then `Dropped`).
            let events: StreamReader<u32> =
                accessor.with(|access| StreamReader::new(access, vec![1u32, 2, 3, 4, 5, 6]))?;
            let sum = instance.call_run(accessor, events).await?;
            println!("[host] run(stream) summed = {sum}");
            assert_eq!(sum, 21, "1+2+3+4+5+6 should equal 21");

            println!("[host] G3 PASS");
            Ok(())
        })
        .await??;

    Ok(())
}
