/// 🧮️ The fold of edits, commits and reverts, the forward and reverse scans, the frontier delta and the index.

//#region 🔖️Fold
fn fold_edit(id: &str, op_id: &str, physical_ms: i64) -> HistoryEdit {
    HistoryEdit { line: None,
        id: id.to_string(),
        actor: Some("alice".into()),
        started_at: "2024-01-15T10:30:00Z".to_string(),
        finished_at: None, verb: None,
        ops: vec![OpPayload { text: Some(format!("set {id}=1")), binary: None }],
        inverse: Vec::new(),
        meta: Some(vec![HistoryOpMeta { op_id: Some(op_id.to_string()), hlt: Some((1, physical_ms, 0)), ..HistoryOpMeta::default() }]),
        lane: None,
    }
}

fn fold_log() -> HistoryLog {
    HistoryLog {
        doc_id: "doc-f".to_string(),
        schema: "schema-f".to_string(),
        edits: [fold_edit("edit-a", "op-a", 100), fold_edit("edit-b", "op-b", 200)].into_iter().collect(),
        transitions: vec![
            transition_record("doc-f", "alice", protocol::HistoryTransition::Revert { mutation_ids: vec![protocol::MutationId("op-b".to_string())] }, (1, 400, 0)),
            transition_record("doc-f", "alice", commit("ck-1", None, "change-1", &["op-a", "op-b"]), (1, 300, 0)),
        ],
        composition: None,
        conflicts: Vec::new(),
        viewer_line: None,
        viewer_checkpoint: None,
    }
}

#[semio_framework_async_macros::async_test]
async fn fold_derives_applied_redo_and_checkpoint_from_edits_commit_and_revert() {
    let fold = fold_log().fold().unwrap();
    assert_eq!(fold.applied, vec!["edit-a".to_string()]);
    assert_eq!(fold.redo, vec!["edit-b".to_string()]);
    assert_eq!(fold.checkpoint.as_deref(), Some("ck-1"));
    assert_eq!(fold.alternative, None);
    assert_eq!(fold.changes.len(), 1);
    assert_eq!(fold.changes[0].edit_ids, vec!["edit-a".to_string(), "edit-b".to_string()]);
    assert_eq!(fold.checkpoints[0].change_ids, vec!["change-1".to_string()]);
    assert_eq!(fold.checkpoints[0].authors[0].name, "Ueli Saluz");
}

#[semio_framework_async_macros::async_test]
async fn fold_is_identical_after_a_binary_round_trip() {
    let log = fold_log();
    let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
    assert_eq!(decode_history(&bytes, &DecodeOptions::default()).await.unwrap().fold().unwrap(), log.fold().unwrap());
}

#[semio_framework_async_macros::async_test]
async fn fold_falls_back_to_positional_mutation_ids_and_zero_clock_without_meta() {
    assert_eq!(sample_log().await.fold().unwrap().alternative, None, "a branch selects no replica's head: without a viewer line the fold is the trunk tip");
    let log = HistoryLog { viewer_line: Some("alt-1".to_string()), ..sample_log().await };
    let fold = log.fold().unwrap();
    assert_eq!(fold.changes[0].edit_ids, vec!["edit-1".to_string(), "edit-2".to_string()]);
    assert_eq!(fold.alternative.as_deref(), Some("alt-1"));
    assert_eq!(fold.alternatives.iter().find(|alternative| alternative.id == "alt-1").expect("the branched alternative").checkpoint_ids, vec!["ck-1".to_string()]);
    assert_eq!(fold.alternatives[0].id, fold.trunk, "the trunk is listed first once a commit gave it a chain");
    assert_eq!(fold.applied, vec!["edit-1".to_string(), "edit-2".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn fold_excludes_edits_quarantined_by_an_unaccepted_conflict() {
    let mut log = fold_log();
    log.transitions.clear();
    let quarantined = protocol::MutationEnvelope {
        mutation_id: protocol::MutationId("op-a".to_string()),
        document_id: protocol::ArtifactId("doc-f".to_string()),
        actor: protocol::ActorId("bob".into()),
        dependencies: Vec::new(),
        observed: None,
        target: Vec::new(),
        diff: protocol::ArtifactDiff { schema: protocol::SchemaId("schema-f".to_string()), payload: vec![1] },
        inverse: protocol::InverseMutation { schema: protocol::SchemaId("schema-f".to_string()), payload: Vec::new() },
        timestamp: protocol::HybridLogicalTimestamp { actor: 2, physical_ms: 100, logical: 0 },
        transaction: None, verb: None, line: None,
    };
    let mut envelope = Vec::new();
    protocol::encode_envelope(&quarantined, &mut envelope);
    log.conflicts.push(HistoryConflict { id: "conflict-1".into(), kind: 0, status: 0, actors: vec!["bob".into()], hlt: (2, 100, 0), edit_ids: Vec::new(), envelopes: vec![envelope], messages: Vec::new() });
    assert_eq!(log.fold().unwrap().applied, vec!["edit-b".to_string()]);
    log.conflicts[0].status = 1;
    assert_eq!(log.fold().unwrap().applied, vec!["edit-a".to_string(), "edit-b".to_string()]);
    log.conflicts[0].status = 2;
    assert_eq!(log.fold().unwrap().applied, vec!["edit-b".to_string()]);
}

#[semio_framework_async_macros::async_test]
async fn fold_rejects_a_transition_naming_an_unknown_operation_and_a_negative_clock() {
    let mut log = fold_log();
    log.transitions.push(transition_record("doc-f", "alice", protocol::HistoryTransition::Reinstate { mutation_ids: vec![protocol::MutationId("op-missing".to_string())] }, (1, 500, 0)));
    assert!(log.fold().is_err());
    let mut log = fold_log();
    log.edits[0].meta.as_mut().unwrap()[0].hlt = Some((1, -1, 0));
    assert!(log.fold().is_err());
}
//#endregion 🔖️Fold

//#region 🔖️Scan
#[semio_framework_async_macros::async_test]
async fn reader_edits_forward_matches_log() {
    let log = sample_log().await;
    let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
    let reader = HistoryReader::open(&bytes, &DecodeOptions::default()).await.unwrap();
    let edits: Vec<HistoryEdit> = reader.edits().await.map(|r| r.unwrap()).collect();
    assert_eq!(edits.len(), log.edits.len());
    assert!(edits.iter().eq(log.edits.iter()));
}

#[semio_framework_async_macros::async_test]
async fn reader_edits_rev_matches_tail_in_reverse() {
    let log = sample_log().await;
    let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
    let reader = HistoryReader::open(&bytes, &DecodeOptions::default()).await.unwrap();
    let rev: Vec<HistoryEdit> = reader.edits_rev(1).await.map(|r| r.unwrap()).collect();
    assert_eq!(rev.len(), 1);
    assert_eq!(rev[0], log.edits[1]);
}

#[semio_framework_async_macros::async_test]
async fn reader_edits_rev_full_matches_reversed_forward() {
    let log = sample_log().await;
    let bytes = encode_history(&log, &EncodeOptions::default()).await.unwrap();
    let reader = HistoryReader::open(&bytes, &DecodeOptions::default()).await.unwrap();
    let rev: Vec<HistoryEdit> = reader.edits_rev(usize::MAX).await.map(|r| r.unwrap()).collect();
    assert_eq!(rev.len(), log.edits.len());
    assert!(rev.iter().eq(log.edits.iter().rev()));
}
//#endregion 🔖️Scan

//#region 🔖️Frontier
async fn frontier(head_edit_ordinal: u64, head_edit_id: &str, chain_hash: [u8; 32]) -> FrontierSummary {
    FrontierSummary { document_id: "doc-1".to_string(), head_edit_ordinal, head_edit_id: head_edit_id.to_string(), alternatives: Vec::new(), last_commit_seq: 1, chain_hash }
}

#[semio_framework_async_macros::async_test]
async fn frontier_delta_reports_equal_ahead_behind_diverged() {
    let a = frontier(5, "edit-5", [1u8; 32]).await;
    let b = frontier(5, "edit-5", [1u8; 32]).await;
    assert_eq!(frontier_delta(&a, &b).await, FrontierComparison::Equal);

    let ahead = frontier(6, "edit-6", [2u8; 32]).await;
    assert_eq!(frontier_delta(&ahead, &a).await, FrontierComparison::Ahead);
    assert_eq!(frontier_delta(&a, &ahead).await, FrontierComparison::Behind);

    let diverged = frontier(5, "edit-5-alt", [3u8; 32]).await;
    assert_eq!(frontier_delta(&a, &diverged).await, FrontierComparison::Diverged { common_edit_count: 5 });
}
//#endregion 🔖️Frontier

//#region 🔖️Index
#[semio_framework_async_macros::async_test]
async fn index_round_trips_edits_checkpoints_and_snapshots() {
    let mut builder = IndexBuilder::new().await;
    builder.record_edit(0, 100).await;
    builder.record_edit(5, 300).await;
    builder.record_edit(10, 500).await;
    builder.record_checkpoint("ck-1", 700, 10).await;
    builder.record_snapshot(5, 250).await;
    builder.record_sealed(50).await;
    let payload = builder.build().await;

    let reader = IndexReader::open(&payload).await.unwrap();
    assert_eq!(reader.edit_offset_at_or_before(7).await, Some(300));
    assert_eq!(reader.edit_offset_at_or_before(0).await, Some(100));
    assert_eq!(reader.edit_offset_at_or_before(10).await, Some(500));
    assert_eq!(reader.checkpoint_offset("ck-1").await, Some((700, 10)));
    assert_eq!(reader.checkpoint_offset("missing").await, None);
    assert_eq!(reader.latest_snapshot_offset_at_or_before(9).await, Some(250));
}
//#endregion 🔖️Index
