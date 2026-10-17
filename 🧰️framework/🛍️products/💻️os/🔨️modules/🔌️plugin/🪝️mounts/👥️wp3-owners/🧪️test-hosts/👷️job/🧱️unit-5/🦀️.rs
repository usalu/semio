//! 🧪️ Unit 5 of the facade test host of `job`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use semio_framework_plugin_decl_app::ArtifactApp;
#[cfg(test)]
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultCode;
use store::Mutation;
#[cfg(test)]
use semio_framework_plugin_decl_views::NoDraftMutation;
use protocol::OpBinary;
#[cfg(test)]
use semio_framework_plugin_reactor_app::PluginApp;
#[cfg(test)]
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework_plugin_window_config::WindowConfigOwner;
#[cfg(test)]
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_ui_contract::*;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework::FaultFrom;
use semio_framework_value_derive::FromValue;
use store::MutationDiff;
use protocol::OpText;
use semio_framework_value_derive::ToValue;
#[cfg(test)]
#[path = "../../../../../🧪️tests/🔬️tool-run-part-2/🦀️.rs"]
mod tool_run_tests_2;
