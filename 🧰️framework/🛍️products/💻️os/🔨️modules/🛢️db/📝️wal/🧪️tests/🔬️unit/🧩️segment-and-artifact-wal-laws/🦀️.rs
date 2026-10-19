// 🧩️ Segment and artifact WAL laws: the single segment commit, group commit, fsync durability, torn tail, resumed transaction ids, bounded segment verification, rotation chains, sealed segment tears and the artifact WAL writer reacquisition.

//#region 🔖️Segment + ArtifactWal
#[semio_framework_async_macros::async_test]
async fn single_segment_write_commit_flush_recovers_cleanly() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();

    let receipt = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"cmd-1").await), DurabilityClass::Fsync, 1).await;
    assert!(receipt.committed, "Fsync durability must force an immediate commit");
    assert_eq!(receipt.tx_id, 1);

    let bytes = segment_bytes(&storage, &document, 0).await;
    let report = protocol::format::recover(&bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).await.unwrap();
    assert_eq!(report.bytes_recovered, bytes.len() as u64);
    assert_eq!(report.torn_tail_bytes, 0);

    assert_eq!(replay_summaries(&storage, &document).await, vec![ReplaySummary::Segment(0, None), ReplaySummary::Begin(1), ReplaySummary::Command(b"cmd-1".to_vec()), ReplaySummary::Commit(1, 1)]);
    wal.close().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn group_commit_batches_until_policy_threshold_then_commits() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let policy = GroupCommitPolicy { max_delay_ms: 1_000_000, max_bytes: u64::MAX, max_records: 5 };
    let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), policy, 0)).unwrap();

    let receipt_1 = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"a").await), DurabilityClass::Memory, 10).await;
    assert!(!receipt_1.committed);
    assert_eq!(db_actor::block_on(storage.segment_len(&document, 0)).unwrap(), wal.active.flushed_len, "nothing new should have flushed yet");

    let receipt_2 = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"b").await), DurabilityClass::Memory, 11).await;
    assert!(receipt_2.committed);
    assert_eq!(wal.active.pending_records, 0);
    assert!(db_actor::block_on(storage.segment_len(&document, 0)).unwrap() > 32, "flush must have appended past the bare header");
    wal.close().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn fsync_durability_forces_immediate_commit_regardless_of_policy() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let policy = GroupCommitPolicy { max_delay_ms: u64::MAX, max_bytes: u64::MAX, max_records: u32::MAX };
    let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document, policy, 0)).unwrap();
    let receipt = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"a").await), DurabilityClass::Fsync, 0).await;
    assert!(receipt.committed);
    wal.close().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn torn_tail_is_recovered_by_truncating_only_the_uncommitted_suffix() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    {
        let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
        submit_one(&storage, &mut wal, WalRecord::Command(retained(b"trusted").await), DurabilityClass::Fsync, 1).await;
        wal.close().await.unwrap();
    }

    let committed = segment_bytes(&storage, &document, 0).await;
    let writer = storage.acquire_writer(&document).await.unwrap();
    storage.append(&writer, 0, pages(b"\x0Fgarbage")).await.unwrap();
    writer.release().await.unwrap();

    let (mut wal, report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 100)).unwrap();
    assert!(report.torn_tail_bytes > 0);
    assert_eq!(report.segments_seen, 1);
    wal.close().await.unwrap();

    assert_eq!(replay_summaries(&storage, &document).await, vec![ReplaySummary::Segment(0, None), ReplaySummary::Begin(1), ReplaySummary::Command(b"trusted".to_vec()), ReplaySummary::Commit(1, 1)]);

    let bytes = segment_bytes(&storage, &document, 0).await;
    let post_recovery = protocol::format::recover(&bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).await.unwrap();
    assert_eq!(post_recovery.bytes_recovered, bytes.len() as u64, "the truncated segment must itself be torn-tail-free");
    assert_eq!(bytes, committed, "all original commits are byte-preserved");
}

#[semio_framework_async_macros::async_test]
async fn recovery_resumes_next_tx_id_and_accepts_further_submits() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    {
        let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
        submit_one(&storage, &mut wal, WalRecord::Command(retained(b"one").await), DurabilityClass::Fsync, 1).await;
        submit_one(&storage, &mut wal, WalRecord::Command(retained(b"two").await), DurabilityClass::Fsync, 2).await;
        wal.close().await.unwrap();
    }

    let (mut wal, _report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 3)).unwrap();
    let receipt = submit_one(&storage, &mut wal, WalRecord::Command(retained(b"three").await), DurabilityClass::Fsync, 4).await;
    assert_eq!(receipt.tx_id, 3, "recovery must resume tx ids strictly past whatever was already durable");

    let commands: Vec<_> = replay_summaries(&storage, &document)
        .await
        .into_iter()
        .filter_map(|record| match record {
            ReplaySummary::Command(bytes) => Some(bytes),
            _ => None,
        })
        .collect();
    assert_eq!(commands, vec![b"one".to_vec(), b"two".to_vec(), b"three".to_vec()]);
    wal.close().await.unwrap();
}

/// ⏱️ Opening a WAL is bounded per verified segment, never over its whole history: a chain whose
/// verification needs more fuel in total than one step grants still opens under a stall-bounded
/// control, while the same budget as one fixed allowance refuses it. A grown document stays openable.
#[semio_framework_async_macros::async_test]
async fn wal_open_bounds_each_verified_segment_not_the_whole_history() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document = doc("grown-history").await;
    let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
    wal.max_segment_bytes = 400;
    for index in 0..48u32 {
        submit_one(&storage, &mut wal, WalRecord::Command(retained(format!("history-{index:04}-{}", "h".repeat(96)).as_bytes()).await), DurabilityClass::Fsync, u64::from(index)).await;
    }
    wal.close().await.unwrap();
    let segments = db_actor::block_on(storage.list_segments(&document)).unwrap().len() as usize;
    assert!(segments >= 8, "the history spans many segments: {segments}");
    let spent = {
        let mut control = WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000).unwrap();
        let (mut opened, _) = ArtifactWal::open_with_control(&storage, document.clone(), GroupCommitPolicy::default(), 1, &mut control).await.unwrap();
        opened.close().await.unwrap();
        1_000_000 - control.fuel
    };
    let step_fuel = spent.div_ceil(segments) * 2;
    assert!(step_fuel < spent, "one step's fuel covers less than the whole chain: step={step_fuel} whole={spent}");
    let mut fixed = WalCursorControl::new(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), step_fuel).unwrap();
    match ArtifactWal::open_with_control(&storage, document.clone(), GroupCommitPolicy::default(), 2, &mut fixed).await {
        Err(rejected) => assert!(matches!(rejected_open_error(rejected).await, DbError::LimitExceeded("wal cursor fuel"))),
        Ok((mut opened, _)) => {
            opened.close().await.unwrap();
            panic!("one fixed step budget must not cover the whole chain");
        }
    }
    let mut stepped = WalCursorControl::stall_bounded(std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Duration::from_secs(30), step_fuel).unwrap();
    let (mut opened, report) = ArtifactWal::open_with_control(&storage, document.clone(), GroupCommitPolicy::default(), 3, &mut stepped).await.unwrap();
    assert_eq!(report.segments_seen as usize, segments);
    opened.close().await.unwrap();
    let (mut reopened, _) = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 4).await.unwrap();
    reopened.close().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn multi_segment_rotation_chains_prev_hash_and_replay_spans_segments() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
    wal.max_segment_bytes = 200;

    for i in 0..20u32 {
        submit_one(&storage, &mut wal, WalRecord::Command(retained(format!("cmd-{i}").as_bytes()).await), DurabilityClass::Fsync, u64::from(i)).await;
    }

    let segments = db_actor::block_on(storage.list_segments(&document)).unwrap();
    assert!(segments.len() >= 2, "the byte threshold must have forced at least one rotation");

    let seg0_bytes = segment_bytes(&storage, &document, 0).await;
    let seg0_report = protocol::format::recover(&seg0_bytes, &protocol::ProtocolLimits::default(), protocol::RecoveryMode::LastCommit).await.unwrap();
    let commit_frame_end = (seg0_report.last_commit_offset + protocol::format::COMMIT_FRAME_LEN) as usize;
    let mut cursor = protocol::FrameCursor::new(&seg0_bytes[seg0_report.last_commit_offset as usize..commit_frame_end], 0).await;
    let commit_frame = cursor.next_frame().await.unwrap().unwrap();
    let expected_chain_hash = protocol::format::parse_commit_payload(commit_frame.payload().await).unwrap().chain_hash;

    let full_replay = replay_summaries(&storage, &document).await;
    assert!(full_replay.contains(&ReplaySummary::Segment(0, None)));
    assert!(full_replay.contains(&ReplaySummary::Segment(1, Some(expected_chain_hash))));
    let commands_in_order: Vec<String> = full_replay
        .into_iter()
        .filter_map(|record| match record {
            ReplaySummary::Command(bytes) => Some(String::from_utf8(bytes).unwrap()),
            _ => None,
        })
        .collect();
    let expected: Vec<String> = (0..20u32).map(|i| format!("cmd-{i}")).collect();
    assert_eq!(commands_in_order, expected, "replay must span every segment in rotation order");
    wal.close().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn recovery_rejects_a_torn_non_active_sealed_segment() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let mut wal = db_actor::block_on(ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
    wal.max_segment_bytes = 1;
    submit_one(&storage, &mut wal, WalRecord::Command(retained(b"forces-rotation").await), DurabilityClass::Fsync, 0).await;
    assert!(db_actor::block_on(storage.list_segments(&document)).unwrap().len() >= 2);
    wal.close().await.unwrap();

    let seg0_bytes = segment_bytes(&storage, &document, 0).await;
    let writer = storage.acquire_writer(&document).await.unwrap();
    storage.delete_segment(&writer, 0).await.unwrap();
    storage.create_segment(&writer, 0).await.unwrap();
    storage.append(&writer, 0, pages(&seg0_bytes[..seg0_bytes.len() - 1])).await.unwrap();
    storage.seal(&writer, 0).await.unwrap();
    writer.release().await.unwrap();

    let result = db_actor::block_on(ArtifactWal::open(&storage, document, GroupCommitPolicy::default(), 100));
    let rejected = match result {
        Err(rejected) => matches!(rejected_open_error(rejected).await, DbError::Corrupt(_)),
        Ok((mut wal, _)) => {
            wal.close().await.unwrap();
            false
        }
    };
    assert!(rejected, "a torn sealed (non-active) segment must be a hard recovery error");
}

#[semio_framework_async_macros::async_test]
async fn empty_document_open_creates_a_fresh_wal() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document = doc("doc-1").await;
    let (mut wal, report) = db_actor::block_on(ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 0)).unwrap();
    assert_eq!(report, WalRecoveryReport::default());
    assert_eq!(wal.active_segment_index().await, 0);
    assert_eq!(db_actor::block_on(storage.list_segments(&document)).unwrap(), vec![0]);
    wal.close().await.unwrap();
}

#[semio_framework_async_macros::async_test]
async fn artifact_wal_open_rejection_retains_exact_writer_for_close_or_same_owner_retry() {
    let inner = std::sync::Arc::new(db_storage::DbBackend::Memory(MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap()));
    let storage = db_storage::FaultStorage::new(inner).await;
    let document = ArtifactId::from("wal-open-retained-owner");
    storage.set_script(db_storage::FaultScript { fail_nth_write: Some(1), ..db_storage::FaultScript::default() }).await;
    let rejected = match ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).await {
        Err(rejected) => rejected,
        Ok(mut wal) => {
            wal.close().await.unwrap();
            panic!("faulted WAL create was admitted");
        }
    };
    assert!(matches!(rejected.error(), DbError::Io(_)));
    assert!(rejected.has_release_owner());
    assert!(matches!(storage.acquire_writer(&document).await, Err(DbError::Conflict(_))));
    assert!(matches!(rejected_open_error(rejected).await, DbError::Io(_)));

    let writer = storage.acquire_writer(&document).await.unwrap();
    storage.set_script(db_storage::FaultScript { fail_nth_write: Some(storage.append_calls().await + 1), ..db_storage::FaultScript::default() }).await;
    let mut open_control = control();
    let rejected = match ArtifactWal::open_acquired(&storage, writer, GroupCommitPolicy::default(), 1, &mut open_control).await {
        Err(rejected) => rejected,
        Ok((mut wal, _)) => {
            wal.close().await.unwrap();
            panic!("faulted acquired WAL open was admitted");
        }
    };
    assert!(matches!(rejected.error(), DbError::Io(_)));
    assert!(matches!(storage.acquire_writer(&document).await, Err(DbError::Conflict(_))));
    storage.set_script(db_storage::FaultScript::default()).await;
    let mut retry_control = control();
    let (mut wal, report) = rejected.retry_open(&storage, GroupCommitPolicy::default(), 2, &mut retry_control).await.unwrap();
    assert_eq!(report.segments_seen, 1);
    wal.close().await.unwrap();
    storage.acquire_writer(&document).await.unwrap().release().await.unwrap();
}
//#endregion 🔖️Segment + ArtifactWal

/// 🅿️ A parked writer is lent, never lost: once other documents took every writer slot of the backend — reclaiming the
/// least recently parked WAL writer first — a WAL acquires a fresh writer on its next write and appends exactly where it
/// left off, while a WAL whose active segment another writer sealed meanwhile refuses to append (`Conflict`) and leaves
/// the segment exactly as that writer left it.
#[semio_framework_async_macros::async_test]
async fn a_reclaimed_wal_writer_is_reacquired_and_verified_before_the_next_append() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let (kept, sealed) = (doc("reclaimed-writer-kept").await, doc("reclaimed-writer-sealed").await);
    let mut kept_wal = ArtifactWal::create(&storage, kept.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
    let mut sealed_wal = ArtifactWal::create(&storage, sealed.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
    submit_one(&storage, &mut kept_wal, WalRecord::Frontier(sample_frontier(&kept).await), DurabilityClass::Fsync, 1).await;
    submit_one(&storage, &mut sealed_wal, WalRecord::Frontier(sample_frontier(&sealed).await), DurabilityClass::Fsync, 1).await;
    let (kept_key, sealed_key) = (kept_wal.writer.as_ref().unwrap().key(), sealed_wal.writer.as_ref().unwrap().key());
    kept_wal.park_writer();
    sealed_wal.park_writer();
    let mut fillers = Vec::new();
    for ordinal in 0..db_io_core::WAL_WRITER_CAPACITY {
        fillers.push(storage.acquire_writer(&doc(&format!("reclaiming-filler-{ordinal}")).await).await.unwrap());
        let reclaimed = [kept_key, sealed_key].map(|key| db_io_core::parked_since(key).is_none());
        let expected = match (ordinal + 3).saturating_sub(db_io_core::WAL_WRITER_CAPACITY) {
            0 => [false, false],
            1 => [true, false],
            _ => [true, true],
        };
        assert_eq!(reclaimed, expected, "filler {ordinal}: the least recently parked writer is reclaimed first");
    }
    for filler in fillers {
        filler.release().await.unwrap();
    }
    let foreign = storage.acquire_writer(&sealed).await.expect("a reclaimed writer no longer owns its document");
    storage.seal(&foreign, 0).await.unwrap();
    foreign.release().await.unwrap();
    let sealed_len = storage.segment_len(&sealed, 0).await.unwrap();

    let receipt = submit_one(&storage, &mut kept_wal, WalRecord::Frontier(sample_frontier(&kept).await), DurabilityClass::Fsync, 2).await;
    assert!(receipt.committed && receipt.segment_index == 0 && receipt.tx_id == 2, "the kept WAL appends where it left off: {receipt:?}");
    let commits = replay_summaries(&storage, &kept).await.into_iter().filter(|summary| matches!(summary, ReplaySummary::Commit(..))).count();
    assert_eq!(commits, 2, "both transactions replay from one unbroken segment");

    let mut records = WalRecordBatch::new();
    assert!(records.push(WalRecord::Frontier(sample_frontier(&sealed).await)).is_ok());
    let refused = sealed_wal.submit(&storage, &[], &records, DurabilityClass::Fsync, 2).await;
    while records.close_step().unwrap() {}
    assert!(matches!(refused, Err(DbError::Conflict(ref detail)) if detail.contains("changed while its writer was reclaimed")), "{refused:?}");
    assert_eq!(storage.segment_state(&sealed, 0).await.unwrap(), db_io_core::WalSegmentState::Sealed);
    assert_eq!(storage.segment_len(&sealed, 0).await.unwrap(), sealed_len, "the refused WAL wrote nothing");

    kept_wal.close().await.unwrap();
    sealed_wal.close().await.unwrap();
    assert!(kept_wal.terminal_is_empty() && sealed_wal.terminal_is_empty());
}
