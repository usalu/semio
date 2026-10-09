//! 🔀️ History transitions: the structural half of an artifact's semantic event log. An edit's
//! operations travel as ordinary [`super::MutationEnvelope`]s whose `diff` carries the domain op;
//! every other history step (undo, redo, checkpoint commit, alternative branch, checkout, pin)
//! travels as a [`super::MutationEnvelope`] too, tagged with [`HISTORY_TRANSITION_SCHEMA`] and
//! carrying one encoded [`HistoryTransition`]. One causal, HLC-ordered stream therefore holds every
//! semantic change of a document; a replica materializes the document by folding that stream over
//! its genesis snapshot — never by merging another replica's snapshot.
//!
//! Identity across replicas is the per-operation [`crate::ids::MutationId`]: a receiver
//! materializes one local edit per wire operation, so transitions reference operations, never a
//! replica-local edit id. Checkpoint, change and alternative ids are minted once by the author
//! and carried verbatim.

#[path = "🔁️fold/🦀️.rs"]
mod retained_fold;
pub use retained_fold::*;

#[path = "📝️drafts/🦀️.rs"]
mod input_drafts;
pub use input_drafts::HistoryInputDrafts;

use crate::ids::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId, SchemaId};

//#region 🔖️Vocabulary
/// 🏷️ `diff.schema` of every transition envelope. Operation envelopes carry their
/// artifact's own schema, so the tag alone routes an envelope to the transition fold.
pub const HISTORY_TRANSITION_SCHEMA: &str = "semio.history.transition";

/// 🧑‍🎨️ Author stamped on a committed checkpoint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransitionAuthor {
    pub id: String,
    pub name: String,
    pub avatar: Option<String>,
}

/// 🚩️ The facts one checkpoint commit introduces: the change grouping the operations that
/// were uncommitted at the author, and the checkpoint stacking that change onto `parent_id`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransitionCheckpoint {
    pub checkpoint_id: String,
    pub parent_id: Option<String>,
    pub change_id: String,
    pub mutation_ids: Vec<MutationId>,
    pub description: Option<String>,
    pub saved_at: String,
    pub authors: Vec<TransitionAuthor>,
    pub message: Option<String>,
    pub timestamp: String,
    /// 🌿️ The alternative this checkpoint grows. `None` and the trunk id grow the trunk.
    pub line_id: Option<String>,
}

/// 📌️ One owned child's checkpoint pin, as `(child artifact uri, child checkpoint id)`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransitionPin {
    pub child_uri: String,
    pub checkpoint_id: String,
}

/// ♻️ The effective input a [`HistoryTransition::Supersede`] installs for one operation: the
/// artifact aggregate op re-encoded canonically (`schema` names the artifact schema, `payload` holds
/// its `OpBinary` bytes), or a withdrawal that folds the operation as a no-op. The superseded
/// operation itself is never rewritten.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputReplacement {
    Input { schema: String, payload: Vec<u8> },
    Withdrawn,
}

/// 🎯️ One operation whose effective input a supersession replaces.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SupersededInput {
    pub target: MutationId,
    pub replacement: InputReplacement,
}

/// ✏️ An atomic batch of input replacements, effective in every alternative (`scope: None`) or only
/// while the alternative named `scope` is the fold's final alternative.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransitionSupersede {
    pub scope: Option<String>,
    pub inputs: Vec<SupersededInput>,
}

/// 📏️ Codec ceiling of [`TransitionSupersede::scope`] in UTF-8 bytes.
pub const SUPERSEDE_SCOPE_MAX_BYTES: usize = 256;
/// 📏️ Codec ceiling of one [`InputReplacement::Input`] payload in bytes.
pub const SUPERSEDE_PAYLOAD_MAX_BYTES: usize = 262_144;

impl TransitionSupersede {
    /// 🔗️ The superseded operations in declaration order: the transition envelope's `dependencies`.
    pub fn targets(&self) -> Vec<MutationId> {
        self.inputs.iter().map(|input| input.target.clone()).collect()
    }

    /// 🛂️ The codec law every encoded supersession satisfies: at least one input, unique targets,
    /// `scope` at most [`SUPERSEDE_SCOPE_MAX_BYTES`], every payload at most [`SUPERSEDE_PAYLOAD_MAX_BYTES`].
    pub fn validate(&self) -> Result<(), crate::ProtocolError> {
        if self.scope.as_ref().is_some_and(|scope| scope.len() > SUPERSEDE_SCOPE_MAX_BYTES) {
            return Err(malformed(0, format!("supersede scope exceeds {SUPERSEDE_SCOPE_MAX_BYTES} bytes")));
        }
        if self.inputs.is_empty() {
            return Err(malformed(0, "supersede names no input"));
        }
        for (index, input) in self.inputs.iter().enumerate() {
            if self.inputs[..index].iter().any(|earlier| earlier.target == input.target) {
                return Err(malformed(0, format!("supersede repeats target {}", input.target.0)));
            }
            if let InputReplacement::Input { payload, .. } = &input.replacement {
                if payload.len() > SUPERSEDE_PAYLOAD_MAX_BYTES {
                    return Err(malformed(0, format!("supersede payload exceeds {SUPERSEDE_PAYLOAD_MAX_BYTES} bytes")));
                }
            }
        }
        Ok(())
    }
}

/// 🔀️ One structural history step. Every variant is a pure function of the fold state it
/// lands on, so replicas holding the same event set converge regardless of arrival order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HistoryTransition {
    /// ⏪️ The author withdraws its own operations (undo).
    Revert { mutation_ids: Vec<MutationId> },
    /// ⏩️ The author restores operations it reverted earlier (redo).
    Reinstate { mutation_ids: Vec<MutationId> },
    /// 🚩️ A checkpoint commit.
    Commit(TransitionCheckpoint),
    /// 🌿️ A new alternative rooted at `checkpoint_id`. Registration is shared; it does not select any viewer's head.
    Branch { alternative_id: String, name: String, checkpoint_id: String },
    /// 🎯️ Names `checkpoint_id` (and `alternative_id`, if any). A viewer head is local; this event selects none.
    Checkout { checkpoint_id: String, alternative_id: Option<String> },
    /// 🧩️ Re-identifies `checkpoint_id` as `pinned_checkpoint_id` once its composed children's pins are known.
    Repin { checkpoint_id: String, pinned_checkpoint_id: String, pins: Vec<TransitionPin> },
    /// ✏️ Replaces the effective input of existing operations, non-destructively (history editing).
    Supersede(TransitionSupersede),
}

/// 🧭️ The supersession a fold resolved for one operation: which transition installed it, who
/// authored it and when, its scope, and the effective input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EffectiveSupersession {
    pub transition_id: String,
    pub actor: semio_framework_value::SharedUtf8,
    pub timestamp: HybridLogicalTimestamp,
    pub scope: Option<String>,
    pub replacement: InputReplacement,
}

/// 🌉️ Internally tagged on `"kind"` (`input` carries `schema` and `payload` bytes, `withdrawn` nothing).
impl crate::value::ToValue for InputReplacement {
    fn to_value(&self) -> crate::value::DslValue {
        match self {
            InputReplacement::Input { schema, payload } => crate::value::DslValue::object(vec![
                ("kind".to_string(), crate::value::DslValue::String("input".to_string())),
                ("schema".to_string(), crate::value::ToValue::to_value(schema)),
                ("payload".to_string(), crate::value::ToValue::to_value(payload)),
            ]),
            InputReplacement::Withdrawn => crate::value::DslValue::object(vec![("kind".to_string(), crate::value::DslValue::String("withdrawn".to_string()))]),
        }
    }
}
impl crate::value::FromValue for InputReplacement {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected an object for InputReplacement, found {value:?}")));
        };
        let mut kind = None;
        let mut schema = None;
        let mut payload = None;
        for (key, entry) in fields {
            match key.as_str() {
                "kind" => kind = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("kind"))?),
                "schema" => schema = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("schema"))?),
                "payload" => payload = Some(<Vec<u8> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("payload"))?),
                _ => {}
            }
        }
        match kind.as_deref() {
            Some("input") => Ok(InputReplacement::Input {
                schema: schema.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "InputReplacement::Input missing schema"))?,
                payload: payload.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "InputReplacement::Input missing payload"))?,
            }),
            Some("withdrawn") => Ok(InputReplacement::Withdrawn),
            Some(other) => Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("unknown InputReplacement kind `{other}`"))),
            None => Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "InputReplacement missing kind")),
        }
    }
}

impl crate::value::ToValue for EffectiveSupersession {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("transitionId".to_string(), crate::value::ToValue::to_value(&self.transition_id)),
            ("actor".to_string(), crate::value::ToValue::to_value(&self.actor)),
            ("timestamp".to_string(), crate::value::ToValue::to_value(&self.timestamp)),
            ("scope".to_string(), crate::value::ToValue::to_value(&self.scope)),
            ("replacement".to_string(), crate::value::ToValue::to_value(&self.replacement)),
        ])
    }
}
impl crate::value::FromValue for EffectiveSupersession {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected an object for EffectiveSupersession, found {value:?}")));
        };
        let mut transition_id = None;
        let mut actor = None;
        let mut timestamp = None;
        let mut scope = None;
        let mut replacement = None;
        for (key, entry) in fields {
            match key.as_str() {
                "transitionId" => transition_id = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("transitionId"))?),
                "actor" => actor = Some(<semio_framework_value::SharedUtf8 as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("actor"))?),
                "timestamp" => timestamp = Some(<HybridLogicalTimestamp as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("timestamp"))?),
                "scope" => scope = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("scope"))?,
                "replacement" => replacement = Some(<InputReplacement as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("replacement"))?),
                _ => {}
            }
        }
        Ok(EffectiveSupersession {
            transition_id: transition_id.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "EffectiveSupersession missing transitionId"))?,
            actor: actor.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "EffectiveSupersession missing actor"))?,
            timestamp: timestamp.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "EffectiveSupersession missing timestamp"))?,
            scope,
            replacement: replacement.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "EffectiveSupersession missing replacement"))?,
        })
    }
}

/// 🏷️ The kind of a [`HistoryTransition`], named by its wire tag's lower-case spelling.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HistoryTransitionKind {
    Revert,
    Reinstate,
    Commit,
    Branch,
    Checkout,
    Repin,
    Supersede,
}

impl HistoryTransitionKind {
    /// 📚️ Every kind, in wire-tag order.
    pub const ALL: [HistoryTransitionKind; 7] = [Self::Revert, Self::Reinstate, Self::Commit, Self::Branch, Self::Checkout, Self::Repin, Self::Supersede];

    /// 🔤️ The kind's name, as the transition corpus and every twin spell it.
    pub fn name(self) -> &'static str {
        match self {
            Self::Revert => "revert",
            Self::Reinstate => "reinstate",
            Self::Commit => "commit",
            Self::Branch => "branch",
            Self::Checkout => "checkout",
            Self::Repin => "repin",
            Self::Supersede => "supersede",
        }
    }
}

impl HistoryTransition {
    /// 🏷️ This transition's kind.
    pub fn kind(&self) -> HistoryTransitionKind {
        match self {
            Self::Revert { .. } => HistoryTransitionKind::Revert,
            Self::Reinstate { .. } => HistoryTransitionKind::Reinstate,
            Self::Commit(_) => HistoryTransitionKind::Commit,
            Self::Branch { .. } => HistoryTransitionKind::Branch,
            Self::Checkout { .. } => HistoryTransitionKind::Checkout,
            Self::Repin { .. } => HistoryTransitionKind::Repin,
            Self::Supersede(_) => HistoryTransitionKind::Supersede,
        }
    }
}

/// 🗂️ The shape of a history: a `Document` holds every transition; a `Config` holds its edits with undo and redo only —
/// never a checkpoint, an alternative, a pin or a supersession, which a config history does not persist.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum HistoryShape {
    #[default]
    Document,
    Config,
}

impl HistoryShape {
    /// 🛂️ Whether a history of this shape holds a transition of `kind`.
    pub fn admits(self, kind: HistoryTransitionKind) -> bool {
        match self {
            Self::Document => true,
            Self::Config => matches!(kind, HistoryTransitionKind::Revert | HistoryTransitionKind::Reinstate),
        }
    }

    /// 🔤️ The shape's name, as the transition corpus and every twin spell it.
    pub fn name(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::Config => "config",
        }
    }
}
//#endregion 🔖️Vocabulary

//#region 🔖️Codec
fn write_optional_str(out: &mut Vec<u8>, value: &Option<String>) {
    match value {
        Some(value) => {
            out.push(1);
            crate::write_str(out, value);
        }
        None => out.push(0),
    }
}

fn read_u8(bytes: &[u8], pos: &mut usize) -> Result<u8, crate::ProtocolError> {
    let value = *bytes.get(*pos).ok_or_else(|| malformed(*pos, "truncated"))?;
    *pos += 1;
    Ok(value)
}

fn malformed(offset: usize, detail: impl Into<String>) -> crate::ProtocolError {
    crate::ProtocolError::Malformed { what: "history transition", offset: offset as u64, detail: detail.into() }
}

fn write_ids(out: &mut Vec<u8>, ids: &[MutationId]) {
    crate::wire::write_varint_u64(out, ids.len() as u64);
    for id in ids {
        crate::write_str(out, &id.0);
    }
}

/// 🎯️ `tag varint | variant fields in declaration order` — the transition payload bytes. A
/// `Supersede` (tag 6) writes `scope option | input count varint | (target str | replacement u8
/// (0 = input: schema str, payload bytes; 1 = withdrawn))*` and must satisfy
/// [`TransitionSupersede::validate`], which [`decode_history_transition`] enforces.
pub fn encode_history_transition(transition: &HistoryTransition) -> Vec<u8> {
    let mut out = Vec::new();
    match transition {
        HistoryTransition::Revert { mutation_ids } => {
            crate::wire::write_varint_u64(&mut out, 0);
            write_ids(&mut out, mutation_ids);
        }
        HistoryTransition::Reinstate { mutation_ids } => {
            crate::wire::write_varint_u64(&mut out, 1);
            write_ids(&mut out, mutation_ids);
        }
        HistoryTransition::Commit(checkpoint) => {
            crate::wire::write_varint_u64(&mut out, 2);
            crate::write_str(&mut out, &checkpoint.checkpoint_id);
            write_optional_str(&mut out, &checkpoint.parent_id);
            crate::write_str(&mut out, &checkpoint.change_id);
            write_ids(&mut out, &checkpoint.mutation_ids);
            write_optional_str(&mut out, &checkpoint.description);
            crate::write_str(&mut out, &checkpoint.saved_at);
            crate::wire::write_varint_u64(&mut out, checkpoint.authors.len() as u64);
            for author in &checkpoint.authors {
                crate::write_str(&mut out, &author.id);
                crate::write_str(&mut out, &author.name);
                write_optional_str(&mut out, &author.avatar);
            }
            write_optional_str(&mut out, &checkpoint.message);
            crate::write_str(&mut out, &checkpoint.timestamp);
            write_optional_str(&mut out, &checkpoint.line_id);
        }
        HistoryTransition::Branch { alternative_id, name, checkpoint_id } => {
            crate::wire::write_varint_u64(&mut out, 3);
            crate::write_str(&mut out, alternative_id);
            crate::write_str(&mut out, name);
            crate::write_str(&mut out, checkpoint_id);
        }
        HistoryTransition::Checkout { checkpoint_id, alternative_id } => {
            crate::wire::write_varint_u64(&mut out, 4);
            crate::write_str(&mut out, checkpoint_id);
            write_optional_str(&mut out, alternative_id);
        }
        HistoryTransition::Repin { checkpoint_id, pinned_checkpoint_id, pins } => {
            crate::wire::write_varint_u64(&mut out, 5);
            crate::write_str(&mut out, checkpoint_id);
            crate::write_str(&mut out, pinned_checkpoint_id);
            crate::wire::write_varint_u64(&mut out, pins.len() as u64);
            for pin in pins {
                crate::write_str(&mut out, &pin.child_uri);
                crate::write_str(&mut out, &pin.checkpoint_id);
            }
        }
        HistoryTransition::Supersede(supersede) => {
            crate::wire::write_varint_u64(&mut out, 6);
            write_optional_str(&mut out, &supersede.scope);
            crate::wire::write_varint_u64(&mut out, supersede.inputs.len() as u64);
            for input in &supersede.inputs {
                crate::write_str(&mut out, &input.target.0);
                match &input.replacement {
                    InputReplacement::Input { schema, payload } => {
                        out.push(0);
                        crate::write_str(&mut out, schema);
                        crate::write_bytes(&mut out, payload);
                    }
                    InputReplacement::Withdrawn => out.push(1),
                }
            }
        }
    }
    out
}

/// 🎯️ Inverse of [`encode_history_transition`]; refuses trailing bytes.
pub fn decode_history_transition(bytes: &[u8]) -> Result<HistoryTransition, crate::ProtocolError> {
    HistoryFoldJob::new(|control| async move { decode_history_transition_controlled(bytes, &control).await }).finish_cold()
}
//#endregion 🔖️Codec

//#region 🔖️Envelope
/// 🪪️ Content-addressed transition id: `transition-{hex16(blake3(hlc.actor varint | hlc.physical_ms varint |
/// hlc.logical varint | payload bytes))}`. The HLC already carries the author's numeric replica actor; the
/// envelope's free-form actor string is authentication metadata the hub rebinds to the socket subject, so it
/// never enters the address and every replica re-derives the same id from the authored `(hlc, payload)`.
pub fn history_transition_id(timestamp: &HybridLogicalTimestamp, payload: &[u8]) -> MutationId {
    let mut material = Vec::with_capacity(payload.len() + 32);
    crate::wire::write_varint_u64(&mut material, timestamp.actor);
    crate::wire::write_varint_u64(&mut material, timestamp.physical_ms);
    crate::wire::write_varint_u64(&mut material, timestamp.logical);
    crate::write_bytes(&mut material, payload);
    let digest = crate::wire::RecordHasher::hash(&crate::format::Blake3Hasher, &material);
    let mut id = String::with_capacity("transition-".len() + 16);
    id.push_str("transition-");
    for byte in &digest[..8] {
        id.push_str(&format!("{byte:02x}"));
    }
    MutationId(id)
}

/// 🌳️ The id of `document_id`'s trunk — the implicit root line every document starts on, active whenever no branched
/// alternative is: `trunk-{hex16(blake3(str "semio.history.trunk" | str document_id))}` (strings as `varint length |
/// utf-8`). The log never names it: a `Checkout` to the trunk carries no alternative, and the fold resolves the id.
pub fn trunk_alternative_id(document_id: &ArtifactId) -> String {
    let mut material = Vec::with_capacity(document_id.0.len() + 24);
    crate::write_str(&mut material, "semio.history.trunk");
    crate::write_str(&mut material, &document_id.0);
    let digest = crate::wire::RecordHasher::hash(&crate::format::Blake3Hasher, &material);
    let mut id = String::with_capacity("trunk-".len() + 16);
    id.push_str("trunk-");
    for byte in &digest[..8] {
        id.push_str(&format!("{byte:02x}"));
    }
    id
}

/// ✉️ Wraps `transition` as a causal envelope: schema-tagged payload, empty inverse (a
/// transition is undone by a later transition, never by an inverse payload), no target and no
/// transaction. A `Supersede` envelope's `dependencies` are its [`TransitionSupersede::targets`];
/// the conflict target is filled from the replacement op by the store, never at this level.
pub fn history_transition_envelope(transition: &HistoryTransition, document_id: &ArtifactId, actor: &ActorId, dependencies: Vec<MutationId>, timestamp: HybridLogicalTimestamp) -> super::MutationEnvelope {
    let payload = encode_history_transition(transition);
    let mutation_id = history_transition_id(&timestamp, &payload);
    let schema = SchemaId(HISTORY_TRANSITION_SCHEMA.to_string());
    super::MutationEnvelope {
        mutation_id,
        document_id: document_id.clone(),
        actor: actor.clone(),
        dependencies,
        observed: None,
        target: Vec::new(),
        diff: super::ArtifactDiff { schema: schema.clone(), payload },
        inverse: super::InverseMutation { schema, payload: Vec::new() },
        timestamp,
        transaction: None, verb: None, line: None,
    }
}

/// 🔎️ Whether `envelope` is a history transition rather than a domain operation.
pub fn is_history_transition(envelope: &super::MutationEnvelope) -> bool {
    envelope.diff.schema.0 == HISTORY_TRANSITION_SCHEMA
}

/// 📤️ Decodes `envelope`'s transition, or `None` for a domain-operation envelope.
pub fn history_transition_from_envelope(envelope: &super::MutationEnvelope) -> Result<Option<HistoryTransition>, crate::ProtocolError> {
    if !is_history_transition(envelope) {
        return Ok(None);
    }
    decode_history_transition(&envelope.diff.payload).map(Some)
}
//#endregion 🔖️Envelope

//#region 🔖️Fold
/// ✏️ One edit as the fold sees it: its replica-local id, author, HLC and the wire
/// operations it owns.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldEdit {
    pub id: String,
    pub actor: Option<semio_framework_value::SharedUtf8>,
    pub timestamp: HybridLogicalTimestamp,
    pub mutation_ids: Vec<MutationId>,
    /// 🌿️ The alternative this edit was authored on. `None` and the trunk id are the trunk.
    pub line: Option<String>,
}

/// 📦️ A change fact the fold materialized from a [`HistoryTransition::Commit`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldChange {
    pub id: String,
    pub edit_ids: Vec<String>,
    pub description: Option<String>,
    pub saved_at: String,
}

/// 🚩️ A checkpoint fact the fold materialized, with its full change chain and pins.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldCheckpoint {
    pub id: String,
    pub change_ids: Vec<String>,
    pub parent_id: Option<String>,
    pub authors: Vec<TransitionAuthor>,
    pub message: Option<String>,
    pub timestamp: String,
    pub pins: Vec<TransitionPin>,
}

/// 🌿️ An alternative fact with the checkpoint chain the fold grew it to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoldAlternative {
    pub id: String,
    pub name: String,
    pub checkpoint_ids: Vec<String>,
}

/// 🧮️ Everything a document's history projects to: the active edits in HLC order, the redo
/// stack, the undo/redo transitions refused because they name an operation another actor authored
/// (`refused`, transition ids in fold order), the current checkpoint and alternative, every
/// change/checkpoint/alternative fact, and the effective supersession of every superseded operation
/// (every fold site folds an operation's effective input, never its original, once it is listed here).
/// `alternative` is `None` while the folded head is the trunk. The trunk ([`trunk_alternative_id`]) is listed
/// first in `alternatives`, with an empty name the UI localizes, as soon as a commit made on it gives it a chain.
/// Which head was folded is the caller's [`ViewerHead`]; the event log does not store it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HistoryFold {
    pub applied: Vec<String>,
    pub redo: Vec<String>,
    pub refused: Vec<String>,
    pub checkpoint: Option<String>,
    pub alternative: Option<String>,
    pub trunk: String,
    pub changes: Vec<FoldChange>,
    pub checkpoints: Vec<FoldCheckpoint>,
    pub alternatives: Vec<FoldAlternative>,
    pub supersessions: HistoryFoldIndex<MutationId, EffectiveSupersession>,
}

fn fold_error(detail: impl Into<String>) -> crate::ProtocolError {
    crate::ProtocolError::Malformed { what: "history fold", offset: 0, detail: detail.into() }
}

/// 👁 One replica's head. `checkpoint_id: None` is the tip of `line_id` (its committed chain plus
/// uncommitted edits tagged to that line). `Some` is that checkpoint alone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewerHead {
    pub line_id: String,
    pub checkpoint_id: Option<String>,
}

impl ViewerHead {
    /// 🌳️ The trunk of `document_id` at its tip. [`fold_history`] and hub Check In project this head.
    pub fn canonical_trunk(document_id: &ArtifactId) -> Self {
        Self { line_id: trunk_alternative_id(document_id), checkpoint_id: None }
    }
}

/// 🧮️ Folds `edits` and the transition envelopes in `transitions` in `(hlc, id)` order into the
/// canonical trunk projection: the trunk at its tip. A pure function of the event set plus that head.
/// See [`fold_history_for`].
pub fn fold_history(document_id: &ArtifactId, edits: &[FoldEdit], transitions: &[super::MutationEnvelope], excluded: &std::collections::HashSet<String>) -> Result<HistoryFold, crate::ProtocolError> {
    fold_history_for(document_id, edits, transitions, excluded, &ViewerHead::canonical_trunk(document_id))
}

/// 👁 [`fold_history`] for one viewer's [`ViewerHead`]. Branch registration, commits, repins, reverts
/// and supersessions are shared. The head selects `applied`, `checkpoint`, `alternative` and which
/// scoped supersessions are effective. A `Checkout` refuses an unknown checkpoint and otherwise
/// changes nothing. A `Branch` registers the alternative and does not select the head. An uncommitted
/// edit is visible only at the tip of the line it is tagged to (`FoldEdit::line` absent or the trunk id
/// means the trunk).
pub fn fold_history_for(document_id: &ArtifactId, edits: &[FoldEdit], transitions: &[super::MutationEnvelope], excluded: &std::collections::HashSet<String>, head: &ViewerHead) -> Result<HistoryFold, crate::ProtocolError> {
    HistoryFoldJob::new(|control| async move { fold_history_for_controlled(document_id, edits, transitions, &|id| excluded.contains(id), head, &control).await }).finish_cold()
}

//#endregion 🔖️Fold

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
