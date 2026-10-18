//! 🎭️ Authority impl and terminal job: the `ArtifactAuthority` impl and the runner terminal job. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️Actor
#[cfg(not(target_arch = "wasm32"))]
impl ArtifactAuthority {
    /// 🚀️ Builds the engine on the injected pool and resolves only after construction, so
    /// a caller never receives an authority whose engine failed to open.
    pub async fn spawn<F: Future<Output = Result<Box<ArtifactEngine>, ArtifactEngineOpenRejected>> + Send + 'static>(
        pool: Arc<semio_framework_async::WorkerPool>,
        build: impl FnOnce() -> F + Send + 'static,
        capacities: MailboxCapacities,
    ) -> Result<ArtifactAuthority, ArtifactEngineOpenRejected> {
        let pool_use = pool.acquire_use().map_err(|error| ArtifactEngineOpenRejected::BeforeWal(DbError::Unavailable(format!("artifact authority WorkerPool use rejected: {error:?}"))))?;
        Self::spawn_with_pool_use(pool, pool_use, build, capacities).await
    }

    /// 🧵️ Mounts an authority under an already-retained process-pool use. Database document
    /// mounts pass their exact use cell through this boundary, so no second lifecycle admission
    /// can conflict after the catalog transaction has begun.
    pub async fn spawn_with_pool_use<F: Future<Output = Result<Box<ArtifactEngine>, ArtifactEngineOpenRejected>> + Send + 'static>(
        pool: Arc<semio_framework_async::WorkerPool>,
        pool_use: Arc<semio_framework_async::WorkerPoolUse>,
        build: impl FnOnce() -> F + Send + 'static,
        capacities: MailboxCapacities,
    ) -> Result<ArtifactAuthority, ArtifactEngineOpenRejected> {
        let retirement = ArtifactRunnerRetirementReservation::try_reserve(pool.clone()).map_err(ArtifactEngineOpenRejected::BeforeWal)?;
        let (address, receiver) = db_actor::mailbox::<ArtifactMessage>(capacities);
        let (ready_tx, ready_rx) = db_actor::oneshot::<Result<(), ArtifactEngineOpenRejected>>();
        let (done_tx, done_rx) = db_actor::oneshot();
        ARTIFACT_RUNNER_HANDOFF_POOL_USES.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        let handoff = Arc::new(ArtifactRunnerHandoff {
            pool: pool.clone(),
            pool_use: std::sync::Mutex::new(Some(pool_use.clone())),
            terminal_job: std::sync::Mutex::new(None),
            close_runner: std::sync::Mutex::new(None),
            retirement_maintenance: std::sync::Mutex::new(None),
            close_error: std::sync::Mutex::new(None),
            close_retry: std::sync::Mutex::new(ArtifactCloseRetry::clear()),
            #[cfg(test)]
            close_faults: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            close_polls: std::sync::atomic::AtomicUsize::new(0),
            #[cfg(test)]
            retirement_turns: std::sync::atomic::AtomicUsize::new(0),
            active_history: std::sync::atomic::AtomicBool::new(false),
            driver: std::sync::atomic::AtomicU8::new(ArtifactRunnerDriver::RunnableIdle as u8),
            terminal: std::sync::atomic::AtomicBool::new(false),
        });
        let runner = Arc::new(ArtifactRunner {
            pool,
            address: address.clone(),
            receiver,
            generation: address.generation().0,
            builder: std::sync::Mutex::new(Some(Box::pin(build()))),
            engine: std::sync::Mutex::new(None),
            turn: std::sync::Mutex::new(None),
            ready: std::sync::Mutex::new(Some(ready_tx)),
            done: std::sync::Mutex::new(Some(done_tx)),
            handoff: handoff.clone(),
            retry_job: std::sync::Mutex::new(None),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            retry_generation: std::sync::atomic::AtomicU64::new(1),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            close_driving: std::sync::atomic::AtomicBool::new(false),
            terminal: std::sync::atomic::AtomicBool::new(false),
        });
        let weak = Arc::downgrade(&runner);
        address.set_consumer_wake(Arc::new(move || {
            if let Some(runner) = weak.upgrade() {
                runner.schedule();
            }
        }));
        let weak = Arc::downgrade(&runner);
        *handoff.close_runner.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Arc::new(move || -> bool { weak.upgrade().is_none_or(|runner| runner.close_one()) }));
        let retirement_runner = runner.clone();
        let retirement_close: Arc<dyn Fn() -> bool + Send + Sync> = Arc::new(move || retirement_runner.close_one());
        let runner_for_cancel = runner.clone();
        let cancel: Arc<dyn Fn() + Send + Sync> = Arc::new(move || runner_for_cancel.cancel());
        runner.schedule();

        match ready_rx.await {
            Ok(Ok(())) => {
                ARTIFACT_AUTHORITY_POOL_USES.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                Ok(ArtifactAuthority { address, cancel, handoff, retirement: Some(retirement), retirement_close: Some(retirement_close), _pool_use: pool_use, done: std::sync::Mutex::new(Some(done_rx)) })
            }
            Ok(Err(err)) => Err(err),
            Err(_) => Err(ArtifactEngineOpenRejected::BeforeWal(DbError::Closed)),
        }
    }

    /// 📓️ Nonblocking retained fixed-three decision append through this authority's
    /// existing WAL owner. The opaque record can only be constructed by Store admission.
    pub fn append_durable_group_decision_retained(
        &self,
        record: store::durable_group::DurableOwnedGroupJournalRecordV1,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
        now_ms: u64,
    ) -> db_actor::AskFuture<ArtifactMessage, Result<ArtifactDurableGroupJournalAppendV1, DbError>> {
        self.address.ask(Priority::Command, |reply| ArtifactMessage::AppendDurableGroupDecision { record, cancelled, now_ms, reply })
    }

    /// 🔐️ Builds the Store journal port over this authority's clone-cheap mailbox address;
    /// it never acquires or opens another WAL writer.
    pub fn durable_group_journal_sink(&self, now_ms: u64) -> Box<dyn store::durable_group::DurableOwnedGroupJournalSinkV1> {
        Box::new(ArtifactDurableGroupJournalSinkV1 { address: self.address.clone(), now_ms })
    }

    /// 🧭 Starts one actor-serialized committed-Event replay while the returned owner retains
    /// every exact Store across caller cancellation.
    pub fn durable_group_recovery_retained<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        &self,
        document: ArtifactId,
        admission: store::durable_group::DurableOwnedMapRecoveryAdmissionV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
    ) -> ArtifactDurableGroupRecoveryOwnerV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>
    where
        ParentP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        DrawingP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        ValueP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
    {
        let (mut owner, driver) = ArtifactDurableGroupRecoveryOwnerV1::new(document, admission);
        owner.request = Some(Box::pin(self.address.ask(Priority::Command, |reply| ArtifactMessage::RecoverDurableGroupDecisions { driver, reply })));
        owner
    }

    pub fn resume_durable_group_recovery<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>(
        &self,
        owner: &mut ArtifactDurableGroupRecoveryOwnerV1<ParentP, ParentMutation, DrawingP, DrawingMutation, ValueP, ValueMutation>,
    ) -> Result<(), DbError>
    where
        ParentP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ParentMutation: store::Mutation<ParentP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        DrawingP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        DrawingMutation: store::Mutation<DrawingP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
        ValueP: pack::snapshot::ArtifactPack + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + 'static,
        ValueMutation: store::Mutation<ValueP> + Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + 'static,
    {
        if owner.request.is_some() || owner.shared.scan_complete.load(std::sync::atomic::Ordering::Acquire) {
            return Err(DbError::Conflict("durable group recovery scan is already active or complete".to_string()));
        }
        let driver = owner.driver();
        owner.request = Some(Box::pin(self.address.ask(Priority::Command, |reply| ArtifactMessage::RecoverDurableGroupDecisions { driver, reply })));
        Ok(())
    }

    /// 📨️ Nonblocking retained submit cursor used by `db_engine::SubmitFuture`.
    pub fn submit_retained(&self, batch: CommandBatch, options: SubmitOptions, now_ms: u64) -> db_actor::AskFuture<ArtifactMessage, Result<CommandReceipt, DbError>> {
        self.address.ask(Priority::Command, |reply| ArtifactMessage::Submit { batch, options, now_ms, reply })
    }

    pub fn history_retained(&self, operation_generation: u64, cancelled: Arc<std::sync::atomic::AtomicBool>, reservation: HistoryReplayReservation) -> db_actor::AskFuture<ArtifactMessage, Result<ArtifactHistoryView, DbError>> {
        self.address.ask(Priority::Query, |reply| ArtifactMessage::History { operation_generation, cancelled, reservation, reply })
    }

    pub fn compact_retained(
        &self,
        holder: db_io_core::DbIoText,
        consolidate_snapshots: bool,
        budget: db_compact::CompactionBudget,
        now_ms: u64,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
    ) -> db_actor::AskFuture<ArtifactMessage, Result<db_compact::CompactionReport, DbError>> {
        self.address.ask(Priority::Command, |reply| ArtifactMessage::Compact { holder, consolidate_snapshots, budget, now_ms, cancelled, reply })
    }

    pub fn generation(&self) -> GenerationId {
        self.address.generation()
    }

    pub fn take_terminal_job(&self) -> Option<ArtifactRunnerTerminalJob> {
        if self.handoff.driver.load(std::sync::atomic::Ordering::Acquire) != ArtifactRunnerDriver::Parked as u8 {
            return None;
        }
        self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactRunnerTerminalJob { handoff: self.handoff.clone(), owner: Some(owner) })
    }

    pub fn close_step(&self) -> bool {
        let active = self.handoff.active_history.load(std::sync::atomic::Ordering::Acquire);
        let driver = self.handoff.driver.load(std::sync::atomic::Ordering::Acquire);
        if !active
            && driver != ArtifactRunnerDriver::Parked as u8
            && driver != ArtifactRunnerDriver::ClosingReady as u8
            && driver != ArtifactRunnerDriver::ClosingPolling as u8
            && driver != ArtifactRunnerDriver::ClosingPollingWake as u8
            && driver != ArtifactRunnerDriver::ClosingParked as u8
            && self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
        {
            return false;
        }
        let close = self.handoff.close_runner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        let closed = close.as_ref().is_none_or(|close| close());
        let mut terminal = self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal.is_none() {
            return active;
        }
        if closed {
            let owner = terminal.take();
            drop(owner);
        }
        true
    }

    /// 📈️ Progress of this authority's faulted close retry, `None` while its close is healthy.
    pub fn close_retry_progress(&self) -> Option<ArtifactCloseRetryProgress> {
        self.handoff.close_retry_progress()
    }

    /// 🔂️ Grants an exhausted or cancelled faulted close a fresh bounded retry budget.
    pub fn readmit_close_retry(&self) -> bool {
        self.handoff.readmit_close_retry()
    }

    /// 🛑️ Stops the automatic retries of a faulted close; the runner stays retained for re-admission.
    pub fn cancel_close_retry(&self) -> bool {
        self.handoff.cancel_close_retry()
    }

    pub fn terminal_is_empty(&self) -> bool {
        let driver = self.handoff.driver.load(std::sync::atomic::Ordering::Acquire);
        driver != ArtifactRunnerDriver::Parked as u8
            && driver != ArtifactRunnerDriver::ClosingReady as u8
            && driver != ArtifactRunnerDriver::ClosingPolling as u8
            && driver != ArtifactRunnerDriver::ClosingPollingWake as u8
            && driver != ArtifactRunnerDriver::ClosingParked as u8
            && self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.handoff.active_history.load(std::sync::atomic::Ordering::Acquire)
    }

    pub async fn submit(&self, batch: CommandBatch, options: SubmitOptions, now_ms: u64) -> Result<CommandReceipt, DbError> {
        self.address.ask(Priority::Command, |reply| ArtifactMessage::Submit { batch, options, now_ms, reply }).await?
    }

    pub async fn query(&self, path: &str) -> Result<Option<db_query::QueryBytes>, DbError> {
        let path = path.to_string();
        self.address.ask(Priority::Query, |reply| ArtifactMessage::Query { path, reply }).await?
    }

    pub async fn frontier(&self) -> Result<Frontier, DbError> {
        self.address.ask(Priority::Query, |reply| ArtifactMessage::Frontier { reply }).await
    }

    /// 📍️ Captures one checkpoint-publication snapshot through the document actor.
    pub async fn checkpoint_publication_snapshot(&self) -> Result<CheckpointPublicationSnapshot, DbError> {
        self.address.ask(Priority::Query, |reply| ArtifactMessage::CheckpointPublicationSnapshot { reply }).await
    }

    pub async fn run_query(&self, query: db_query::Query, consistency: db_query::Consistency) -> Result<db_query::QueryResult, DbError> {
        self.address.ask(Priority::Query, |reply| ArtifactMessage::RunQuery { query, consistency, reply }).await?
    }

    pub async fn snapshot_now(&self, now_ms: u64) -> Result<u64, DbError> {
        self.address.ask(Priority::Command, |reply| ArtifactMessage::SnapshotNow { now_ms, reply }).await?
    }

    pub async fn compact(&self, holder: db_io_core::DbIoText, consolidate_snapshots: bool, budget: db_compact::CompactionBudget, now_ms: u64, cancelled: Arc<std::sync::atomic::AtomicBool>) -> Result<db_compact::CompactionReport, DbError> {
        self.compact_retained(holder, consolidate_snapshots, budget, now_ms, cancelled).await?
    }

    /// 🚪️ Requests closure and advances one finite retained runner turn. The authority remains
    /// caller-owned until the returned terminal acknowledgement is `true`.
    /// 🏁️ Takes the one signal that resolves once this authority's runner is terminal — its WAL
    /// closed and its writer released — so a successor mount of the same document can wait for it.
    pub fn take_terminal_signal(&self) -> Option<db_actor::ReplyReceiver<()>> {
        self.done.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()
    }

    pub fn shutdown_step(&self) -> bool {
        self.address.close();
        (self.cancel)();
        self.close_step();
        self.handoff.terminal.load(std::sync::atomic::Ordering::Acquire)
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn shutdown_debug_witness(&self) -> String {
        let close_error = self.handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        let close_retry = self.handoff.close_retry_progress();
        format!(
            "driver={} terminal={} active_history={} terminal_job={} retirement_maintenance={} runner_pool_use={} close_error={:?} close_retry={:?}",
            self.handoff.driver.load(std::sync::atomic::Ordering::Acquire),
            self.handoff.terminal.load(std::sync::atomic::Ordering::Acquire),
            self.handoff.active_history.load(std::sync::atomic::Ordering::Acquire),
            self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),
            self.handoff.retirement_maintenance.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),
            self.handoff.pool_use.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some(),
            close_error,
            close_retry,
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ArtifactAuthority {
    fn drop(&mut self) {
        ARTIFACT_AUTHORITY_POOL_USES.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        if self.handoff.terminal.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        self.address.close();
        (self.cancel)();
        let Some(retirement) = self.retirement.take() else { return };
        let Some(close) = self.retirement_close.take() else { return };
        retirement.commit(close, self.handoff.clone(), self._pool_use.clone());
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ArtifactRunnerTerminalJob {
    pub fn reason(&self) -> semio_framework_async::WorkerSubmitErrorKind {
        self.owner.as_ref().expect("terminal artifact runner job already resolved").0
    }

    pub fn resume(mut self) -> Result<(), Self> {
        let owner = self.owner.take().expect("terminal artifact runner job already resolved");
        if self.handoff.driver.compare_exchange(ArtifactRunnerDriver::Parked as u8, ArtifactRunnerDriver::Queued as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            if self.handoff.driver.load(std::sync::atomic::Ordering::Acquire) == ArtifactRunnerDriver::Terminal as u8 {
                return Ok(());
            }
            self.owner = Some(owner);
            return Err(self);
        }
        match self.handoff.pool.try_submit(semio_framework_async::Lane::UserVisible, owner.1) {
            Ok(()) => Ok(()),
            Err(error) => {
                self.handoff.driver.store(ArtifactRunnerDriver::Parked as u8, std::sync::atomic::Ordering::Release);
                self.owner = Some((error.kind(), error.into_job()));
                Err(self)
            }
        }
    }

    pub fn close(mut self) -> Result<(), Self> {
        let owner = self.owner.take().expect("terminal artifact runner job already resolved");
        let driver = self.handoff.driver.load(std::sync::atomic::Ordering::Acquire);
        if driver == ArtifactRunnerDriver::Parked as u8 {
            if self.handoff.driver.compare_exchange(ArtifactRunnerDriver::Parked as u8, ArtifactRunnerDriver::ClosingReady as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
                self.owner = Some(owner);
                return Err(self);
            }
        } else if driver == ArtifactRunnerDriver::Terminal as u8 {
            return Ok(());
        } else if driver != ArtifactRunnerDriver::ClosingReady as u8 {
            self.owner = Some(owner);
            return Err(self);
        }
        let close = self.handoff.close_runner.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        let closed = close.as_ref().is_none_or(|close| close());
        if closed {
            self.handoff.driver.store(ArtifactRunnerDriver::Terminal as u8, std::sync::atomic::Ordering::Release);
            Ok(())
        } else {
            self.owner = Some(owner);
            Err(self)
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for ArtifactRunnerTerminalJob {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            if self.handoff.driver.load(std::sync::atomic::Ordering::Acquire) != ArtifactRunnerDriver::Terminal as u8 {
                *self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
            }
        }
    }
}
//#endregion 🔖️Actor
//#endregion 🔖️Ids
