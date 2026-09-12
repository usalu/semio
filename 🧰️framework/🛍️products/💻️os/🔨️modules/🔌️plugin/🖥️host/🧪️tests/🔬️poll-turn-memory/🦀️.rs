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

/// 🔒️ `semio_framework_job::set_runtime_diagnostics` is a PROCESS-WIDE switch, and `cargo test` runs
/// this file's laws on parallel threads in one binary — so a law that arms it would otherwise change
/// what a sibling law's guest does mid-run. Every law here that instantiates a component takes this
/// first, which makes them sequential with respect to each other and to nothing else.
static WASM_HOST_SERIAL: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn wasm_host_serial() -> std::sync::MutexGuard<'static, ()> {
    WASM_HOST_SERIAL.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

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

/// ⚖️ LAW: the guest's own runtime diagnostics are REACHABLE from a wasm host — a real
/// `wasm32-wasip2` component, booted with the host's diagnostics armed, prints the same `[DEBUG]`
/// lines the native in-process suites print, through `wasi:cli/environment` and `wasi:cli/stderr`.
///
/// 🐛️ Regression guard for `📓️audit-guest-tick-cost-2026-09-12.md` §0: the browser's
/// `localStorage.SEMIO_RUNTIME_DIAGNOSTICS` switch armed only TYPESCRIPT-side traces, and the
/// guest's `runtime_diagnostics_from_environment()` resolved against an environment no host ever
/// populated — so every Rust-side probe this ticket added was invisible in every browser capture,
/// and the 3.5-18.4 s per-hop cost could not be attributed to a function. Nothing in a native
/// in-process suite can catch that: `std::env::var` there reads the TEST's own environment. Only a
/// real component behind a real `wasi:cli/environment` can, which is what this drives.
///
/// 🔇️ The mirror half matters just as much: with diagnostics OFF the same component must print
/// NOTHING, because those sites are byte-proportional work an interactive boot must never pay for.
#[semio_framework_async_macros::async_test]
async fn an_armed_host_reaches_the_guests_own_runtime_diagnostics() {
    let _serial = wasm_host_serial();
    let Some(component) = staged_actor_component() else {
        eprintln!("[DEBUG] no staged wasm32-wasip2 actor component — build one with `cargo build -p semio-s-plugin-procedural --target wasm32-wasip2 --profile wasm-dev`");
        return;
    };
    let turns: usize = std::env::var("SEMIO_POLL_TURN_DIAGNOSTICS_TURNS").ok().and_then(|value| value.parse().ok()).unwrap_or(24);
    let silent = drive_diagnostics_turns(&component, turns, false).await;
    let armed = drive_diagnostics_turns(&component, turns, true).await;
    eprintln!("[DEBUG] guest stderr: disarmed={:?} armed={} bytes", silent.as_ref().map(String::len), armed.as_deref().map(str::len).unwrap_or(0));
    for line in armed.as_deref().unwrap_or_default().lines().take(16) {
        eprintln!("[DEBUG] guest said: {line}");
    }
    assert_eq!(silent.as_deref(), None, "a host with diagnostics OFF must hand the guest no environment and no stderr sink at all, got {silent:?}");
    let armed = armed.expect("an armed host retains the guest's stderr");
    assert!(armed.contains("[DEBUG]"), "an armed host must reach the guest's own `[DEBUG]` trace sites; the component printed {} bytes and none of them were a trace line:\n{armed}", armed.len());
}

/// 🩺️ Boots the staged component with the process-wide diagnostics switch forced to `armed`, drives
/// `turns` idle turns, and returns whatever the guest wrote to `wasi:cli/stderr`. The switch is a
/// process-wide atomic, so the two halves of the law above run in ONE test, sequentially, rather
/// than racing each other across two `#[test]`s in the same binary.
async fn drive_diagnostics_turns(component: &std::path::Path, turns: usize, armed: bool) -> Option<String> {
    semio_framework_job::set_runtime_diagnostics(armed);
    let bytes = std::fs::read(component).unwrap_or_else(|error| panic!("read {}: {error}", component.display()));
    let runtime = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("engine builds");
    let hash = *semio_framework_hash::hash(&bytes).as_bytes();
    let package = PackageRef { package: PackageId(format!("poll-turn-diagnostics-{armed}")), hash: PackageHash(hash) };
    let compiled = runtime.compile(&package, &bytes).await.expect("the staged actor component compiles");
    let budget = Budget { fuel: u64::MAX, deadline_ms: 60_000, max_effects: 256, max_patch_bytes: 1 << 20, max_frames: 64 };
    let mut instance = runtime.instantiate(&compiled, RuntimeActorId(1), &[], &budget).await.expect("the staged actor component instantiates");
    let app_id = std::env::var("SEMIO_POLL_TURN_LEAK_APP").unwrap_or_else(|_| "s.procedural.generation3d@1/*#editor".to_string());
    let mut events: Vec<Event> = vec![Event::InstanceOpen {
        request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: 1, instance_id: 1, request_sequence: 1 },
        app_id: semio_framework::AppInstanceId(app_id),
        actor: "poll-turn-diagnostics".to_string(),
        config: Vec::new(),
        assets: Vec::new(),
        capabilities: Vec::new(),
        quotas: Default::default(),
    }];
    for turn in 0..turns {
        match runtime.execute_turn(&mut instance, &events, budget).await {
            Ok(result) => {
                events = result.lifecycle_receipt.map(|receipt| vec![Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt })]).unwrap_or_default();
            }
            Err(fault) => panic!("diagnostics turn {turn} faulted: {fault:?}"),
        }
    }
    let text = instance.guest_diagnostics_text();
    semio_framework_job::set_runtime_diagnostics(false);
    text
}

#[semio_framework_async_macros::async_test]
async fn the_poll_export_keeps_guest_linear_memory_flat_across_turns() {
    let _serial = wasm_host_serial();
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
