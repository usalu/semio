//! ⚔️ Protocol conflict layer: first-class merge conflicts, replacing the deleted `protocol_crdt`
//! blind-merge machinery. A conflict is data, not a silently-resolved absorb — either a whole
//! remote batch gets *quarantined* (rejected outright, replayable later once an authority accepts
//! it) or an accepted-but-messy merge gets flagged *degraded* (applied, but worth a human's
//! attention). Frozen contract:
//! `.🦑️repo/🎫️tickets/26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/📋️contract-freeze.md`
//! §C5. `📡️spr/🎮️command` owns `MutationMessage`/`MutationOutcome`/`worst_level`; `📡️spr/🧾️wire`
//! owns `MergePolicy`; this module is the third leg — what an authority DOES once it has both.

//#region 🔖️ConflictId
/// @emoji 🆔️ Content-addressed conflict identity: two authorities independently detecting the
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
/// @emoji 🚧️ What kind of conflict this is. `Quarantined`: a whole incoming batch was rejected
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
            return Err(crate::value::ValueError::new(format!("expected an object for ConflictKind, found {value:?}")));
        };
        let get = |key: &str| fields.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let kind = match get("kind") {
            Some(crate::value::DslValue::String(s)) => s,
            _ => return Err(crate::value::ValueError::new("ConflictKind missing kind")),
        };
        match kind.as_str() {
            "quarantined" => Ok(ConflictKind::Quarantined {
                envelopes: <Vec<crate::MutationEnvelope> as crate::value::FromValue>::from_value(get("envelopes").ok_or_else(|| crate::value::ValueError::new("ConflictKind.quarantined missing envelopes"))?).map_err(|e| e.under("envelopes"))?,
            }),
            "degraded" => Ok(ConflictKind::Degraded {
                edit_ids: <Vec<String> as crate::value::FromValue>::from_value(get("editIds").ok_or_else(|| crate::value::ValueError::new("ConflictKind.degraded missing editIds"))?).map_err(|e| e.under("editIds"))?,
            }),
            other => Err(crate::value::ValueError::new(format!("unknown ConflictKind kind `{other}`"))),
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
/// @emoji 🚦️ A conflict's own lifecycle, independent of the `MutationMessage`s it carries.
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
                other => Err(crate::value::ValueError::new(format!("unknown ConflictStatus variant `{other}`"))),
            },
            other => Err(crate::value::ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }
}

/// @emoji ✅️❌️ What a human/authority decided to do with an `Open` conflict.
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
                other => Err(crate::value::ValueError::new(format!("unknown ConflictResolution variant `{other}`"))),
            },
            other => Err(crate::value::ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }
}
//#endregion 🔖️ConflictStatus

//#region 🔖️Conflict
/// @emoji ⚔️ One first-class conflict: identity, what it is, its lifecycle status, the messages that
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
            return Err(crate::value::ValueError::new(format!("expected an object for Conflict, found {value:?}")));
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
            id: id.ok_or_else(|| crate::value::ValueError::new("Conflict missing id"))?,
            kind: kind.ok_or_else(|| crate::value::ValueError::new("Conflict missing kind"))?,
            status: status.ok_or_else(|| crate::value::ValueError::new("Conflict missing status"))?,
            messages: messages.ok_or_else(|| crate::value::ValueError::new("Conflict missing messages"))?,
            actors: actors.ok_or_else(|| crate::value::ValueError::new("Conflict missing actors"))?,
            timestamp: timestamp.ok_or_else(|| crate::value::ValueError::new("Conflict missing timestamp"))?,
        })
    }
}
//#endregion 🔖️Conflict

//#region 🔖️Reports
/// @emoji 📨️ One edit's worth of `MutationMessage`s — the per-edit unit `MergeReport::replayed`
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
            return Err(crate::value::ValueError::new(format!("expected an object for EditMessages, found {value:?}")));
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
            edit_id: edit_id.ok_or_else(|| crate::value::ValueError::new("EditMessages missing editId"))?,
            messages: messages.ok_or_else(|| crate::value::ValueError::new("EditMessages missing messages"))?,
        })
    }
}

/// @emoji 📤️ The report a single LOCAL dispatch (one `ArtifactStore::dispatch`-shaped call)
/// produces: the policy it was judged against, the worst level reached, and every message.
#[derive(Clone, Debug, PartialEq)]
pub struct DispatchReport {
    pub policy: crate::MergePolicy,
    pub worst: Option<crate::diagnostic::Severity>,
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
            return Err(crate::value::ValueError::new(format!("expected an object for DispatchReport, found {value:?}")));
        };
        let mut policy = None;
        let mut worst = None;
        let mut messages = None;
        for (key, entry) in fields {
            match key.as_str() {
                "policy" => policy = Some(<crate::MergePolicy as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("policy"))?),
                "worst" => worst = <Option<crate::diagnostic::Severity> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("worst"))?,
                "messages" => messages = Some(<Vec<crate::MutationMessage> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("messages"))?),
                _ => {}
            }
        }
        Ok(DispatchReport {
            policy: policy.ok_or_else(|| crate::value::ValueError::new("DispatchReport missing policy"))?,
            worst,
            messages: messages.ok_or_else(|| crate::value::ValueError::new("DispatchReport missing messages"))?,
        })
    }
}

/// @emoji 🔀️ The report one `ingest_remote`/`merge_remote_snapshot`/`resolve_conflict` merge
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
    pub worst: Option<crate::diagnostic::Severity>,
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
            return Err(crate::value::ValueError::new(format!("expected an object for MergeReport, found {value:?}")));
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
                "worst" => worst = <Option<crate::diagnostic::Severity> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("worst"))?,
                "conflict" => conflict = <Option<ConflictId> as crate::value::FromValue>::from_value(entry).map_err(|e| e.under("conflict"))?,
                _ => {}
            }
        }
        Ok(MergeReport {
            policy: policy.ok_or_else(|| crate::value::ValueError::new("MergeReport missing policy"))?,
            accepted: accepted.ok_or_else(|| crate::value::ValueError::new("MergeReport missing accepted"))?,
            insertion_index: insertion_index.ok_or_else(|| crate::value::ValueError::new("MergeReport missing insertionIndex"))?,
            replayed: replayed.ok_or_else(|| crate::value::ValueError::new("MergeReport missing replayed"))?,
            worst,
            conflict,
        })
    }
}
//#endregion 🔖️Reports

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn hlc(actor: u64, physical_ms: u64) -> crate::ids::HybridLogicalTimestamp {
        crate::ids::HybridLogicalTimestamp::new(actor, physical_ms)
    }

    //#region 🔖️ConflictId
    #[semio_framework_async_macros::async_test]
    async fn conflict_id_is_deterministic_and_content_sensitive() {
        let artifact = crate::ids::ArtifactId("doc-1".into());
        let ids = vec![crate::ids::MutationId("op-2".into()), crate::ids::MutationId("op-1".into())];
        let ids_reordered = vec![crate::ids::MutationId("op-1".into()), crate::ids::MutationId("op-2".into())];
        let kind = ConflictKind::Degraded { edit_ids: vec!["e1".into()] };
        let stamp = hlc(1, 100);

        let a = ConflictId::new(&kind, &artifact, &ids, &stamp).await;
        let b = ConflictId::new(&kind, &artifact, &ids_reordered, &stamp).await;
        assert_eq!(a, b, "mutation id order must not affect the conflict id (sorted before hashing).await");
        assert!(a.0.starts_with("conflict-"));

        let different_artifact = ConflictId::new(&kind, &crate::ids::ArtifactId("doc-2".into()), &ids, &stamp).await;
        assert_ne!(a, different_artifact);

        let different_kind = ConflictKind::Quarantined { envelopes: Vec::new() };
        let different_kind_id = ConflictId::new(&different_kind, &artifact, &ids, &stamp).await;
        assert_ne!(a, different_kind_id, "Quarantined vs Degraded with the same mutation ids must diverge");

        let different_hlc = ConflictId::new(&kind, &artifact, &ids, &hlc(1, 200)).await;
        assert_ne!(a, different_hlc);
    }
    //#endregion 🔖️ConflictId

    //#region 🔖️Reports
    #[semio_framework_async_macros::async_test]
    async fn dispatch_report_carries_worst_and_messages() {
        let report = DispatchReport { policy: crate::MergePolicy::Vigilant, worst: Some(crate::diagnostic::Severity::Warning), messages: vec![crate::MutationMessage::warn("mutation.clamped", "value clamped to range")] };
        assert_eq!(report.worst, Some(crate::diagnostic::Severity::Warning));
        assert_eq!(report.messages.len(), 1);
    }

    /// 🌱️ Rewritten off `serde_json` (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
    /// 26/09/01): round-trips through `ToValue`/`FromValue` instead — the first-party analog this
    /// crate now owns, same round-trip law the old name asserted.
    #[semio_framework_async_macros::async_test]
    async fn merge_report_round_trips_through_to_value() {
        let report = MergeReport {
            policy: crate::MergePolicy::Normal,
            accepted: true,
            insertion_index: 3,
            replayed: vec![EditMessages { edit_id: "e1".into(), messages: vec![crate::MutationMessage::info("mutation.cascade", "cascaded")] }],
            worst: Some(crate::diagnostic::Severity::Info),
            conflict: None,
        };
        let value = crate::value::ToValue::to_value(&report);
        let round_tripped: MergeReport = crate::value::FromValue::from_value(value).expect("decode");
        assert_eq!(round_tripped, report);
    }
    //#endregion 🔖️Reports

    //#region 🔖️Conflict
    #[semio_framework_async_macros::async_test]
    async fn conflict_kind_status_resolution_are_distinct() {
        let artifact = crate::ids::ArtifactId("doc-1".into());
        let quarantined = ConflictKind::Quarantined { envelopes: Vec::new() };
        let degraded = ConflictKind::Degraded { edit_ids: vec!["e1".into()] };
        let stamp = hlc(1, 100);
        let conflict = Conflict {
            id: ConflictId::new(&quarantined, &artifact, &[], &stamp).await,
            kind: quarantined,
            status: ConflictStatus::Open,
            messages: vec![crate::MutationMessage::error("mutation.target-missing", "target missing")],
            actors: vec![crate::ids::ActorId("actor-1".into())],
            timestamp: stamp,
        };
        assert_eq!(conflict.status, ConflictStatus::Open);
        assert!(matches!(conflict.kind, ConflictKind::Quarantined { .. }));
        assert!(!matches!(degraded, ConflictKind::Quarantined { .. }));
        assert_ne!(ConflictResolution::Accept, ConflictResolution::Discard);
    }
    //#endregion 🔖️Conflict
}
//#endregion 🧪️Tests
