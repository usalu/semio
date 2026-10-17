//! 🔬️ Part 1 of 7 of the plugin builder contract laws (cut from `🔬️plugin-runtime-plugin-builder-contract`; the fixtures several parts use are the support modules).
//! 🧪️ The plugin contract's own unit test: a `TestApp` implementing the pure `ArtifactApp`
//! surface (B1), wrapped in `VcsArtifactApp`, exercising typed operations with true inverses, config
//! operations that emit no document operations, history interception, and remote-operation ingest
//! idempotency. `TestCommand` is `TestApp`'s typed `Self::Command`; framework-reserved verbs
//! (history/clipboard/revert/filter/noteShellCommand) still dispatch by string via `handle_action`/
//! `handle_command` — everything app-specific dispatches via `dispatch_typed`.


use semio_framework_plugin_document_media::ActiveArtifactStoreReplacement;
use semio_framework_plugin_document_media::ActiveArtifactStoreReplacementState;
use semio_framework_plugin_plugin_program::AppInstance;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationHandle;
use semio_framework_plugin_document_media::ArtifactEnvelopeDecodeOperationPoll;
use store::ArtifactStoreInitializationAuthority;
use semio_framework_ui_runtime::ComponentTree;
use protocol::LOCAL_ACTOR_ID;
use semio_framework_plugin_instance_ladders::ladders_plugin_app_impl::MAINTENANCE_STAGES;
use semio_framework_plugin_decl_views::child_view::MemberKeyRef;
use semio_framework_plugin_decl_views::NoConfigMutation;
use semio_framework_plugin_child::OwnedDocumentMemberIngress;
use semio_framework_plugin_builder::Ready;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_value::RetirementDemand;
use semio_framework_plugin_interaction::UiAssemblyResult;
use serde_json::Value;
use semio_framework_value::ValueError;
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_window_transient::built_text_to_component_tree;
use semio_framework_plugin_plugin_program::encode_wire_serialized;
use semio_framework_os_kernel as protocol;
use semio_framework_os_kernel as store;
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
use semio_framework_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::{RENDER_CONTEXT_PROBE, TestApp, app_maintenance_turn, close_member_admission_app, close_member_admission_fixture, contract_app_raw, meta, test_child_dialect};
use semio_framework_ui_contract::*;
use semio_framework::*;
use semio_framework::kernel::*;
use semio_framework_plugin_reactor_app::__semio_dispatch_PluginApp;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_decl_views::{ArtifactView, ConfigView, DraftView, InteractionView, NoDraft, NoDraftMutation};
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_abi::{ArtifactDeserializer, ArtifactSerializer};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::PublicationPresence;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::presence::mutations::PublicationPresenceMutation;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::transient::PublicationTransient;
use semio_framework_plugin_testing_fixtures_publication::publication_fixture::transient::mutations::PublicationTransientMutation;
use semio_framework_diagnostic::FaultFrom;
use semio_framework::IconName;
use semio_framework::ViewModel;
use semio_framework_os_kernel::Mutation;
use semio_framework::Fault;
use semio_framework_job::InteractiveJob as _;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;
use semio_framework_artifact_reference::ArtifactRef;
use semio_framework_2d::compute::EngineHandles;
use pack::snapshot::ArtifactPack;
use semio_framework_os_kernel::os_store::backbone::Backbone;
use semio_framework_os_kernel::{MemberFactory, SpaceMember};
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::TestConfig;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
include!("../🔬️plugin-runtime-plugin-builder-contract-part-1-x/🦀️.rs");


    #[semio_framework_async_macros::async_test]
    async fn member_factory_closed_dialect_open_failure_retains_pin_and_drains_exact_member() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{
        let mounted_grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32_768,maximum_capacity_bytes:262_144,maximum_release_bytes:1_048_576,maximum_depth:4_096};
        let mounted_policy=semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:mounted_grant,maintenance:mounted_grant,close:mounted_grant};
        let identity_started=std::time::Instant::now();
        let mut identity_progress=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{progress.owned_bytes<=1_048_576&&identity_started.elapsed()<std::time::Duration::from_secs(60)};
        let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = VcsArtifactApp::<ComposedParentApp, TestMembers>::new(ComposedParentApp::default(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
        let dialect = test_child_dialect().await;
        let expected = ArtifactRef { artifact_id: "child-1".into(), dialect: dialect.clone() };
        app.child_composition.pending_child_pins.push(vcs::CompositionPin { child_ref: expected.clone(), checkpoint_id: "missing-checkpoint".into() });
        let parent = ArtifactRef { artifact_id: app.stores.store.envelope().id.clone(), dialect: ComposedParentApp::<true>::DIALECT.into() };
        assert_eq!(app.stores.store.envelope().dialect.as_ref(), Some(&parent.dialect));
        assert!(app.open_child("slot", "child-1", dialect.clone(), &[], &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.is_err());
        assert!(app.child_composition.test_child_admission_state(1).children_empty && app.child_composition.test_child_admission_state(1).content_empty && app.child_composition.test_child_admission_state(1).abort_empty);
        assert_eq!(app.child_composition.test_child_admission_state(1).generation, 0);
        let mut persisted = TestMembers::create("child-1", &dialect, &TestSnapshot::default().encode_pack(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.unwrap();
        persisted.set_owner(Some(store::OwnerRef { parent, slot: "slot".into(), child_id: "child-1".into() })).await;
        let packed = persisted.envelope_pack_bytes().await.expect("persist exact owned child");
        close_member_admission_fixture(&mut persisted);
        assert!(app.open_child("slot", "child-1", dialect, &packed, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.is_err());
        assert!(app.child_composition.test_child_admission_state(1).children_empty && app.child_composition.test_child_admission_state(1).content_empty && app.child_composition.test_child_admission_state(1).roots_retiring_empty);
        assert_eq!(app.child_composition.test_child_admission_state(1).generation, 0);
        assert_eq!(app.child_composition.pending_child_pins.len(), 1);
        assert_eq!(app.child_composition.pending_child_pins[0].checkpoint_id, "missing-checkpoint");
        assert!(app.child_composition.composition.graph_mut().await.owner_of("child-1").await.is_none());
        assert_eq!(app.child_composition.test_child_admission_state(1).abort_generation, 1);
        assert!(app.child_composition.test_child_admission_state(1).requested_abort_retained);
        for _ in 0..65_536 {
            app.maintenance.maintenance_stage = 20;
            match app_maintenance_turn(&mut app,cx).expect("live failed-member cleanup") {
                semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(_) | semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(_) => {}
                semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::AwaitingInput { reason } => panic!("failed member cleanup awaited input: {reason}"),
                semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Blocked { reason } => panic!("failed member must have a real cleanup owner: {reason}"),
            }
            if app.child_composition.test_child_admission_state(1).abort_empty {
                break;
            }
        }
        assert!(app.child_composition.test_child_admission_state(1).abort_empty);
        app.maintenance.maintenance_stage = 20;
        assert!(matches!(app_maintenance_turn(&mut app,cx).unwrap(), semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(_)));
        close_member_admission_app(&mut app,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn member_factory_closed_dialect_fresh_register_and_restore_publish_exact_parent_owner() {
        let mut original_caller=semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().expect("original builder fixture caller admission");let mut original_context=original_caller.context();let cx=&mut original_context;let result=async{

        let mounted_grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32_768,maximum_capacity_bytes:262_144,maximum_release_bytes:1_048_576,maximum_depth:4_096};
        let mounted_policy=semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1{preparation:mounted_grant,maintenance:mounted_grant,close:mounted_grant};
        let identity_started=std::time::Instant::now();
        let mut identity_progress=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{progress.owned_bytes<=1_048_576&&identity_started.elapsed()<std::time::Duration::from_secs(60)};
        let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576,&mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let mut app = VcsArtifactApp::<ComposedParentApp, TestMembers>::new(ComposedParentApp::default(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
        let dialect = test_child_dialect().await;
        let member = TestMembers::create("child-1", &dialect, &TestSnapshot::default().encode_pack(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await.unwrap();
        app.register_child("slot", "child-1", dialect.clone(), member, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("pure fresh member is adopted");
        assert_eq!(app.child_composition.test_child_admission_state(1).generation, 1);
        let owner = store::OwnerRef { parent: ArtifactRef { artifact_id: app.stores.store.envelope().id.clone(), dialect: ComposedParentApp::<true>::DIALECT.into() }, slot: "slot".into(), child_id: "child-1".into() };
        let member = app.child_store("slot", "child-1").await.unwrap();
        assert_eq!(member.owner_ref(), Some(owner.clone()));
        let packed = member.envelope_pack_bytes().await.unwrap();
        let mut restored = VcsArtifactApp::<ComposedParentApp, TestMembers>::new(ComposedParentApp::default(), protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), mounted_policy, &mut identity).await;
        restored.open_child("slot", "child-1", dialect, &packed, &mut semio_framework_plugin_testing_laws::artifact_app_laws::fixture_identity()).await.expect("fresh factory restores exact parent owner");
        assert_eq!(restored.test_child_admission_state(1).generation, 1);
        assert_eq!(restored.child_store("slot", "child-1").await.unwrap().owner_ref(), Some(owner.clone()));
        assert_eq!(restored.composition.graph_mut().await.owner_of("child-1").await, Some(owner.parent.artifact_id.as_str()));
        assert_eq!(restored.composition.graph_mut().await.slot_of("child-1").await, Some("slot"));
        assert!(restored.child_content_root.typed_read::<TestSnapshot>("slot", "child-1").is_ok());
        close_member_admission_app(&mut restored,cx);
        close_member_admission_app(&mut app,cx);
    
        }.await;let fuel=cx.fuel_remaining();drop(original_context);original_caller.retire(fuel).expect("original builder fixture paid terminal retirement");result
    }

    #[semio_framework_async_macros::async_test]
    async fn reserved_section_surfaces_render_accessor_maps_instead_of_app_bodies() {
        let mounted_grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
        let mounted_policy = semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: mounted_grant, maintenance: mounted_grant, close: mounted_grant };
        let identity_started = std::time::Instant::now();
        let mut identity_progress = |progress: semio_framework_value::native_encoding::NativeEncodeProgress| {
            progress.owned_bytes <= 1_048_576 && identity_started.elapsed() < std::time::Duration::from_secs(60)
        };
        let mut identity = semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new(1_048_576, &mut identity_progress as &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::Observer<'_>).expect("explicit original fixture identity authority");

        let fixture: Value = serde_json::from_str(include_str!("../../⚛️reactor/🪟️surfaces/🧪️tests/🧫️fixtures/🪟️surface-context-lifecycle/🔣️.json")).unwrap();
        let mut host_view: ViewModel = serde_json::from_value(fixture["view"].clone()).unwrap();
        host_view.active_tool_id = Some("fill".into());
        let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new({ let grant = semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }; semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1 { preparation: grant, maintenance: grant, close: grant } }).expect("explicit test mounted owner policy");
        let app = contract_app_raw(mounted_policy, &mut identity).await;
        runtime.instances.borrow_mut().insert_admitted(7, std::sync::Arc::new(semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(AppInstance { id: 7, app, surface_contexts: Default::default() }, runtime.mounted_policy)));
        let mut payloads = BTreeMap::new();
        for section in UiRefreshSection::ALL {
            let surface = format!("7:{}", section.body_key());
            semio_framework_plugin_runtime_ops::ops_surface_render::plugin_mount_surface(&runtime, 7, surface.clone(), section.body_key().into(), &semio_framework_plugin_plugin_program::encode_wire_serialized(&host_view)).await.unwrap();
            let (tree, presence) = semio_framework_plugin_runtime_ops::ops_surface_render::plugin_render_surface(&runtime, 7, &surface).await.unwrap();
            assert!(presence.is_empty());
            assert!(RENDER_CONTEXT_PROBE.with(|probe| probe.take()).is_none(), "a reserved section body key must never reach ArtifactApp::render");
            let projected: Value = serde_json::from_str(&artifact_app_laws::project_and_retire_fixture_tree(tree).unwrap()).unwrap();
            assert_eq!(projected["key"], section.body_key());
            assert_eq!(projected["component"]["type"], "container");
            let mut payload = String::new();
            let mut frontier = vec![projected.clone()];
            while let Some(node) = frontier.pop() {
                if node["component"]["type"] == "text" {
                    payload.push_str(node["component"]["value"].as_str().unwrap());
                }
                for child in node["children"].as_array().unwrap().iter().rev() {
                    frontier.push(child.clone());
                }
            }
            payloads.insert(section.key(), serde_json::from_str::<Value>(&payload).unwrap());
        }
        assert_eq!(payloads["engagements"], serde_json::json!({}));
        assert_eq!(payloads["measures"]["left"][0]["id"], "left-measure");
        assert_eq!(payloads["measures"]["right"][0]["id"], "right-measure");
        assert_eq!(payloads["tools"]["fill"][0]["id"], "fill-measure");
    }