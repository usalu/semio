//! 🤝️ Artifact handle: `ArtifactHandle` with its submit, history and compaction entry points. See [`super`].

use super::*;

//#region 🔖️ArtifactHandle
/// 🎭️ The frozen `ArtifactHandle`: a clone-cheap live handle to one open document.
#[derive(Clone)]
pub struct ArtifactHandle {
    pub authority: Arc<db_artifact::ArtifactAuthority>,
    pub _lease: ArtifactHandleLease,
    pub document: protocol::ArtifactId,
    pub pool: Arc<WorkerPool>,
}

impl ArtifactHandle {
    /// 📓️ Creates the typed Store journal port backed by this document authority's
    /// already-retained WAL writer; no generic command submission or second permit is exposed.
    pub fn durable_group_journal_sink(&self, now_ms: u64) -> Box<dyn store::durable_group::DurableOwnedGroupJournalSinkV1> {
        Box::new(ArtifactHandleJournalSinkV1 { sink: self.authority.durable_group_journal_sink(now_ms), _lease: self._lease.clone() })
    }

    /// 🧭 Replays committed fixed-three decisions through this document's existing actor and WAL
    /// owner. The returned owner retains all three Stores if its awaiting caller is cancelled.
    pub fn durable_group_recovery_retained<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        &self,
        admission: store::durable_group::DurableOwnedMapRecoveryAdmissionV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
    ) -> db_artifact_contract::ArtifactDurableGroupRecoveryOwnerV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
    where
        ParentP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        DrawingP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        ValueP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
    {
        self.authority.durable_group_recovery_retained(db_ids::ArtifactId(self.document.0.clone()), admission)
    }

    pub fn resume_durable_group_recovery<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        &self,
        owner: &mut db_artifact_contract::ArtifactDurableGroupRecoveryOwnerV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
    ) -> Result<(), DbError>
    where
        ParentP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        DrawingP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        ValueP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
    {
        self.authority.resume_durable_group_recovery(owner)
    }

    /// ✍️ The frozen `submit`: commits `batch` through the document's real
    /// `ArtifactAuthority` mailbox. Admission retains the exact request owner, and every I/O-lane
    /// grant advances either request-to-mailbox handoff or one actor-future poll.
    pub fn submit(&self, batch: db_artifact_contract::CommandBatch, options: db_artifact_contract::SubmitOptions) -> SubmitFuture {
        SubmitFuture::submit(self, batch, options)
    }

    /// 🧹 Queues one bounded compaction turn behind prior document commands.
    pub fn compact_retained(
        &self,
        holder: db_io_core::DbIoText,
        consolidate_snapshots: bool,
        budget: db_compact::CompactionBudget,
        now_ms: u64,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
    ) -> ArtifactCompactionFuture {
        ArtifactCompactionFuture { ask: self.authority.compact_retained(holder, consolidate_snapshots, budget, now_ms, cancelled), _lease: self._lease.clone() }
    }

    pub async fn compact(&self, holder: &str, consolidate_snapshots: bool, cancelled: Arc<std::sync::atomic::AtomicBool>) -> Result<db_compact::CompactionReport, DbError> {
        let holder = db_io_core::DbIoText::try_from_str(holder)?;
        self.compact_retained(holder, consolidate_snapshots, db_compact::CompactionBudget::default(), now_ms().await, cancelled).await?
    }

    /// 🔎️ The frozen `query`. `Consistency::Canonical` reads the document's live state
    /// directly. `AtLeast`/`Exact` read canonical too, then verify the resulting frontier actually
    /// satisfies the request (`DbError::Unavailable` if not — a true wait-for-frontier primitive
    /// would need a `ArtifactMessage` variant `db_artifact`'s mailbox doesn't expose yet).
    /// `Historical`/`Speculative`/`PreviewAugmented` are `DbError::Unimplemented` — see module doc.
    // 🔒️ `consistency`'s by-value signature is the frozen contract API
    // (`ArtifactHandle::query(&self, query: Query, consistency: Consistency)`, contract.md's
    // "Stable API" block) — not changeable even though this revision's body only borrows it.
    #[allow(clippy::needless_pass_by_value)]
    pub async fn query(&self, query: Query, consistency: Consistency) -> Result<QueryStream, DbError> {
        engine_query_maintenance_step()?;
        match &consistency {
            Consistency::Historical(_) | Consistency::PreviewAugmented(_) => {
                return Err(DbError::Unimplemented("historical/preview-augmented query consistency is not yet wired at the db_engine layer (db_query/db_projection integration deferred)"));
            }
            Consistency::Speculative(_) => {
                return Err(DbError::Unimplemented("speculative (preview) query consistency is not yet reachable: ArtifactAuthority's mailbox only exposes Submit/Query/Frontier messages"));
            }
            Consistency::Canonical | Consistency::AtLeast(_) | Consistency::Exact(_) => {}
        }

        let paths: Vec<String> = match query {
            Query::Get { path } => vec![path],
            Query::GetMany { paths } => paths,
        };
        if paths.len() > 64 {
            return Err(DbError::LimitExceeded("engine query result entries"));
        }
        let mut results = QueryStream::new();
        if !paths.is_empty() {
            results.retirement = Some(reserve_engine_query_retirement().ok_or_else(|| DbError::Unavailable("engine query retirement pressure refused query".to_string()))?);
        }
        for path in paths {
            let value = match self.authority.query(&path).await {
                Ok(value) => value,
                Err(error) => {
                    drop(results);
                    return Err(error);
                }
            };
            let path = match db_io_core::DbIoText::try_from_str(&path) {
                Ok(path) => path,
                Err(error) => {
                    drop(value);
                    drop(results);
                    return Err(error);
                }
            };
            results.push_preflighted(QueryResultEntry { path, value });
        }

        let frontier = match self.frontier().await {
            Ok(frontier) => frontier,
            Err(error) => {
                drop(results);
                return Err(error);
            }
        };
        match &consistency {
            Consistency::AtLeast(requested) if !frontier.dominates(requested)? => {
                drop(results);
                return Err(DbError::Unavailable("document has not yet reached the requested frontier".to_string()));
            }
            Consistency::Exact(requested) if &frontier != requested => {
                drop(results);
                return Err(DbError::Unavailable("document frontier does not exactly match the requested frontier".to_string()));
            }
            _ => {}
        }
        Ok(results)
    }

    /// 📡️ The frozen `subscribe` — see module doc's `//🎯️ Design choice`: always
    /// `DbError::Unimplemented`, a real (not faked) extension seam pending a `ArtifactMessage`
    /// variant `db_artifact` doesn't expose yet.
    pub async fn subscribe(&self, _spec: LiveQuerySpec) -> Result<LiveQuery, DbError> {
        Err(DbError::Unimplemented("live-query subscription is not yet reachable: ArtifactAuthority's mailbox only exposes Submit/Query/Frontier messages"))
    }

    /// 🧭️ The frozen `frontier`.
    pub async fn frontier(&self) -> Result<Frontier, DbError> {
        let core_frontier = self.authority.frontier().await?;
        Ok(to_engine_frontier(&core_frontier, self.document.clone()))
    }

    /// 📍️ Captures the exact committed frontier and tip identity in one serialized turn.
    pub async fn checkpoint_publication_snapshot(&self) -> Result<CheckpointPublicationSnapshot, DbError> {
        let snapshot = self.authority.checkpoint_publication_snapshot().await?;
        Ok(CheckpointPublicationSnapshot { authority_generation: self.authority.generation().0, frontier: to_engine_frontier(&snapshot.frontier, self.document.clone()), head_edit_id: snapshot.head_edit_id })
    }

    /// 🌫️ The frozen `preview` — see `subscribe`'s doc; same deferral reason.
    pub async fn preview(&self, _base: Frontier) -> Result<PreviewHandle, DbError> {
        Err(DbError::Unimplemented("preview publish/query is not yet reachable: ArtifactAuthority's mailbox only exposes Submit/Query/Frontier messages"))
    }

    /// 📜️ Replays history through the document authority. Each accepted process-pool grant
    /// advances one retained mailbox, WAL, envelope, or result-mapping opportunity.
    pub fn history(&self) -> HistoryFuture {
        HistoryFuture::submit(self)
    }

    pub fn history_terminal(&self, generation: u64) -> Option<ArtifactHistoryTerminalHandle> {
        artifact_history_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().find_map(|slot| slot.as_ref().filter(|state| state.generation == generation).map(|state| ArtifactHistoryTerminalHandle { state: state.clone() }))
    }

    /// 📸️ The frozen `snapshot_now` — see module doc's `//🎯️ Design choice`: always resolves
    /// to `DbError::Unimplemented`, a real extension seam (no full-state enumeration exists yet to
    /// serialize, and `db_snapshot` is not a direct dependency of this crate).
    pub async fn snapshot_now(&self, _kind: SnapshotKind) -> SnapshotFuture {
        let (reply_tx, reply_rx) = db_actor::oneshot();
        reply_tx.send(Err(DbError::Unimplemented("db_engine does not yet build real pack snapshots (no db_snapshot dependency this wave, and DocumentState exposes no full-state enumeration to serialize)")));
        reply_rx
    }

    pub async fn document_id(&self) -> &protocol::ArtifactId {
        &self.document
    }
}
//#endregion 🔖️ArtifactHandle
