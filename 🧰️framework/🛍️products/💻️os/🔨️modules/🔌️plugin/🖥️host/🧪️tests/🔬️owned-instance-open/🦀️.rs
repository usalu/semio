
use semio_framework::kernel::{Budget, Event};
use semio_framework_plugin_host_guest_runtime::{GuestRuntimes, PackageRef, WasmtimeRuntime};
use semio_framework_plugin_interpreter_runtime::OwnedRuntime;
use semio_framework_actor::{PackageHash, PackageId, ActorId as RuntimeActorId};
use std::path::PathBuf;
use semio_framework_plugin_host_engine::SharedEngineConfig;
use semio_framework_plugin_guest_contract::TurnFault;
use semio_framework_plugin_host_guest_runtime::GuestRuntime as _;

#[path = "🪶️lease/🦀️.rs"]
mod count_component_tests;

const FIXTURE_EDITOR_APP:&str="fixture.neutral-host-fixture.counter@1/*#editor";
fn repo_root()->PathBuf {
    semio_framework_test_fixtures::repo_root!()
}
fn fixture_component()->PathBuf {
    let path=repo_root().join("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/📦️packages/🦀️rust/dist/components/semio_framework_plugin_host_test_component.wasm");
    assert!(path.is_file(),"neutral fixture producer prerequisite did not deliver {}",path.display());
    path
}

fn package_ref(package_id: &str, bytes: &[u8]) -> PackageRef {
    PackageRef { package: PackageId(package_id.to_string()), hash: PackageHash(*semio_framework_hash::hash(bytes).as_bytes()) }
}

fn open_budget() -> Budget {
    Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, u64::MAX, semio_framework_job::default_now_us().unwrap_or(0), 8, 64, 1 << 20, 64)
}

fn jit_budget() -> Budget {
    Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, u64::MAX, semio_framework_job::default_now_us().unwrap_or(0), 120_000, 64, 1 << 20, 64)
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

#[semio_framework_async_macros::async_test]
async fn a_mid_flight_owned_turn_refuses_new_events_instead_of_dropping_them() {
    let mut original_observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|true;
    let mut original_recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut identity=semio_framework_plugin_host_operation::test_native_authority::original(&mut original_observer,&mut original_recipient);

    let path=fixture_component();
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new(semio_framework_job::default_now_us);
    let compiled = runtime.compile(&package_ref("semio:neutral-host-fixture", &bytes), &bytes).await.expect("compile plugin component");
    let mut instance = runtime.instantiate(&compiled, RuntimeActorId(3), &[], &open_budget()).await.expect("instantiate plugin actor");
    let one_instruction = Budget { retained: semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, work: semio_framework_actor::OriginalActorTurnWork{clock_binding:1,absolute_deadline_us:u64::MAX,fuel:1_000_000}, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, fuel: 1, ..open_budget() };
    assert!(matches!(runtime.execute_turn(&mut instance, &[instance_open_event(FIXTURE_EDITOR_APP, Vec::new())], one_instruction, &mut identity).await, Err(TurnFault::FuelExhausted)));
    assert!(instance.turn_in_flight(), "a fuel-yielded turn is mid-flight");
    let refusal = runtime.execute_turn(&mut instance, &[Event::Wake], open_budget(), &mut identity).await.expect_err("a mid-flight turn must refuse new events");
    assert!(format!("{refusal:?}").contains("mid-flight"), "a mid-flight turn must say so, got {refusal:?}");
    let resumed = runtime.execute_turn(&mut instance, &[], Budget { retained: semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, work: semio_framework_actor::OriginalActorTurnWork{clock_binding:1,absolute_deadline_us:u64::MAX,fuel:1_000_000}, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, fuel: 1_000_000, ..open_budget() }, &mut identity).await;
    assert!(matches!(resumed, Err(TurnFault::FuelExhausted | TurnFault::DeadlineExceeded)), "resuming with no events continues the same turn rather than refusing it, got {resumed:?}");
    assert!(instance.turn_in_flight(), "the resumed turn is still the same one");
    semio_framework_plugin_host_operation::test_native_authority::close(&mut identity);
}

const FIXTURE_ARTIFACT_KIND: &str = "fixture.neutral-host-fixture.counter";

const MINTED_DOCUMENT_ID: &str = "artifact-0123456789abcdef0123456789abcdef";

fn codec_budget() -> Budget {
    Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, 8_000_000_000, semio_framework_job::default_now_us().unwrap_or(0), 120_000, 0, 0, 0)
}

#[semio_framework_async_macros::async_test]
async fn owned_codec_genesis_answers_on_a_real_plugin_component() {
    let path=fixture_component();
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new(semio_framework_job::default_now_us);
    let compiled = runtime.compile_component(&bytes).expect("compile plugin component");
    let pair = runtime.codec_genesis(&compiled, FIXTURE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, codec_budget()).await.expect("codec.genesis by document schema");
    assert!(!pair.pack.is_empty() && !pair.spr.is_empty(), "codec.genesis produced an empty pair: pack={} spr={}", pair.pack.len(), pair.spr.len());
}

#[semio_framework_async_macros::async_test]
async fn owned_codec_genesis_answers_by_artifact_kind_too() {
    let path=fixture_component();
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new(semio_framework_job::default_now_us);
    let compiled = runtime.compile_component(&bytes).expect("compile plugin component");
    let by_kind = runtime.codec_genesis(&compiled, FIXTURE_ARTIFACT_KIND, MINTED_DOCUMENT_ID, codec_budget()).await.expect("codec.genesis by artifact kind");
    let by_schema = runtime.codec_genesis(&compiled, FIXTURE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, codec_budget()).await.expect("codec.genesis by document schema");
    assert_eq!(by_kind, by_schema, "the two resolver keys must select the same editor and therefore the same genesis pair");
}

#[semio_framework_async_macros::async_test]
async fn owned_codec_pack_schema_hash_answers_on_a_real_plugin_component() {
    let path=fixture_component();
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new(semio_framework_job::default_now_us);
    let compiled = runtime.compile_component(&bytes).expect("compile plugin component");
    let hash = runtime.codec_pack_schema_hash(&compiled, FIXTURE_DOCUMENT_SCHEMA, codec_budget()).await.expect("codec.pack-schema-hash by document schema");
    assert_ne!(hash, [0; 32], "a kind with a structural record specification must not answer the zero fingerprint");
}

#[semio_framework_async_macros::async_test]
async fn owned_codec_print_mirror_round_trips_a_genesis_pair() {
    let mut original_observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|true;
    let mut original_recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut identity=semio_framework_plugin_host_operation::test_native_authority::original(&mut original_observer,&mut original_recipient);

    let path=fixture_component();
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new(semio_framework_job::default_now_us);
    let compiled = runtime.compile_component(&bytes).expect("compile plugin component");
    let pair = runtime.codec_genesis(&compiled, FIXTURE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, codec_budget()).await.expect("codec.genesis");
    let mirror = runtime.codec_print_mirror(&compiled, FIXTURE_DOCUMENT_SCHEMA, &pair.pack, &pair.spr, codec_budget()).await.expect("codec.print-mirror");
    assert!(!mirror.dsl.is_empty(), "the mirror printed no dsl at all");
    assert!(
        mirror.ops.contains(MINTED_DOCUMENT_ID) && mirror.ops.contains(FIXTURE_DOCUMENT_SCHEMA),
        "the mirrored ops log must open on a doc header carrying the minted identity and the schema, got {} bytes beginning {:?}",
        mirror.ops.len(),
        mirror.ops.chars().take(320).collect::<String>()
    );
    let applied = runtime.codec_apply_ops(&compiled, FIXTURE_DOCUMENT_SCHEMA, &pair.pack, &pair.spr, &[], codec_budget(), &mut identity).await.expect("codec.apply-ops with an empty batch");
    assert!(!applied.pack.is_empty() && !applied.spr.is_empty(), "an empty apply-ops batch must return the baseline pair, not an empty one");
    semio_framework_plugin_host_operation::test_native_authority::close(&mut identity);
}

#[semio_framework_async_macros::async_test]
async fn wasmtime_codec_genesis_answers_the_same_pair_as_the_interpreter() {
    let path=fixture_component();
    let bytes = std::fs::read(&path).expect("read plugin component");
    let jit = WasmtimeRuntime::new(SharedEngineConfig::default(),semio_framework_job::default_now_us).await.expect("engine builds");
    let compiled = jit.compile(&package_ref("semio:neutral-host-fixture", &bytes), &bytes).await.expect("compile plugin component");
    let jit_pair = jit.codec_genesis(&compiled, FIXTURE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, &jit_budget()).await.expect("wasmtime codec.genesis");
    assert!(!jit_pair.pack.is_empty() && !jit_pair.spr.is_empty(), "wasmtime codec.genesis produced an empty pair");
    let hash = jit.codec_pack_schema_hash(&compiled, FIXTURE_DOCUMENT_SCHEMA, &jit_budget()).await.expect("wasmtime codec.pack-schema-hash");
    assert_ne!(hash, [0; 32], "a kind with a structural record specification must not answer the zero fingerprint");
    let owned = OwnedRuntime::new(semio_framework_job::default_now_us);
    let owned_compiled = owned.compile_component(&bytes).expect("compile plugin component");
    let owned_pair = owned.codec_genesis(&owned_compiled, FIXTURE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, codec_budget()).await.expect("owned codec.genesis");
    assert_eq!(jit_pair, owned_pair, "a pure codec export must answer identically under both runtimes");
    let owned_from_origin = owned.codec_genesis(&owned_compiled, FIXTURE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, codec_budget()).await.expect("owned codec.genesis from the assembled origin");
    assert_eq!(jit_pair, owned_from_origin, "a call from the owned codec origin answers what the JIT's fresh instance answers");
}

const STAGED_CODEC_COMPONENTS:[(&str,&str,&str,&str,CodecSweepRuntime,GenesisMirrorText);2]=[
    ("semio:neutral-host-fixture","semio_framework_plugin_host_test_component.wasm",FIXTURE_ARTIFACT_KIND,FIXTURE_DOCUMENT_SCHEMA,CodecSweepRuntime::Owned,GenesisMirrorText::Structured),
    ("semio:neutral-host-fixture","semio_framework_plugin_host_test_component.wasm",FIXTURE_ARTIFACT_KIND,FIXTURE_DOCUMENT_SCHEMA,CodecSweepRuntime::Jit,GenesisMirrorText::Structured),
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum GenesisMirrorText {
    Structured,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CodecSweepRuntime {
    Owned,
    Jit,
}

async fn codec_sweep_one_component(package: &str, file_name: &str, kind: &str, schema: &str, which: CodecSweepRuntime, text: GenesisMirrorText) -> Result<(), String> {
    let mut original_observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|true;
    let mut original_recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut identity=semio_framework_plugin_host_operation::test_native_authority::original(&mut original_observer,&mut original_recipient);
    let result=async {

    let path=fixture_component();
    let bytes = std::fs::read(&path).map_err(|error| format!("read {}: {error}", path.display()))?;
    let runtime = match which {
        CodecSweepRuntime::Owned => GuestRuntimes::Owned(OwnedRuntime::new(semio_framework_job::default_now_us)),
        CodecSweepRuntime::Jit => GuestRuntimes::Wasmtime(WasmtimeRuntime::new(SharedEngineConfig::default(),semio_framework_job::default_now_us).await.map_err(|error| format!("engine: {error:?}"))?),
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
        _ => {}
    }
    if !mirror.ops.contains(MINTED_DOCUMENT_ID) || !mirror.ops.contains(schema) {
        return Err(format!(
            "codec.print-mirror({schema}) lost the minted identity: its ops log must open on a doc header carrying {MINTED_DOCUMENT_ID} and {schema}, got {} bytes beginning {:?}",
            mirror.ops.len(),
            mirror.ops.chars().take(320).collect::<String>()
        ));
    }
    let applied = runtime.codec_apply_ops(&compiled, schema, &pair.pack, &pair.spr, &[], &budget, &mut identity).await.map_err(|error| format!("codec.apply-ops({schema}): {error:?}"))?;
    if applied.pack.is_empty() || applied.spr.is_empty() {
        return Err(format!("codec.apply-ops({schema}) returned an empty baseline"));
    }
    Ok(())
    }.await;
    semio_framework_plugin_host_operation::test_native_authority::close(&mut identity);
    result
}

#[semio_framework_async_macros::async_test]
async fn owned_codec_answers_every_call_on_every_staged_component() {
    let mut swept = 0usize;
    let mut failures = Vec::new();
    for (package, file_name, kind, schema, which, text) in STAGED_CODEC_COMPONENTS {
        if !fixture_component().is_file() {
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

fn fixture_codec_budget() -> Budget { Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, 4_000_000_000, semio_framework_job::default_now_us().unwrap_or(0), 30_000, 0, 0, 0) }

const FIXTURE_DOCUMENT_SCHEMA: &str = "fixture.neutral-host-fixture.counter";

#[semio_framework_async_macros::async_test]
async fn owned_codec_genesis_completes_under_the_fixture_budget() {
    let path=fixture_component();
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new(semio_framework_job::default_now_us);
    let compiled = runtime.compile_component(&bytes).expect("compile plugin component");
    let started = std::time::Instant::now();
    let pair = runtime
        .codec_genesis(&compiled, FIXTURE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, fixture_codec_budget())
        .await
        .unwrap_or_else(|error| panic!("codec.genesis({FIXTURE_DOCUMENT_SCHEMA}) on {} ({} bytes) after {:?}: {error}", path.display(), bytes.len(), started.elapsed()));
    assert!(!pair.pack.is_empty() && !pair.spr.is_empty(), "codec.genesis produced an empty pair: pack={} spr={}", pair.pack.len(), pair.spr.len());
    let hashed = std::time::Instant::now();
    let hash = runtime
        .codec_pack_schema_hash(&compiled, FIXTURE_DOCUMENT_SCHEMA, fixture_codec_budget())
        .await
        .unwrap_or_else(|error| panic!("codec.pack-schema-hash({FIXTURE_DOCUMENT_SCHEMA}) after {:?}: {error}", hashed.elapsed()));
    assert_ne!(hash, [0; 32], "a kind with a structural record specification must not answer the zero fingerprint");
}

//#region 🗂️GuestCodecDispatch
#[semio_framework_async_macros::async_test]
async fn guest_runtimes_forwards_all_four_codec_exports_to_the_runtime_beneath_it() {
    let mut original_observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|true;
    let mut original_recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut identity=semio_framework_plugin_host_operation::test_native_authority::original(&mut original_observer,&mut original_recipient);

    let path=fixture_component();
    let bytes = std::fs::read(&path).expect("read plugin component");
    let concrete = WasmtimeRuntime::new(SharedEngineConfig::default(),semio_framework_job::default_now_us).await.expect("engine builds");
    let compiled = concrete.compile(&package_ref("semio:neutral-host-fixture", &bytes), &bytes).await.expect("compile plugin component");
    let direct_hash = concrete.codec_pack_schema_hash(&compiled, FIXTURE_DOCUMENT_SCHEMA, &jit_budget()).await.expect("wasmtime codec.pack-schema-hash");
    let direct_pair = concrete.codec_genesis(&compiled, FIXTURE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, &jit_budget()).await.expect("wasmtime codec.genesis");

    let routed = GuestRuntimes::from(concrete);
    assert_eq!(routed.codec_pack_schema_hash(&compiled, FIXTURE_DOCUMENT_SCHEMA, &jit_budget()).await.expect("routed codec.pack-schema-hash"), direct_hash);
    assert_eq!(routed.codec_genesis(&compiled, FIXTURE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, &jit_budget()).await.expect("routed codec.genesis"), direct_pair);

    let mirror = routed.codec_print_mirror(&compiled, FIXTURE_DOCUMENT_SCHEMA, &direct_pair.pack, &direct_pair.spr, &jit_budget()).await.expect("routed codec.print-mirror");
    assert!(!mirror.dsl.is_empty(), "a genesis pair prints a non-empty dsl mirror");
    let applied = routed.codec_apply_ops(&compiled, FIXTURE_DOCUMENT_SCHEMA, &direct_pair.pack, &direct_pair.spr, &[], &jit_budget(), &mut identity).await.expect("routed codec.apply-ops with an empty batch");
    assert_eq!(applied, direct_pair, "an empty batch applied to a pair is that pair");

    let refusal = routed.codec_pack_schema_hash(&compiled, "not.a.kind.this.package.owns", &jit_budget()).await.expect_err("a foreign kind has no fingerprint here");
    assert!(matches!(&refusal, TurnFault::Guest(fault) if !fault.code.0.is_empty() && !fault.message.is_empty()), "the guest's refusal reads as its own typed fault, as under the interpreter: {refusal}");
    semio_framework_plugin_host_operation::test_native_authority::close(&mut identity);
}
//#endregion 🗂️GuestCodecDispatch

#[test]
fn neutral_component_inventory_is_portable_and_names_both_independent_runtime_oracles() {
    let source=include_str!("../../🧪️testing/🧩️component/🧪️tests/🧫️fixtures/🔣️.json");
    let law:serde_json::Value=serde_json::from_str(source).unwrap();
    let owned:semio_framework_value::DslValue=semio_framework_pack_json::from_json_str(source, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&owned)).unwrap(),law);
    assert_eq!(law["artifactKind"],FIXTURE_ARTIFACT_KIND);
    assert_eq!(law["documentSchema"],FIXTURE_DOCUMENT_SCHEMA);
    assert_eq!(law["runtimeOracles"],serde_json::json!(["owned","wasmtime"]));
    assert_eq!(law["wireVectors"].as_array().unwrap().len(),3);
}
