// 🧭️ Codec, state, frontier and bootstrap laws: the sync fixtures, the envelope codec, the replica state, the frontier delta, the missing commands, the bootstrap decisions, the resume token and the hello and frontier advertise handlers.

//#region 🔖️Codec
#[semio_framework_async_macros::async_test]
async fn command_envelope_round_trips_through_encode_decode() {
    let envelope = sample_envelope("op-1", 7).await;
    let bytes = encode_command_envelope(&envelope).await;
    assert_eq!(decode_command_envelope(&bytes).await.unwrap(), envelope);
}

#[semio_framework_async_macros::async_test]
async fn decode_command_envelope_rejects_malformed_bytes_without_panicking() {
    assert!(matches!(decode_command_envelope(b"not json").await, Err(DbError::Corrupt(_))));
}
//#endregion 🔖️Codec

//#region 🔖️ReplicaState
#[semio_framework_async_macros::async_test]
async fn replay_sync_state_derives_frontier_and_ordered_commands() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 3).await;

    let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();
    assert_eq!(state.frontier.head_seq, 3);
    assert_eq!(state.frontier.commit_seq, 3);
    assert_eq!(state.floor_head_seq, 0);
    assert_eq!(state.commands.len(), 3);
    assert_eq!(state.commands[0].mutation_id.0, "op-0");
    assert_eq!(state.commands[2].mutation_id.0, "op-2");
}

#[semio_framework_async_macros::async_test]
async fn replay_sync_state_on_empty_document_is_genesis() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 0).await;

    let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();
    assert_eq!(state.frontier.head_seq, 0);
    assert_eq!(state.frontier.chain_hash, [0u8; 32]);
    assert!(state.commands.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn replay_sync_state_tracks_the_latest_snapshot_pub_as_the_floor() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 5).await;
    let floor_frontier = Frontier { document: document.clone(), head_seq: 2, commit_seq: 2, chain_hash: [1u8; 32], epoch: 0 };
    publish_snapshot_marker(&storage, &document, 1, floor_frontier).await;

    let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();
    assert_eq!(state.floor_head_seq, 2);
    assert_eq!(state.frontier.head_seq, 5, "the marker itself carries no commands");
}
//#endregion 🔖️ReplicaState

//#region 🔖️Frontier
#[semio_framework_async_macros::async_test]
async fn frontier_delta_reports_the_command_gap_and_rejects_backwards() {
    let document: ArtifactId = "doc-1".into();
    let from = Frontier { document: document.clone(), head_seq: 2, commit_seq: 2, chain_hash: [0u8; 32], epoch: 0 };
    let to = Frontier { document, head_seq: 5, commit_seq: 5, chain_hash: [9u8; 32], epoch: 0 };

    let delta = frontier_delta(&from, &to).await.unwrap();
    assert_eq!(delta.commands, 3);
    assert!(!delta.is_empty().await);
    assert!(frontier_delta(&to, &from).await.is_err(), "a delta only ever moves forward");
}

#[semio_framework_async_macros::async_test]
async fn frontier_summary_bridges_round_trip() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 2).await;
    let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();

    let summary = state_frontier_summary(&state).await;
    assert_eq!(summary.head_edit_ordinal, 2);
    assert_eq!(summary.head_edit_id, "op-1");
    assert_eq!(summary.last_commit_seq, state.frontier.commit_seq);
    assert_eq!(summary.chain_hash, state.frontier.chain_hash);

    let bridged_back = from_frontier_summary(&summary);
    assert_eq!(bridged_back.head_seq, state.frontier.head_seq);
    assert_eq!(bridged_back.commit_seq, state.frontier.commit_seq);
    assert_eq!(bridged_back.chain_hash, state.frontier.chain_hash);
    assert_eq!(bridged_back.document, state.frontier.document);
}
//#endregion 🔖️Frontier

//#region 🔖️MissingCommands
#[semio_framework_async_macros::async_test]
async fn missing_commands_transfer_round_trip_from_genesis() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 4).await;
    let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();

    let replica_frontier = Frontier::genesis(document);
    let missing = missing_commands(&state, &replica_frontier).await.unwrap();
    assert_eq!(missing, state.commands, "a genesis replica is missing every command");

    // "Applying" the transfer catches the replica up to the server's frontier exactly.
    let caught_up = state.frontier.clone();
    assert!(missing_commands(&state, &caught_up).await.unwrap().is_empty());
}

#[semio_framework_async_macros::async_test]
async fn missing_commands_transfer_round_trip_for_a_partially_caught_up_replica() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 3).await;
    let first_state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();
    let replica_frontier = first_state.frontier;

    // More commands land on the server after the replica already caught up once.
    {
        let (mut wal, _report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 100)).unwrap();
        for i in 3..6u64 {
            let envelope = sample_envelope(&format!("op-{i}"), i).await;
            submit_record(&storage, &mut wal, command_record(&envelope).await, i).await;
        }
    }

    let second_state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();
    let missing = missing_commands(&second_state, &replica_frontier).await.unwrap();
    assert_eq!(missing.len(), 3);
    assert_eq!(missing[0].mutation_id.0, "op-3");
    assert_eq!(missing[2].mutation_id.0, "op-5");
}

#[semio_framework_async_macros::async_test]
async fn missing_commands_rejects_document_mismatch_and_a_replica_ahead_of_server() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 2).await;
    let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();

    let other_document = Frontier::genesis("doc-2".into());
    assert!(matches!(missing_commands(&state, &other_document).await, Err(DbError::InvalidArgument(_))));

    let ahead = Frontier { head_seq: 99, ..state.frontier.clone() };
    assert!(matches!(missing_commands(&state, &ahead).await, Err(DbError::InvalidArgument(_))));
}

#[semio_framework_async_macros::async_test]
async fn missing_commands_rejects_a_replica_behind_the_retained_floor() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 5).await;
    let floor_frontier = Frontier { document: document.clone(), head_seq: 3, commit_seq: 3, chain_hash: [2u8; 32], epoch: 0 };
    publish_snapshot_marker(&storage, &document, 1, floor_frontier).await;
    let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();

    let too_far_behind = Frontier { document, head_seq: 1, commit_seq: 1, chain_hash: [0u8; 32], epoch: 0 };
    assert!(matches!(missing_commands(&state, &too_far_behind).await, Err(DbError::Unavailable(_))));
}
//#endregion 🔖️MissingCommands

//#region 🔖️Bootstrap
#[semio_framework_async_macros::async_test]
async fn decide_bootstrap_serves_tail_for_a_fresh_replica_within_the_floor() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 3).await;
    let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();

    let plan = db_actor::block_on(decide_bootstrap(&state, &storage, None)).unwrap();
    assert_eq!(plan, BootstrapPlan::Tail { envelopes: state.commands });
}

#[semio_framework_async_macros::async_test]
async fn decide_bootstrap_reports_none_for_an_already_caught_up_replica() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 3).await;
    let state = db_actor::block_on(replay_sync_state(&storage, document)).unwrap();

    let plan = db_actor::block_on(decide_bootstrap(&state, &storage, Some(&state.frontier))).unwrap();
    assert_eq!(plan, BootstrapPlan::None);
}

#[semio_framework_async_macros::async_test]
async fn decide_bootstrap_serves_snapshot_when_a_generation_is_available_below_the_floor() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 5).await;
    let floor_frontier = Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [3u8; 32], epoch: 0 };
    publish_snapshot_marker(&storage, &document, 7, floor_frontier).await;
    let pages = db_io_registry::db_io_copy_pages(b"snapshot-bytes").unwrap().await.unwrap();
    SnapshotStorage::write_generation(&storage, &document, 7, pages).await.unwrap();
    let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();

    let stale_replica = Frontier { document, head_seq: 0, commit_seq: 0, chain_hash: [0u8; 32], epoch: 0 };
    let plan = db_actor::block_on(decide_bootstrap(&state, &storage, Some(&stale_replica))).unwrap();
    match plan {
        BootstrapPlan::Snapshot { generation, pages, pack_hash } => {
            assert_eq!(generation, 7);
            assert_eq!(pages, b"snapshot-bytes");
            assert_eq!(pack_hash, *semio_framework_hash::hash(b"snapshot-bytes").as_bytes());
        }
        other => panic!("expected a Snapshot plan, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn decide_bootstrap_reports_unavailable_when_below_floor_with_no_snapshot() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 5).await;
    let floor_frontier = Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [3u8; 32], epoch: 0 };
    publish_snapshot_marker(&storage, &document, 7, floor_frontier).await;
    let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();

    let stale_replica = Frontier { document, head_seq: 0, commit_seq: 0, chain_hash: [0u8; 32], epoch: 0 };
    assert!(matches!(db_actor::block_on(decide_bootstrap(&state, &storage, Some(&stale_replica))), Err(DbError::Unavailable(_))));
}
//#endregion 🔖️Bootstrap

//#region 🔖️ResumeToken
#[semio_framework_async_macros::async_test]
async fn issue_resume_token_produces_the_documented_v1_wire_format() {
    let document: ArtifactId = "doc-1".into();
    let frontier = Frontier { document, head_seq: 4, commit_seq: 4, chain_hash: [5u8; 32], epoch: 0 };
    let token = issue_resume_token(&frontier).await.unwrap();
    assert!(token.starts_with("v1|doc-1|4|4|0|"));
}
//#endregion 🔖️ResumeToken

//#region 🔖️Hello
#[semio_framework_async_macros::async_test]
async fn handle_hello_bootstraps_a_fresh_replica_via_tail_and_issues_a_resume_token() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 3).await;
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(storage));

    let response = db_actor::block_on(handle_hello(db_io_driver::db_io_test_pool(), storage, document, None, "session-1".to_string(), protocol::ActorId("semio_hub".to_string()), 64 * 1024)).unwrap();
    let protocol::ServerFrame::Welcome { bootstrap, server_frontier, resume_token, .. } = &response.welcome else {
        panic!("expected a Welcome frame");
    };
    assert_eq!(*bootstrap, protocol::Bootstrap::Tail);
    assert_eq!(server_frontier.head_edit_ordinal, 3);
    assert!(!resume_token.is_empty());
    assert_eq!(response.follow_up.len(), 1);
    match &response.follow_up[0] {
        protocol::ServerFrame::Commands { envelopes, .. } => assert_eq!(envelopes.len(), 3),
        other => panic!("expected a Commands follow-up frame, got {other:?}"),
    }
}

#[semio_framework_async_macros::async_test]
async fn handle_hello_reports_no_follow_up_for_an_already_caught_up_replica() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 2).await;
    let state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();
    let hello_frontier = state_frontier_summary(&state).await;
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(storage));

    let response = db_actor::block_on(handle_hello(db_io_driver::db_io_test_pool(), storage, document, Some(hello_frontier), "session-2".to_string(), protocol::ActorId("semio_hub".to_string()), 64 * 1024)).unwrap();
    let protocol::ServerFrame::Welcome { bootstrap, .. } = &response.welcome else {
        panic!("expected a Welcome frame");
    };
    assert_eq!(*bootstrap, protocol::Bootstrap::None);
    assert!(response.follow_up.is_empty());
}

#[semio_framework_async_macros::async_test]
async fn handle_hello_chunks_a_snapshot_larger_than_the_requested_chunk_size() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 4).await;
    let floor_frontier = Frontier { document: document.clone(), head_seq: 4, commit_seq: 4, chain_hash: [1u8; 32], epoch: 0 };
    publish_snapshot_marker(&storage, &document, 9, floor_frontier).await;
    let big_snapshot = vec![7u8; 10];
    let pages = db_io_registry::db_io_copy_pages(&big_snapshot).unwrap().await.unwrap();
    SnapshotStorage::write_generation(&storage, &document, 9, pages).await.unwrap();
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(storage));

    let stale_hello_frontier = protocol::RuntimeFrontierSummary { document_id: protocol::ArtifactId(document.0.clone()), head_edit_ordinal: 0, head_edit_id: String::new(), last_commit_seq: 0, chain_hash: [0u8; 32] };
    let response = db_actor::block_on(handle_hello(db_io_driver::db_io_test_pool(), storage, document, Some(stale_hello_frontier), "session-3".to_string(), protocol::ActorId("semio_hub".to_string()), 4)).unwrap();

    let protocol::ServerFrame::Welcome { bootstrap, .. } = &response.welcome else {
        panic!("expected a Welcome frame");
    };
    assert!(matches!(bootstrap, protocol::Bootstrap::Snapshot { inline: None, .. }));
    // 10 bytes chunked at 4 bytes/chunk -> 3 chunks (4, 4, 2), plus one SnapshotDone.
    assert_eq!(response.follow_up.len(), 4);
    assert!(matches!(response.follow_up[3], protocol::ServerFrame::SnapshotDone { seq_count: 3 }));
}

#[semio_framework_async_macros::async_test]
async fn handle_hello_rejects_zero_snapshot_chunk_bytes() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 1).await;
    let storage = std::sync::Arc::new(db_storage::DbBackend::Memory(storage));
    assert!(matches!(db_actor::block_on(handle_hello(db_io_driver::db_io_test_pool(), storage, document, None, "s".to_string(), protocol::ActorId("semio_hub".to_string()), 0)), Err(DbError::InvalidArgument(_))));
}

#[semio_framework_async_macros::async_test]
async fn handle_frontier_advertise_relays_missing_commands_and_none_when_caught_up() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document: ArtifactId = "doc-1".into();
    seed_wal(&storage, &document, 2).await;
    let first_state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();
    let replica_summary = state_frontier_summary(&first_state).await;

    {
        let (mut wal, _report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 100)).unwrap();
        let envelope = sample_envelope("op-2", 2).await;
        submit_record(&storage, &mut wal, command_record(&envelope).await, 100).await;
    }

    let frame = db_actor::block_on(handle_frontier_advertise(&storage, document.clone(), &replica_summary, protocol::ActorId("semio_hub".to_string()))).unwrap();
    match frame {
        Some(protocol::ServerFrame::Commands { envelopes, .. }) => {
            assert_eq!(envelopes.len(), 1);
            assert_eq!(envelopes[0].mutation_id.0, "op-2");
        }
        other => panic!("expected a Commands frame, got {other:?}"),
    }

    let up_to_date_state = db_actor::block_on(replay_sync_state(&storage, document.clone())).unwrap();
    let up_to_date_summary = state_frontier_summary(&up_to_date_state).await;
    assert!(db_actor::block_on(handle_frontier_advertise(&storage, document, &up_to_date_summary, protocol::ActorId("semio_hub".to_string()))).unwrap().is_none());
}
//#endregion 🔖️Hello
