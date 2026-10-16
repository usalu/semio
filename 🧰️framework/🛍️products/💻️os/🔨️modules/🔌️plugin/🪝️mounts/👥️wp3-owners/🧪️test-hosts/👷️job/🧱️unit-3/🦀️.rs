//! 🧪️ Unit 3 of the facade test host of `job`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_emission::ActionMeta;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::kernel::ActorId;
#[cfg(test)]
use semio_framework_plugin_app_builder::App;
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use store::os_store::store_schema::ArtifactCommand;
use semio_framework_artifact_reference::ArtifactDialect;
#[cfg(test)]
use semio_framework_plugin_operation::ArtifactDocumentAuthority;
#[cfg(test)]
use semio_framework_plugin_decl_emission::ArtifactMutationOutcome;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_artifact_reference::ArtifactRef;
use store::ArtifactStore;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionValue;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicationLane;
#[cfg(test)]
use semio_framework_plugin_decl_views::ArtifactView;
use semio_framework_ui_contract::BuiltNode;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_ui_runtime::ComponentTree;
#[cfg(any(test, feature = "artifact-app-testing"))]
use store::ConfigStore;
#[cfg(test)]
use semio_framework_plugin_decl_views::ConfigView;
use semio_framework_value::retirement::controlled::ControlledRetirement;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_artifact_reference::Dialect;
#[cfg(test)]
use semio_framework_plugin_decl_views::DraftView;
use semio_framework_value::DslValue;
use semio_framework::kernel::Effect;
use semio_framework_plugin_decl_emission::Emit;
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_contract::EmptyArtifactInstanceOperationOwner;
use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_run_contract::FRAMEWORK_TOOL_RUN_BODY_KEY;
use semio_framework::Fault;
use semio_framework::FaultCode;
use semio_framework::IconName;
#[cfg(test)]
use semio_framework_plugin_decl_views::InteractionView;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use store::Mutation;
#[cfg(test)]
use semio_framework_plugin_decl_views::NoDraft;
#[cfg(test)]
use semio_framework_plugin_decl_views::NoDraftMutation;
#[cfg(test)]
use semio_framework_plugin_decl_views::NoPresence;
#[cfg(test)]
use semio_framework_plugin_decl_views::NoPresenceMutation;
#[cfg(test)]
use semio_framework_plugin_decl_views::NoTransient;
#[cfg(test)]
use semio_framework_plugin_decl_views::NoTransientMutation;
use protocol::OpBinary;
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_run_contract::OriginalToolRunJobPortRetirement;
#[cfg(test)]
use crate::plugin_app::PluginApp;
#[cfg(test)]
use semio_framework_plugin_ownership::sdk_fault::PluginAssemblyError;
#[cfg(test)]
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_artifact_reference::StandardId;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_artifact_reference::SubsetId;
use semio_framework_ui_locale::Terminology;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::ToolDefinition;
#[cfg(test)]
use semio_framework_plugin_operation::ToolOperationKey;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::ToolRef;
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJob;
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJobPort;
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
use semio_framework_plugin_interaction::UiAssemblyResult;
use semio_framework::kernel::UiDirtyScope;
#[cfg(any(test, feature = "artifact-app-testing"))]
use serde_json::Value;
#[cfg(any(test, feature = "artifact-app-testing"))]
use crate::artifact_instance::VcsArtifactApp;
#[cfg(test)]
use semio_framework::ViewModel;
use semio_framework_plugin_window_config::WindowConfigMutation;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigOwner;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigOwnerRegistry;
#[cfg(test)]
use crate::app_testing::artifact_app_laws;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::bounded_config_store_disposer;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::bounded_config_store_owners;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_disposer;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::bounded_document_store_owners;
#[cfg(test)]
use semio_framework_plugin_window_transient::built_text_to_component_tree;
#[cfg(test)]
use semio_framework_plugin_window_transient::built_to_component_tree;
#[cfg(test)]
use crate::app_testing::mutation_fixture;
#[cfg(test)]
use semio_framework_plugin_window_transient::scene_surface;
#[cfg(test)]
use crate::typed_operation::test_blank_mounted_operation;
use crate::tool_jobs::tool_run;
use semio_framework_ui_contract::*;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework::FaultFrom;
use semio_framework_value_derive::FromValue;
use store::MutationDiff;
use protocol::OpText;
use semio_framework_value_derive::ToValue;
#[cfg(test)]
#[path = "../../../../../🧪️tests/🔬️tool-run/🦀️.rs"]
mod tool_run_tests;
