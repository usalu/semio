//! 🧪️ Unit 1 of the facade test host of `tool-run-turns`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_tool_run::TOOL_RUN_ACTION_IDS;
use semio_framework_ui_contract::*;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use semio_framework_plugin_decl_app::tool_contract::ArtifactOwnedToolJobFactory;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use semio_framework_value_derive::FromValue;
use store::MemberFactory;
use store::Mutation;
use store::MutationDiff;
use protocol::OpBinary;
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_scene::SceneDoc;
use store::{ReferenceSpaceMember, SpaceMember};
use semio_framework_value_derive::ToValue;
use semio_framework_plugin_tool_run::ToolRunMemberEmissionOwner;
use semio_framework_plugin_tool_run::ToolRunMemberOwners;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;

#[doc(hidden)]
#[cfg(test)]
#[path = "../../../../../⏯️tool-run/🧵️instance/🧪️tests/🦀️.rs"]
pub mod extension_tests;
