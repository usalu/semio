//! 🪘️ Staged slice of the `plugin-app-impl` band of the mounted artifact instance: its `instance-ladders` members (WP6 step S0 of the instance family).
//!
//! 🔌️ The `PluginApp` implementation of the mounted artifact instance with the framework-reserved settle helpers and the maintenance stage constants.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;

/// 🕰️ Length of [`PluginApp::maintenance_step`]'s fixed cooperative round robin. Every stage
/// runs at most one bounded unit per call, so a step-budget law needs exactly this many calls
/// to observe each stage once.
pub const MAINTENANCE_STAGES: u8 = 27;

/// 🧹️ The rotation stage that drains the document store's displaced-owner queue.
pub const MAINTENANCE_DOCUMENT_DISPLACED_STAGE: u8 = 9;

/// 🧹️ The rotation stage that drains the config-lane stores' displaced-owner queues.
pub const MAINTENANCE_CONFIG_LANE_DISPLACED_STAGE: u8 = 25;

/// 🍂️ The rotation stage that disposes retired child-content roots, also run out of turn under child pressure.
pub(super) const MAINTENANCE_CHILD_ROOT_STAGE: u8 = 4;

/// 🥀️ The rotation stage that closes retired child members, also run out of turn under child pressure.
pub(super) const MAINTENANCE_CHILD_MEMBER_STAGE: u8 = 20;
