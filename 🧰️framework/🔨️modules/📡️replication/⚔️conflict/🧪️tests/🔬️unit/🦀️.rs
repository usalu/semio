
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
    let report = DispatchReport { policy: crate::MergePolicy::Vigilant, worst: Some(semio_framework_diagnostic::Severity::Warning), messages: vec![crate::MutationMessage::warning("mutation.clamped", "value clamped to range")] };
    assert_eq!(report.worst, Some(semio_framework_diagnostic::Severity::Warning));
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
        worst: Some(semio_framework_diagnostic::Severity::Info),
        conflict: None,
    };
    let value = crate::value::ToValue::to_value(&report);
    let round_tripped: MergeReport = crate::value::FromValue::from_value(value).expect("decode");
    assert_eq!(round_tripped, report);
}

/// 📋️ Language-agnostic replay reports (shared with the TypeScript twin): every report decodes, re-encodes to the
/// identical JSON, carries the maximum outcome level as `worst`, and blocks finalize iff an outcome is Error or Fatal.
#[test]
fn replay_report_fixture_decodes_round_trips_and_gates_finalize() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧫️replay-report/🔣️.json")).expect("replay report fixture parses");
    assert_eq!(fixture["schema"].as_str(), Some("semio.replication.replay-report"));
    for case in fixture["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let report: ReplayReport = crate::value::FromValue::from_value(crate::value::DslValue::from(&case["report"])).unwrap_or_else(|error| panic!("{id}: {error:?}"));
        assert_eq!(serde_json::Value::from(&crate::value::ToValue::to_value(&report)), case["report"], "{id}: round trip");
        assert_eq!(report.worst, report.outcomes.iter().filter_map(|outcome| outcome.worst).max(), "{id}: worst");
        assert_eq!(report.blocks_finalize(), case["expect"]["blocksFinalize"].as_bool().expect("blocksFinalize"), "{id}: blocks finalize");
    }
}

/// 🚧️ Warnings never block finalize; one Error or Fatal anywhere in the suffix does.
#[test]
fn replay_report_blocks_finalize_on_error_or_fatal_only() {
    let outcome = |worst: Option<semio_framework_diagnostic::Severity>| MutationReplayOutcome { mutation_id: crate::ids::MutationId("op".into()), edit_id: "e".into(), op_index: 0, worst, messages: Vec::new(), superseded: false, withdrawn: false };
    assert!(!ReplayReport::default().blocks_finalize());
    for (worst, blocks) in [(None, false), (Some(semio_framework_diagnostic::Severity::Info), false), (Some(semio_framework_diagnostic::Severity::Warning), false), (Some(semio_framework_diagnostic::Severity::Error), true), (Some(semio_framework_diagnostic::Severity::Fatal), true)] {
        let report = ReplayReport { from_position: 0, outcomes: vec![outcome(None), outcome(worst)], worst };
        assert_eq!(report.blocks_finalize(), blocks, "{worst:?}");
    }
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
