//! ↩️ Engine undo, durable group, query and preview: receipts, index maintenance, undo, durable group decisions, compaction, snapshots, queries, conflict advice and previews. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️Engine
impl ArtifactEngine {
    /// 🧾️ Keeps `receipt` for a whole-batch resend, forgetting the oldest beyond [`APPLIED_RECEIPTS_MAX`].
    pub(super) fn remember_receipt(&mut self, receipt: &CommandReceipt) {
        if self.applied_receipts.insert(receipt.command_id.0.clone(), receipt.clone()).is_none() {
            self.applied_receipt_order.push_back(receipt.command_id.0.clone());
        }
        while self.applied_receipt_order.len() > APPLIED_RECEIPTS_MAX {
            if let Some(oldest) = self.applied_receipt_order.pop_front() {
                self.applied_receipts.remove(&oldest);
            }
        }
    }

    /// 🗂️ Writes every full index run the commits so far queued — their WAL records are the durable truth
    /// ([`ArtifactIndexBacklog`]). The document runner calls it after a commit's receipt was sent, so no Ack waits
    /// for index I/O; `submit` calls it first only when the backlog outgrew [`INDEX_BACKLOG_ENTRIES_MAX`].
    pub async fn maintain_index(&mut self) -> Result<(), DbError> {
        let index_facet = self.storage.index().await;
        self.index_backlog.flush(&index_facet, &self.document, false).await
    }

    /// ↩️ The inverse-undo pipeline: looks up `target`'s already-applied envelope, flips its
    /// `inverse` into a compensating envelope's `diff` (and vice versa, so the compensating
    /// envelope's OWN inverse can re-undo the undo), and submits it as a fresh, ordinary command
    /// depending on `target` — undo is just another commit, not a WAL rewrite. This is the crate's
    /// "inverse undo, using protocol's inverse-operation machinery".
    pub async fn undo(&mut self, target: &protocol::MutationId, undo_mutation_id: protocol::MutationId, actor: protocol::ActorId, now_ms: u64) -> Result<CommandReceipt, DbError> {
        let original = self.applied.get(&target.0).cloned().ok_or_else(|| DbError::NotFound(format!("operation {} not found for undo", target.0)))?;
        let undo_diff_entries = inverse_entries(&original.inverse).await?;
        let redo_inverse_entries = diff_entries(&original.diff).await?;
        let compensating = protocol::MutationEnvelope {
            mutation_id: undo_mutation_id,
            document_id: self.protocol_document.clone(),
            actor,
            dependencies: vec![target.clone()],
            observed: None,
            target: Vec::new(),
            diff: protocol::ArtifactDiff { schema: original.inverse.schema.clone(), payload: encode_pathmap(&entries_to_value(&undo_diff_entries)).await },
            inverse: protocol::InverseMutation { schema: original.diff.schema, payload: encode_pathmap(&entries_to_value(&redo_inverse_entries)).await },
            timestamp: protocol::HybridLogicalTimestamp::new(0, now_ms),
            transaction: None, verb: None, line: None,
        };
        self.submit(CommandBatch::new(vec![compensating]).await?, SubmitOptions::default(), now_ms).await
    }

    pub async fn get(&self, path: &str) -> Result<Option<db_query::QueryBytes>, DbError> {
        let Some(value) = self.state.get(path) else { return Ok(None) };
        let mut control = db_query::QueryCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        Ok(Some(db_query::QueryBytes::copy_from_pages(value, &mut control).await?))
    }

    pub async fn frontier(&self) -> Frontier {
        self.frontier.clone()
    }

    /// 📍️ Captures the frontier and its exact committed tip in one serialized actor turn.
    pub async fn checkpoint_publication_snapshot(&self) -> CheckpointPublicationSnapshot {
        CheckpointPublicationSnapshot { frontier: self.frontier.clone(), head_edit_id: self.head_edit_id.clone() }
    }

    pub fn history_replay(&self, operation_generation: u64, cancelled: Arc<std::sync::atomic::AtomicBool>, reservation: HistoryReplayReservation) -> HistoryReplayFuture {
        HistoryReplayFuture::new(self.storage.clone(), self.document.clone(), operation_generation, cancelled, reservation)
    }

    /// 📓️ Appends one Store-admitted fixed-three decision as a forced-Fsync event on this
    /// authority's already-retained WAL writer. Admission and capacity rejection complete before
    /// storage I/O; every error returned by `submit` remains an uncertain durable outcome.
    pub async fn append_durable_group_decision(&mut self, record: store::durable_group::DurableOwnedGroupJournalRecordV1, cancelled: Arc<std::sync::atomic::AtomicBool>, now_ms: u64) -> Result<ArtifactDurableGroupJournalAppendV1, DbError> {
        if let Some(receipt) = self.durable_group_receipts.get(record.decision_sha256()) {
            return Ok(ArtifactDurableGroupJournalAppendV1::Committed(receipt.clone()));
        }
        if cancelled.load(std::sync::atomic::Ordering::Acquire) {
            return Ok(ArtifactDurableGroupJournalAppendV1::Absent);
        }
        let head_edit_id = protocol::MutationId(record.parent_edit_id().to_string());
        let next_frontier = Frontier {
            document: self.document.clone(),
            head_seq: self.frontier.head_seq.checked_add(1).ok_or_else(|| DbError::LimitExceeded("durable group head sequence"))?,
            commit_seq: self.frontier.commit_seq.checked_add(1).ok_or_else(|| DbError::LimitExceeded("durable group commit sequence"))?,
            chain_hash: record.parent_post_revision(),
            epoch: self.frontier.epoch,
        };
        let (canonical_pack, decision_sha256, anchor_sha256, document) = record.into_parts();
        if document.artifact_id != self.protocol_document.0 {
            return Ok(ArtifactDurableGroupJournalAppendV1::Rejected(DbError::InvalidArgument("durable group decision anchor targets a different document".to_string())));
        }
        let mut control = db_wal_log::WalCursorControl::new(cancelled.clone(), std::time::Instant::now() + std::time::Duration::from_secs(30), 1_000_000)?;
        let event = match admit_wal_bytes(canonical_pack, db_io_core::DB_IO_MAX_READ_BYTES, &mut control).await {
            Ok(event) => event,
            Err(error) => return Ok(ArtifactDurableGroupJournalAppendV1::Rejected(error)),
        };
        let mut records = db_wal_log::WalRecordBatch::new();
        if let Err(error) = push_wal_record(&mut records, db_wal_log::WalRecord::Event(event), &mut control).await {
            let close = close_wal_record_batch(&mut records).await;
            return Ok(ArtifactDurableGroupJournalAppendV1::Rejected(close.err().unwrap_or(error)));
        }
        if let Err(error) = self.wal.preflight_submit(&[], &records) {
            let close = close_wal_record_batch(&mut records).await;
            return Ok(ArtifactDurableGroupJournalAppendV1::Rejected(close.err().unwrap_or(error)));
        }
        if cancelled.load(std::sync::atomic::Ordering::Acquire) {
            close_wal_record_batch(&mut records).await?;
            return Ok(ArtifactDurableGroupJournalAppendV1::Absent);
        }
        let wal_facet = self.storage.wal().await;
        let append = self.wal.submit(&wal_facet, &[], &records, DurabilityClass::Fsync, now_ms).await;
        drop(wal_facet);
        let close = close_wal_record_batch(&mut records).await;
        let receipt = match (append, close) {
            (Ok(receipt), Ok(())) => receipt,
            (Err(error), Ok(())) => return Err(error),
            (Ok(_), Err(error)) => return Err(error),
            (Err(error), Err(close_error)) => return Err(DbError::Internal(format!("durable group WAL append failed: {error}; record retirement failed: {close_error}"))),
        };
        if !receipt.committed {
            return Err(DbError::Internal("forced-Fsync durable group decision did not commit".to_string()));
        }
        let receipt = store::durable_group::DurableOwnedGroupJournalReceiptV1 { anchor_sha256, decision_sha256, transaction_id: receipt.tx_id, segment_index: receipt.segment_index };
        self.frontier = next_frontier;
        self.durable_group_edit_ids.insert(head_edit_id.0.clone());
        remember_recent_touch(&mut self.recent_touches, durable_group_touch(&head_edit_id.0));
        self.head_edit_id = Some(head_edit_id);
        self.durable_group_receipts.insert(receipt.decision_sha256.clone(), receipt.clone());
        Ok(ArtifactDurableGroupJournalAppendV1::Committed(receipt))
    }

    /// 🧭 Replays every committed fixed-three Event in physical WAL order into one opaque
    /// retained target. The target, cursor pages, transaction gate and current Store recovery
    /// never cross the caller reply boundary.
    pub async fn recover_durable_group_decisions(&mut self, mut driver: ArtifactDurableGroupRecoveryDriverV1) -> Result<(), DbError> {
        if driver.owner.document() != &self.document {
            return Err(DbError::InvalidArgument("durable group recovery target differs from the document authority".to_string()));
        }
        while !driver.owner.drive_recovery_step()? {
            semio_framework_async::yield_once().await;
        }
        let wal_facet = self.storage.wal().await;
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let control = db_wal_log::WalCursorControl::new(cancelled, std::time::Instant::now() + std::time::Duration::from_secs(2), 65_536)?;
        let mut replay = db_wal_log::replay_committed_document(&wal_facet, &self.document, control).await?;
        let outcome = async {
            loop {
                replay.replenish(std::time::Instant::now() + std::time::Duration::from_secs(2), 65_536)?;
                let transaction = match replay.next_transaction_step().await? {
                    db_wal_log::WalCommittedStep::Transaction(transaction) => transaction,
                    db_wal_log::WalCommittedStep::Yield => {
                        semio_framework_async::yield_once().await;
                        continue;
                    }
                    db_wal_log::WalCommittedStep::Done => break,
                };
                if let Some(witness) = committed_durable_group_decision_from_transaction(&self.document, transaction).await? {
                    driver.owner.accept(witness)?;
                    while !driver.owner.drive_recovery_step()? {
                        semio_framework_async::yield_once().await;
                    }
                }
            }
            driver.owner.finish_scan()
        }
        .await;
        loop {
            replay.replenish(std::time::Instant::now() + std::time::Duration::from_secs(2), 65_536)?;
            if !replay.close_owner_step()? {
                break;
            }
            semio_framework_async::yield_once().await;
        }
        if !replay.terminal_is_empty() {
            return Err(DbError::Internal("durable group WAL recovery cursor did not retire terminally".to_string()));
        }
        outcome
    }

    /// 🧹 Runs compaction inside this document authority while its WAL writer stays retained.
    pub async fn compact_retained(
        &mut self,
        holder: db_io_core::DbIoText,
        consolidate_snapshots: bool,
        budget: db_compact::CompactionBudget,
        now_ms: u64,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
    ) -> Result<db_compact::CompactionReport, DbError> {
        let index_facet = self.storage.index().await;
        self.index_backlog.flush(&index_facet, &self.document, true).await?;
        drop(index_facet);
        db_compact::retained_compaction_with_wal(self.storage.clone(), &mut self.wal, holder, consolidate_snapshots, budget, now_ms, cancelled).await
    }

}

impl ArtifactEngine {
    //#region 🔖️Snapshot
    /// 📸️ Publishes a new `db_snapshot` generation of the whole current `DocumentState` —
    /// new this revision; the counterpart `open` reads back to accelerate materialization.
    pub async fn snapshot_now(&self, now_ms: u64) -> Result<u64, DbError> {
        let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let encoder = StatePageEncodeCursor::try_new(&self.state.values, cancelled, std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        let page = db_state::Page::try_from_pages(encoder.finish().await?).await?;
        let snapshot_facet = self.storage.snapshot().await;
        let snapshot_manager = db_snapshot::SnapshotManager::new(&snapshot_facet).await;
        let origin = if snapshot_manager.load_latest(&self.document).await?.is_some() { db_snapshot::SnapshotOrigin::Incremental } else { db_snapshot::SnapshotOrigin::FullBaseline };
        let body = db_snapshot::SnapshotBody {
            head_seq: self.frontier.head_seq,
            commit_seq: self.frontier.commit_seq,
            epoch: self.frontier.epoch,
            chain_hash: self.frontier.chain_hash,
            protocol_version: 1,
            base_pack_hash: None,
            roots: vec![page.hash],
            created_at_ms: now_ms,
        };
        snapshot_manager.publish(&self.document, origin, &[page], body).await
    }
    //#endregion 🔖️Snapshot

    //#region 🔖️Query
    /// 🔎️ One-shot query over the document's current materialized state, resolved under
    /// `consistency` via `db_index`'s `CommitIndex`/`FrontierIndex`. `StateQuerySource` always reads
    /// the CURRENT canonical state regardless of what `consistency` resolved to — a true
    /// point-in-time replay is `db_engine`'s documented deferred extension (see its own module doc).
    // 🔒️ Mirrors `submit`'s ownership rationale above: `ArtifactMessage::RunQuery` already owns
    // `query`/`consistency` one level up on the actor-mailbox side, and query descriptors are
    // small, cheap-to-move value types — taking them by reference here would only push the
    // ownership question onto every caller for no benefit.
    #[allow(clippy::needless_pass_by_value)]
    pub async fn query(&self, query: db_query::Query, consistency: db_query::Consistency) -> Result<db_query::QueryResult, DbError> {
        let index_facet = self.storage.index().await;
        let resolver = ArtifactConsistencyResolver {
            current: self.frontier.clone(),
            pending: &self.index_backlog.frontiers,
            commits: db_index::CommitIndex::new(&index_facet, self.document.clone()).await,
            frontiers: db_index::FrontierIndex::new(&index_facet, self.document.clone()).await,
        };
        // A fresh document has no recorded frontier yet; canonical reads still succeed via the
        // in-memory frontier, so only consult the resolver for modes that truly need the index.
        if !matches!(consistency, db_query::Consistency::Canonical) {
            db_query::resolve_consistency(&consistency, &resolver).await?;
        }
        let source = StateQuerySource(&self.state.values);
        let mut control = db_query::QueryCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536)?;
        db_query::execute(&query, &source, None::<&db_query::NoFullTextLookup>, &db_query::QueryLimits::default(), &mut control).await
    }

    /// 📡️ Registers a live query, returning its subscription id — new this revision.
    pub async fn subscribe(&mut self, spec: db_query::LiveQuerySpec) -> u64 {
        let id = self.next_live_query_id;
        self.next_live_query_id += 1;
        self.live_queries.insert(id, db_query::LiveQuery::new(spec).await);
        id
    }

    pub async fn unsubscribe(&mut self, id: u64) {
        self.live_queries.remove(&id);
    }

    /// 📡️ Live-query notify: re-evaluates every registered live query and returns what
    /// changed. Called automatically at the end of `submit`; also callable directly.
    pub async fn refresh_live_queries(&mut self) -> Vec<(u64, db_query::QueryDiff)> {
        let source = StateQuerySource(&self.state.values);
        let limits = db_query::QueryLimits::default();
        let mut diffs = Vec::new();
        for (id, live_query) in self.live_queries.iter_mut() {
            let mut control = match db_query::QueryCursorControl::new(Arc::new(std::sync::atomic::AtomicBool::new(false)), std::time::Instant::now() + std::time::Duration::from_secs(30), 65_536) {
                Ok(control) => control,
                Err(_) => continue,
            };
            if let Ok(diff) = live_query.refresh(&source, None::<&db_query::NoFullTextLookup>, &limits, &mut control).await {
                if !diff.added.is_empty() || !diff.removed.is_empty() || !diff.updated.is_empty() {
                    diffs.push((*id, diff));
                }
            }
        }
        diffs
    }
    //#endregion 🔖️Query

    //#region 🔖️Advisory
    /// 🔮️ Advisory-only, real `db_conflict::ConflictDetector` integration: runs `batch`'s
    /// envelopes' touched regions against recent commit history WITHOUT executing anything, using
    /// the family's real bloom-filter/kind-matrix machinery — a caller (e.g. a UI) can call this
    /// before `submit` to preview likely conflicts. `submit`'s own returned `ConflictRecord`s stay
    /// this crate's original path-granular last-writer detection (see `🔖️Conflict`'s doc on why
    /// `db_conflict`'s per-command-pair shape can't replace it without breaking `db_engine`'s frozen
    /// `CommandReceipt.conflicts` element type).
    pub async fn preview_conflicts(&self, batch: &CommandBatch) -> Result<Vec<db_conflict::ConflictRecord>, DbError> {
        let mut probe: Vec<db_conflict::CommandTouch> = self.recent_touches.iter().cloned().collect();
        for envelope in &batch.envelopes {
            let entries = diff_entries(&envelope.diff).await?;
            let touched = entries_touched(&entries);
            probe.push(command_touch(envelope, &touched));
        }
        Ok(db_conflict::ConflictDetector::new().detect(&probe))
    }
    //#endregion 🔖️Advisory

    //#region 🔖️Preview
    /// 🌫️ Publishes a new preview overlaying the CURRENT committed state — never durable
    /// (never touches the WAL), per the contract's preview law. Backed by a real
    /// `db_preview::PreviewStore` this revision (previously a local, minimal stand-in).
    pub async fn publish_preview(&mut self, entries: &[(String, Option<serde_json::Value>)], now_ms: u64) -> Result<db_preview::PreviewId, DbError> {
        let dsl_entries: Vec<(String, Option<DslValue>)> = entries.iter().map(|(path, value)| (path.clone(), value.as_ref().map(semio_framework_value::DslValue::from))).collect();
        let touched = entries_touched(&dsl_entries);
        let envelope = protocol::MutationEnvelope {
            mutation_id: protocol::MutationId(format!("preview-{}", entries.len())),
            document_id: self.protocol_document.clone(),
            actor: protocol::ActorId("preview".to_string()),
            dependencies: Vec::new(),
            observed: None,
            target: Vec::new(),
            diff: protocol::ArtifactDiff { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&entries_to_value(&dsl_entries)).await },
            inverse: protocol::InverseMutation { schema: protocol::SchemaId(DB_PATHMAP_SCHEMA.to_string()), payload: encode_pathmap(&semio_framework_value::DslValue::Object(vec![])).await },
            timestamp: protocol::HybridLogicalTimestamp::new(0, now_ms),
            transaction: None, verb: None, line: None,
        };
        self.previews.publish(db_preview::PublishPreviewRequest { document: self.document.clone(), actor: ActorId("preview".to_string()), key: format!("preview-{now_ms}"), base: self.frontier.clone(), envelope, touched, ttl_ms: None, now_ms })
    }

    /// 🌫️ The value a preview would show at `path`: the preview's own diff if it touches
    /// `path`, else falling through to the committed state.
    pub async fn preview_get(&self, id: &db_preview::PreviewId, path: &str) -> Result<Option<db_query::QueryBytes>, DbError> {
        let preview = self.previews.get(id).ok_or_else(|| DbError::NotFound(format!("preview {id} not found")))?;
        for (entry_path, value) in diff_entries(&preview.envelope.diff).await? {
            if entry_path == path {
                return match value {
                    Some(dsl_value) => {
                        let bytes = store::pack_rt::encode_wire_value(&dsl_value);
                        let mut writer = db_io_core::DbIoPageWriter::try_reserve(bytes.capacity().div_ceil(db_io_core::DB_IO_PAGE_BYTES)).map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
                        let mut offset = 0;
                        while offset < bytes.len() {
                            offset += writer.write_fragment(&bytes[offset..])?;
                        }
                        let pages = writer.seal_retained().await.map_err(db_io_core::DbIoPageWriterRejected::into_error)?;
                        Ok(Some(db_query::QueryBytes::from_pages(pages).map_err(|(error, mut pages)| {
                            let _ = pages.close_step();
                            drop(pages);
                            error
                        })?))
                    }
                    None => Ok(None),
                };
            }
        }
        self.get(path).await
    }

    pub async fn preview_status(&self, id: &db_preview::PreviewId) -> Result<db_preview::PreviewState, DbError> {
        Ok(self.previews.get(id).ok_or_else(|| DbError::NotFound(format!("preview {id} not found")))?.state)
    }

    pub async fn withdraw_preview(&mut self, id: &db_preview::PreviewId) -> Result<(), DbError> {
        self.previews.withdraw(id)
    }

    pub async fn reject_preview(&mut self, id: &db_preview::PreviewId) -> Result<(), DbError> {
        self.previews.reject(id)
    }

    pub async fn expire_previews(&mut self, now_ms: u64) -> Vec<db_preview::PreviewId> {
        self.previews.sweep_expired(now_ms)
    }
    //#endregion 🔖️Preview
}
//#endregion 🔖️Engine
//#endregion 🔖️Ids
