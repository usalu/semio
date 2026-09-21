use super::*;

const PLUGIN_WASM_CARGO_CACHE_DIR: &str = ".🧬semio/🦑️repo/⚡️cache/cargo";
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

/// 🔎️ The FRESHEST build of one plugin component anywhere in the shared cargo cache. Preamble
/// rule 25 gives every slice a private `CARGO_TARGET_DIR` (`target-<slice>`) beside the shared
/// `target`, so the component a fleet slice just rebuilt is routinely NOT under `target/`. Picking
/// the newest mtime across every `target*` root is what keeps these laws honest about the tree
/// that is actually checked out rather than about whichever build happened to land first.
fn plugin_wasm(file_name: &str) -> Option<PathBuf> {
    let cache = repo_root().join(PLUGIN_WASM_CARGO_CACHE_DIR);
    let mut newest: Option<(std::time::SystemTime, PathBuf)> = None;
    for entry in std::fs::read_dir(&cache).ok()?.flatten() {
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };
        if name != "target" && !name.starts_with("target-") {
            continue;
        }
        for profile in PLUGIN_WASM_PROFILE_DIRS {
            let candidate = entry.path().join("wasm32-wasip2").join(profile).join(file_name);
            let Ok(modified) = candidate.metadata().and_then(|meta| meta.modified()) else { continue };
            if newest.as_ref().is_none_or(|(seen, _)| modified > *seen) {
                newest = Some((modified, candidate));
            }
        }
    }
    newest.map(|(_, path)| path)
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

/// 🗒️ `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️.rs`'s `NOTE_DOCUMENT_SCHEMA` — the PRIMARY key
/// `plugin_artifact_codec_app` resolves on and the exact key the hub's `VerifiedTrustedCatalog` pin
/// carries, so a law that asks by it walks the identical guest path a server does.
const NOTE_DOCUMENT_SCHEMA: &str = "note.document";

/// 🪪️ `s.note.note`'s own artifact kind — the SECOND key the resolver admits, because a compiled
/// descriptor publishes a kind and never a document schema. A note bundle owns two apps whose
/// dialect carries this kind (an editor and a viewer), so this key is also what exercises the
/// resolver's two-role path.
const NOTE_ARTIFACT_KIND: &str = "s.note.note";

/// 🪪️ One server-minted identity of the exact shape `artifact_app_genesis_pair` admits
/// (`artifact-` plus 32 lowercase hex digits, not all zero); any other spelling is refused by the
/// producer before a document is built, which would hide a guest-side fault behind a validation.
const MINTED_DOCUMENT_ID: &str = "artifact-0123456789abcdef0123456789abcdef";

/// 🌱️ A codec budget, not an interactive slice: the four `codec` exports run to completion on a
/// throwaway instance, so slicing them at 8 ms would only measure the interpreter's re-entry.
fn codec_budget() -> Budget {
    Budget { fuel: u64::MAX, deadline_ms: 120_000, max_effects: 0, max_patch_bytes: 0, max_frames: 0 }
}

/// 🌱️ The permanent oracle that `codec.genesis` answers on a REAL staged component. Ticket
/// 26/09/18 slice TC3c measured this trapping with `wasm trap: unreachable` at 16:14 on
/// 2026-09-21, which killed the three-package trusted-catalog bootstrap in its last step — while
/// the single native producer `artifact_app_genesis_pair` was green, so the fault was the guest
/// half of `world actor`'s `codec` interface and nothing in the genesis reduction itself.
///
/// 🪦️ The cause: `plugin_artifact_codec_app` constructs EVERY app of the installed bundle to read
/// its `artifact_schema()` and dropped each one it did not return, and a `VcsArtifactApp` owns an
/// `ArtifactStore` whose `Drop` asserts an exact terminal-empty shallow-shell witness. In a
/// `panic = "abort"` wasm32 guest that assert is the `unreachable`. Any bundle with more than one
/// app traps here, so this is a law over every plugin rather than over note.
#[semio_framework_async_macros::async_test]
async fn owned_codec_genesis_answers_on_a_real_plugin_component() {
    let Some(path) = plugin_wasm("semio_s_plugin_note.wasm") else { return };
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new();
    let compiled = runtime.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let pair = runtime.codec_genesis(&compiled, NOTE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, codec_budget()).await.expect("codec.genesis by document schema");
    assert!(!pair.pack.is_empty() && !pair.spr.is_empty(), "codec.genesis produced an empty pair: pack={} spr={}", pair.pack.len(), pair.spr.len());
}

/// 🪪️ The same export through the SECOND resolver key. A build tool holding nothing but a compiled
/// descriptor asks by kind, so a resolver that answers only by schema silently strands every
/// catalog builder — and a kind whose bundle owns both an editor and a viewer is exactly the shape
/// that used to drop the loser and abort.
#[semio_framework_async_macros::async_test]
async fn owned_codec_genesis_answers_by_artifact_kind_too() {
    let Some(path) = plugin_wasm("semio_s_plugin_note.wasm") else { return };
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new();
    let compiled = runtime.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let by_kind = runtime.codec_genesis(&compiled, NOTE_ARTIFACT_KIND, MINTED_DOCUMENT_ID, codec_budget()).await.expect("codec.genesis by artifact kind");
    let by_schema = runtime.codec_genesis(&compiled, NOTE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, codec_budget()).await.expect("codec.genesis by document schema");
    assert_eq!(by_kind, by_schema, "the two resolver keys must select the same editor and therefore the same genesis pair");
}

/// 🧬️ `codec.pack-schema-hash` shares `plugin_artifact_codec_app` with `codec.genesis`, so it is
/// the discriminator TC3c's report asked for: if THIS answers while genesis traps the fault is
/// genesis-specific, and if both trap the resolver is the fault. It is also the one call the
/// trusted-catalog bootstrap now makes per unlinked package, so a red here is a dead bootstrap.
#[semio_framework_async_macros::async_test]
async fn owned_codec_pack_schema_hash_answers_on_a_real_plugin_component() {
    let Some(path) = plugin_wasm("semio_s_plugin_note.wasm") else { return };
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new();
    let compiled = runtime.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let hash = runtime.codec_pack_schema_hash(&compiled, NOTE_DOCUMENT_SCHEMA, codec_budget()).await.expect("codec.pack-schema-hash by document schema");
    assert_ne!(hash, [0; 32], "a kind with a structural record specification must not answer the zero fingerprint");
}

/// 🧩️ The round trip a hub performs for a package whose Rust codec it links nothing for: create the
/// document, then print its pair back through the guest's own mirror. `print-mirror` and `apply-ops`
/// take the SELECTED app by reference and used to drop it on the way out, so they carry the same
/// defect as genesis and need the same oracle.
#[semio_framework_async_macros::async_test]
async fn owned_codec_print_mirror_round_trips_a_genesis_pair() {
    let Some(path) = plugin_wasm("semio_s_plugin_note.wasm") else { return };
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new();
    let compiled = runtime.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let pair = runtime.codec_genesis(&compiled, NOTE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, codec_budget()).await.expect("codec.genesis");
    let mirror = runtime.codec_print_mirror(&compiled, NOTE_DOCUMENT_SCHEMA, &pair.pack, &pair.spr, codec_budget()).await.expect("codec.print-mirror");
    assert!(mirror.dsl.contains(MINTED_DOCUMENT_ID), "the mirrored document must carry the minted identity, got {} bytes of dsl", mirror.dsl.len());
    let applied = runtime.codec_apply_ops(&compiled, NOTE_DOCUMENT_SCHEMA, &pair.pack, &pair.spr, &[], codec_budget()).await.expect("codec.apply-ops with an empty batch");
    assert!(!applied.pack.is_empty() && !applied.spr.is_empty(), "an empty apply-ops batch must return the baseline pair, not an empty one");
}

/// 🐎️ The A/B half of the `codec.genesis` bisect: the same component, the same export, through the
/// compiled runtime instead of the interpreter. Both runtimes trapped identically before the fix —
/// which is what placed the fault in the GUEST's own resolver rather than in either host — and both
/// must answer the identical bytes after it, because `codec` is a pure function of the component.
#[semio_framework_async_macros::async_test]
async fn wasmtime_codec_genesis_answers_the_same_pair_as_the_interpreter() {
    let Some(path) = plugin_wasm("semio_s_plugin_note.wasm") else { return };
    let bytes = std::fs::read(&path).expect("read plugin component");
    let jit = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("engine builds");
    let compiled = jit.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let jit_pair = jit.codec_genesis(&compiled, NOTE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, &jit_budget()).await.expect("wasmtime codec.genesis");
    assert!(!jit_pair.pack.is_empty() && !jit_pair.spr.is_empty(), "wasmtime codec.genesis produced an empty pair");
    let hash = jit.codec_pack_schema_hash(&compiled, NOTE_DOCUMENT_SCHEMA, &jit_budget()).await.expect("wasmtime codec.pack-schema-hash");
    assert_ne!(hash, [0; 32], "a kind with a structural record specification must not answer the zero fingerprint");
    let owned = OwnedRuntime::new();
    let owned_compiled = owned.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let owned_pair = owned.codec_genesis(&owned_compiled, NOTE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, codec_budget()).await.expect("owned codec.genesis");
    assert_eq!(jit_pair, owned_pair, "a pure codec export must answer identically under both runtimes");
}
