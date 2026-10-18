//! ⚙️ Capability open state operations: the drive, submit, retry and completion operations of the shared open state. See [`super`].

use super::*;

//#region 🔖️CapabilityOpen
impl DatabaseCapabilityOpenState {
    pub fn try_lease(self: &Arc<Self>, expected: u8, finalize: bool) -> Option<DatabaseCapabilityOpenLease> {
        self.lease.compare_exchange(expected, DATABASE_CAPABILITY_OPEN_ACTIVE, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).ok()?;
        Some(DatabaseCapabilityOpenLease { state: self.clone(), held: true, finalize })
    }

    pub fn request_drive(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        let mut observed = self.lease.load(Ordering::Acquire);
        loop {
            if observed & (DATABASE_CAPABILITY_OPEN_CLOSED | DATABASE_CAPABILITY_OPEN_QUEUED) != 0 {
                return;
            }
            match self.lease.compare_exchange_weak(observed, observed | DATABASE_CAPABILITY_OPEN_QUEUED, Ordering::AcqRel, Ordering::Acquire) {
                Ok(_) => {
                    if observed == 0 {
                        self.submit_drive_job();
                    }
                    return;
                }
                Err(current) => observed = current,
            }
        }
    }

    pub fn cancel(self: &Arc<Self>) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.set_progress(DatabaseCapabilityOpenProgress::Cancelled);
        if let Some(_lease) = self.try_lease(0, false) {
            self.stage_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
        } else {
            self.request_drive();
        }
    }

    pub fn phase(&self) -> DatabaseCapabilityOpenPhase {
        DatabaseCapabilityOpenPhase::from_u8(self.phase.load(std::sync::atomic::Ordering::Acquire))
    }

    pub fn set_phase(&self, phase: DatabaseCapabilityOpenPhase) {
        self.phase.store(phase as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn set_progress(&self, progress: DatabaseCapabilityOpenProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn is_current(&self) -> bool {
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseCapabilityOpenAdmission::is_current)
    }

    pub fn wake_waiter(&self) {
        if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            waker.wake();
        }
    }

    pub fn complete(&self, result: Result<DatabaseCapabilityOpenResult, DbError>, progress: DatabaseCapabilityOpenProgress) {
        self.set_progress(progress);
        let mut completion = self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            drop(completion);
            *self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        } else {
            *completion = Some(result);
            drop(completion);
            #[cfg(test)]
            if let Some(hook) = self.controlled_completion_before_wake_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                hook();
            }
            self.wake_waiter();
        }
    }

    pub fn schedule(self: &Arc<Self>) {
        self.request_drive();
    }

    pub fn schedule_cleanup(self: &Arc<Self>) {
        self.request_drive();
    }

    pub fn submit_drive_job(self: &Arc<Self>) {
        let state = self.clone();
        let generation = self.generation;
        let job: semio_framework_async::Job = Box::new(move || state.drive_one(generation));
        #[cfg(test)]
        let submit = self.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        #[cfg(test)]
        let job = if let Some(submit) = submit {
            match submit(job) {
                Ok(()) => return,
                Err(job) => job,
            }
        } else {
            job
        };
        self.submit_exact(job, 0);
    }

    pub fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        #[cfg(test)]
        if let Some(kind) = self.controlled_submit_refusal.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.handle_submit_refusal(kind, job, attempt);
            return;
        }
        if let Err(error) = self.pool.try_submit(Lane::Io, job) {
            self.handle_submit_refusal(error.kind(), error.into_job(), attempt);
        }
    }

    pub fn handle_submit_refusal(self: &Arc<Self>, kind: semio_framework_async::WorkerSubmitErrorKind, job: semio_framework_async::Job, attempt: u8) {
        let mut lease = self.try_lease(DATABASE_CAPABILITY_OPEN_QUEUED, true).expect("exact capability refusal owns the queued turn");
        if self.completion_consumed.load(std::sync::atomic::Ordering::Acquire) && self.roots_are_empty() {
            drop(job);
            lease.retire();
            return;
        }
        match worker_submit_retry_attempt(kind, attempt, DATABASE_CAPABILITY_OPEN_RETRY_LIMIT) {
            Some(next) => {
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, next));
                self.arm_retry();
            }
            None => {
                *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((kind, job));
                self.set_phase(DatabaseCapabilityOpenPhase::RetainWork);
                self.complete(Err(DbError::Unavailable(format!("database capability-open WorkerPool submission failed: {kind:?}"))), DatabaseCapabilityOpenProgress::Fault);
            }
        }
    }

    pub fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        self.advance_retry_generation_once();
    }

    fn advance_retry_generation_once(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        let current = self.retry_generation.load(Ordering::Acquire);
        self.advance_retry_generation_observed_once(current);
    }

    pub(super) fn advance_retry_generation_observed_once(self: &Arc<Self>, current: u64) {
        use std::sync::atomic::Ordering;
        let Some(generation) = current.checked_add(1) else {
            self.retry_armed.store(false, Ordering::Release);
            self.set_progress(DatabaseCapabilityOpenProgress::Fault);
            self.set_phase(DatabaseCapabilityOpenPhase::RetainWork);
            self.complete(Err(DbError::LimitExceeded("database capability-open retry generation")), DatabaseCapabilityOpenProgress::Fault);
            return;
        };
        if let Err(observed) = self.retry_generation.compare_exchange(current, generation, Ordering::AcqRel, Ordering::Acquire) {
            let state = self.clone();
            self.pool.callback_at(self.pool.now_ms().saturating_add(DATABASE_CAPABILITY_OPEN_RETRY_MS), move || {
                if state.retry_armed.load(Ordering::Acquire) && observed == state.retry_generation.load(Ordering::Acquire) {
                    state.advance_retry_generation_observed_once(observed);
                }
            });
            return;
        }
        self.retry_submission_at(generation);
    }

    pub fn retry_submission_at(self: &Arc<Self>, generation: u64) {
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(DATABASE_CAPABILITY_OPEN_RETRY_MS), move || state.retry_submission_once(generation));
    }

    pub fn retry_submission_once(self: &Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        if generation != self.retry_generation.load(Ordering::Acquire) || !self.retry_armed.load(Ordering::Acquire) {
            return;
        }
        let Some(lease) = self.try_lease(0, true) else {
            if !self.finished.load(Ordering::Acquire) {
                self.retry_submission_at(generation);
            }
            return;
        };
        if generation != self.retry_generation.load(Ordering::Acquire) || !self.retry_armed.load(Ordering::Acquire) {
            return;
        }
        self.retry_armed.store(false, Ordering::Release);
        let retry = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some((job, attempt)) = retry {
            if self.cancelled.load(Ordering::Acquire) || !self.is_current() {
                *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, job));
                self.stage_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
            } else {
                lease.submit_exact(job, attempt);
            }
        }
    }

    pub fn drive_one(self: Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        if generation != self.generation {
            return;
        }
        let Some(_lease) = self.try_lease(DATABASE_CAPABILITY_OPEN_QUEUED, true) else { return };
        if !self.is_current() && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.phase() != DatabaseCapabilityOpenPhase::Terminal {
            self.stage_terminal(DbError::Unavailable("database capability-open generation became stale".to_string()), DatabaseCapabilityOpenProgress::Fault);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.phase() != DatabaseCapabilityOpenPhase::Terminal {
            self.stage_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
            return;
        }
        match self.phase() {
            DatabaseCapabilityOpenPhase::Handoff => {
                let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else {
                    self.stage_terminal(DbError::Unavailable("database capability-open handoff owner missing".to_string()), DatabaseCapabilityOpenProgress::Fault);
                    return;
                };
                #[cfg(test)]
                if let Some(hook) = self.controlled_owner_transfer_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                    hook(work.storage_identity);
                }
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.set_phase(DatabaseCapabilityOpenPhase::Poll);
                self.schedule();
            }
            DatabaseCapabilityOpenPhase::Poll => self.poll_backend_once(generation),
            DatabaseCapabilityOpenPhase::RetainWork => {
                let work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take());
                if let Some(work) = work {
                    #[cfg(test)]
                    if let Some(hook) = self.controlled_owner_transfer_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        hook(work.storage_identity);
                    }
                    *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                }
                self.set_phase(DatabaseCapabilityOpenPhase::DrainWork);
                self.schedule_cleanup();
            }
            DatabaseCapabilityOpenPhase::DrainWork => {
                let mut terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let progressed = terminal.as_mut().is_some_and(DatabaseCapabilityOpenWork::close_step);
                let empty = terminal.as_ref().is_some_and(DatabaseCapabilityOpenWork::terminal_is_empty);
                drop(terminal);
                if progressed || empty {
                    self.set_phase(DatabaseCapabilityOpenPhase::ReleaseWork);
                    self.schedule_cleanup();
                } else {
                    self.stage_terminal(DbError::Unavailable("database capability-open retained work failed to release".to_string()), DatabaseCapabilityOpenProgress::Fault);
                }
            }
            DatabaseCapabilityOpenPhase::ReleaseWork => {
                let terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                if terminal.as_ref().is_some_and(DatabaseCapabilityOpenWork::terminal_is_empty) {
                    self.set_phase(if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() && self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
                        DatabaseCapabilityOpenPhase::RetainResult
                    } else {
                        DatabaseCapabilityOpenPhase::Publish
                    });
                    self.schedule_cleanup();
                } else {
                    *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = terminal;
                    self.stage_terminal(DbError::Unavailable("database capability-open terminal work witness failed".to_string()), DatabaseCapabilityOpenProgress::Fault);
                }
            }
            DatabaseCapabilityOpenPhase::RetainResult => {
                if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                    *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(result));
                }
                self.set_phase(DatabaseCapabilityOpenPhase::Publish);
                self.schedule_cleanup();
            }
            DatabaseCapabilityOpenPhase::Publish => self.publish_staged(),
            DatabaseCapabilityOpenPhase::Terminal => {}
        }
    }

    fn publish_poll_terminal(&self, error: DbError, progress: DatabaseCapabilityOpenProgress) {
        use std::sync::atomic::Ordering;
        self.cancelled.store(true, Ordering::Release);
        self.set_progress(progress);
        let mut terminal_error = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal_error.is_none() {
            *terminal_error = Some((error, progress));
        }
        drop(terminal_error);
        self.set_phase(DatabaseCapabilityOpenPhase::RetainWork);
    }

    fn poll_terminal_if_cancelled_or_stale(&self) -> bool {
        if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
            self.publish_poll_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
            true
        } else if !self.is_current() {
            self.publish_poll_terminal(DbError::Unavailable("database capability-open generation became stale during polling".to_string()), DatabaseCapabilityOpenProgress::Fault);
            true
        } else {
            false
        }
    }

    pub fn release_terminal_poll(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        self.polling.store(false, Ordering::Release);
        self.wake_requested.store(false, Ordering::Release);
        self.schedule_cleanup();
    }

    pub fn poll_backend_once(self: &Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        let Some(mut work) = self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else {
            self.stage_terminal(DbError::Unavailable("database capability-open poll owner missing".to_string()), DatabaseCapabilityOpenProgress::Fault);
            return;
        };
        #[cfg(test)]
        if let Some(hook) = self.controlled_owner_transfer_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            hook(work.storage_identity);
        }
        self.polling.store(true, Ordering::Release);
        self.set_progress(DatabaseCapabilityOpenProgress::Polling);
        let wake = std::task::Waker::from(Arc::new(DatabaseCapabilityOpenWake { state: Arc::downgrade(self), generation }));
        let mut context = std::task::Context::from_waker(&wake);
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| work.poll(&mut context)));
        #[cfg(test)]
        if let Some(hook) = self.poll_publication_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            hook();
        }
        match polled {
            Ok(std::task::Poll::Pending) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                if self.poll_terminal_if_cancelled_or_stale() {
                    self.release_terminal_poll();
                } else {
                    self.set_progress(DatabaseCapabilityOpenProgress::Pending);
                    self.polling.store(false, Ordering::Release);
                    if self.wake_requested.swap(false, Ordering::AcqRel) {
                        self.schedule();
                    }
                }
            }
            Ok(std::task::Poll::Ready(output)) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                *self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(output);
                self.set_phase(DatabaseCapabilityOpenPhase::RetainWork);
                if self.poll_terminal_if_cancelled_or_stale() {
                    self.release_terminal_poll();
                } else {
                    self.polling.store(false, Ordering::Release);
                    self.wake_requested.store(false, Ordering::Release);
                    self.schedule_cleanup();
                }
            }
            Err(_) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.publish_poll_terminal(DbError::Unavailable("database capability-open backend poll panicked".to_string()), DatabaseCapabilityOpenProgress::Fault);
                self.release_terminal_poll();
            }
        }
    }

    pub fn stage_terminal(self: &Arc<Self>, error: DbError, progress: DatabaseCapabilityOpenProgress) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.set_progress(progress);
        {
            let mut terminal_error = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if terminal_error.is_none() {
                *terminal_error = Some((error, progress));
            }
        }
        let has_work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() || self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        let has_terminal_work = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        let phase = if has_work {
            DatabaseCapabilityOpenPhase::RetainWork
        } else if has_terminal_work {
            match self.phase() {
                DatabaseCapabilityOpenPhase::ReleaseWork => DatabaseCapabilityOpenPhase::ReleaseWork,
                _ => DatabaseCapabilityOpenPhase::DrainWork,
            }
        } else if self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            DatabaseCapabilityOpenPhase::RetainResult
        } else {
            DatabaseCapabilityOpenPhase::Publish
        };
        self.set_phase(phase);
        if self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.schedule_cleanup();
        } else {
            self.complete(Err(DbError::Closed), progress);
        }
    }

    pub fn publish_staged(self: &Arc<Self>) {
        if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                self.stage_terminal(DbError::Closed, DatabaseCapabilityOpenProgress::Cancelled);
                return;
            }
            if !self.is_current() {
                self.stage_terminal(DbError::Unavailable("database capability-open generation became stale before publication".to_string()), DatabaseCapabilityOpenProgress::Fault);
                return;
            }
        }
        if let Some((error, progress)) = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.complete(Err(error), progress);
        } else if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.complete(Ok(result), DatabaseCapabilityOpenProgress::Completed);
        } else {
            self.complete(Err(DbError::Unavailable("database capability-open publication owner missing".to_string())), DatabaseCapabilityOpenProgress::Fault);
        }
        self.set_phase(DatabaseCapabilityOpenPhase::Terminal);
    }

    pub fn roots_are_empty(&self) -> bool {
        self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.terminal_result_checked_out.load(std::sync::atomic::Ordering::Acquire)
            && self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }

    pub fn close_step(self: &Arc<Self>) -> DatabaseCapabilityOpenCloseStep {
        use std::sync::atomic::Ordering;
        self.cancelled.store(true, Ordering::Release);
        let Some(mut lease) = self.try_lease(0, false) else {
            return if self.terminal_is_empty() { DatabaseCapabilityOpenCloseStep::Complete } else { DatabaseCapabilityOpenCloseStep::Blocked };
        };
        if let Some((_, job)) = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if let Some((job, _)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if self.retry_armed.load(Ordering::Acquire) {
            let current = self.retry_generation.load(Ordering::Acquire);
            if let Some(next) = current.checked_add(1) {
                self.retry_generation.store(next, Ordering::Release);
                self.retry_armed.store(false, Ordering::Release);
                return DatabaseCapabilityOpenCloseStep::Progress;
            }
            return DatabaseCapabilityOpenCloseStep::Blocked;
        }
        if self.polling.load(Ordering::Acquire) {
            return DatabaseCapabilityOpenCloseStep::Blocked;
        }
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if let Some(work) = self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        {
            let mut terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(work) = terminal.as_mut() {
                if work.close_step() {
                    return DatabaseCapabilityOpenCloseStep::Progress;
                }
                if work.terminal_is_empty() {
                    terminal.take();
                    return DatabaseCapabilityOpenCloseStep::Progress;
                }
            }
        }
        if self.terminal_result_checked_out.load(Ordering::Acquire) {
            return DatabaseCapabilityOpenCloseStep::Blocked;
        }
        if let Some(result) = self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if let Some(result) = self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(result));
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if let Some(result) = self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            lease.retire();
            return DatabaseCapabilityOpenCloseStep::Progress;
        }
        if self.roots_are_empty() {
            lease.retire();
            DatabaseCapabilityOpenCloseStep::Complete
        } else {
            DatabaseCapabilityOpenCloseStep::Blocked
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.finished.load(std::sync::atomic::Ordering::Acquire) && self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.roots_are_empty()
    }

    pub fn release_success(self: &Arc<Self>) {
        let Some(mut lease) = self.try_lease(0, false) else {
            self.request_drive();
            return;
        };
        if self.completion_consumed.load(std::sync::atomic::Ordering::Acquire) && self.roots_are_empty() {
            lease.retire();
        }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn retained_owner_count(&self) -> usize {
        usize::from(self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
    }
}
//#endregion 🔖️CapabilityOpen
