//! 🧪️ Unit 1 of the facade test host of `tool-run`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use semio_framework_plugin_ownership::bounded_ownership::ARTIFACT_LIVE_OUTPUT_SLOTS;
#[cfg(any(test, feature = "artifact-app-testing"))]
use semio_framework::kernel::ActorId;
use std::sync::Arc;
use semio_framework::kernel::HybridLogicalTimestamp;
use semio_framework_tool_machine::ScrubPhase;
#[cfg(test)]
use semio_framework_tool_machine::TypingCommit;
#[cfg(any(test, feature = "artifact-app-testing"))]
use serde_json::Value;
#[cfg(test)]
use semio_framework_plugin_testing_laws::artifact_app_laws;
use semio_framework_ui_contract::*;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_framework_value_derive::FromValue;
use store::Mutation;
use store::MutationDiff;
use protocol::OpBinary;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value_derive::ToValue;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;

#[doc(hidden)]
#[cfg(test)]
#[path = "../../../../../🧪️tests/🧪️scrub/🦀️.rs"]
pub mod tests;
#[cfg(test)]
#[path = "../../../../../🧪️tests/🧪️typing/🦀️.rs"]
mod typing_tests;
