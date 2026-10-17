//! 🥁️ Staged slice of the `plugin-app-impl` band of the mounted artifact instance: its `instance-docs` members (WP6 step S0 of the instance family).
//!
//! 🔌️ The `PluginApp` implementation of the mounted artifact instance with the framework-reserved settle helpers and the maintenance stage constants.
use semio_framework::kernel::AppEvent;
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_value::DslValue;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::FaultFrom;
use std::future::Future;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_ui_contract::*;

/// 📣️ Signals the shell that the document's checkpoint/alternative history changed (after an
/// undo/redo/checkpoint/alternative command) so it can re-render history-dependent surfaces.
#[doc(hidden)]
pub async fn history_changed_event() -> AppEvent {
    AppEvent { kind: "history-changed".into(), payload: DslValue::Null }
}
