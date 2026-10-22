//! 🧪️ Unit 1 of the facade test host of `time-travel-turns`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use semio_framework::history_edit::HISTORY_EDIT_BEGIN_ACTION_ID;
use semio_framework::history_edit::HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID;
use semio_framework::history_edit::HISTORY_EDIT_COMMIT_ACTION_ID;
use semio_framework_plugin_instance_extension::InstanceExtension;
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
use store::{ReferenceSpaceMember, SpaceMember};
use semio_framework_plugin_time_travel::TimeTravelOwners;
use semio_framework_value_derive::ToValue;
use semio_framework_plugin_decl_views::tool_run_contract::ToolRunRetargetableJob;
use semio_framework_plugin_window_config::WindowConfigOwner;

#[doc(hidden)]
#[cfg(test)]
#[path = "../../../../../⏪️time-travel/🧵️instance/🧪️tests/🦀️.rs"]
pub mod extension_tests;
