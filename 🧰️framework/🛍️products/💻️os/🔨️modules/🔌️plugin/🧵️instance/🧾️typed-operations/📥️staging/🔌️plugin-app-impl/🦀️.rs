//! 🎺️ Staged slice of the `plugin-app-impl` band of the mounted artifact instance: its `instance-typed-ops` members (WP6 step S0 of the instance family).
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

/// ⏳️ Rounds one child publication spends reclaiming the retirements ahead of it before it waits instead.
/// ♻️ Returned document roots a publication leaves to the maintenance rotation: only beyond this many does a publication
/// unit drain them on its critical path. A one-item publication returns exactly one pre-edit root, so it never walks the
/// document; a multi-item batch drains before its second item reads a base, which keeps it bounded to the staged root,
/// the base it reads and this many returned roots (the 816-op fill OOM), never to one root per folded item.
pub(super) const PUBLICATION_RETURNED_ROOT_ALLOWANCE: usize = 1;
