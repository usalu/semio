//! ⏯️ Part 1 of 2 of the runtime laws of the tool run ledger and driver (cut from `🔬️tool-run`; the fixtures several parts use are the support module).

use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionValue;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicationLane;
use semio_framework_ui_contract::BuiltNode;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
use semio_framework_plugin_decl_views::tool_run_contract::FRAMEWORK_TOOL_RUN_BODY_KEY;
#[cfg(test)]
use semio_framework_ui_locale::Locale;
use semio_framework_plugin_decl_views::NoDraftMutation;
#[cfg(test)]
use semio_framework_ui_locale::Terminology;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJobPort;
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_window_config::WindowConfigMutation;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;
use semio_framework_plugin_abi::{ArtifactAnalysis, ArtifactAnalyzer, ArtifactBuilder, ArtifactChildren, ArtifactComposer, ArtifactComposition, ArtifactDecomposer, ArtifactDeserializer, ArtifactSerializer, DerivedArtifactSpec};
use semio_framework_plugin_decl_surface::{ArtifactEditor, ArtifactViewer};
use semio_framework_plugin_decl_tool_contract::ArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::tool_contract::ArtifactReservedJob;
use store::ArtifactStoreInitializationAuthority;
use semio_framework_plugin_decl_emission::{AsyncTaskRun, AsyncTaskSource, ToolCommandCatalog};
use semio_framework_ui_contract::{Buildable, HasBase, HasChildren};
use semio_framework_plugin_extension::extension_runtime_lift::extension_retirement::ExtensionResourceOwner;
use semio_framework_plugin_window_kits::{LabelAxes, WindowKit};
use semio_framework_plugin_decl_app::NaturalFileDecodeCursor;
use semio_framework_plugin_plugin_program::PluginProgram;
use semio_framework_job::StepContext;
use semio_framework_plugin_tests_06::tool_run_support_1::{TOY_REQUEST_WINDOWS, ToyApp, ToyFixtureCaller, ToyPortJobCustody, ToyRunApp, ToyRunJob, ToyRunJobOriginal, ToyWorldWindowConfig, advance_toy, close, encoded, fixture, number, pump_until, run_action, set_target, start, text, tool_run_action, toy_app, toy_fixture_caller_policy, toy_manifest, toy_meta, toy_target};
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJobPurpose;
use semio_framework_plugin_decl_app::tool_run_contract::ToolRunJobRequest;
use semio_framework_plugin_window_transient::window_transient::WindowTransientOwner;
use semio_framework_os_kernel::io;
use semio_framework_plugin_interaction as local_interaction;
use semio_framework_plugin_window_config as window_config;
use semio_framework_ui_scene::scene_lane_hash;
#[cfg(test)]
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture;
use semio_framework::*;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::kernel::ActorId;
#[cfg(any(test, feature = "artifact-app-testing"))]
use store::os_store::store_schema::ArtifactCommand;
use semio_framework_artifact_reference::ArtifactDialect;
#[cfg(test)]
use semio_framework_plugin_operation::{ArtifactDocumentAuthority, ToolOperationKey};
use pack::snapshot::ArtifactPack;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_artifact_reference::ArtifactRef;
use store::{ArtifactStore, Mutation, MutationDiff};
use semio_framework_value::DslValue;
use semio_framework::kernel::{Effect, UiDirtyScope};
use semio_framework::{Fault, FaultFrom};
use semio_framework_value_derive::{FromValue, ToValue};
use protocol::{OpBinary, OpText};
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
#[cfg(any(test, feature = "artifact-app-testing"))]
use serde_json::Value;
#[cfg(test)]
use semio_framework::ViewModel;
#[cfg(test)]
use semio_framework_plugin_retained_command::typed_operation::test_blank_mounted_operation;
use semio_framework_plugin_tool_run as tool_run;
use semio_framework_ui_contract::*;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::change_test_config_selection::ChangeTestConfigSelection;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_count::SetCount;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::set_label::SetLabel;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::config::mutations::TestConfigMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation;
use semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot;
use semio_framework::RequestId;
use semio_framework::ViewWindowInstance;
use semio_framework_tool_run::{ToolRunId, ToolRunIdentity, ToolRunProgress, ToolRunState, ToolRunStepRing, ToolRunTick, ToolRunTickWriter, ToolRunTraceCursor, ToolRunTraceDelta, ToolRunTraceSubject, ToolRunVerdict};
use semio_framework_ui_scene::{Board2dScene, World3dScene};
use store::os_store::backbone::{Backbone, BackboneMessage, MemoryBackbone};
use semio_framework_plugin_tests_06::tool_run_support_1::original_toy_unit::OriginalToyUnit;
use semio_framework_plugin_tests_06::tool_run_support_1::original_toy_emission::OriginalToyEmission;

#[path="../🔬️tool-run/🚦️unit/🧪️tests/🦀️.rs"]
mod original_toy_job_tests;

include!("🧸️toy-job-ticks-abort-and-finalize/🦀️.rs");
include!("🚰️reclaim-ingest-pause-and-reconfigure/🦀️.rs");
include!("🔌️integration-seams/🦀️.rs");
include!("🔢️action-arg-carriers-and-read-only-runs/🦀️.rs");
include!("🎯️base-change-chords-and-retarget/🦀️.rs");
include!("🪟️trace-requests-and-window-config/🦀️.rs");
include!("📖️reader-windows-and-panel-fixtures/🦀️.rs");
include!("🛑️terminal-run-and-retained-config-close/🦀️.rs");
#[doc(hidden)]
#[path = "../🧪️tool-run-member/🦀️.rs"]
pub mod member;
