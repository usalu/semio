use super::*;

const PLUGIN_WASM_TARGET_DIR: &str = ".🧬semio/🦑️repo/⚡️cache/cargo/target/wasm32-wasip2";
const PLUGIN_WASM_PROFILE_DIRS: [&str; 2] = ["wasm-dev", "wasm-release"];

/// 🗒️ `✏️s/🔌️plugins/🗒️note/🔣️.json` `manifest.apps[0].id` — the guest refuses any other id with
/// `plugin.internal: unknown app`, which is itself the proof that the open reached real app routing.
const NOTE_EDITOR_APP: &str = "s.note.note@1/*#editor";

/// ⏱️ How long an open may keep taking slices before this law calls it wedged. The owned interpreter
/// needs hundreds of 8 ms slices for a cold component, so this is wall clock, not an attempt count.
const OPEN_WALL_BUDGET: std::time::Duration = std::time::Duration::from_secs(120);

fn repo_root() -> PathBuf {
    let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !dir.join("AGENTS.md").is_file() || !dir.join(".🧬semio").is_dir() {
        if !dir.pop() {
            panic!("no repository root above {}", env!("CARGO_MANIFEST_DIR"));
        }
    }
    dir
}

fn plugin_wasm(file_name: &str) -> Option<PathBuf> {
    let root = repo_root();
    PLUGIN_WASM_PROFILE_DIRS.iter().map(|profile| root.join(PLUGIN_WASM_TARGET_DIR).join(profile).join(file_name)).find(|path| path.is_file())
}

/// 🖍️ `✏️s/🔌️plugins/🖍️draw/🔣️.json` `manifest.apps[0].id` — the bigger of the two staged components
/// (60 MB of `wasm-dev` against `🗒️note`'s smaller build), and the one the MCP gateway's own
/// `client-e2e` gate drives.
const DRAW_EDITOR_APP: &str = "s.draw.drawing@1/*#editor";

/// 🔏️ The component's OWN content hash. `WasmtimeRuntime::compile` keys its `.cwasm` cache on this,
/// so a placeholder makes every later run replay whichever build first populated the cache.
fn package_ref(package_id: &str, bytes: &[u8]) -> PackageRef {
    PackageRef { package: PackageId(package_id.to_string()), hash: PackageHash(*semio_framework_hash::hash(bytes).as_bytes()) }
}

fn open_budget() -> Budget {
    Budget { fuel: u64::MAX, deadline_ms: 8, max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 64 }
}

/// 🐎️ The JIT needs no slicing: a wasmtime turn cut by an epoch deadline leaves the component
/// instance mid-call and every later call answers `cannot enter component instance`, so its budget is
/// a real ceiling rather than an interactive slice.
fn jit_budget() -> Budget {
    Budget { deadline_ms: 120_000, ..open_budget() }
}

fn instance_open_event(app_id: &str, config: Vec<u8>) -> Event {
    Event::InstanceOpen {
        request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: 1, instance_id: 1, request_sequence: 1 },
        app_id: semio_framework::kernel::AppInstanceId(app_id.to_string()),
        actor: "editor".to_string(),
        config,
        assets: Vec::new(),
        capabilities: Vec::new(),
        quotas: semio_framework::kernel::QuotaSchema::default(),
    }
}

/// 🎬️ Drives one open the way a host must: slices until the guest is `Idle` with nothing left owed,
/// acknowledging the lifecycle receipt it publishes, resuming a mid-flight turn with NO new events,
/// and retrying the guest's own retained lifecycle-deadline verdict. Returns the settling turn.
async fn open_to_settle(runtime: &impl GuestRuntime, instance: &mut GuestInstance, app_id: &str, budget: impl Fn() -> Budget, mid_flight: impl Fn(&GuestInstance) -> bool) -> TurnResult {
    let mut owed = vec![instance_open_event(app_id, Vec::new())];
    let started = std::time::Instant::now();
    loop {
        let events = if mid_flight(instance) { Vec::new() } else { std::mem::take(&mut owed) };
        match runtime.execute_turn(instance, &events, budget()).await {
            Ok(turn) => {
                if let Some(receipt) = turn.lifecycle_receipt {
                    owed.push(Event::InstanceLifecycleAck(semio_framework::kernel::ActorInstanceLifecycleAck { receipt }));
                }
                if matches!(turn.status, TurnStatus::Idle) && owed.is_empty() {
                    return turn;
                }
            }
            Err(TurnFault::DeadlineExceeded | TurnFault::FuelExhausted) => {}
            Err(fault) if retryable_lifecycle_turn(&fault, &events) => {
                owed.splice(0..0, events);
            }
            Err(fault) => panic!("InstanceOpen faulted after {:?}: {fault:?}", started.elapsed()),
        }
        assert!(started.elapsed() < OPEN_WALL_BUDGET, "InstanceOpen never settled within {OPEN_WALL_BUDGET:?}");
    }
}

/// 🪤️ The MCP gateway's `ensure_instance`, reduced to its smallest real form: one real plugin
/// component, one `OwnedRuntime`, one `Event::InstanceOpen`, pumped to settle. Ticket 26/09/18 slice
/// A1 §5.1 measured this trapping every guest with `memory write is out of bounds: start=4294409068`
/// — a shadow-stack underflow at call depth 26 in a component wasm-ld had linked with its DEFAULT
/// 1 MiB stack, because `-zstack-size` was passed by a TypeScript build plan instead of
/// `.cargo/config.toml`. This law is the permanent oracle that a staged component opens.
#[semio_framework_async_macros::async_test]
async fn owned_runtime_instance_open_settles_against_a_real_plugin_component() {
    let Some(path) = plugin_wasm("semio_s_plugin_note.wasm") else { return };
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new();
    let compiled = runtime.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let mut instance = runtime.instantiate(&compiled, RuntimeActorId(1), &[], &open_budget()).await.expect("instantiate plugin actor");
    let turn = open_to_settle(&runtime, &mut instance, NOTE_EDITOR_APP, open_budget, |guest| runtime.turn_in_flight(guest)).await;
    assert!(matches!(turn.status, TurnStatus::Idle), "InstanceOpen settled as {:?}", turn.status);
}

/// 🐎️ The same component, the same event, through the runtime the os dev host uses — the A/B half of
/// the bisect. Before the link-arg fix BOTH runtimes trapped in
/// `<UiPatchApplyArena as Default>::default`, which is what ruled the owned interpreter out as the
/// cause; this law keeps them honest against each other.
#[semio_framework_async_macros::async_test]
async fn wasmtime_runtime_instance_open_settles_against_a_real_plugin_component() {
    let Some(path) = plugin_wasm("semio_s_plugin_note.wasm") else { return };
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("engine builds");
    let compiled = runtime.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let mut instance = runtime.instantiate(&compiled, RuntimeActorId(1), &[], &jit_budget()).await.expect("instantiate plugin actor");
    let turn = open_to_settle(&runtime, &mut instance, NOTE_EDITOR_APP, jit_budget, |_| false).await;
    assert!(matches!(turn.status, TurnStatus::Idle), "InstanceOpen settled as {:?}", turn.status);
}

/// 🖍️ The same law against the BIGGER staged component, and the one the MCP gateway's `client-e2e`
/// gate actually drives. `🖍️draw`'s open is what no amount of budget tuning could close under the
/// owned interpreter — ticket 26/09/18 slice R2 §10.6 measured it needing more than the MCP client's
/// own 240 s per-call budget — so this is the permanent oracle that the compiled runtime settles it.
///
/// ⏱️ The two costs are reported separately on purpose. `compile` is a cranelift compilation of a
/// 60 MB `wasm-dev` component, paid ONCE per build of that component and then served from
/// `compiled_cache_path`'s `.cwasm`; the open is what every later instance pays. Reading them as one
/// number is what makes a warm gateway look slow.
#[semio_framework_async_macros::async_test]
async fn wasmtime_runtime_instance_open_settles_against_the_draw_component() {
    let Some(path) = plugin_wasm("semio_s_plugin_draw.wasm") else { return };
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("engine builds");
    let compiling = std::time::Instant::now();
    let compiled = runtime.compile(&package_ref("semio:draw", &bytes), &bytes).await.expect("compile plugin component");
    let compiled_in = compiling.elapsed();
    let opening = std::time::Instant::now();
    let mut instance = runtime.instantiate(&compiled, RuntimeActorId(1), &[], &jit_budget()).await.expect("instantiate plugin actor");
    let turn = open_to_settle(&runtime, &mut instance, DRAW_EDITOR_APP, jit_budget, |_| false).await;
    println!("draw: bytes={} compile={compiled_in:?} open={:?}", bytes.len(), opening.elapsed());
    assert!(matches!(turn.status, TurnStatus::Idle), "InstanceOpen settled as {:?}", turn.status);
}

/// ☠️ A trapped owned instance is never reusable. Its shadow-stack pointer and allocator state are
/// whatever the trap left behind, so the next call starts lower and traps again — ticket 26/09/18
/// slice A1 read that drift ("the address decreases by exactly 112 per attempt") as the defect
/// itself, having measured it against a gateway that kept the trapped guest. The runtime must refuse
/// instead, naming re-instantiation as the only recovery. A guest FAULT is not a trap and must not
/// poison: a guest that answers with a `Fault` is healthy and keeps serving.
#[semio_framework_async_macros::async_test]
async fn a_trapped_owned_instance_refuses_every_later_turn() {
    let Some(path) = plugin_wasm("semio_s_plugin_note.wasm") else { return };
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new();
    let compiled = runtime.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let mut instance = runtime.instantiate(&compiled, RuntimeActorId(2), &[], &open_budget()).await.expect("instantiate plugin actor");
    owned_state_mut(&mut instance).expect("owned instance").poisoned = true;
    let refusal = runtime.execute_turn(&mut instance, &[], open_budget()).await.expect_err("a trapped owned instance must refuse");
    assert!(format!("{refusal:?}").contains("poisoned"), "a trapped owned instance must name its poisoning, got {refusal:?}");
    let refusal = runtime.start_job(&mut instance, 1, "semio.test", Vec::new()).await.expect_err("a trapped owned instance must refuse jobs too");
    assert!(format!("{refusal:?}").contains("poisoned"), "a trapped owned instance must name its poisoning, got {refusal:?}");
}

/// 📬️ An owned turn that yielded mid-flight owns the host's next call and CANNOT admit new events.
/// It used to accept them and silently drop them — `begin_owned_operation` returns `Ok` as soon as an
/// operation of the same kind is pending, so the freshly serialised `OwnedPollInput` was discarded.
/// That is what made the gateway's `Event::Wake`-on-resume branch inert, and what would have dropped
/// a whole retained `CommandIngressPage` had a command ever raced a cold open.
#[semio_framework_async_macros::async_test]
async fn a_mid_flight_owned_turn_refuses_new_events_instead_of_dropping_them() {
    let Some(path) = plugin_wasm("semio_s_plugin_note.wasm") else { return };
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new();
    let compiled = runtime.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let mut instance = runtime.instantiate(&compiled, RuntimeActorId(3), &[], &open_budget()).await.expect("instantiate plugin actor");
    let one_instruction = Budget { fuel: 1, ..open_budget() };
    assert!(matches!(runtime.execute_turn(&mut instance, &[instance_open_event(NOTE_EDITOR_APP, Vec::new())], one_instruction).await, Err(TurnFault::FuelExhausted)));
    assert!(runtime.turn_in_flight(&instance), "a fuel-yielded turn is mid-flight");
    let refusal = runtime.execute_turn(&mut instance, &[Event::Wake], open_budget()).await.expect_err("a mid-flight turn must refuse new events");
    assert!(format!("{refusal:?}").contains("mid-flight"), "a mid-flight turn must say so, got {refusal:?}");
    let resumed = runtime.execute_turn(&mut instance, &[], Budget { fuel: 1_000_000, ..open_budget() }).await;
    assert!(matches!(resumed, Err(TurnFault::FuelExhausted | TurnFault::DeadlineExceeded)), "resuming with no events continues the same turn rather than refusing it, got {resumed:?}");
    assert!(runtime.turn_in_flight(&instance), "the resumed turn is still the same one");
}
