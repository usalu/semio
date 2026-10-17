//! 🔬️ Part 3 of 7 of the plugin builder contract laws (cut from `🔬️plugin-runtime-plugin-builder-contract`; the fixtures several parts use are the support modules).
//! 🧪️ The plugin contract's own unit test: a `TestApp` implementing the pure `ArtifactApp`
//! surface (B1), wrapped in `VcsArtifactApp`, exercising typed operations with true inverses, config
//! operations that emit no document operations, history interception, and remote-operation ingest
//! idempotency. `TestCommand` is `TestApp`'s typed `Self::Command`; framework-reserved verbs
//! (history/clipboard/revert/filter/noteShellCommand) still dispatch by string via `handle_action`/
//! `handle_command` — everything app-specific dispatches via `dispatch_typed`.


use semio_framework_plugin_decl_views::media_contract::ARTIFACT_OUTPUT_CHUNK_BYTES;
use semio_framework_plugin_plugin_program::AppInstance;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_plugin_operation::ArtifactDocumentAuthority;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletion;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparation;
use semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparationStep;
use semio_framework_ui_runtime::ComponentTree;
use semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND;
use protocol::LOCAL_ACTOR_ID;
use semio_framework_plugin_instance_ladders::ladders_plugin_app_impl::MAINTENANCE_STAGES;
use semio_framework_plugin_decl_views::NoTransient;
use semio_framework_plugin_decl_views::NoTransientMutation;
use semio_framework_plugin_decl_views::PeerPresenceRoot;
use semio_framework_plugin_builder::Ready;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_value::RetirementDemand;
use semio_framework_plugin_operation::TOOL_CANCELLATION_SLOTS;
use semio_framework_ui_runtime::TreeNode;
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
use serde_json::Value;
use semio_framework_value::ValueError;
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_instance_docs::docs_maintenance_ladder::item;
use semio_framework_os_kernel as protocol;
use semio_framework_os_kernel as store;
#[cfg(test)]
use semio_framework_plugin_tests_02::publication_mounted_test_hosts::publication_mounted_unit_2::test_mounted_original_owned_publication;
use semio_framework_plugin_composition_publication::test_original_pending_child_group_closure;
use semio_framework_plugin_abi::ArtifactAnalysis;
use semio_framework_plugin_abi::ArtifactAnalyzer;
use semio_framework_plugin_abi::ArtifactBuilder;
use semio_framework_plugin_abi::ArtifactChildren;
use semio_framework_plugin_abi::ArtifactComposer;
use semio_framework_plugin_abi::ArtifactComposition;
use semio_framework_plugin_abi::ArtifactDecomposer;
use semio_framework_plugin_decl_surface::ArtifactEditor;
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
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
use semio_framework_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::{KeyedTestApp, TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_SCHEMA, TEST_RETAINED_COMMAND_STEP_CALLS, TEST_RETAINED_COMMAND_TOOL, TestApp, TestCommand, TestRetainedCommandFactory, app_maintenance_turn, apply_test_mutations, close_member_admission_app, close_member_admission_fixture, close_reserved_app, contract_app, contract_app_under_test, contract_composed_app, contract_composed_app_raw, declare_test_child, drain_and_close_composed_fixture, drain_and_close_fixture, drive_job_close, dv, fixture_close_grant, interaction_app_under_test, interaction_target_args, keyed_test_registry, label_in, meta, new_bare_test_child, new_test_child, original_publication_turn, prepare_original_fixture_child, publish_presence_roster, reactor_native_budget, reads_child_count, register_test_child, reserved_action, sample_presence_peer, settle_reserved, test_child_dialect, test_child_emit, test_retained_command_payload};
use semio_framework_ui_contract::*;
use semio_framework::*;
use semio_framework::kernel::*;
use semio_framework_plugin_reactor_app::__semio_dispatch_PluginApp;
use semio_framework_plugin_instance_docs::docs_instance_root::artifact_handle_of;
use semio_framework_plugin_instance_children::children_plugin_app_impl::MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY;
use semio_framework_plugin_tests_01::retained_command_test_hosts::retained_command_unit_1::test_typed_operation_lands_its_edit_inside_the_admitting_call;
use semio_framework_plugin_tests_01::retained_command_test_hosts::retained_command_unit_1::test_typed_operation_completes_under_a_status_only_host_call;
use semio_framework_plugin_tests_01::retained_command_test_hosts::retained_command_unit_1::test_typed_operation_never_parks_a_turn_that_reports_no_runnable_work;
use semio_framework_plugin_tests_01::retained_command_test_hosts::retained_command_unit_1::test_typed_operation_slot_release_on_every_outcome;
use semio_framework_plugin_tests_01::retained_command_test_hosts::retained_command_unit_1::test_typed_operation_slot_retirement_under_storm;
use semio_framework_plugin_tests_01::retained_command_test_hosts::retained_command_unit_1::test_typed_operation_slot_preadmission;
use semio_framework_plugin_tests_01::retained_command_test_hosts::retained_command_unit_1::test_retained_keyed_dispatch;
use semio_framework_value::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_ui_locale::Locale;
use semio_framework_plugin_decl_emission::{ActionMeta, ChildEmit, Emit};
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_decl_views::media_contract::ArtifactDownloadOutput;
use semio_framework_plugin_decl_views::media_contract::ArtifactOutputChunks;
use semio_framework_plugin_decl_views::{NoPresence, NoPresenceMutation};
use semio_framework_plugin_decl_views::PeerPresence;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_abi::{ArtifactDeserializer, ArtifactSerializer};
use semio_framework_diagnostic::FaultFrom;
use semio_framework::IconName;
use semio_framework::ViewModel;
use semio_framework_os_kernel::Mutation;
use semio_framework::Fault;
use semio_framework::kernel::ArtifactHandle;
use semio_framework::kernel::Effect;
use semio_framework::NOTE_SHELL_COMMAND_ACTION_ID;
use semio_framework_job::InteractiveJob as _;
use semio_framework_ui_contract::SurfaceId;
use serde_json::json;
use semio_framework_artifact_reference::ArtifactRef;
use pack::snapshot::ArtifactPack;
use semio_framework_os_kernel::os_store::backbone::Backbone;
use semio_framework_os_kernel::{MemberFactory, SpaceMember};
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::{MAXIMUM_CHILD_CLONES, MAXIMUM_CHILD_ENCODINGS, MAXIMUM_CHILD_PROBE_BYTES};
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;


    /// 🎟️ Spends exactly the quoted close turn of a hosted app.
    fn app_close_turn<P: PluginApp>(app: &mut P,cx:&mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let demand = app.close_retirement_demands(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)?;
        let grant = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close;
        assert!(grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth,"app demand exceeds its original fixture close policy");
        let step = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,grant|Ok((app.close_step(same,grant)?,app.close_terminal_is_empty())))?;
        let _ = app.take_constructor_receipt();
        assert!(step.progress().is_none_or(|progress| progress.fits(grant)), "close receipt {step:?} exceeds its exact grant {grant:?}");
        Ok(step)
    }

    fn test_retained_wire_input(bus: &ActionBus, bytes: &[u8]) -> (ToolWireAdmission, action_bus::RetainedToolWireInput) {
        let (admission, mut input) = bus.begin_exact_wire(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, bytes.len()).expect("test retained command admission");
        for page in bytes.chunks(action_bus::TOOL_WIRE_PAGE_BYTES) {
            input.admit_page(action_bus::ToolWirePage::try_copy_from(page).expect("test retained wire page")).map_err(|(fault, _)| fault).expect("test retained page admission");
        }
        input.seal().expect("test retained wire seal");
        (admission, input)
    }

    #[test]
    fn declared_child_projection_default_preserves_empty_and_full_literal_references() {
        let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧩️composition/🔎️projection/🧪️tests/🧫️fixtures/🔣️.json"))).expect("closed projection fixture");
        for case in fixture["cases"].as_array().expect("projection cases") {
            let rows = case["rows"].as_array().expect("declared rows");
            let text = |row: &Value, name: &str| row[name].as_str().expect("literal child field").to_string();
            let snapshot = TestSnapshot { count: i32::MIN, label: "parent\0引用😀".into(), slot: rows.iter().map(|row| store::os_store::artifact_child::ArtifactChild::new(text(row, "childId"), ArtifactRef { artifact_id: text(row, "artifactId"), dialect: ArtifactDialect { artifact_kind: text(row, "kind"), standard: text(row, "standard"), subset: text(row, "subset") } })).collect() };
            let projected = <KeyedTestApp as ArtifactApp>::child_restore_projection(&snapshot).expect("default projects the actual declared snapshot");
            assert_eq!(projected.len(), rows.len());
            assert_eq!(projected.is_empty(), rows.is_empty());
            for (index, row) in rows.iter().enumerate() {
                let (slot, actual) = projected.get(index).expect("full child occurrence");
                assert_eq!(slot, row["slot"].as_str().expect("slot"));
                assert_eq!(actual.child_id, row["childId"].as_str().expect("logical"));
                assert_eq!(actual.artifact_id, row["artifactId"].as_str().expect("addressed"));
                assert_eq!(actual.artifact_kind, row["kind"].as_str().expect("kind"));
                assert_eq!(actual.standard, row["standard"].as_str().expect("standard"));
                assert_eq!(actual.subset, row["subset"].as_str().expect("subset"));
            }
            let structural = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(&snapshot).expect("independent declared structural projection");
            projected.admit_complete((0..structural.len()).map(|index| structural.get(index).expect("declared row"))).expect("complete declared set");
            if !rows.is_empty() { assert!(projected.admit_complete(std::iter::empty()).is_err()); }
        }
    }

    #[test]
    fn keyed_fixture_no_state_disposers_and_retirement_factories_close_live_owners() {
        use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
        let page = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
        fn close_root<T: Send + Sync + 'static>(factory: std::sync::Arc<dyn store::SnapshotRetirementFactory<T>>, root: T) {
            let page = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
            let admission = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation;
            let (mut retirement, born) = match store::SnapshotRetirementFactory::retire(factory.as_ref(), std::sync::Arc::new(root), admission) {
                Ok(admitted) => admitted,
                Err((error, _)) => panic!("no-state root retirement constructor refused its page grant: {error:?}"),
            };
            assert!(born.fits(admission));
            for _ in 0..4 {
                if store::ErasedSnapshotRetirement::terminal_is_empty(retirement.as_ref()) {
                    return;
                }
                let copy = store::ErasedSnapshotRetirement::next_copy_byte_demand(retirement.as_ref()).expect("quoted copy demand");
                let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: store::ErasedSnapshotRetirement::next_capacity_byte_demand(retirement.as_ref(), copy).expect("quoted capacity demand"), maximum_release_bytes: store::ErasedSnapshotRetirement::next_release_byte_demand(retirement.as_ref()).expect("quoted release demand"), maximum_depth: store::ErasedSnapshotRetirement::next_depth_demand(retirement.as_ref()).expect("quoted depth demand").max(1) };
                let unfunded = store::ErasedSnapshotRetirement::close_step(retirement.as_mut(), RetainedCloneGrant { maximum_items: 0, ..grant }).expect("zero-item turn yields");
                assert_eq!(unfunded.progress(), RetainedCloneProgress::default());
                let step = store::ErasedSnapshotRetirement::close_step(retirement.as_mut(), grant).expect("bounded root retirement");
                assert!(step.progress().fits(grant));
                if matches!(step, store::RetainedCloneStep::Complete(_)) {
                    assert!(store::ErasedSnapshotRetirement::terminal_is_empty(retirement.as_ref()));
                    return;
                }
            }
            panic!("no-state root retirement exceeded its bounded close turns");
        }

        close_root(<KeyedTestApp as ArtifactApp>::build_presence_local_root_retirement_factory().expect("presence local factory"), NoPresence::default());
        close_root(<KeyedTestApp as ArtifactApp>::build_presence_peer_retirement_factory().expect("presence peer factory"), NoPresence::default());
        close_root(<KeyedTestApp as ArtifactApp>::build_transient_local_root_retirement_factory().expect("transient local factory"), NoTransient::default());

        let mut presence = store::PresenceStore::<NoPresence, NoPresenceMutation>::new(NoPresence::default());
        presence.install_local_retirement_factory(<KeyedTestApp as ArtifactApp>::build_presence_local_root_retirement_factory().expect("presence local factory")).expect("presence local factory installs once");
        presence.install_peer_retirement_factory(<KeyedTestApp as ArtifactApp>::build_presence_peer_retirement_factory().expect("presence peer factory")).expect("presence peer factory installs once");
        let mut presence_disposer = <KeyedTestApp as ArtifactApp>::build_presence_store_disposer().expect("presence disposer");
        let presence_grant = fixture_close_grant(presence_disposer.retirement_demands(&presence, page).expect("quoted presence demand"));
        assert_eq!(presence_disposer.close_step(&mut presence, RetainedCloneGrant { maximum_items: 0, ..presence_grant }).expect("zero grant"), PluginLifecycleStep::Progress(RetainedCloneProgress::default()));
        for _ in 0..8 {
            let grant = fixture_close_grant(presence_disposer.retirement_demands(&presence, page).expect("quoted presence demand"));
            let step = presence_disposer.close_step(&mut presence, grant).expect("presence close");
            assert!(step.progress().expect("presence close receipt").fits(grant));
            if matches!(step, PluginLifecycleStep::Complete(_)) {
                break;
            }
        }
        assert!(presence_disposer.terminal_is_empty(&presence));
        assert!(presence.retirement_started() && presence.peers_root().is_empty());

        let mut transient = store::TransientStore::<NoTransient, NoTransientMutation>::new(NoTransient::default());
        let mut transient_disposer = <KeyedTestApp as ArtifactApp>::build_transient_store_disposer().expect("transient disposer");
        let original_transient_root = transient.current_root();
        let original_transient_weak = std::sync::Arc::downgrade(&original_transient_root);
        let original_transient_generation = transient.generation_now();
        let quoted = transient_disposer.retirement_demands(&transient, page).expect("quoted transient demand");
        let exact_grant = fixture_close_grant(quoted);
        assert_eq!(transient_disposer.close_step(&mut transient, RetainedCloneGrant { maximum_items: 0, ..exact_grant }).expect("zero grant"), PluginLifecycleStep::Progress(RetainedCloneProgress::default()));
        assert!(!transient_disposer.terminal_is_empty(&transient));
        for starved in [
            (quoted.copy_bytes != 0).then(|| RetainedCloneGrant { maximum_copy_bytes: quoted.copy_bytes - 1, ..exact_grant }),
            (quoted.capacity_bytes != 0).then(|| RetainedCloneGrant { maximum_capacity_bytes: quoted.capacity_bytes - 1, ..exact_grant }),
            (quoted.release_bytes != 0).then(|| RetainedCloneGrant { maximum_release_bytes: quoted.release_bytes - 1, ..exact_grant }),
        ]
        .into_iter()
        .flatten()
        {
            assert_eq!(transient_disposer.close_step(&mut transient, starved).expect("short transient grant"), PluginLifecycleStep::Progress(RetainedCloneProgress::default()));
            assert_eq!(transient.generation_now(), original_transient_generation);
            assert!(std::sync::Arc::ptr_eq(&original_transient_root, &transient.current_root()));
        }
        for _ in 0..8 {
            if !std::sync::Arc::ptr_eq(&original_transient_root, &transient.current_root()) {
                break;
            }
            let grant = fixture_close_grant(transient_disposer.retirement_demands(&transient, page).expect("quoted transient demand"));
            let step = transient_disposer.close_step(&mut transient, grant).expect("transient owned-store retirement");
            assert!(step.progress().expect("transient receipt").fits(grant));
        }
        assert_eq!(transient.generation_now(), 0);
        assert!(!std::sync::Arc::ptr_eq(&original_transient_root, &transient.current_root()));
        drop(original_transient_root);
        assert!(original_transient_weak.upgrade().is_none());
        for _ in 0..8 {
            if transient_disposer.terminal_is_empty(&transient) {
                break;
            }
            let grant = fixture_close_grant(transient_disposer.retirement_demands(&transient, page).expect("quoted transient demand"));
            let step = transient_disposer.close_step(&mut transient, grant).expect("transient terminal completion");
            assert!(step.progress().expect("transient receipt").fits(grant));
        }
        assert!(transient_disposer.terminal_is_empty(&transient));
        let repeated = fixture_close_grant(transient_disposer.retirement_demands(&transient, page).expect("quoted terminal demand"));
        assert!(matches!(transient_disposer.close_step(&mut transient, repeated).expect("transient repeated complete"), PluginLifecycleStep::Complete(progress) if progress == RetainedCloneProgress::default()));
        transient = store::TransientStore::new(NoTransient::default());
        let drifted_transient_root = transient.current_root();
        assert!(!transient_disposer.terminal_is_empty(&transient));
        assert!(transient_disposer.close_step(&mut transient, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation).is_err(), "transient terminal witness rejects owner drift");
        assert!(std::sync::Arc::ptr_eq(&drifted_transient_root, &transient.current_root()));
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_latest_wins_registered_dispatch_rebases_worker_and_publishes_real_document() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit keyed fixture identity authority");
        test_retained_keyed_dispatch::<KeyedTestApp>(
            keyed_test_registry().await,
            |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value },
            |value| TestMutation::SetCount(SetCount { value }),
            |snapshot| snapshot.count,
            None,
            mounted_policy,
            &mut identity,
        )
        .await;
    }

    #[semio_framework_async_macros::async_test]
    async fn typed_operation_ingress_pre_admits_the_exact_slot_before_it_mints_an_operation_id() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit keyed fixture identity authority");
        test_typed_operation_slot_preadmission::<KeyedTestApp>(keyed_test_registry().await, "compositeEdit", |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value }, mounted_policy, &mut identity).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn every_admitted_typed_operation_slot_is_released_by_the_host_continuation_that_reports_it_runnable() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit keyed fixture identity authority");
        test_typed_operation_slot_retirement_under_storm::<KeyedTestApp>(keyed_test_registry().await, "compositeEdit", |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value }, mounted_policy, &mut identity).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn a_settled_a_replaced_and_a_cancelled_typed_operation_all_release_their_exact_slot() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit keyed fixture identity authority");
        test_typed_operation_slot_release_on_every_outcome::<KeyedTestApp>(keyed_test_registry().await, "compositeEdit", |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value }, mounted_policy, &mut identity).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn a_mounted_typed_operation_never_parks_a_turn_that_reports_no_runnable_work() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit keyed fixture identity authority");
        test_typed_operation_never_parks_a_turn_that_reports_no_runnable_work::<KeyedTestApp>(keyed_test_registry().await, "compositeEdit", |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value }, mounted_policy, &mut identity).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn a_status_only_host_call_finishes_every_typed_operation_it_admitted() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit keyed fixture identity authority");
        test_typed_operation_completes_under_a_status_only_host_call::<KeyedTestApp>(keyed_test_registry().await, "compositeEdit", |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value }, mounted_policy, &mut identity).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn an_admitting_host_call_hands_back_the_terminal_lane_it_earned() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit keyed fixture identity authority");
        test_typed_operation_lands_its_edit_inside_the_admitting_call::<KeyedTestApp>(keyed_test_registry().await, "compositeEdit", |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value }, mounted_policy, &mut identity).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn microsecond_registered_factory_dispatch_preserves_exact_half_ms_fake_clock() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit keyed fixture identity authority");
        fn clock() -> Option<u64> {
            Some(1_000)
        }
        test_retained_keyed_dispatch::<KeyedTestApp>(
            keyed_test_registry().await,
            |target, value| TestCommand::CompositeEdit { slot: String::new(), child_id: target.into(), child_value: value },
            |value| TestMutation::SetCount(SetCount { value }),
            |snapshot| snapshot.count,
            Some(clock),
            mounted_policy,
            &mut identity,
        )
        .await;
    }

    /// ⏱️ Continuation turns a staged operation may spend beyond one per acknowledgeable result page —
    /// the terminating turn on which the scan finally answers "nothing runnable".
    const TYPED_OPERATION_CONTINUATION_SLACK: u64 = 1;

    /// 📄️ LAW: k live operations each holding a presentable result page retire in ONE host crossing.
    ///
    /// 🐛️ `advance_typed_operation_output` took exactly one page and `TypedOperationGrant::spent` ended
    /// the turn the instant it existed, so every acknowledgeable publication unit of every live
    /// operation cost its own worker round trip. Served measurement on the React door of 🧊️generation3d
    /// (`📓️shard-message-crossings-2026-09-15.md` §2): 200 acknowledgement crossings over 69
    /// operations, **every one of them exactly one ack wide**, 10.0 of a `flowEvalTick` hop's 36.2
    /// crossings. A page's ACK is per-page by protocol; the crossing that carries it is not — the wire's
    /// `Event::Message` is a list and the reactor's poll acknowledges every entry of it.
    ///
    /// The law is about CARDINALITY, so it admits whatever lane each page carries: four simultaneous
    /// edits of one immutable document root make three of them rebase refusals, and a refusal is
    /// published as its own acknowledgeable `Fault` page on exactly the wire this law measures.
    #[semio_framework_async_macros::async_test]
    async fn concurrent_typed_operations_hand_every_presentable_page_to_one_turn() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("closed finite original reactor builder caller");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        const OPERATIONS: usize = 4;
        let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
        let id = fixture["wire"]["receiver"].as_u64().unwrap() as u32;
        let mut app = VcsArtifactApp::<KeyedTestApp>::with_registry(KeyedTestApp, keyed_test_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(id).await;
        let meta = ActionMeta { actor: "fixture".into(), instance_id: id, view_state: None };
        for index in 0..OPERATIONS {
            let command = TestCommand::CompositeEdit { slot: String::new(), child_id: format!("batch-{index}").into(), child_value: index as i32 + 1 };
            app.dispatch_typed(command, &meta, &mut identity,cx).await.unwrap();
        }
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        let cell = std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }, runtime.mounted_policy));
        runtime.instances.borrow_mut().insert_admitted(id, cell.clone());
        let (mut pages, mut crossings, mut widest) = (0usize, 0usize, 0usize);
        for _ in 0..fixture["command"]["maximumTurns"].as_u64().unwrap() * OPERATIONS as u64 {
            semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_live_cleanup(&runtime,cx).unwrap();
            let (output, scan) = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_continue_typed_operations(&runtime, semio_framework_plugin_runtime_ops::ops_typed_operations::TypedOperationGrant::turn(reactor_native_budget(cx),cx), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.unwrap();
            let more = scan.runnable || scan.contended;
            if let Some((receiver, output)) = output {
                assert_eq!(receiver, id);
                if !output.typed_operation_results.is_empty() {
                    crossings += 1;
                    widest = widest.max(output.typed_operation_results.len());
                }
                for page in output.typed_operation_results {
                    pages += 1;
                    semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_acknowledge_typed_operation_result(&runtime, page.token).await.unwrap();
                }
            }
            if !more {
                break;
            }
            std::thread::yield_now();
        }
        assert!(pages >= OPERATIONS, "each of the {OPERATIONS} admitted operations owes at least one result page, saw {pages}");
        assert!(widest >= 2, "{OPERATIONS} concurrently publishing operations must share a crossing; the widest batch was {widest}");
        assert!(crossings < pages, "{pages} result pages retired in {crossings} crossings — a page per crossing is the cardinality this law removes");
        let active = cell.instance.lock().unwrap();
        assert!(!active.app.has_pending_typed_operations());
        drop(active);
        drop(cell);
        semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_destroy_app(&runtime, id).await.unwrap();
        for _ in 0..100_000 {
            semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_close_cleanup(&runtime,cx).unwrap();
            if runtime.close_quarantine.borrow().get(id).is_none() {
                break;
            }
        }
        }.await;let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("same original reactor builder caller terminal retirement");result
    }

    /// 🧮️ A many-mutation edit publishes every one of its mutations and reaches its terminal page,
    /// spending many publication units under the stall guard without being terminated as stalled.
    ///
    /// 🩺️ The guard (`fault_stalled_typed_operation_publication`) terminates a `Publishing`
    /// operation once 4 096 consecutive units leave its stall witness unchanged, and the witness
    /// had no term for the ladder's own progress. This law is the biggest single document
    /// publication the store admits — `ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES` is 1 MiB and one
    /// unit spends up to `TYPED_OPERATION_RESULT_PAGE_BYTES` of it — which is why it is expressed
    /// on the fold being COMPLETE and fault-free rather than on a unit count: a single batched
    /// document publication cannot reach 4 096 units at all (measured: 4 400 mutations = 126 units,
    /// 150 000 are refused as "batched preparation footprint exceeds its fixed item or byte
    /// capacity").
    #[semio_framework_async_macros::async_test]
    async fn a_many_mutation_publication_folds_every_mutation_and_is_never_terminated_as_stalled() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("closed finite original reactor builder caller");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        const ROWS: i32 = 4_400;
        let id = 4_103;
        let mut app = VcsArtifactApp::<KeyedTestApp>::with_registry(KeyedTestApp, keyed_test_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(id).await;
        app.dispatch_typed(TestCommand::BulkEdit { rows: ROWS }, &ActionMeta { actor: "fixture".into(), instance_id: id, view_state: None }, &mut identity).await.unwrap();
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        let cell = std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }, runtime.mounted_policy));
        runtime.instances.borrow_mut().insert_admitted(id, cell.clone());
        let mut terminal = false;
        let mut faults: Vec<String> = Vec::new();
        let mut units = 0u64;
        for _ in 0..(ROWS as u64 * 8) {
            let (output, scan) = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_continue_typed_operations(&runtime, semio_framework_plugin_runtime_ops::ops_typed_operations::TypedOperationGrant::UNIT, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.unwrap();
            units += 1;
            if let Some((receiver, output)) = output {
                assert_eq!(receiver, id);
                for page in output.typed_operation_results {
                    if page.lane == TypedOperationResultLane::Fault {
                        faults.push(String::from_utf8_lossy(page.bytes()).into_owned());
                    }
                    terminal |= page.lane == TypedOperationResultLane::Terminal;
                    semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_acknowledge_typed_operation_result(&runtime, page.token).await.unwrap();
                }
            }
            if !(scan.runnable || scan.contended) {
                break;
            }
        }
        assert!(faults.is_empty(), "a moving publication ladder must never be terminated as stalled, saw {faults:?} after {units} publication units");
        assert!(terminal, "the {ROWS}-mutation publication owes a terminal result page, saw {units} publication units");
        assert!(units > 64, "this law only exercises the stall guard while the publication spends more units than its 64-unit witness floor, it spent {units}");
        let active = cell.instance.lock().unwrap();
        assert_eq!(active.app.snapshot().unwrap().count, ROWS, "every one of the {ROWS} folded mutations reached the document");
        assert!(!active.app.has_pending_typed_operations());
        drop(active);
        drop(cell);
        semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_destroy_app(&runtime, id).await.unwrap();
        for _ in 0..100_000 {
            semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_close_cleanup(&runtime,cx).unwrap();
            if runtime.close_quarantine.borrow().get(id).is_none() {
                break;
            }
        }
        }.await;let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("same original reactor builder caller terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn a_host_crossing_outranks_every_background_hold_on_the_same_app() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let id = 4_101;
        let mut app = VcsArtifactApp::<KeyedTestApp>::with_registry(KeyedTestApp, keyed_test_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(id).await;
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        let cell = std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }, runtime.mounted_policy));
        runtime.instances.borrow_mut().insert_admitted(id, cell.clone());
        let background_entered = std::sync::atomic::AtomicBool::new(false);
        std::thread::scope(|scope| {
            let holder = scope.spawn(|| {
                let held = cell.background_instance().expect("an idle app admits a background turn");
                background_entered.store(true, std::sync::atomic::Ordering::SeqCst);
                while cell.host_crossings.load(std::sync::atomic::Ordering::SeqCst) == 0 {
                    std::thread::yield_now();
                }
                drop(held);
            });
            while !background_entered.load(std::sync::atomic::Ordering::SeqCst) {
                std::thread::yield_now();
            }
            assert_eq!(cell.background_holds.load(std::sync::atomic::Ordering::SeqCst), 1, "a live background turn counts its own hold so the host can tell it from an owner that never releases");
            let crossing = cell.host_instance();
            assert!(crossing.is_some(), "a host crossing waits out a background turn instead of being refused over it");
            let crossing = crossing.expect("the host crossing was admitted");
            assert_eq!(cell.host_crossings.load(std::sync::atomic::Ordering::SeqCst), 1);
            assert_eq!(cell.background_holds.load(std::sync::atomic::Ordering::SeqCst), 0);
            assert!(matches!(cell.background_instance(), Err(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeInstanceRefusal::Contended)), "a background turn never competes with a live host crossing");
            drop(crossing);
            holder.join().expect("the background holder released its turn to the host crossing");
        });
        assert_eq!(cell.host_crossings.load(std::sync::atomic::Ordering::SeqCst), 0);
        assert!(cell.background_instance().is_ok(), "the app is free again once the crossing ends");
        drop(cell);
        semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_destroy_app(&runtime, id).await.unwrap();
        for _ in 0..100_000 {
            semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_close_cleanup(&runtime).unwrap();
            if runtime.close_quarantine.borrow().get(id).is_none() {
                break;
            }
        }
    }

    /// ⚖️ LAW: a staged operation costs the host ONE turn per acknowledgeable result page, not one
    /// per publication unit. The host has to come back for a result page because the page is only
    /// released by its own ACK; every other unit is internal to the guest, and answering `MoreWork`
    /// after each of them cost this fixture 48 turns before ticket 26/09/02 wave B24.
    /// 🧮️ A turn the operation was not even schedulable on — `contended` without `runnable`, the
    /// shared worker pool busy with somebody else — is the machine's pacing, not the continuation's,
    /// and is not counted: on a loaded machine it drew one such turn in roughly one run in five
    /// (measured 2026-09-21, trace `[(false, true, true), (true, false, true), …]`) while every
    /// runnable turn still carried its exact receipt. The ceiling constant is untouched.
    #[semio_framework_async_macros::async_test]
    async fn retained_operation_continues_after_command_admission_until_publication_and_retirement() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("closed finite original reactor builder caller");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
        let id = fixture["wire"]["receiver"].as_u64().unwrap() as u32;
        let mut app = VcsArtifactApp::<KeyedTestApp>::with_registry(KeyedTestApp, keyed_test_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut app, mounted_policy);
        app.bind_instance_id(id).await;
        let value = fixture["command"]["value"].as_i64().unwrap() as i32;
        let command = TestCommand::CompositeEdit { slot: String::new(), child_id: fixture["command"]["target"].as_str().unwrap().into(), child_value: value };
        app.dispatch_typed(command, &ActionMeta { actor: "fixture".into(), instance_id: id, view_state: None }, &mut identity,cx).await.unwrap();
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        let cell = std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id, app, surface_contexts: Default::default() }, runtime.mounted_policy));
        runtime.instances.borrow_mut().insert_admitted(id, cell.clone());
        let mut terminal = false;
        let mut receipts = 0;
        let mut spent = 0;
        let mut turns: Vec<(bool, bool, bool)> = Vec::new();
        for turn in 0..fixture["command"]["maximumTurns"].as_u64().unwrap() {
            semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_live_cleanup(&runtime,cx).unwrap();
            let (output, scan) = semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_continue_typed_operations(&runtime, semio_framework_plugin_runtime_ops::ops_typed_operations::TypedOperationGrant::turn(reactor_native_budget(cx),cx), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity(), cx).await.unwrap();
            let more = scan.runnable || scan.contended;
            spent += u64::from(scan.runnable || !scan.contended);
            turns.push((scan.runnable, scan.contended, output.is_some()));
            if let Some((receiver, output)) = output {
                assert_eq!(receiver, id);
                for page in output.typed_operation_results {
                    assert_ne!(page.lane, TypedOperationResultLane::Fault, "{}", String::from_utf8_lossy(page.bytes()));
                    terminal |= page.lane == TypedOperationResultLane::Terminal;
                    receipts += 1;
                    semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_acknowledge_typed_operation_result(&runtime, page.token).await.unwrap();
                }
            }
            if !more {
                assert!(terminal, "the actor must remain runnable until its terminal result");
                let _ = turn;
                break;
            }
            std::thread::yield_now();
        }
        assert!(
            spent <= receipts + TYPED_OPERATION_CONTINUATION_SLACK,
            "a {receipts}-receipt operation spent {spent} continuation turns (ceiling {}) — the continuation is pacing one publication unit per host round trip; turns (runnable, contended, produced): {turns:?}",
            receipts + TYPED_OPERATION_CONTINUATION_SLACK
        );
        let active = cell.instance.lock().unwrap();
        assert!(terminal);
        assert!(!active.app.has_pending_typed_operations());
        assert_eq!(active.app.snapshot().unwrap().count, value);
        drop(active);
        let retired = std::sync::Arc::downgrade(&cell);
        drop(cell);
        semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_destroy_app(&runtime, id).await.unwrap();
        for _ in 0..100_000 {
            if let Err(error) = semio_framework_plugin_runtime_instances::inst_cleanup_turns::plugin_step_close_cleanup(&runtime,cx) {
                let entries = runtime.close_quarantine.borrow();
                let state = &entries.get(id).unwrap().state;
                let pump = state.pump.lock().unwrap();
                let detail = state.last_fault.lock().unwrap();
                panic!(
                    "{error:?}: elapsed={} stalled={} terminal={} complete={} blocked={} faulted={} pending={:?} origin={} detail={}",
                    state.last_callback_elapsed_us.load(std::sync::atomic::Ordering::SeqCst),
                    state.stalled_steps.load(std::sync::atomic::Ordering::SeqCst),
                    pump.terminal,
                    pump.complete,
                    pump.blocked,
                    pump.faulted,
                    pump.pending_status,
                    state.last_fault_origin.load(std::sync::atomic::Ordering::SeqCst),
                    String::from_utf8_lossy(&detail[..])
                );
            }
            if runtime.close_quarantine.borrow().get(id).is_none() {
                break;
            }
            std::thread::yield_now();
        }
        assert!(runtime.close_quarantine.borrow().get(id).is_none());
        assert!(retired.upgrade().is_none());
        }.await;let fuel=original_context.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("same original reactor builder caller terminal retirement");result
    }

    /// 🪜️ Park the ladder exactly ON the segment stage without touching it. Every stage before it
    /// is bookkeeping that releases no bytes, and the segment stage alone refuses a grant under one
    /// chunk (`maximum_bytes < ARTIFACT_OUTPUT_CHUNK_BYTES`), so a sub-chunk budget walks the ladder
    /// to that stage and then stands still there — which is itself the bounded-grant refusal.
    #[semio_framework_async_macros::async_test]
    async fn app_close_step_drains_at_most_one_segment_and_one_chunk_budget() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app_under_test(mounted_policy, &mut identity).await;
        app.close.close_started = true;
        app.close.close_cancellation_cursor = TOOL_CANCELLATION_SLOTS + ARTIFACT_LIVE_OUTPUT_SLOTS;
        app.close.close_media_cursor = ARTIFACT_LIVE_OUTPUT_SLOTS;
        app.close.close_media_cleanup_cursor = ARTIFACT_LIVE_OUTPUT_SLOTS;
        let chunks = ArtifactOutputChunks::new(ARTIFACT_OUTPUT_CHUNK_BYTES * 2);
        assert_eq!(chunks.push(vec![1; ARTIFACT_OUTPUT_CHUNK_BYTES]), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES));
        assert_eq!(chunks.push(vec![2; ARTIFACT_OUTPUT_CHUNK_BYTES]), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES * 2));
        assert_eq!(chunks.seal(), Ok(ARTIFACT_OUTPUT_CHUNK_BYTES * 2));
        assert!(app.media.segmented_downloads.insert(37, ArtifactDownloadOutput::new("close.bin", "application/octet-stream", None, chunks.clone()).expect("sealed close output")).is_ok());
        let mut parked = false;
        for _ in 0..(MAINTENANCE_STAGES as usize * ARTIFACT_LIVE_OUTPUT_SLOTS) {
            let demand = app.close_retirement_demands(ARTIFACT_OUTPUT_CHUNK_BYTES).expect("quoted pre-segment close demand");
            let sub_chunk = store::RetainedCloneGrant { maximum_release_bytes: demand.release_bytes.min(ARTIFACT_OUTPUT_CHUNK_BYTES - 1), ..fixture_close_grant(demand) };
            match { let step = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,sub_chunk,demand,|same,grant|Ok((app.close_step(same,grant)?,app.close_terminal_is_empty()))).expect("original bounded pre-segment close slice"); let _ = app.take_constructor_receipt(); step } {
                semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress) if progress == store::RetainedCloneProgress::default() => {
                    assert_eq!(demand.release_bytes, ARTIFACT_OUTPUT_CHUNK_BYTES, "the ladder parked before its segmented-download rung");
                    parked = true;
                    break;
                }
                step => assert_eq!(step.progress().map(|progress| progress.released_bytes), Some(0), "a pre-segment close stage released bytes under a sub-chunk grant: {step:?}"),
            }
        }
        assert!(parked, "the close ladder never reached its segmented-download stage");
        assert_eq!(chunks.chunks_remaining(), 2, "walking the ladder to the segment stage released no chunk");
        assert_eq!(app_close_turn(&mut *app,cx).expect("first close slice"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(store::RetainedCloneProgress { copied_items: 1, released_bytes: ARTIFACT_OUTPUT_CHUNK_BYTES, ..Default::default() }));
        assert_eq!(chunks.chunks_remaining(), 1);
        assert_eq!(app_close_turn(&mut *app,cx).expect("second close slice"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(store::RetainedCloneProgress { copied_items: 1, released_bytes: ARTIFACT_OUTPUT_CHUNK_BYTES, ..Default::default() }));
        assert_eq!(chunks.chunks_remaining(), 0);
        assert_eq!(app_close_turn(&mut *app,cx).expect("terminal close slice"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(store::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        assert!(!app.media.segmented_downloads.contains(37));
    
        
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn shared_retained_command_checkpoint_resumes_exact_cursor_and_cancels_with_bounded_close() {
        let bus = ActionBus::new();
        bus.register(TestRetainedCommandFactory::new()).expect("test retained command factory");
        let command = TestCommand::SetLabel { value: "wire".into() };
        let wire = <TestCommand as protocol::OpBinary>::encode_op(&command).expect("test retained command wire");
        let operation = semio_framework_job::Operation::new(semio_framework_job::OperationId(41), semio_framework_job::RevisionId(2), semio_framework_job::Generation(3), 5);
        let policy=semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy();
        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation,operation.generation,policy.preparation,||Some(0),u64::MAX,1);
        let original_completion = ArtifactToolCompletion::<TestApp>::new();
        let original_consumer = original_completion.clone();
        let original_payload = test_retained_command_payload(original_completion).await;
        let original_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, original_payload, operation);
        let (original_admission, original_input) = test_retained_wire_input(&bus, &wire);
        let mut original = match bus.dispatch_wire_retained_with_spec(&original_admission, original_input, None, original_spec) {
            Ok(dispatch) => dispatch,
            Err(_) => panic!("test retained command initial dispatch"),
        };
        let mut checkpoint_bytes = None;
        for _ in 0..64 {
            match caller.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut original.job,cx))).unwrap() {
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Preview(_) => {},
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Checkpoint { state: bytes, .. } => {
                    if bytes.get(4) == Some(&3) && bytes.get(5) == Some(&1) && bytes.get(48) == Some(&1) {
                        checkpoint_bytes = Some(bytes);
                    }
                    if checkpoint_bytes.is_some() {
                        break;
                    }
                }
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Yield | semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Idle => {}
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Cancelled => panic!("initial retained command cancelled"),
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Complete { .. } => panic!("initial retained command completed before its work checkpoint"),
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Fault(_) => {
                    panic!("initial retained command faulted");
                }
            }
        }
        let checkpoint_bytes = checkpoint_bytes.expect("work checkpoint after one custom cursor step");
        original.job.begin_close();
        drive_job_close(&mut original.job,&mut caller);
        assert!(original.job.terminal_is_empty(), "interrupted original retires every retained owner incrementally");
        assert!(original_consumer.take_emit().expect("original completion cell").is_none(), "interrupted original never publishes a result");

        TEST_RETAINED_COMMAND_STEP_CALLS.store(0, std::sync::atomic::Ordering::SeqCst);
        let resumed_completion = ArtifactToolCompletion::<TestApp>::new();
        let resumed_consumer = resumed_completion.clone();
        let resumed_payload = test_retained_command_payload(resumed_completion).await;
        let resumed_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, resumed_payload, operation);
        let (resumed_admission, resumed_input) = test_retained_wire_input(&bus, &wire);
        let (_, resumed_checkpoint) = test_retained_wire_input(&bus, &checkpoint_bytes);
        let mut resumed = match bus.dispatch_wire_retained_with_spec(&resumed_admission, resumed_input, Some(resumed_checkpoint), resumed_spec) {
            Ok(dispatch) => dispatch,
            Err(_) => panic!("test retained command resume dispatch"),
        };
        let mut completed = false;
        for _ in 0..96 {
            match caller.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut resumed.job,cx))).unwrap() {
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Preview(_) => {},
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Checkpoint { .. } => {},
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Yield | semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Idle => {}
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Complete { .. } => {
                    completed = true;
                    break;
                }
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Cancelled => panic!("resumed retained command cancelled"),
                semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Fault(_) => {
                    panic!("resumed retained command faulted");
                }
            }
        }
        assert!(completed, "resumed retained command reaches its commit boundary");
        assert_eq!(TEST_RETAINED_COMMAND_STEP_CALLS.load(std::sync::atomic::Ordering::SeqCst), 2, "resume continues after cursor one instead of replaying completed custom work");
        let (result, _) = resumed_consumer.take_emit().expect("resumed completion cell").expect("resumed completion value");
        assert_eq!(result.expect("resumed command emit").artifact_mutations, vec![TestMutation::SetLabel(SetLabel { value: "resumed".into() })]);
        resumed.job.begin_close();
        drive_job_close(&mut resumed.job,&mut caller);
        assert!(resumed.job.terminal_is_empty(), "completed resume retires every retained owner incrementally");

        let cancelled_completion = ArtifactToolCompletion::<TestApp>::new();
        let cancelled_consumer = cancelled_completion.clone();
        let cancelled_payload = test_retained_command_payload(cancelled_completion).await;
        let cancelled_spec = ToolOperationSpec::new(TEST_RETAINED_COMMAND_CONTROLLER, TEST_RETAINED_COMMAND_TOOL, TEST_RETAINED_COMMAND_SCHEMA, cancelled_payload, operation);
        let (cancelled_admission, cancelled_input) = test_retained_wire_input(&bus, &wire);
        let mut cancelled = match bus.dispatch_wire_retained_with_spec(&cancelled_admission, cancelled_input, None, cancelled_spec) {
            Ok(dispatch) => dispatch,
            Err(_) => panic!("test retained command cancellation dispatch"),
        };
        caller.cancel().cancel_now();
        assert!(matches!(caller.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut cancelled.job,cx))).unwrap(),semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Cancelled));
        cancelled.job.begin_close();
        drive_job_close(&mut cancelled.job,&mut caller);
        assert!(cancelled.job.terminal_is_empty(), "cancelled retained command retires every owner incrementally");
        assert!(cancelled_consumer.take_emit().expect("cancelled completion cell").is_none(), "cancelled retained command never publishes a result");
        caller.retire(policy.close);
    }

    /// 👥️ Contract-freeze §C7.6: `adopt_presence` (1) adopts an app-typed `presence_pack` into
    /// `presence_store` ONLY when one is present, (2) unconditionally upserts `color`/`surface`/
    /// `interaction` into `peer_presence` for every peer in the roster, and (3) treats the roster
    /// as the single source of truth — a peer absent from a later call is dropped from BOTH maps.
    ///
    /// 👋 bob leaves the roster — a second call carrying only alice must drop bob from BOTH maps.
    #[semio_framework_async_macros::async_test]
    async fn retained_presence_fills_presence_store_and_peer_marks_and_drops_left_peers() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        let alice = sample_presence_peer("user:alice#s1", Some(3), true);
        let bob = sample_presence_peer("user:bob#s1", Some(5), false);

        assert!(publish_presence_roster(&mut app, 1, Some(9), &[alice.clone(), bob], 1000,cx).await.fault.is_none());
        assert_eq!(app.roster.own_color, Some(9));
        let typed_root = app.stores.presence_store.peers_root();
        assert_eq!(typed_root.peers().count(), 1, "only the peer carrying a presence_pack adopts into presence_store");
        assert_eq!(typed_root.peers().next().unwrap().0, "user:alice#s1");
        assert_eq!(app.roster.peer_presence.len(), 2, "both peers upsert into peer_presence regardless of presence_pack");
        assert_eq!(app.roster.peer_presence.get("user:alice#s1").unwrap().color, Some(3));
        assert_eq!(app.roster.peer_presence.get("user:bob#s1").unwrap().color, Some(5));
        drop(typed_root);

        assert!(publish_presence_roster(&mut app, 2, Some(9), &[alice], 2000,cx).await.fault.is_none());
        assert_eq!(app.stores.presence_store.peers_root().peers().count(), 1);
        assert_eq!(app.roster.peer_presence.len(), 1);
        assert!(app.roster.peer_presence.contains_key("user:alice#s1"));
        assert!(!app.roster.peer_presence.contains_key("user:bob#s1"), "an actor absent from the roster must be dropped, not left stale");
        drain_and_close_fixture(&mut app, mounted_policy,cx);
    
        
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    struct TestSnapshotRetirement {
        pub owner: Option<Box<dyn store::ErasedSnapshotRetirement>>,
        lie_about_terminal: bool,
    }

    impl store::ErasedSnapshotRetirement for TestSnapshotRetirement {
        fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> {
            use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
            if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())); }
            if self.lie_about_terminal {
                self.lie_about_terminal = false;
                return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
            }
            store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.owner, grant)
        }

        fn terminal_is_empty(&self) -> bool { self.owner.is_none() }
        fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.demands(0)?.copy_bytes) }
        fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> { Ok(self.demands(body)?.capacity_bytes) }
        fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.demands(0)?.release_bytes) }
        fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { Ok(self.demands(0)?.depth) }
    }

    impl TestSnapshotRetirement {
        pub fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> {
            if self.lie_about_terminal { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
            self.owner.as_ref().map_or(Ok(Default::default()), |owner| store::os_store::snapshot_read::artifact_retirement_box_demands(owner, body))
        }
    }

    /// 🎭️ Spends one invalid terminal witness while retaining the actual controlled payload and its physical child frame.
    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct TestLyingOwnedValueRetirementFactory {
        #[factory_owned]
        lie_about_terminal: std::sync::atomic::AtomicBool,
    }

    impl store::ArtifactOwnedValueRetirementFactory<TestSnapshot> for TestLyingOwnedValueRetirementFactory {
        fn retirement_birth_bytes(&self, _: &TestSnapshot) -> usize {
            std::mem::size_of::<TestSnapshotRetirement>() + semio_framework_value::retirement::owned_retirement_birth_bytes::<TestSnapshot>()
        }

        fn retire_owned(&self, value: TestSnapshot, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, semio_framework_value::retained_clone::RetainedCloneProgress), (semio_framework_value::ValueError, TestSnapshot)> {
            use semio_framework_value::retained_clone::RetainedCloneBirthDemand;
            let frame = std::mem::size_of::<TestSnapshotRetirement>();
            let demand = RetainedCloneBirthDemand { capacity_bytes: self.retirement_birth_bytes(&value), depth: 3 };
            if let Err(error) = demand.admit(grant) { return Err((error, value)); }
            let child = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_capacity_bytes: grant.maximum_capacity_bytes - frame, maximum_depth: grant.maximum_depth - 1, ..grant };
            let (owner, mut progress) = semio_framework_value::retirement::admit_owned_retirement(value, child)?;
            progress.retained_capacity_bytes += frame;
            Ok((Box::new(TestSnapshotRetirement { owner: Some(owner), lie_about_terminal: self.lie_about_terminal.swap(false, std::sync::atomic::Ordering::AcqRel) }), progress))
        }
    }

    /// 🎭️ Installs the member owner catalog whose OWNED-VALUE factory carries the lie —
    /// the one `retire_snapshot_read_erased` draws its unique-owner disposer from, which is the only
    /// place a child-content retirement can meet a lying terminal witness at all. The DISPLACEMENT
    /// factory stays honest on purpose: it retires the snapshot a member replaces, and a lying one
    /// would pin that alias for ever, so the captured root could never become the snapshot's last
    /// owner and `Arc::into_inner` would never reach the owned-value factory.
    #[doc(hidden)]
pub fn install_test_snapshot_retirement(app: &mut VcsArtifactApp<TestApp, TestMembers>, child_id: &str, lie_about_terminal: bool) {
        let TestMembers::Child(child) = &mut app.child_composition.children.get_mut(&("slot".to_string(), child_id.to_string())).expect("exact child retirement owner").member;
        type Catalog = store::os_store::document_store_owners::DocumentStoreOwners<TestSnapshot, TestMutation>;
        let capacity = Catalog::source_birth_bytes::<semio_framework_value::factory::SharedValueRetirementFactory<TestSnapshot>, TestLyingOwnedValueRetirementFactory, semio_framework_value::factory::OwnedValueRetirementFactory<TestMutation>, store::ArtifactStoreCursorDisposer<TestSnapshot, TestMutation>>().expect("exact catalog source birth");
        let admitted = Catalog::admit_source_constructor(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: capacity, maximum_depth: 1, ..Default::default() }, || (
            semio_framework_value::factory::SharedValueRetirementFactory::<TestSnapshot>::default(),
            TestLyingOwnedValueRetirementFactory { lie_about_terminal: std::sync::atomic::AtomicBool::new(lie_about_terminal) },
            semio_framework_value::factory::OwnedValueRetirementFactory::<TestMutation>::default(),
            store::ArtifactStoreCursorDisposer::<TestSnapshot, TestMutation>::new(),
        ));
        store::install_unscheduled_catalog(child, store::os_store::document_store_owners::complete_unscheduled_catalog(admitted)).expect("exact original lying-witness catalog");
    }

    #[semio_framework_async_macros::async_test]
    async fn member_factory_closed_dialect_register_rejects_pin_without_mutating_member() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        let dialect = test_child_dialect().await;
        let expected = ArtifactRef { artifact_id: "child-1".into(), dialect: dialect.clone() };
        app.child_composition.pending_child_pins.push(vcs::CompositionPin { child_ref: expected.clone(), checkpoint_id: "missing-checkpoint".into() });
        let member = TestMembers::create("child-1", &dialect, &TestSnapshot::default().encode_pack(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.unwrap();
        let before = member.envelope_pack_bytes().await.unwrap();
        declare_test_child(&mut app, "child-1").await;
        let error = app.register_child("slot", "child-1", dialect, member, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect_err("direct transfer must not apply a deferred restore pin");
        let mut returned = error.member.expect("exact caller member returned");
        assert_eq!(returned.envelope_pack_bytes().await.unwrap(), before);
        assert_eq!(returned.artifact_ref().as_ref(), Some(&expected));
        assert!(returned.owner_ref().is_none());
        assert!(app.child_composition.test_child_admission_state(1).children_empty && app.child_composition.test_child_admission_state(1).content_empty && app.child_composition.test_child_admission_state(1).abort_empty);
        assert_eq!(app.child_composition.test_child_admission_state(1).generation, 0);
        assert_eq!(app.child_composition.pending_child_pins.len(), 1);
        assert!(app.child_composition.composition.graph_mut().await.owner_of("child-1").await.is_none());
        close_member_admission_fixture(&mut returned);
        close_member_admission_app(&mut app,cx);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧾️ One `KernelMutation` for the parent's own op, one for the child's — each carrying
    /// its OWN document handle, never the parent's, for the child entry (Task 3's "REAL
    /// target" requirement).
    ///
    /// 🧾️ ONE `UndoGroup` names BOTH documents via `member_edits`.
    ///
    /// The child store actually applied its own op.
    ///
    /// And the command log recorded the child's edit id in `child_edit_ids`.
    ///
    /// 🧾️ The migrated route logs the admission row (no edit of its own) before the composed
    /// publication logs the real one, so the subject is the row that CARRIES the gesture: exactly
    /// one composite row names the child's edit, and it is the row that carries the parent's.
    #[semio_framework_async_macros::async_test]
    async fn composite_gesture_produces_one_undo_group_spanning_parent_and_child_with_real_handles() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-1").await;

        let result = app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta(), &mut identity,cx).await.expect("composite edit");

        assert_eq!(
            result.mutations.len(),
            2,
            "a composite gesture carries the parent's op and the child's; got documents {:?}, {} member edit(s), settle receipt {:?}",
            result.mutations.iter().map(|mutation| mutation.document).collect::<Vec<_>>(),
            result.inverse_group.member_edits.len(),
            result.output
        );
        let parent_handle = ArtifactHandle(meta().instance_id as u128);
        let child_handle = artifact_handle_of("child-1").await;
        assert_ne!(parent_handle, child_handle);
        let mutation_documents: std::collections::HashSet<ArtifactHandle> = result.mutations.iter().map(|mutation| mutation.document).collect();
        assert!(mutation_documents.contains(&parent_handle), "the parent's own edit must carry the parent's handle");
        assert!(mutation_documents.contains(&child_handle), "the child's edit must carry the CHILD's handle, not the parent's");

        assert_eq!(result.inverse_group.member_edits.len(), 2);
        let member_documents: std::collections::HashSet<ArtifactHandle> = result.inverse_group.member_edits.iter().map(|edit_ref| edit_ref.document).collect();
        assert!(member_documents.contains(&parent_handle));
        assert!(member_documents.contains(&child_handle));

        let entry = app.child_composition.children.get_mut(&("slot".to_string(), "child-1".to_string())).expect("child stays registered after dispatch");
        assert_eq!(entry.reference.dialect.artifact_kind, "s.test.child");
        let TestMembers::Child(child_store) = &mut entry.member;
        assert_eq!(child_store.snapshot().expect("child snapshot").count, 7);

        let history = app.render_ctx().test_history().await;
        let composite_rows: Vec<_> = history.commands.iter().filter(|entry| entry.action_id == "compositeEdit").collect();
        let carrying: Vec<_> = composite_rows.iter().filter(|row| !row.child_edit_ids.is_empty()).collect();
        assert_eq!(carrying.len(), 1, "exactly one composite row names the child's own edit id: {:?}", composite_rows.iter().map(|row| (row.edit_id.clone(), row.child_edit_ids.clone())).collect::<Vec<_>>());
        assert_eq!(carrying[0].child_edit_ids.len(), 1, "the composite row names the child's own edit id exactly once: {:?}", carrying[0].child_edit_ids);
        assert!(carrying[0].edit_id.is_some(), "the row that names the child's edit also carries the parent's own edit: {:?}", carrying[0].edit_id);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn an_agent_transaction_carries_owned_child_op_groups_and_commits_undoes_and_redoes_them_as_one_group() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-1").await;
        let children = vec![test_child_emit("slot", "child-1", &[TestMutation::SetCount(SetCount { value: 5 })])];
        let wire = ChildEmit::encode_groups(&children);
        assert!(!wire.is_empty(), "a gesture that touches a child carries its group");
        assert_eq!(ChildEmit::decode_groups(&wire).expect("the wire pack decodes"), children, "the group survives the wire byte for byte");
        assert!(ChildEmit::encode_groups(&[]).is_empty(), "a childless gesture leaves the wire unchanged");
        assert_eq!(ChildEmit::decode_groups(&[7, 7, 7]).expect_err("rubbish is refused").code.0, "transaction.child-groups-malformed");

        let parent_op = <TestMutation as ::protocol::OpBinary>::encode_op(&TestMutation::SetLabel(SetLabel { value: "agent".into() })).expect("encode parent op");
        let outcome = app.transaction_prepare("txn-agent-1", "", &[], &[parent_op], &wire, Some(protocol::MutationOrigin::Owner)).await;
        assert!(outcome.rejection.is_none(), "prepare admits a held child: {:?}", outcome.rejection.as_ref().map(|fault| &fault.message));
        let edit_id = app.transaction_commit("txn-agent-1", &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("commit the composite transaction");
        assert!(!edit_id.is_empty());
        macro_rules! child_count {
            ($app:expr) => {{
                let TestMembers::Child(child_store) = &mut $app.child_composition.children.get_mut(&("slot".to_string(), "child-1".to_string())).expect("child stays registered").member;
                child_store.snapshot().expect("child snapshot").count
            }};
        }
        assert_eq!(child_count!(app), 5, "the child applied the agent's op");
        assert_eq!(app.stores.test_snapshot().await.label, "agent", "the parent applied the agent's op");
        assert_eq!(app.stores.store.tail_group_id().await.as_deref(), Some("txn-agent-1"), "the group identity is the transaction id");

        app.transaction_undo("txn-agent-1", &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("undo the whole group");
        assert_eq!(child_count!(app), 0, "undo reverts the child");
        assert_ne!(app.stores.test_snapshot().await.label, "agent", "undo reverts the parent");
        app.transaction_redo("txn-agent-1", &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("redo the whole group");
        assert_eq!(child_count!(app), 5, "redo reapplies the child");
        assert_eq!(app.stores.test_snapshot().await.label, "agent", "redo reapplies the parent");
        assert!(app.transaction_undo("txn-someone-else", &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.is_err(), "a group no member carries is refused by name");

        let only_child = ChildEmit::encode_groups(&[test_child_emit("slot", "child-1", &[TestMutation::SetCount(SetCount { value: 8 })])]);
        let outcome = app.transaction_prepare("txn-agent-2", "", &[], &[], &only_child, Some(protocol::MutationOrigin::Owner)).await;
        assert!(outcome.rejection.is_none(), "a children-only transaction is a pre-planned transaction");
        app.transaction_commit("txn-agent-2", &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("commit the children-only transaction");
        assert_eq!(child_count!(app), 8);
        app.transaction_undo("txn-agent-2", &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("undo the children-only group");
        assert_eq!(child_count!(app), 5);

        let stray = ChildEmit::encode_groups(&[test_child_emit("slot", "ghost", &[TestMutation::SetCount(SetCount { value: 1 })])]);
        let refused = app.transaction_prepare("txn-agent-3", "", &[], &[], &stray, Some(protocol::MutationOrigin::Owner)).await;
        assert_eq!(refused.rejection.expect("a child this instance does not hold is refused").code.0, "transaction.member-rejected");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪️ Runs the reserved spawn-job the host would for `admitted` and answers the bytes its
    /// `JobCompleted` carries.
    async fn finish_reserved_spawn_job(admitted: &semio_framework::InvocationResult) -> (u64, Result<Vec<u8>, Fault>) {
        let (job, input) = admitted
            .requested_effects
            .iter()
            .find_map(|effect| match effect {
                Effect::SpawnJob { job, kind, input, .. } if kind == semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND => Some((*job, input.clone())),
                _ => None,
            })
            .expect("a reserved route admits its spawn-job");
        (job, semio_framework_plugin_reactor_jobs::reserved::drive_framework_reserved_registry_job(job, input, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().preparation))
    }

    /// 🧪️ Polls one publication under the same once-paid caller throughout its transaction.
    fn continuation_unit_is_ready_on_first_poll(app:&mut VcsArtifactApp<TestApp,TestMembers>,caller:&mut semio_framework_job::OriginalJobFixtureCaller,grant:semio_framework_value::retained_clone::RetainedCloneGrant){
        let mut identity = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity();
        original_publication_turn(app,&mut identity,caller,grant).expect("original continuation unit");
    }

    /// ⚖️ A composed `commitCheckpoint` is a job driven across turns: the host's `JobCompleted` only
    /// queues the commit, every continuation unit completes on its first poll, the commit takes one
    /// unit per composed child plus the parent's own, reports that progress while it runs, and
    /// publishes its result — pins included — only after the last unit.
    #[semio_framework_async_macros::async_test]
    async fn a_composed_checkpoint_commit_is_driven_one_ready_unit_per_turn_with_progress() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let mut publication_caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(7),mounted_policy.preparation,||Some(0),u64::MAX,4096);
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        for child in ["child-1", "child-2"] {
            register_test_child(&mut app, child).await;
            app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: child.into(), child_value: 7 }, &meta(), &mut identity,cx).await.expect("composite edit");
        }
        let admitted = PluginApp::handle_action(&mut *app, "commitCheckpoint", Some(dv(serde_json::json!({ "message": "v1" }))), &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("admit checkpoint");
        let (job, output) = finish_reserved_spawn_job(&admitted).await;
        assert!(PluginApp::admit_reserved_spawned_job(&mut *app, job, output).expect("JobCompleted queues the commit"));
        assert!(PluginApp::has_runnable_typed_operations(&*app), "a queued commit keeps the actor's turn armed");
        let mut progress = Vec::new();
        let mut units = 0;
        let outcome = loop {
            continuation_unit_is_ready_on_first_poll(&mut app,&mut publication_caller,mounted_policy.maintenance);
            units += 1;
            if let Some(outcome) = PluginApp::take_reserved_commit_outcome(&mut *app) {
                break outcome;
            }
            progress.push(PluginApp::reserved_commit_progress(&*app).expect("a running commit reports progress"));
            assert!(units < 64, "the checkpoint commit never finished");
        };
        let result = outcome.expect("checkpoint commit");
        assert_eq!(units, 3, "two composed children plus the parent commit are three units");
        assert_eq!(progress.iter().map(|step| (step.applied, step.total)).collect::<Vec<_>>(), vec![(1, 3), (2, 3)]);
        assert!(progress.iter().all(|step| step.operation == job));
        assert!(result.events.iter().any(|event| event.kind == "history-changed"));
        assert!(PluginApp::reserved_commit_progress(&*app).is_none());
        let checkpoint = app.stores.test_store().await.current_checkpoint_id().map(str::to_string).expect("parent checkpoint exists");
        let pins = app.stores.test_store().await.envelope().vcs.checkpoints.iter().find(|entry| entry.id == checkpoint).map(|entry| entry.composition_pins.clone()).expect("checkpoint found");
        assert_eq!(pins.iter().map(|pin| pin.child_ref.artifact_id.as_str()).collect::<Vec<_>>(), ["child-1", "child-2"]);
        publication_caller.retire(mounted_policy.close);

    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// ⚖️ Cancelling the operation between units stops a composed checkpoint before the parent
    /// commits: the outcome is `interactive-job.cancelled`, no parent checkpoint exists, and the app
    /// still closes to terminal emptiness.
    #[semio_framework_async_macros::async_test]
    async fn a_composed_checkpoint_commit_cancels_between_units() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let mut publication_caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(7),mounted_policy.preparation,||Some(0),u64::MAX,4096);
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        for child in ["child-1", "child-2"] {
            register_test_child(&mut app, child).await;
            app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: child.into(), child_value: 7 }, &meta(), &mut identity,cx).await.expect("composite edit");
        }
        let admitted = PluginApp::handle_action(&mut *app, "commitCheckpoint", Some(dv(serde_json::json!({ "message": "v1" }))), &meta(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("admit checkpoint");
        let (job, output) = finish_reserved_spawn_job(&admitted).await;
        assert!(PluginApp::admit_reserved_spawned_job(&mut *app, job, output).expect("JobCompleted queues the commit"));
        continuation_unit_is_ready_on_first_poll(&mut app,&mut publication_caller,mounted_policy.maintenance);
        assert_eq!(PluginApp::reserved_commit_progress(&*app).map(|step| step.applied), Some(1));
        assert!(app.tool_cancellation_handle().cancel_document(semio_framework_plugin_operation::ArtifactDocumentAuthority(meta().instance_id)).expect("cancel the document's operations"));
        continuation_unit_is_ready_on_first_poll(&mut app,&mut publication_caller,mounted_policy.maintenance);
        let fault = PluginApp::take_reserved_commit_outcome(&mut *app).expect("a cancelled commit answers").expect_err("cancelled");
        assert_eq!(fault.code.0, "interactive-job.cancelled");
        assert!(app.stores.test_store().await.current_checkpoint_id().is_none(), "a cancelled checkpoint never commits the parent");
        assert!(!PluginApp::has_runnable_typed_operations(&*app));
        publication_caller.retire(mounted_policy.close);

    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 📌️ Checkpoint the parent: the cascade must commit the dirty child first, then pin the
    /// child checkpoint that commit produced.
    ///
    /// ⏭️ Move both forward, past the pin.
    ///
    /// ⏮️ Checking the parent out to the pinned checkpoint must drag the child back with it —
    /// otherwise a restored composition silently mixes an old parent with a new child.
    #[semio_framework_async_macros::async_test]
    async fn a_checkpoint_pins_its_children_and_a_checkout_cascades_back_to_them() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-1").await;
        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 7 }, &meta(), &mut identity,cx).await.expect("first composite edit");

        app.dispatch_action("commitCheckpoint", Some(&dv(serde_json::json!({ "message": "v1" }))), &meta(), &mut identity,cx).await.expect("checkpoint");
        let pinned_checkpoint = app.stores.test_store().await.current_checkpoint_id().map(str::to_string).expect("parent checkpoint exists");
        let pins = app.stores.test_store().await.envelope().vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == pinned_checkpoint).map(|checkpoint| checkpoint.composition_pins.clone()).expect("checkpoint found");
        assert_eq!(pins.len(), 1, "a composing document's checkpoint must pin its children");
        assert_eq!(pins[0].child_ref.artifact_id, "child-1");

        app.dispatch_typed(TestCommand::CompositeEdit { slot: "slot".into(), child_id: "child-1".into(), child_value: 42 }, &meta(), &mut identity,cx).await.expect("second composite edit");
        let live = reads_child_count(&app);
        assert_eq!(live.await, 42);

        app.dispatch_action("checkoutCheckpoint", Some(&dv(serde_json::json!({ "checkpointId": pinned_checkpoint }))), &meta(), &mut identity,cx).await.expect("checkout");
        assert_eq!(reads_child_count(&app).await, 7, "checkout did not cascade to the pinned child");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧾️ The command capture this law holds IS the alias that blocks the retiring root's close —
    /// a real captured root is released when its operation retires, and every assertion above has
    /// already run against it.
    #[semio_framework_async_macros::async_test]
    async fn child_content_publication_path_copies_fixed_pages_and_command_capture_retains_one_root() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-a").await;
        let admitted = ChildContentView::clone(&app.child_composition.child_content_root);
        let admitted_root = admitted.root.as_ref().expect("published root").clone();
        assert!(std::sync::Arc::ptr_eq(&admitted_root, app.child_composition.child_content_root.root.as_ref().expect("live root")), "command capture retains exactly one immutable root Arc");

        register_test_child(&mut app, "child-b").await;
        let current_root = app.child_composition.child_content_root.root.as_ref().expect("advanced root");
        assert!(!std::sync::Arc::ptr_eq(&admitted_root, current_root), "a child lifecycle event publishes a new root");
        assert!(admitted.typed_read::<TestSnapshot>("slot", "child-a").is_ok(), "the admitted root remains exact after later publication");
        assert!(admitted.typed_read::<TestSnapshot>("slot", "child-b").is_err(), "the admitted root never observes a later child");
        assert!(ChildContentView::clone(&app.child_composition.child_content_root).typed_read::<TestSnapshot>("slot", "child-b").is_ok());
        assert!(!app.child_composition.child_content_retirements.is_empty(), "the replaced nonempty root remains under explicit retirement authority");
        drop(admitted);
        drop(admitted_root);
        drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🎡️ The property is RECLAMATION, not the step word. `maintenance_step` rotates: one call
    /// walks every stage that still has something to give, so a later idle stage legitimately answers
    /// `Complete` and that is what the call returns — the previous form of this law read that as the
    /// LYING owner reaching a terminal step and failed on a stage it never visited. What must never
    /// happen is the lying owner's authority leaving the registry: every turn until the fault arrives
    /// is asserted to leave `child_content_retirements` populated, which is strictly what the old
    /// clause was a proxy for, and the fault must still arrive by name before any reclaim.
    ///
    /// 🪢️ The registered root captured the member's CURRENT snapshot, so while the member still
    /// holds that exact `Arc` the returned-read retirement is only an alias: `Arc::into_inner`
    /// answers `None`, the alias is released truthfully and the member's own owned-value factory
    /// — where the lie lives — is never consulted. Advancing the member past that snapshot and
    /// draining the alias its displacement retired makes the retiring root the snapshot's LAST
    /// owner, which is the only state in which a child-content retirement meets a lying disposer
    /// at all.
    ///
    /// 🪪️ Either named guard is an acceptable answer — the plugin ladder's own terminal-witness
    /// refusal, or the nested disposer refusal it now attributes by name instead of as the
    /// anonymous `plugin.internal` — but an unnamed internal error is not.
    ///
    /// 🧹️ The lie is spent (see `TestSnapshotRetirement::close_step`), so the very disposer that
    /// was caught now answers honestly and the fixture closes through the ordinary ladder — the
    /// law proves the refusal, not a leak.
    #[semio_framework_async_macros::async_test]
    async fn child_root_maintenance_requires_terminal_empty_before_reclaim() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app_raw(mounted_policy, &mut identity).await;
        declare_test_child(&mut app, "child-a").await;
        app.register_child("slot", "child-a", test_child_dialect().await, new_bare_test_child("child-a").await.expect("construct child-a"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("register child-a");
        install_test_snapshot_retirement(&mut app, "child-a", true);
        {
            let TestMembers::Child(child) = &mut app.child_composition.children.get_mut(&("slot".to_string(), "child-a".to_string())).expect("exact child owner").member;
            for value in 1..=2 {
                apply_test_mutations(child, vec![TestMutation::SetCount(SetCount { value })]).await.expect("member advances past the captured snapshot");
            }
            for _ in 0..1_024 {
                if child.maintenance_retirements_terminal_is_empty() {
                    break;
                }
                let demand = child.maintenance_retirements_demands(4096).expect("quoted displaced snapshot demand");
                let _ = child.maintenance_retirements_step(fixture_close_grant(demand)).expect("member drains its displaced snapshot aliases");
            }
            assert!(child.maintenance_retirements_terminal_is_empty(), "the member released every snapshot alias it displaced");
        }
        let generation = app.child_composition.admit_child_content_publication().expect("admit replacement root");
        app.child_composition.publish_child_content_member(generation, "slot", "child-a").await.expect("replace the exact child snapshot lease");
        app.maintenance.maintenance_stage = 4;
        assert!(matches!(app_maintenance_turn(&mut app,cx).expect("transfer retirement authority"), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(_)));
        let mut faulted = None;
        for turn in 0..64 * usize::from(MAINTENANCE_STAGES) {
            app.maintenance.maintenance_stage = 4;
            match app_maintenance_turn(&mut app,cx) {
                Ok(step) => assert!(
                    !app.child_composition.child_content_retirements.is_empty(),
                    "the lying owner's authority must never be reclaimed without its terminal witness: turn {turn} answered {step:?} from stage {} and left the registry empty",
                    (app.maintenance.maintenance_stage + MAINTENANCE_STAGES - 1) % MAINTENANCE_STAGES
                ),
                Err(fault) => {
                    faulted = Some(fault);
                    break;
                }
            }
        }
        let fault = faulted.expect("lying Complete must fail before registry removal");
        assert!(
            matches!(fault.code.0.as_str(), "interactive-job.child-snapshot-terminal-not-empty" | "interactive-job.child-snapshot-disposer-refused"),
            "the lie must be caught BY NAME: {fault:?}"
        );
        assert!(!app.child_composition.child_content_retirements.is_empty(), "terminal witness failure retains the registry authority");
        drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn child_root_maintenance_reclaims_completed_owner_for_later_publication() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app_raw(mounted_policy, &mut identity).await;
        declare_test_child(&mut app, "child-a").await;
        app.register_child("slot", "child-a", test_child_dialect().await, new_bare_test_child("child-a").await.expect("construct child-a"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("register child-a");
        install_test_snapshot_retirement(&mut app, "child-a", false);
        let generation = app.child_composition.admit_child_content_publication().expect("admit replacement root");
        app.child_composition.publish_child_content_member(generation, "slot", "child-a").await.expect("replace the exact child snapshot lease");
        for _ in 0..64 {
            if app.child_composition.child_content_retirements.is_empty() {
                break;
            }
            app.maintenance.maintenance_stage = 4;
            let _ = app_maintenance_turn(&mut app,cx).expect("bounded retirement progress");
        }
        assert!(app.child_composition.child_content_retirements.is_empty(), "terminal-empty retirement is reclaimed while the app remains live");
        let generation = app.child_composition.admit_child_content_publication().expect("later publication can reuse the fixed retirement registry");
        app.child_composition.publish_child_content_member(generation, "slot", "child-a").await.expect("later publication after bounded reclaim");
        drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// ⌨️ LAW (coordinator P1, ticket 26/09/23 C12): child-content publications at the maximum rate — 10 000 with no
    /// maintenance turn between them — never fault. Every admission waits on the retirement ahead of it (bounded synchronous
    /// reclaim) instead of answering `interactive-job.child-root-retirement-saturated`, every publication is admitted in
    /// order (exactly the next generation), the registry never holds more than its fixed slots, and the saturated registry
    /// reports maintenance pressure until maintenance returned it below its quarter-occupancy bound.
    #[semio_framework_async_macros::async_test]
    async fn child_publications_at_the_maximum_rate_wait_on_retirement_instead_of_faulting() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app_raw(mounted_policy, &mut identity).await;
        declare_test_child(&mut app, "child-a").await;
        app.register_child("slot", "child-a", test_child_dialect().await, new_bare_test_child("child-a").await.expect("construct child-a"), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("register child-a");
        install_test_snapshot_retirement(&mut app, "child-a", false);
        let start = app.child_composition.child_content_generation;
        for index in 1..=10_000u64 {
            let generation = app.child_composition.admit_child_content_publication().unwrap_or_else(|fault| panic!("publication {index} faulted instead of waiting: {fault:?}"));
            assert_eq!(generation, start + index, "publication {index} is admitted in order");
            app.child_composition.publish_child_content_member(generation, "slot", "child-a").await.unwrap_or_else(|fault| panic!("publication {index} did not land: {fault:?}"));
            assert!(app.child_composition.child_content_retirements.len() <= ARTIFACT_LIVE_OUTPUT_SLOTS, "the child-root retirement registry stays within its fixed slots");
        }
        assert!(PluginApp::maintenance_under_pressure(&app), "a saturated child-root retirement registry reports maintenance pressure");
        semio_framework_plugin_testing_laws::artifact_app_laws::drain_maintenance_pressure(&mut app,cx);
        assert!(app.child_composition.child_content_retirements.len() < MAINTENANCE_CHILD_RETIREMENT_PRESSURE_OCCUPANCY, "pressure maintenance returns the child roots below their quarter-occupancy bound");
        drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// ⏱️ This clause used to be an 8 ms wall clock. The clock was a PROXY for one property — the
    /// maximum child's bytes never cross the public dispatch boundary, the dispatch hands back a
    /// continuation instead — and on a loaded machine the proxy fails while the property holds. The
    /// property itself is asserted directly and deterministically: the dispatch requests its
    /// continuation, its own answer carries no child bytes, and the two materialization counters
    /// below stay at zero. Nothing the clock could catch is lost: a dispatch that walked the
    /// 4 MiB snapshot would have to clone it or pack-encode it, and both are counted.
    #[semio_framework_async_macros::async_test]
    async fn maximum_child_public_dispatch_reaches_first_continuation_without_clone_or_encode() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        register_test_child(&mut app, "child-maximum").await;
        let TestMembers::Child(child) = &mut app.child_composition.children.get_mut(&("slot".to_string(), "child-maximum".to_string())).expect("maximum child").member;
        apply_test_mutations(child, vec![TestMutation::SetLabel(SetLabel { value: "x".repeat(MAXIMUM_CHILD_PROBE_BYTES) })]).await.expect("seed maximum child");
        let publication_generation = app.child_composition.admit_child_content_publication().expect("admit maximum child publication");
        app.child_composition.publish_child_content_member(publication_generation, "slot", "child-maximum").await.expect("publish maximum child root");
        MAXIMUM_CHILD_CLONES.store(0, std::sync::atomic::Ordering::Release);
        MAXIMUM_CHILD_ENCODINGS.store(0, std::sync::atomic::Ordering::Release);

        let result = app.dispatch_action("probeChild", Some(&dv(serde_json::json!({ "slot": "slot", "childId": "child-maximum" }))), &meta(), &mut identity,cx).await.expect("public maximum-child probe");
        assert!(result.requested_effects.iter().any(|effect| matches!(effect, Effect::DispatchAction { action, .. } if action == "probeChildContinuation")));
        let answer = format!("{:?}", result.output);
        assert!(answer.len() < 4096, "the public dispatch's own answer carried {} bytes of the maximum child instead of a continuation", answer.len());
        assert!(result.mutations.is_empty(), "the public dispatch published mutations instead of deferring to its continuation");
        assert_eq!(MAXIMUM_CHILD_CLONES.load(std::sync::atomic::Ordering::Acquire), 0, "ChildContentView must clone only the existing Arc, never the maximum snapshot graph");
        assert_eq!(MAXIMUM_CHILD_ENCODINGS.load(std::sync::atomic::Ordering::Acquire), 0, "ChildContentView must not pack-encode maximum child content during public dispatch");
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧒️ The parent declares the created child first: absorb admits a member only under the
    /// identity the published parent declares, like every other admission.
    ///
    /// 🌱️ Proves `VcsArtifactApp::absorb_created_children` — the mechanism a
    /// `ChildGenesis`-authoring `Emit` constructor (a later wave) will rely on to make a
    /// freshly-minted child reachable at all; per B2's own `GroupReceipt::created_children`
    /// doc comment, skipping this step would make `ChildGenesis` pointless.
    #[semio_framework_async_macros::async_test]
    async fn created_children_survive_absorb_into_the_child_store_map() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_composed_app(mounted_policy, &mut identity).await;
        declare_test_child(&mut app, "genesis-child").await;
        let parent_id = app.stores.store.envelope().id.clone();
        app.child_composition.composition.graph_mut().await.insert_owns(&parent_id, "slot", "genesis-child").await.expect("seed ownership so absorb's slot_of lookup resolves");
        let target = ArtifactRef { artifact_id: "genesis-child".into(), dialect: test_child_dialect().await };
        let created: Vec<(ArtifactRef, TestMembers)> = vec![(target, new_test_child("genesis-child").await.expect("construct genesis child"))];

        app.child_composition.absorb_created_children(&mut app.stores, created).await.expect("absorb child and publish immutable root");

        let entry = app.child_composition.children.get_mut(&("slot".to_string(), "genesis-child".to_string())).expect("genesis child absorbed into the live map under the slot its parent declares it in");
        assert_eq!(entry.reference.dialect.artifact_kind, "s.test.child");
        assert_eq!(entry.member.document_id().await, "genesis-child");
        drain_and_close_composed_fixture(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 Browser #37 chrome: [Set Active Example, Resize Window] then spawn-admit undo.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_pops_chrome_top_shell_then_publishes_history_patch() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = contract_app(mounted_policy, &mut identity).await;
        reserved_action(
            &mut app,
            NOTE_SHELL_COMMAND_ACTION_ID,
            Some(&dv(json!({
                "commandId": "setActiveExample",
                "label": "Set Active Example",
                "inverseCommandId": "setActiveExample",
                "inverseArgs": { "exampleId": "forest" }
            }))),
         &mut identity)
        .await;
        reserved_action(
            &mut app,
            NOTE_SHELL_COMMAND_ACTION_ID,
            Some(&dv(json!({
                "commandId": "os.resizeWindow",
                "label": "Resize Window",
                "inverseCommandId": "os.resizeWindow",
                "inverseArgs": { "width": 800, "height": 600 }
            }))),
         &mut identity)
        .await;
        let before = app.render_ctx().test_history().await;
        assert_eq!(before.commands.iter().map(|entry| label_in(&entry.label, Locale::En)).collect::<Vec<_>>(), ["Resize Window", "Set Active Example"]);
        assert!(before.can_undo);
        assert!(!before.can_redo);

        let admitted = app.handle_action("undo", None, &meta(), &mut identity).await.expect("admit chrome undo");
        assert!(
            admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND)),
            "first turn must admit Isolated framework.reserved.tool"
        );
        assert!(admitted.history_patch.is_none(), "first turn answers with an empty patch");
        assert!(admitted.requested_effects.iter().all(|effect| !matches!(effect, Effect::ReplayShellCommand { .. })), "first turn must not apply the inverse");

        let settled = settle_reserved(&mut app, admitted).await;
        assert_eq!(
            settled.requested_effects,
            vec![Effect::ReplayShellCommand { action_id: "os.resizeWindow".into(), args: Some(dv(json!({ "width": 800, "height": 600 }))) }]
        );
        let patch = settled.history_patch.expect("chrome undo publishes history_patch");
        assert!(patch.cursor > 0, "cursor must advance past the populated snapshot");
        assert!(!patch.upserts.is_empty(), "patch must carry upserts the host chrome applies");
        assert!(patch.can_undo, "Set Active Example must remain reachable");
        assert!(patch.can_redo);
        let after_resize = app.render_ctx().test_history().await;
        let resize = after_resize.commands.iter().find(|entry| entry.action_id == "os.resizeWindow").expect("resize stays in the append-only log");
        assert!(!resize.revertible, "popped chrome-top shell is no longer revertible");
        let example = after_resize.commands.iter().find(|entry| entry.action_id == "setActiveExample").expect("example");
        assert!(example.revertible);

        let second = reserved_action(&mut app, "undo", None, &mut identity).await;
        assert_eq!(
            second.requested_effects,
            vec![Effect::ReplayShellCommand { action_id: "setActiveExample".into(), args: Some(dv(json!({ "exampleId": "forest" }))) }]
        );
        let patch2 = second.history_patch.expect("second undo publishes history_patch");
        assert!(!patch2.can_undo);
        assert!(patch2.can_redo);

        let redo_example = reserved_action(&mut app, "redo", None, &mut identity).await;
        assert_eq!(redo_example.requested_effects, vec![Effect::ReplayShellCommand { action_id: "setActiveExample".into(), args: None }]);
        let redo_patch = redo_example.history_patch.expect("redo publishes history_patch");
        assert!(redo_patch.can_undo);
        assert!(redo_patch.can_redo);
        assert!(app.render_ctx().test_history().await.commands.iter().find(|entry| entry.action_id == "setActiveExample").is_some_and(|entry| entry.revertible));

        let redo_resize = reserved_action(&mut app, "redo", None, &mut identity).await;
        assert_eq!(redo_resize.requested_effects, vec![Effect::ReplayShellCommand { action_id: "os.resizeWindow".into(), args: None }]);
        let redo_resize_patch = redo_resize.history_patch.expect("second redo publishes history_patch");
        assert!(redo_resize_patch.can_undo);
        assert!(!redo_resize_patch.can_redo);
        assert!(app.render_ctx().test_history().await.commands.iter().find(|entry| entry.action_id == "os.resizeWindow").is_some_and(|entry| entry.revertible));
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }


    /// 🧪 Browser-shaped `noteShellCommand`: a note with NO `inverseCommandId` is logged but is not an
    /// undo target, so chrome-order undo steps over `shell.windowActivate` and `shell.windowResize`
    /// and pops the one row that declared a real inverse. A self-inverse (replaying a chrome id with
    /// its own `detail`) is an identity, not an inverse — and replaying a `shell.*` id into the guest
    /// is what the window-kind gate refused `undeclared-action` on every undo (ticket 26/09/18 §3.2).
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_browser_note_without_inverse_is_not_an_undo_target() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/reserved-undo-browser-note.json")).expect("browser-note fixture");
        let mut app = contract_app(mounted_policy, &mut identity).await;
        for entry in fixture["stack"].as_array().expect("stack") {
            reserved_action(&mut app, NOTE_SHELL_COMMAND_ACTION_ID, Some(&dv(entry.clone())), &mut identity).await;
        }
        let before = app.render_ctx().test_history().await;
        assert_eq!(before.commands.iter().map(|entry| label_in(&entry.label, Locale::En)).collect::<Vec<_>>(), ["Activate Window", "Resize Window", "Set Active Example"]);
        for skipped in fixture["undo"]["skippedActionIds"].as_array().expect("skippedActionIds") {
            let action_id = skipped.as_str().expect("skipped action id");
            let entry = before.commands.iter().find(|entry| entry.action_id == action_id).expect("undeclared-inverse chrome is still logged");
            assert!(!entry.revertible, "{action_id} declared no inverse, so it must not be revertible");
        }
        let admitted = app.handle_action("undo", None, &meta(), &mut identity).await.expect("admit browser-note undo");
        assert!(
            admitted.requested_effects.iter().any(|effect| matches!(effect, Effect::SpawnJob { kind, placement: semio_framework::kernel::JobPlacement::Isolated, .. } if kind == semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND)),
            "first turn must admit Isolated framework.reserved.tool"
        );
        assert!(admitted.requested_effects.iter().all(|effect| !matches!(effect, Effect::ReplayShellCommand { .. })), "first turn must not apply the inverse");
        let settled = settle_reserved(&mut app, admitted).await;
        let replay_id = fixture["undo"]["replayActionId"].as_str().expect("replayActionId");
        let replay_args = dv(fixture["undo"]["replayArgs"].clone());
        assert_eq!(
            settled.requested_effects,
            vec![Effect::ReplayShellCommand { action_id: replay_id.into(), args: Some(replay_args) }],
            "undo must step over every chrome row that declared no inverse"
        );
        let patch = settled.history_patch.expect("chrome undo publishes history_patch");
        assert_eq!(patch.can_undo, fixture["undo"]["canUndo"].as_bool().expect("canUndo"));
        assert_eq!(patch.can_redo, fixture["undo"]["canRedo"].as_bool().expect("canRedo"));
        assert!(!patch.upserts.is_empty());
        let after = app.render_ctx().test_history().await;
        assert!(after.commands.iter().find(|entry| entry.action_id == replay_id).is_some_and(|entry| !entry.revertible), "popped chrome-top shell is no longer revertible");
        let redo = reserved_action(&mut app, "redo", None, &mut identity).await;
        assert_eq!(redo.requested_effects, vec![Effect::ReplayShellCommand { action_id: fixture["redo"]["replayActionId"].as_str().expect("redo replayActionId").into(), args: None }]);
        assert!(redo.history_patch.is_some_and(|patch| patch.can_undo && !patch.can_redo));
        assert!(app.render_ctx().test_history().await.commands.iter().find(|entry| entry.action_id == replay_id).is_some_and(|entry| entry.revertible));
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧪 `ReplayShellCommand` survives the leftover `pack_rt` encode/`decode_wire_effect` path.
    #[semio_framework_async_macros::async_test]
    async fn reserved_undo_replay_shell_command_survives_wire_roundtrip() {
        let fixture: Value = serde_json::from_str(include_str!("../🧫️fixtures/reserved-undo-browser-note.json")).expect("browser-note fixture");
        let wire = &fixture["wire"];
        let effect = Effect::ReplayShellCommand { action_id: wire["actionId"].as_str().expect("actionId").into(), args: Some(dv(wire["args"].clone())) };
        let packed = store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&effect));
        let value = store::pack_rt::decode_wire_value(&packed).expect("leftover pack decode");
        let decoded = match <Effect as semio_framework_value::FromValue>::from_value(value.clone()) {
            Ok(decoded) => decoded,
            Err(_) => {
                let replay = value.get("replayShellCommand").or_else(|| value.get("ReplayShellCommand")).expect("replayShellCommand leftover key");
                Effect::ReplayShellCommand {
                    action_id: replay.get("actionId").or_else(|| replay.get("action_id")).and_then(semio_framework_value::DslValue::as_str).expect("actionId").into(),
                    args: replay.get("args").cloned(),
                }
            }
        };
        assert_eq!(decoded, effect);
    }

    /// 👥️ Contract-freeze §C7.6 peer setup — M2 (ticket 26/08/17 `design-unified.md`) makes
    /// this the real presence-derivation fixture the prior packet's own gap note anticipated.
    ///
    /// 👥️ M2: `stamp_and_cache_interaction_ui` no longer writes selection/hover back onto the
    /// tree itself (`TreeNode`/`Component::TreeItem` still carry no presence field — see that
    /// method's own doc comment for why "the framework wins" onto the tree is gone for good);
    /// it derives `ui_contract::PresenceUpdate`s into `self.pending_presence` instead, asserted
    /// below.
    ///
    /// 👥️ M2 acceptance: selecting item-1 (own) with alice ALSO selecting it derives exactly
    /// one `PresenceUpdate` for that node — own.selected true, one peer mark, own color
    /// threaded through, with the app-local body key before runtime surface binding.
    #[semio_framework_async_macros::async_test]
    async fn ui_tree_stamping_caches_interaction_topology_from_a_domain_bound_tree() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        app.dispatch_typed(TestCommand::SetLabel { value: "seed".into() }, &meta(), &mut identity,cx).await.expect("seed label");
        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;

        app.roster.own_color = Some(9);
        let mut peer_presence = PeerPresenceRoot::empty();
        peer_presence
            .insert(
                "user:alice#s1".to_string(),
                PeerPresence {
                    color: Some(3),
                    surface: None,
                    interaction: Some(PresenceInteraction {
                        app_id: "s.test.synthetic@1/*#editor".to_string(),
                        domains: vec![PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: vec!["item-1".to_string()], hovered: Vec::new() }],
                    }),
                },
            )
            .expect("peer fixture fits fixed root");
        let previous = std::mem::replace(&mut *app.roster.peer_presence, std::sync::Arc::new(peer_presence));
        assert!(previous.is_empty());
        drop(previous);

        let item = TreeNode::try_new(
            "item-1",
            Component::TreeItem(TreeItemProps {
                label: Label(UiText::try_from_str("Item 1").expect("bounded fixture")),
                description: None,
                icon: None,
                default_open: None,
                draggable: None,
                drag_data: None,
                dimmed: None,
                selected: None,
                window: None,
                granularity: None,
                inline_toolbar: None,
                detail: None,
                row_actions: UiFixedList::default(),
                target: None,
            }),
        )
        .expect("bounded fixture");
        let section = TreeNode::try_new("sec", Component::TreeSection(TreeSectionProps { label: None, default_open: None, header_toolbar: None, window: None })).expect("bounded fixture").try_with_children([item]).unwrap_or_else(|_| panic!("bounded fixture"));
        let root = TreeNode::try_new("root", Component::Tree(TreeProps { presentation: Default::default(), interaction_domain: Some(UiText::try_from_str("items").expect("bounded fixture")) }))
            .expect("bounded fixture")
            .try_with_children([section])
            .unwrap_or_else(|_| panic!("bounded fixture"));
        let tree = ComponentTree::new(root);
        let state = app.interaction_state().await;
        app.tools.stamp_and_cache_interaction_ui(&mut app.roster, &mut app.interaction, &tree, &state, "window").await.expect("bounded fixture");
        let topology = app.interaction.interaction_ui_topology.get("items").expect("topology cached for the items domain");
        assert_eq!(topology.ordered.len(), 1);
        assert_eq!(topology.ordered[0].id, "item-1");

        assert_eq!(app.roster.pending_presence.len(), 1, "expected exactly one dirty presence key, got {:?}", app.roster.pending_presence);
        let update = &app.roster.pending_presence[0];
        assert_eq!(update.surface, semio_framework_ui_contract::SurfaceId::try_from("window").expect("bounded fixture"));
        assert_eq!(update.node_key, "item-1");
        assert!(update.own.selected, "own selection must be reported");
        assert!(!update.own.hovered);
        assert_eq!(update.own.color, Some(9));
        assert_eq!(update.peers.len(), 1);
        assert_eq!(update.peers[0].actor, "user:alice#s1");
        assert_eq!(update.peers[0].color, Some(3));
        assert!(update.peers[0].selected);
        assert!(!update.peers[0].hovered);
    
        artifact_app_laws::close_registered_fixture_app(&mut app.0,app.1,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    /// 🧹️ A selection a later render DROPS must be retired on the render plane. `derive_node_presence`
    /// reports only marked nodes and `PresenceHub::record_own` holds own presence until it is
    /// overwritten, so without `retire_dropped_presence` the first selected row stays painted selected
    /// for the life of the session on BOTH renderers — the row the user deselected never hears about it.
    #[semio_framework_async_macros::async_test]
    async fn a_dropped_selection_publishes_a_cleared_presence_update() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = interaction_app_under_test(mounted_policy, &mut identity).await;
        let row = |key: &str, label: &str| {
            TreeNode::try_new(
                key,
                Component::TreeItem(TreeItemProps {
                    label: Label(UiText::try_from_str(label).expect("bounded fixture")),
                    description: None,
                    icon: None,
                    default_open: None,
                    draggable: None,
                    drag_data: None,
                    dimmed: None,
                    selected: None,
                    window: None,
                    granularity: None,
                    inline_toolbar: None,
                    detail: None,
                    row_actions: UiFixedList::default(),
                    target: None,
                }),
            )
            .expect("bounded fixture")
        };
        let tree = || {
            let section = TreeNode::try_new("sec", Component::TreeSection(TreeSectionProps { label: None, default_open: None, header_toolbar: None, window: None }))
                .expect("bounded fixture")
                .try_with_children([row("item-1", "Item 1"), row("item-2", "Item 2")])
                .unwrap_or_else(|_| panic!("bounded fixture"));
            let root = TreeNode::try_new("root", Component::Tree(TreeProps { presentation: Default::default(), interaction_domain: Some(UiText::try_from_str("items").expect("bounded fixture")) }))
                .expect("bounded fixture")
                .try_with_children([section])
                .unwrap_or_else(|_| panic!("bounded fixture"));
            ComponentTree::new(root)
        };

        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-1")), &mut identity).await;
        let state = app.interaction_state().await;
        app.tools.stamp_and_cache_interaction_ui(&mut app.roster, &mut app.interaction, &tree(), &state, "window").await.expect("bounded fixture");
        let first: Vec<(String, bool)> = app.roster.pending_presence.drain(..).map(|update| (update.node_key, update.own.selected)).collect();
        assert_eq!(first, vec![("item-1".to_string(), true)]);

        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-2")), &mut identity).await;
        let state = app.interaction_state().await;
        app.tools.stamp_and_cache_interaction_ui(&mut app.roster, &mut app.interaction, &tree(), &state, "window").await.expect("bounded fixture");
        let second: Vec<(String, bool)> = app.roster.pending_presence.drain(..).map(|update| (update.node_key, update.own.selected)).collect();
        assert!(second.contains(&("item-2".to_string(), true)), "the newly selected row must be marked: {second:?}");
        assert!(second.contains(&("item-1".to_string(), false)), "the row that LOST the selection must be retired, not left marked: {second:?}");

        reserved_action(&mut app, INTERACTION_SELECT_ACTION_ID, Some(&interaction_target_args(json!({ "domainId": "items", "merge": "replace", "method": "pick" }), "item-2")), &mut identity).await;
        let state = app.interaction_state().await;
        app.tools.stamp_and_cache_interaction_ui(&mut app.roster, &mut app.interaction, &tree(), &state, "window").await.expect("bounded fixture");
        let third: Vec<(String, bool)> = app.roster.pending_presence.drain(..).map(|update| (update.node_key, update.own.selected)).collect();
        assert_eq!(third, vec![("item-2".to_string(), true)], "an unchanged selection must cost exactly its own mark, never a repeated retirement");
        close_reserved_app(&mut app, mounted_policy,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }
    mod mounted_owned_child_laws { include!("../🔬️plugin-runtime-plugin-builder-contract/🪟️mounted-owned-child/🦀️.rs"); }