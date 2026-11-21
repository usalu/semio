//! 📬️ Verifies the durable boundary and replay bytes of a complete authored media edit.

use super::*;

#[semio_framework_async_macros::async_test]
async fn original_group_replays_from_one_durable_boundary() {
    use protocol::atomic_hub::{AtomicSocketEdit, AtomicWalEdit};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️original-group.json")).unwrap();
    let author = fixture["actor"].as_str().unwrap();
    let edit_id = fixture["edit"].as_str().unwrap();
    let rows = fixture["rows"].as_u64().unwrap() as usize;
    let document = document_id().await;
    let core_document = to_core_document_id(&document).await;
    let mut events = Vec::with_capacity(rows);
    let ids: Vec<String> = (0..rows).map(|ordinal| format!("{edit_id}-{ordinal}")).collect();
    for (ordinal, id) in ids.iter().enumerate() {
        let bytes = vec![b'a' + ordinal as u8; fixture["bytesPerRow"].as_u64().unwrap() as usize];
        let dependencies = if ordinal == 0 { vec![] } else { vec![ids[ordinal - 1].as_str()] };
        let mut row = envelope(id, &dependencies, author, &[]).await;
        let schema = protocol::SchemaId("fixture.original-media.v1".into());
        row.diff = protocol::ArtifactDiff { schema: schema.clone(), payload: protocol::CausalPayload::try_from_slice(&bytes).unwrap() };
        row.inverse = protocol::InverseMutation { schema, payload: protocol::CausalPayload::try_from_slice(&[]).unwrap() };
        row.transaction = Some(protocol::TransactionRef {
            id: edit_id.to_owned(), tool: fixture["tool"].as_str().unwrap().to_owned(),
        });
        events.push(row);
    }
    let frames = AtomicWalEdit { edit_id: edit_id.to_owned(), rows }.relay(&events, &[]).unwrap();
    assert!(frames.len() > 1);
    let verify = |actor: &str| {
        let mut socket = AtomicSocketEdit::default();
        let mut original = None;
        for frame in &frames {
            original = socket.push(1, &frame[4..], &document.0, actor)?;
        }
        original.ok_or("atomic group did not complete")
    };
    assert!(verify(fixture["otherActor"].as_str().unwrap()).is_err());
    assert!(CommandBatch::new(events).await.is_err());
    let batch = || CommandBatch::from_verified_atomic(verify(author).unwrap(), &document).unwrap();

    let storage = storage().await;
    let mut engine = ArtifactEngine::create_retained(document.clone(), storage.clone(), ArtifactEngineConfig::default(), 0).await.unwrap();
    let options = || SubmitOptions { durability: DurabilityClass::Fsync, ..Default::default() };
    let receipt = engine.submit(batch(), options(), 1).await.unwrap();
    assert_eq!(receipt.frontier.head_seq, rows as u64);
    assert_eq!(receipt.frontier.commit_seq, fixture["commits"].as_u64().unwrap());
    assert_eq!(engine.submit(batch(), options(), 2).await.unwrap().frontier, receipt.frontier);
    let point = ArtifactLedgerPoint::of_snapshot(&engine.checkpoint_publication_snapshot().await);
    assert_eq!(point.head_edit_id, Some(protocol::MutationId(edit_id.to_owned())));
    std::future::poll_fn(|cx| engine.poll_close(cx)).await.unwrap();
    drop(engine);

    let (mut engine, _) = ArtifactEngine::open_retained(document.clone(), storage.clone(), ArtifactEngineConfig::default(), 3).await.unwrap();
    assert_eq!(ArtifactLedgerPoint::of_snapshot(&engine.checkpoint_publication_snapshot().await), point);
    let wal = storage.wal().await;
    let tail = artifact_ledger_tail(&wal, &core_document, &ArtifactLedgerPoint::genesis(), &point, Arc::new(std::sync::atomic::AtomicBool::new(false))).await.unwrap();
    let [commit] = tail.as_slice() else { panic!("expected one grouped commit") };
    assert_eq!(commit.original_edit.as_ref().unwrap().edit_id, edit_id);
    assert_eq!(AtomicWalEdit { edit_id: edit_id.to_owned(), rows }.relay(&commit.envelopes, &[]).unwrap(), frames);

    let state = db_sync::replay_sync_state(&wal, core_document.clone()).await.unwrap();
    let mut interior = db_durability::Frontier::genesis(core_document);
    interior.head_seq = fixture["catchupFrom"].as_u64().unwrap();
    let replay = db_sync::grouped_frontier_frames(&state, &interior, protocol::ActorId("fixture-hub".into())).await.unwrap();
    let [protocol::ServerFrame::Commands { envelopes, frontier, .. }] = replay.as_slice() else { panic!("expected one grouped replay frame") };
    assert_eq!(frontier.head_edit_id, edit_id);
    assert_eq!(envelopes, &commit.envelopes);
    drop(wal);
    std::future::poll_fn(|cx| engine.poll_close(cx)).await.unwrap();
}
