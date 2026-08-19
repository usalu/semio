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
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ConflictId(pub String);

impl ConflictId {
    /// 🔑️ `blake3(kind-tag || artifact id || sorted mutation ids || hlc)`, using this crate's own
    /// `blake3::hash` primitive directly (the same hashing dependency `🌿️vcs`'s
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

        let digest = *blake3::hash(&input).as_bytes();
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
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ConflictKind {
    Quarantined { envelopes: Vec<crate::MutationEnvelope> },
    Degraded { edit_ids: Vec<String> },
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConflictStatus {
    Open,
    Accepted,
    Discarded,
}

/// @emoji ✅️❌️ What a human/authority decided to do with an `Open` conflict.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConflictResolution {
    Accept,
    Discard,
}
//#endregion 🔖️ConflictStatus

//#region 🔖️Conflict
/// @emoji ⚔️ One first-class conflict: identity, what it is, its lifecycle status, the messages that
/// explain it, who was involved, and when it was detected.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conflict {
    pub id: ConflictId,
    pub kind: ConflictKind,
    pub status: ConflictStatus,
    pub messages: Vec<crate::MutationMessage>,
    pub actors: Vec<crate::ids::ActorId>,
    pub timestamp: crate::ids::HybridLogicalTimestamp,
}
//#endregion 🔖️Conflict

//#region 🔖️Reports
/// @emoji 📨️ One edit's worth of `MutationMessage`s — the per-edit unit `MergeReport::replayed`
/// carries and `📡️spr/📜️history`'s durable ledger keys by `edit_id`.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditMessages {
    pub edit_id: String,
    pub messages: Vec<crate::MutationMessage>,
}

/// @emoji 📤️ The report a single LOCAL dispatch (one `ArtifactStore::dispatch`-shaped call)
/// produces: the policy it was judged against, the worst level reached, and every message.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DispatchReport {
    pub policy: crate::MergePolicy,
    pub worst: Option<crate::diagnostic::Severity>,
    pub messages: Vec<crate::MutationMessage>,
}

/// @emoji 🔀️ The report one `ingest_remote`/`merge_remote_snapshot`/`resolve_conflict` merge
/// produces: whether the incoming batch was accepted, where it landed (`insertion_index` — the
/// position in `applied_edit_ids` the batch's first edit was inserted at, meaningful only when
/// `accepted`), every replayed edit's messages, the worst level across the whole replayed suffix,
/// and the id of a `Conflict` this merge raised, if any (`Quarantined` on reject, `Degraded` on an
/// accepted-but-messy merge).
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergeReport {
    pub policy: crate::MergePolicy,
    pub accepted: bool,
    pub insertion_index: u32,
    pub replayed: Vec<EditMessages>,
    pub worst: Option<crate::diagnostic::Severity>,
    pub conflict: Option<ConflictId>,
}
//#endregion 🔖️Reports

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    async fn hlc(actor: u64, physical_ms: u64) -> crate::ids::HybridLogicalTimestamp {
        crate::ids::HybridLogicalTimestamp::new(actor, physical_ms).await
    }

    //#region 🔖️ConflictId
    #[semio_framework_async_macros::async_test]
    async fn conflict_id_is_deterministic_and_content_sensitive() {
        let artifact = crate::ids::ArtifactId("doc-1".into());
        let ids = vec![crate::ids::MutationId("op-2".into()), crate::ids::MutationId("op-1".into())];
        let ids_reordered = vec![crate::ids::MutationId("op-1".into()), crate::ids::MutationId("op-2".into())];
        let kind = ConflictKind::Degraded { edit_ids: vec!["e1".into()] };
        let stamp = hlc(1, 100).await;

        let a = ConflictId::new(&kind, &artifact, &ids, &stamp).await;
        let b = ConflictId::new(&kind, &artifact, &ids_reordered, &stamp).await;
        assert_eq!(a, b, "mutation id order must not affect the conflict id (sorted before hashing).await");
        assert!(a.0.starts_with("conflict-"));

        let different_artifact = ConflictId::new(&kind, &crate::ids::ArtifactId("doc-2".into()), &ids, &stamp).await;
        assert_ne!(a, different_artifact);

        let different_kind = ConflictKind::Quarantined { envelopes: Vec::new() };
        let different_kind_id = ConflictId::new(&different_kind, &artifact, &ids, &stamp).await;
        assert_ne!(a, different_kind_id, "Quarantined vs Degraded with the same mutation ids must diverge");

        let different_hlc = ConflictId::new(&kind, &artifact, &ids, &hlc(1, 200).await).await;
        assert_ne!(a, different_hlc);
    }
    //#endregion 🔖️ConflictId

    //#region 🔖️Reports
    #[semio_framework_async_macros::async_test]
    async fn dispatch_report_carries_worst_and_messages() {
        let report = DispatchReport {
            policy: crate::MergePolicy::Vigilant,
            worst: Some(crate::diagnostic::Severity::Warning),
            messages: vec![crate::MutationMessage::warn("mutation.clamped", "value clamped to range").await],
        };
        assert_eq!(report.worst, Some(crate::diagnostic::Severity::Warning));
        assert_eq!(report.messages.len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn merge_report_round_trips_through_serde() {
        let report = MergeReport {
            policy: crate::MergePolicy::Normal,
            accepted: true,
            insertion_index: 3,
            replayed: vec![EditMessages { edit_id: "e1".into(), messages: vec![crate::MutationMessage::info("mutation.cascade", "cascaded").await] }],
            worst: Some(crate::diagnostic::Severity::Info),
            conflict: None,
        };
        let json = serde_json::to_string(&report).expect("serialize");
        let round_tripped: MergeReport = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round_tripped, report);
    }
    //#endregion 🔖️Reports

    //#region 🔖️Conflict
    #[semio_framework_async_macros::async_test]
    async fn conflict_kind_status_resolution_are_distinct() {
        let artifact = crate::ids::ArtifactId("doc-1".into());
        let quarantined = ConflictKind::Quarantined { envelopes: Vec::new() };
        let degraded = ConflictKind::Degraded { edit_ids: vec!["e1".into()] };
        let stamp = hlc(1, 100).await;
        let conflict = Conflict {
            id: ConflictId::new(&quarantined, &artifact, &[], &stamp).await,
            kind: quarantined,
            status: ConflictStatus::Open,
            messages: vec![crate::MutationMessage::error("mutation.target-missing", "target missing").await],
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
