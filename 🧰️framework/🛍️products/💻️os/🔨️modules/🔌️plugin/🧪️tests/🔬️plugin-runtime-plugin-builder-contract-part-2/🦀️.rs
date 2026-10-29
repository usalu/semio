//! 🔬️ Part 2 of 7 of the plugin builder contract laws (cut from `🔬️plugin-runtime-plugin-builder-contract`; the fixtures several parts use are the support modules).
//! 🧪️ The plugin contract's own unit test: a `TestApp` implementing the pure `ArtifactApp`
//! surface (B1), wrapped in `VcsArtifactApp`, exercising typed operations with true inverses, config
//! operations that emit no document operations, history interception, and remote-operation ingest
//! idempotency. `TestCommand` is `TestApp`'s typed `Self::Command`; framework-reserved verbs
//! (history/clipboard/revert/filter/noteShellCommand) still dispatch by string via `handle_action`/
//! `handle_command` — everything app-specific dispatches via `dispatch_typed`.


use semio_framework_plugin_plugin_program::AppInstance;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparation;
use semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparationStep;
use semio_framework_plugin_interaction::mutations::InteractionConfigMutation;
use protocol::LOCAL_ACTOR_ID;
use semio_framework_plugin_decl_views::NoConfigMutation;
use semio_framework_plugin_builder::Ready;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
use serde_json::Value;
use semio_framework_value::ValueError;
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_plugin_program::decode_wire_serialized;
use semio_framework_plugin_plugin_program::encode_wire_serialized;
use semio_framework_plugin_job::reserved_jobs::plugin_job_yield_once;
use semio_framework_plugin_plugin_program::plugin_runtime;
use semio_framework_os_kernel as protocol;
use semio_framework_plugin_job::reserved_job_outcomes;
use semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission;
use semio_framework_os_kernel as store;
use semio_framework_plugin_tool_run as tool_run;
use semio_framework_plugin_abi::ArtifactAnalysis;
use semio_framework_plugin_abi::ArtifactAnalyzer;
use semio_framework_plugin_abi::ArtifactBuilder;
use semio_framework_plugin_abi::ArtifactChildren;
use semio_framework_plugin_abi::ArtifactComposer;
use semio_framework_plugin_abi::ArtifactComposition;
use semio_framework_plugin_abi::ArtifactDecomposer;
use semio_framework_plugin_decl_surface::ArtifactEditor;
use semio_framework_plugin_decl_tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob;
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_decl_surface::ArtifactViewer;
use semio_framework_plugin_decl_emission::AsyncTaskRun;
use semio_framework_plugin_decl_emission::AsyncTaskSource;
use semio_framework_plugin_decl_views::child_view::ChildContentRetainers;
use semio_framework_plugin_abi::DerivedArtifactSpec;
use semio_framework_plugin_window_kits::LabelAxes;
use semio_framework_plugin_decl_app::NaturalFileDecodeCursor;
use semio_framework_os_kernel::io::SubsetValidator;
use semio_framework_plugin_decl_emission::ToolCommandCatalog;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_window_config::WindowConfigReplaceMutation;
use semio_framework_plugin_window_kits::WindowKit;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
use semio_framework_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::{KEYED_WINDOW_BODY_KEY, KeyedTestApp, TestApp, TestCommand, app_maintenance_turn, apply_test_mutations, interaction_app_raw, keyed_pick_carrier_scope, keyed_test_registry, meta, new_test_child, original_publication_turn, reactor_native_budget, test_child_dialect};
use semio_framework_ui_contract::*;
use semio_framework::*;
use semio_framework::kernel::*;
use semio_framework_plugin_reactor_app::__semio_dispatch_PluginApp;
use semio_framework_value::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin_decl_emission::{ActionMeta, ChildEmit, Emit};
use semio_framework_plugin_app_builder::App;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_window_kits::AppActionRegistry;
use semio_framework_plugin_decl_views::NoDraftMutation;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_abi::{ArtifactDeserializer, ArtifactSerializer};
use semio_framework_diagnostic::FaultFrom;
use semio_framework::IconName;
use semio_framework::ViewModel;
use semio_framework_os_kernel::Mutation;
use semio_framework::Fault;
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework::ActionKind;
use semio_framework_job::InteractiveJob as _;
use serde_json::json;
use semio_framework_artifact_reference::ArtifactRef;
use pack::snapshot::ArtifactPack;
use semio_framework_os_kernel::os_store::backbone::Backbone;
use semio_framework_os_kernel::{MemberFactory, SpaceMember};
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_slot_children::SetSlotChildren;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::TestMembers;

    mod local_interaction_dispatch {
        include!("../../🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs");
    }
    mod child_emission_refusal { include!("../🧩️composition/📨️emission/🦀️.rs"); }

    /// 🗝️ `keyed_test_registry` plus the declared `items` interaction domain a `PickItem`'s verb
    /// names — the registry the typed-ladder fold laws dispatch against. `PickItem` rides the
    /// fixture's one generated tool (`compositeEdit`, see `command_id`), so no second command is declared.
    async fn keyed_pick_test_registry() -> AppActionRegistry {
        let manifest = App::from_builder(
            App::builder(KeyedTestApp::APP_ID, LocalizedLabel::data("Keyed Fixture"))
                .await
                .document(["state"])
                .mode("edit", LocalizedLabel::data("Edit"), "pencil")
                .await
                .window_kind("main", LocalizedLabel::data("Main"), KEYED_WINDOW_BODY_KEY, SurfaceKind::Canvas2d, IconName::AppWindow)
                .await
                .app_command("compositeEdit", LocalizedLabel::data("Set Parameter"), "fixture", ActionKind::Mutation)
                .await
                .interaction(InteractionDefinition {
                    id: "items".into(),
                    label: LocalizedLabel::data("Items"),
                    granularities: vec![GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
                    hierarchy: HierarchyProvider::Topology,
                    hover: HoverSpec::default(),
                    selection: SelectionSpec {
                        modes: vec![SelectionMode::Multiple, SelectionMode::Single],
                        methods: vec![SelectionMethod::Pick],
                        merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive, MergeMode::Range],
                        transitive: false,
                        broadcast: true,
                    },
                })
                .await
                .window_kind_interactions("main", vec![InteractionRef::new("items")])
                .await
                .interactive_jobs(InteractiveJobClassification::Migrated)
                .await,
        )
        .await;
        AppActionRegistry::from_definition(&manifest.definition)
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_child_group_publishes_one_acknowledged_parent_child_gesture_and_retires() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        let id = 41;
        let mut app = VcsArtifactApp::<KeyedTestApp, TestMembers>::with_registry(KeyedTestApp, keyed_test_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await.expect("the reference snapshot births");
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(id).await;
        let declared = ArtifactRef { artifact_id: "child-1".into(), dialect: test_child_dialect().await }.to_uri();
        apply_test_mutations(app.stores.test_store_mut().await, vec![TestMutation::SetSlotChildren(SetSlotChildren { children: vec![declared] })]).await.expect("the actual parent declares the exact member before registration");
        let child = new_test_child("child-1").await.expect("construct child");
        app.register_child("slot", "child-1", test_child_dialect().await, child, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("register child");
        semio_framework_plugin_testing_laws::artifact_app_laws::dispatch_original_fixture_command(&mut app, &TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 9 }, &ActionMeta { actor: "fixture".into(), instance_id: id, view_state: None }, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut identity, cx).await.expect("admit retained child gesture");

        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        let cell = std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }, runtime.mounted_policy));
        runtime.instances.borrow_mut().insert_admitted(id, cell.clone());
        let acknowledgement_fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/🥇️tool-latest-wins.json")).expect("language-neutral result ACK fixture");
        let mut lanes = Vec::new();
        for turn in 0..100_000 {
            if let Err(error) = semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_live_cleanup(&runtime) {
                let pump = cell.maintenance_pump.lock().expect("failed maintenance pump");
                panic!(
                    "{error:?}: turn={turn} status={} generation={} stalled={} entries={} session={} outcome={} rejected={} terminal={} pending={:?} closing={} faulted={}",
                    cell.maintenance_status.load(std::sync::atomic::Ordering::SeqCst),
                    cell.maintenance_generation.load(std::sync::atomic::Ordering::SeqCst),
                    cell.maintenance_stalled_steps.load(std::sync::atomic::Ordering::SeqCst),
                    cell.maintenance_probe_entries.load(std::sync::atomic::Ordering::Relaxed),
                    pump.session.is_some(),
                    pump.outcome_pending,
                    pump.rejected.is_some(),
                    pump.terminal,
                    pump.pending_status,
                    pump.closing,
                    pump.faulted
                );
            }
            let (output, scan) = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_continue_typed_operations(&runtime, semio_framework_plugin_runtime_ops::ops_typed_operations::TypedOperationGrant::turn(reactor_native_budget(cx),cx), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("drive one production publication turn");
            let mut more = scan.runnable || scan.contended;
            if let Some((receiver, output)) = output {
                assert_eq!(receiver, id);
            }
            {
                for page in native_pages {
                    assert_ne!(page.lane, TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
                    lanes.push(page.lane);
                    if page.lane == TypedOperationResultLane::Child {
                        assert_eq!(page.token.attempt, acknowledgement_fixture["resultAck"]["attempt"].as_u64().unwrap() as u8);
                        let delayed_token = page.close(mounted_policy.close);
                        {
                            let active = cell.instance.lock().expect("delayed ACK retained app");
                            assert!(active.app.has_pending_typed_operations());
                            assert_eq!(active.app.typed_operation_result_state_for_test(delayed_token.operation), Some((delayed_token, TypedOperationResultLane::Child, true, true)));
                        }
                        let maintenance_entries_before = cell.maintenance_probe_entries.load(std::sync::atomic::Ordering::Relaxed);
                        let pre_ack_polls = acknowledgement_fixture["resultAck"]["preAckPolls"].as_u64().unwrap();
                        for poll in 1..=pre_ack_polls {
                            let expected_entries = maintenance_entries_before + poll;
                            for _ in 0..100_000 {
                                semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_live_cleanup(&runtime).expect("delayed renderer ACK remains valid maintenance input wait");
                                plugin_job_yield_once().await;
                                if cell.maintenance_probe_entries.load(std::sync::atomic::Ordering::Relaxed) >= expected_entries {
                                    break;
                                }
                            }
                            assert!(cell.maintenance_probe_entries.load(std::sync::atomic::Ordering::Relaxed) >= expected_entries, "production maintenance callback must execute before counting one delayed ACK poll");
                            let (output, _) = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_continue_typed_operations(&runtime, semio_framework_plugin_runtime_ops::ops_typed_operations::TypedOperationGrant::UNIT, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.expect("delayed renderer ACK does not fault continuation");
                            assert!(output.as_ref().and_then(|(_, output)| output.typed_operation_results.first()).is_none(), "presented result page must not be republished before an explicit retry deadline");
                        }
                        assert!(cell.maintenance_probe_entries.load(std::sync::atomic::Ordering::Relaxed) >= maintenance_entries_before + pre_ack_polls);
                        let input_waits_before_alignment = cell.maintenance_probe_input_waits.load(std::sync::atomic::Ordering::SeqCst);
                        for _ in 0..100_000 {
                            semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_live_cleanup(&runtime).expect("reach one processed exact ACK-wait maintenance stage");
                            plugin_job_yield_once().await;
                            if cell.maintenance_probe_input_waits.load(std::sync::atomic::Ordering::SeqCst) > input_waits_before_alignment
                                && semio_framework_plugin_reactor_lifecycle::life_cleanup_vocabulary::RuntimeMaintenanceStatus::from_repr(cell.maintenance_status.load(std::sync::atomic::Ordering::SeqCst)) == semio_framework_plugin_reactor_lifecycle::life_cleanup_vocabulary::RuntimeMaintenanceStatus::Ready
                                && cell.maintenance_stalled_steps.load(std::sync::atomic::Ordering::SeqCst) == 0
                            {
                                break;
                            }
                        }
                        assert!(cell.maintenance_probe_input_waits.load(std::sync::atomic::Ordering::SeqCst) > input_waits_before_alignment, "maintenance must process the presented result as an external input wait");
                        {
                            let active = cell.instance.lock().expect("stable delayed ACK retained app");
                            assert_eq!(active.app.typed_operation_result_state_for_test(delayed_token.operation), Some((delayed_token, TypedOperationResultLane::Child, true, true)));
                        }
                        assert_eq!(cell.maintenance_stalled_steps.load(std::sync::atomic::Ordering::SeqCst), 0);
                        semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_acknowledge_typed_operation_result(&runtime, delayed_token).await.expect("delayed exact retained result ACK");
                        more = true;
                    } else {
                        semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_acknowledge_typed_operation_result(&runtime, page.close(mounted_policy.close)).await.expect("ack exact retained result");
                        more = true;
                    }
                }
            }
            if !more {
                assert!(lanes.contains(&TypedOperationResultLane::Terminal), "runtime retired before its terminal page");
                eprintln!("retained Child runtime published every host lane and retired after {turn} turns");
                break;
            }
            plugin_job_yield_once().await;
        }
        assert!(lanes.contains(&TypedOperationResultLane::Child));
        assert!(lanes.contains(&TypedOperationResultLane::Terminal));
        {
            let mut active = cell.instance.lock().expect("live runtime app");
            assert!(!active.app.has_pending_typed_operations());
            assert_eq!(active.app.snapshot().expect("parent snapshot").count, 9);
            let TestMembers::Child(child) = &mut active.app.children.get_mut(&("slot".to_string(), "child-1".to_string())).expect("live child").member;
            assert_eq!(child.snapshot().expect("child snapshot").count, 9);

            let admitted = active.app.dispatch_action("undo", None, &ActionMeta { actor: "fixture".into(), instance_id: id, view_state: None }, &mut identity).await.expect("undo retained group");
            let undone = semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(&mut active.app, admitted, cx).await.expect("undo reserved-job commit");
            artifact_app_laws::settle_registered_typed_operation(&mut active.app, id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await.expect("undo publication");
            let skipped: Vec<String> = undone.diagnostics.iter().map(|diagnostic| diagnostic.message.clone()).collect();
            assert!(skipped.is_empty(), "the group undo skips no member of its own gesture: {skipped:?}");
            assert_eq!(active.app.snapshot().expect("undone parent snapshot").count, 0, "the parent's edit of the gesture is undone with its group (parent tail group {:?})", store::SpaceMember::tail_group_id(&*active.app.store).await);
            let TestMembers::Child(child) = &mut active.app.children.get_mut(&("slot".to_string(), "child-1".to_string())).expect("undone child").member;
            assert_eq!(child.snapshot().expect("undone child snapshot").count, 0);

            let admitted = active.app.dispatch_action("redo", None, &ActionMeta { actor: "fixture".into(), instance_id: id, view_state: None }, &mut identity).await.expect("redo retained group");
            semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission(&mut active.app, admitted, cx).await.expect("redo reserved-job commit");
            artifact_app_laws::settle_registered_typed_operation(&mut active.app, id, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(),cx).await.expect("redo publication");
            assert_eq!(active.app.snapshot().expect("redone parent snapshot").count, 9);
            let TestMembers::Child(child) = &mut active.app.children.get_mut(&("slot".to_string(), "child-1".to_string())).expect("redone child").member;
            assert_eq!(child.snapshot().expect("redone child snapshot").count, 9);
        }
        let retired = std::sync::Arc::downgrade(&cell);
        drop(cell);
        semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_destroy_app(&runtime, id).await.expect("begin runtime destroy");
        for _ in 0..100_000 {
            semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_close_cleanup(&runtime).expect("drive exact runtime close");
            if runtime.close_quarantine.borrow().get(id).is_none() {
                break;
            }
            std::thread::yield_now();
        }
        assert!(runtime.close_quarantine.borrow().get(id).is_none());
        assert!(retired.upgrade().is_none());

        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🔀️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — the PER-TURN law of the typed-operation
    /// ladder, observed the way the browser host observes it (`advance_typed_operation_output`
    /// drains one effect per turn from `typed_effect_outbox` and hands it to `routeHostEffects`):
    /// on NO turn of a typed app command does a guest-emitted interaction verb leave the reactor as
    /// an effect. The emit is fem2d's `canvasPointerUp` shape — no durable store lane, ONE
    /// `interactionSelect`, a partial window refresh — because that is the shape that leaked:
    /// `publish_mounted_typed_operation_unit` installed the completion and, in the SAME call, fell
    /// through to `emit.effects.pop()` → `typed_effect_outbox`, while the fold gate
    /// (`mounted_typed_inline_interaction_verbs_are_next`) had been consulted BEFORE the install
    /// and answered false on a still-`None` publication. Measured: 18 `replayShellCommand dispatch
    /// {"actionId":"interactionSelect"}` host lines per battery on the fem2d React lane.
    ///
    /// Laws asserted per turn and at the end: the verb is never an effect of any turn; no `Effect`
    /// result page is ever published; the selection snapshot moved inside the operation; the `Ui`
    /// progress scope the host receives is already WIDER than the carrier's own partial scope (the
    /// fold ran before the `Ui` page was minted); the fold's leftover `InteractionView` rides that
    /// same progress unit; the operation retires through `Ui` + `Terminal` and one completion.
    /// Fails-before: the first turn after admission yielded the `ReplayShellCommand`.
    ///
    /// 📡️ EXACTLY the host's per-turn effect drain — every effect this turn hands out.
    #[semio_framework_async_macros::async_test]
    async fn typed_ladder_never_hands_a_guest_interaction_verb_to_the_host_on_any_turn() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let mut publication_caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(7),mounted_policy.preparation,||Some(0),u64::MAX,4096);
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let id = 47u32;
        let mut app = VcsArtifactApp::<KeyedTestApp>::with_registry(KeyedTestApp, keyed_pick_test_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await.expect("the reference snapshot births");
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(id).await;
        let meta = ActionMeta { actor: "fixture".into(), instance_id: id, view_state: None };
        let outcome: Result<(), String> = async {
            let admitted = semio_framework_plugin_testing_laws::artifact_app_laws::dispatch_original_fixture_command(&mut app, &TestCommand::PickItem { id: "item-1".into() }, &meta, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut identity, cx).await.map_err(|error| format!("admit the pick: {error:?}"))?;
            if !admitted.requested_effects.is_empty() || !admitted.mutations.is_empty() {
                return Err(format!("the admitting result of a typed command carries no effect and no mutation: {:?}", admitted.requested_effects));
            }
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(30);
            let (mut turns, mut lanes, mut progress_units, mut completions) = (0usize, Vec::new(), Vec::new(), 0usize);
            while app.has_pending_typed_operations() {
                if std::time::Instant::now() >= deadline {
                    return Err(format!("the pick did not retire within 30 s: turns={turns} lanes={lanes:?}"));
                }
                app_maintenance_turn(&mut app,cx).map_err(|error| format!("maintenance: {error:?}"))?;
                original_publication_turn(&mut app,&mut identity,&mut publication_caller,mounted_policy.maintenance).map_err(|error| format!("turn {turns}: {error:?}"))?;
                turns += 1;
                while let Some(mut page) = semio_framework_plugin_testing_laws::artifact_app_laws::take_fixture_result_page(&mut app,id,semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation).expect("original fixture page presentation") {
                    if page.lane == TypedOperationResultLane::Fault {
                        return Err(format!("turn {turns}: fault page {}", String::from_utf8_lossy(page.bytes())));
                    }
                    lanes.push(page.lane);
                    if !app.acknowledge_typed_operation_result(semio_framework_plugin_testing_laws::artifact_app_laws::close_fixture_result_page(&mut page,semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close)).map_err(|error| format!("ack: {error:?}"))? {
                        return Err(format!("turn {turns}: exact result ACK rejected"));
                    }
                }
                while let Some(effect) = app.take_typed_operation_effect() {
                    let verb = match &effect {
                        Effect::ReplayShellCommand { action_id, .. } => semio_framework_plugin_interaction::interaction_topology::INTERACTION_ACTION_IDS.contains(&action_id.as_str()),
                        Effect::DispatchAction { action, .. } => semio_framework_plugin_interaction::interaction_topology::INTERACTION_ACTION_IDS.contains(&action.as_str()),
                        _ => false,
                    };
                    if verb {
                        return Err(format!("turn {turns}: a guest interaction verb left the reactor as a host effect: {effect:?} (lanes so far {lanes:?})"));
                    }
                }
                while app.take_typed_operation_event().is_some() {}
                while let Some(progress) = app.take_typed_operation_ui_progress() {
                    progress_units.push(progress);
                }
                while app.take_typed_operation_completion().await.map_err(|error| format!("completion: {error:?}"))?.is_some() {
                    completions += 1;
                }
                while let Some(reply) = app.take_local_interaction_query_reply() {
                    if let protocol::LocalInteractionQueryReply::Page { page } = reply {
                        let token = protocol::LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal };
                        if !app.acknowledge_local_interaction_query(&token) {
                            return Err("local-interaction ACK rejected".into());
                        }
                    }
                }
                std::thread::yield_now();
            }
            println!("typed-ladder pick retired after {turns} turns: lanes={lanes:?} completions={completions} progress={}", progress_units.len());
            if lanes.contains(&TypedOperationResultLane::Effect) {
                return Err(format!("no Effect result page may be minted for a verb that folds inline: {lanes:?}"));
            }
            if !lanes.contains(&TypedOperationResultLane::Ui) || !lanes.contains(&TypedOperationResultLane::Terminal) || completions != 1 {
                return Err(format!("the operation retires through Ui + Terminal with one completion witness: {lanes:?} / {completions}"));
            }
            let selected = app.interaction_state().await.selection.get("items").map(|selection| selection.ids.clone());
            if selected != Some(vec!["item-1".to_string()]) {
                return Err(format!("the pick landed inside the operation: {selected:?}"));
            }
            let Some(progress) = progress_units.last() else { return Err("the host receives one Ui progress unit".into()) };
            if progress.ui_scope == keyed_pick_carrier_scope() {
                return Err(format!("the Ui scope the host receives is widened by the folded verb's declared interaction refresh scope, got the bare carrier scope {:?}", progress.ui_scope));
            }
            match &progress.ui_scope {
                UiDirtyScope::Full => {}
                UiDirtyScope::Partial { window_bodies, .. } if window_bodies.iter().any(|body| body == KEYED_WINDOW_BODY_KEY) => {}
                other => return Err(format!("the carrier's own window body survives the merge: {other:?}")),
            }
            let Some(leftover) = progress.leftover.as_ref() else { return Err("the folded verb's leftover InteractionView rides the Ui progress unit".into()) };
            if leftover.operation != admitted.output.get("operationId").and_then(semio_framework_value::DslValue::as_str).and_then(|id| id.parse::<u64>().ok()).unwrap_or(u64::MAX) {
                return Err(format!("the leftover is tagged with the operation that folded it: {} vs {:?}", leftover.operation, admitted.output));
            }
            let ids = leftover.view.get("interactionView").and_then(|view| view.get("selectedIds")).and_then(semio_framework_value::DslValue::as_array).ok_or_else(|| format!("leftover shape: {leftover:?}"))?;
            if !ids.iter().any(|id| id.as_str() == Some("item-1")) {
                return Err(format!("leftover selected ids {ids:?}"));
            }
            let history = app.render_ctx().test_history().await;
            if history.commands.iter().any(|entry| entry.action_id == INTERACTION_SELECT_ACTION_ID || entry.kind == ActionKind::Interaction) {
                return Err("the folded verb is never a history row".into());
            }
            Ok(())
        }
        .await;
        semio_framework_plugin_testing_laws::artifact_app_laws::close_registered_fixture_app(&mut app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
        outcome.expect("typed-ladder per-turn fold law");
        publication_caller.retire(mounted_policy.close);

    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }