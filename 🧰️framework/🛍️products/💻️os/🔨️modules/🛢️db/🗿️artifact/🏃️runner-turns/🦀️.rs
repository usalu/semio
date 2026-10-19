//! 🏃️ Runner turns: the `ArtifactRunner` impl. See [`super`].

use super::*;

//#region 🔖️Ids
//#region 🔖️Actor
#[cfg(not(target_arch = "wasm32"))]
impl ArtifactRunner {
    pub(super) fn close_one(self: &Arc<Self>) -> bool {
        use std::sync::atomic::Ordering;
        self.cancelled.store(true, Ordering::Release);
        if !self.handoff.close_poll_admitted() {
            return self.terminal.load(Ordering::Acquire);
        }
        self.close_driving.store(true, Ordering::Release);
        loop {
            let driver = self.handoff.driver.load(Ordering::Acquire);
            if driver == ArtifactRunnerDriver::Terminal as u8
                || driver == ArtifactRunnerDriver::Polling as u8
                || driver == ArtifactRunnerDriver::PollingWake as u8
                || driver == ArtifactRunnerDriver::ClosingReady as u8
                || driver == ArtifactRunnerDriver::ClosingPolling as u8
                || driver == ArtifactRunnerDriver::ClosingPollingWake as u8
            {
                break;
            }
            if self.handoff.driver.compare_exchange(driver, ArtifactRunnerDriver::ClosingReady as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                break;
            }
        }
        let generation = self.generation;
        let witness = self.clone();
        self.clone().run_turn(generation, true);
        let closed = witness.terminal.load(Ordering::Acquire);
        witness.close_driving.store(false, Ordering::Release);
        closed
    }

    fn retained_terminal_job(self: &Arc<Self>, kind: semio_framework_async::WorkerSubmitErrorKind) -> (semio_framework_async::WorkerSubmitErrorKind, semio_framework_async::Job) {
        let runner = self.clone();
        let generation = self.generation;
        (kind, Box::new(move || runner.run_turn(generation, false)))
    }

    fn park_terminal_job(self: &Arc<Self>, kind: semio_framework_async::WorkerSubmitErrorKind) {
        use std::sync::atomic::Ordering;
        let mut terminal = self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal.is_some() {
            return;
        }
        loop {
            let driver = self.handoff.driver.load(Ordering::Acquire);
            if driver != ArtifactRunnerDriver::RunnableIdle as u8 && driver != ArtifactRunnerDriver::Queued as u8 {
                return;
            }
            if self.handoff.driver.compare_exchange(driver, ArtifactRunnerDriver::Parked as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                *terminal = Some(self.retained_terminal_job(kind));
                return;
            }
        }
    }

    pub(super) fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        loop {
            let driver = self.handoff.driver.load(Ordering::Acquire);
            if driver == ArtifactRunnerDriver::RunnableIdle as u8 {
                if self.handoff.driver.compare_exchange(driver, ArtifactRunnerDriver::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                    self.submit_scheduled();
                    return;
                }
            } else if driver == ArtifactRunnerDriver::Polling as u8 {
                if self.handoff.driver.compare_exchange(driver, ArtifactRunnerDriver::PollingWake as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                    return;
                }
            } else if driver == ArtifactRunnerDriver::ClosingPolling as u8 {
                if self.handoff.driver.compare_exchange(driver, ArtifactRunnerDriver::ClosingPollingWake as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                    return;
                }
            } else if driver == ArtifactRunnerDriver::ClosingParked as u8 {
                if self.handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
                    return;
                }
                if self.handoff.driver.compare_exchange(driver, ArtifactRunnerDriver::ClosingReady as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                    self.handoff.request_retirement_maintenance();
                    return;
                }
            } else {
                return;
            }
        }
    }

    pub(super) fn submit_scheduled(self: &Arc<Self>) {
        let runner = Arc::downgrade(self);
        let generation = self.generation;
        self.submit_exact(
            Box::new(move || {
                if let Some(runner) = runner.upgrade() {
                    if runner.generation == generation {
                        runner.run_turn(generation, false);
                    }
                }
            }),
            0,
        );
    }

    fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(semio_framework_async::Lane::UserVisible, job) {
            Ok(()) => {}
            Err(error) => match worker_submit_retry_attempt(error.kind(), attempt, ARTIFACT_RUNNER_RETRY_LIMIT) {
                Some(next) => {
                    *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), next));
                    self.arm_retry();
                }
                None => {
                    let kind = error.kind();
                    let job = error.into_job();
                    if let Some(ready) = self.ready.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        drop(job);
                        ready.send(Err(ArtifactEngineOpenRejected::BeforeWal(DbError::Unavailable(format!("artifact authority WorkerPool submission failed: {kind:?}")))));
                        self.finish();
                    } else {
                        drop(job);
                        self.park_terminal_job(kind);
                    }
                }
            },
        }
    }

    fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let generation = match self.retry_generation.try_update(Ordering::AcqRel, Ordering::Acquire, |generation| generation.checked_add(1).filter(|next| *next != 0)) {
            Ok(previous) => match previous.checked_add(1) {
                Some(generation) => generation,
                None => {
                    self.terminalize_retry_authority("artifact runner retry generation exhausted");
                    return;
                }
            },
            Err(_) => {
                self.terminalize_retry_authority("artifact runner retry generation exhausted");
                return;
            }
        };
        let Some(deadline) = self.pool.now_ms().checked_add(ARTIFACT_RUNNER_RETRY_MS) else {
            self.terminalize_retry_authority("artifact runner retry deadline exhausted");
            return;
        };
        let runner = self.clone();
        self.pool.callback_at(deadline, move || {
            if generation != runner.retry_generation.load(Ordering::Acquire) {
                return;
            }
            runner.retry_armed.store(false, Ordering::Release);
            let retry = runner.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some((job, attempt)) = retry {
                if runner.cancelled.load(Ordering::Acquire) {
                    drop(job);
                    runner.park_terminal_job(semio_framework_async::WorkerSubmitErrorKind::Saturated);
                } else {
                    runner.submit_exact(job, attempt);
                }
            }
        });
    }

    fn terminalize_retry_authority(self: &Arc<Self>, detail: &'static str) {
        self.retry_armed.store(false, std::sync::atomic::Ordering::Release);
        let retained = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some((job, attempt)) = retained {
            if self.handoff.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
                drop(job);
                self.park_terminal_job(semio_framework_async::WorkerSubmitErrorKind::Saturated);
            } else {
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            }
        }
        if let Some(ready) = self.ready.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            ready.send(Err(ArtifactEngineOpenRejected::BeforeWal(DbError::Unavailable(detail.to_string()))));
            self.finish();
        }
    }

    pub(super) fn cancel(self: &Arc<Self>) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.schedule();
    }

    pub fn finish(self: &Arc<Self>) {
        if self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|turn| matches!(turn, ArtifactTurn::History { replay, .. } if !replay.terminal_is_empty())) {
            return;
        }
        {
            let mut engine = self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if engine.as_ref().is_some_and(|owner| owner.close_flush_pending()) {
                let mut turn = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                if turn.is_none() {
                    let mut owner = engine.take().expect("checked artifact engine close-flush owner");
                    drop(engine);
                    *turn = Some(ArtifactTurn::CloseFlush(Box::pin(async move {
                        let _ = owner.close_flush().await;
                        owner
                    })));
                    drop(turn);
                    self.schedule();
                    return;
                }
            }
            if let Some(owner) = engine.as_mut() {
                #[cfg(test)]
                {
                    self.handoff.close_polls.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                    if self.handoff.close_faults.try_update(std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire, |faults| faults.checked_sub(1)).is_ok() {
                        drop(engine);
                        self.record_close_fault(DbError::Io("injected artifact engine close fault".to_string()));
                        return;
                    }
                }
                let waker = std::task::Waker::from(Arc::new(ArtifactRunnerWake { runner: Arc::downgrade(self), generation: self.generation }));
                match owner.poll_close(&mut std::task::Context::from_waker(&waker)) {
                    std::task::Poll::Pending => {
                        drop(engine);
                        self.clear_close_fault();
                        return;
                    }
                    std::task::Poll::Ready(Ok(())) => {
                        engine.take();
                        self.clear_close_fault();
                    }
                    std::task::Poll::Ready(Err(error)) => {
                        drop(engine);
                        self.record_close_fault(error);
                        return;
                    }
                }
            }
        }
        if !self.terminal.swap(true, std::sync::atomic::Ordering::AcqRel) {
            let builder = self.builder.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if builder.is_none() {
                let turn = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                drop(turn);
            }
            self.handoff.enter_terminal();
            if let Some(done) = self.done.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                done.send(());
            }
        }
    }

    fn record_close_fault(self: &Arc<Self>, error: DbError) {
        *self.handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(error);
        self.handoff.arm_close_retry();
    }

    fn clear_close_fault(&self) {
        if self.handoff.close_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            self.handoff.clear_close_retry();
        }
    }

    fn start_turn(engine: Box<ArtifactEngine>, message: ArtifactMessage) -> ArtifactTurn {
        match message {
            ArtifactMessage::History { operation_generation, cancelled, reservation, reply } => {
                let replay = engine.history_replay(operation_generation, cancelled, reservation);
                ArtifactTurn::History { engine: Some(engine), replay, reply: Some(reply) }
            }
            ArtifactMessage::AppendDurableGroupDecision { record, cancelled, now_ms, reply } => ArtifactTurn::Future(Box::pin(async move {
                let mut engine = engine;
                reply.send(engine.append_durable_group_decision(record, cancelled, now_ms).await);
                engine
            })),
            ArtifactMessage::RecoverDurableGroupDecisions { driver, reply } => ArtifactTurn::Future(Box::pin(async move {
                let mut engine = engine;
                reply.send(engine.recover_durable_group_decisions(driver).await);
                engine
            })),
            ArtifactMessage::Submit { batch, options, now_ms, reply } => ArtifactTurn::Future(Box::pin(async move {
                let mut engine = engine;
                reply.send(engine.submit(batch, options, now_ms).await);
                engine.park_writer();
                let _ = engine.maintain_index().await;
                engine
            })),
            ArtifactMessage::Query { path, reply } => ArtifactTurn::Future(Box::pin(async move {
                reply.send(engine.get(&path).await);
                engine
            })),
            ArtifactMessage::Frontier { reply } => ArtifactTurn::Future(Box::pin(async move {
                reply.send(engine.frontier().await);
                engine
            })),
            ArtifactMessage::CheckpointPublicationSnapshot { reply } => ArtifactTurn::Future(Box::pin(async move {
                reply.send(engine.checkpoint_publication_snapshot().await);
                engine
            })),
            ArtifactMessage::RunQuery { query, consistency, reply } => ArtifactTurn::Future(Box::pin(async move {
                reply.send(engine.query(query, consistency).await);
                engine
            })),
            ArtifactMessage::SnapshotNow { now_ms, reply } => ArtifactTurn::Future(Box::pin(async move {
                reply.send(engine.snapshot_now(now_ms).await);
                engine
            })),
            ArtifactMessage::Compact { holder, consolidate_snapshots, budget, now_ms, cancelled, reply } => ArtifactTurn::Future(Box::pin(async move {
                let mut engine = engine;
                reply.send(engine.compact_retained(holder, consolidate_snapshots, budget, now_ms, cancelled).await);
                engine
            })),
        }
    }

    /// 🔁️ Keeps polling in this worker turn when the future woke itself while it was being
    /// polled (a cooperative yield, a storage step that completed inline) and the turn budget
    /// remains: every such wake used to cost a full worker-pool round trip.
    fn repoll_within_turn(&self, turn_ends: std::time::Instant) -> bool {
        use std::sync::atomic::Ordering;
        std::time::Instant::now() < turn_ends
            && !self.cancelled.load(Ordering::Acquire)
            && self.handoff.driver.compare_exchange(ArtifactRunnerDriver::PollingWake as u8, ArtifactRunnerDriver::Polling as u8, Ordering::AcqRel, Ordering::Acquire).is_ok()
    }

    fn run_turn(self: Arc<Self>, generation: u64, direct_close: bool) {
        use std::panic::AssertUnwindSafe;
        use std::sync::atomic::Ordering;

        if generation != self.generation || self.terminal.load(Ordering::Acquire) {
            return;
        }
        let (_close_poll, _poll) = if direct_close {
            if self.handoff.driver.compare_exchange(ArtifactRunnerDriver::ClosingReady as u8, ArtifactRunnerDriver::ClosingPolling as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
                return;
            }
            (Some(ArtifactRunnerClosePoll { handoff: self.handoff.clone() }), None)
        } else if self.handoff.driver.compare_exchange(ArtifactRunnerDriver::Queued as u8, ArtifactRunnerDriver::Polling as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            (None, Some(ArtifactRunnerPoll { runner: self.clone() }))
        } else {
            return;
        };
        if self.cancelled.load(Ordering::Acquire) {
            let mut turn = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            match turn.as_mut() {
                Some(ArtifactTurn::History { replay, .. }) => replay.request_close(DbError::Closed),
                Some(ArtifactTurn::CloseFlush(_)) => {}
                _ => {
                    drop(turn);
                    self.finish();
                    return;
                }
            }
        }

        let waker = std::task::Waker::from(Arc::new(ArtifactRunnerWake { runner: Arc::downgrade(&self), generation }));
        let mut context = std::task::Context::from_waker(&waker);
        let turn_ends = std::time::Instant::now() + std::time::Duration::from_micros(ARTIFACT_RUNNER_TURN_MICROS);

        let mut builder = self.builder.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(future) = builder.as_mut() {
            let polled = loop {
                let polled = std::panic::catch_unwind(AssertUnwindSafe(|| future.as_mut().poll(&mut context)));
                if !matches!(polled, Ok(std::task::Poll::Pending)) || !self.repoll_within_turn(turn_ends) {
                    break polled;
                }
            };
            match polled {
                Ok(std::task::Poll::Pending) => return,
                Ok(std::task::Poll::Ready(Ok(engine))) => {
                    builder.take();
                    drop(builder);
                    engine.park_writer();
                    *self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(engine);
                    if let Some(ready) = self.ready.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        ready.send(Ok(()));
                    }
                    if self.address.has_messages() {
                        self.schedule();
                    }
                    return;
                }
                Ok(std::task::Poll::Ready(Err(error))) => {
                    builder.take();
                    drop(builder);
                    if let Some(ready) = self.ready.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        ready.send(Err(error));
                    }
                    self.finish();
                    return;
                }
                Err(_) => {
                    builder.take();
                    drop(builder);
                    if let Some(ready) = self.ready.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        ready.send(Err(ArtifactEngineOpenRejected::BeforeWal(DbError::Internal("document authority construction panicked".to_string()))));
                    }
                    self.finish();
                    return;
                }
            }
        }
        drop(builder);

        let mut turn = self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(active) = turn.as_mut() {
            match active {
                ArtifactTurn::Future(future) => match loop {
                    let polled = std::panic::catch_unwind(AssertUnwindSafe(|| future.as_mut().poll(&mut context)));
                    if !matches!(polled, Ok(std::task::Poll::Pending)) || !self.repoll_within_turn(turn_ends) {
                        break polled;
                    }
                } {
                    Ok(std::task::Poll::Pending) => return,
                    Ok(std::task::Poll::Ready(engine)) => {
                        turn.take();
                        drop(turn);
                        engine.park_writer();
                        *self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(engine);
                        if self.cancelled.load(Ordering::Acquire) || self.address.is_idle_and_closed() {
                            self.finish();
                        } else if self.address.has_messages() {
                            self.schedule();
                        }
                        return;
                    }
                    Err(_) => {
                        turn.take();
                        drop(turn);
                        self.address.close();
                        self.finish();
                        return;
                    }
                },
                ArtifactTurn::CloseFlush(future) => match std::panic::catch_unwind(AssertUnwindSafe(|| future.as_mut().poll(&mut context))) {
                    Ok(std::task::Poll::Pending) => return,
                    Ok(std::task::Poll::Ready(engine)) => {
                        turn.take();
                        drop(turn);
                        *self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(engine);
                        self.finish();
                        return;
                    }
                    Err(_) => {
                        turn.take();
                        drop(turn);
                        self.address.close();
                        self.finish();
                        return;
                    }
                },
                ArtifactTurn::History { engine, replay, reply } => {
                    if self.cancelled.load(Ordering::Acquire) || self.address.is_idle_and_closed() {
                        replay.request_close(DbError::Closed);
                    }
                    match loop {
                        let polled = std::panic::catch_unwind(AssertUnwindSafe(|| Pin::new(&mut *replay).poll(&mut context)));
                        if !matches!(polled, Ok(std::task::Poll::Pending)) || !self.repoll_within_turn(turn_ends) {
                            break polled;
                        }
                    } {
                        Ok(std::task::Poll::Pending) => return,
                        Ok(std::task::Poll::Ready(result)) => {
                            let engine = engine.take();
                            if let Some(reply) = reply.take() {
                                reply.send(result);
                            }
                            let replay_empty = replay.terminal_is_empty();
                            if replay_empty {
                                self.handoff.active_history.store(false, Ordering::Release);
                                turn.take();
                            }
                            drop(turn);
                            if let Some(engine) = engine {
                                *self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(engine);
                            }
                            if !replay_empty {
                                self.schedule();
                            } else if self.cancelled.load(Ordering::Acquire) || self.address.is_idle_and_closed() {
                                self.finish();
                            } else if self.address.has_messages() {
                                self.schedule();
                            }
                            return;
                        }
                        Err(_) => {
                            replay.request_close(DbError::Internal("history replay cursor panicked".to_string()));
                            drop(turn);
                            self.address.close();
                            self.schedule();
                            return;
                        }
                    }
                }
            }
        }
        drop(turn);

        if let Some(envelope) = self.receiver.try_recv() {
            let engine = self.engine.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some(engine) = engine {
                let turn = Self::start_turn(engine, envelope.payload);
                self.handoff.active_history.store(matches!(&turn, ArtifactTurn::History { .. }), Ordering::Release);
                *self.turn.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(turn);
                self.schedule();
            } else {
                self.address.close();
                self.finish();
            }
            return;
        }
        if self.cancelled.load(Ordering::Acquire) || self.address.is_idle_and_closed() {
            self.finish();
        }
    }
}
//#endregion 🔖️Actor
//#endregion 🔖️Ids
