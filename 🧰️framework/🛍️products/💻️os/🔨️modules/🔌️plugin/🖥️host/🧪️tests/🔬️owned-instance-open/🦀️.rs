use super::*;

const PLUGIN_COMPONENT_PROFILE_DIRS: [&str; 2] = ["component-dev", "component-release"];

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

/// 🔎️ The FRESHEST deliverable of one plugin component: `<component crate>/dist/<profile>/<file>` over every plugin
/// and extension crate under `✏️s/🔌️plugins`, newest mtime first. These are the bytes `describe`, the dev staging and the
/// trusted catalog read; cargo's own target directories are build internals no law reads.
fn plugin_wasm(file_name: &str) -> Option<PathBuf> {
    plugin_wasm_in_profiles(file_name, &PLUGIN_COMPONENT_PROFILE_DIRS)
}

/// 🎯️ The same search restricted to named profiles. A law about how LONG a component takes must
/// say which build it means: `component-dev` carries four times the code of `component-release` for the same
/// plugin (215 MB against 48 MB for `🌍️gis` on 2026-09-22), and a trusted catalog stages the
/// RELEASE component, so a timing law that silently picked up whichever profile a peer rebuilt last
/// measures a build no hub ever runs — which is exactly what happened to slice HC1 at 21:52.
fn plugin_wasm_in_profiles(file_name: &str, profiles: &[&str]) -> Option<PathBuf> {
    let plugins = repo_root().join("✏️s/🔌️plugins");
    let mut crates = Vec::new();
    for owner in std::fs::read_dir(&plugins).ok()?.flatten() {
        crates.push(owner.path().join("📦️packages/🦀️rust"));
        for extension in std::fs::read_dir(owner.path().join("🧩️extensions")).into_iter().flatten().flatten() {
            crates.push(extension.path().join("📦️packages/🦀️rust"));
        }
    }
    let mut newest: Option<(std::time::SystemTime, PathBuf)> = None;
    for crate_root in crates {
        for profile in profiles.iter().copied() {
            let candidate = crate_root.join("dist").join(profile).join(file_name);
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
    let turn = open_to_settle(&runtime, &mut instance, NOTE_EDITOR_APP, open_budget, |guest| guest.turn_in_flight()).await;
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
    assert!(instance.turn_in_flight(), "a fuel-yielded turn is mid-flight");
    let refusal = runtime.execute_turn(&mut instance, &[Event::Wake], open_budget()).await.expect_err("a mid-flight turn must refuse new events");
    assert!(format!("{refusal:?}").contains("mid-flight"), "a mid-flight turn must say so, got {refusal:?}");
    let resumed = runtime.execute_turn(&mut instance, &[], Budget { fuel: 1_000_000, ..open_budget() }).await;
    assert!(matches!(resumed, Err(TurnFault::FuelExhausted | TurnFault::DeadlineExceeded)), "resuming with no events continues the same turn rather than refusing it, got {resumed:?}");
    assert!(instance.turn_in_flight(), "the resumed turn is still the same one");
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
/// ⛽️ A `codec` call is bounded by the guest STALLING (`OwnedDeadline::NoFuelProgress`, slice HC1),
/// so its ceiling is the FUEL cap and `u64::MAX` fuel would let a guest that spins keep these laws
/// running for ever. 8 G is the same order the describe path uses for the biggest staged component.
fn codec_budget() -> Budget {
    Budget { fuel: 8_000_000_000, deadline_ms: 120_000, max_effects: 0, max_patch_bytes: 0, max_frames: 0 }
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
///
/// 🪪️ The mirror is TWO files and the minted identity lives in exactly one of them. `print-mirror`
/// answers `(dsl, ops)`: the dsl is `initial_snapshot.print_dsl()` — the snapshot's OWN domain
/// fields, so `semio note.note.dsl v1\nschema=note.document id=empty …` is `NoteSnapshot::id`, not
/// the envelope's — while `print_ops_log`'s very first line is
/// `OpsHeaderLine::Doc { id: envelope.id, schema: envelope.schema }` (`🏪️store/🦀️.rs`), which is
/// the server-minted identity and the exact field `parse_document_text` reads it back out of.
/// Reading this as "print-mirror lost the minted identity" (2026-09-22) and then relaxing the
/// assertion to the dsl alone drops the round-trip property altogether, so ticket 26/09/18 slice
/// TC4 asserts it where the identity actually is.
#[semio_framework_async_macros::async_test]
async fn owned_codec_print_mirror_round_trips_a_genesis_pair() {
    let Some(path) = plugin_wasm("semio_s_plugin_note.wasm") else { return };
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new();
    let compiled = runtime.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let pair = runtime.codec_genesis(&compiled, NOTE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, codec_budget()).await.expect("codec.genesis");
    let mirror = runtime.codec_print_mirror(&compiled, NOTE_DOCUMENT_SCHEMA, &pair.pack, &pair.spr, codec_budget()).await.expect("codec.print-mirror");
    assert!(!mirror.dsl.is_empty(), "the mirror printed no dsl at all");
    assert!(
        mirror.ops.contains(MINTED_DOCUMENT_ID) && mirror.ops.contains(NOTE_DOCUMENT_SCHEMA),
        "the mirrored ops log must open on a doc header carrying the minted identity and the schema, got {} bytes beginning {:?}",
        mirror.ops.len(),
        mirror.ops.chars().take(320).collect::<String>()
    );
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


/// 🧾️ Every package the trusted-catalog bootstrap stages, as
/// `(package id, component file, artifact kind, document schema, runtime, genesis mirror text)`.
/// Adding a package is one row.
///
/// 🎯️ Membership is EVERY staged package, not only the ones the hub routes through the guest.
/// `🌎️hub/📦️packages/🦀️rust/📜️script.ts` short-circuits a package carrying a `linkedCodecRegistry`
/// to its linked rows, so a hub asks note's component and not gis's or stdio's — but the four
/// `codec` exports are the component's own contract, the bundle is published either way, and
/// `plugin_artifact_codec_app` constructs and closes EVERY app of the bundle it is asked of. That
/// makes this sweep the one standing law that walks all 298 editor/viewer apps of the three staged
/// packages and refuses a fail-closed bounded disposer anywhere among them.
///
/// ⚙️ Why gis and stdio run under the JIT. The owned interpreter is the runtime a HUB arms, and it
/// holds note's 14 610 991 B component comfortably; at 47 969 539 B and 49 723 044 B the same
/// interpreter needs ~840 s for a single `codec.genesis` (ticket 26/09/18 slice HC1 §1) and took the
/// whole test process to `signal: 9, SIGKILL` twice on this 32 GiB machine (slice TC3e). The
/// components are pure functions of their bytes and
/// `wasmtime_codec_genesis_answers_the_same_pair_as_the_interpreter` is the standing proof that the
/// two runtimes agree, so the big two are swept through `GuestRuntimes::Wasmtime` and note through
/// both. A law nobody can afford to run proves nothing.
const STAGED_CODEC_COMPONENTS: [(&str, &str, &str, &str, CodecSweepRuntime, GenesisMirrorText); 4] = [
    ("semio:note", "semio_s_plugin_note.wasm", NOTE_ARTIFACT_KIND, NOTE_DOCUMENT_SCHEMA, CodecSweepRuntime::Owned, GenesisMirrorText::Structured),
    ("semio:note", "semio_s_plugin_note.wasm", NOTE_ARTIFACT_KIND, NOTE_DOCUMENT_SCHEMA, CodecSweepRuntime::Jit, GenesisMirrorText::Structured),
    ("semio:gis", "semio_s_plugin_gis.wasm", "s.gis.gismap", GIS_DOCUMENT_SCHEMA, CodecSweepRuntime::Jit, GenesisMirrorText::Structured),
    ("semio:stdio", "semio_s_plugin_stdio.wasm", "s.stdio.txt", STDIO_DOCUMENT_SCHEMA, CodecSweepRuntime::Jit, GenesisMirrorText::CarrierRaw),
];

/// 📝️ What `codec.print-mirror` must print as the text of a genesis document.
///
/// 🧬️ A `CARRIER_TEXT` kind's text IS the raw external file, verbatim
/// (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/…/📸️snapshot/🦀️.rs` `impl store::ArtifactDsl for
/// TxtSnapshot`), so its empty genesis document prints exactly the empty string. Every other kind
/// prints a structured DSL that is never empty, even for its genesis snapshot.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum GenesisMirrorText {
    Structured,
    CarrierRaw,
}

/// 🏎️ Which guest runtime one sweep row is driven through — see `STAGED_CODEC_COMPONENTS`'s own doc
/// for why the two biggest staged components are not interpreted.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CodecSweepRuntime {
    Owned,
    Jit,
}

/// 🔤️ `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🦀️.rs` `STDIO_TXT_DOCUMENT_SCHEMA`, pinned here
/// because this crate cannot depend on an `s` plugin.
const STDIO_DOCUMENT_SCHEMA: &str = "stdio.txt";

/// 🧹️ Drives ALL FOUR `codec` exports over one staged component, both resolver keys included.
/// Returns the failure text rather than panicking so the sweep above it can report every component
/// in one run instead of dying on the first.
async fn codec_sweep_one_component(package: &str, file_name: &str, kind: &str, schema: &str, which: CodecSweepRuntime, text: GenesisMirrorText) -> Result<(), String> {
    let Some(path) = plugin_wasm_in_profiles(file_name, &["component-release"]) else { return Err(format!("{file_name} is not built in any target root")) };
    let bytes = std::fs::read(&path).map_err(|error| format!("read {}: {error}", path.display()))?;
    let runtime = match which {
        CodecSweepRuntime::Owned => GuestRuntimes::Owned(OwnedRuntime::new()),
        CodecSweepRuntime::Jit => GuestRuntimes::Wasmtime(WasmtimeRuntime::new(SharedEngineConfig::default()).await.map_err(|error| format!("engine: {error:?}"))?),
    };
    let budget = match which {
        CodecSweepRuntime::Owned => codec_budget(),
        CodecSweepRuntime::Jit => jit_budget(),
    };
    let compiled = runtime.compile(&package_ref(package, &bytes), &bytes).await.map_err(|error| format!("compile {}: {error:?}", path.display()))?;
    let hash = runtime.codec_pack_schema_hash(&compiled, schema, &budget).await.map_err(|error| format!("codec.pack-schema-hash({schema}): {error:?}"))?;
    if hash == [0; 32] {
        return Err(format!("codec.pack-schema-hash({schema}) answered the zero fingerprint"));
    }
    let pair = runtime.codec_genesis(&compiled, schema, MINTED_DOCUMENT_ID, &budget).await.map_err(|error| format!("codec.genesis({schema}): {error:?}"))?;
    if pair.pack.is_empty() || pair.spr.is_empty() {
        return Err(format!("codec.genesis({schema}) produced an empty pair"));
    }
    let by_kind = runtime.codec_genesis(&compiled, kind, MINTED_DOCUMENT_ID, &budget).await.map_err(|error| format!("codec.genesis({kind}): {error:?}"))?;
    if by_kind != pair {
        return Err(format!("codec.genesis({kind}) and codec.genesis({schema}) selected different apps"));
    }
    let mirror = runtime.codec_print_mirror(&compiled, schema, &pair.pack, &pair.spr, &budget).await.map_err(|error| format!("codec.print-mirror({schema}): {error:?}"))?;
    match text {
        GenesisMirrorText::Structured if mirror.dsl.is_empty() => return Err(format!("codec.print-mirror({schema}) printed no dsl at all")),
        GenesisMirrorText::CarrierRaw if !mirror.dsl.is_empty() => return Err(format!("codec.print-mirror({schema}) printed {} bytes for an empty carrier document", mirror.dsl.len())),
        _ => {}
    }
    if !mirror.ops.contains(MINTED_DOCUMENT_ID) || !mirror.ops.contains(schema) {
        return Err(format!(
            "codec.print-mirror({schema}) lost the minted identity: its ops log must open on a doc header carrying {MINTED_DOCUMENT_ID} and {schema}, got {} bytes beginning {:?}",
            mirror.ops.len(),
            mirror.ops.chars().take(320).collect::<String>()
        ));
    }
    let applied = runtime.codec_apply_ops(&compiled, schema, &pair.pack, &pair.spr, &[], &budget).await.map_err(|error| format!("codec.apply-ops({schema}): {error:?}"))?;
    if applied.pack.is_empty() || applied.spr.is_empty() {
        return Err(format!("codec.apply-ops({schema}) returned an empty baseline"));
    }
    Ok(())
}

/// 🧹️ The per-component sweep ticket 26/09/18 slice TC3d §6(b) asked for. The `codec` resolver is
/// shared by every package, so a defect in it is invisible when only one component is ever driven —
/// and note is the only one the hub routes through the guest interface, because stdio and gis carry
/// a linked Rust codec (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`, `spec.linkedCodecRegistry`).
///
/// 🪦️ `plugin_artifact_codec_app` constructs EVERY app of a bundle to read its schema and closes
/// each one it rejects, so this sweep is also the standing proof that every app of a staged bundle
/// can reach its terminal-empty shell. That is exactly what note's viewer could not do until TC3e:
/// `interactive-job.close-owned-disposer-missing … document-store`, which killed the three-package
/// bootstrap at 04:12:44 on 2026-09-22.
///
/// 🚧️ A component that has no `wasm-release` build in any target root is SKIPPED, not failed — a
/// slice rebuilds only the plugins it needs, and a law that demanded all three would be red on every
/// machine that has not run the bootstrap. The sweep fails if it found nothing at all. The profile
/// is named rather than taken by mtime because a hub stages the RELEASE component and a `wasm-dev`
/// build of the same plugin carries four times the code (ticket 26/09/18 slice HC1 §1).
#[semio_framework_async_macros::async_test]
async fn owned_codec_answers_every_call_on_every_staged_component() {
    let mut swept = 0usize;
    let mut failures = Vec::new();
    for (package, file_name, kind, schema, which, text) in STAGED_CODEC_COMPONENTS {
        if plugin_wasm_in_profiles(file_name, &["component-release"]).is_none() {
            continue;
        }
        swept += 1;
        let began = std::time::Instant::now();
        if let Err(detail) = codec_sweep_one_component(package, file_name, kind, schema, which, text).await {
            failures.push(format!("{package} [{which:?}] after {:?}: {detail}", began.elapsed()));
        }
    }
    assert!(swept > 0, "no staged plugin component has a wasm-release build in any target root, so this law proved nothing");
    assert!(failures.is_empty(), "{swept} staged component rows swept, {} failed:\n{}", failures.len(), failures.join("\n"));
}

/// ⏱️ The budget a HUB actually arms on every guest `codec` call, copied verbatim from
/// `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` `GUEST_CODEC_BUDGET`. Every law above runs
/// a budget this crate chose for itself; this one runs the caller's, because the fault ticket
/// 26/09/18 slice C8 measured — every `POST …/artifact-creations` failing in 32 s with
/// `genesis materialization failed: … epoch deadline exceeded` — lives entirely in the difference
/// between those two numbers.
const HUB_GUEST_CODEC_BUDGET: Budget = Budget { fuel: 4_000_000_000, deadline_ms: 30_000, max_effects: 0, max_patch_bytes: 0, max_frames: 0 };

/// 🗺️ `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs` `GIS_MAP_SCHEMA`, and the biggest component
/// the trusted-catalog bootstrap stages (≈ 48 MB of `wasm-release`).
const GIS_DOCUMENT_SCHEMA: &str = "gis.map";

/// ⏱️ The law ticket 26/09/18 slice HC1 owes the hub: a `codec` call is bounded by the GUEST
/// STALLING, never by a wall clock the machine's other work spends for it.
///
/// 🪦️ `codec_call` spent `budget.deadline_ms` as [`OwnedDeadline::TotalWall`] — the exact bound
/// `describe_observed` had already abandoned for [`OwnedDeadline::NoFuelProgress`] after `🀄️wfc`
/// and `🧩️puzzle` both died `DeadlineExceeded` while progressing normally. On the hub's 30 s that
/// made the biggest staged component uncreatable: C8's hub 7671 failed every creation in 32.0 s at
/// load 120, 33 and 21 alike, and HC1 measured the hub process burning 130–145 % CPU for the whole
/// window (`🗑️generated/hc1-create-sampled-1.txt`) — a guest that is running, not one that hangs.
///
/// 🚧️ Skipped when gis is not staged in any target root: a slice rebuilds only the plugins it
/// needs, and note is driven by the laws above.
#[semio_framework_async_macros::async_test]
async fn owned_codec_genesis_answers_the_biggest_staged_component_under_the_hub_s_own_budget() {
    let Some(path) = plugin_wasm_in_profiles("semio_s_plugin_gis.wasm", &["component-release"]) else { return };
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new();
    let compiled = runtime.compile(&package_ref("semio:gis", &bytes), &bytes).await.expect("compile plugin component");
    let started = std::time::Instant::now();
    let pair = runtime
        .codec_genesis(&compiled, GIS_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, HUB_GUEST_CODEC_BUDGET)
        .await
        .unwrap_or_else(|error| panic!("codec.genesis({GIS_DOCUMENT_SCHEMA}) on {} ({} bytes) after {:?}: {error}", path.display(), bytes.len(), started.elapsed()));
    assert!(!pair.pack.is_empty() && !pair.spr.is_empty(), "codec.genesis produced an empty pair: pack={} spr={}", pair.pack.len(), pair.spr.len());
    let hashed = std::time::Instant::now();
    let hash = runtime
        .codec_pack_schema_hash(&compiled, GIS_DOCUMENT_SCHEMA, HUB_GUEST_CODEC_BUDGET)
        .await
        .unwrap_or_else(|error| panic!("codec.pack-schema-hash({GIS_DOCUMENT_SCHEMA}) after {:?}: {error}", hashed.elapsed()));
    assert_ne!(hash, [0; 32], "a kind with a structural record specification must not answer the zero fingerprint");
    // ⏱️ No wall-clock assertion lives here, deliberately. A first cut of this law failed a run in
    // which BOTH calls answered — genesis 840.703220583 s, pack-schema-hash 895.51642 s at machine
    // load ≈ 40 (`🗑️generated/hc1-codec-laws-3.txt`) — which is the very judgement this slice took
    // out of the product. What the law asserts is that the two calls a hub makes per creation ANSWER
    // under the hub's own budget; how long they take is the machine's business, and the durations
    // ride in the two failure messages above for whoever needs them.
}

//#region 🗂️GuestCodecDispatch
/// 🗂️ ticket 26/09/18 slice M10: every real caller of a compiled plugin holds the `GuestRuntimes`
/// ENUM, not a concrete runtime — a `wasmtime::Component` belongs to the `Engine` that compiled it,
/// so a second runtime could not instantiate it at all. The enum forwarded the `GuestRuntime` trait
/// and nothing else, so all four `codec` exports were unreachable from `🏃️run` and `🌉️mcp` alike,
/// which is what left a headless server unable to register a document codec for a package it does
/// not link. This law drives all four through the enum and pins them against the concrete runtime
/// underneath: a forwarding method that dropped an argument or crossed two operations would answer
/// something, and something is exactly what a fingerprint must never be.
#[semio_framework_async_macros::async_test]
async fn guest_runtimes_forwards_all_four_codec_exports_to_the_runtime_beneath_it() {
    let Some(path) = plugin_wasm("semio_s_plugin_note.wasm") else { return };
    let bytes = std::fs::read(&path).expect("read plugin component");
    let concrete = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("engine builds");
    let compiled = concrete.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let direct_hash = concrete.codec_pack_schema_hash(&compiled, NOTE_DOCUMENT_SCHEMA, &jit_budget()).await.expect("wasmtime codec.pack-schema-hash");
    let direct_pair = concrete.codec_genesis(&compiled, NOTE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, &jit_budget()).await.expect("wasmtime codec.genesis");

    let routed = GuestRuntimes::from(concrete);
    assert_eq!(routed.codec_pack_schema_hash(&compiled, NOTE_DOCUMENT_SCHEMA, &jit_budget()).await.expect("routed codec.pack-schema-hash"), direct_hash);
    assert_eq!(routed.codec_genesis(&compiled, NOTE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, &jit_budget()).await.expect("routed codec.genesis"), direct_pair);

    // 📥️ `print-mirror` and `apply-ops` had no `WasmtimeRuntime` implementation at all before this
    // slice — only the owned interpreter carried them — so these two rows are the compiled half's
    // first execution as well as the enum's.
    let mirror = routed.codec_print_mirror(&compiled, NOTE_DOCUMENT_SCHEMA, &direct_pair.pack, &direct_pair.spr, &jit_budget()).await.expect("routed codec.print-mirror");
    assert!(!mirror.dsl.is_empty(), "a genesis pair prints a non-empty dsl mirror");
    let applied = routed.codec_apply_ops(&compiled, NOTE_DOCUMENT_SCHEMA, &direct_pair.pack, &direct_pair.spr, &[], &jit_budget()).await.expect("routed codec.apply-ops with an empty batch");
    assert_eq!(applied, direct_pair, "an empty batch applied to a pair is that pair");

    // 🚫️ …and a schema this component does not own is a typed refusal, never a fabricated answer —
    // which is what makes `print-mirror` usable as the pair-validation DISCRIMINATOR the WIT says
    // it is.
    routed.codec_pack_schema_hash(&compiled, "not.a.kind.this.package.owns", &jit_budget()).await.expect_err("a foreign kind has no fingerprint here");
}
//#endregion 🗂️GuestCodecDispatch
