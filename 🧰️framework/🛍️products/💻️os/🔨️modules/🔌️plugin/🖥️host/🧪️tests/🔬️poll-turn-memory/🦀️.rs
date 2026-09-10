// 📈️ The `reactor.poll` export must not grow the guest's linear memory per turn.
//
// Boot #9b of `26/09/09/PROCEDURAL-3D-END-TO-END` died on
// `memory allocation of 189328 bytes failed` inside
// `wit_bindgen::rt::async_support::start_task` (`__export_poll_cabi`). `start_task` boxes the
// async-lifted export's future once per call; whether that box is FREED again is the whole
// question, and it is answerable only against a real `wasm32-wasip2` component. This law drives N
// idle turns through Wasmtime and reads `GuestInstance::guest_linear_memory_bytes` (the store's
// own `BudgetLimiter` high-water witness, i.e. `memory.size`) after each one.
//
// Points at a built component through `SEMIO_POLL_TURN_LEAK_COMPONENT` (same shape as
// `SEMIO_OWNED_DIFFERENTIAL_FIXTURES` in `🖨️describe`), because a `wasm32-wasip2 --profile
// wasm-dev` artifact is not a `cargo test` prerequisite. `SEMIO_POLL_TURN_LEAK_TURNS` overrides
// the turn count. See `📓️poll-task-leak-2026-09-10.md`.
use super::*;

/// 📐️ One wasm page — the tolerance the flatness law admits over the whole run.
const GUEST_PAGE_BYTES: usize = 65_536;

/// 🔁️ Drives `turns` idle `poll` turns and returns `(bytes after instantiate, per-turn readings,
/// turns that answered `MoreWork`)`. Acknowledges every lifecycle receipt the guest publishes, the
/// way a real host does — an open the host never acknowledges is retained state the guest re-offers
/// forever, and measuring "idle" against it measures the missing ACK instead.
async fn drive_idle_poll_turns(component: &std::path::Path, turns: usize) -> (usize, Vec<usize>, Vec<usize>) {
    let bytes = std::fs::read(component).unwrap_or_else(|error| panic!("read {}: {error}", component.display()));
    let runtime = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("engine builds");
    let hash = *semio_framework_hash::hash(&bytes).as_bytes();
    let package = PackageRef { package: PackageId("poll-turn-memory".to_string()), hash: PackageHash(hash) };
    let compiled = runtime.compile(&package, &bytes).await.expect("the staged actor component compiles");
    let max_frames: u32 = std::env::var("SEMIO_POLL_TURN_LEAK_FRAMES").ok().and_then(|value| value.parse().ok()).unwrap_or(64);
    let budget = Budget { fuel: u64::MAX, deadline_ms: 60_000, max_effects: 256, max_patch_bytes: 1 << 20, max_frames };
    let mut instance = runtime.instantiate(&compiled, RuntimeActorId(1), &[], &budget).await.expect("the staged actor component instantiates");
    let instantiated = instance.guest_linear_memory_bytes().expect("a wasmtime instance owns linear memory");
    let app_id = std::env::var("SEMIO_POLL_TURN_LEAK_APP").unwrap_or_else(|_| "s.procedural.generation3d@1/*#editor".to_string());
    let instances: u32 = std::env::var("SEMIO_POLL_TURN_LEAK_INSTANCES").ok().and_then(|value| value.parse().ok()).unwrap_or(1);
    let opens: Vec<Event> = (1..=instances)
        .map(|index| Event::InstanceOpen {
            request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: 1, instance_id: index, request_sequence: index as u64 },
            app_id: semio_framework::AppInstanceId(app_id.clone()),
            actor: format!("poll-turn-memory-{index}"),
            config: Vec::new(),
            assets: Vec::new(),
            capabilities: Vec::new(),
            quotas: Default::default(),
        })
        .collect();
    let mut readings = Vec::with_capacity(turns);
    let mut acks: Vec<Event> = Vec::new();
    let mut hot = 0_usize;
    // 🔬️ Attribution knob: drives a DIFFERENT async-lifted export the same number of times, so a
    // per-call cost of the shared component-async machinery separates from the reactor turn itself.
    if std::env::var("SEMIO_POLL_TURN_LEAK_EXPORT").as_deref() == Ok("checkpoint") {
        for _ in 0..turns {
            runtime.checkpoint(&mut instance).await.expect("checkpoint export");
            readings.push(instance.guest_linear_memory_bytes().expect("a wasmtime instance owns linear memory"));
        }
        eprintln!("[DEBUG] drove {turns} checkpoint exports instead of poll turns");
        return (instantiated, readings, Vec::new());
    }
    let mut hot_turns = Vec::new();
    for turn in 0..turns {
        let mut events: Vec<Event> = std::mem::take(&mut acks);
        if turn < opens.len() && !app_id.is_empty() {
            events.push(opens[turn].clone());
        }
        match runtime.execute_turn(&mut instance, &events, budget).await {
            Ok(result) => {
                if matches!(result.status, TurnStatus::MoreWork) {
                    hot += 1;
                    hot_turns.push(turn);
                }
                if let Some(receipt) = result.lifecycle_receipt {
                    acks.push(Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt }));
                }
                if turn < 4 || turn % 64 == 0 {
                    eprintln!("[DEBUG] turn {turn} result: status={:?} effects={} patches={} presence={} receipt={:?}", result.status, result.effects.len(), result.ui_patches.len(), result.presence.len(), result.lifecycle_receipt.is_some());
                }
            }
            Err(fault) => panic!("poll turn {turn} faulted: {fault:?}"),
        }
        readings.push(instance.guest_linear_memory_bytes().expect("a wasmtime instance owns linear memory"));
    }
    eprintln!("[DEBUG] {hot} of {turns} turns answered MoreWork, at {hot_turns:?}");
    (instantiated, readings, hot_turns)
}

/// ⏳️ Turns the guest may still answer `MoreWork` after an open before it must be settled — the open,
/// its ACK, and the ACK's own round trip.
const SETTLE_TURNS: usize = 8;

/// 🗂️ The component this law drives: the repo's own staged `wasm32-wasip2` procedural actor, unless
/// `SEMIO_POLL_TURN_LEAK_COMPONENT` names another one. No longer an opt-in — the law runs against
/// whatever the dev loop has staged, and only steps aside when nothing is staged at all (a fresh
/// clone that has never run `dev`), which it says out loud rather than passing silently.
fn staged_actor_component() -> Option<std::path::PathBuf> {
    if let Some(component) = std::env::var_os("SEMIO_POLL_TURN_LEAK_COMPONENT") {
        return Some(std::path::PathBuf::from(component));
    }
    let staged = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().nth(8)?.join("target/wasm32-wasip2/wasm-dev/semio_s_plugin_procedural.wasm");
    staged.exists().then_some(staged)
}

#[semio_framework_async_macros::async_test]
async fn the_poll_export_keeps_guest_linear_memory_flat_across_turns() {
    let Some(component) = staged_actor_component() else {
        eprintln!("[DEBUG] no staged wasm32-wasip2 actor component — build one with `cargo build -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev`");
        return;
    };
    let turns: usize = std::env::var("SEMIO_POLL_TURN_LEAK_TURNS").ok().and_then(|value| value.parse().ok()).unwrap_or(512);
    let (instantiated, readings, hot_turns) = drive_idle_poll_turns(&component, turns).await;
    let settled = readings[readings.len() / 4];
    let last = *readings.last().expect("at least one turn");
    let settled_turn = readings.len() / 4;
    let per_turn = (last - settled) as f64 / (readings.len() - settled_turn) as f64;
    eprintln!("[DEBUG] guest linear memory: instantiate={instantiated} B, turn {settled_turn}={settled} B, turn {}={last} B, {per_turn:.1} B/turn", readings.len() - 1);
    for (turn, reading) in readings.iter().enumerate().filter(|(turn, _)| turn % (readings.len() / 16).max(1) == 0) {
        eprintln!("[DEBUG] turn {turn}: {reading} B");
    }
    let still_hot: Vec<usize> = hot_turns.iter().copied().filter(|turn| *turn >= SETTLE_TURNS).collect();
    assert!(still_hot.is_empty(), "the guest answered MoreWork on {} turns past its open, at {still_hot:?} — an idle actor must answer Idle; every MoreWork is a host turn round trip that produced nothing", still_hot.len());
    assert!(last <= settled + GUEST_PAGE_BYTES, "reactor.poll grew the guest's linear memory by {} B over {} turns ({per_turn:.1} B/turn)", last - settled, readings.len() - settled_turn);
}
