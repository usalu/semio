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
    pub actor: String,
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
            return Err(crate::value::ValueError::new(format!("expected an object for InputReplacement, found {value:?}")));
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
                schema: schema.ok_or_else(|| crate::value::ValueError::new("InputReplacement::Input missing schema"))?,
                payload: payload.ok_or_else(|| crate::value::ValueError::new("InputReplacement::Input missing payload"))?,
            }),
            Some("withdrawn") => Ok(InputReplacement::Withdrawn),
            Some(other) => Err(crate::value::ValueError::new(format!("unknown InputReplacement kind `{other}`"))),
            None => Err(crate::value::ValueError::new("InputReplacement missing kind")),
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
            return Err(crate::value::ValueError::new(format!("expected an object for EffectiveSupersession, found {value:?}")));
        };
        let mut transition_id = None;
        let mut actor = None;
        let mut timestamp = None;
        let mut scope = None;
        let mut replacement = None;
        for (key, entry) in fields {
            match key.as_str() {
                "transitionId" => transition_id = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("transitionId"))?),
                "actor" => actor = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("actor"))?),
                "timestamp" => timestamp = Some(<HybridLogicalTimestamp as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("timestamp"))?),
                "scope" => scope = <Option<String> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("scope"))?,
                "replacement" => replacement = Some(<InputReplacement as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("replacement"))?),
                _ => {}
            }
        }
        Ok(EffectiveSupersession {
            transition_id: transition_id.ok_or_else(|| crate::value::ValueError::new("EffectiveSupersession missing transitionId"))?,
            actor: actor.ok_or_else(|| crate::value::ValueError::new("EffectiveSupersession missing actor"))?,
            timestamp: timestamp.ok_or_else(|| crate::value::ValueError::new("EffectiveSupersession missing timestamp"))?,
            scope,
            replacement: replacement.ok_or_else(|| crate::value::ValueError::new("EffectiveSupersession missing replacement"))?,
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

fn read_optional_str(bytes: &[u8], pos: &mut usize) -> Result<Option<String>, crate::ProtocolError> {
    match read_u8(bytes, pos)? {
        0 => Ok(None),
        1 => Ok(Some(crate::read_str(bytes, pos)?)),
        other => Err(malformed(*pos, format!("invalid option tag {other}"))),
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

fn read_ids(bytes: &[u8], pos: &mut usize) -> Result<Vec<MutationId>, crate::ProtocolError> {
    let count = crate::wire::read_varint_u64(bytes, pos)?;
    if count > (bytes.len() - (*pos).min(bytes.len())) as u64 {
        return Err(malformed(*pos, "id count exceeds payload"));
    }
    let mut ids = Vec::with_capacity(count as usize);
    for _ in 0..count {
        ids.push(MutationId(crate::read_str(bytes, pos)?));
    }
    Ok(ids)
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

fn read_supersede(bytes: &[u8], pos: &mut usize) -> Result<TransitionSupersede, crate::ProtocolError> {
    let scope = read_optional_str(bytes, pos)?;
    let input_count = crate::wire::read_varint_u64(bytes, pos)?;
    if input_count > (bytes.len() - (*pos).min(bytes.len())) as u64 {
        return Err(malformed(*pos, "input count exceeds payload"));
    }
    let mut inputs = Vec::with_capacity(input_count as usize);
    for _ in 0..input_count {
        let target = MutationId(crate::read_str(bytes, pos)?);
        let replacement = match read_u8(bytes, pos)? {
            0 => {
                let schema = crate::read_str(bytes, pos)?;
                let length_at = *pos;
                let length = crate::wire::read_varint_u64(bytes, pos)?;
                if length > SUPERSEDE_PAYLOAD_MAX_BYTES as u64 {
                    return Err(malformed(length_at, format!("supersede payload exceeds {SUPERSEDE_PAYLOAD_MAX_BYTES} bytes")));
                }
                *pos = length_at;
                InputReplacement::Input { schema, payload: crate::read_bytes(bytes, pos)? }
            }
            1 => InputReplacement::Withdrawn,
            other => return Err(malformed(*pos - 1, format!("invalid replacement tag {other}"))),
        };
        inputs.push(SupersededInput { target, replacement });
    }
    let supersede = TransitionSupersede { scope, inputs };
    supersede.validate()?;
    Ok(supersede)
}

/// 🎯️ Inverse of [`encode_history_transition`]; refuses trailing bytes.
pub fn decode_history_transition(bytes: &[u8]) -> Result<HistoryTransition, crate::ProtocolError> {
    let mut pos = 0usize;
    let transition = match crate::wire::read_varint_u64(bytes, &mut pos)? {
        0 => HistoryTransition::Revert { mutation_ids: read_ids(bytes, &mut pos)? },
        1 => HistoryTransition::Reinstate { mutation_ids: read_ids(bytes, &mut pos)? },
        2 => {
            let checkpoint_id = crate::read_str(bytes, &mut pos)?;
            let parent_id = read_optional_str(bytes, &mut pos)?;
            let change_id = crate::read_str(bytes, &mut pos)?;
            let mutation_ids = read_ids(bytes, &mut pos)?;
            let description = read_optional_str(bytes, &mut pos)?;
            let saved_at = crate::read_str(bytes, &mut pos)?;
            let author_count = crate::wire::read_varint_u64(bytes, &mut pos)?;
            if author_count > (bytes.len() - pos.min(bytes.len())) as u64 {
                return Err(malformed(pos, "author count exceeds payload"));
            }
            let mut authors = Vec::with_capacity(author_count as usize);
            for _ in 0..author_count {
                authors.push(TransitionAuthor { id: crate::read_str(bytes, &mut pos)?, name: crate::read_str(bytes, &mut pos)?, avatar: read_optional_str(bytes, &mut pos)? });
            }
            let message = read_optional_str(bytes, &mut pos)?;
            let timestamp = crate::read_str(bytes, &mut pos)?;
            let line_id = read_optional_str(bytes, &mut pos)?;
            HistoryTransition::Commit(TransitionCheckpoint { checkpoint_id, parent_id, change_id, mutation_ids, description, saved_at, authors, message, timestamp, line_id })
        }
        3 => HistoryTransition::Branch { alternative_id: crate::read_str(bytes, &mut pos)?, name: crate::read_str(bytes, &mut pos)?, checkpoint_id: crate::read_str(bytes, &mut pos)? },
        4 => HistoryTransition::Checkout { checkpoint_id: crate::read_str(bytes, &mut pos)?, alternative_id: read_optional_str(bytes, &mut pos)? },
        5 => {
            let checkpoint_id = crate::read_str(bytes, &mut pos)?;
            let pinned_checkpoint_id = crate::read_str(bytes, &mut pos)?;
            let pin_count = crate::wire::read_varint_u64(bytes, &mut pos)?;
            if pin_count > (bytes.len() - pos.min(bytes.len())) as u64 {
                return Err(malformed(pos, "pin count exceeds payload"));
            }
            let mut pins = Vec::with_capacity(pin_count as usize);
            for _ in 0..pin_count {
                pins.push(TransitionPin { child_uri: crate::read_str(bytes, &mut pos)?, checkpoint_id: crate::read_str(bytes, &mut pos)? });
            }
            HistoryTransition::Repin { checkpoint_id, pinned_checkpoint_id, pins }
        }
        6 => HistoryTransition::Supersede(read_supersede(bytes, &mut pos)?),
        other => return Err(malformed(0, format!("unknown transition tag {other}"))),
    };
    if pos != bytes.len() {
        return Err(malformed(pos, "trailing bytes"));
    }
    Ok(transition)
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
    pub actor: Option<String>,
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
    pub supersessions: std::collections::BTreeMap<MutationId, EffectiveSupersession>,
}

enum FoldEvent<'a> {
    Edit(&'a FoldEdit),
    Transition { id: &'a str, actor: &'a str, timestamp: HybridLogicalTimestamp, transition: HistoryTransition },
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
    let mut owners: std::collections::HashMap<&str, &str> = std::collections::HashMap::new();
    let mut authors: std::collections::HashMap<&str, Option<&str>> = std::collections::HashMap::new();
    for edit in edits {
        authors.insert(edit.id.as_str(), edit.actor.as_deref());
        for mutation_id in &edit.mutation_ids {
            owners.insert(mutation_id.0.as_str(), edit.id.as_str());
        }
    }
    let mut events: Vec<((u64, u64, u64), &str, FoldEvent<'_>)> = Vec::with_capacity(edits.len() + transitions.len());
    for edit in edits {
        events.push((edit.timestamp.cmp_key(), edit.id.as_str(), FoldEvent::Edit(edit)));
    }
    let mut identities: std::collections::HashSet<&str> = edits.iter().map(|edit| edit.id.as_str()).collect();
    if identities.len() != edits.len() {
        return Err(fold_error("history repeats an edit"));
    }
    for envelope in transitions {
        if !identities.insert(envelope.mutation_id.0.as_str()) {
            return Err(fold_error(format!("history repeats transition {}", envelope.mutation_id.0)));
        }
        let transition = history_transition_from_envelope(envelope)?.ok_or_else(|| fold_error(format!("{} is not a history transition", envelope.mutation_id.0)))?;
        events.push((envelope.timestamp.cmp_key(), envelope.mutation_id.0.as_str(), FoldEvent::Transition { id: envelope.mutation_id.0.as_str(), actor: envelope.actor.0.as_str(), timestamp: envelope.timestamp, transition }));
    }
    events.sort_by(|left, right| (left.0, left.1).cmp(&(right.0, right.1)));
    let owned = |mutation_ids: &[MutationId]| -> Result<Vec<String>, crate::ProtocolError> {
        let mut edit_ids: Vec<String> = Vec::with_capacity(mutation_ids.len());
        for mutation_id in mutation_ids {
            let edit_id = owners.get(mutation_id.0.as_str()).ok_or_else(|| fold_error(format!("transition references unknown operation {}", mutation_id.0)))?;
            if !edit_ids.iter().any(|known| known == edit_id) {
                edit_ids.push((*edit_id).to_string());
            }
        }
        Ok(edit_ids)
    };
    let foreign = |edit_ids: &[String], actor: &str| edit_ids.iter().any(|edit_id| authors.get(edit_id.as_str()).copied().flatten().is_some_and(|author| author != actor));
    let mut fold = HistoryFold { trunk: trunk_alternative_id(document_id), ..HistoryFold::default() };
    let mut trunk_chain: Vec<String> = Vec::new();
    let mut active: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut superseding: Vec<(MutationId, EffectiveSupersession)> = Vec::new();
    let mut renamed: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for (_, _, event) in events {
        match event {
            FoldEvent::Edit(edit) => {
                if excluded.contains(&edit.id) {
                    continue;
                }
                active.insert(edit.id.clone());
                fold.redo.retain(|redo| authors.get(redo.as_str()).copied().flatten() != edit.actor.as_deref());
            }
            FoldEvent::Transition { id, actor, transition: HistoryTransition::Revert { mutation_ids }, .. } => {
                let edit_ids = owned(&mutation_ids)?;
                if foreign(&edit_ids, actor) {
                    fold.refused.push(id.to_string());
                    continue;
                }
                for edit_id in edit_ids {
                    if active.remove(&edit_id) {
                        fold.redo.push(edit_id);
                    }
                }
            }
            FoldEvent::Transition { id, actor, transition: HistoryTransition::Reinstate { mutation_ids }, .. } => {
                let edit_ids = owned(&mutation_ids)?;
                if foreign(&edit_ids, actor) {
                    fold.refused.push(id.to_string());
                    continue;
                }
                for edit_id in edit_ids {
                    if let Some(position) = fold.redo.iter().position(|redo| *redo == edit_id) {
                        fold.redo.remove(position);
                        active.insert(edit_id);
                    }
                }
            }
            FoldEvent::Transition { transition: HistoryTransition::Commit(checkpoint), .. } => {
                let mut change_ids = match &checkpoint.parent_id {
                    Some(parent_id) => fold.checkpoints.iter().find(|known| known.id == *parent_id).ok_or_else(|| fold_error(format!("checkpoint {} names unknown parent {parent_id}", checkpoint.checkpoint_id)))?.change_ids.clone(),
                    None => Vec::new(),
                };
                change_ids.push(checkpoint.change_id.clone());
                fold.changes.push(FoldChange { id: checkpoint.change_id, edit_ids: owned(&checkpoint.mutation_ids)?, description: checkpoint.description, saved_at: checkpoint.saved_at });
                fold.checkpoints.push(FoldCheckpoint { id: checkpoint.checkpoint_id.clone(), change_ids, parent_id: checkpoint.parent_id, authors: checkpoint.authors, message: checkpoint.message, timestamp: checkpoint.timestamp, pins: Vec::new() });
                let named = checkpoint.line_id.as_ref().filter(|line_id| *line_id != &fold.trunk);
                match named {
                    Some(line_id) => {
                        let alternative = fold.alternatives.iter_mut().find(|alternative| alternative.id == *line_id).ok_or_else(|| fold_error(format!("commit {} names unknown alternative {line_id}", checkpoint.checkpoint_id)))?;
                        alternative.checkpoint_ids.push(checkpoint.checkpoint_id.clone());
                    }
                    None => trunk_chain.push(checkpoint.checkpoint_id.clone()),
                }
            }
            FoldEvent::Transition { transition: HistoryTransition::Branch { alternative_id, name, checkpoint_id }, .. } => {
                if alternative_id == fold.trunk {
                    return Err(fold_error(format!("branch claims the trunk alternative {alternative_id}")));
                }
                if !fold.checkpoints.iter().any(|known| known.id == checkpoint_id) {
                    return Err(fold_error(format!("branch names unknown checkpoint {checkpoint_id}")));
                }
                fold.alternatives.push(FoldAlternative { id: alternative_id, name, checkpoint_ids: vec![checkpoint_id] });
            }
            FoldEvent::Transition { transition: HistoryTransition::Checkout { checkpoint_id, .. }, .. } => {
                if !fold.checkpoints.iter().any(|known| known.id == checkpoint_id) {
                    return Err(fold_error(format!("checkout names unknown checkpoint {checkpoint_id}")));
                }
            }
            FoldEvent::Transition { transition: HistoryTransition::Repin { checkpoint_id, pinned_checkpoint_id, pins }, .. } => {
                let checkpoint = fold.checkpoints.iter_mut().find(|known| known.id == checkpoint_id).ok_or_else(|| fold_error(format!("repin names unknown checkpoint {checkpoint_id}")))?;
                checkpoint.id = pinned_checkpoint_id.clone();
                checkpoint.pins = pins;
                for id in fold.alternatives.iter_mut().flat_map(|alternative| alternative.checkpoint_ids.iter_mut()).chain(trunk_chain.iter_mut()) {
                    if *id == checkpoint_id {
                        *id = pinned_checkpoint_id.clone();
                    }
                }
                renamed.insert(checkpoint_id, pinned_checkpoint_id);
            }
            FoldEvent::Transition { id, actor, timestamp, transition: HistoryTransition::Supersede(supersede) } => {
                owned(&supersede.targets())?;
                for input in supersede.inputs {
                    superseding.push((input.target, EffectiveSupersession { transition_id: id.to_string(), actor: actor.to_string(), timestamp, scope: supersede.scope.clone(), replacement: input.replacement }));
                }
            }
        }
    }
    let on_trunk = head.line_id == fold.trunk;
    let chain = if on_trunk {
        trunk_chain.clone()
    } else {
        fold.alternatives.iter().find(|alternative| alternative.id == head.line_id).ok_or_else(|| fold_error(format!("viewer head names unknown alternative {}", head.line_id)))?.checkpoint_ids.clone()
    };
    let mut viewed = head.checkpoint_id.clone();
    if let Some(id) = &mut viewed {
        let mut guard = 0u8;
        while let Some(next) = renamed.get(id).cloned() {
            *id = next;
            guard += 1;
            if guard == 64 {
                return Err(fold_error("repin cycle"));
            }
        }
        if !chain.iter().any(|known| known == id) {
            return Err(fold_error(format!("viewer head names unknown checkpoint {id}")));
        }
    }
    let at_tip = viewed.is_none();
    let checkpoint_id = viewed.or_else(|| chain.last().cloned());
    let mut visible: std::collections::HashSet<String> = std::collections::HashSet::new();
    if let Some(id) = &checkpoint_id {
        let checkpoint = fold.checkpoints.iter().find(|known| known.id == *id).ok_or_else(|| fold_error(format!("viewer head names unknown checkpoint {id}")))?;
        for change_id in &checkpoint.change_ids {
            let change = fold.changes.iter().find(|change| change.id == *change_id).ok_or_else(|| fold_error(format!("checkpoint {id} names unknown change {change_id}")))?;
            visible.extend(change.edit_ids.iter().filter(|edit_id| active.contains(*edit_id)).cloned());
        }
    }
    if at_tip {
        let mut committed: std::collections::HashSet<&str> = std::collections::HashSet::new();
        for change in &fold.changes {
            committed.extend(change.edit_ids.iter().map(String::as_str));
        }
        for edit in edits {
            if !active.contains(&edit.id) || committed.contains(edit.id.as_str()) {
                continue;
            }
            let on_line = match &edit.line {
                Some(line) if line != &fold.trunk => line == &head.line_id,
                _ => on_trunk,
            };
            if on_line {
                visible.insert(edit.id.clone());
            }
        }
    }
    for (target, supersession) in superseding {
        if supersession.scope.as_ref().is_none_or(|scope| *scope == head.line_id) {
            fold.supersessions.insert(target, supersession);
        }
    }
    if !trunk_chain.is_empty() {
        fold.alternatives.insert(0, FoldAlternative { id: fold.trunk.clone(), name: String::new(), checkpoint_ids: trunk_chain });
    }
    let mut ordered: Vec<&FoldEdit> = edits.iter().filter(|edit| visible.contains(&edit.id)).collect();
    ordered.sort_by(|left, right| (left.timestamp.cmp_key(), left.id.as_str()).cmp(&(right.timestamp.cmp_key(), right.id.as_str())));
    fold.applied = ordered.into_iter().map(|edit| edit.id.clone()).collect();
    fold.checkpoint = checkpoint_id;
    fold.alternative = if on_trunk { None } else { Some(head.line_id.clone()) };
    Ok(fold)
}

//#endregion 🔖️Fold

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
