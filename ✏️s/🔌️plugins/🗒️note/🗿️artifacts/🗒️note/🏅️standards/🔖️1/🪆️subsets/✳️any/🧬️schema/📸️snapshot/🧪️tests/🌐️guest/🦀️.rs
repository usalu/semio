//! 🗒️ Actual Note component open and codec laws under both independent host runtimes.
use semio_framework_plugin_host::{GuestRuntime,GuestRuntimes,GuestInstance,OwnedRuntime,WasmtimeRuntime,SharedEngineConfig,PackageRef,PackageId,PackageHash,TurnFault,retryable_lifecycle_turn};
use semio_framework::kernel::{Budget,Event,TurnResult,TurnStatus};
use semio_framework_actor::ActorId as RuntimeActorId;
use std::path::PathBuf;
const NOTE_EDITOR_APP:&str="s.note.note@1/*#editor";
const NOTE_DOCUMENT_SCHEMA:&str="note.document";
const NOTE_ARTIFACT_KIND:&str="s.note.note";
const MINTED_DOCUMENT_ID:&str="artifact-0123456789abcdef0123456789abcdef";
const OPEN_WALL_BUDGET:std::time::Duration=std::time::Duration::from_secs(120);
fn plugin_wasm(_file:&str)->PathBuf{
    let root=PathBuf::from(env!("CARGO_MANIFEST_DIR")).ancestors().find(|path|path.join("nx.json").is_file()).unwrap().to_path_buf();
    let path=root.join("🌎️hub/🧩️compositions/🗒️note/📦️packages/🦀️rust/dist/component-dev/semio_hub_note.wasm");
    assert!(path.is_file(),"Note component producer prerequisite is required");path
}
fn package_ref(package_id: &str, bytes: &[u8]) -> PackageRef {
    PackageRef { package: PackageId(package_id.to_string()), hash: PackageHash(*semio_framework_hash::hash(bytes).as_bytes()) }
}

fn open_budget() -> Budget {
    Budget { fuel: u64::MAX, deadline_ms: 8, max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 64 }
}

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

fn codec_budget() -> Budget {
    Budget { fuel: 8_000_000_000, deadline_ms: 120_000, max_effects: 0, max_patch_bytes: 0, max_frames: 0 }
}

#[semio_framework_async_macros::async_test]
async fn owned_runtime_instance_open_settles_against_a_real_plugin_component() {
    let path=plugin_wasm("semio_hub_note.wasm");
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new();
    let compiled = runtime.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let mut instance = runtime.instantiate(&compiled, RuntimeActorId(1), &[], &open_budget()).await.expect("instantiate plugin actor");
    let turn = open_to_settle(&runtime, &mut instance, NOTE_EDITOR_APP, open_budget, |guest| guest.turn_in_flight()).await;
    assert!(matches!(turn.status, TurnStatus::Idle), "InstanceOpen settled as {:?}", turn.status);
}

#[semio_framework_async_macros::async_test]
async fn wasmtime_runtime_instance_open_settles_against_a_real_plugin_component() {
    let path=plugin_wasm("semio_hub_note.wasm");
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("engine builds");
    let compiled = runtime.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let mut instance = runtime.instantiate(&compiled, RuntimeActorId(1), &[], &jit_budget()).await.expect("instantiate plugin actor");
    let turn = open_to_settle(&runtime, &mut instance, NOTE_EDITOR_APP, jit_budget, |_| false).await;
    assert!(matches!(turn.status, TurnStatus::Idle), "InstanceOpen settled as {:?}", turn.status);
}

#[semio_framework_async_macros::async_test]
async fn owned_codec_genesis_answers_on_a_real_plugin_component() {
    let path=plugin_wasm("semio_hub_note.wasm");
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new();
    let compiled = runtime.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let pair = runtime.codec_genesis(&compiled, NOTE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, codec_budget()).await.expect("codec.genesis by document schema");
    assert!(!pair.pack.is_empty() && !pair.spr.is_empty(), "codec.genesis produced an empty pair: pack={} spr={}", pair.pack.len(), pair.spr.len());
}

#[semio_framework_async_macros::async_test]
async fn owned_codec_genesis_answers_by_artifact_kind_too() {
    let path=plugin_wasm("semio_hub_note.wasm");
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new();
    let compiled = runtime.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let by_kind = runtime.codec_genesis(&compiled, NOTE_ARTIFACT_KIND, MINTED_DOCUMENT_ID, codec_budget()).await.expect("codec.genesis by artifact kind");
    let by_schema = runtime.codec_genesis(&compiled, NOTE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, codec_budget()).await.expect("codec.genesis by document schema");
    assert_eq!(by_kind, by_schema, "the two resolver keys must select the same editor and therefore the same genesis pair");
}

#[semio_framework_async_macros::async_test]
async fn owned_codec_pack_schema_hash_answers_on_a_real_plugin_component() {
    let path=plugin_wasm("semio_hub_note.wasm");
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new();
    let compiled = runtime.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let hash = runtime.codec_pack_schema_hash(&compiled, NOTE_DOCUMENT_SCHEMA, codec_budget()).await.expect("codec.pack-schema-hash by document schema");
    assert_ne!(hash, [0; 32], "a kind with a structural record specification must not answer the zero fingerprint");
}

#[semio_framework_async_macros::async_test]
async fn owned_codec_print_mirror_round_trips_a_genesis_pair() {
    let path=plugin_wasm("semio_hub_note.wasm");
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

#[semio_framework_async_macros::async_test]
async fn wasmtime_codec_genesis_answers_the_same_pair_as_the_interpreter() {
    let path=plugin_wasm("semio_hub_note.wasm");
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
    let owned_from_origin = owned.codec_genesis(&owned_compiled, NOTE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, codec_budget()).await.expect("owned codec.genesis from the assembled origin");
    assert_eq!(jit_pair, owned_from_origin, "a call from the owned codec origin answers what the JIT's fresh instance answers");
}

#[semio_framework_async_macros::async_test]
async fn guest_runtimes_forwards_all_four_codec_exports_to_the_runtime_beneath_it() {
    let path=plugin_wasm("semio_hub_note.wasm");
    let bytes = std::fs::read(&path).expect("read plugin component");
    let concrete = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("engine builds");
    let compiled = concrete.compile(&package_ref("semio:note", &bytes), &bytes).await.expect("compile plugin component");
    let direct_hash = concrete.codec_pack_schema_hash(&compiled, NOTE_DOCUMENT_SCHEMA, &jit_budget()).await.expect("wasmtime codec.pack-schema-hash");
    let direct_pair = concrete.codec_genesis(&compiled, NOTE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, &jit_budget()).await.expect("wasmtime codec.genesis");

    let routed = GuestRuntimes::from(concrete);
    assert_eq!(routed.codec_pack_schema_hash(&compiled, NOTE_DOCUMENT_SCHEMA, &jit_budget()).await.expect("routed codec.pack-schema-hash"), direct_hash);
    assert_eq!(routed.codec_genesis(&compiled, NOTE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, &jit_budget()).await.expect("routed codec.genesis"), direct_pair);

    let mirror = routed.codec_print_mirror(&compiled, NOTE_DOCUMENT_SCHEMA, &direct_pair.pack, &direct_pair.spr, &jit_budget()).await.expect("routed codec.print-mirror");
    assert!(!mirror.dsl.is_empty(), "a genesis pair prints a non-empty dsl mirror");
    let applied = routed.codec_apply_ops(&compiled, NOTE_DOCUMENT_SCHEMA, &direct_pair.pack, &direct_pair.spr, &[], &jit_budget()).await.expect("routed codec.apply-ops with an empty batch");
    assert_eq!(applied, direct_pair, "an empty batch applied to a pair is that pair");

    let refusal = routed.codec_pack_schema_hash(&compiled, "not.a.kind.this.package.owns", &jit_budget()).await.expect_err("a foreign kind has no fingerprint here");
    assert!(matches!(&refusal, TurnFault::Guest(fault) if !fault.code.0.is_empty() && !fault.message.is_empty()), "the guest's refusal reads as its own typed fault, as under the interpreter: {refusal}");
}

