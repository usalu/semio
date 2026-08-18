//! 👶️ `semio-shard` — MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (P1-process-shards): the `[[bin]]`
//! `📓️design-runtime.md` §2 names ("in wave P1, the `semio-shard` `[[bin]]` runs over stdio").
//! Hosts exactly ONE `🧵️shard/🦀️component.rs::ShardLoop`, driven by a real
//! [`semio_framework_plugin_host::WasmtimeRuntime`] over
//! [`semio_framework_plugin_host::process_transport::StdioTransport`] — the child-process half of
//! `ProcessTransport`'s duplex link (`../🚚️process-transport/🦀️component.rs`). One process per
//! shard, one `semio-shard` invocation per process; multiple actors on the SAME shard register on
//! this one `ShardLoop` exactly like a thread shard would (this binary does not special-case actor
//! count — only its CLI carries a single starter actor today, see `## gaps` in the P1 report for why
//! multi-actor bootstrap over CLI args wasn't built).
//!
//! Usage: `semio-shard <component.wasm> <package-id> <actor-id>` — compiles and instantiates
//! `component.wasm` as `actor-id` under `package-id`, registers it on a fresh `ShardLoop`, and pumps
//! forever. The FIRST envelope sent on stdin should normally be `Event::InstanceOpen` (its `config`
//! bytes select the guest's own runtime behaviour, e.g. the scale fixture's `{"profile":"idle"}`) —
//! this binary does not synthesize one itself, matching `ShardLoop`'s own "never branches on
//! payload semantics beyond `Payload`'s tag" stance.

use semio_framework::kernel::Budget;
use semio_framework_actor::ActorId;
use semio_framework_plugin_host::process_transport::StdioTransport;
use semio_framework_plugin_host::shard::ShardLoop;
use semio_framework_plugin_host::{GuestRuntime, PackageHash, PackageId, PackageRef, SharedEngineConfig, WasmtimeRuntime};
use std::sync::Arc;
use std::time::Duration;

/// ⛽️ Generous fixed budget for this packet's own kill/rebuild proof — a real per-turn `Budget`
/// only a live `Kernel`/scheduler can hand down per envelope (out of scope here, same gap
/// `🧵️shard/🦀️component.rs`'s own `JOB_STEP_BUDGET` constant already documents for job steps).
const SHARD_CHILD_BUDGET: Budget = Budget { fuel: 200_000_000, deadline_ms: 500, max_effects: 32, max_patch_bytes: 1 << 16, max_frames: 8 };

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let [_, wasm_path, package_id, actor_id_arg] = args.as_slice() else {
        eprintln!("[semio-shard] usage: semio-shard <component.wasm> <package-id> <actor-id>");
        std::process::exit(2);
    };
    let actor_id: u64 = actor_id_arg.parse().unwrap_or_else(|error| {
        eprintln!("[semio-shard] actor-id must be a u64: {error}");
        std::process::exit(2);
    });

    let runtime = Arc::new(WasmtimeRuntime::new(SharedEngineConfig::default()).unwrap_or_else(|error| {
        eprintln!("[semio-shard] engine init failed: {error}");
        std::process::exit(1);
    }));
    let bytes = std::fs::read(wasm_path).unwrap_or_else(|error| {
        eprintln!("[semio-shard] read {wasm_path}: {error}");
        std::process::exit(1);
    });
    let hash = *blake3::hash(&bytes).as_bytes();
    let package = PackageRef { package: PackageId(package_id.clone()), hash: PackageHash(hash) };
    let compiled = runtime.compile(&package, &bytes).unwrap_or_else(|error| {
        eprintln!("[semio-shard] compile failed: {error}");
        std::process::exit(1);
    });
    let instance = runtime.instantiate(&compiled, ActorId(actor_id), &[], &SHARD_CHILD_BUDGET).unwrap_or_else(|error| {
        eprintln!("[semio-shard] instantiate failed: {error}");
        std::process::exit(1);
    });

    let transport = StdioTransport::new(200);
    let mut shard = ShardLoop::new(runtime, Box::new(transport));
    shard.register(ActorId(actor_id), instance);
    eprintln!("[semio-shard] pid={} package={package_id} actor={actor_id} ready", std::process::id());

    // 🌀️ `ShardLoop::pump` only drains what is ALREADY buffered and never blocks (its own doc
    // comment) — this loop is the thing that keeps calling it, exactly the role a thread shard's
    // OS-thread loop plays for a `ThreadTransport`-backed `ShardLoop` (not built in this repo yet,
    // see the P1 report's `## gaps`; this binary is that loop's process-shard sibling).
    loop {
        if let Err(error) = shard.pump(|_actor| SHARD_CHILD_BUDGET) {
            eprintln!("[semio-shard] pump error: {error}");
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}
