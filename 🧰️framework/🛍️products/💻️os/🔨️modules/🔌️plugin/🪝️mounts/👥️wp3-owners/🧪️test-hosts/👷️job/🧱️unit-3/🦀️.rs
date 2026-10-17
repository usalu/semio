//! 🧪️ Unit 3 of the facade test host of `job`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_decl_emission::ActionMeta;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::kernel::ActorId;
use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(any(test, feature = "artifact-app-testing"))]
use store::os_store::store_schema::ArtifactCommand;
use semio_framework_artifact_reference::ArtifactDialect;
#[cfg(test)]
use semio_framework_plugin_operation::ArtifactDocumentAuthority;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_artifact_reference::ArtifactRef;
use store::ArtifactStore;
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolCompletionValue;
#[cfg(test)]
use semio_framework_plugin_decl_app::tool_contract::ArtifactToolPublicationLane;
use semio_framework_ui_contract::BuiltNode;
use semio_framework_value::DslValue;
use semio_framework::kernel::Effect;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework_plugin_decl_app::app_emission::EphemeralEmit;
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_run_contract::FRAMEWORK_TOOL_RUN_BODY_KEY;
use semio_framework::Fault;
use semio_framework_ui_locale::Locale;
use store::Mutation;
#[cfg(test)]
use semio_framework_plugin_decl_views::NoDraftMutation;
use protocol::OpBinary;
#[cfg(test)]
use semio_framework_plugin_reactor_app::PluginApp;
#[cfg(test)]
use semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep;
use semio_framework_ui_locale::Terminology;
#[cfg(test)]
use semio_framework_plugin_operation::ToolOperationKey;
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunJobPort;
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_decl_app::tool_contract::TypedOperationResultLane;
use semio_framework::kernel::UiDirtyScope;
#[cfg(any(test, feature = "artifact-app-testing"))]
use serde_json::Value;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_instance_app::VcsArtifactApp;
#[cfg(test)]
use semio_framework::ViewModel;
use semio_framework_plugin_window_config::WindowConfigMutation;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigOwner;
#[cfg(test)]
use semio_framework_plugin_testing_laws::artifact_app_laws;
#[cfg(test)]
use semio_framework_plugin_retained_command::typed_operation::test_blank_mounted_operation;
use semio_framework_plugin_tool_run as tool_run;
use semio_framework_ui_contract::*;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework::FaultFrom;
use semio_framework_value_derive::FromValue;
use store::MutationDiff;
use protocol::OpText;
use semio_framework_value_derive::ToValue;
#[cfg(test)]
#[path = "../../../../../🧪️tests/🔬️tool-run-part-1/🦀️.rs"]
mod tool_run_tests_1;
