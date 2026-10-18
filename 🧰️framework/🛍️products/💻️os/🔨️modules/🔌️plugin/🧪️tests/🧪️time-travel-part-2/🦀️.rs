//! ⏪️ Part 2 of 2 of the history-edit session laws of the plugin runtime (cut from `🧪️time-travel`; the fixtures several parts use are the support module).

use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_ui_contract::BuiltNode;
use semio_framework_plugin_command::lifted_command::HISTORY_ROW_MUTATION_ROWS;
#[cfg(test)]
use semio_framework_ui_locale::Locale;
#[cfg(test)]
use semio_framework_ui_locale::Terminology;
use semio_framework_plugin_command::lifted_command::TimeTravelEditorPanel;
use semio_framework_plugin_command::lifted_command::TimeTravelPanel;
use semio_framework_plugin_window_kits_rich::tree_kit::TreeWindows;
use semio_framework_plugin_decl_app::ArtifactApp;
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
use semio_framework_plugin_abi::{ArtifactAnalysis, ArtifactAnalyzer, ArtifactBuilder, ArtifactChildren, ArtifactComposer, ArtifactComposition, ArtifactDecomposer, ArtifactDeserializer, ArtifactSerializer, DerivedArtifactSpec};
use semio_framework_plugin_decl_surface::{ArtifactEditor, ArtifactViewer};
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactOwnedToolJobFactory, ArtifactReservedJob};
use semio_framework_plugin_decl_emission::{AsyncTaskRun, AsyncTaskSource, Emit, ToolCommandCatalog};
use semio_framework_ui_contract::{Buildable, Component, HasBase, HasChildren};
use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner;
use semio_framework_plugin_window_kits::{LabelAxes, WindowKit};
#[cfg(test)]
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1;
use semio_framework_plugin_decl_app::NaturalFileDecodeCursor;
use semio_framework_plugin_decl_views::NoDraftMutation;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_plugin_program::PluginProgram;
use semio_framework_plugin_time_travel::TimeTravelActionRefusal;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_tests_06::time_travel_support_1::{ToyApp, ToyHistoryApp, apply_long_edit, close, collect_original_history_command_receipt, commit_scenario, dsl, edit_authors, find_node, fixture, head, meta, original_time_travel_fixture_caller, pump_until, rejected, render_body, render_history, row_actions, run_step, seed_op, seeded_app, seeded_mutation, text, toy_manifest, verb};
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_instance_app as artifact_instance;
use semio_framework_os_kernel::io;
use semio_framework::*;
use semio_framework::kernel::{ActorId, Effect};
use store::os_store::store_schema::ArtifactCommand;
use pack::snapshot::ArtifactPack;
use std::collections::BTreeSet;
use semio_framework_plugin_command::CATALOGUE_EXAMPLE_ACTION_ID;
use semio_framework_value::DslValue;
use semio_framework::{FaultCode, NOTE_SHELL_COMMAND_ACTION_ID, SET_HISTORY_COMMAND_FILTER_ACTION_ID};
use std::future::Future;
#[cfg(test)]
use semio_framework_plugin_command::{HISTORY_ROWS_BUILT, history_panel_mutation_row};
#[cfg(test)]
use semio_framework_plugin_decl_views::history_view::HistoryCommandFilter;
use store::{MemberFactory, Mutation, SpaceMember};
#[cfg(test)]
use semio_framework_plugin_decl_surface::replay_envelopes_fault;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission;
use semio_framework_plugin_time_travel as time_travel;
#[cfg(test)]
use semio_framework_plugin_decl_app::viewer_rejects_action;
use semio_framework_ui_contract::*;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework_time_travel::TimeTravelStage;
use store::os_store::backbone::{Backbone, MemoryBackbone};

include!("🗑️retirement-custody-and-transaction-rows/🦀️.rs");
include!("🔢️input-shapes-lists-unions-and-sources/🦀️.rs");
include!("🎨️references-colours-nullable-and-replay/🦀️.rs");
include!("🧮️ledger-replay-chips-and-alternatives/🦀️.rs");
include!("🗝️editor-keys-history-patch-and-actors/🦀️.rs");
include!("🪣️shared-schemas-filters-and-backbone/🦀️.rs");
