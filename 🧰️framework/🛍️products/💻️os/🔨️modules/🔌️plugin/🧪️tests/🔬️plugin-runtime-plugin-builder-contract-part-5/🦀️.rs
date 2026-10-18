//! 🔬️ Part 5 of 7 of the plugin builder contract laws (cut from `🔬️plugin-runtime-plugin-builder-contract`; the fixtures several parts use are the support modules).
//! 🧪️ The plugin contract's own unit test: a `TestApp` implementing the pure `ArtifactApp`
//! surface (B1), wrapped in `VcsArtifactApp`, exercising typed operations with true inverses, config
//! operations that emit no document operations, history interception, and remote-operation ingest
//! idempotency. `TestCommand` is `TestApp`'s typed `Self::Command`; framework-reserved verbs
//! (history/clipboard/revert/filter/noteShellCommand) still dispatch by string via `handle_action`/
//! `handle_command` — everything app-specific dispatches via `dispatch_typed`.

use semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparation;
use semio_framework_plugin_decl_emission::child_emit_preparation::ChildEmitPreparationStep;
use semio_framework::kernel::FRAMEWORK_RESERVED_JOB_KIND;
use semio_framework_plugin_command::HISTORY_ROW_OPERATION_PREVIEW;
use semio_framework_plugin_instance_ladders::ladders_plugin_app_impl::MAINTENANCE_STAGES;
use semio_framework_plugin_decl_app::MediaConsumption;
use semio_framework_plugin_builder::Ready;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use serde_json::Value;
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_window_kits::context_menu;
use semio_framework_plugin_instance_docs::docs_maintenance_ladder::item;
use semio_framework_plugin_decl_app::media_type;
use semio_framework_plugin_plugin_program::plugin_runtime;
use semio_framework_os_kernel as protocol;
use semio_framework_os_kernel::io::resolve;
use semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission;
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
use semio_framework_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::{INGRESS_TURNS_PER_COMMAND, TEST_APP_WINDOW_BODY_KEY, TestApp, TestCommand, __semio_plugin_bundle, app_maintenance_turn, close_reserved_app, command_page_authority_budget, contract_app, contract_app_raw, contract_app_under_test, contract_app_with, contract_composed_app_raw, declare_test_app_verbs, declare_test_child, drain_and_close_composed_fixture, drain_and_close_fixture, dv, fixture_close_grant, inline_interaction_select, interaction_app_under_test, interaction_target_args, label_in, meta, new_bare_test_child, prepare_original_fixture_child, publish_presence_roster, reactor_native_lifecycle_ack, reactor_native_lifecycle_finish, reactor_native_lifecycle_open, reactor_native_lifecycle_poll, register_test_child, reserved_action, sample_presence_peer, settle_contract_app, settle_reserved, test_app_surface_id, test_child_dialect};
use semio_framework_ui_contract::*;
use semio_framework::*;
use semio_framework::kernel::*;
use semio_framework_plugin_reactor_app::__semio_dispatch_PluginApp;
use semio_framework_plugin_interaction::interaction_topology::INLINE_INTERACTION_VERB_DIAGNOSTIC_CODE;
use semio_framework_value::DslValue;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_ui_locale::Terminology;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_app_builder::App;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_window_kits::AppActionRegistry;
use semio_framework_plugin_decl_views::media_contract::ArtifactDownloadOutput;
use semio_framework_plugin_decl_views::media_contract::ArtifactOutputChunks;
use semio_framework_plugin_decl_views::history_view::CommandView;
use semio_framework_plugin_decl_views::history_view::HistoryCommandFilter;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_abi::{ArtifactDeserializer, ArtifactSerializer};
use semio_framework_diagnostic::FaultFrom;
use semio_framework::IconName;
use semio_framework::ViewModel;
use semio_framework::{MediaClass, MediaType};
use semio_framework_os_kernel::Mutation;
use semio_framework::Fault;
use semio_framework::kernel::{ClipboardFragment, Effect, PasteAnchor, UiDirtyScope};
use semio_framework::{ActionArgDef, ActionDefinition, ActionKind, MediaForm, NOTE_SHELL_COMMAND_ACTION_ID, REVERT_TO_COMMAND_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID};
use semio_framework_job::InteractiveJob as _;
use serde_json::json;
use std::collections::BTreeMap;
use pack::snapshot::ArtifactPack;
use semio_framework_os_kernel::os_store::backbone::{Backbone, BackboneMessage, MemoryBackbone};
use semio_framework_os_kernel::{MemberFactory, SpaceMember};
use semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY;
use semio_framework_ui_layout::layout::{ContextMenuItemSpec, ContextMenuRequest, UiMenuRef};
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;

include!("🎡️maintenance-rotation-download-and-ingress/🦀️.rs");
include!("🥾️operation-action-stack-and-peer-presence/🦀️.rs");
include!("📎️member-owners-media-edge-and-clipboard/🦀️.rs");
include!("🧶️streamed-ticks-and-reserved-undo/🦀️.rs");
include!("📔️command-log-laws/🦀️.rs");
include!("🔭️empty-undo-ingest-and-context-menu/🦀️.rs");
include!("🧮️gestures-manifest-commands-and-selection-persistence/🦀️.rs");
include!("🎚️selection-modes-verbs-and-hover/🦀️.rs");
    mod owned_child_dispatch_laws { include!("../🔬️plugin-runtime-plugin-builder-contract/📨️owned-child-dispatch/🦀️.rs"); }
