//! ⚙️ Catalog read state operations: the drive, submit, retry and completion operations of the shared read state. See [`super`].

use super::*;

//#region 🔖️CatalogRootRead
impl DatabaseCatalogReadState {
    pub fn try_lease(self: &Arc<Self>, expected: u8, finalize: bool) -> Option<DatabaseCatalogReadLease> {
        self.lease.compare_exchange(expected, DATABASE_CATALOG_READ_ACTIVE, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).ok()?;
        Some(DatabaseCatalogReadLease { state: self.clone(), held: true, finalize })
    }

    pub(super) fn has_retained_submission(&self) -> bool {
        let retry = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        retry || self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
    }

    pub fn request_drive(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        let mut observed = self.lease.load(Ordering::Acquire);
        loop {
            if observed & (DATABASE_CATALOG_READ_CLOSED | DATABASE_CATALOG_READ_QUEUED) != 0 || self.has_retained_submission() {
                return;
            }
            match self.lease.compare_exchange_weak(observed, observed | DATABASE_CATALOG_READ_QUEUED, Ordering::AcqRel, Ordering::Acquire) {
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
        self.set_progress(DatabaseCatalogReadProgress::Cancelled);
        if let Some(_lease) = self.try_lease(0, false) {
            if self.has_retained_submission() {
                return;
            }
            if self.phase() != DatabaseCatalogReadPhase::Terminal && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
                self.stage_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
            } else {
                self.request_drive();
            }
        } else {
            self.request_drive();
        }
    }

    pub fn phase(&self) -> DatabaseCatalogReadPhase {
        DatabaseCatalogReadPhase::from_u8(self.phase.load(std::sync::atomic::Ordering::Acquire))
    }

    pub fn set_phase(&self, phase: DatabaseCatalogReadPhase) {
        self.phase.store(phase as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn set_progress(&self, progress: DatabaseCatalogReadProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn is_current(&self) -> bool {
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseCatalogReadAdmission::is_current)
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
        self.submit_exact(job, 0);
    }

    pub fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
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
        #[cfg(test)]
        let refusal = self.controlled_submit_refusal.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        #[cfg(test)]
        if let Some(kind) = refusal {
            self.handle_submit_refusal(kind, job, attempt);
            return;
        }
        if let Err(error) = self.pool.try_submit(Lane::Io, job) {
            self.handle_submit_refusal(error.kind(), error.into_job(), attempt);
        }
    }

    pub fn handle_submit_refusal(self: &Arc<Self>, kind: semio_framework_async::WorkerSubmitErrorKind, job: semio_framework_async::Job, attempt: u8) {
        let mut lease = self.try_lease(DATABASE_CATALOG_READ_QUEUED, true).expect("exact catalog refusal owns the queued turn");
        if self.completion_consumed.load(std::sync::atomic::Ordering::Acquire) && self.roots_are_empty() {
            drop(job);
            lease.retire();
            return;
        }
        if let Some(next) = worker_submit_retry_attempt(kind, attempt, DATABASE_CATALOG_READ_RETRY_LIMIT) {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, next));
            self.arm_retry();
        } else {
            *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((kind, job));
            self.set_phase(DatabaseCatalogReadPhase::RetainWork);
            self.set_progress(DatabaseCatalogReadProgress::Fault);
            self.publish_public_completion(Err(DbError::Unavailable(format!("database catalog-read WorkerPool submission failed: {kind:?}"))));
        }
    }

    pub fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.swap(true, Ordering::AcqRel) {
            return;
        }
        let Some(generation) = self.retry_generation.load(Ordering::Acquire).checked_add(1) else {
            self.retry_armed.store(false, Ordering::Release);
            self.set_progress(DatabaseCatalogReadProgress::Fault);
            self.publish_public_completion(Err(DbError::LimitExceeded("database catalog-read retry generation")));
            return;
        };
        self.retry_generation.store(generation, Ordering::Release);
        self.retry_submission_at(generation);
    }

    pub fn retry_submission_at(self: &Arc<Self>, generation: u64) {
        let state = self.clone();
        self.schedule_retry_callback(Box::new(move || state.retry_submission_once(generation)));
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
            let stale = !self.is_current();
            if self.cancelled.load(Ordering::Acquire) || stale {
                *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, job));
                self.set_phase(DatabaseCatalogReadPhase::RetainWork);
                self.set_progress(if stale { DatabaseCatalogReadProgress::Fault } else { DatabaseCatalogReadProgress::Cancelled });
                self.publish_public_completion(Err(if stale { DbError::Unavailable("database catalog-read retry generation stale".into()) } else { DbError::Closed }));
            } else {
                lease.submit_exact(job, attempt);
            }
        }
    }

    fn schedule_retry_callback(self: &Arc<Self>, job: semio_framework_async::Job) {
        #[cfg(test)]
        let callback = self.controlled_retry_callback_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone();
        #[cfg(test)]
        let job = if let Some(callback) = callback {
            match callback(job) {
                Ok(()) => return,
                Err(job) => job,
            }
        } else {
            job
        };
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), job);
    }

    pub fn drive_one(self: Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        if generation != self.generation {
            return;
        }
        let Some(_lease) = self.try_lease(DATABASE_CATALOG_READ_QUEUED, true) else { return };
        if !self.is_current() && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.phase() != DatabaseCatalogReadPhase::Terminal {
            self.stage_terminal(DbError::Unavailable("database catalog-read generation stale".to_string()), DatabaseCatalogReadProgress::Fault);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() && self.phase() != DatabaseCatalogReadPhase::Terminal {
            self.stage_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
            return;
        }
        match self.phase() {
            DatabaseCatalogReadPhase::Handoff => {
                let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else {
                    self.stage_terminal(DbError::Unavailable("database catalog-read handoff owner missing".to_string()), DatabaseCatalogReadProgress::Fault);
                    return;
                };
                #[cfg(test)]
                if let Some(hook) = self.controlled_owner_transfer_hook.lock().unwrap().take() {
                    hook(work.storage_identity);
                }
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.set_phase(DatabaseCatalogReadPhase::Poll);
                self.schedule();
            }
            DatabaseCatalogReadPhase::Poll => self.poll_backend_once(generation),
            DatabaseCatalogReadPhase::RetainWork => {
                let work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take());
                if let Some(work) = work {
                    #[cfg(test)]
                    if let Some(hook) = self.controlled_owner_transfer_hook.lock().unwrap().take() {
                        hook(work.storage_identity);
                    }
                    *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                }
                self.set_phase(DatabaseCatalogReadPhase::DrainWork);
                self.schedule_cleanup();
            }
            DatabaseCatalogReadPhase::DrainWork => {
                let mut terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let progressed = terminal.as_mut().is_some_and(DatabaseCatalogReadWork::close_step);
                let empty = terminal.as_ref().is_some_and(DatabaseCatalogReadWork::terminal_is_empty);
                drop(terminal);
                if progressed || empty {
                    self.set_phase(DatabaseCatalogReadPhase::ReleaseWork);
                    self.schedule_cleanup();
                } else {
                    self.stage_terminal(DbError::Unavailable("database catalog-read work failed to release".to_string()), DatabaseCatalogReadProgress::Fault);
                }
            }
            DatabaseCatalogReadPhase::ReleaseWork => {
                let terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                if terminal.as_ref().is_some_and(DatabaseCatalogReadWork::terminal_is_empty) {
                    let has_result = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
                    self.set_phase(if has_result { DatabaseCatalogReadPhase::RetainResult } else { DatabaseCatalogReadPhase::Publish });
                    self.schedule_cleanup();
                } else {
                    *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = terminal;
                    self.stage_terminal(DbError::Unavailable("database catalog-read work witness failed".to_string()), DatabaseCatalogReadProgress::Fault);
                }
            }
            DatabaseCatalogReadPhase::RetainResult => {
                if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
                    if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                        *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(result));
                    }
                }
                self.set_phase(DatabaseCatalogReadPhase::Publish);
                self.schedule_cleanup();
            }
            DatabaseCatalogReadPhase::Publish => self.publish_staged(),
            DatabaseCatalogReadPhase::Terminal => {}
        }
    }

    pub fn poll_backend_once(self: &Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        #[cfg(test)]
        self.poll_worker_thread.store(std::thread::current().name().is_some_and(|name| name.starts_with("semio-pool-worker-")), Ordering::Release);
        let Some(mut work) = self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else {
            self.stage_terminal(DbError::Unavailable("database catalog-read poll owner missing".to_string()), DatabaseCatalogReadProgress::Fault);
            return;
        };
        #[cfg(test)]
        if let Some(hook) = self.controlled_owner_transfer_hook.lock().unwrap().take() {
            hook(work.storage_identity);
        }
        self.polling.store(true, Ordering::Release);
        self.set_progress(DatabaseCatalogReadProgress::Polling);
        let wake = std::task::Waker::from(Arc::new(DatabaseCatalogReadWake { state: Arc::downgrade(self), generation }));
        let mut context = std::task::Context::from_waker(&wake);
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| work.poll(&mut context)));
        match polled {
            Ok(std::task::Poll::Pending) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                if !self.is_current() {
                    self.publish_terminal(DbError::Unavailable("database catalog-read generation stale during poll".to_string()), DatabaseCatalogReadProgress::Fault);
                    self.release_terminal_poll();
                } else if self.cancelled.load(Ordering::Acquire) {
                    self.publish_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
                    self.release_terminal_poll();
                } else {
                    self.set_progress(DatabaseCatalogReadProgress::Pending);
                    self.polling.store(false, Ordering::Release);
                    if self.wake_requested.swap(false, Ordering::AcqRel) {
                        self.schedule();
                    }
                }
            }
            Ok(std::task::Poll::Ready(result)) => {
                let too_large = result.root.as_ref().ok().and_then(Option::as_ref).is_some_and(|(bytes, _)| bytes.len() > DATABASE_CATALOG_READ_BYTES as usize);
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                *self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
                self.set_phase(DatabaseCatalogReadPhase::RetainWork);
                if too_large {
                    self.publish_terminal(DbError::LimitExceeded("database catalog-read result bytes"), DatabaseCatalogReadProgress::Fault);
                } else if !self.is_current() {
                    self.publish_terminal(DbError::Unavailable("database catalog-read generation stale after result".to_string()), DatabaseCatalogReadProgress::Fault);
                } else if self.cancelled.load(Ordering::Acquire) {
                    self.publish_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
                }
                self.release_terminal_poll();
            }
            Err(_) => {
                work.pollable = false;
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.publish_terminal(DbError::Unavailable("database catalog-read backend poll panicked".to_string()), DatabaseCatalogReadProgress::Fault);
                self.release_terminal_poll();
            }
        }
    }

    fn publish_terminal(&self, error: DbError, progress: DatabaseCatalogReadProgress) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.set_progress(progress);
        let mut terminal = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal.is_none() {
            *terminal = Some((error, progress));
        }
        drop(terminal);
        self.set_phase(DatabaseCatalogReadPhase::RetainWork);
    }

    pub fn release_terminal_poll(self: &Arc<Self>) {
        self.polling.store(false, std::sync::atomic::Ordering::Release);
        self.wake_requested.store(false, std::sync::atomic::Ordering::Release);
        self.schedule_cleanup();
    }

    pub fn stage_terminal(self: &Arc<Self>, error: DbError, progress: DatabaseCatalogReadProgress) {
        self.publish_terminal(error, progress);
        self.schedule_cleanup();
    }

    pub fn publish_staged(self: &Arc<Self>) {
        if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            if !self.is_current() {
                self.stage_terminal(DbError::Unavailable("database catalog-read generation stale before publication".to_string()), DatabaseCatalogReadProgress::Fault);
                return;
            }
            if self.cancelled.load(std::sync::atomic::Ordering::Acquire) {
                self.stage_terminal(DbError::Closed, DatabaseCatalogReadProgress::Cancelled);
                return;
            }
        }
        let result = if let Some((error, progress)) = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.set_progress(progress);
            Err(error)
        } else if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            self.set_progress(DatabaseCatalogReadProgress::Completed);
            Ok(result)
        } else {
            self.set_progress(DatabaseCatalogReadProgress::Fault);
            Err(DbError::Unavailable("database catalog-read publication owner missing".to_string()))
        };
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            *self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        } else {
            self.publish_public_completion(result);
        }
        self.set_phase(DatabaseCatalogReadPhase::Terminal);
    }

    pub(super) fn publish_public_completion(&self, result: Result<DatabaseCatalogReadResult, DbError>) {
        let mut completion = self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            drop(completion);
            *self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            return;
        }
        *completion = Some(result);
        drop(completion);
        #[cfg(test)]
        if let Some(hook) = self.controlled_completion_before_wake_hook.lock().unwrap().take() {
            hook();
        }
        let waker = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(waker) = waker {
            waker.wake();
        }
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

    pub fn close_step(self: &Arc<Self>) -> DatabaseCatalogReadCloseStep {
        use std::sync::atomic::Ordering;
        self.cancelled.store(true, Ordering::Release);
        let Some(mut lease) = self.try_lease(0, false) else {
            return if self.terminal_is_empty() { DatabaseCatalogReadCloseStep::Complete } else { DatabaseCatalogReadCloseStep::Blocked };
        };
        if let Some((_, job)) = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if let Some((job, _)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(job);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if self.retry_armed.load(Ordering::Acquire) {
            let current = self.retry_generation.load(Ordering::Acquire);
            if let Some(next) = current.checked_add(1) {
                self.retry_generation.store(next, Ordering::Release);
            }
            self.retry_armed.store(false, Ordering::Release);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if self.polling.load(Ordering::Acquire) {
            return DatabaseCatalogReadCloseStep::Blocked;
        }
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if let Some(work) = self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        {
            let mut terminal = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(work) = terminal.as_mut() {
                if work.close_step() {
                    return DatabaseCatalogReadCloseStep::Progress;
                }
                if work.terminal_is_empty() {
                    terminal.take();
                    return DatabaseCatalogReadCloseStep::Progress;
                }
            }
        }
        if self.terminal_result_checked_out.load(Ordering::Acquire) {
            return DatabaseCatalogReadCloseStep::Blocked;
        }
        if let Some(result) = self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if let Some(result) = self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            drop(result);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if let Some(result) = self.staged_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Ok(result));
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if let Some(result) = self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() {
            lease.retire();
            return DatabaseCatalogReadCloseStep::Progress;
        }
        if self.roots_are_empty() {
            lease.retire();
            DatabaseCatalogReadCloseStep::Complete
        } else {
            DatabaseCatalogReadCloseStep::Blocked
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
}
//#endregion 🔖️CatalogRootRead
