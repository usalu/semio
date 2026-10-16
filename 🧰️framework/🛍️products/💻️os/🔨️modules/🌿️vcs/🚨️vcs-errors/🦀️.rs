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
    NothingToUndo,
    ForeignEdit(String),
    NothingToRedo,
    Serialize(String),
    Deserialize(String),
    Backbone(String),
    /// 🧬️ A migration/replay/merge was attempted across two envelopes/mutations whose
    /// `dialect` coordinates don't match (see `crate::os_store::store_schema::ArtifactEnvelope::dialect`, `26/08/10` D4
    /// evolution slice). Not yet raised by any call site in this pass — additive only.
    DialectMismatch(String),
    /// 🧬️ An operation needs a dialect migration to run first (see `crate::os_store::codec_registry::migrate_document`)
    /// before it can proceed. Not yet raised by any call site in this pass — additive only.
    MigrationRequired(String),
    /// 🧬️ A registered dialect migration ran but failed. Not yet raised by any call site in
    /// this pass — additive only.
    MigrationFailed(String),
    /// 🔁️ A composition-pin graph traversal (parent → child → …) found a cycle back to an
    /// ancestor — an owned-child forest must stay acyclic. Raised by `crate::os_store::composition_coordinator::CompositionGraph::
    /// would_cycle_owns`/`would_cycle_links` via `crate::os_store::composition_coordinator::CompositionCoordinator::dispatch_group`'s
    /// phase-1 validation (`UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` `🔖️CompositionCoordinator`, wave B2).
    CompositionCycle(String),
    /// 🚫️ An operation would violate composition's single-ownership invariant (e.g.
    /// adopting a child that already has a different owner, or dispatching to a child a group's
    /// stated parent does not actually own). Raised by `crate::os_store::composition_coordinator::CompositionGraph::insert_owns` and
    /// `crate::os_store::composition_coordinator::CompositionCoordinator::dispatch_group`'s phase-1 ownership check.
    OwnershipViolation(String),
    /// 🛂️ A structural failure rejected an operation during
    /// `crate::os_store::composition_coordinator::CompositionCoordinator::dispatch_group`'s phase-1 pass (or the object-safe
    /// `store::SpaceMember::preview_wire`/`dispatch_wire` bridge that pass uses) — the group is
    /// aborted with zero side effects anywhere. Reserved for structural failures only (ticket
    /// `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` §C6): an ordinary
    /// mutation-level rejection now travels as a `MutationMessage` on the op's own
    /// `MutationOutcome`, never through this variant. Additive; not raised anywhere else in this
    /// crate (every other command path reports its own more specific `VcsError` variant).
    ValidationFailed(String),
    /// 🧯️ A `CompositionCoordinator::dispatch_group` call failed AFTER some members were
    /// already applied, and the reverse-order `Undo` compensation pass (see that method's doc
    /// comment) itself failed on at least one member — i.e. the group could not be fully rolled
    /// back. The message embeds a human-readable rollback report (which members compensated
    /// cleanly, which did not, and why) so a caller can surface/log the exact partial state rather
    /// than silently losing it. This is the one path in this crate where a command's Result can
    /// legitimately leave a multi-member gesture inconsistent — every other `VcsError` variant is
    /// raised BEFORE any mutation lands.
    CompensationFailed(String),
    /// 🛑️ A command was rejected WHOLESALE by the authority's own `protocol::MergePolicy`
    /// — nothing in the command was applied (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-
    /// CLASS-CONFLICTS` §C6). Carries the policy that rejected it and every message the rejected
    /// replay produced, so a caller can explain the rejection without re-running anything.
    /// `ValidationFailed` survives ONLY for structural failures now — an ordinary mutation-level
    /// rejection travels through this variant instead.
    Rejected {
        policy: protocol::MergePolicy,
        messages: Vec<protocol::MutationMessage>,
    },
    /// ❓️ `store::ArtifactStore::resolve_conflict` was called with an id that names no
    /// currently-`Open` conflict on this store.
    UnknownConflict(String),
    /// ⌛️ A result computed against store generation `expected_generation` (a finished history replay) was offered
    /// after the store moved on to `generation`; the caller computes it again against the live state.
    Stale { expected_generation: u64, generation: u64 },
    /// 🗂️ A history transition of `kind` was authored on, or offered to, a store whose history `shape` does not hold it —
    /// a config store's history holds undo and redo only. Refused before anything was recorded.
    HistoryShape { shape: protocol::HistoryShape, kind: protocol::HistoryTransitionKind },
    /// 🎞️ The tool transaction `transaction_id` is open on this store: until its commit or abort only its own appends, a
    /// remote ingest and a merge-policy change run — nothing else may interleave with its open edit.
    TransactionOpen { transaction_id: String },
    /// 🕳️ A commit, abort or append named a tool transaction that is not the one open on this store.
    UnknownTransaction(String),
    /// 🧱️ The document's edit history holds `capacity` edits and is full: the edit was refused before anything was recorded.
    HistoryFull { capacity: usize },
    /// 🐢️ A local history step (undo, redo, checkout, alternative switch, checkpoint or supersession) still replays across
    /// turns on this store: no other history step may be authored before it is adopted or discarded.
    HistoryReplaying,
    /// 🐘️ A gesture's edit needs `rows` staged rows (forwards plus inverses) where at most `capacity` are admitted — the one-item
    /// ceiling, or the rows its leaves declare (`x-semio-inverse-rows`, design §20.5). Refused before anything was recorded.
    TooLarge { rows: usize, capacity: usize },
    /// 🧷️ The operation `mutation_id` belongs to the cross-artifact unit `unit`: it planned steps in other documents, or it is
    /// such a step. A unit is superseded in all of its documents at once, which no replica can do yet, so the supersession
    /// was refused before anything was recorded.
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
            _ => None,
        }
    }
}

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
