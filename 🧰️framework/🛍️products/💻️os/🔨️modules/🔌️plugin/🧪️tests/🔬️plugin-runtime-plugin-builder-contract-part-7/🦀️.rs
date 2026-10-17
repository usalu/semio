//! 🔬️ Part 7 of 7 of the plugin builder contract laws (cut from `🔬️plugin-runtime-plugin-builder-contract`; the fixtures several parts use are the support modules).
//! 🧪️ The plugin contract's own unit test: a `TestApp` implementing the pure `ArtifactApp`
//! surface (B1), wrapped in `VcsArtifactApp`, exercising typed operations with true inverses, config
//! operations that emit no document operations, history interception, and remote-operation ingest
//! idempotency. `TestCommand` is `TestApp`'s typed `Self::Command`; framework-reserved verbs
//! (history/clipboard/revert/filter/noteShellCommand) still dispatch by string via `handle_action`/
//! `handle_command` — everything app-specific dispatches via `dispatch_typed`.


use semio_framework_plugin_decl_views::media_contract::ARTIFACT_OUTPUT_CHUNK_BYTES;
use semio_framework_plugin_plugin_program::AppInstance;
use semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportResult;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedToolJob;
use semio_framework_plugin_decl_emission::AsyncTaskSource;
use protocol::LOCAL_ACTOR_ID;
use semio_framework_plugin_decl_views::PeerPresenceRoot;
use semio_framework_plugin_plugin_program::Plugin;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_plugin_window_kits::SelectionKind;
use semio_framework_plugin_decl_app::tool_contract::ToolOwnerWitness;
use semio_framework_plugin_job::TypedOperationLeftover;
use serde_json::Value;
use semio_framework_value::ValueError;
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_plugin_program::encode_wire_serialized;
use semio_framework_plugin_composition_publication::group;
use semio_framework_plugin_command::history_row_applied;
use semio_framework_plugin_command::history_row_is_recorded;
use semio_framework_plugin_instance_docs::docs_maintenance_ladder::item;
use semio_framework_plugin_decl_app::media_contract;
use semio_framework_plugin_plugin_program::plugin_runtime;
use semio_framework_os_kernel as protocol;
use semio_framework_plugin_job::reserved_jobs;
use semio_framework_os_kernel as store;
#[cfg(test)]
use semio_framework_plugin_tests_02::child_test_hosts::child_unit_2::test_mounted_command_history_pages;
#[cfg(test)]
use semio_framework_plugin_tests_02::child_test_hosts::child_unit_2::test_mounted_command_history_replacement;
#[cfg(test)]
use semio_framework_plugin_tests_02::publication_mounted_test_hosts::publication_mounted_unit_1::test_mounted_private_child_frame;
use semio_framework_plugin_abi::ArtifactAnalysis;
use semio_framework_plugin_abi::ArtifactAnalyzer;
use semio_framework_plugin_abi::ArtifactBuilder;
use semio_framework_plugin_abi::ArtifactChildren;
use semio_framework_plugin_abi::ArtifactComposer;
use semio_framework_plugin_abi::ArtifactComposition;
use semio_framework_plugin_abi::ArtifactDecomposer;
use semio_framework_plugin_decl_surface::ArtifactEditor;
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob;
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_decl_surface::ArtifactViewer;
use semio_framework_plugin_decl_emission::AsyncTaskRun;
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
use semio_framework_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::{RENDER_CONTEXT_PROBE, TEST_APP_TOOLS_NONE, TEST_APP_WINDOW_BODY_KEY, TestApp, TestCommand, TestRestartFactory, TestTaskSource, __semio_plugin_bundle, contract_app_raw, contract_registry, drive_job_close, fixture_close_grant, interaction_app_definition, interaction_app_raw, job_close_grant, meta, synthetic_play_app, test_app_surface_id, test_document_preparation_factory, test_task};
use semio_framework_ui_contract::*;
use semio_framework::*;
use semio_framework::kernel::*;
use semio_framework_plugin_reactor_app::__semio_dispatch_PluginApp;
use semio_framework_plugin_job::test_artifact_owned_tool_job_context_identity_digest;
use semio_framework_plugin_tests_01::retained_command_test_hosts::retained_command_unit_1::test_retained_document_cancellation;
use semio_framework_plugin_tests_01::retained_command_test_hosts::retained_command_unit_1::test_retained_latest_wins_slot_and_publication_fairness;
use semio_framework_plugin_tests_01::retained_command_test_hosts::retained_command_unit_1::test_retained_cancellation_publication_boundaries;
use semio_framework_value::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use semio_framework_plugin_runtime_ops::ops_surface_render::ContextMenuWireRequest;
use semio_framework_plugin_decl_emission::{ActionMeta, Emit, TaskCtx, TaskResolution};
use semio_framework_plugin_app_builder::App;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_window_kits::AppActionRegistry;
use semio_framework_plugin_decl_views::media_contract::ArtifactOutputChunks;
use semio_framework_plugin_decl_views::{InteractionHoverState, InteractionView, NoDraftMutation};
use semio_framework_plugin_time_travel_panel::command_history_panel_lift::ui_history_panel;
use semio_framework_plugin_decl_views::history_view::CommandView;
use semio_framework_plugin_decl_views::history_view::HistoryCommandFilter;
use semio_framework_plugin_decl_views::history_view::HistoryView;
use semio_framework_plugin_window_kits::Menu;
use semio_framework_plugin_decl_views::PeerPresence;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_abi::{ArtifactDeserializer, ArtifactSerializer, deserializer_entry_of, serializer_entry_of};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_os_kernel::io::{ErasedComposeSource, IoPayload};
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::PublicationPresence;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::transient::PublicationTransient;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::transient::mutations::PublicationTransientMutation;
use semio_framework_diagnostic::FaultFrom;
use semio_framework::IconName;
use semio_framework::ViewModel;
use semio_framework::{MediaClass, MediaType};
use semio_framework_plugin_window_kits::selection_count_phrase;
use semio_framework_os_kernel::Mutation;
use semio_framework::Fault;
use semio_framework::kernel::UiDirtyScope;
use semio_framework::{ActionDefinition, ActionKind, CommandDefinition, InteractionVerb, MediaForm};
use semio_framework_job::InteractiveJob as _;
use semio_framework_plugin_window_kits_rich::tree_kit::TREE_WINDOW_DEFAULT_ROWS;
use semio_framework::TreeWindowRequest;
use semio_framework_ui_contract::{ActionId, BuiltNode, SurfaceId, Trigger, TreeWindow, UiIntent, UiNodeId, UiRevision, UI_BUILT_CHILDREN_MAX, UI_VALUE_PAGE_ROWS};
use serde_json::json;
use std::collections::BTreeMap;
use pack::snapshot::ArtifactPack;
use semio_framework_os_kernel::os_store::backbone::Backbone;
use semio_framework_os_kernel::{MemberFactory, SpaceMember};
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
include!("../🔬️plugin-runtime-plugin-builder-contract-part-7-x/🦀️.rs");


    /// 🏷️ The recorded digest is an oracle for ONE digest domain. Its domain tag is bumped
    /// (`ARC-CONTEXT-3` → `-4` …) exactly when the identity's input set changes, which is what
    /// invalidates the literal — so the tag is joined here first and a drift fails on the clause
    /// that names its cause instead of on a bare number.
    #[test]
    fn app_owned_request_context_identity_matches_language_neutral_oracle_and_rejects_every_root_drift() {
        let fixture: Value = serde_json::from_str(include_str!("../../🧵️retained-command/🧪️tests/🧫️fixtures/🧬️request-context.json")).expect("request context fixture");
        let hex = |text: &str| text.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).expect("hex pair"), 16).expect("hex byte")).collect::<Vec<_>>();
        let revision: [u8; 32] = hex(fixture["canonicalBaseRevisionHex"].as_str().expect("revision hex")).try_into().expect("revision width");
        let expected = u64::from_str_radix(fixture["expectedIdentityDigestHex"].as_str().expect("digest hex"), 16).expect("digest");
        let app_instance_id = fixture["appInstanceId"].as_u64().expect("app instance") as u32;
        let draft_generation = fixture["draftGeneration"].as_u64().expect("draft generation");
        let transient_generation = fixture["transientGeneration"].as_u64().expect("transient generation");
        let children_digest = u64::from_str_radix(fixture["childrenDigestHex"].as_str().expect("children digest hex"), 16).expect("children digest");
        let identity = test_artifact_owned_tool_job_context_identity_digest;
        let source = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../👷️job/🦀️.rs"));
        let tag = source.split("let mut digest = extend(0xcbf2_9ce4_8422_2325, b\"").nth(1).expect("the identity digest seeds itself with its own domain tag").split('"').next().expect("domain tag literal");
        assert_eq!(tag, fixture["domainTag"].as_str().expect("fixture domain tag"), "the recorded oracle belongs to a different identity domain — re-record the digest with the tag that replaced it");
        assert_eq!(identity(app_instance_id, revision, draft_generation, transient_generation, children_digest), expected);
        assert_eq!(fixture["mismatchCases"].as_array().expect("mismatch cases").len(), 5);
        assert_ne!(identity(app_instance_id + 1, revision, draft_generation, transient_generation, children_digest), expected);
        let mut changed_revision = revision;
        changed_revision[31] ^= 1;
        assert_ne!(identity(app_instance_id, changed_revision, draft_generation, transient_generation, children_digest), expected);
        assert_ne!(identity(app_instance_id, revision, draft_generation + 1, transient_generation, children_digest), expected);
        assert_ne!(identity(app_instance_id, revision, draft_generation, transient_generation + 1, children_digest), expected);
        assert_ne!(identity(app_instance_id, revision, draft_generation, transient_generation, children_digest ^ 1), expected);
        assert_eq!(fixture["restartBoundary"]["workerThreadLoss"], "exact-resume-with-retained-context");
        assert_eq!(fixture["restartBoundary"]["processRestartWithoutExactTransientSnapshot"], "fail-closed");
    }

    /// 🧪️ Trivial dummy `ArtifactSerializer`/`ArtifactDeserializer` pair, round-tripping
    /// `TestSnapshot` to itself, for `serializer_entry_of`/`deserializer_entry_of` smoke tests.
    struct DummySerializer;
    impl ArtifactSerializer for DummySerializer {
        type From = TestSnapshot;
        type Into = TestSnapshot;
        const FROM: Dialect = Dialect { artifact_kind: "s.test.dummy", standard: StandardId("1"), subset: SubsetId("*") };
        const INTO: Dialect = Dialect { artifact_kind: "s.test.dummy.out", standard: StandardId("1"), subset: SubsetId("*") };
        async fn serialize(from: &TestSnapshot) -> Result<TestSnapshot, store::PackError> {
            Ok(from.clone())
        }
    }

    struct DummyDeserializer;
    impl ArtifactDeserializer for DummyDeserializer {
        type From = TestSnapshot;
        type Into = TestSnapshot;
        const FROM: Dialect = Dialect { artifact_kind: "s.test.dummy.out", standard: StandardId("1"), subset: SubsetId("*") };
        const INTO: Dialect = Dialect { artifact_kind: "s.test.dummy", standard: StandardId("1"), subset: SubsetId("*") };
        async fn deserialize(from: &TestSnapshot) -> Result<TestSnapshot, store::PackError> {
            Ok(from.clone())
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn serializer_entry_of_and_deserializer_entry_of_erase_correctly() {
        let ser = serializer_entry_of::<DummySerializer>();
        assert_eq!(ser.writes, DummySerializer::INTO);
        assert_eq!(ser.reads.to_vec(), vec![DummySerializer::FROM]);
        let de = deserializer_entry_of::<DummyDeserializer>();
        assert_eq!(de.writes, DummyDeserializer::INTO);
        assert_eq!(de.reads.to_vec(), vec![DummyDeserializer::FROM]);

        let seed = TestSnapshot { count: 7, label: "x".into(), slot: Vec::new() };
        let bytes = ArtifactPack::encode_pack(&seed);
        let composed = ::semio_framework_async::poll::resolve_ready((ser.compose)(&[ErasedComposeSource { dialect: DummySerializer::FROM, payload: IoPayload::Binary(bytes) }])).expect("serializer_entry_of erased compose should succeed with exactly 1 source");
        assert_eq!(composed.dialect, DummySerializer::INTO);
        match composed.payload {
            IoPayload::Binary(out) => assert_eq!(<TestSnapshot as ArtifactPack>::decode_pack(&out).unwrap(), seed),
            IoPayload::Text(_) => panic!("expected Binary payload"),
        }

        let zero_sources_err = match ::semio_framework_async::poll::resolve_ready((de.compose)(&[])) {
            Err(err) => err,
            Ok(_) => panic!("deserializer_entry_of erased compose should reject 0 sources"),
        };
        assert!(zero_sources_err.message.contains("needs exactly 1 source"), "{}", zero_sources_err.message);
        let two_sources = [ErasedComposeSource { dialect: DummyDeserializer::FROM, payload: IoPayload::Binary(Vec::new()) }, ErasedComposeSource { dialect: DummyDeserializer::FROM, payload: IoPayload::Binary(Vec::new()) }];
        let two_sources_err = match ::semio_framework_async::poll::resolve_ready((de.compose)(&two_sources)) {
            Err(err) => err,
            Ok(_) => panic!("deserializer_entry_of erased compose should reject 2 sources"),
        };
        assert!(two_sources_err.message.contains("needs exactly 1 source"), "{}", two_sources_err.message);
    }



    #[derive(semio_framework_value::FactoryPayloadRetirement)]
    struct PublicationPresenceRetirementFactory;

    impl store::SnapshotRetirementFactory<PublicationPresence> for PublicationPresenceRetirementFactory {
        fn retirement_birth_bytes(&self, _snapshot: &std::sync::Arc<PublicationPresence>) -> usize { semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<PublicationPresence>() }
        fn retire(&self, snapshot: std::sync::Arc<PublicationPresence>, grant: semio_framework_value::RetainedCloneGrant) -> Result<(Box<dyn store::ErasedSnapshotRetirement>, semio_framework_value::RetainedCloneProgress), (semio_framework_value::ValueError, std::sync::Arc<PublicationPresence>)> {
            semio_framework_value::retirement::shared::admit_shared_retirement(snapshot, grant, true)
        }
    }

    /// 🚫️ The UNPROVED twin: `contract_registry`'s `BatchOnlyPendingRewrite` declarations against an
    /// app that owns no tool factory at all, for the laws whose whole subject is the refusal.
    /// 🚫️ A `TestApp` that registers no owned factory at all, on `contract_registry` — every verb
    /// `BatchOnlyPendingRewrite`. The migrated registry is not an option here: an app declaring migrated
    /// verbs and supplying no proof row cannot be CONSTRUCTED at all (`interactive-job.catalog-incomplete`),
    /// which is the same fail-closed join from the other side.
    async fn unproved_contract_app(mounted_policy: semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> VcsArtifactApp<TestApp<false, TEST_APP_TOOLS_NONE>> {
        VcsArtifactApp::with_registry(TestApp::<false, TEST_APP_TOOLS_NONE>::default(), contract_registry().await, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, identity).await
    }

    #[test]
    fn shared_framework_job_is_cancellable_resumable_and_boundedly_closeable() {
        let policy=semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy();
        let mut before=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(7),policy.preparation,||Some(0),u64::MAX,4096);
        before.cancel().cancel_now();
        let mut cancelled_before=FrameworkSetSelectionModeJob::new(vec![1;8193],1);
        assert!(matches!(before.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut cancelled_before,cx))).unwrap(),semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Cancelled));
        semio_framework_job::InteractiveJob::begin_close(&mut cancelled_before);drive_job_close(&mut cancelled_before,&mut before);before.retire(policy.close);

        let mut after=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::allocate_operation_id(),semio_framework_job::Generation(7),policy.preparation,||Some(0),u64::MAX,4096);
        let mut cancelled_after=FrameworkSetSelectionModeJob::new(vec![1;8193],1);
        assert!(matches!(after.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut cancelled_after,cx))).unwrap(),semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Preview(_)));
        after.cancel().cancel_now();
        assert!(matches!(after.turn(policy.maintenance,|cx|Ok(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_step(&mut cancelled_after,cx))).unwrap(),semio_framework_plugin_testing_laws::artifact_app_laws::FixtureStepOutcome::Cancelled));
        semio_framework_job::InteractiveJob::begin_close(&mut cancelled_after);drive_job_close(&mut cancelled_after,&mut after);after.retire(policy.close);

        let operation=semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(),semio_framework_job::RevisionId(11),semio_framework_job::Generation(3),29);
        let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation,operation.generation,policy.preparation,||Some(0),u64::MAX,4096);
        let mut retained=ArtifactReservedToolJob::new(FrameworkSetSelectionModeJob::new(vec![2;8193],1));
        retained.bind_operation(operation).expect("first exact operation authority");
        assert!(retained.clone().bind_operation(operation).is_err(),"a second factory transfer cannot replace the live operation generation");
        semio_framework_job::InteractiveJob::begin_close(&mut retained);
        let quoted=job_close_grant(&retained);
        assert_eq!(caller.turn(semio_framework_value::RetainedCloneGrant{maximum_items:0,..quoted},|cx|semio_framework_job::InteractiveJob::close_step(&mut retained,cx)).unwrap(),semio_framework_job::InteractiveJobCloseStep::Pending{progress:Default::default()});
        let(mut turns,mut released)=(0usize,0usize);
        while !semio_framework_job::InteractiveJob::terminal_is_empty(&retained){
            let grant=job_close_grant(&retained);let step=caller.turn(grant,|cx|semio_framework_job::InteractiveJob::close_step(&mut retained,cx)).unwrap();
            assert!(!matches!(step,semio_framework_job::InteractiveJobCloseStep::Blocked),"original close turn {turns} blocked: {step:?}");
            assert_eq!(step.progress(),caller.receipt());assert!(step.progress().fits(grant));released+=step.progress().released_bytes;turns+=1;assert!(turns<100000);
        }
        assert!(released>=8193,"physical bytes must leave through the original release axis");
        let terminal=job_close_grant(&retained);
        assert_eq!(caller.turn(terminal,|cx|semio_framework_job::InteractiveJob::close_step(&mut retained,cx)).unwrap(),semio_framework_job::InteractiveJobCloseStep::Complete{progress:Default::default()});
        caller.retire(policy.close);
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_latest_wins_cancellation_guards_real_store_publication_and_preserves_committed_ack() {
        test_retained_cancellation_publication_boundaries::<TestApp<false, TEST_APP_TOOLS_FACTORIES>>().await;
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_latest_wins_reserved_slots_and_ready_publisher_are_fair() {
        test_retained_latest_wins_slot_and_publication_fairness::<TestApp<false, TEST_APP_TOOLS_FACTORIES>>().await;
    }

    #[test]
    fn retained_latest_wins_raw_capacity_is_not_initialized_byte_retirement() {
        semio_framework_plugin_retained_command::test_raw_allocation_close::<TestApp>();
    }

    #[semio_framework_async_macros::async_test]
    async fn retained_latest_wins_real_document_publication_cancellation_and_delayed_ack_close() {
        test_retained_document_cancellation::<TestApp<false, TEST_APP_TOOLS_FACTORIES>>(store::mutation_apply_preparation_factory::<TestSnapshot, TestMutation>(), || TestMutation::SetCount(SetCount { value: 42 }), |snapshot| snapshot.count).await;
    }





    #[test]
    fn completed_media_result_uses_explicit_mime_instead_of_schema() {
        let chunks = ArtifactOutputChunks::new(4);
        assert_eq!(chunks.push(vec![1, 2, 3, 4]), Ok(4));
        assert_eq!(chunks.seal(), Ok(4));
        let result = semio_framework_plugin_decl_views::media_contract::ArtifactMediaExportResult::structured(MediaType { class: MediaClass::Presentation, form: MediaForm::Sequence }, "stdio.mp4.isobmff", "video/mp4", chunks).expect("completed media");
        assert_eq!(result.mime_type, "video/mp4");
        assert_eq!(result.schema, "stdio.mp4.isobmff");
        assert_ne!(result.mime_type, result.schema);
        assert_eq!(result.chunks.bytes(), 4);
        assert!(result.chunks.is_sealed());
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_close_final_destructor_is_constant_after_every_owned_field_is_drained() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let page = store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
        let mut app = contract_app_raw(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(), &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await;
        let _ = app.render_ctx().test_history().await;
        let mut turns = 0;
        for _ in 0..4_096 {
            if app.ladder_ref().retained_fields_own_terminal_is_empty() {
                break;
            }
            let demand = app.ladder_ref().retained_fields_close_demand(page).expect("quoted retained field demand");
            let grant = fixture_close_grant(demand);
            let starved = store::RetainedCloneGrant { maximum_items: 0, ..grant };
            assert_eq!(app.ladder_ctx().retained_fields_close_step(starved).expect("unfunded retained field turn yields").progress(), store::RetainedCloneProgress::default());
            let progress = app.ladder_ctx().retained_fields_close_step(grant).expect("retained field turn").progress();
            assert!(progress.fits(grant), "retained field receipt {progress:?} exceeds its exact grant {grant:?}");
            assert_ne!(progress, store::RetainedCloneProgress::default(), "a funded retained field turn must move one owner");
            turns += 1;
        }
        assert!(turns != 0, "the warmed projection cache and history view are retained fields the rung must release");
        assert!(app.ladder_ref().retained_fields_own_terminal_is_empty());
        assert!(matches!(app.ladder_ctx().retained_fields_close_step(semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close).expect("repeated terminal turn"), store::RetainedCloneStep::Complete(progress) if progress == store::RetainedCloneProgress::default()));
        artifact_app_laws::close_registered_fixture_app(&mut app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[test]
    fn retained_field_maximum_and_maximum_plus_one_are_language_neutral() {
        use store::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
        for (letter, length) in [("x", ARTIFACT_OUTPUT_CHUNK_BYTES), ("y", ARTIFACT_OUTPUT_CHUNK_BYTES + 1)] {
            let mut registry = AppActionRegistry::test_with_controller_id(letter.repeat(length));
            let detach = fixture_close_grant(registry.retirement_demands());
            assert_eq!(registry.close_step(detach).expect("row detaches"), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }), "the row detaches whole");
            let quote = registry.retirement_demands();
            assert!(quote.release_bytes >= length, "the detached identifier is quoted as one whole release");
            let exact = fixture_close_grant(quote);
            let starved = RetainedCloneGrant { maximum_release_bytes: quote.release_bytes - 1, ..exact };
            assert_eq!(registry.close_step(starved).expect("starved release yields"), RetainedCloneStep::Progress(RetainedCloneProgress::default()), "a release grant below the quote yields instead of refusing");
            assert!(!registry.terminal_is_empty(), "the unreleased allocation stays retained until its own whole grant");
            assert_eq!(registry.close_step(exact).expect("exact release"), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: quote.release_bytes, ..Default::default() }), "a grant at the quote retires the identifier in one whole release");
            assert!(registry.terminal_is_empty());
            let terminal = fixture_close_grant(registry.retirement_demands());
            assert_eq!(registry.close_step(terminal).expect("terminal"), RetainedCloneStep::Complete(RetainedCloneProgress::default()));
            assert_eq!(registry.close_step(terminal).expect("repeated terminal"), RetainedCloneStep::Complete(RetainedCloneProgress::default()), "repeated close is idempotent after interruption and resume");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn unproved_command_fails_before_an_overrun_reducer_can_start() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = unproved_contract_app(mounted_policy, &mut identity).await;
        assert!(!app.tools.test_registered_tool_keys().iter().any(|key| key.1 == "watchdogOverrun"), "the unproved fixture registers no owned factory for this verb");
        let error = app.dispatch_typed(TestCommand::WatchdogOverrun, &meta(), &mut identity).await.expect_err("an unproved operation must fail before an over-budget reducer starts");
        assert_eq!(error.code.0, "interactive-job.not-ui-safe", "a verb with no proof is a verb that was never migrated, and the backstop names the cause");
        assert_eq!(app.stores.test_snapshot().await.count, 0, "the over-budget reducer never started");
        artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn plugin_builder_builds_bundle_from_fluent_spec() {
        let bundle = __semio_plugin_bundle().await.expect("synthetic plugin assembly");
        assert_eq!(bundle.manifest.plugin_id, "test");
        assert_eq!(bundle.manifest.label.as_str(), "Synthetic");
        assert_eq!(bundle.manifest.version, "0.0.1");
        assert!(bundle.manifest.apps.iter().any(|app| app.id == TestApp::<false>::APP_ID));
    }

    #[semio_framework_async_macros::async_test]
    async fn test_app_id_matches_its_own_dialect() {
        assert_eq!(TestApp::<false>::APP_ID, test_app_surface_id().await, "TestApp::<false>::APP_ID must not drift from TEST_APP_DIALECT (contract §1)");
    }

    #[semio_framework_async_macros::async_test]
    async fn plugin_builder_wires_app_factory_for_create_app() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32_768,maximum_capacity_bytes:262_144,maximum_release_bytes:1_048_576,maximum_depth:4_096};
        let mounted_policy=semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:mounted_grant,maintenance:mounted_grant,close:mounted_grant};
        let identity_started=std::time::Instant::now();
        let mut identity_progress=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{progress.owned_bytes<=1_048_576&&identity_started.elapsed()<std::time::Duration::from_secs(60)};
        let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let bundle = __semio_plugin_bundle().await.expect("synthetic plugin assembly");
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🏪️store/🧪️tests/🧫️fixtures/🧫️actor-genesis/🔣️.json")).expect("neutral actor/genesis fixture");
        let actor = protocol::ActorId(fixture["actors"]["opened"].as_str().expect("opened actor").into());
        let mut app = bundle.create_app(TestApp::<false>::APP_ID, actor.clone(), mounted_policy, &mut identity).expect("registered app");
        assert_eq!(app.app_id().await, TestApp::<false>::APP_ID);
        let TestRuntimeApps::Test(owner) = &mut app;
        semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(owner, mounted_policy);
        assert_eq!(owner.store.local_actor_id(), &actor);
        assert_eq!(owner.config_store.local_actor_id(), &actor);
        assert_eq!(owner.draft_store.local_actor_id(), &actor);
        assert_eq!(owner.interaction_store.local_actor_id(), &actor);
        let roots = [owner.store.local_actor_id(), owner.config_store.local_actor_id(), owner.draft_store.local_actor_id(), owner.interaction_store.local_actor_id()];
        assert_eq!(serde_json::to_value(roots.map(|actor| actor.0.as_str())).expect("independent actor projection"), serde_json::json!([fixture["actors"]["opened"], fixture["actors"]["opened"], fixture["actors"]["opened"], fixture["actors"]["opened"]]));
        let pack = owner.document_pack().await.expect("stored document");
        artifact_app_laws::load_document(owner, &pack, &mut identity).await.expect("production reload");
        assert_eq!(owner.store.local_actor_id(), &actor, "reload retains the opened actor");
        assert!(bundle.create_app("unknown-app", actor, mounted_policy, &mut identity).is_none());
        artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn interaction_view_peers_selecting_returns_actor_and_color() {
        let mut peers = PeerPresenceRoot::empty();
        let mark_interaction = |selected: &[&str], hovered: &[&str]| {
            Some(PresenceInteraction {
                app_id: "draw".to_string(),
                domains: vec![PresenceDomain { domain: "items".to_string(), granularity: "item".to_string(), selected: selected.iter().map(|id| id.to_string()).collect(), hovered: hovered.iter().map(|id| id.to_string()).collect() }],
            })
        };
        peers.insert("user:zed#s1".to_string(), PeerPresence { color: Some(7), surface: None, interaction: mark_interaction(&["item-1"], &[]) }).expect("zed peer");
        peers.insert("user:alice#s1".to_string(), PeerPresence { color: Some(2), surface: None, interaction: mark_interaction(&["item-1"], &[]) }).expect("alice peer");
        peers.insert("user:bob#s1".to_string(), PeerPresence { color: Some(4), surface: None, interaction: mark_interaction(&[], &["item-1"]) }).expect("bob peer");

        let state = InteractionState::default();
        let hover = InteractionHoverState::new();
        let view = InteractionView { state: &state, hover: &hover, peers: &peers, draft_references: &std::collections::BTreeMap::new() };

        let selecting = view.peers_selecting("items", "item-1");
        assert_eq!(selecting.len(), 2, "only alice and zed selected item-1");
        assert_eq!(selecting[0].actor, "user:alice#s1", "sorted by actor");
        assert_eq!(selecting[0].color, Some(2));
        assert_eq!(selecting[1].actor, "user:zed#s1");
        assert_eq!(selecting[1].color, Some(7));
        assert!(view.peers_selecting("items", "item-2").is_empty(), "no peer selected item-2");

        let hovering = view.peers_hovering("items", "item-1");
        assert_eq!(hovering.len(), 1);
        assert_eq!(hovering[0].actor, "user:bob#s1");
        assert_eq!(hovering[0].color, Some(4));
    }

    impl store::os_store::document_store_owners::MemberStoreProducer<TestSnapshot,TestMutation> for semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::TestMemberStoreProducer {
        /// 🚪️ The fixture child is a real openable member, not an un-openable one. While this was
        /// `UnsupportedMemberSnapshotOpen` every composed replacement that carried a `TestMembers`
        /// candidate was refused at its first member-open step with `MemberOpenDiagnostic::Decode`
        /// (`UnsupportedMemberSnapshotOpen::step` has no other answer), so the three
        /// `retained_composed_replacement_*` laws could never reach `ValidatingClosure` or
        /// `CandidateReady`. `ProducedMemberSnapshotOpen` is the framework's own pack decoder and
        /// `TestSnapshot` is an `ArtifactPack`, which is exactly what a real member declares.
        type SnapshotOpen = store::ProducedMemberSnapshotOpen<TestSnapshot,store::ArtifactPackSnapshotDecoder<TestSnapshot>>;

        fn member_store_owners_birth_demand(&self) -> Result<semio_framework_value::retained_clone::RetainedCloneBirthDemand, semio_framework_value::ValueError> {
            let base = store::os_store::document_store_owners::DocumentStoreOwners::<TestSnapshot, TestMutation>::source_birth_bytes::<semio_framework_value::factory::SharedValueRetirementFactory<TestSnapshot>, semio_framework_value::factory::OwnedValueRetirementFactory<TestSnapshot>, semio_framework_value::factory::OwnedValueRetirementFactory<TestMutation>, store::ArtifactStoreCursorDisposer<TestSnapshot, TestMutation>>()?;
            let preparation = store::operation_wire_preparation_factory_source_birth_demand::<TestSnapshot, TestMutation>(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: store::paged_one_item_factory_birth_bytes::<TestSnapshot, TestMutation, store::MutationApplyEdit<TestSnapshot, TestMutation>>(), depth: 1 })?;
            let capacity_bytes = base.checked_add(preparation.capacity_bytes).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "member original source tree capacity overflow"))?;
            Ok(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes, depth: preparation.depth.max(1) })
        }

        fn member_store_owners(&self,grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(store::os_store::document_store_owners::DocumentStoreOwners<TestSnapshot, TestMutation>, semio_framework_value::retained_clone::RetainedCloneProgress), store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError<TestSnapshot, TestMutation>> {
            let preparation = store::operation_wire_preparation_factory_source_birth_demand::<TestSnapshot, TestMutation>(semio_framework_value::retained_clone::RetainedCloneBirthDemand { capacity_bytes: store::paged_one_item_factory_birth_bytes::<TestSnapshot, TestMutation, store::MutationApplyEdit<TestSnapshot, TestMutation>>(), depth: 1 }).map_err(|error| store::os_store::document_store_owners::DocumentStoreOwnersAdmissionError { error, owners: None, progress: Default::default() })?;
            store::os_store::document_store_owners::DocumentStoreOwners::admit_source_constructor_with_one_item_preparation(grant, preparation, || (
                semio_framework_value::factory::SharedValueRetirementFactory::<TestSnapshot>::default(),
                semio_framework_value::factory::OwnedValueRetirementFactory::<TestSnapshot>::default(),
                semio_framework_value::factory::OwnedValueRetirementFactory::<TestMutation>::default(),
                store::ArtifactStoreCursorDisposer::<TestSnapshot, TestMutation>::new(),
                test_document_preparation_factory(),
            ))
        }
    }

    /// ⏪️ A row follows its parent document lane, else its children (design §20.13, L4: a config-lane edit is never part
    /// of a row). 💠️lowpoly `addPrimitive` and 🎥️shooting `addShot` once read applied for ever after an undo because a
    /// config lane answered for them (ticket 26/09/18 S11 §3.4b, S12 §4); no lane but the document and its children can.
    #[test]
    fn a_row_follows_its_parent_document_lane_and_else_its_children() {
        use semio_framework_plugin_command::history_row_applied as applied;
        assert!(!applied(true, false, true), "a row that published a parent edit is UNDONE once that edit is retracted, however live its child edit is");
        assert!(!applied(true, false, false), "a parent-lane row with nothing applied anywhere is undone");
        assert!(applied(true, true, false), "a parent-lane row whose document edit is applied is applied");
        assert!(applied(true, true, true), "a multi-lane row is applied while its parent edit is");
        assert!(applied(false, false, true), "a CHILD-only row answers with its children — 🌊️flow's addWidget and 🎬️sequence's addStep");
        assert!(!applied(false, false, false), "a row applied nowhere is not applied");
    }

    #[semio_framework_async_macros::async_test]
    async fn copy_cut_paste_are_registered_as_clipboard_kind_actions() {
        let definition = synthetic_play_app().await.definition;
        for id in ["copy", "cut", "paste"] {
            let action = definition
                .actions
                .iter()
                .chain(definition.window_kinds.iter().flat_map(|window| window.actions.iter()))
                .find(|a| a.id == id)
                .unwrap_or_else(|| panic!("{id} must be auto-injected into every app's manifest"));
            assert_eq!(action.kind, ActionKind::Clipboard);
        }
    }

    /// 📌️ Six since the explicit check-in row landed (`#s-checkin`, React's
    /// `framework.history.checkin`) between Commit Checkpoint and Create Alternative.
    #[semio_framework_async_macros::async_test]
    async fn ui_history_panel_filters_rows_and_gates_the_backwards_action() {
        let history = HistoryView {
            columns: Vec::new(),
            can_undo: true,
            can_redo: false,
            active_alternative_id: None, alternatives: Vec::new(),
            current_checkpoint_id: None,
            commands: vec![
                CommandView {
                    seq: 1,
                    action_id: "increment".into(),
                    label: LocalizedLabel::native("Increment", "Erhöhen"),
                    kind: ActionKind::Mutation,
                    timestamp: "0".into(),
                    edit_id: Some("e1".into()),
                    child_edit_ids: Vec::new(),
                    transition_id: None,
                    author: None,
                    op_lines: vec!["set-count value=1".into()],
                    op_count: 1,
                    applied: true,
                    revertible: true,
                    count: 1,
                    inverse: None,
                    transaction: None,
                    mutations: Vec::new(),
                },
                CommandView {
                    seq: 2,
                    action_id: "undo".into(),
                    label: LocalizedLabel::native("Undo", "Rückgängig"),
                    kind: ActionKind::History,
                    timestamp: "1".into(),
                    edit_id: None,
                    child_edit_ids: Vec::new(),
                    transition_id: None,
                    author: None,
                    op_lines: Vec::new(),
                    op_count: 0,
                    applied: false,
                    revertible: false,
                    count: 1,
                    inverse: None,
                    transaction: None,
                    mutations: Vec::new(),
                },
            ],
            command_filter: HistoryCommandFilter::All,
        };
        let all_panel = ui_history_panel(&history, None, None, &Default::default(), "ctrl", Locale::En, false, false, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("bounded history panel");
        assert_eq!(all_panel.children.len(), 2, "Actions + Commands sections");
        let Component::TreeSection(actions_props) = &all_panel.children[0].component else { panic!("expected a TreeSection") };
        assert_eq!(actions_props.label.as_ref().map(|label| label.0.as_str()), Some("Actions"));
        assert_eq!(all_panel.children[0].children.len(), 6, "undo/redo/commit/checkin/alternative/filter");
        assert_eq!(all_panel.children[0].children[3].key.as_str(), "framework.history.checkin", "the check-in row follows Commit Checkpoint");
        let viewer_panel = ui_history_panel(&history, None, None, &Default::default(), "ctrl", Locale::En, true, false, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("viewer history panel");
        assert_eq!(viewer_panel.children[0].children.len(), 5, "a viewer never gets the check-in row at all \u{2014} React's `canCheckIn` gate removes it rather than disabling it");
        assert!(all_panel.children[0].children.iter().all(|item| !item.children.is_empty()), "Actions rows carry their control as a child node");
        let Component::TreeSection(commands_props) = &all_panel.children[1].component else { panic!("expected a TreeSection") };
        assert_eq!(commands_props.label.as_ref().map(|label| label.0.as_str()), Some("Commands"));
        assert_eq!(all_panel.children[1].children.len(), 2);
        let Component::TreeItem(revertible_props) = &all_panel.children[1].children[0].component else { panic!("expected a TreeItem") };
        assert!(!revertible_props.row_actions.is_empty(), "the revertible entry must offer inverse");
        let Component::TreeItem(non_revertible_props) = &all_panel.children[1].children[1].component else { panic!("expected a TreeItem") };
        assert!(non_revertible_props.row_actions.is_empty(), "the non-revertible entry must not offer inverse");

        let only_ops = HistoryView { command_filter: HistoryCommandFilter::OnlyMutations, ..history.clone() };
        let ops_panel = ui_history_panel(&only_ops, None, None, &Default::default(), "ctrl", Locale::En, false, false, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("bounded history panel");
        assert_eq!(ops_panel.children[1].children.len(), 1);
        assert_eq!(ops_panel.children[1].children[0].key.as_str(), "framework.history.entry.1");

        let without_ops = HistoryView { command_filter: HistoryCommandFilter::WithoutMutations, ..history };
        let no_ops_panel = ui_history_panel(&without_ops, None, None, &Default::default(), "ctrl", Locale::En, false, false, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("bounded history panel");
        assert_eq!(no_ops_panel.children[1].children.len(), 1);
        assert_eq!(no_ops_panel.children[1].children[0].key.as_str(), "framework.history.entry.2");
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_history_panel_clips_an_oversized_operation_description() {
        let history = HistoryView {
            columns: Vec::new(),
            can_undo: true,
            can_redo: false,
            active_alternative_id: None, alternatives: Vec::new(),
            current_checkpoint_id: None,
            commands: vec![CommandView {
                seq: 1,
                action_id: "fill".into(),
                label: LocalizedLabel::native("Fill", "Füllen"),
                kind: ActionKind::Mutation,
                timestamp: "0".into(),
                edit_id: Some("e1".into()),
                child_edit_ids: Vec::new(),
                transition_id: None,
                author: None,
                op_lines: vec![format!("register-mesh vertices=[{}]", "1.0 ".repeat(1_024))],
                op_count: 1,
                applied: true,
                revertible: true,
                count: 1,
                inverse: None,
                transaction: None,
                mutations: Vec::new(),
            }],
            command_filter: HistoryCommandFilter::All,
        };
        let panel = ui_history_panel(&history, None, None, &Default::default(), "ctrl", Locale::En, false, false, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("an oversized operation line must not fail admission");
        let Component::TreeItem(props) = &panel.children[1].children[0].component else { panic!("expected a TreeItem") };
        let description = props.description.as_ref().expect("clipped description").as_str();
        assert!(description.starts_with("register-mesh vertices=[1.0 "));
        assert!(description.ends_with(UI_TEXT_CLIP_MARK));
    }

    /// 🏷️ A command label is authored copy that can carry the whole argument form of an edit; a folded
    /// row appends ` xN` on top. Both must clip on a char boundary instead of failing the admission of
    /// every panel in the refresh (seen 2026-09-10: `duplicateSelection` on the puzzle 3d Perspective
    /// window failed `refreshUi` with `history-panel.command-label`).
    #[semio_framework_async_macros::async_test]
    async fn ui_history_panel_clips_an_oversized_command_label_and_its_folded_count() {
        let long_label = format!("Duplicate {}", "seed-left-001 ".repeat(64));
        let entry = |seq: u64, count: u32| CommandView {
            seq,
            action_id: "duplicateSelection".into(),
            label: LocalizedLabel::data(long_label.clone()),
            kind: ActionKind::Mutation,
            timestamp: "0".into(),
            edit_id: Some("e1".into()),
            child_edit_ids: Vec::new(),
            transition_id: None,
            author: None,
            op_lines: Vec::new(),
            op_count: 0,
            applied: true,
            revertible: true,
            count,
            inverse: None,
            transaction: None,
            mutations: Vec::new(),
        };
        let history = HistoryView { columns: Vec::new(), can_undo: true, can_redo: false, active_alternative_id: None, alternatives: Vec::new(), current_checkpoint_id: None, commands: vec![entry(1, 1), entry(2, 3)], command_filter: HistoryCommandFilter::All };
        let panel = ui_history_panel(&history, None, None, &Default::default(), "ctrl", Locale::En, false, false, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("an oversized command label must not fail admission");
        for (index, expected_tail) in [(0, UI_TEXT_CLIP_MARK), (1, UI_TEXT_CLIP_MARK)] {
            let Component::TreeItem(props) = &panel.children[1].children[index].component else { panic!("expected a TreeItem") };
            let label = props.label.0.as_str();
            assert!(label.starts_with("Duplicate seed-left-001 "), "{label}");
            assert!(label.ends_with(expected_tail), "{label}");
            assert!(label.len() <= UI_TEXT_MAX_BYTES, "{label}");
        }
    }

    fn history_window_fixture() -> Value {
        serde_json::from_str(include_str!("../🧫️fixtures/history-panel-command-window/🔣️.json")).unwrap()
    }

    /// 🪙️ A revertible row materialises a `UiValue::Map`; a non-revertible one costs zero argument
    /// arena. The window laws below use the zero-cost shape so their row counts are the WINDOW's
    /// arithmetic alone, never the process-global arena another test in this binary is holding.
    fn history_window_log(rows: usize, revertible: bool) -> HistoryView {
        let entry = move |seq: u64| CommandView {
            seq,
            action_id: "translateSelection".into(),
            label: LocalizedLabel::data(format!("Move {seq}")),
            kind: ActionKind::Mutation,
            timestamp: "0".into(),
            edit_id: Some(format!("edit-{seq}")),
            child_edit_ids: Vec::new(),
            transition_id: None,
            author: None,
            op_lines: Vec::new(),
            op_count: 0,
            applied: true,
            revertible,
            count: 1,
            inverse: None,
            transaction: None,
            mutations: Vec::new(),
        };
        HistoryView { columns: Vec::new(), can_undo: true, can_redo: false, active_alternative_id: None, alternatives: Vec::new(), current_checkpoint_id: None, commands: (1..=rows as u64).map(entry).collect(), command_filter: HistoryCommandFilter::All }
    }

    /// 🧾️ The body as the host reads it — `BuiltChildren` is retained page transport and refuses a
    /// direct serde walk, so the tree is projected node by node.
    fn history_body_json(node: &BuiltNode) -> Value {
        json!({
            "key": node.key.as_str(),
            "component": serde_json::to_value(&node.component).expect("component JSON"),
            "children": node.children.iter().map(history_body_json).collect::<Vec<_>>(),
        })
    }

    fn history_commands_window(panel: &BuiltNode) -> Option<TreeWindow> {
        let Component::TreeSection(props) = &panel.children[1].component else { panic!("expected the Commands TreeSection") };
        props.window
    }

    /// 🪟️ ARTIFACT-TREE-VIRTUALISED-STREAMING: the Commands section is a window, not a page. Its rows
    /// are the live entries themselves (no `UI_BUILT_CHILDREN_MAX`-ary page columns between the section
    /// and its rows), it publishes the FULL filtered row count as `TreeWindow::total` however few rows
    /// it materialises, a cold paint draws `TREE_WINDOW_DEFAULT_ROWS` clamped by the built-children
    /// ceiling, a host request draws exactly its slice keyed by the raw entry id, and no `+N`
    /// continuation row exists at any point.
    #[semio_framework_async_macros::async_test]
    async fn ui_history_panel_windows_command_rows_over_the_live_count() {
        let fixture = history_window_fixture();
        let rows = fixture["commandRowCount"].as_u64().unwrap() as usize;
        let default_rows = fixture["defaultWindowRows"].as_u64().unwrap() as usize;
        assert_eq!(default_rows, TREE_WINDOW_DEFAULT_ROWS as usize, "fixture must pin the live first-paint budget");
        let prefix = fixture["entryKeyPrefix"].as_str().unwrap();
        let history = history_window_log(rows, false);

        let cold = ui_history_panel(&history, None, None, &Default::default(), "ctrl", Locale::En, false, false, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("a log of any length must assemble");
        assert_eq!(serde_json::to_value(&cold.layout).expect("the authored history root layout"), fixture["rootLayout"]);
        assert_eq!(cold.children[1].children.len(), default_rows.min(UI_BUILT_CHILDREN_MAX), "a cold paint materialises one viewport, clamped by the built-children ceiling");
        assert!(cold.children[1].children.iter().all(|row| row.key.as_str().starts_with(prefix)), "rows are the entries themselves, never page columns");
        assert_eq!(history_commands_window(&cold), Some(TreeWindow { row_extent: Default::default(), total: rows as u32, offset: 0 }), "the host sees the whole extent");

        let offset = fixture["requestOffset"].as_u64().unwrap() as u32;
        let requested = fixture["requestRows"].as_u64().unwrap() as u32;
        assert!(requested as usize <= UI_BUILT_CHILDREN_MAX, "fixture slice must fit one built-children page");
        let view = ViewModel {
            tree_windows: vec![TreeWindowRequest {
                body_key: fixture["bodyKey"].as_str().unwrap().to_string(),
                node_key: fixture["nodeKey"].as_str().unwrap().to_string(),
                open: Some(true),
                offset,
                rows: requested,
            }],
            ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
        };
        let scrolled = ui_history_panel(&history, None, None, &Default::default(), "ctrl", Locale::En, false, false, &view).await.expect("a scrolled window must assemble");
        assert_eq!(scrolled.children[1].children.len(), requested as usize);
        assert_eq!(scrolled.children[1].children[0].key.as_str(), format!("{prefix}{}", offset + 1), "the slice starts where the host scrolled to");
        assert_eq!(history_commands_window(&scrolled), Some(TreeWindow { row_extent: Default::default(), total: rows as u32, offset }));

        for panel in [&cold, &scrolled] {
            let json = history_body_json(panel).to_string();
            assert!(!json.contains(".more"), "no continuation row key survives: {json}");
            assert!(!json.contains("\"+"), "no `+N` label survives: {json}");
        }
    }

    /// 🪙️ Every revertible row the window materialises keeps its inline revert, and the window can
    /// never out-spend the process-wide argument arena: a slice is capped by `UI_BUILT_CHILDREN_MAX`,
    /// which the contract prices at `UI_VALUE_PAGE_ROWS` interactive rows. A refusal under live arena
    /// pressure shortens the window — it never faults the render and never drops a row's revert.
    #[semio_framework_async_macros::async_test]
    async fn ui_history_panel_keeps_every_materialised_revert_inside_the_arena_page() {
        let entries = 20usize;
        let history = history_window_log(entries, true);
        let panel = ui_history_panel(&history, None, None, &Default::default(), "ctrl", Locale::En, false, false, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("revertible entries must assemble without an alias refusal");
        fn census(node: &BuiltNode, rows: &mut usize, actions: &mut usize) {
            if node.key.as_str().starts_with("framework.history.entry.") {
                *rows += 1;
                if let Component::TreeItem(props) = &node.component {
                    *actions += props.row_actions.len();
                }
            }
            for child in node.children.iter() {
                census(child, rows, actions);
            }
        }
        let (mut rows, mut actions) = (0usize, 0usize);
        census(&panel, &mut rows, &mut actions);
        assert!(rows > 0 && rows <= entries, "the window never exceeds the live count: {rows}");
        assert_eq!(actions, rows, "every materialised row keeps its inline revert");
        assert!(actions <= UI_VALUE_PAGE_ROWS, "a window can never out-spend the arena page: {actions} > {UI_VALUE_PAGE_ROWS}");
        assert_eq!(history_commands_window(&panel), Some(TreeWindow { row_extent: Default::default(), total: entries as u32, offset: 0 }));
    }

    /// 🙈️ The row predicate: interactions never, views only with an edit or inverse, everything else always.
    #[test]
    fn history_rows_exclude_interactions_and_pure_views() {
        for (kind, edited, inverse, expected) in [
            (ActionKind::Interaction, true, true, false),
            (ActionKind::View, false, false, false),
            (ActionKind::View, true, false, true),
            (ActionKind::View, false, true, true),
            (ActionKind::Mutation, false, false, true),
            (ActionKind::History, false, false, true),
            (ActionKind::Clipboard, false, false, true),
            (ActionKind::Shell, false, false, true),
        ] {
            assert_eq!(history_row_is_recorded(kind, edited, inverse), expected, "{kind:?} edited={edited} inverse={inverse}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn selection_count_phrase_formats_mixed_selection() {
        assert_eq!(selection_count_phrase(semio_framework_ui_locale::Locale::En, &[(8, SelectionKind::Node), (13, SelectionKind::Edge)]).as_deref(), Some("8 nodes and 13 edges"));
        assert_eq!(selection_count_phrase(semio_framework_ui_locale::Locale::En, &[(1, SelectionKind::Node)]).as_deref(), Some("1 node"));
        assert_eq!(selection_count_phrase(semio_framework_ui_locale::Locale::De, &[(8, SelectionKind::Node), (13, SelectionKind::Edge)]).as_deref(), Some("8 Knoten und 13 Kanten"));
    }

    //#region 🗂️GroupedContextMenu
    #[semio_framework_async_macros::async_test]
    async fn action_definition_with_category_sets_the_ribbon_taxonomy_field() {
        let action = ActionDefinition::bounded_catalog("x", LocalizedLabel::data("X"), ActionKind::Mutation).with_category("view");
        assert_eq!(action.category.as_deref(), Some("view"));
    }

    #[semio_framework_async_macros::async_test]
    async fn menu_group_produces_a_group_row_keyed_by_category() {
        let registry = contract_registry().await;
        let items = Menu::of(&registry, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).action("setLabelRequired").group("export", |m| m.command("incrementViaCommand")).build();
        assert_eq!(items.len(), 2);
        assert_eq!(items[1].id, "menu.group.export");
        assert_eq!(items[1].label, None, "group rows travel with no label — the host resolves it via `ribbon_parent_label`");
        let children: Vec<&str> = items[1].children.as_ref().unwrap().iter().map(|child| child.id.as_str()).collect();
        assert_eq!(children, vec!["incrementViaCommand"]);
    }

    #[semio_framework_async_macros::async_test]
    async fn context_menu_wire_requires_and_forwards_canonical_view_state() {
        assert!(serde_json::from_str::<ContextMenuWireRequest>(r#"{"menu":{"id":"window"}}"#).is_err());
        let wire: ContextMenuWireRequest = serde_json::from_str(r#"{"menu":{"id":"window"},"viewState":{"locale":"de","terminology":"reuse"}}"#).expect("canonical viewState parses");
        let (request, view_state) = wire.into_parts().unwrap();
        assert_eq!(request.menu.id, "window");
        assert_eq!(view_state.locale, semio_framework_ui_locale::Locale::De);
        assert_eq!(view_state.terminology, semio_framework_ui_locale::Terminology::Reuse);
        let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🪟️surfaces/🧪️tests/🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
        for surface in fixture["surfaces"].as_array().unwrap() {
            let wire: ContextMenuWireRequest = serde_json::from_value(json!({"menu":{"id":"window"},"windowInstanceId":surface["windowId"],"viewState":fixture["view"]})).unwrap();
            let (_, view) = wire.into_parts().unwrap();
            assert_eq!(view.window_id.as_deref(), surface["windowId"].as_str());
            assert_eq!(view.active_utility_id.as_deref(), surface["activeUtilityId"].as_str());
        }
        let wire: ContextMenuWireRequest = serde_json::from_value(json!({"menu":{"id":"window"},"windowInstanceId":"unknown","viewState":fixture["view"]})).unwrap();
        assert!(wire.into_parts().is_err());
    }

    #[semio_framework_async_macros::async_test]
    async fn surface_context_reaches_real_app_render_and_rejects_hidden_surfaces() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🪟️surfaces/🧪️tests/🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
        let host_view: ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        let app = contract_app_raw(mounted_policy, &mut identity).await;
        let cell = std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id: 7, app, surface_contexts: Default::default() }, runtime.mounted_policy));
        runtime.instances.borrow_mut().insert_admitted(7, cell);
        for surface in fixture["surfaces"].as_array().unwrap() {
            let view = surface["windowId"].as_str().map(|window| host_view.for_window_instance(window).unwrap()).unwrap_or_else(|| host_view.clone());
            semio_framework_plugin_runtime_ops::ops_surface_render::plugin_mount_surface(&runtime, 7, surface["id"].as_str().unwrap().into(), surface["bodyKey"].as_str().unwrap().into(), &semio_framework_plugin_plugin_program::encode_wire_serialized(&view)).await.unwrap();
        }
        for surface in fixture["surfaces"].as_array().unwrap() {
            semio_framework_plugin_runtime_ops::ops_surface_render::plugin_render_surface(&runtime, 7, surface["id"].as_str().unwrap()).await.unwrap();
            let (body, actual) = RENDER_CONTEXT_PROBE.with(|probe| probe.take().unwrap());
            assert_eq!(body, surface["bodyKey"].as_str().unwrap());
            assert_eq!(actual.window_id.as_deref(), surface["windowId"].as_str());
            assert_eq!(actual.active_utility_id.as_deref(), surface["activeUtilityId"].as_str());
            assert_eq!(actual.locale, host_view.locale);
            assert_eq!(actual.terminology, host_view.terminology);
        }
        let hidden = fixture["hidden"].as_str().unwrap();
        semio_framework_plugin_runtime_ops::ops_surface_render::plugin_hide_surface(&runtime, 7, hidden).await.unwrap();
        assert!(semio_framework_plugin_runtime_ops::ops_surface_render::plugin_render_surface(&runtime, 7, hidden).await.is_err());
        assert!(semio_framework_plugin_runtime_ops::ops_surface_render::plugin_render_surface(&runtime, 7, fixture["survivor"].as_str().unwrap()).await.is_ok());
    }
    //#endregion 🗂️GroupedContextMenu

    #[semio_framework_async_macros::async_test]
    async fn surface_context_refresh_projects_panels_from_focused_window_state() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🪟️surfaces/🧪️tests/🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
        let host_view: ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
        let focused = host_view.for_window_instance("right").unwrap();
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        let app = contract_app_raw(mounted_policy, &mut identity).await;
        runtime.instances.borrow_mut().insert_admitted(7, std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id: 7, app, surface_contexts: Default::default() }, runtime.mounted_policy)));
        let response = semio_framework_plugin_runtime_ops::ops_surface_render::plugin_refresh_ui(&runtime, 7, &serde_json::to_string(&json!({"viewState":focused,"panels":[{"key":"properties","bodyKey":"properties"}]})).unwrap()).await.unwrap();
        let response: Value = serde_json::from_str(&response).unwrap();
        assert_eq!(response["panels"][0]["key"], "properties");
        let (body, actual) = RENDER_CONTEXT_PROBE.with(|probe| probe.take().unwrap());
        assert_eq!(body, "properties");
        assert_eq!(serde_json::to_value(actual).unwrap(), serde_json::to_value(host_view.for_panel()).unwrap());
    }



    //#region 🚚️World3dSceneLaneCarriers












    //#endregion 🚚️World3dSceneLaneCarriers

    #[semio_framework_async_macros::async_test]
    async fn surface_context_presence_targets_each_concrete_surface() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🪟️surfaces/🧪️tests/🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
        let host_view: ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
        let mut app = interaction_app_raw(mounted_policy, &mut identity).await;
        let mut peers = PeerPresenceRoot::empty();
        peers.insert(fixture["presence"]["actor"].as_str().unwrap().into(), PeerPresence {
            color: Some(3), surface: None,
            interaction: Some(PresenceInteraction {
                app_id: "s.test.synthetic@1/*#editor".into(),
                domains: vec![PresenceDomain { domain: fixture["presence"]["domainId"].as_str().unwrap().into(), granularity: "item".into(), selected: vec![fixture["presence"]["nodeKey"].as_str().unwrap().into()], hovered: Vec::new() }],
            }),
        }).unwrap();
        *app.roster.peer_presence = std::sync::Arc::new(peers);
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        runtime.instances.borrow_mut().insert_admitted(7, std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id: 7, app, surface_contexts: Default::default() }, runtime.mounted_policy)));
        for surface in fixture["surfaces"].as_array().unwrap() {
            let id = surface["id"].as_str().unwrap();
            let view = surface["windowId"].as_str().map(|window| host_view.for_window_instance(window).unwrap()).unwrap_or_else(|| host_view.for_panel());
            semio_framework_plugin_runtime_ops::ops_surface_render::plugin_mount_surface(&runtime, 7, id.into(), surface["bodyKey"].as_str().unwrap().into(), &semio_framework_plugin_plugin_program::encode_wire_serialized(&view)).await.unwrap();
            let (_, presence) = semio_framework_plugin_runtime_ops::ops_surface_render::plugin_render_surface(&runtime, 7, id).await.unwrap();
            assert_eq!(presence.len(), 1);
            let mut expected = fixture["presence"]["expected"].clone();
            expected["surface"] = surface["id"].clone();
            assert_eq!(serde_json::to_value(&presence[0]).unwrap(), expected);
        }
    }

    /// 🎯️ M1 decision: `ActionId.version` mismatch returns a `Fault`, never silently dispatches a
    /// stale contract — the default `command_from_intent` bridge only resolves version 1.
    #[semio_framework_async_macros::async_test]
    async fn command_from_intent_rejects_a_non_v1_action_version() {
        let intent = UiIntent {
            surface: SurfaceId::try_from("s").expect("bounded fixture"),
            revision: UiRevision(0),
            node: UiNodeId::default(),
            node_key: UiText::try_from_str("counter").expect("bounded fixture"),
            trigger: Trigger::Activate,
            action: ActionId::new(UiText::try_from_str("app").expect("bounded fixture"), UiText::try_from_str("incrementViaCommand").expect("bounded fixture"), 2),
            args: None,
            input: None,
            seq: 3,
        };
        let error = TestApp::<false>::command_from_intent(&intent).await.expect_err("a non-v1 action must be rejected, never silently dispatched");
        assert!(error.message.contains("version"), "unexpected error: {}", error.message);
    }

    #[semio_framework_async_macros::async_test]
    async fn ui_dispatch_backstop_rejects_every_non_migrated_action_and_command() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        use semio_framework::InteractiveJobClassification::{BatchOnlyPendingRewrite, Deleted, ForbiddenFromUi, Unclassified};
        use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};

        for classification in [Unclassified, BatchOnlyPendingRewrite, ForbiddenFromUi, Deleted] {
            let mut action_registry = contract_registry().await;
            action_registry.test_set_action_classification("badView", classification);
            let mut action_app = VcsArtifactApp::<TestApp<false, TEST_APP_TOOLS_NONE>>::with_registry(TestApp::<false, TEST_APP_TOOLS_NONE>::default(), action_registry, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
            semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut action_app, mounted_policy);
            assert!(!action_app.test_registered_tool_keys().iter().any(|key| key.1 == "badView"));
            let action_error = action_app.dispatch_typed(TestCommand::BadView, &meta(), &mut identity).await.expect_err("non-migrated action must be rejected before its handler runs");
            assert_eq!(action_error.code.0, "interactive-job.not-ui-safe");
            assert_eq!(action_app.test_snapshot().await.count, 0);

            let mut command_registry = contract_registry().await;
            command_registry.test_set_app_command_classification("incrementViaCommand", classification);
            let mut command_app = VcsArtifactApp::<TestApp<false, TEST_APP_TOOLS_NONE>>::with_registry(TestApp::<false, TEST_APP_TOOLS_NONE>::default(), command_registry, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
            semio_framework_plugin_testing_laws::artifact_app_laws::settle_store_constructor(&mut command_app, mounted_policy);
            assert!(!command_app.test_registered_tool_keys().iter().any(|key| key.1 == "incrementViaCommand"));
            let mut invocation =
                CommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::App { plugin_id: "test".into(), app_id: TestApp::<false>::APP_ID.into() }, command_id: "incrementViaCommand".into() }, arguments: Default::default() };
            let command_error = command_app.handle_command(&mut invocation, None, Some("edit"), &meta(), &mut identity).await.expect_err("non-migrated command must be rejected before its handler runs");
            assert_eq!(command_error.code.0, "interactive-job.not-ui-safe");
            assert_eq!(command_app.test_snapshot().await.count, 0);
            artifact_app_laws::close_registered_fixture_app(&mut action_app, mounted_policy,cx).expect("original contract fixture paid terminal close");
            artifact_app_laws::close_registered_fixture_app(&mut command_app, mounted_policy,cx).expect("original contract fixture paid terminal close");
        }
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn plugin_command_handler_is_program_owned_across_app_instances() {
        use semio_framework::kernel::{InvocationId, InvocationResult, UndoGroup};
        use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};
        use std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        };
        let calls = Arc::new(AtomicUsize::new(0));
        let handler_calls = calls.clone();
        let plugin: Plugin = Plugin::new("fixture", "Fixture", "0.1.0").plugin_command(
            CommandDefinition::bounded_catalog("refresh", LocalizedLabel::data("Refresh"), "plugin", ActionKind::Shell),
            Box::new(move |_, meta| {
                handler_calls.fetch_add(meta.instance_id as usize, Ordering::SeqCst);
                Ok(InvocationResult {
                    output: semio_framework_value::DslValue::Null,
                    mutations: Vec::new(),
                    inverse_group: UndoGroup { invocation_id: InvocationId(String::new()), mutations: Vec::new(), inverse_mutations: Vec::new(), member_edits: Vec::new() },
                    diagnostics: Vec::new(),
                    requested_effects: Vec::new(),
                    events: Vec::new(),
                    ui_scope: UiDirtyScope::None,
                    history_patch: None,
                })
            }),
        );
        let mut invocation = CommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::Plugin { plugin_id: "fixture".into() }, command_id: "refresh".into() }, arguments: Default::default() };
        plugin.handle_plugin_command(&invocation, &ActionMeta { actor: "a".into(), instance_id: 1, view_state: None }).expect("first app instance");
        plugin.handle_plugin_command(&invocation, &ActionMeta { actor: "b".into(), instance_id: 2, view_state: None }).expect("second app instance");
        assert_eq!(calls.load(Ordering::SeqCst), 3);
    }

    #[semio_framework_async_macros::async_test]
    async fn registry_less_construction_rejects_before_the_reducer() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32_768,maximum_capacity_bytes:262_144,maximum_release_bytes:1_048_576,maximum_depth:4_096};
        let mounted_policy=semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:mounted_grant,maintenance:mounted_grant,close:mounted_grant};
        let identity_started=std::time::Instant::now();
        let mut identity_progress=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{progress.owned_bytes<=1_048_576&&identity_started.elapsed()<std::time::Duration::from_secs(60)};
        let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = VcsArtifactApp::<TestApp<false, TEST_APP_TOOLS_FACTORIES>>::new(TestApp::<false, TEST_APP_TOOLS_FACTORIES>::default(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
        let error = app.dispatch_typed(TestCommand::BadView, &meta(), &mut identity).await.expect_err("an empty registry must fail closed");
        assert_eq!(error.code.0, "interactive-job.unknown-key");
        assert_eq!(app.stores.test_snapshot().await, TestSnapshot::default());
        artifact_app_laws::close_registered_fixture_app(&mut app, mounted_policy,cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn build_definition_rejects_transitive_flat_interaction() {
        let __base = App::builder("bad-transitive-flat", LocalizedLabel::data("Bad"))
            .await
            .document(["state"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .window_kind("main", LocalizedLabel::data("Main"), "bad.main", SurfaceKind::Canvas2d, IconName::AppWindow)
            .await;
        let outcome = std::panic::catch_unwind(move || {
            let builder = ::semio_framework_async::poll::resolve_ready(__base.interaction(InteractionDefinition {
                id: "items".into(),
                label: LocalizedLabel::data("Items"),
                granularities: vec![GranularityDefinition { id: "item".into(), label: LocalizedLabel::data("Item"), icon_id: IconName::AppWindow }],
                hierarchy: HierarchyProvider::Flat,
                hover: HoverSpec { transitive: true, ..HoverSpec::default() },
                selection: SelectionSpec { modes: vec![SelectionMode::Single], methods: vec![SelectionMethod::Pick], merges: vec![MergeMode::Replace], transitive: false, broadcast: true },
            }));
            ::semio_framework_async::poll::resolve_ready(App::from_builder(builder))
        });
        assert!(outcome.is_err(), "build_definition must reject transitive hover paired with HierarchyProvider::Flat");
    }

    /// 🌀️ The widest, always-correct answer is what is left when there is nothing to derive from: no
    /// window kind declares the touched domain, so the framework may not guess which body paints it.
    #[semio_framework_async_macros::async_test]
    async fn a_domain_no_window_declares_keeps_the_framework_full_scope() {
        let definition = interaction_app_definition().await;
        let windows = semio_framework::interaction_window_bodies_by_domain(&definition);
        let panels = semio_framework::panel_leaf_body_keys(&definition);
        for verb in InteractionVerb::ALL {
            assert!(semio_framework::interaction_declared_refresh_scope(verb, &["nowhere"], &windows, &panels).is_none(), "{verb:?} narrowed a domain no window declares");
            assert!(semio_framework::interaction_declared_refresh_scope(verb, &[], &windows, &panels).is_none(), "{verb:?} narrowed a verb that touched no domain");
        }
        assert_eq!(windows.get("items").map(Vec::as_slice), Some([TEST_APP_WINDOW_BODY_KEY.to_string()].as_slice()), "the derivation reads the window kind's own interaction refs");
    }

    /// 🎮️ Wave B34 — the verb type the declaration is keyed by is exactly the six ids
    /// `INTERACTION_ACTION_IDS` intercepts, in both directions, so a seventh verb cannot be dispatched
    /// without an author deciding what it dirties.
    #[semio_framework_async_macros::async_test]
    async fn every_intercepted_interaction_id_round_trips_through_its_verb() {
        for verb in InteractionVerb::ALL {
            assert_eq!(InteractionVerb::of_action(verb.action_id()), Some(verb), "{verb:?}");
        }
        assert_eq!(InteractionVerb::ALL.len(), semio_framework_plugin_interaction::interaction_topology::INTERACTION_ACTION_IDS.len());
        for action in semio_framework_plugin_interaction::interaction_topology::INTERACTION_ACTION_IDS {
            assert!(InteractionVerb::of_action(action).is_some(), "{action} is intercepted but names no verb");
        }
        assert_eq!(InteractionVerb::of_action("undo"), None);
    }

    /// 🕹️ Ticket 26/09/16/INPUT-CAUSALITY-LEDGER §2 C — the leftover a typed operation's folded verb
    /// owes the host rides the `Invocation` reply of the command that STARTED that operation
    /// (`merge_typed_operation_leftover_into_reply`, run by `plugin_exchange` after its typed pump):
    /// the reply's `{ operationId, generation }` witness keeps both keys and gains `interactionView`,
    /// the envelope `interactionViewFromLeftoverOutput` peels from `response.output`. A reply that
    /// answers another operation, an unsolicited (`in_reply_to: 0`) frame and a non-`Invocation`
    /// frame are left byte-identical. Fails-before: the typed ladder dropped `fold.leftover`, so the
    /// host's leftover overlay / Inspection tab never moved for a folded pick.
    #[test]
    fn typed_operation_leftover_merges_into_exactly_the_carrying_reply() {
        let started = |operation: &str| store::pack_rt::encode_wire_value(&semio_framework_value::DslValue::Object(vec![("operationId".into(), semio_framework_value::DslValue::String(operation.into())), ("generation".into(), semio_framework_value::DslValue::String("7".into()))]));
        let invocation = |in_reply_to: u64, output: Vec<u8>| protocol::AppFrame::Invocation { in_reply_to, output, diagnostics: Vec::new(), ui_scope: Vec::new(), history_patch: Vec::new(), messages: Vec::new(), mutations: Vec::new(), inverse_group: Vec::new() };
        let view = semio_framework_value::DslValue::Object(vec![("interactionView".into(), semio_framework_value::DslValue::Object(vec![("selectedIds".into(), semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::String("item-1".into())]))]))]);
        let leftover = semio_framework_plugin_job::TypedOperationLeftover { operation: 42, view };
        let mut frames = vec![protocol::AppFrame::Done { in_reply_to: 1 }, invocation(2, started("41")), invocation(0, started("42")), invocation(3, started("42"))];
        let untouched: Vec<Vec<u8>> = frames.iter().take(3).map(|frame| ::semio_framework_async::poll::resolve_ready(protocol::encode_app_frame(frame))).collect();
        assert!(semio_framework_plugin_runtime_ops::ops_typed_operations::merge_typed_operation_leftover_into_reply(&mut frames, &leftover), "the reply that started operation 42 receives the leftover");
        for (index, before) in untouched.iter().enumerate() {
            assert_eq!(&::semio_framework_async::poll::resolve_ready(protocol::encode_app_frame(&frames[index])), before, "frame {index} answers no fold and stays byte-identical");
        }
        let protocol::AppFrame::Invocation { in_reply_to: 3, output, .. } = &frames[3] else { panic!("the carrying reply keeps its sequence") };
        let merged = store::pack_rt::decode_wire_value(output).expect("merged output decodes");
        assert_eq!(merged.get("operationId").and_then(semio_framework_value::DslValue::as_str), Some("42"), "the operation witness the host awaits survives: {merged:?}");
        assert_eq!(merged.get("generation").and_then(semio_framework_value::DslValue::as_str), Some("7"));
        let ids = merged.get("interactionView").and_then(|view| view.get("selectedIds")).and_then(semio_framework_value::DslValue::as_array).expect("interactionView joins the witness");
        assert_eq!(ids.iter().filter_map(semio_framework_value::DslValue::as_str).collect::<Vec<_>>(), vec!["item-1"]);
        let mut none = vec![invocation(4, started("40"))];
        assert!(!semio_framework_plugin_runtime_ops::ops_typed_operations::merge_typed_operation_leftover_into_reply(&mut none, &leftover), "no reply of this exchange started the folding operation");
    }

    /// 🧵️ Defers the original task factory until the actor polls its admitted future.
    #[semio_framework_async_macros::async_test]
    async fn task_admission_defers_the_user_factory_until_actor_execution() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../⚛️reactor/🧵️task/🧪️tests/🧫️fixtures/📥️admission/🔣️.json")).expect("canonical task admission fixture");
        let expected = |phase: &str| fixture["observations"].as_array().expect("observations").iter().find(|row| row["phase"] == phase).expect("phase")["invokesTaskFactory"].as_bool().expect("factory invocation");
        let calls = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let factory_calls = calls.clone();
        let instance = 510;
        let meta = ActionMeta { actor: "factory-owner".into(), instance_id: instance, view_state: None };
        let task = test_task("deferred-factory", move |_ctx| {
            factory_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            async move { Ok(TaskResolution::Done) }
        });
        semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(instance, &meta, task).expect("original task admitted");
        assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), usize::from(expected("admission")));
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_count_for_instance(instance).await, 1);
        semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::run_until_idle(8).await;
        assert_eq!(calls.load(std::sync::atomic::Ordering::SeqCst), usize::from(expected("execution")));
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_count_for_instance(instance).await, 0);
    }

    impl<F, Fut> AsyncTaskSource<TestMutation, TestConfigMutation, NoDraftMutation> for TestTaskSource<F>
    where
        F: FnOnce(TaskCtx) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<TaskResolution<TestMutation, TestConfigMutation, NoDraftMutation>, Fault>> + 'static,
    {
        type Future = Fut;
        fn run(self, context: TaskCtx) -> Fut { (self.0)(context) }
    }

    /// 🚫️ The (quota+1)th task on one instance is refused with a typed `Fault` — never a
    /// silent drop — while earlier tasks and OTHER instances are unaffected.
    ///
    /// 🔓️ A different instance has its OWN quota accounting, unaffected by 502's exhaustion.
    #[semio_framework_async_macros::async_test]
    async fn spawn_task_quota_gate_faults_the_n_plus_1th_task_and_never_silently_drops_it() {
        let instance = 502;
        let meta = ActionMeta { actor: "local".into(), instance_id: instance, view_state: None };
        semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::set_instance_quota(instance, 2).await;

        for label in ["first", "second"] {
            let task = test_task(label, |_ctx| async move { Ok(TaskResolution::Done) });
            semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(instance, &meta, task).unwrap_or_else(|error| panic!("task '{label}' must be admitted under quota 2: {error:?}"));
        }
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_count_for_instance(instance).await, 2);

        let third = test_task("third", |_ctx| async move { Ok(TaskResolution::Done) });
        let error = semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(instance, &meta, third).expect_err("the 3rd task must be refused — quota is 2, not a silent drop");
        assert_eq!(error.code.0, "plugin.task.quota-exceeded");
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_count_for_instance(instance).await, 2, "a refused spawn must not have added a 3rd record");

        let other_instance = 503;
        let other_meta = ActionMeta { actor: "local".into(), instance_id: other_instance, view_state: None };
        let task = test_task("elsewhere", |_ctx| async move { Ok(TaskResolution::Done) });
        semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(other_instance, &other_meta, task).expect("a different instance must not be affected by 502's quota exhaustion");
        semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::run_until_idle(8).await;
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_count_for_instance(instance).await, 0, "the admitted tasks retire their owners before the thread ends");
    }

    /// 🔑️ Spawning a second task under the SAME `(instance, key)` cancels the first — its
    /// future is dropped (never runs to completion, never queues a resume) — and the dedupe
    /// index tracks only the NEW task afterward.
    #[semio_framework_async_macros::async_test]
    async fn key_dedupe_cancels_the_previously_live_task_under_the_same_key() {
        let instance = 504;
        let meta = ActionMeta { actor: "local".into(), instance_id: instance, view_state: None };
        let first_ran = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let first_ran_inner = first_ran.clone();
        let first = test_task("first", move |_ctx| async move {
            first_ran_inner.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(TaskResolution::Done)
        })
        .keyed("search");
        semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(instance, &meta, first).expect("first must be admitted");
        assert!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_key_is_live(instance, "search").await);
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_count_for_instance(instance).await, 1);

        let second_ran = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let second_ran_inner = second_ran.clone();
        let second = test_task("second", move |_ctx| async move {
            second_ran_inner.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(TaskResolution::Done)
        })
        .keyed("search");
        semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(instance, &meta, second).expect("second must be admitted, cancelling the first");
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_count_for_instance(instance).await, 1, "the SAME key must never have two live tasks at once");

        semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::run_until_idle(8).await;
        assert!(!first_ran.load(std::sync::atomic::Ordering::SeqCst), "the cancelled first task must never have run its body");
        assert!(second_ran.load(std::sync::atomic::Ordering::SeqCst), "the surviving second task must have run");
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_count_for_instance(instance).await, 0, "the second task completed with TaskResolution::Done — no follow-up, cleaned up");
    }

    /// 🚫️ `Event::InstanceClose` cancellation: every task an instance owns is dropped from the
    /// executor (never runs to completion) and its pending `RequestRegistry` slot is gone too
    /// — no leaked slot — while a DIFFERENT instance's live task/request is untouched.
    ///
    /// parks forever in this test
    ///
    /// 🚫️ `cancel_instance_tasks` alone drops the future (and its parked `RequestFuture`
    /// with it); `RequestRegistry::cancel_instance` is the defense-in-depth sweep `poll`'s
    /// `Event::InstanceClose` arm runs right after — exercised here to prove BOTH steps
    /// together leave no slot behind, matching that call site exactly.
    #[semio_framework_async_macros::async_test]
    async fn instance_close_cancellation_drops_the_instances_tasks_and_leaks_no_registry_slot() {
        let dying = 505;
        let survivor = 506;
        let dying_meta = ActionMeta { actor: "local".into(), instance_id: dying, view_state: None };
        let survivor_meta = ActionMeta { actor: "local".into(), instance_id: survivor, view_state: None };

        let dying_ran = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let dying_ran_inner = dying_ran.clone();
        let dying_task = test_task("dying", move |ctx: TaskCtx| async move {
            let _ = ctx.host.storage_read("never-resolved").await;
            dying_ran_inner.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(TaskResolution::Done)
        });
        semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(dying, &dying_meta, dying_task).expect("dying instance's task must spawn");

        let survivor_task = test_task("survivor", move |ctx: TaskCtx| async move {
            let _ = ctx.host.storage_read("also-never-resolved").await;
            Ok(TaskResolution::Done)
        });
        semio_framework_plugin_reactor_tasks::tasks_task_lane::spawn_task(survivor, &survivor_meta, survivor_task).expect("survivor instance's task must spawn");

        let pending = semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::run_until_idle(8).await;
        assert!(pending, "both tasks must have parked on their own storage_read");
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::pending_request_count().await, 2, "one RequestRegistry slot per parked task");

        semio_framework_plugin_reactor_tasks::tasks_task_lane::cancel_instance_tasks(dying);
        let removed = semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::cancel_instance_registry_requests(dying);
        assert_eq!(removed.await, 0, "cancel_instance_tasks already dropped the task's own RequestFuture — nothing left for the registry sweep to remove");

        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_count_for_instance(dying).await, 0, "the dying instance's task record must be gone");
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::pending_request_count().await, 1, "only the SURVIVOR's request may remain pending");
        assert_eq!(semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::task_count_for_instance(survivor).await, 1, "the survivor's task must be untouched");

        let still_pending = semio_framework_plugin_tests_03::reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::run_until_idle(8).await;
        assert!(still_pending, "the survivor's task is still legitimately parked");
        assert!(!dying_ran.load(std::sync::atomic::Ordering::SeqCst), "the cancelled task must never observe its await resolving, because it never runs again");
    }

    #[test]
    fn checkpoint_restart_mode_requires_its_exact_concrete_factory_owner() {
        let fixture: Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/⏳️completion/🔣️.json"))).unwrap();
        assert_eq!(<TestCommand as ::protocol::OpBinary>::TOOL_JOB_IDS, fixture["restartAuthority"]["generatedToolIds"].as_array().unwrap().iter().map(|id| id.as_str().unwrap()).collect::<Vec<_>>());
        assert_eq!(TestApp::<false>::bounded_first_step_tool_proofs().len() as u64, fixture["restartAuthority"]["defaultProofs"].as_u64().unwrap());
        assert_eq!(TestApp::<true>::bounded_first_step_tool_proofs().len() as u64, fixture["restartAuthority"]["retainedProofs"].as_u64().unwrap());
        assert_ne!(ToolOwnerWitness::of::<TestApp<false>>(), ToolOwnerWitness::of::<TestApp<true>>());
        assert_eq!(std::any::TypeId::of::<<TestRestartFactory<true, 2> as ArtifactOwnedToolJobFactory>::Owner>(), std::any::TypeId::of::<TestApp<true>>());
        assert_eq!(<TestRestartFactory<true, 2> as ArtifactOwnedToolJobFactory>::TOOL_IDS, &[fixture["restartAuthority"]["tool"].as_str().unwrap()]);
    }
    #[semio_framework_async_macros::async_test]
    async fn mounted_command_history_original_pages_retain_exact_app_close_authority() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = Box::pin(contract_app_raw(mounted_policy, &mut identity)).await;
        semio_framework_plugin_tests_02::child_test_hosts::child_unit_2::test_mounted_command_history_pages(&mut app);
    }
    #[semio_framework_async_macros::async_test]
    async fn mounted_command_history_document_replacement_keeps_original_held_rows() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = Box::pin(contract_app_raw(mounted_policy, &mut identity)).await;
        app.stores.test_store_mut().await.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations: vec![TestMutation::SetCount(SetCount { value: 1 })], transaction: None }, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.unwrap();
        semio_framework_plugin_tests_02::child_test_hosts::child_unit_2::test_mounted_command_history_replacement(&mut app);
    }
    #[semio_framework_async_macros::async_test]
    async fn mounted_private_child_frame_retains_exact_admission_and_close_authority() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = Box::pin(contract_app_raw(mounted_policy, &mut identity)).await;
        semio_framework_plugin_tests_02::publication_mounted_test_hosts::publication_mounted_unit_1::test_mounted_private_child_frame(&mut app);
        semio_framework_plugin_testing_laws::artifact_app_laws::close_registered_fixture_app(&mut app, semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy(),cx).expect("original contract fixture paid terminal close");
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

/// 👷️ Original reserved route returns physical backing separately from logical payload work.
#[test]
fn original_reserved_job_currency_law_retains_spare_backing_until_exact_release(){
 use semio_framework_plugin_job::reserved_jobs::FrameworkSetSelectionModeJob;
 use semio_framework_job::{InteractiveJob,InteractiveJobCloseStep};
 use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/👷️reserved/🔣️.json")).unwrap();
 let source:RetainedCloneGrant=serde_json::from_value(law["caller"]["source"].clone()).unwrap();let mut caller=semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(law["caller"]["operation"].as_u64().unwrap()),semio_framework_job::Generation(law["caller"]["generation"].as_u64().unwrap()),source,||Some(1000),law["caller"]["deadline"].as_u64().unwrap(),law["caller"]["fuel"].as_u64().unwrap());assert_eq!(law["caller"]["clock"],1000);
 let (raw,birth)=observe(||{let mut raw=Vec::with_capacity(law["backingBytes"].as_u64().unwrap()as usize);raw.resize(law["payloadBytes"].as_u64().unwrap()as usize,0x72);raw});
 let pointer=raw.as_ptr();let capacity=raw.capacity();assert_eq!((birth.requested_bytes,birth.released_bytes),(capacity,0));
 let mut job=FrameworkSetSelectionModeJob::new(raw,1);job.begin_close();
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:capacity,maximum_depth:1};
 for refused in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant}]{let(step,event)=observe(||caller.turn(refused,|cx|job.close_step(cx)).unwrap());assert_eq!(step,InteractiveJobCloseStep::Pending{progress:Default::default()});assert_eq!((event.requested_bytes,event.released_bytes),(0,0));assert_eq!(job.raw.as_ptr(),pointer);assert_eq!(job.raw.len(),8193);}
 let(step,event)=observe(||caller.turn(RetainedCloneGrant{maximum_depth:0,..grant},|cx|job.close_step(cx)));assert!(matches!(step,Err(ref failure) if failure.primary_value().is_some_and(|cause|cause.kind==semio_framework_value::ValueRefusalKind::DepthLimit)));assert_eq!((event.requested_bytes,event.released_bytes),(0,0));assert_eq!(job.raw.as_ptr(),pointer);
 for expected in law["turns"].as_array().unwrap().iter().take(3){let(step,event)=observe(||caller.turn(grant,|cx|job.close_step(cx)).unwrap());let progress=step.progress();assert_eq!(progress.copied_items,expected["copiedItems"].as_u64().unwrap()as usize);assert_eq!(progress.copied_bytes,expected["copiedBytes"].as_u64().unwrap()as usize);assert_eq!(progress.retained_capacity_bytes,0);assert_eq!(progress.released_bytes,0);assert_eq!((event.requested_bytes,event.released_bytes),(0,0));assert_eq!(job.raw.capacity(),capacity);assert!(!job.terminal_is_empty());}
 let(step,event)=observe(||caller.turn(RetainedCloneGrant{maximum_release_bytes:capacity-1,..grant},|cx|job.close_step(cx)).unwrap());assert_eq!(step,InteractiveJobCloseStep::Pending{progress:Default::default()});assert_eq!((event.requested_bytes,event.released_bytes),(0,0));assert_eq!(job.raw.as_ptr(),pointer);assert_eq!(job.raw.capacity(),capacity);
 let(step,event)=observe(||caller.turn(grant,|cx|job.close_step(cx)).unwrap());assert_eq!(step,InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()}});assert_eq!((event.requested_bytes,event.released_bytes),(0,capacity));assert_eq!(job.raw.capacity(),0);assert!(job.terminal_is_empty());
 let(step,event)=observe(||caller.turn(grant,|cx|job.close_step(cx)).unwrap());assert_eq!(step,InteractiveJobCloseStep::Complete{progress:Default::default()});assert_eq!((event.requested_bytes,event.released_bytes),(0,0));
 caller.retire(source);
}