//! 🎫️ The ticket domain of the semio repository: the dated id and path scheme, the
//! `🎫️ticket.json` document codec, the open/close/reopen/change lifecycle, the important-document
//! transaction, artifact purging, search, plan/spec source resolution and issue synchronisation.
//!
//! Every filesystem touch goes through [`TicketStore`], every issue mutation through the
//! `IssueTracker` port of `🧩️providers`, every clock read through [`Clock`] and every emission
//! through [`EventSink`], so the whole lifecycle runs in memory with nothing on disk and no `gh`
//! on the machine — see `🧪️tests/🔓️open-close-reopen-lifecycle`.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/🧬️schema/🔣️.json
//! @see 🧰️framework/🛍️products/🦑️repo/AGENTS.md (§5.6, §8.10 — the ticket algebra and id rule)

use semio_framework_repo_model::{Interaction, InteractionFile, Ticket, TicketAgent, TicketManagementData, TicketPlan, TicketStatus};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

//#region 🔁️Reexports

/// 🔁️ The provider shapes a client of this crate needs in order to call it.
///
/// A client may not name `semio-framework-repo-providers` itself — a generated test host links only
/// the crate under test — so everything the ticket API takes or returns is re-exported here
/// explicitly, per the repository's "reexport explicitly if the client needs it" rule.
pub use semio_framework_repo_providers::{IssueTracker, ManagementIssue, ManagementLabel, ManagementMilestone, ManagementProviders, McpClientKind, NullManagementProvider, ProviderError, ProviderResult};

//#endregion 🔁️Reexports

//#region ❌️Errors

/// ⚠️ Every ticket failure, carrying a class for machine consumers and the Go message verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TicketError {
    pub class: String,
    pub message: String,
}

impl TicketError {
    /// 🆕️ Builds an error of a class.
    pub fn new(class: &str, message: impl Into<String>) -> TicketError {
        TicketError { class: class.to_string(), message: message.into() }
    }

    /// 🚫️ An input the caller may correct.
    pub fn invalid(message: impl Into<String>) -> TicketError {
        TicketError::new("invalid", message)
    }

    /// 🔍️ A ticket, document or node that is not there.
    pub fn not_found(message: impl Into<String>) -> TicketError {
        TicketError::new("not-found", message)
    }

    /// 💥️ A store operation that failed.
    pub fn store(message: impl Into<String>) -> TicketError {
        TicketError::new("store", message)
    }

    /// 🔀️ A state that forbids the requested transition.
    pub fn conflict(message: impl Into<String>) -> TicketError {
        TicketError::new("conflict", message)
    }
}

impl std::fmt::Display for TicketError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for TicketError {}

/// 🎯️ The result of every operation in this crate.
pub type TicketResult<T> = Result<T, TicketError>;

//#endregion ❌️Errors

#[path = "🪪️ids/🦀️.rs"]
mod ids;
pub use ids::*;

#[path = "📄️codec/🦀️.rs"]
mod codec;
pub use codec::*;

#[path = "🗄️store/🦀️.rs"]
mod store;
pub use store::*;

#[path = "💾️documents/🦀️.rs"]
mod documents;
pub use documents::*;

#[path = "📐️file-scope/🦀️.rs"]
mod file_scope;
pub use file_scope::*;

#[path = "🐙️issue-sync/🦀️.rs"]
mod issue_sync;
pub use issue_sync::*;

#[path = "⏰️ports/🦀️.rs"]
mod ports;
pub use ports::*;

#[path = "📝️drafts/🦀️.rs"]
mod drafts;

#[path = "🔓️lifecycle/🦀️.rs"]
mod lifecycle;
pub use lifecycle::*;

#[path = "🔎️search/🦀️.rs"]
mod search;
pub use search::*;

//#region 🧪️Tests

#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests

#[cfg(test)]
#[path = "../../../🔗️graphql/🧪️tests/🌿️tickets/📁️scope/🦀️.rs"]
mod rooted_lifecycle_tests;
