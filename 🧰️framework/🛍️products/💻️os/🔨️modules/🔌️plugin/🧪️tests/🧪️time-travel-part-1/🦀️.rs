//! ⏪️ Part 1 of 2 of the history-edit session laws of the plugin runtime (cut from `🧪️time-travel`; the fixtures several parts use are the support module).

use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_ui_layout::layout::FRAMEWORK_HISTORY_BODY_KEY;
use semio_framework_plugin_decl_emission::HostEvent;
#[cfg(test)]
use semio_framework_ui_locale::Locale;
use semio_framework_plugin_time_travel::TIME_TRAVEL_TURN_WALL_US;
#[cfg(test)]
use semio_framework_ui_locale::Terminology;
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
use semio_framework_plugin_abi::{ArtifactAnalysis, ArtifactAnalyzer, ArtifactBuilder, ArtifactChildren, ArtifactComposer, ArtifactComposition, ArtifactDecomposer, ArtifactDeserializer, ArtifactSerializer, DerivedArtifactSpec};
use semio_framework_plugin_decl_surface::{ArtifactEditor, ArtifactViewer};
use semio_framework_plugin_decl_views::tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::{ArtifactOwnedToolJobFactory, ArtifactReservedJob};
use semio_framework_plugin_decl_emission::{AsyncTaskRun, AsyncTaskSource, Emit, ToolCommandCatalog};
use semio_framework_ui_contract::{Buildable, HasBase, HasChildren};
use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner;
use semio_framework_plugin_window_kits::{LabelAxes, WindowKit};
#[cfg(test)]
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin_mounted_owner::MountedOwnerPolicyV1;
use semio_framework_plugin_decl_app::NaturalFileDecodeCursor;
use semio_framework_plugin_decl_views::NoDraftMutation;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_plugin_program::PluginProgram;
use semio_framework_plugin_tests_06::time_travel_support_1::{TOY_HOST_EVENTS, ToyApp, ToyHistoryApp, apply_long_edit, close, collect_original_history_command_receipt, commit_scenario, dsl, edit_authors, find_node, fixture, head, meta, original_time_travel_fixture_caller, publish_as, pump_until, rejected, render_body, render_history, row_actions, run_step, seed_op, seeded_app, seeded_app_as, seeded_mutation, text, toy_manifest, verb};
use semio_framework_plugin_time_travel::TimeTravelActionRefusal;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_testing_laws::{artifact_app_laws, with_authoring_identity};
use semio_framework_plugin_instance_app as artifact_instance;
use semio_framework_plugin_actor_host as host;
use semio_framework_plugin_extension::describe;
use semio_framework_plugin_command::lifted_command::history_mutation_entry;
use semio_framework_os_kernel::io;
use semio_framework::*;
use semio_framework::kernel::{ActorId, HybridLogicalTimestamp, InvocationResult, MutationId, UiDirtyScope};
use store::os_store::store_schema::{ArtifactCommand, HistoryLane};
use semio_framework_artifact_reference::ArtifactDialect;
use pack::snapshot::ArtifactPack;
use store::{ArtifactStore, MemberFactory, Mutation, SpaceMember};
use std::collections::{BTreeMap, BTreeSet};
use semio_framework_value::DslValue;
use semio_framework::{FaultCode, SET_HISTORY_COMMAND_FILTER_ACTION_ID};
use std::future::Future;
use semio_framework_plugin_interaction::mutations::InteractionConfigMutation;
use protocol::LOCAL_ACTOR_ID;
#[cfg(test)]
use semio_framework_plugin_command::PURE_HISTORY_UNAVAILABLE_CODE;
#[cfg(test)]
use semio_framework_plugin_decl_emission::TransactionPhase;
use store::os_store::document_materialize::create_document_envelope;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_app::plugin_app_impl::settle_framework_reserved_admission;
use semio_framework_plugin_time_travel as time_travel;
use semio_framework_ui_contract::*;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_slot_children::SetSlotChildren;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework_time_travel::TimeTravelStage;
use store::os_store::backbone::{Backbone, BackboneMessage, MemoryBackbone};

include!("🎬️fixture-scenarios-sessions-and-drafts/🦀️.rs");
include!("🚚️long-transactions-and-single-controls/🦀️.rs");
include!("🔁️unchanged-input-stage-and-alternatives/🦀️.rs");
include!("🪞️replicas-and-history-edit-rows/🦀️.rs");
include!("✅️acceptance-blocked-review-and-long-history/🦀️.rs");
include!("⏲️interior-undo-pure-lane-and-wall-deadline/🦀️.rs");
include!("🔄️opened-instance-edit-drafts-and-withdraw/🦀️.rs");
include!("🧨️inert-label-op-and-hostile-input/🦀️.rs");
