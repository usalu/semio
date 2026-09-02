//! 👶️ `semio-shard` — MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (P1-process-shards): the `[[bin]]`
//! `📓️design-runtime.md` §2 names ("in wave P1, the `semio-shard` `[[bin]]` runs over stdio").
//! Hosts exactly ONE `🧵️shard/🦀️.rs::ShardLoop`, driven by a real
//! [`semio_framework_plugin_host::OwnedRuntime`] over
//! [`semio_framework_plugin_host::process_transport::StdioTransport`] — the child-process half of
//! `ProcessTransport`'s duplex link (`../🚚️process-transport/🦀️.rs`). One process per
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
use semio_framework_plugin_host::shard::{ShardLoop, ShardTransports};
use semio_framework_plugin_host::{GuestRuntime, GuestRuntimes, OwnedRuntime, PackageHash, PackageId, PackageRef};
use std::sync::Arc;
use std::time::Duration;

/// ⛽️ Generous fixed budget for `GuestRuntime::instantiate`'s ONE-TIME initial fuel/deadline
/// setting — unrelated to per-turn scheduling (terra-shard-grants: per-turn budgets now travel in
/// `ShardFrame::Grant`, sent by whichever caller drives this process's stdin; this binary does not
/// itself compute or need one for `instantiate`, which only runs once at startup).
const INSTANTIATE_BUDGET: Budget = Budget { fuel: 200_000_000, deadline_ms: 500, max_effects: 32, max_patch_bytes: 1 << 16, max_frames: 8 };

fn main() {
    // 🎚️ P1f: this process hosts exactly ONE `ShardLoop`, pumped directly on THIS thread (below) —
    // never submitted to `semio_framework_plugin_host::plugin_host_worker_pool()`. That pool's only
    // tenants here are the epoch ticker and `StdioTransport`'s heartbeat sender (two sub-millisecond
    // periodic jobs), so it needs exactly one worker, not `available_parallelism()-1`.
    let args: Vec<String> = std::env::args().collect();
    let [_, wasm_path, package_id, actor_id_arg] = args.as_slice() else {
        eprintln!("[semio-shard] usage: semio-shard <component.wasm> <package-id> <actor-id>");
        std::process::exit(2);
    };
    let actor_id: u64 = actor_id_arg.parse().unwrap_or_else(|error| {
        eprintln!("[semio-shard] actor-id must be a u64: {error}");
        std::process::exit(2);
    });

    // 👶️ host-dedyn: `fn main` (E3) is this process's thread root — every async startup step below
    // crosses the sync↔async boundary via its own `block_on`, same bridge the pump loop uses.
    let runtime = Arc::new(GuestRuntimes::Owned(OwnedRuntime::new()));
    let bytes = std::fs::read(wasm_path).unwrap_or_else(|error| {
        eprintln!("[semio-shard] read {wasm_path}: {error}");
        std::process::exit(1);
    });
    let hash = *semio_framework_hash::hash(&bytes).as_bytes();
    let package = PackageRef { package: PackageId(package_id.clone()), hash: PackageHash(hash) };
    let compiled = semio_framework_async::block_on(runtime.compile(&package, &bytes)).unwrap_or_else(|error| {
        eprintln!("[semio-shard] compile failed: {error}");
        std::process::exit(1);
    });
    let instance = semio_framework_async::block_on(runtime.instantiate(&compiled, ActorId(actor_id), &[], &INSTANTIATE_BUDGET)).unwrap_or_else(|error| {
        eprintln!("[semio-shard] instantiate failed: {error}");
        std::process::exit(1);
    });

    let transport = semio_framework_async::block_on(StdioTransport::new(200));
    let mut shard = semio_framework_async::block_on(ShardLoop::new(runtime, ShardTransports::Stdio(transport)));
    shard.register(ActorId(actor_id), instance);
    eprintln!("[semio-shard] pid={} package={package_id} actor={actor_id} ready", std::process::id());

    // 🌀️ `ShardLoop::pump` only drains what is ALREADY buffered and never blocks (its own doc
    // comment) — this loop is the thing that keeps calling it, exactly the role a thread shard's
    // OS-thread loop plays for a `ThreadTransport`-backed `ShardLoop` (not built in this repo yet,
    // see the P1 report's `## gaps`; this binary is that loop's process-shard sibling). Per-turn
    // budgets now arrive via `ShardFrame::Grant` on the wire (terra-shard-grants) — until a real
    // caller sends one, envelopes for this actor run under `lane_defaults::budget_for(Lane::
    // Maintenance)` (`ShardLoop::granted_budget`'s own documented fallback), a behavior change
    // from the previous hardcoded 200M-fuel constant, and an honest one: this binary never
    // computed a real per-turn budget itself either.
    //
    // 👶️ host-dedyn: ONE `block_on` wrapping the whole loop — `fn main` (E3) is this process's
    // thread root, exactly `poll_ready`'s replacement the packet brief describes
    // ("each shard thread runs block_on(loop.run())").
    semio_framework_async::block_on(async {
        loop {
            if let Err(error) = shard.pump().await {
                eprintln!("[semio-shard] pump error: {error}");
            }
            std::thread::sleep(Duration::from_millis(5));
        }
    });
}
