//! 🌱️ Engine lifecycle: close, park, create, open and assemble of the engine. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️Engine
impl ArtifactEngine {
    /// 🔕️ One close step: progress wakes the caller's waker, a pending writer release parks it
    /// in the release signal until the backend's terminal notification.
    pub fn poll_close(&mut self, context: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), DbError>> {
        if self.wal.poll_close(context)?.is_pending() {
            return std::task::Poll::Pending;
        }
        if self.state.values.close_step()? {
            context.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        std::task::Poll::Ready(Ok(()))
    }

    #[cfg(test)]
    pub async fn close(&mut self) -> Result<(), DbError> {
        std::future::poll_fn(|context| self.poll_close(context)).await
    }

    pub fn close_flush_pending(&self) -> bool {
        self.wal.has_pending()
    }

    /// 🅿️ The engine rests between turns: its WAL parks the document's writer (unless a group commit still waits), so a
    /// backend with more open documents than writers lends it to whichever document writes next.
    pub fn park_writer(&self) {
        self.wal.park_writer();
    }

    pub async fn close_flush(&mut self) -> Result<(), DbError> {
        let wal_facet = self.storage.wal().await;
        self.wal.close_flush(&wal_facet).await
    }

    /// 🌱️ Retained constructor used by the document authority. Every storage wait remains
    /// represented by this future so a pool worker only polls it once before yielding.
    pub async fn create_retained(document: protocol::ArtifactId, storage: Arc<db_storage::DbBackend>, config: ArtifactEngineConfig, now_ms: u64) -> Result<ArtifactEngine, ArtifactEngineOpenRejected> {
        let core_id = to_core_document_id(&document).await;
        let wal = db_wal::ArtifactWal::create(&storage.wal().await, core_id.clone(), db_wal_log::GroupCommitPolicy::default(), now_ms).await.map_err(ArtifactEngineOpenRejected::WalOpen)?;
        Ok(ArtifactEngine::assemble(document, core_id, storage, wal, config).await)
    }

    /// 🌱️ Creates a brand-new document: a genesis WAL (segment 0) and an empty state.
    /// Errors `AlreadyExists` if `document` already has WAL segments in `storage`.
    /// Process/test entry-point convenience; live document authorities use `create_retained`.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn create(document: protocol::ArtifactId, storage: Arc<db_storage::DbBackend>, config: ArtifactEngineConfig, now_ms: u64) -> Result<ArtifactEngine, ArtifactEngineOpenRejected> {
        db_actor::block_on(Self::create_retained(document, storage, config, now_ms))
    }

    /// 🚑️ Retained materialization as initial ⊕ snapshot ⊕ WAL suffix.
    pub async fn open_retained(document: protocol::ArtifactId, storage: Arc<db_storage::DbBackend>, config: ArtifactEngineConfig, now_ms: u64) -> Result<(ArtifactEngine, MaterializeReport), ArtifactEngineOpenRejected> {
        let core_id = to_core_document_id(&document).await;
        let mut report = MaterializeReport::default();

        let mut state = DocumentState::new();
        let mut applied_head_seq = 0u64;
        let snapshot_facet = storage.snapshot().await;
        let snapshot_manager = db_snapshot::SnapshotManager::new(&snapshot_facet).await;
        if let Some((generation, descriptor)) = snapshot_manager.load_latest(&core_id).await? {
            report.from_snapshot = true;
            report.snapshot_generation = Some(generation);
            let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
            let control = db_snapshot::SnapshotCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
            let mut snapshot_cursor = snapshot_manager.chain_cursor(&core_id, generation, control);
            for hash in &descriptor.roots {
                let mut page_bytes = snapshot_cursor.read_page(*hash).await?;
                let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
                let mut decoder = StatePageDecodeCursor::new(&page_bytes, cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
                while let Some(entry) = decoder.next().await? {
                    let replaced = match state.values.insert(entry) {
                        Ok(replaced) => replaced,
                        Err(mut rejected) => {
                            let _ = rejected.close_step()?;
                            return Err(ArtifactEngineOpenRejected::BeforeWal(DbError::LimitExceeded("retained state entries")));
                        }
                    };
                    if let Some(mut replaced) = replaced {
                        let _ = replaced.close_step()?;
                        drop(replaced);
                    }
                }
                let _ = decoder.close_step()?;
                drop(decoder);
                let _ = page_bytes.close_step()?;
                drop(page_bytes);
            }
            let _ = snapshot_cursor.close_step()?;
            applied_head_seq = descriptor.head_seq;
        }
        drop(snapshot_manager);
        drop(snapshot_facet);

        let (wal, wal_recovery) = db_wal::ArtifactWal::open(&storage.wal().await, core_id.clone(), db_wal_log::GroupCommitPolicy::default(), now_ms).await.map_err(ArtifactEngineOpenRejected::WalOpen)?;
        report.torn_tail_bytes = wal_recovery.torn_tail_bytes;
        let mut engine = ArtifactEngine::assemble(document, core_id.clone(), storage.clone(), wal, config).await;
        engine.state = state;
        engine.frontier.head_seq = applied_head_seq;

        let replay = async {
            let wal_facet = storage.wal().await;
            let replay_cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
            let replay_control = db_wal_log::WalCursorControl::new(replay_cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
            let watermarks = ArtifactIndexBacklog::watermarks(&storage.index().await, &core_id).await?;
            let mut records = db_wal_log::replay_committed_document(&wal_facet, &core_id, replay_control).await?;
            let mut decode_control = db_wal_log::WalCursorControl::stall_bounded(Arc::new(std::sync::atomic::AtomicBool::new(false)), db_wal_log::WAL_REPLAY_STEP_STALL_BOUND, db_wal_log::WAL_REPLAY_STEP_FUEL)?;
            let mut turn = db_wal_log::WalTurn::start();
            let mut batch_ids: HashSet<String> = HashSet::new();
            let mut seen: u64 = 0;
            let mut replay_frontier = Frontier::genesis(core_id.clone());
            let mut replay_head_edit_id = None;
            let mut replay_projected = false;
            let result = async {
                loop {
                    records.renew_step()?;
                    decode_control.renew_step()?;
                    let mut transaction = match records.next_transaction_step().await? {
                        db_wal_log::WalCommittedStep::Transaction(transaction) => transaction,
                        db_wal_log::WalCommittedStep::Yield => {
                            turn.step().await;
                            continue;
                        }
                        db_wal_log::WalCommittedStep::Done => break,
                    };
                    let transaction_id = transaction.transaction_id();
                    let segment_index = transaction.segment_index();
                    batch_ids.clear();
                    let mut frontier_seen = false;
                    let mut body_records = 0usize;
                    let mut durable_group = None;
                    loop {
                        let record = match transaction.next_record_step()? {
                            db_wal_log::WalCommittedRecordStep::Record(record) => record,
                            db_wal_log::WalCommittedRecordStep::Yield => {
                                turn.step().await;
                                continue;
                            }
                            db_wal_log::WalCommittedRecordStep::Done => break,
                        };
                        if frontier_seen {
                            return Err(DbError::Corrupt("artifact committed frontier is not terminal".to_string()));
                        }
                        body_records = body_records.checked_add(1).ok_or_else(|| DbError::LimitExceeded("artifact committed body records"))?;
                        if durable_group.is_some() {
                            return Err(DbError::Corrupt("durable group decision must be the sole committed transaction body".to_string()));
                        }
                        match record {
                            db_wal_log::WalRecord::Command(bytes) => {
                                let envelope = db_sync::decode_wal_command(bytes, &mut decode_control)?;
                                if envelope.document_id.0 != core_id.0 {
                                    return Err(DbError::Corrupt("artifact envelope document differs".to_string()));
                                }
                                replay_head_edit_id = Some(envelope.mutation_id.clone());
                                seen += 1;
                                batch_ids.insert(envelope.mutation_id.0.clone());
                                let actor = to_core_actor_id(&envelope.actor).await;
                                if seen <= applied_head_seq {
                                    *engine.actor_seq.entry(envelope.actor.0.clone()).or_insert(0) += 1;
                                    engine.applied.insert(envelope.mutation_id.0.clone(), envelope.clone());
                                } else {
                                    let touched = match engine.plan_one(&envelope, &batch_ids, &mut HashMap::new()).await? {
                                        Some(plan) => {
                                            engine.commit_one(&envelope, plan.entries).await?;
                                            plan.touched
                                        }
                                        None => db_state::TouchedSet::new(),
                                    };
                                    let touch = command_touch(&envelope, &touched);
                                    engine.remember_target(&envelope);
                                    remember_recent_touch(&mut engine.recent_touches, touch);
                                    report.commands_replayed += 1;
                                }
                                let actor_seq = *engine.actor_seq.get(&envelope.actor.0).unwrap_or(&0);
                                engine.index_backlog.queue_command(&watermarks, seen, db_index::RecordLocation { segment: segment_index, offset: seen, len: 1 }, actor, actor_seq);
                            }
                            db_wal_log::WalRecord::Frontier(frontier) => {
                                if frontier.document != core_id {
                                    return Err(DbError::Corrupt("artifact frontier document differs".to_string()));
                                }
                                engine.index_backlog.queue_frontier(&watermarks, frontier);
                                frontier_seen = true;
                                replay_frontier = frontier.clone();
                                replay_projected = true;
                            }
                            db_wal_log::WalRecord::Event(bytes) => {
                                if body_records != 1 {
                                    return Err(DbError::Corrupt("durable group decision must be the sole committed transaction body".to_string()));
                                }
                                let mut canonical_pack = Vec::with_capacity(bytes.len());
                                for fragment in bytes.fragments() {
                                    canonical_pack.extend_from_slice(fragment);
                                }
                                let record = store::durable_group::DurableOwnedGroupJournalRecordV1::admit_canonical(canonical_pack).map_err(|error| DbError::Corrupt(format!("committed durable group decision is invalid: {error}")))?;
                                if record.document().artifact_id != core_id.0 {
                                    return Err(DbError::Corrupt("committed durable group decision targets a different replay document".to_string()));
                                }
                                durable_group = Some(record);
                            }
                            _ => {}
                        }
                        while transaction.close_record_step()? {
                            turn.step().await;
                        }
                        turn.step().await;
                    }
                    if !batch_ids.is_empty() && !frontier_seen {
                        return Err(DbError::Corrupt("artifact committed commands have no frontier".to_string()));
                    }
                    if let Some(record) = durable_group {
                        if body_records != 1 {
                            return Err(DbError::Corrupt("durable group decision must be the sole committed transaction body".to_string()));
                        }
                        seen = seen.checked_add(1).ok_or_else(|| DbError::LimitExceeded("artifact replay head sequence"))?;
                        let receipt = store::durable_group::DurableOwnedGroupJournalReceiptV1 { anchor_sha256: record.anchor_sha256().to_string(), decision_sha256: record.decision_sha256().to_string(), transaction_id, segment_index };
                        let duplicate = engine.durable_group_receipts.insert(receipt.decision_sha256.clone(), receipt).is_some();
                        engine.durable_group_edit_ids.insert(record.parent_edit_id().to_string());
                        if !duplicate {
                            replay_frontier = Frontier {
                                document: core_id.clone(),
                                head_seq: replay_frontier.head_seq.checked_add(1).ok_or_else(|| DbError::LimitExceeded("durable group replay head sequence"))?,
                                commit_seq: replay_frontier.commit_seq.checked_add(1).ok_or_else(|| DbError::LimitExceeded("durable group replay commit sequence"))?,
                                chain_hash: record.parent_post_revision(),
                                epoch: replay_frontier.epoch,
                            };
                            replay_head_edit_id = Some(protocol::MutationId(record.parent_edit_id().to_string()));
                            remember_recent_touch(&mut engine.recent_touches, durable_group_touch(record.parent_edit_id()));
                            replay_projected = true;
                        }
                    }
                    transaction.finish()?;
                }
                if replay_projected {
                    engine.frontier = replay_frontier;
                    engine.head_edit_id = replay_head_edit_id;
                }
                Ok::<(), DbError>(())
            }
            .await;
            let closed = async {
                while records.close_owner_step()? {
                    semio_framework_async::yield_once().await;
                }
                Ok::<(), DbError>(())
            }
            .await;
            drop(records);
            drop(wal_facet);
            result.and(closed)
        }
        .await;
        if let Err(error) = replay {
            let state_close = async {
                while engine.state.values.close_step()? {
                    semio_framework_async::yield_once().await;
                }
                Ok::<(), DbError>(())
            }
            .await;
            let ArtifactEngine { wal, .. } = engine;
            if let Err(close_error) = state_close {
                return Err(ArtifactEngineOpenRejected::retained_wal_after_cleanup_error(error, close_error, wal));
            }
            return Err(ArtifactEngineOpenRejected::retained_wal(error, wal));
        }
        Ok((engine, report))
    }

    /// 🚑️ Process/test entry-point convenience; live authorities use `open_retained`.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn open(document: protocol::ArtifactId, storage: &Arc<db_storage::DbBackend>, config: ArtifactEngineConfig, now_ms: u64) -> Result<(ArtifactEngine, MaterializeReport), ArtifactEngineOpenRejected> {
        db_actor::block_on(Self::open_retained(document, storage.clone(), config, now_ms))
    }

    async fn assemble(protocol_document: protocol::ArtifactId, core_id: ArtifactId, storage: Arc<db_storage::DbBackend>, wal: db_wal::ArtifactWal, config: ArtifactEngineConfig) -> ArtifactEngine {
        let preview_budgets = db_preview::PreviewBudgets { default_ttl_ms: config.preview_ttl_ms, max_ttl_ms: config.preview_ttl_ms, ..db_preview::PreviewBudgets::default() };
        ArtifactEngine {
            document: core_id.clone(),
            protocol_document,
            storage,
            wal,
            state: DocumentState::new(),
            applied: HashMap::new(),
            applied_receipts: HashMap::new(),
            applied_receipt_order: VecDeque::new(),
            durable_group_receipts: HashMap::new(),
            durable_group_edit_ids: HashSet::new(),
            recent_targets: HashMap::new(),
            actor_seq: HashMap::new(),
            frontier: Frontier::genesis(core_id.clone()),
            head_edit_id: None,
            previews: db_preview::PreviewStore::new(core_id, preview_budgets),
            recent_touches: VecDeque::new(),
            live_queries: HashMap::new(),
            next_live_query_id: 0,
            index_backlog: ArtifactIndexBacklog::default(),
            config,
        }
    }

}
//#endregion 🔖️Engine
//#endregion 🔖️Ids
