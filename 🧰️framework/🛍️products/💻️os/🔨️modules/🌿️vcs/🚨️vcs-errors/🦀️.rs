//! 🚨️ Version graph errors: the version graph error with its display, fault and conversion implementations.
//!
//! Holds the `🔖️Errors` region of the module; the root re-exports its items.

use super::*;

//#region 🔖️Errors
// 🎞️ `Eq` dropped: `Rejected` below carries
// `Vec<protocol::MutationMessage>`, and `MutationMessage` itself only derives `PartialEq`
// (not `Eq`) — see `📡️spr/🎮️command`'s `🔖️Message` region.
#[derive(Debug, PartialEq, semio_framework_value::RetireOwned)]
pub enum VcsError {
    UnknownEdit(String),
    UnknownChange(String),
    UnknownAlternative(String),
    NoCheckpoint,
    EmptyApply,
    MutationApply(MutationApplyError),
    InverseRefused(ValueError),
    NativeEncoding(ValueError),
    /// ♻️ The complete original retirement failure retains every independent cause and receipt.
    OriginalRetirement(semio_framework_value::OriginalRetirementFailure),
    NothingToUndo,
    ForeignEdit(String),
    NothingToRedo,
    Serialize(String),
    Deserialize(String),
    Backbone(String),
    /// 🧬️ Envelope or mutation dialect coordinates differ.
    DialectMismatch(String),
    /// 🪜️ The operation requires a dialect migration first.
    MigrationRequired(String),
    /// 🛠️ A registered dialect migration failed.
    MigrationFailed(String),
    /// 🔁️ The composition edge would create an ancestor cycle; owned-child forests must remain acyclic.
    CompositionCycle(String),
    /// 🚫️ A child has another owner or is outside the group's stated parent.
    OwnershipViolation(String),
    /// 🛂️ Structural validation rejects the whole group before effects. Ordinary mutation refusals use `Rejected`.
    ValidationFailed(String),
    /// 🧯️ Group application and reverse compensation failed; the message identifies restored and remaining members.
    CompensationFailed(String),
    /// 🛑️ Merge policy rejects the whole command before application, retaining its policy and all replay messages.
    Rejected {
        policy: protocol::MergePolicy,
        messages: Vec<protocol::MutationMessage>,
    },
    /// ❓️ `store::ArtifactStore::resolve_conflict` was called with an id that names no
    /// currently-`Open` conflict on this store.
    UnknownConflict(String),
    /// ⌛️ The replay's expected generation is stale; recompute against the live generation.
    Stale { expected_generation: u64, generation: u64 },
    /// 🗂️ The history shape cannot hold this transition; refuse before recording it.
    HistoryShape { shape: protocol::HistoryShape, kind: protocol::HistoryTransitionKind },
    /// 🎞️ Until commit or abort, only this transaction's appends, remote ingest and merge-policy changes may run.
    TransactionOpen { transaction_id: String },
    /// 🕳️ A commit, abort or append named a tool transaction that is not the one open on this store.
    UnknownTransaction(String),
    /// 🧱️ The document's edit history holds `capacity` edits and is full: the edit was refused before anything was recorded.
    HistoryFull { capacity: usize },
    /// 🐢️ A history replay must finish or be discarded before another history step is authored.
    HistoryReplaying,
    /// 🐘️ Forward and inverse rows exceed the admitted capacity; refuse before recording them.
    TooLarge { rows: usize, capacity: usize },
    /// 🧷️ This unit spans documents; supersession requires changing all member documents together.
    UnitSpansDocuments { mutation_id: String, unit: String },
}

impl std::fmt::Display for VcsError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownEdit(id) => write!(formatter, "unknown edit id: {id}"),
            Self::UnknownChange(id) => write!(formatter, "unknown change id: {id}"),
            Self::UnknownAlternative(id) => write!(formatter, "unknown alternative id: {id}"),
            Self::NoCheckpoint => formatter.write_str("no checkpoint for alternative"),
            Self::EmptyApply => formatter.write_str("empty apply command"),
            Self::MutationApply(error) => write!(formatter, "mutation diff rejected: {error}"),
            Self::InverseRefused(error) => write!(formatter, "mutation inverse refused: {error}"),
            Self::NativeEncoding(error) => write!(formatter, "native encoding refused: {error}"),
            Self::OriginalRetirement(error) => write!(formatter, "original retirement refused: {error}"),
            Self::NothingToUndo => formatter.write_str("nothing to undo"),
            Self::ForeignEdit(id) => write!(formatter, "cannot undo edit authored by another actor: {id}"),
            Self::NothingToRedo => formatter.write_str("nothing to redo"),
            Self::Serialize(message) => write!(formatter, "serialize error: {message}"),
            Self::Deserialize(message) => write!(formatter, "deserialize error: {message}"),
            Self::Backbone(message) => write!(formatter, "backbone error: {message}"),
            Self::DialectMismatch(message) => write!(formatter, "dialect mismatch: {message}"),
            Self::MigrationRequired(message) => write!(formatter, "migration required: {message}"),
            Self::MigrationFailed(message) => write!(formatter, "migration failed: {message}"),
            Self::CompositionCycle(message) => write!(formatter, "composition cycle: {message}"),
            Self::OwnershipViolation(message) => write!(formatter, "ownership violation: {message}"),
            Self::ValidationFailed(message) => write!(formatter, "validation failed: {message}"),
            Self::CompensationFailed(message) => write!(formatter, "group dispatch failed and rollback also failed: {message}"),
            Self::Rejected { policy, .. } => write!(formatter, "rejected by merge policy {policy:?}"),
            Self::UnknownConflict(id) => write!(formatter, "unknown conflict id: {id}"),
            Self::Stale { expected_generation, generation } => write!(formatter, "stale result: computed at generation {expected_generation}, store is at {generation}"),
            Self::HistoryShape { shape, kind } => write!(formatter, "a {} history holds no {} transition", shape.name(), kind.name()),
            Self::TransactionOpen { transaction_id } => write!(formatter, "tool transaction {transaction_id} is open on this store"),
            Self::UnknownTransaction(transaction_id) => write!(formatter, "tool transaction {transaction_id} is not open on this store"),
            Self::HistoryFull { capacity } => write!(formatter, "the edit history holds at most {capacity} edits and is full"),
            Self::HistoryReplaying => formatter.write_str("a history step still replays on this store"),
            Self::TooLarge { rows, capacity } => write!(formatter, "the edit needs {rows} staged rows where at most {capacity} are admitted"),
            Self::UnitSpansDocuments { mutation_id, unit } => write!(formatter, "operation {mutation_id} belongs to the cross-artifact unit {unit}, which one document alone cannot supersede"),
        }
    }
}

impl std::error::Error for VcsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MutationApply(error) => Some(error),
            Self::InverseRefused(error) => Some(error),
            Self::NativeEncoding(error) => Some(error),
            Self::OriginalRetirement(error) => Some(error),
            _ => None,
        }
    }
}

impl From<semio_framework_value::OriginalRetirementFailure> for VcsError {fn from(error:semio_framework_value::OriginalRetirementFailure)->Self{Self::OriginalRetirement(error)}}

impl From<ValueError> for VcsError {fn from(error:ValueError)->Self{Self::NativeEncoding(error)}}

impl From<MutationApplyError> for VcsError {
    fn from(error: MutationApplyError) -> Self {
        Self::MutationApply(error)
    }
}

/// 🚨️ Every `VcsError` is a module fault; the refusals a runtime surfaces as their own notices carry their own codes:
/// `toolTransaction.open`, `toolTransaction.unknown`, `history.full` (its capacity the notice's `{n}` param),
/// `history.replaying`, `mutation.too-large` and `history.unit-spans-documents`.
impl semio_framework_diagnostic::FaultFrom for VcsError {
    fn fault_origin(&self) -> semio_framework_diagnostic::FaultOrigin {
        semio_framework_diagnostic::FaultOrigin::Module
    }

    fn fault_code(&self) -> semio_framework_diagnostic::FaultCode {
        semio_framework_diagnostic::FaultCode::new(match self {
            Self::TransactionOpen { .. } => "toolTransaction.open",
            Self::UnknownTransaction(_) => "toolTransaction.unknown",
            Self::HistoryFull { .. } => "history.full",
            Self::HistoryReplaying => "history.replaying",
            Self::TooLarge { .. } => "mutation.too-large",
            Self::UnitSpansDocuments { .. } => "history.unit-spans-documents",
            _ => "module.vcs",
        })
    }

    fn fault_severity(&self) -> semio_framework_diagnostic::Severity {
        semio_framework_diagnostic::Severity::Error
    }

    fn fault_message(&self) -> String {
        self.to_string()
    }

    fn fault_params(&self) -> semio_framework_diagnostic::FaultParams {
        match self {
            Self::HistoryFull { capacity } => semio_framework_diagnostic::FaultParams(vec![("n".to_string(), capacity.to_string())]),
            _ => semio_framework_diagnostic::FaultParams::default(),
        }
    }
}

//#endregion 🔖️Errors
