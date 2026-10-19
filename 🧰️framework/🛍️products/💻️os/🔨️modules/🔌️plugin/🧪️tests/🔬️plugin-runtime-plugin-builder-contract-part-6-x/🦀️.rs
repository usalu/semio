/// 🔬️ Items of Part 6 of 7 of the plugin builder contract laws that came from included fixture files.


use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::TestMembers;
#[semio_framework_async_macros::async_test]
async fn reactor_native_lifecycle_retains_exact_close_until_ack() {
    use semio_framework::kernel::{ActorInstanceCloseRequest, ActorInstanceLifecycleReceipt as Receipt, Event};
    let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🚪️lifetime/🧪️tests/🧫️fixtures/🧵️production.json")).unwrap();
    let instance = fixture["open"]["instance_id"].as_u64().unwrap() as u32;
    let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
    semio_framework_plugin_runtime_instances::inst_descriptor::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
    let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(instance, 8, "native-fixture".into())]).await.lifecycle_receipt.expect("Captured receipt");
    let Receipt::Captured { lifetime, request_sequence } = captured else { panic!("open must emit Captured") };
    assert_eq!(request_sequence, 8);
    assert!(!runtime.guest_lifetimes.borrow().get(instance).unwrap().cell.is_live());
    assert_eq!(semio_framework_plugin_runtime_instances::inst_wire_frames::instance_actor(&runtime, instance).await.expect("admitted actor"), "native-fixture");
    let early = Event::InstanceClose(ActorInstanceCloseRequest { lifetime, request_sequence: 9 });
    assert!(semio_framework_plugin_reactor_turn::poll_kernel(&runtime, vec![early], None, None, reactor_native_lifecycle_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step_context()).await.is_err());
    assert!(semio_framework_plugin_reactor_lifecycle::life_instance_registry::instance_lifetime::plugin_capture_instance_close(&runtime, instance).is_ok());
    reactor_native_lifecycle_ack(&runtime, captured).await;
    assert!(runtime.guest_lifetimes.borrow().get(instance).unwrap().cell.is_live());
    reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
}

#[semio_framework_async_macros::async_test]
async fn reactor_native_lifecycle_rejects_foreign_and_colliding_owners() {
    use semio_framework::kernel::{ActorInstanceCloseRequest, ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt as Receipt, Event};
    let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
    semio_framework_plugin_runtime_instances::inst_descriptor::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
    assert!(semio_framework_plugin_reactor_turn::poll_kernel(&runtime, vec![reactor_native_lifecycle_open(7, 8, "a".repeat(semio_framework_plugin_reactor_lifecycle::life_instance_registry::PLUGIN_RUNTIME_ACTOR_BYTES + 1))], None, None, reactor_native_lifecycle_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step_context()).await.is_err());
    assert!(runtime.guest_lifetimes.borrow().get(7).is_none());
    assert!(semio_framework_plugin_reactor_lifecycle::life_instance_registry::instance_lifetime::plugin_capture_instance_close(&runtime, 7).is_err());
    let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(7, 8, "native-fixture".into())]).await.lifecycle_receipt.unwrap();
    let Receipt::Captured { lifetime, .. } = captured else { panic!("real captured owner") };
    assert!(semio_framework_plugin_reactor_turn::poll_kernel(&runtime, vec![reactor_native_lifecycle_open(1031, 8, "collision".into())], None, None, reactor_native_lifecycle_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step_context()).await.is_err());
    assert_eq!(runtime.guest_lifetimes.borrow().get(7).unwrap().cell.lifetime(), lifetime);
    reactor_native_lifecycle_ack(&runtime, captured).await;
    for foreign in [
        ActorInstanceLifetime { activation_generation: 42, ..lifetime },
        ActorInstanceLifetime { guest_lifetime: lifetime.guest_lifetime + 1, ..lifetime },
    ] {
        assert!(semio_framework_plugin_reactor_turn::poll_kernel(&runtime, vec![Event::InstanceClose(ActorInstanceCloseRequest { lifetime: foreign, request_sequence: 9 })], None, None, reactor_native_lifecycle_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step_context()).await.is_err());
        assert!(semio_framework_plugin_reactor_lifecycle::life_instance_registry::instance_lifetime::plugin_capture_instance_close(&runtime, 7).is_ok());
        assert!(runtime.guest_lifetimes.borrow().get(7).unwrap().cell.is_live());
    }
    let wrong = Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt: Receipt::Retired { lifetime, request_sequence: 9, close_generation: 1 } });
    assert!(semio_framework_plugin_reactor_turn::poll_kernel(&runtime, vec![wrong], None, None, reactor_native_lifecycle_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step_context()).await.is_err());
    reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
    let reopened = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(7, 10, "native-fixture".into())]).await.lifecycle_receipt.unwrap();
    let Receipt::Captured { lifetime: replacement, .. } = reopened else { panic!("new captured owner") };
    assert!(replacement.guest_lifetime > lifetime.guest_lifetime);
    assert!(semio_framework_plugin_reactor_turn::poll_kernel(&runtime, vec![Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt: captured })], None, None, reactor_native_lifecycle_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step_context()).await.is_err());
    reactor_native_lifecycle_ack(&runtime, reopened).await;
    reactor_native_lifecycle_finish(&runtime, replacement, 11).await;
}

#[semio_framework_async_macros::async_test]
async fn reactor_native_lifecycle_output_failure_preserves_ack_and_owner() {
    use semio_framework::kernel::{ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt as Receipt, Event};
    let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
    semio_framework_plugin_runtime_instances::inst_descriptor::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
    let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(7, 8, "native-fixture".into())]).await.lifecycle_receipt.unwrap();
    let Receipt::Captured { lifetime, .. } = captured else { panic!("real captured owner") };
    let ack = Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt: captured });
    let failed = crate::reactor_driver::poll_with_output_failure(&runtime, vec![ack], reactor_native_lifecycle_budget()).await;
    assert!(failed.is_err());
    {
        let lifetimes = runtime.guest_lifetimes.borrow();
        let slot = lifetimes.get(7).unwrap();
        assert_eq!(slot.cell.retained_receipt(), Some(captured));
        assert!(!slot.cell.is_live());
        assert!(slot.cell.owner().is_some());
    }
    reactor_native_lifecycle_ack(&runtime, captured).await;
    reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
}

/// 🧪️ `PATCHES`/`PENDING_PATCHES` are ONE process-wide authority per thread (the actor is a
/// singleton by design), so the two cases must not share a surface: residue from the first
/// case would otherwise be emitted by the second case's very first poll and the law would
/// report it as "a freshly queued patch was emitted immediately". Each case therefore runs
/// on its own instance pair and its own surface id, and every clause names its case.
///
/// 🩹️ A queued patch is carried by the FIRST turn that runs after it is queued. It used to
/// need two (`publish_into` the turn handback on one call, extraction on the next — one whole
/// host round trip per published surface, the `<n>:e---p` half of the two-state ping-pong in
/// `📓️reactor-reconcile-spin-2026-09-14.md` §1), and this law used to spend a preparation poll
/// on that dead turn. The extraction now belongs to the same call, so the faulting poll below
/// IS the turn that stages this patch — which is exactly what makes it the turn under test.
#[semio_framework_async_macros::async_test]
async fn reactor_output_fault_returns_real_patch_and_preserves_other_lifecycle_ack() {
    use semio_framework::kernel::{ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt as Receipt, Event};
    for late_clock in [false, true] {
        let (instance_a, instance_b) = if late_clock { (17u32, 18u32) } else { (7u32, 8u32) };
        let surface = format!("{instance_a}:output-fault-window");
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        semio_framework_plugin_runtime_instances::inst_descriptor::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
        let captured_a = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(instance_a, 8, "native-fixture".into())]).await.lifecycle_receipt.unwrap();
        let Receipt::Captured { lifetime: a, .. } = captured_a else { unreachable!() };
        reactor_native_lifecycle_ack(&runtime, captured_a).await;
        let captured_b = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(instance_b, 8, "native-fixture".into())]).await.lifecycle_receipt.unwrap();
        let Receipt::Captured { lifetime: b, .. } = captured_b else { unreachable!() };
        crate::reactor_driver::queue_external_patch(UiPatch {
            surface: semio_framework_ui_contract::SurfaceId::try_from(surface.as_str()).unwrap(),
            base_revision: semio_framework_ui_contract::UiRevision(0), revision: semio_framework_ui_contract::UiRevision(2), ops: Default::default(),
        });
        assert!(
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../⚛️reactor/📨️pending/🦀️.rs")).contains("`publish_into` is atomic (it moves the whole
    /// source or nothing), so the extraction belongs to the same call"),
            "late_clock={late_clock}: this law depends on the atomic same-call publication"
        );
        let event = Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt: captured_b });
        let (failed, provisional) = crate::reactor_driver::poll_with_patch_output_fault(&runtime, vec![event.clone()], reactor_native_lifecycle_budget(), late_clock).await;
        let failure = failed.expect_err("injected real output failure");
        if late_clock { assert_eq!(failure.code.0, "plugin.reactor-turn-deadline"); assert!(failure.retryable); }
        let provisional = provisional.expect("the real pending patch was staged");
        assert!(!crate::reactor_driver::patch_receipt_is_issued(provisional));
        assert_eq!(runtime.guest_lifetimes.borrow().get(instance_b).unwrap().cell.retained_receipt(), Some(captured_b));
        assert!(!runtime.guest_lifetimes.borrow().get(instance_b).unwrap().cell.is_live());
        let emitted = reactor_native_lifecycle_poll(&runtime, vec![event]).await;
        let issued = emitted.ui_patch_receipt.unwrap();
        assert_eq!(issued.lifetime, a);
        assert!(issued.patch_sequence > provisional.patch_sequence);
        let patch = emitted.ui_patches.iter().find(|patch| patch.surface.0.as_str() == surface).unwrap_or_else(|| panic!("late_clock={late_clock}: the staged patch is emitted on the very next turn"));
        assert_eq!(patch.revision.0, 2, "late_clock={late_clock}");
        assert!(runtime.guest_lifetimes.borrow().get(instance_b).unwrap().cell.is_live());
        let foreign = ActorUiPatchReceipt { lifetime: ActorInstanceLifetime { guest_lifetime: a.guest_lifetime + 1, ..a }, ..issued };
        for receipt in [provisional, foreign] {
            reactor_native_lifecycle_poll(&runtime, vec![Event::PatchAck { receipt, surface: surface.clone().into(), revision: 2 }]).await;
            assert!(crate::reactor_driver::patch_receipt_is_issued(issued));
        }
        reactor_native_lifecycle_poll(&runtime, vec![Event::PatchAck { receipt: issued, surface: surface.clone().into(), revision: 2 }]).await;
        assert!(!crate::reactor_driver::patch_receipt_is_issued(issued));
        drop(emitted);
        reactor_native_lifecycle_finish(&runtime, b, 9).await;
        reactor_native_lifecycle_finish(&runtime, a, 9).await;
    }
}


/// 🪪️ Native opening admits an explicit actor and refuses missing or empty authority.
#[semio_framework_async_macros::async_test]
async fn reactor_native_open_preserves_the_neutral_actor_authority() {
    let fixture: Value = serde_json::from_str(include_str!("../../../🏪️store/🧪️tests/🧫️fixtures/🧫️actor-genesis/🔣️.json")).expect("neutral actor/genesis fixture");
    let actor = fixture["actors"]["opened"].as_str().expect("opened actor");
    let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
    semio_framework_plugin_runtime_instances::inst_descriptor::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.expect("production declarations"));
    assert!(semio_framework_plugin_runtime_instances::inst_wire_frames::instance_actor(&runtime, 23).await.is_err(), "a missing admitted actor never invents authority");
    assert!(semio_framework_plugin_reactor_turn::poll_kernel(&runtime, vec![reactor_native_lifecycle_open(23, 1, String::new())], None, None, reactor_native_lifecycle_budget(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step_context()).await.is_err());
    assert!(runtime.guest_lifetimes.borrow().get(23).is_none(), "empty actor admission owns no instance");
    let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(23, 2, actor.into())]).await.lifecycle_receipt.expect("native open receipt");
    let ActorInstanceLifecycleReceipt::Captured { lifetime, .. } = captured else { panic!("opened native lifetime") };
    reactor_native_lifecycle_ack(&runtime, captured).await;
    assert_eq!(semio_framework_plugin_runtime_instances::inst_wire_frames::instance_actor(&runtime, 23).await.expect("admitted native actor"), actor);
    let document = semio_framework_plugin_runtime_ops::ops_document::plugin_document_pack(&runtime, 23).await.expect("native stored document");
    semio_framework_plugin_testing_laws::artifact_app_laws::plugin_load_document(&runtime, 23, &document, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("native bounded document reload");
    assert_eq!(semio_framework_plugin_runtime_instances::inst_wire_frames::instance_actor(&runtime, 23).await.expect("native actor after reload"), actor);
    reactor_native_lifecycle_finish(&runtime, lifetime, 3).await;
    let reopened = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(23, 4, actor.into())]).await.lifecycle_receipt.expect("native reopen receipt");
    let ActorInstanceLifecycleReceipt::Captured { lifetime, .. } = reopened else { panic!("reopened native lifetime") };
    reactor_native_lifecycle_ack(&runtime, reopened).await;
    assert_eq!(semio_framework_plugin_runtime_instances::inst_wire_frames::instance_actor(&runtime, 23).await.expect("reopened native actor"), fixture["actors"]["reopened"].as_str().expect("reopened actor"));
    reactor_native_lifecycle_finish(&runtime, lifetime, 5).await;
}

// 🧮️ …and what one turn RETAINS. `🖥️host/🧪️tests/🔬️poll-turn-memory` measured 4 437 B of guest
// linear memory per `poll` turn against a live instance on the real `wasm32-wasip2` component —
// identically for two different apps and with `max_frames = 0`, so the owner is a framework phase.
// This is the same measurement natively, through `PLUGIN_HEAP_WITNESS`, where it can be attributed
// to a phase with `SEMIO_RUNTIME_DIAGNOSTICS=1`.

/// ⏱️ Settled turns measured; growth is linear in the leak, so a short run extrapolates.
const SETTLED_TURNS: isize = 256;

/// 📐️ A settled turn may retain at most this much — one turn of a live actor holds NO new state, and
/// the guest's whole budget is one fixed linear memory with no process to restart.
const SETTLED_TURN_RETENTION_CEILING_BYTES: isize = 64;

/// 🧵️ The turn runs on THIS thread, so the law differences this thread's own allocator scope.
/// The process-wide reading is not a measurement of one turn under `cargo test`: libtest runs the
/// whole crate in one process on many threads, and the same law reads −144 B alone but tens of
/// kilobytes of its neighbours' work in a full suite. Isolating the scope is not loosening the
/// ceiling — `SETTLED_TURN_RETENTION_CEILING_BYTES` is unchanged.
///
/// 🧹️ The lifetime is retired and the runtime dropped BEFORE the assertion, never after it.
/// `NativeLifecycleRegistry`'s own `Drop` asserts that every runtime lifetime reached its
/// terminal exact ACK (`⚛️reactor/🚪️lifetime/🦀️.rs:489`); with the assertion first, a failed
/// reading unwound past `reactor_native_lifecycle_finish`, that `Drop` panicked a SECOND time
/// while unwinding, and "panic in a destructor during cleanup" is a NON-UNWINDING panic —
/// SIGABRT for the whole test binary, silently cancelling every one of this file's other laws.
/// A measurement that fails must fail as one red test, not as a process abort.
#[semio_framework_async_macros::async_test]
async fn a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford() {
    let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::<TestRuntimeApps>::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
    semio_framework_plugin_runtime_instances::inst_descriptor::install_plugin_bundle(&runtime, __semio_plugin_bundle().await.unwrap());
    let captured = reactor_native_lifecycle_poll(&runtime, vec![reactor_native_lifecycle_open(4_001, 8, "turn-retention".into())]).await.lifecycle_receipt.expect("Captured receipt");
    let semio_framework::kernel::ActorInstanceLifecycleReceipt::Captured { lifetime, .. } = captured else { panic!("open must emit Captured") };
    reactor_native_lifecycle_ack(&runtime, captured).await;
    for _ in 0..32 {
        reactor_native_lifecycle_poll(&runtime, Vec::new()).await;
    }
    let settled = semio_framework_trace::retained_heap_bytes_on_this_thread();
    for _ in 0..SETTLED_TURNS {
        reactor_native_lifecycle_poll(&runtime, Vec::new()).await;
    }
    let after = semio_framework_trace::retained_heap_bytes_on_this_thread();
    let per_turn = (after - settled) / SETTLED_TURNS;
    eprintln!("settled reactor turn retention: settled={settled} after={after} per_turn={per_turn} B");
    reactor_native_lifecycle_finish(&runtime, lifetime, 9).await;
    drop(runtime);
    assert!(per_turn <= SETTLED_TURN_RETENTION_CEILING_BYTES, "a settled reactor turn retains {per_turn} B — {} B over {SETTLED_TURNS} turns; the guest's linear memory is fixed", after - settled);
}

/// 📦️ The RAW effect an app mints in `reset_document_effect`: its own `encode_pack` bytes, a bare
/// `store::os_store::text_format::empty_document_spr` carrying no composition section at all, and an EMPTY member roster.
/// This is what leaves the command handler, NOT what may legally reach the archive loader.
async fn raw_single_document_archive(doc_id: &str, snapshot: &TestSnapshot) -> protocol::DocumentArchivePack {
    protocol::DocumentArchivePack {
        parent_pack: <TestSnapshot as pack::snapshot::ArtifactPack>::encode_pack(snapshot),
        parent_spr: Box::pin(store::os_store::text_format::empty_document_spr(doc_id, SingleDocumentApp::DOCUMENT_SCHEMA)).await,
        members: Vec::new(),
    }
}

/// 🪪️ The same archive after the runtime's own `stamp_load_document_identity` — the live store's
/// id, schema, dialect and owner written into the log's composition section. `ShellHost` round-trips
/// the effect's bytes untouched, so this is exactly what the guest's archive loader receives.
async fn stamped_single_document_archive(app: &SingleDocumentAppUnderTest, snapshot: &TestSnapshot) -> protocol::DocumentArchivePack {
    let envelope = app.stores.store.envelope();
    let mut archive = Box::pin(raw_single_document_archive(&envelope.id, snapshot)).await;
    let dialect: ArtifactDialect = SingleDocumentApp::DIALECT.into();
    archive.parent_spr = Box::pin(store::os_store::text_format::stamp_document_spr_identity(&archive.parent_spr, &envelope.id, SingleDocumentApp::DOCUMENT_SCHEMA, &dialect, envelope.owner.as_ref()))
        .await
        .expect("load-document identity stamp");
    archive
}

/// 🎯️ THE slice's reproduction: a `setActiveExample`-shaped whole-document replace must land.
#[semio_framework_async_macros::async_test]
async fn a_childless_whole_document_archive_replaces_the_live_document() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    let mut app = Box::pin(VcsArtifactApp::<SingleDocumentApp, TestMembers>::new(SingleDocumentApp, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())).await;
    assert_eq!(app.snapshot().expect("initial projection").label, "initial");
    let archive = Box::pin(stamped_single_document_archive(&app, &TestSnapshot { count: 42, label: "example".into(), slot: Vec::new() })).await;
    PluginApp::begin_document_archive_load(&mut app, 91, archive).expect("whole-document archive admission");
    let status = Box::pin(drive_single_document_archive(&mut app, 91, &mut cx)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "whole-document archive replacement failed: {}", archive_fault_text(&status));
    PluginApp::acknowledge_document_archive_load(&mut app, 91).expect("whole-document archive acknowledgement");
    let loaded = app.snapshot().expect("replaced projection");
    assert_eq!(loaded.count, 42, "the live document was not replaced by the archive's snapshot");
    assert_eq!(loaded.label, "example");
    close_member_admission_app(&mut app, &mut cx);

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}

/// 🚪️ The lane that carries the effect must stamp it. The refresh poll is a real emission lane — a
/// tool run that finalizes into a whole-document replace leaves through it — and it handed the shell
/// the app's own unstamped log, which parent hydration then refused. Whatever the app mints, what
/// leaves here must already name the live store, so this round trip stamps nothing of its own.
#[semio_framework_async_macros::async_test]
async fn the_refresh_poll_lane_stamps_the_load_document_it_emits() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    let mut app = Box::pin(VcsArtifactApp::<SingleDocumentApp, TestMembers>::new(SingleDocumentApp, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())).await;
    let effects = Box::pin(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_pending_effects(&mut app, None)).await.unwrap();
    let Some(Effect::LoadDocument { pack, spr }) = effects.first().cloned() else { panic!("the refresh poll lane dropped the LoadDocument effect: {effects:?}") };
    let archive = protocol::DocumentArchivePack { parent_pack: pack, parent_spr: spr, members: Vec::new() };
    PluginApp::begin_document_archive_load(&mut app, 94, archive).expect("polled archive admission");
    let status = Box::pin(drive_single_document_archive(&mut app, 94, &mut cx)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Ready, "the refresh poll lane emitted an unstamped LoadDocument: {}", archive_fault_text(&status));
    PluginApp::acknowledge_document_archive_load(&mut app, 94).expect("polled archive acknowledgement");
    assert_eq!(app.snapshot().expect("replaced projection").count, 42);
    close_member_admission_app(&mut app, &mut cx);

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}

/// 🪪️ The identity stamp is the WHOLE contract between an app's `Effect::LoadDocument` and this
/// loader: an app mints its log with `store::os_store::text_format::empty_document_spr`, which carries no composition
/// section, and hydration refuses such a log outright (`MemberOpenDiagnostic::Identity`) long before
/// any of the three replacement legs runs. Whichever dispatch lane carries the effect MUST stamp it.
#[semio_framework_async_macros::async_test]
async fn an_unstamped_whole_document_archive_is_refused_by_parent_hydration() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    let mut app = Box::pin(VcsArtifactApp::<SingleDocumentApp, TestMembers>::new(SingleDocumentApp, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())).await;
    let live_id = app.stores.store.envelope().id.clone();
    let archive = Box::pin(raw_single_document_archive(&live_id, &TestSnapshot { count: 9, label: "unstamped".into(), slot: Vec::new() })).await;
    PluginApp::begin_document_archive_load(&mut app, 93, archive).expect("unstamped archive admission");
    let status = Box::pin(drive_single_document_archive(&mut app, 93, &mut cx)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Fault);
    assert!(archive_fault_text(&status).contains("Identity"), "an unstamped log must be refused as an identity mismatch: {}", archive_fault_text(&status));
    assert_eq!(app.snapshot().expect("retained projection").label, "initial");
    PluginApp::acknowledge_document_archive_load(&mut app, 93).expect("unstamped archive acknowledgement");
    close_member_admission_app(&mut app, &mut cx);

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}

/// 🧭️ The refusal must name its leg on the wire, not collapse into one sentence for eleven causes.
/// An archive whose SPR carries a foreign schema is refused by the app's own initialization job;
/// before this slice the host saw only "failed closure, authority, or retained publication
/// validation" and no `#[cfg(test)]`-free way to tell which.
#[semio_framework_async_macros::async_test]
async fn a_refused_whole_document_archive_names_the_leg_that_refused_it() {
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    let mut app = Box::pin(VcsArtifactApp::<SingleDocumentApp, TestMembers>::new(SingleDocumentApp, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())).await;
    let envelope_id = app.stores.store.envelope().id.clone();
    let dialect: ArtifactDialect = SingleDocumentApp::DIALECT.into();
    let mut archive = Box::pin(stamped_single_document_archive(&app, &TestSnapshot { count: 7, label: "foreign".into(), slot: Vec::new() })).await;
    let foreign = Box::pin(store::os_store::text_format::empty_document_spr(&envelope_id, "semio.test.some-other-app/v1")).await;
    archive.parent_spr = Box::pin(store::os_store::text_format::stamp_document_spr_identity(&foreign, &envelope_id, "semio.test.some-other-app/v1", &dialect, None)).await.expect("foreign stamp");
    PluginApp::begin_document_archive_load(&mut app, 92, archive).expect("foreign-schema archive admission");
    let status = Box::pin(drive_single_document_archive(&mut app, 92, &mut cx)).await;
    assert_eq!(status.state, protocol::DocumentArchiveLoadState::Fault);
    let text = archive_fault_text(&status);
    assert!(
        !text.contains("failed closure, authority, or retained publication validation"),
        "the terminal report is still the opaque three-leg collapse: {text}"
    );
    assert_eq!(app.snapshot().expect("retained projection").label, "initial", "a refused archive must leave the live document untouched");
    PluginApp::acknowledge_document_archive_load(&mut app, 92).expect("foreign-schema archive acknowledgement");
    close_member_admission_app(&mut app, &mut cx);

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}

#[semio_framework_async_macros::async_test]
async fn original_lifecycle_archive_pending_status_observation_preserves_retained_source(){
    let mut caller=semio_framework_plugin_testing_laws::OriginalFixtureCaller::admit().expect("original composition caller input");let mut cx=caller.context();
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧾️document-archive-load-legs/🧫️fixtures/🔣️.json")).unwrap();
    let mut app=Box::pin(VcsArtifactApp::<SingleDocumentApp,TestMembers>::new(SingleDocumentApp,protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()),semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),&mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity())).await;
    let operation=law["operation"].as_u64().unwrap();let archive=Box::pin(stamped_single_document_archive(&app,&TestSnapshot{count:42,label:"example".into(),slot:Vec::new()})).await;
    PluginApp::begin_document_archive_load(&mut app,operation,archive).unwrap();
    let mut identity=semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity();
    let first=semio_framework_async::poll::resolve_ready(PluginApp::poll_document_archive_load(&mut app,operation,&mut identity, &mut cx)).unwrap();assert_eq!(first.state,protocol::DocumentArchiveLoadState::Pending);
    for _ in 0..law["observations"].as_u64().unwrap(){
        let(status,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_async::poll::resolve_ready(PluginApp::poll_document_archive_load(&mut app,operation,&mut identity, &mut cx)).unwrap());
        assert_eq!(status,first);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(app.snapshot().unwrap().count,0);
    }
    PluginApp::cancel_document_archive_load(&mut app,operation).unwrap();close_member_admission_app(&mut app, &mut cx);

    let fuel=cx.fuel_remaining();drop(cx);caller.retire(fuel).expect("original composition caller paid retirement");
}/// 🪟️ A registry whose ONE interaction domain is `HierarchyProvider::Flat` — deliberately not
/// `interaction_registry`'s `Topology` twin: a `Flat` domain is omitted from
/// `build_full_interaction_topology`'s pruning map, so the pick this module dispatches survives
/// `validate_state` without seeding a document label first, and this law measures the surface route
/// instead of a typed-command factory proof.
async fn surface_routing_registry() -> AppActionRegistry {
    let builder = App::builder(test_app_surface_id().await, LocalizedLabel::data("Synthetic"))
            .await
            .document(["state"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .window_kind("main", LocalizedLabel::data("Main"), "synthetic.main", SurfaceKind::Canvas2d, IconName::AppWindow)
            .await
            .interaction(InteractionDefinition {
                id: "items".into(),
                label: LocalizedLabel::data("Items"),
                granularities: vec![GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec::default(),
                selection: SelectionSpec { modes: vec![SelectionMode::Multiple, SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
            })
            .await
            .window_kind_interactions("main", vec![InteractionRef::new("items")])
            .await;
    let app = App::from_builder(declare_test_app_verbs(builder).interactive_jobs(InteractiveJobClassification::Migrated).await).await;
    AppActionRegistry::from_definition(&app.definition)
}

/// 🪟️ Materializes ONE surface's mounted host view exactly as the shell builds it — the shared base
/// context of the refresh generation that mounted it, overlaid with that generation's own fields.
/// Mirrors `windowViewContext`/`panelViewContext` (`🛂️manifest/🟦️.ts`).
fn surface_routing_view(base: &Value, overrides: &Value) -> ViewModel {
    let mut merged = base.clone();
    for (key, value) in overrides.as_object().expect("surface view overrides") {
        merged[key] = value.clone();
    }
    serde_json::from_value(merged).expect("surface view context")
}

/// 🎯️ The projection a mounted surface MUST be rendered against: its own window instance, or the
/// panel projection when it names no window.
fn surface_routing_projection(view: &ViewModel) -> ViewModel {
    match view.window_id.as_deref() {
        Some(window) => view.for_window_instance(window).expect("mounted window instance"),
        None => view.for_panel(),
    }
}

async fn surface_routing_render(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<VcsArtifactApp<TestApp>>, surface: &str, expected: &ViewModel, node_key: &str) {
    let (_, presence) = semio_framework_plugin_runtime_ops::ops_surface_render::plugin_render_surface(runtime, 1, surface).await.expect("mounted surface renders");
    let (body, actual) = RENDER_CONTEXT_PROBE.with(|probe| probe.take()).expect("the app body was rendered");
    assert_eq!(
        serde_json::to_value(&actual).unwrap(),
        serde_json::to_value(expected).unwrap(),
        "surface {surface} (body {body}) was rendered against another surface's view state"
    );
    assert!(presence.iter().all(|update| update.surface.as_ref() == surface), "surface {surface} published presence addressed at another surface");
    assert!(presence.iter().any(|update| update.node_key == node_key && update.own.selected), "the pick must reach surface {surface}'s own render");
}

/// 🪟️ THE route the browser renders every body through and no law ever covered:
/// `Event::SurfaceVisible` → `plugin_mount_surface` → `SurfaceContexts` → `plugin_render_surface`.
/// Mounts the three surfaces a split puzzle3d session mounts — two panes of the SAME window kind and the
/// Inspection panel — each with the host view of the refresh generation that mounted it, dispatches one
/// browser-shaped `interactionSelect`, renders every body, and then requires the SYNTHETIC `1:window`
/// surface to be refused by name.
///
/// 🚫️ WAVE B56 LAW: no surface named `window` is ever mounted for an app with window instances. The
/// host used to mint exactly that fourth surface, bound to the last pane
/// (`windowHostContextBindings`, `🔌️PluginRuntime/🟦️.tsx`, wave W-G3 §8.29), and its reconcile
/// reservation refusal left every real surface deferred with the reactor answering `more-work` for ever
/// (`reserve_refusal=1:window:registry-reservation-unavailable`, wave B54 §6.4). A mount request for that
/// name is now a named fault, not an alias.
///
/// 🎯️ ticket 26/09/02/PUZZLE-3D-END-TO-END wave B25: `SurfaceContexts` kept ONE `view_state` that
/// every mount overwrote, so a body's `ViewModel` was rebuilt from whichever sibling surface mounted
/// LAST — the Inspection panel rendered against the last pane's projection and the first pane
/// rendered against the alias's. B23 §3 named this as the one hop no testkit render helper
/// (`render_body`/`render_panel_body`/`render_window_refresh`/`render_window`/`render_composite`)
/// can reach, because every one of them calls `PluginApp::render` with a hand-built `ViewModel`.
///
/// 🕹️ The pick's own stamping pass already queued its presence; draining it here makes every
/// assertion below about the presence THAT surface's render produced.
///
/// 🧹 The reserved pick parks its worker-session retirement PROCESS-globally (maintenance stage
/// 23); left behind it pollutes the heap `a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford`
/// measures — the same rule `close_fixture_app_to_terminal_emptiness` states for every fixture app.
#[semio_framework_async_macros::async_test]
async fn every_mounted_surface_renders_against_its_own_view_state_while_one_pick_reaches_every_body() {
    let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🪟️surfaces/🧪️tests/🧫️fixtures/🪟️surface-view-state-routing/🔣️.json")).expect("surface view-state routing fixture");
    let node_key = fixture["pickedNodeKey"].as_str().expect("picked node");
    let domain = fixture["domainId"].as_str().expect("domain");
    let mut identity = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity();
    let mut app = VcsArtifactApp::with_registry(TestApp::<false>::default(), surface_routing_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut identity).await;
    semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy());
    reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": domain, "merge": "replace", "method": "pick" }), node_key)), &mut identity).await;
    assert_eq!(app.interaction_state().await.selection.get(domain).map(|selection| selection.ids.clone()), Some(vec![node_key.to_string()]), "the browser-shaped pick must be persisted before any surface renders");
    drop(PluginApp::take_pending_presence(&mut app).await);
    for _ in 0..(MAINTENANCE_STAGES as usize * ARTIFACT_LIVE_OUTPUT_SLOTS) {
        if !semio_framework_job_worker::worker_job_retirements_are_parked() {
            break;
        }
        let demand=PluginApp::maintenance_retirement_demands(&app,TYPED_OPERATION_RESULT_PAGE_BYTES).expect("original maintenance quote");let grant=semio_framework_plugin_ownership::bounded_ownership::plugin_demand_grant(demand);let step=PluginApp::maintenance_step(&mut app,grant).expect("bounded maintenance");if let PluginLifecycleStep::Progress(progress)|PluginLifecycleStep::Complete(progress)=step{assert!(progress.fits(grant));}
        semio_framework_plugin_job::reserved_jobs::plugin_job_yield_once().await;
    }

    let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
    runtime.instances.borrow_mut().insert_admitted(1, std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id: 1, app, surface_contexts: Default::default() }, runtime.mounted_policy)));
    let surfaces = fixture["surfaces"].as_array().expect("surfaces");
    let mut expected: BTreeMap<String, ViewModel> = BTreeMap::new();
    for surface in surfaces {
        let id = surface["id"].as_str().expect("surface id");
        let mounted = surface_routing_view(&fixture["base"], &surface["view"]);
        semio_framework_plugin_runtime_ops::ops_surface_render::plugin_mount_surface(&runtime, 1, id.into(), surface["bodyKey"].as_str().expect("body key").into(), &semio_framework_plugin_plugin_program::encode_wire_serialized(&mounted)).await.expect("surface mounts");
        expected.insert(id.into(), surface_routing_projection(&mounted));
    }

    for surface in surfaces {
        surface_routing_render(&runtime, surface["id"].as_str().expect("surface id"), &expected[surface["id"].as_str().expect("surface id")], node_key).await;
    }

    let synthetic = &fixture["syntheticSurface"];
    let synthetic_id = synthetic["id"].as_str().expect("synthetic surface id");
    let synthetic_view = surface_routing_view(&fixture["base"], &synthetic["view"]);
    let refused = semio_framework_plugin_runtime_ops::ops_surface_render::plugin_mount_surface(&runtime, 1, synthetic_id.into(), synthetic["bodyKey"].as_str().expect("body key").into(), &semio_framework_plugin_plugin_program::encode_wire_serialized(&synthetic_view))
        .await
        .expect_err("the synthetic `window` surface must never mount for an app with window instances");
    assert!(refused.message.contains(fixture["syntheticRefusal"].as_str().expect("refusal")), "the refusal must name the synthetic surface rule: {}", refused.message);
    assert!(semio_framework_plugin_runtime_ops::ops_surface_render::plugin_render_surface(&runtime, 1, synthetic_id).await.is_err(), "a refused synthetic surface must have no host context to render from");
    surface_routing_render(&runtime, fixture["pickedSurface"].as_str().expect("picked surface"), &expected[fixture["pickedSurface"].as_str().expect("picked surface")], node_key).await;
    surface_routing_render(&runtime, fixture["inspectionSurface"].as_str().expect("inspection surface"), &expected[fixture["inspectionSurface"].as_str().expect("inspection surface")], node_key).await;

    let refreshed = surface_routing_view(&fixture["base"], &fixture["refreshedView"]);
    semio_framework_plugin_runtime_ops::ops_surface_render::plugin_render(&runtime, 1, "graph", &serde_json::to_string(&refreshed).expect("host view serializes")).await.expect("host refresh renders");
    RENDER_CONTEXT_PROBE.with(|probe| probe.take());
    for surface in surfaces {
        let id = surface["id"].as_str().expect("surface id");
        let rebound = match expected[id].window_id.as_deref() {
            Some(window) => refreshed.for_window_instance(window).expect("live window instance"),
            None => refreshed.for_panel(),
        };
        surface_routing_render(&runtime, id, &rebound, node_key).await;
    }
}