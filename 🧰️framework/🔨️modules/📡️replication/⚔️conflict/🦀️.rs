//! ⚔️ Protocol conflict layer: first-class merge conflicts, replacing the deleted `protocol_crdt`
//! blind-merge machinery. A conflict is data, not a silently-resolved absorb — either a whole
//! remote batch gets *quarantined* (rejected outright, replayable later once an authority accepts
//! it) or an accepted-but-messy merge gets flagged *degraded* (applied, but worth a human's
//! attention). Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/📋️contract-freeze.md`
//! §C5. `📡️spr/🎮️command` owns `MutationMessage`/`MutationOutcome`/`worst_level`; `📡️spr/🧾️wire`
//! owns `MergePolicy`; this module is the third leg — what an authority DOES once it has both.

#[path = "♻️retirement/🦀️.rs"]
mod ownership_retirement;
pub use ownership_retirement::ProtocolConflictRetirement;

//#region 🔖️ConflictId
/// 🆔️ Content-addressed conflict identity: two authorities independently detecting the
/// identical conflict (same kind, same artifact, same mutation-id set, same HLC) converge on the
/// identical id.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConflictId(pub String);

/// 🌱️ Hand-written, not derived — same DAG reason `MutationMessage`'s hand-written twin in
/// `🎮️mutation/🦀️.rs` documents (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// 26/09/01). `#[serde(transparent)]` means the wire shape is the bare inner value.
impl crate::value::ToValue for ConflictId {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::ToValue::to_value(&self.0)
    }
}
impl crate::value::FromValue for ConflictId {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        Ok(Self(<String as crate::value::FromValue>::from_value(value)?))
    }
}

impl ConflictId {
    /// 🔑️ `blake3(kind-tag || artifact id || sorted mutation ids || hlc)`, using this crate's own
    /// `semio_framework_hash::hash` primitive directly (the same hashing dependency `🌿️vcs`'s
    /// `content_addressed_entity_id` and `📡️spr/🎮️command`'s `descriptor_fingerprint` already use —
    /// no new dependency added).
    pub async fn new(kind: &ConflictKind, artifact_id: &crate::ids::ArtifactId, mutation_ids: &[crate::ids::MutationId], hlc: &crate::ids::HybridLogicalTimestamp) -> Self {
        let mut sorted: Vec<&str> = mutation_ids.iter().map(|id| id.0.as_str()).collect();
        sorted.sort_unstable();

        let mut input = Vec::new();
        input.extend_from_slice(kind.tag().await.as_bytes());
        input.push(0);
        input.extend_from_slice(artifact_id.0.as_bytes());
        input.push(0);
        for id in sorted {
            input.extend_from_slice(id.as_bytes());
            input.push(0);
        }
        input.extend_from_slice(&hlc.physical_ms.to_le_bytes());
        input.extend_from_slice(&hlc.logical.to_le_bytes());
        input.extend_from_slice(&hlc.actor.to_le_bytes());

        let digest = *semio_framework_hash::hash(&input).as_bytes();
        let hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
        Self(format!("conflict-{hex}"))
    }
}
//#endregion 🔖️ConflictId

//#region 🔖️ConflictKind
/// 🚧️ What kind of conflict this is. `Quarantined`: a whole incoming batch was rejected
/// outright by `policy.rejects(worst).await` — nothing in `envelopes` was applied; `resolve_conflict`'s
/// `Accept` replays it under `LaissezFaire`, `Discard` seeds it into the causal DAG as already-seen
/// without ever relaying it. `Degraded`: the batch WAS applied (its worst level was below the
/// policy's reject floor but still `>= Warning`), so `edit_ids` names the already-durable edits worth
/// a human's attention — resolving only acknowledges/dismisses the flag, it never rewrites history.
#[derive(Clone, Debug, PartialEq)]
pub enum ConflictKind {
    Quarantined { envelopes: Vec<crate::MutationEnvelope> },
    Degraded { edit_ids: Vec<String> },
}

/// 🌱️ Hand-written, not derived — same reason as `ConflictId` above. Internally tagged on
/// `"kind"`, mirroring `#[serde(tag = "kind", rename_all = "camelCase")]`.
impl crate::value::ToValue for ConflictKind {
    fn to_value(&self) -> crate::value::DslValue {
        match self {
            ConflictKind::Quarantined { envelopes } => {
                crate::value::DslValue::object(vec![("kind".to_string(), crate::value::DslValue::String("quarantined".to_string())), ("envelopes".to_string(), crate::value::ToValue::to_value(envelopes))])
            }
            ConflictKind::Degraded { edit_ids } => {
                crate::value::DslValue::object(vec![("kind".to_string(), crate::value::DslValue::String("degraded".to_string())), ("editIds".to_string(), crate::value::ToValue::to_value(edit_ids))])
            }
        }
    }
}
impl crate::value::FromValue for ConflictKind {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected an object for ConflictKind, found {value:?}")));
        };
        let get = |key: &str| fields.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let kind = match get("kind") {
            Some(crate::value::DslValue::String(s)) => s,
            _ => return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "ConflictKind missing kind")),
        };
        match kind.as_str() {
            "quarantined" => Ok(ConflictKind::Quarantined {
                envelopes: <Vec<crate::MutationEnvelope> as crate::value::FromValue>::from_value(get("envelopes").ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "ConflictKind.quarantined missing envelopes"))?).map_err(|e| e.under("envelopes"))?,
            }),
            "degraded" => Ok(ConflictKind::Degraded {
                edit_ids: <Vec<String> as crate::value::FromValue>::from_value(get("editIds").ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "ConflictKind.degraded missing editIds"))?).map_err(|e| e.under("editIds"))?,
            }),
            other => Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("unknown ConflictKind kind `{other}`"))),
        }
    }
}

impl ConflictKind {
    async fn tag(&self) -> &'static str {
        match self {
            ConflictKind::Quarantined { .. } => "quarantined",
            ConflictKind::Degraded { .. } => "degraded",
        }
    }
}
//#endregion 🔖️ConflictKind

//#region 🔖️ConflictStatus
/// 🚦️ A conflict's own lifecycle, independent of the `MutationMessage`s it carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConflictStatus {
    Open,
    Accepted,
    Discarded,
}

/// 🌱️ Hand-written, not derived — same reason as `ConflictId` above. A tag-less, unit-only enum
/// with `rename_all = "camelCase"` serializes as its bare lower-camelCase variant name.
impl crate::value::ToValue for ConflictStatus {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::String(match self { ConflictStatus::Open => "open", ConflictStatus::Accepted => "accepted", ConflictStatus::Discarded => "discarded" }.to_string())
    }
}
impl crate::value::FromValue for ConflictStatus {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        match value {
            crate::value::DslValue::String(s) => match s.as_str() {
                "open" => Ok(ConflictStatus::Open),
                "accepted" => Ok(ConflictStatus::Accepted),
                "discarded" => Ok(ConflictStatus::Discarded),
                other => Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("unknown ConflictStatus variant `{other}`"))),
            },
            other => Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

/// ✅️❌️ What a human/authority decided to do with an `Open` conflict.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConflictResolution {
    Accept,
    Discard,
}

/// 🌱️ Hand-written, not derived — same reason as `ConflictStatus` above.
impl crate::value::ToValue for ConflictResolution {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::String(match self { ConflictResolution::Accept => "accept", ConflictResolution::Discard => "discard" }.to_string())
    }
}
impl crate::value::FromValue for ConflictResolution {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        match value {
            crate::value::DslValue::String(s) => match s.as_str() {
                "accept" => Ok(ConflictResolution::Accept),
                "discard" => Ok(ConflictResolution::Discard),
                other => Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("unknown ConflictResolution variant `{other}`"))),
            },
            other => Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}
//#endregion 🔖️ConflictStatus

//#region 🔖️Conflict
/// ⚔️ One first-class conflict: identity, what it is, its lifecycle status, the messages that
/// explain it, who was involved, and when it was detected.
#[derive(Clone, Debug, PartialEq)]
pub struct Conflict {
    pub id: ConflictId,
    pub kind: ConflictKind,
    pub status: ConflictStatus,
    pub messages: Vec<crate::MutationMessage>,
    pub actors: Vec<crate::ids::ActorId>,
    pub timestamp: crate::ids::HybridLogicalTimestamp,
}

/// 🌱️ Hand-written, not derived — same reason as `ConflictId` above.
impl crate::value::ToValue for Conflict {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("id".to_string(), crate::value::ToValue::to_value(&self.id)),
            ("kind".to_string(), crate::value::ToValue::to_value(&self.kind)),
            ("status".to_string(), crate::value::ToValue::to_value(&self.status)),
            ("messages".to_string(), crate::value::ToValue::to_value(&self.messages)),
            ("actors".to_string(), crate::value::ToValue::to_value(&self.actors)),
            ("timestamp".to_string(), crate::value::ToValue::to_value(&self.timestamp)),
        ])
    }
}
impl crate::value::FromValue for Conflict {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected an object for Conflict, found {value:?}")));
        };
        let mut id = None;
        let mut kind = None;
        let mut status = None;
        let mut messages = None;
        let mut actors = None;
        let mut timestamp = None;
        for (key, entry) in fields {
            match key.as_str() {
                "id" => id = Some(<ConflictId as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("id"))?),
                "kind" => kind = Some(<ConflictKind as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("kind"))?),
                "status" => status = Some(<ConflictStatus as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("status"))?),
                "messages" => messages = Some(<Vec<crate::MutationMessage> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("messages"))?),
                "actors" => actors = Some(<Vec<crate::ids::ActorId> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("actors"))?),
                "timestamp" => timestamp = Some(<crate::ids::HybridLogicalTimestamp as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("timestamp"))?),
                _ => {}
            }
        }
        Ok(Conflict {
            id: id.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "Conflict missing id"))?,
            kind: kind.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "Conflict missing kind"))?,
            status: status.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "Conflict missing status"))?,
            messages: messages.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "Conflict missing messages"))?,
            actors: actors.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "Conflict missing actors"))?,
            timestamp: timestamp.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "Conflict missing timestamp"))?,
        })
    }
}
//#endregion 🔖️Conflict

//#region 🔖️Reports
/// 📨️ One edit's worth of `MutationMessage`s — the per-edit unit `MergeReport::replayed`
/// carries and `📡️spr/📜️history`'s durable ledger keys by `edit_id`.
#[derive(Clone, Debug, PartialEq)]
pub struct EditMessages {
    pub edit_id: String,
    pub messages: Vec<crate::MutationMessage>,
}

/// 🌱️ Hand-written, not derived — same reason as `ConflictId` above.
impl crate::value::ToValue for EditMessages {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![("editId".to_string(), crate::value::ToValue::to_value(&self.edit_id)), ("messages".to_string(), crate::value::ToValue::to_value(&self.messages))])
    }
}
impl crate::value::FromValue for EditMessages {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected an object for EditMessages, found {value:?}")));
        };
        let mut edit_id = None;
        let mut messages = None;
        for (key, entry) in fields {
            match key.as_str() {
                "editId" => edit_id = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("editId"))?),
                "messages" => messages = Some(<Vec<crate::MutationMessage> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("messages"))?),
                _ => {}
            }
        }
        Ok(EditMessages {
            edit_id: edit_id.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "EditMessages missing editId"))?,
            messages: messages.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "EditMessages missing messages"))?,
        })
    }
}

/// 📤️ The report a single LOCAL dispatch (one `ArtifactStore::dispatch`-shaped call)
/// produces: the policy it was judged against, the worst level reached, and every message.
#[derive(Clone, Debug, PartialEq)]
pub struct DispatchReport {
    pub policy: crate::MergePolicy,
    pub worst: Option<semio_framework_diagnostic::Severity>,
    pub messages: Vec<crate::MutationMessage>,
}

/// 🌱️ Hand-written, not derived — same reason as `ConflictId` above.
impl crate::value::ToValue for DispatchReport {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("policy".to_string(), crate::value::ToValue::to_value(&self.policy)),
            ("worst".to_string(), crate::value::ToValue::to_value(&self.worst)),
            ("messages".to_string(), crate::value::ToValue::to_value(&self.messages)),
        ])
    }
}
impl crate::value::FromValue for DispatchReport {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected an object for DispatchReport, found {value:?}")));
        };
        let mut policy = None;
        let mut worst = None;
        let mut messages = None;
        for (key, entry) in fields {
            match key.as_str() {
                "policy" => policy = Some(<crate::MergePolicy as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("policy"))?),
                "worst" => worst = <Option<semio_framework_diagnostic::Severity> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("worst"))?,
                "messages" => messages = Some(<Vec<crate::MutationMessage> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("messages"))?),
                _ => {}
            }
        }
        Ok(DispatchReport {
            policy: policy.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "DispatchReport missing policy"))?,
            worst,
            messages: messages.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "DispatchReport missing messages"))?,
        })
    }
}

/// 🔀️ The report one `ingest_remote`/`resolve_conflict` merge
/// produces: whether the incoming batch was accepted, where it landed (`insertion_index` — the
/// position in `applied_edit_ids` the batch's first edit was inserted at, meaningful only when
/// `accepted`), every replayed edit's messages, the worst level across the whole replayed suffix,
/// and the id of a `Conflict` this merge raised, if any (`Quarantined` on reject, `Degraded` on an
/// accepted-but-messy merge).
#[derive(Clone, Debug, PartialEq)]
pub struct MergeReport {
    pub policy: crate::MergePolicy,
    pub accepted: bool,
    pub insertion_index: u32,
    pub replayed: Vec<EditMessages>,
    pub worst: Option<semio_framework_diagnostic::Severity>,
    pub conflict: Option<ConflictId>,
}

/// 🌱️ Hand-written, not derived — same reason as `ConflictId` above.
impl crate::value::ToValue for MergeReport {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("policy".to_string(), crate::value::ToValue::to_value(&self.policy)),
            ("accepted".to_string(), crate::value::ToValue::to_value(&self.accepted)),
            ("insertionIndex".to_string(), crate::value::ToValue::to_value(&self.insertion_index)),
            ("replayed".to_string(), crate::value::ToValue::to_value(&self.replayed)),
            ("worst".to_string(), crate::value::ToValue::to_value(&self.worst)),
            ("conflict".to_string(), crate::value::ToValue::to_value(&self.conflict)),
        ])
    }
}
impl crate::value::FromValue for MergeReport {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected an object for MergeReport, found {value:?}")));
        };
        let mut policy = None;
        let mut accepted = None;
        let mut insertion_index = None;
        let mut replayed = None;
        let mut worst = None;
        let mut conflict = None;
        for (key, entry) in fields {
            match key.as_str() {
                "policy" => policy = Some(<crate::MergePolicy as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("policy"))?),
                "accepted" => accepted = Some(<bool as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("accepted"))?),
                "insertionIndex" => insertion_index = Some(<u32 as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("insertionIndex"))?),
                "replayed" => replayed = Some(<Vec<EditMessages> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("replayed"))?),
                "worst" => worst = <Option<semio_framework_diagnostic::Severity> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("worst"))?,
                "conflict" => conflict = <Option<ConflictId> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("conflict"))?,
                _ => {}
            }
        }
        Ok(MergeReport {
            policy: policy.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MergeReport missing policy"))?,
            accepted: accepted.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MergeReport missing accepted"))?,
            insertion_index: insertion_index.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MergeReport missing insertionIndex"))?,
            replayed: replayed.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MergeReport missing replayed"))?,
            worst,
            conflict,
        })
    }
}

/// 🔬️ One operation's result in a Report-mode replay, keyed by its replica-independent
/// [`crate::ids::MutationId`] (`edit_id`/`op_index` locate it in this replica's ledger only):
/// the worst level and every message it raised (`worst: None` is success), and whether its
/// effective input is a supersession (`superseded`) that withdraws it (`withdrawn`).
#[derive(Clone, Debug, PartialEq)]
pub struct MutationReplayOutcome {
    pub mutation_id: crate::ids::MutationId,
    pub edit_id: String,
    pub op_index: u32,
    pub worst: Option<semio_framework_diagnostic::Severity>,
    pub messages: Vec<crate::MutationMessage>,
    pub superseded: bool,
    pub withdrawn: bool,
}

impl crate::value::ToValue for MutationReplayOutcome {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("mutationId".to_string(), crate::value::ToValue::to_value(&self.mutation_id)),
            ("editId".to_string(), crate::value::ToValue::to_value(&self.edit_id)),
            ("opIndex".to_string(), crate::value::ToValue::to_value(&self.op_index)),
            ("worst".to_string(), crate::value::ToValue::to_value(&self.worst)),
            ("messages".to_string(), crate::value::ToValue::to_value(&self.messages)),
            ("superseded".to_string(), crate::value::ToValue::to_value(&self.superseded)),
            ("withdrawn".to_string(), crate::value::ToValue::to_value(&self.withdrawn)),
        ])
    }
}
impl crate::value::FromValue for MutationReplayOutcome {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected an object for MutationReplayOutcome, found {value:?}")));
        };
        let mut mutation_id = None;
        let mut edit_id = None;
        let mut op_index = None;
        let mut worst = None;
        let mut messages = None;
        let mut superseded = None;
        let mut withdrawn = None;
        for (key, entry) in fields {
            match key.as_str() {
                "mutationId" => mutation_id = Some(<crate::ids::MutationId as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("mutationId"))?),
                "editId" => edit_id = Some(<String as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("editId"))?),
                "opIndex" => op_index = Some(<u32 as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("opIndex"))?),
                "worst" => worst = <Option<semio_framework_diagnostic::Severity> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("worst"))?,
                "messages" => messages = Some(<Vec<crate::MutationMessage> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("messages"))?),
                "superseded" => superseded = Some(<bool as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("superseded"))?),
                "withdrawn" => withdrawn = Some(<bool as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("withdrawn"))?),
                _ => {}
            }
        }
        Ok(MutationReplayOutcome {
            mutation_id: mutation_id.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MutationReplayOutcome missing mutationId"))?,
            edit_id: edit_id.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MutationReplayOutcome missing editId"))?,
            op_index: op_index.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MutationReplayOutcome missing opIndex"))?,
            worst,
            messages: messages.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MutationReplayOutcome missing messages"))?,
            superseded: superseded.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MutationReplayOutcome missing superseded"))?,
            withdrawn: withdrawn.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "MutationReplayOutcome missing withdrawn"))?,
        })
    }
}

/// 📋️ A policy-independent Report-mode suffix replay: the applied position it started from, one
/// outcome per replayed operation in applied order, and the worst level across all of them.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReplayReport {
    pub from_position: u32,
    pub outcomes: Vec<MutationReplayOutcome>,
    pub worst: Option<semio_framework_diagnostic::Severity>,
}

impl ReplayReport {
    /// 🚧️ Whether any replayed operation reached `Error` or `Fatal` (the `MergePolicy::Normal` floor
    /// hub check-in replays under): such a history edit cannot be finalized.
    pub fn blocks_finalize(&self) -> bool {
        self.outcomes.iter().any(|outcome| outcome.worst.is_some_and(|worst| crate::MergePolicy::Normal.rejects(worst)))
    }
}

impl crate::value::ToValue for ReplayReport {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::object(vec![
            ("fromPosition".to_string(), crate::value::ToValue::to_value(&self.from_position)),
            ("outcomes".to_string(), crate::value::ToValue::to_value(&self.outcomes)),
            ("worst".to_string(), crate::value::ToValue::to_value(&self.worst)),
        ])
    }
}
impl crate::value::FromValue for ReplayReport {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, format!("expected an object for ReplayReport, found {value:?}")));
        };
        let mut from_position = None;
        let mut outcomes = None;
        let mut worst = None;
        for (key, entry) in fields {
            match key.as_str() {
                "fromPosition" => from_position = Some(<u32 as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("fromPosition"))?),
                "outcomes" => outcomes = Some(<Vec<MutationReplayOutcome> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("outcomes"))?),
                "worst" => worst = <Option<semio_framework_diagnostic::Severity> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("worst"))?,
                _ => {}
            }
        }
        Ok(ReplayReport {
            from_position: from_position.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "ReplayReport missing fromPosition"))?,
            outcomes: outcomes.ok_or_else(|| crate::value::ValueError::new(crate::value::ValueRefusalKind::InvalidValue, "ReplayReport missing outcomes"))?,
            worst,
        })
    }
}
//#endregion 🔖️Reports

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path="♻️report/🦀️.rs"]
mod report_retirement;
