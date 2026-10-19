//! 🎞️ Persisted History records and canonical fold ownership.
use protocol::ProtocolError;
pub type HistoryEdits=semio_framework_value_persistent::list::PagedList<HistoryEdit,{usize::MAX}>;
#[path="../🔁️fold/🦀️.rs"]
mod retained_fold;
pub use retained_fold::RetainedHistoryFold;
#[path="../🛬️decode/📋️records/📦️paged/👁️source/🦀️.rs"]
mod original_history_edits;
pub use original_history_edits::OriginalHistoryEdits;
#[path="../👁️source/🦀️.rs"]
mod original_history_source;
pub use original_history_source::OriginalHistorySource;

//#region 🔖️Model
// Persisted history is the semantic event log plus this replica's head. Edits (with the alternative
// they were authored on) and structural transitions are shared. `viewer_line` and `viewer_checkpoint`
// are persisted local-only; absent, the head is the canonical trunk tip. Change/checkpoint/alternative
// ledgers and the cursor stay derived by [`HistoryLog::fold`].
#[derive(Clone, Debug, PartialEq, Default, semio_framework_value::RetireOwned)]
pub struct HistoryLog {
    pub doc_id: String,
    pub schema: String,
    pub edits: HistoryEdits,
    /// 🔀️ Structural history steps (`REC_TRANSITION`) — undo/redo, commits, branches,
    /// checkouts and repins — in persisted order; the fold orders them by `(hlt, id)`.
    pub transitions: Vec<HistoryTransitionRecord>,
    /// 🧩️ Composition overlay (`REC_COMPOSITION`): who owns this document and which dialect
    /// it materializes as. Absent for every non-composed document.
    pub composition: Option<HistoryComposition>,
    /// ⚔️ First-class merge conflicts (`REC_CONFLICT`), durable per
    /// `.🧬semio/🦑️repo/🎫️tickets/26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/
    /// 📋️contract-freeze.md` §C7: a `Quarantined` batch rejected outright, or a `Degraded`
    /// accepted-but-messy merge — see `protocol::conflict::ConflictKind`. Empty for the
    /// overwhelming majority of documents; no record is written when empty.
    pub conflicts: Vec<HistoryConflict>,
    /// 👁 Alternative this replica is on (`REC_VIEWER`). `None` is the canonical trunk.
    pub viewer_line: Option<String>,
    /// 👁 Explicit checkpoint this replica is looking at. `None` is the tip of `viewer_line`.
    pub viewer_checkpoint: Option<String>,
}

/// 🔀️ One persisted history transition: the [`protocol::MutationEnvelope`] minus its
/// document id and schema (implied by `REC_DOC` and [`protocol::HISTORY_TRANSITION_SCHEMA`]).
/// `hlt` is `(actor, physical_ms, logical)` like [`HistoryConflict::hlt`]; `payload` is the encoded
/// [`protocol::HistoryTransition`], opaque to this codec. `observed` is the newest foreign operation the author had
/// seen (a `Supersede`'s grading frontier), kept so a reloaded, still unacknowledged transition is announced as authored.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct HistoryTransitionRecord {
    pub id: String,
    pub actor: semio_framework_value::SharedUtf8,
    pub hlt: (u64, u64, u64),
    pub dependencies: Vec<String>,
    pub observed: Option<String>,
    pub payload: Vec<u8>,
}

/// 🧩️ The durable form of a document's composition facts, carried as ONE extension record
/// rather than as new fields on the format-frozen critical `REC_DOC`: an older/foreign reader must
/// be able to skip it without failing the whole file. Checkpoint pins are not stored here — they
/// are facts of `Repin` transitions, derived by [`HistoryLog::fold`].
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::RetireOwned)]
pub struct HistoryComposition {
    /// 🏠️ `(parent_artifact_uri, slot, child_id)` — the child-side ownership stamp.
    pub owner: Option<(String, String, String)>,
    /// 🎯️ `(artifact_kind, standard, subset)` — the dialect this document materializes as.
    pub dialect: Option<(String, String, String)>,
}

/// ⚔️ Durable form of `protocol::conflict::Conflict` (`REC_CONFLICT`): `kind`/`status`
/// are the numeric mirrors of `protocol::conflict::ConflictKind`/`ConflictStatus`
/// (`kind`: 0 = `Quarantined`, 1 = `Degraded`; `status`: 0 = `Open`, 1 = `Accepted`, 2 =
/// `Discarded`) — `envelopes` is populated only for `Quarantined` (opaque, already-serialized
/// `protocol::causal::MutationEnvelope` bytes — this crate never interprets them, same stance
/// as every other opaque payload here), `edit_ids` only for `Degraded`. No `policy` field: a merge
/// policy is local/authority state per the frozen contract, never part of an artifact's shared
/// history.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct HistoryConflict {
    pub id: String,
    pub kind: u8,
    pub status: u8,
    pub actors: Vec<semio_framework_value::SharedUtf8>,
    /// ⏰️ `(actor, physical_ms, logical)` — field order/types mirror
    /// `protocol::ids::HybridLogicalTimestamp` exactly.
    pub hlt: (u64, u64, u64),
    pub edit_ids: Vec<String>,
    pub envelopes: Vec<Vec<u8>>,
    pub messages: Vec<HistoryMessage>,
}

/// 📨️ Durable form of `protocol::mutation::MutationMessage`: `level` is the numeric
/// mirror of `semio_framework_diagnostic::Severity` (`as_u8`/`from_u8`, 0..3), `code` is dict-interned (the
/// frozen nine `mutation.*` codes repeat heavily across one document's history — see
/// `📋️contract-freeze.md` §C2), `message`/`target` are plain strings (English prose / element
/// address, never interned — they vary per occurrence).
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct HistoryMessage {
    pub level: u8,
    pub code: String,
    pub message: String,
    pub target: Vec<String>,
    pub op_index: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct HistoryEdit {
    pub id: String,
    pub actor: Option<semio_framework_value::SharedUtf8>,
    pub started_at: String,
    pub finished_at: Option<String>,
    /// 🏷️ The id of the action or command that authored this edit (mirrors `protocol::mutation::Edit::verb`).
    pub verb: Option<String>,
    /// 🌿️ Authored branch identity, explicitly null for trunk or unknown provenance.
    pub line: Option<String>,
    pub ops: Vec<OpPayload>,
    /// 🔙️ The edit's inverse operations, in apply order (mirrors `protocol::mutation::Edit
    /// ::inverse`). Empty for text-compiled/imported logs — a decoder recomputing them from a
    /// fresh replay never touches this field; when non-empty, `write_backwards_section` persisted
    /// them explicitly (only the `.spr` binary path ever sets this — the `.ops` text mirror stays
    /// forwards-only, see `crate::os_store::text_format::print_document_spr`/`parse_document_spr`).
    pub inverse: Vec<OpPayload>,
    /// 🧮️ Present iff the caller supplied it; absent for text-compiled/imported logs. Not
    /// required for round-trip — a decoder recomputing inverse/meta from a fresh replay never
    /// touches this field.
    pub meta: Option<Vec<HistoryOpMeta>>,
    /// 🛤️ Which undo/redo cursor this edit belongs to, as the document layer's own lane name
    /// (`crate::os_store::store_schema::HistoryLane`'s camelCase word). `None` means the default DOCUMENT lane, so
    /// an ordinary edit costs nothing on the wire. It has to live here and not only on the envelope:
    /// without it a save/load cycle turned every side-lane edit back into a document edit, and the
    /// reloaded store's plain `Undo` then reverted an interaction edit that must never be undone
    /// that way (`history_lane_interaction_entries_survive_owned_document_round_trip`).
    pub lane: Option<String>,
}

/// 🧾️ `binary` carries the `protocol::mutation::OpBinary` encoding of this op when the
/// caller has one (the `.spr` binary path always sets it, and since the binary-only-spr flip
/// this is the ONLY face `.spr` ever carries); `text` is the `OpText::print_op` form, present
/// only when a text-tooling caller supplied it (`.ops` compile, hand-authored logs). Invariant:
/// at least one of `text`/`binary` is `Some` — both `None` is a construction bug, rejected by
/// `write_op_payload`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct OpPayload {
    pub text: Option<String>,
    pub binary: Option<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq, Default, semio_framework_value::RetireOwned)]
pub struct HistoryOpMeta {
    pub op_id: Option<String>,
    pub dependencies: Vec<String>,
    pub base_version: u64,
    pub author_id: Option<semio_framework_value::SharedUtf8>,
    pub hlt: Option<(u64, i64, u64)>,
    pub undo_policy: u8,
    pub payload_hash: Option<[u8; 32]>,
    /// 🧑‍🤝‍🧑️ Durable twin of `protocol::mutation::MutationMeta.group_id` — the composite-
    /// gesture stamp, present iff the op it describes was authored as one member of a multi-
    /// document composite gesture. Dict-interned like `op_id`/`author_id`/`dependencies` (bullet
    /// design point: every sibling member of one composite gesture shares the identical string,
    /// so the dictionary compresses it near-for-free across a whole edit/checkpoint).
    pub group_id: Option<String>,
    /// 🔀️ Durable twin of `protocol::mutation::MutationMeta.origin` — canonical-JSON
    /// encoded (not dict-interned like the id fields above: an origin's `Contributed`/`Transaction`
    /// payload carries structured data, not a short repeated token). `MutationOrigin::Owner`
    /// (`Default`) whenever absent from the byte log, matching `group_id`'s own "absent for logs
    /// predating this field" contract.
    pub origin: protocol::mutation::MutationOrigin,
    /// 📨️ Durable ledger twin of `protocol::mutation::MutationOutcome::messages` — every
    /// diagnostic this op's `diff` raised, persisted rather than recomputed (unlike inverse/meta's
    /// general "a fresh replay never touches this field" contract, messages are NOT reproducible
    /// from a replay alone — they are the durable record of what actually happened at write time).
    /// Dict-interned per-message `code` (bit6 of the same presence byte, same "absent for logs
    /// predating this field" contract as `group_id`/`origin`) — empty `Vec` for logs predating
    /// this field.
    pub messages: Vec<HistoryMessage>,
    /// 🧾️ Durable twin of `protocol::mutation::MutationMeta.transaction` — the committed tool transaction that
    /// authored the op (bit7 of the same presence byte; id and tool dict-interned, since every op of one transaction
    /// repeats them).
    pub transaction: Option<protocol::mutation::TransactionRef>,
}
//#endregion 🔖️Model

//#region 🔖️Fold
impl HistoryTransitionRecord {
    /// 📥️ Strips `envelope`'s document id and schema, keeping every other transition fact.
    pub fn from_envelope(envelope: &protocol::MutationEnvelope) -> Self {
        Self {
            id: envelope.mutation_id.0.clone(),
            actor: envelope.actor.0.clone(),
            hlt: (envelope.timestamp.actor, envelope.timestamp.physical_ms, envelope.timestamp.logical),
            dependencies: envelope.dependencies.iter().map(|dependency| dependency.0.clone()).collect(),
            observed: envelope.observed.as_ref().map(|observed| observed.0.clone()),
            payload: envelope.diff.payload.clone(),
        }
    }

    /// 📤️ The transition envelope of `document_id`: [`protocol::HISTORY_TRANSITION_SCHEMA`]
    /// diff carrying `payload`, empty inverse under the same schema.
    pub fn to_envelope(&self, document_id: &str) -> protocol::MutationEnvelope {
        let schema = protocol::SchemaId(protocol::HISTORY_TRANSITION_SCHEMA.to_string());
        protocol::MutationEnvelope {
            mutation_id: protocol::MutationId(self.id.clone()),
            document_id: protocol::ArtifactId(document_id.to_string()),
            actor: protocol::ActorId(self.actor.clone()),
            dependencies: self.dependencies.iter().cloned().map(protocol::MutationId).collect(),
            observed: self.observed.clone().map(protocol::MutationId),
            target: Vec::new(),
            diff: protocol::ArtifactDiff { schema: schema.clone(), payload: self.payload.clone() },
            inverse: protocol::InverseMutation { schema, payload: Vec::new() },
            timestamp: protocol::HybridLogicalTimestamp { actor: self.hlt.0, physical_ms: self.hlt.1, logical: self.hlt.2 },
            transaction: None, verb: None, line: None,
        }
    }
}

impl HistoryLog {
    /// 🧮️ Folds this log's edits and transitions ([`protocol::fold_history`]) into every
    /// derived history fact and position. An edit's clock is its first operation's `hlt` (zero when
    /// absent); its mutation ids are each operation's `op_id`, else `{edit.id}#{index}` (the
    /// [`protocol::mutation_ids_for_edit`] fallback). Edits owning an operation of a
    /// non-accepted quarantined conflict are excluded.
    pub fn fold(&self) -> Result<protocol::HistoryFold, ProtocolError> {
        use semio_framework_value::{retirement::controlled::ControlledRetirement, retained_clone::RetainedCloneGrant};
        let (fold, transitions, replay_order, conflicts) = protocol::HistoryFoldJob::new(|control| async move { self.fold_controlled(protocol::HistoryShape::Document, &control).await }).finish_cold()?;
        let mut retirement = ControlledRetirement::new((transitions, replay_order, conflicts)).unwrap_or_else(|_| unreachable!("normalized history declares controlled typed ownership"));
        while !retirement.terminal_is_empty() {
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4096, maximum_capacity_bytes: retirement.next_capacity_byte_demand(4096).expect("normalized history constructor demand"), maximum_release_bytes: retirement.next_release_byte_demand().expect("normalized history whole release demand"), maximum_depth: retirement.next_depth_demand().expect("normalized history actual depth demand") };
            let step = retirement.step(grant).expect("normalized cold fold owners have exact retirement");
            assert!(step.progress().fits(grant));
        }
        Ok(fold)
    }

}
//#endregion 🔖️Fold

