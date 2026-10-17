//! 🔏️ Staged slice of the `instance-root` band of the mounted artifact instance: its `instance-typed-ops` members (WP6 step S0 of the instance family).
//!
//! 🧵️ The mounted artifact instance `VcsArtifactApp`: its state, its turns and its `PluginApp` implementation.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_framework_plugin_decl_emission::Emit;
use semio_framework::{Fault, FaultFrom};
use std::future::Future;
use semio_framework_plugin_instance_extension::InstanceExtension;
use store::{MemberFactory, Mutation, SpaceMember};
use protocol::OpText;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_ownership::sdk_fault::plugin_sdk_fault;
use semio_framework_ui_contract::*;

/// 🧯️ A batched store publication's refusal: the history, tool-transaction and size refusals keep their own fault codes
/// (`history.full`, `toolTransaction.open`, `toolTransaction.unknown`, `mutation.too-large` — the shells show them as notices);
/// every other one is the SDK's publication fault.
pub(crate) fn store_publication_fault(error: vcs::VcsError) -> Fault {
    match error {
        vcs::VcsError::TransactionOpen { .. } | vcs::VcsError::UnknownTransaction(_) | vcs::VcsError::HistoryFull { .. } | vcs::VcsError::TooLarge { .. } => error.into_fault(),
        error => plugin_sdk_fault(error.to_string()),
    }
}
