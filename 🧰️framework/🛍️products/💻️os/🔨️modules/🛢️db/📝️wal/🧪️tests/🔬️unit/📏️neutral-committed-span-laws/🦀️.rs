/// 📏️ Neutral committed span laws: the committed fixture kind, the transaction gate, the immutable source fragmentation, the committed cursor laws and the maximal command batch transaction.

fn committed_fixture_kind(kind: u8) -> &'static str {
    match kind {
        WAL_COMMAND => "command",
        WAL_FRONTIER => "frontier",
        WAL_SNAPSHOT_PUB => "snapshot",
        WAL_PAYLOAD => "cas",
        _ => panic!("unexpected fixture body kind"),
    }
}

#[semio_framework_async_macros::async_test]
async fn wal_transaction_gate_matches_neutral_committed_spans() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let document = ArtifactId::from("committed-fixture");
        let storage = committed_fixture_storage(row, &document).await;
        let mut gate = WalTransactionGate::new();
        let mut transactions = Vec::new();
        let mut recovery_abort = None;
        let mut failure = None;
        for (index, segment) in row["segments"].as_array().unwrap().iter().enumerate() {
            let bytes = segment_bytes(&storage, &document, index as u64).await;
            let mut source = pages(&bytes);
            let mut offset = protocol::format::HEADER_SIZE;
            let result: Result<(), DbError> = async {
                let prior = if index == 0 { WalPriorChainTip::Genesis } else { WalPriorChainTip::RetainedBoundary };
                let mut chain = WalSegmentChain::new(&source, index as u64, prior)?;
                while !chain.step(&source, bytes.len(), &document, &mut control())? {}
                loop {
                    let frame = match wal_next_verified_page_frame(&source, &mut offset, bytes.len(), &mut control())? {
                        WalVerifiedFrameStep::Frame(frame) => frame,
                        WalVerifiedFrameStep::PhysicalCommit => continue,
                        WalVerifiedFrameStep::Done => break,
                    };
                    if gate.push(&source, frame)? {
                        let kinds: Vec<_> = gate.frames.iter().map(|frame| committed_fixture_kind(frame.kind)).collect();
                        transactions.push(serde_json::json!({ "id": gate.ready.unwrap().to_string(), "kinds": kinds }));
                        gate.release()?;
                    }
                    offset = frame.frame_end;
                }
                recovery_abort = gate.finish_segment(segment["state"] == "active" && index + 1 == row["segments"].as_array().unwrap().len())?;
                if index + 1 != row["segments"].as_array().unwrap().len() {
                    gate.advance_segment()?;
                }
                Ok(())
            }
            .await;
            while source.close_step().unwrap().is_some() {}
            if let Err(error) = result {
                failure = Some(match error {
                    DbError::LimitExceeded("wal transaction records") => "capacity",
                    DbError::LimitExceeded("wal transaction sequence") => "sequence",
                    DbError::Corrupt(_) => "corrupt",
                    other => panic!("unexpected fixture error: {other:?}"),
                });
                break;
            }
        }
        let actual = if let Some(error) = failure {
            serde_json::json!({ "accepted": false, "transactions": [], "nextTxId": null, "recoverAbort": null, "error": error })
        } else {
            serde_json::json!({ "accepted": true, "transactions": transactions, "nextTxId": gate.next_tx_id.to_string(), "recoverAbort": recovery_abort.map(|id| id.to_string()), "error": null })
        };
        assert_eq!(actual, row["expected"], "{}", row["name"]);
        if failure.is_none() && recovery_abort.is_none() {
            let mut cursor = replay_committed_document(&storage, &document, control()).await.unwrap();
            let mut replayed = Vec::new();
            loop {
                match cursor.next_transaction_step().await.unwrap() {
                    WalCommittedStep::Transaction(mut transaction) => {
                        let id = transaction.transaction_id().to_string();
                        let mut kinds = Vec::new();
                        loop {
                            match transaction.next_record_step().unwrap() {
                                WalCommittedRecordStep::Record(record) => {
                                    kinds.push(committed_fixture_kind(record.retained_shape().0));
                                }
                                WalCommittedRecordStep::Yield => continue,
                                WalCommittedRecordStep::Done => break,
                            }
                            while transaction.close_record_step().unwrap() {}
                        }
                        transaction.finish().unwrap();
                        replayed.push(serde_json::json!({ "id": id, "kinds": kinds }));
                    }
                    WalCommittedStep::Yield => {}
                    WalCommittedStep::Done => break,
                }
            }
            while cursor.close_owner_step().unwrap() {}
            assert!(cursor.terminal_is_empty());
            assert_eq!(serde_json::json!(replayed), row["expected"]["transactions"], "{}", row["name"]);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn wal_immutable_source_fragmentation_matches_neutral_transactions() {
    struct Fragments<'a> {
        bytes: &'a [u8],
        chunk: usize,
    }
    impl WalImmutableByteSource for Fragments<'_> {
        fn byte_len(&self) -> usize {
            self.bytes.len()
        }
        fn fragment_at(&self, offset: usize, limit: usize) -> Result<&[u8], DbError> {
            if offset >= limit || limit > self.bytes.len() {
                return Err(DbError::Corrupt("fixture immutable range".to_string()));
            }
            Ok(&self.bytes[offset..limit.min((offset / self.chunk + 1) * self.chunk)])
        }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["expected"]["accepted"] == true && row["expected"]["recoverAbort"].is_null()) {
        let document = ArtifactId::from("committed-fragmented-source");
        let storage = committed_fixture_storage(row, &document).await;
        for chunk in [1, 7, 4_093, 4_096, 0] {
            let mut gate = WalTransactionGate::new();
            let mut previous = WalPriorChainTip::Genesis;
            let mut output = Vec::new();
            for index in 0..row["segments"].as_array().unwrap().len() {
                let bytes = segment_bytes(&storage, &document, index as u64).await;
                let fragments = Fragments { bytes: &bytes, chunk: chunk.max(1) };
                let mut ranged = if chunk == 0 {
                    let mut padded = vec![0xad; 4_093];
                    padded.extend_from_slice(&bytes);
                    padded.extend_from_slice(&[0xda; 19]);
                    let mut writer = db_io_core::DbIoPageWriter::try_reserve(padded.len().div_ceil(db_io_core::DB_IO_PAGE_BYTES)).unwrap();
                    for fragment in padded.chunks(db_io_core::DB_IO_PAGE_BYTES) {
                        assert_eq!(writer.write_fragment(fragment).unwrap(), fragment.len());
                    }
                    let pages = writer.finish().unwrap();
                    let pages = pages.try_range(4_093).unwrap_or_else(|_| panic!("fixture range must retain its exact WAL"));
                    Some(pages.try_prefix(bytes.len()).unwrap_or_else(|_| panic!("fixture prefix must exclude trailing padding")))
                } else {
                    None
                };
                let source: &dyn WalImmutableByteSource = match ranged.as_ref() {
                    Some(pages) => pages,
                    None => &fragments,
                };
                assert_eq!(source.byte_len(), bytes.len());
                assert!(source.fragment_at(bytes.len(), bytes.len()).is_err());
                assert!(source.fragment_at(0, bytes.len() + 1).is_err());
                let owner = WalAuthenticatedSource::new(source, gate, index as u64, previous);
                let mut owner = match owner.finish() {
                    Err(owner) => owner,
                    Ok(_) => panic!("unverified source must retain its ownership"),
                };
                let mut opportunity = control();
                assert!(matches!(owner.next_step(&mut opportunity), Err(DbError::Corrupt(_))));
                let mut turns = 0;
                loop {
                    turns += 1;
                    assert!(turns <= bytes.len() + 2);
                    opportunity.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
                    if owner.verify_step(&document, &mut opportunity).unwrap() {
                        break;
                    }
                }
                loop {
                    opportunity.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
                    match owner.next_step(&mut opportunity).unwrap() {
                        WalAuthenticatedStep::Committed => {
                            let mut kinds = Vec::new();
                            for body in 0..64 {
                                let Some(frame) = owner.committed_frame(body) else {
                                    break;
                                };
                                kinds.push(committed_fixture_kind(frame.kind));
                            }
                            output.push(serde_json::json!({ "id": owner.gate.ready.unwrap().to_string(), "kinds": kinds }));
                            assert!(matches!(owner.next_step(&mut opportunity), Err(DbError::Corrupt(_))));
                            owner.finish_transaction().unwrap();
                        }
                        WalAuthenticatedStep::Yield => {}
                        WalAuthenticatedStep::Done => break,
                    }
                }
                let (_, next_gate, tip) = owner.finish().unwrap_or_else(|_| panic!("drained authenticated source must transfer ownership"));
                gate = next_gate;
                previous = WalPriorChainTip::Verified(tip);
                if let Some(pages) = ranged.as_mut() {
                    while !pages.terminal_is_empty() {
                        pages.close_step().unwrap();
                    }
                    assert!(pages.terminal_is_empty());
                }
            }
            assert_eq!(serde_json::json!(output), row["expected"]["transactions"], "{}: chunk={chunk}", row["name"]);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn wal_committed_cursor_single_fuel_and_expired_turns_match_neutral_transactions() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap().iter().filter(|row| row["expected"]["accepted"] == true && row["expected"]["recoverAbort"].is_null()) {
        let document = ArtifactId::from("committed-one-fuel");
        let storage = committed_fixture_storage(row, &document).await;
        let mut cursor = replay_committed_document(&storage, &document, control()).await.unwrap();
        let mut output = Vec::new();
        let mut turns = 0;
        loop {
            turns += 1;
            assert!(turns < 4_096, "single-fuel replay stopped progressing: {}", row["name"]);
            cursor.replenish(std::time::Instant::now(), 1).unwrap();
            assert!(matches!(cursor.next_transaction_step().await, Err(DbError::Unavailable(message)) if message == "wal cursor deadline reached"));
            cursor.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
            match cursor.next_transaction_step().await.unwrap() {
                WalCommittedStep::Transaction(mut transaction) => {
                    let id = transaction.transaction_id().to_string();
                    let mut kinds = Vec::new();
                    loop {
                        transaction.replenish(std::time::Instant::now() + std::time::Duration::from_secs(30), 1).unwrap();
                        match transaction.next_record_step().unwrap() {
                            WalCommittedRecordStep::Record(record) => kinds.push(committed_fixture_kind(record.retained_shape().0)),
                            WalCommittedRecordStep::Yield => continue,
                            WalCommittedRecordStep::Done => break,
                        }
                        while transaction.close_record_step().unwrap() {}
                    }
                    transaction.finish().unwrap();
                    output.push(serde_json::json!({ "id": id, "kinds": kinds }));
                }
                WalCommittedStep::Yield => {}
                WalCommittedStep::Done => break,
            }
        }
        while cursor.close_owner_step().unwrap() {}
        assert!(cursor.terminal_is_empty());
        assert_eq!(serde_json::json!(output), row["expected"]["transactions"], "{}", row["name"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn wal_committed_cursor_cancel_resume_keeps_transaction_position() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "two-commands-only-after-logical-commit").unwrap();
    let document = ArtifactId::from("committed-cancel");
    let storage = committed_fixture_storage(row, &document).await;
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let opportunity = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
    let mut cursor = replay_committed_document(&storage, &document, opportunity).await.unwrap();
    loop {
        match cursor.next_transaction_step().await.unwrap() {
            WalCommittedStep::Transaction(mut transaction) => {
                assert_eq!(transaction.record_count(), 2);
                assert!(matches!(transaction.next_record_step().unwrap(), WalCommittedRecordStep::Yield));
                let offset = transaction.cursor.raw.offset;
                cancelled.store(true, std::sync::atomic::Ordering::Release);
                assert!(matches!(transaction.next_record_step(), Err(DbError::Unavailable(message)) if message == "wal cursor cancelled"));
                assert_eq!(transaction.cursor.raw.offset, offset);
                assert_eq!(transaction.cursor.record_index, 0);
                cancelled.store(false, std::sync::atomic::Ordering::Release);
                let mut output = Vec::new();
                loop {
                    match transaction.next_record_step().unwrap() {
                        WalCommittedRecordStep::Record(WalRecord::Command(bytes)) => output.push(read_retained(bytes).await),
                        WalCommittedRecordStep::Record(_) => panic!("expected command"),
                        WalCommittedRecordStep::Yield => continue,
                        WalCommittedRecordStep::Done => break,
                    }
                    while transaction.close_record_step().unwrap() {}
                }
                assert_eq!(output, vec![vec![2], vec![3]]);
                transaction.finish().unwrap();
                break;
            }
            WalCommittedStep::Yield => {}
            WalCommittedStep::Done => panic!("committed transaction was skipped"),
        }
    }
    while cursor.close_owner_step().unwrap() {}
    assert!(cursor.terminal_is_empty());
}

#[semio_framework_async_macros::async_test]
async fn wal_committed_cursor_unfinished_borrow_poison_and_cancelled_close() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧾️committed-transactions/🔣️.json")).unwrap();
    let row = fixture["cases"].as_array().unwrap().iter().find(|row| row["name"] == "two-commands-only-after-logical-commit").unwrap();
    let document = ArtifactId::from("committed-drop");
    let storage = committed_fixture_storage(row, &document).await;
    for decoded in [false, true] {
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let opportunity = WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536).unwrap();
        let mut cursor = replay_committed_document(&storage, &document, opportunity).await.unwrap();
        loop {
            match cursor.next_transaction_step().await.unwrap() {
                WalCommittedStep::Transaction(mut transaction) => {
                    assert!(matches!(transaction.next_record_step().unwrap(), WalCommittedRecordStep::Yield));
                    if decoded {
                        loop {
                            match transaction.next_record_step().unwrap() {
                                WalCommittedRecordStep::Record(WalRecord::Command(_)) => break,
                                WalCommittedRecordStep::Yield => {}
                                _ => panic!("unfinished-record fixture lost its first command"),
                            }
                        }
                    }
                    drop(transaction);
                    break;
                }
                WalCommittedStep::Yield => {}
                WalCommittedStep::Done => panic!("committed transaction was skipped"),
            }
        }
        assert!(matches!(cursor.next_transaction_step().await, Err(DbError::Corrupt(_))));
        cancelled.store(true, std::sync::atomic::Ordering::Release);
        while cursor.close_owner_step().unwrap() {
            assert!(cancelled.load(std::sync::atomic::Ordering::Acquire));
        }
        assert!(cursor.terminal_is_empty());
    }
}

/// ⚖️ C12 P1 (ticket 26/09/23 session 14): the document backbone's declared maximal batch —
/// `DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES` commands filling `DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES` — is one
/// transaction appended from the commands' own bytes. Staged as `WalBytes` it needed one DB I/O page per command
/// under a single operation's credit and was refused. Recovery (`ArtifactWal::open`) reopens it — the transaction gate
/// held 64 frames — and replay answers every command byte for byte; one record past `WAL_TRANSACTION_RECORDS_MAX` is
/// refused before any I/O.
#[semio_framework_async_macros::async_test]
async fn a_declared_maximal_command_batch_is_one_transaction_replayed_exactly() {
    let storage = MemoryStorage::new(db_io_driver::db_io_test_pool()).await.unwrap();
    let document = doc("maximal-command-batch").await;
    let mut wal = ArtifactWal::create(&storage, document.clone(), GroupCommitPolicy::default(), 0).await.unwrap();
    let count = protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES;
    let each = protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES / count;
    let commands: Vec<Vec<u8>> = (0..count as u64)
        .map(|index| {
            let mut command = index.to_le_bytes().to_vec();
            command.resize(each, 0xC1);
            command
        })
        .collect();
    assert_eq!(commands.iter().map(Vec::len).sum::<usize>(), protocol::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES);
    let receipt = wal.submit(&storage, &commands, &WalRecordBatch::new(), DurabilityClass::Fsync, 1).await.unwrap();
    assert!(receipt.committed);
    assert_eq!((receipt.tx_id, receipt.segment_index), (1, 0));
    let over: Vec<Vec<u8>> = vec![vec![0xC2]; WAL_TRANSACTION_RECORDS_MAX];
    let mut frontier = WalRecordBatch::new();
    assert!(frontier.push(WalRecord::Frontier(sample_frontier(&document).await)).is_ok());
    let before = segment_bytes(&storage, &document, 0).await;
    assert!(matches!(wal.submit(&storage, &over, &frontier, DurabilityClass::Fsync, 2).await, Err(DbError::LimitExceeded("wal transaction records"))));
    assert_eq!(segment_bytes(&storage, &document, 0).await, before, "a refused transaction writes nothing");
    while frontier.close_step().unwrap() {}
    wal.close().await.unwrap();
    let (mut reopened, _) = ArtifactWal::open(&storage, document.clone(), GroupCommitPolicy::default(), 3).await.unwrap();
    assert_eq!(reopened.next_tx_id, 2);
    reopened.close().await.unwrap();
    let summaries = replay_summaries(&storage, &document).await;
    let replayed: Vec<&Vec<u8>> = summaries.iter().filter_map(|summary| if let ReplaySummary::Command(bytes) = summary { Some(bytes) } else { None }).collect();
    assert_eq!(replayed, commands.iter().collect::<Vec<_>>());
    assert!(summaries.contains(&ReplaySummary::Commit(1, count as u32)));
}
