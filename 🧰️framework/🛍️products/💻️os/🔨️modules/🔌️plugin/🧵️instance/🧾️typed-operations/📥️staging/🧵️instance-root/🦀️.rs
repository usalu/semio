//! 🔏️ Staged slice of the `instance-root` band of the mounted artifact instance: its `instance-typed-ops` members (WP6 step S0 of the instance family).
//!
//! 🧵️ The mounted artifact instance `VcsArtifactApp`: its state, its turns and its `PluginApp` implementation.
use super::*;

/// 🧯️ A batched store publication's refusal: the history, tool-transaction and size refusals keep their own fault codes
/// (`history.full`, `toolTransaction.open`, `toolTransaction.unknown`, `mutation.too-large` — the shells show them as notices);
/// every other one is the SDK's publication fault.
pub(crate) fn store_publication_fault(error: vcs::VcsError) -> Fault {
    match error {
        vcs::VcsError::TransactionOpen { .. } | vcs::VcsError::UnknownTransaction(_) | vcs::VcsError::HistoryFull { .. } | vcs::VcsError::TooLarge { .. } => error.into_fault(),
        error => plugin_sdk_fault(error.to_string()),
    }
}
