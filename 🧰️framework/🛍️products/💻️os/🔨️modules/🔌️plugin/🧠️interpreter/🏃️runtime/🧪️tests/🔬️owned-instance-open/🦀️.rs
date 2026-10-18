//! 🧫️ Owned runtime laws that reach its private state — a poisoned instance and the codec origin — run against the
//! neutral SDK component the plugin host's test component producer stages.
use super::*;

const FIXTURE_DOCUMENT_SCHEMA: &str = "fixture.neutral-host-fixture.counter";

const MINTED_DOCUMENT_ID: &str = "artifact-0123456789abcdef0123456789abcdef";

fn fixture_component() -> std::path::PathBuf {
    let root = semio_framework_test_fixtures::repo_root!();
    let path = root.join("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️testing/🧩️component/📦️packages/🦀️rust/dist/components/semio_framework_plugin_host_test_component.wasm");
    assert!(path.is_file(), "neutral fixture producer prerequisite did not deliver {}", path.display());
    path
}

fn open_budget() -> Budget {
    Budget { retained: semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, fuel: u64::MAX, deadline_us: semio_framework_job::default_now_us().unwrap() + 8000, max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 64 }
}


fn codec_budget() -> Budget {
    Budget { retained: semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, fuel: 8_000_000_000, deadline_us: semio_framework_job::default_now_us().unwrap() + 120000000, max_effects: 0, max_patch_bytes: 0, max_frames: 0 }
}


#[test]
fn a_trapped_owned_instance_refuses_every_later_turn() {
    let mut original_observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|true;
    let mut original_recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut identity=crate::test_native_authority::original(&mut original_observer,&mut original_recipient);

    let path=fixture_component();
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new(semio_framework_job::default_now_us);
    let compiled = runtime.compile_component(&bytes).expect("compile plugin component");
    let mut instance = runtime.instantiate_actor(&compiled).expect("instantiate plugin actor");
    instance.poisoned = true;
    let refusal = runtime.execute_actor_turn(&mut instance, &[], open_budget(), &mut identity).expect_err("a trapped owned instance must refuse");
    assert!(format!("{refusal:?}").contains("poisoned"), "a trapped owned instance must name its poisoning, got {refusal:?}");
    let refusal = runtime.start_actor_job(&mut instance, 1, "semio.test", Vec::new()).expect_err("a trapped owned instance must refuse jobs too");
    assert!(format!("{refusal:?}").contains("poisoned"), "a trapped owned instance must name its poisoning, got {refusal:?}");
    crate::test_native_authority::close(&mut identity);
}

//#region 🧊️CodecOrigin
const SECOND_MINTED_DOCUMENT_ID: &str = "artifact-fedcba9876543210fedcba9876543210";

fn fresh_codec_answer<T: serde::de::DeserializeOwned>(runtime: &OwnedRuntime, compiled: &OwnedCompiledGuest, operation: OwnedOperation, input: &OwnedCodecInput<'_>) -> (T, u64) {
    let state = &mut runtime.instantiate_actor(compiled).expect("fresh owned instance");
    begin_owned_operation(state, operation, Some(serde_json::to_vec(input).expect("encode codec input"))).expect("begin fresh codec call");
    let invocation = resume_owned_operation_observed(state, operation, codec_budget().fuel, codec_budget().deadline_us, OwnedDeadline::Original, |_, _| {}, None).expect("fresh codec call completes");
    (decode_owned_result(&invocation.output).expect("fresh codec call answers"), invocation.fuel_used)
}

#[semio_framework_async_macros::async_test]
async fn codec_calls_answer_from_the_assembled_origin_exactly_what_a_fresh_instance_answers() {
    let mut original_observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|true;
    let mut original_recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut identity=crate::test_native_authority::original(&mut original_observer,&mut original_recipient);

    let path=fixture_component();
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new(semio_framework_job::default_now_us);
    let compiled = runtime.compile_component(&bytes).expect("compile plugin component");
    assert_eq!(compiled.codec_origin_bytes(), 0, "a compiled guest holds no origin before its first codec call");
    let genesis_input = |document_id: &'static str| OwnedCodecInput { artifact_schema: FIXTURE_DOCUMENT_SCHEMA, document_id, pack: &[], spr: &[], ops: &[] };
    let (fresh_pair, fresh_fuel): (GuestDocumentPair, u64) = fresh_codec_answer(&runtime, &compiled, OwnedOperation::Genesis, &genesis_input(MINTED_DOCUMENT_ID));

    let cancellation = GuestCallCancellation::default();
    let mut first_fuel = 0;
    let first = runtime.codec_genesis_observed(&compiled, FIXTURE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, codec_budget(), |fuel, _| first_fuel = fuel, &cancellation).await.expect("first codec.genesis assembles the origin");
    let origin_bytes = compiled.codec_origin_bytes();
    assert!(origin_bytes >= 65_536, "the assembled origin holds the guest's linear memory, got {origin_bytes} bytes");
    assert_eq!(compiled.clone().codec_origin_bytes(), origin_bytes, "every clone of a compiled guest shares its one origin");
    let mut second_fuel = 0;
    let second = runtime.codec_genesis_observed(&compiled, FIXTURE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, codec_budget(), |fuel, _| second_fuel = fuel, &cancellation).await.expect("second codec.genesis runs from the origin");
    assert_eq!(first, fresh_pair, "the assembling call answers what a fresh instance answers");
    assert_eq!(second, fresh_pair, "a call from the origin answers what a fresh instance answers");
    let assembly_fuel = match &*compiled.codec_origin.lock() {
        OwnedCodecOriginState::Ready(origin) => origin.assembly_fuel,
        OwnedCodecOriginState::Absent | OwnedCodecOriginState::Assembling { .. } => panic!("the origin is ready after a codec call"),
    };
    assert_eq!(first_fuel, assembly_fuel + second_fuel, "the first call reports the assembly and its own operation, nothing else");
    assert!(second_fuel < fresh_fuel, "a call from the origin skips the bundle assembly a fresh instance pays: {second_fuel} against {fresh_fuel} fuel");
    println!("codec origin: fresh genesis {fresh_fuel} fuel, assembly {assembly_fuel}, genesis from origin {second_fuel}, origin {origin_bytes} bytes");

    let (fresh_other, _): (GuestDocumentPair, u64) = fresh_codec_answer(&runtime, &compiled, OwnedOperation::Genesis, &genesis_input(SECOND_MINTED_DOCUMENT_ID));
    assert_eq!(runtime.codec_genesis(&compiled, FIXTURE_DOCUMENT_SCHEMA, SECOND_MINTED_DOCUMENT_ID, codec_budget()).await.expect("genesis of another document"), fresh_other);
    assert_eq!(runtime.codec_genesis(&compiled, FIXTURE_DOCUMENT_SCHEMA, MINTED_DOCUMENT_ID, codec_budget()).await.expect("genesis again"), fresh_pair, "no codec call sees what an earlier one did");

    let (fresh_hash, _): (Vec<u8>, u64) = fresh_codec_answer(&runtime, &compiled, OwnedOperation::PackSchemaHash, &OwnedCodecInput { artifact_schema: FIXTURE_DOCUMENT_SCHEMA, document_id: "", pack: &[], spr: &[], ops: &[] });
    assert_eq!(runtime.codec_pack_schema_hash(&compiled, FIXTURE_DOCUMENT_SCHEMA, codec_budget()).await.expect("codec.pack-schema-hash from the origin").to_vec(), fresh_hash);
    let pair_input = OwnedCodecInput { artifact_schema: FIXTURE_DOCUMENT_SCHEMA, document_id: "", pack: &fresh_pair.pack, spr: &fresh_pair.spr, ops: &[] };
    let (fresh_mirror, _): (GuestDocumentMirror, u64) = fresh_codec_answer(&runtime, &compiled, OwnedOperation::PrintMirror, &pair_input);
    assert_eq!(runtime.codec_print_mirror(&compiled, FIXTURE_DOCUMENT_SCHEMA, &fresh_pair.pack, &fresh_pair.spr, codec_budget()).await.expect("codec.print-mirror from the origin"), fresh_mirror);
    let (fresh_applied, _): (GuestDocumentPair, u64) = fresh_codec_answer(&runtime, &compiled, OwnedOperation::ApplyOps, &pair_input);
    assert_eq!(runtime.codec_apply_ops(&compiled, FIXTURE_DOCUMENT_SCHEMA, &fresh_pair.pack, &fresh_pair.spr, &[], codec_budget(), &mut identity).await.expect("codec.apply-ops from the origin"), fresh_applied);
    runtime.codec_pack_schema_hash(&compiled, "not.a.kind.this.package.owns", codec_budget()).await.expect_err("a foreign kind is refused from the origin as from a fresh instance");
    crate::test_native_authority::close(&mut identity);
}

#[test]
fn a_call_waiting_for_another_calls_assembly_relays_its_fuel_honours_its_cancellation_and_takes_over() {
    let path=fixture_component();
    let bytes = std::fs::read(&path).expect("read plugin component");
    let runtime = OwnedRuntime::new(semio_framework_job::default_now_us);
    let owned = &runtime.compile_component(&bytes).expect("compile plugin component");
    let cell: &OwnedCodecOriginCell = &owned.codec_origin;
    *cell.lock() = OwnedCodecOriginState::Assembling { fuel: 7 };
    std::thread::scope(|scope| {
        let (relay, relayed) = std::sync::mpsc::channel();
        let cancellation = GuestCallCancellation::default();
        let waiter = scope.spawn({
            let cancellation = cancellation.clone();
            let runtime = &runtime;
            move || runtime.codec_origin(owned, codec_budget(), &mut |fuel: u64, _: std::time::Duration| relay.send(fuel).expect("relay"), Some(&cancellation)).map(|used| used.spent_fuel)
        });
        assert_eq!(relayed.recv().expect("the waiter relays the assembly's fuel"), 7);
        cell.assembled_so_far(11);
        assert_eq!(relayed.recv().expect("the waiter relays the assembly's next fuel"), 11);
        cancellation.cancel();
        assert!(matches!(waiter.join().expect("waiter thread"), Err(TurnFault::Cancelled)), "a cancelled waiter ends without waiting for the assembly");
    });
    assert!(matches!(&*cell.lock(), OwnedCodecOriginState::Assembling { fuel: 11 }), "a waiter never touches another call's assembly");
    std::thread::scope(|scope| {
        let runtime = &runtime;
        let waiter = scope.spawn(move || runtime.codec_origin(owned, codec_budget(), &mut |_: u64, _: std::time::Duration| {}, None));
        std::thread::sleep(OWNED_CODEC_ORIGIN_WAIT_POLL * 4);
        drop(OwnedCodecOriginAssembly { cell, origin: None });
        let assembled = waiter.join().expect("waiter thread").expect("the waiter assembles the origin itself once the other assembly gave up");
        assert!(assembled.spent_fuel > 0 && assembled.spent_fuel == assembled.origin.assembly_fuel && assembled.reported_fuel == assembled.spent_fuel, "the call that assembled spends and reports the assembly's fuel as its own");
        let again = runtime.codec_origin(owned, codec_budget(), &mut |_: u64, _: std::time::Duration| {}, None).expect("the ready origin");
        assert!(Arc::ptr_eq(&assembled.origin, &again.origin) && again.spent_fuel == 0 && again.reported_fuel == 0, "every later call starts from the one origin and assembles nothing");
    });
}
//#endregion 🧊️CodecOrigin
