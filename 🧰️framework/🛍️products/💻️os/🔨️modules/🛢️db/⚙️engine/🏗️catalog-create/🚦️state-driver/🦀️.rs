//! 🚦️ State driver: the phase accessors, scheduling, retries, callback close and driving steps of the state. See [`super`].

use super::*;

//#region 🔖️CreateDocumentCatalogCas
impl DatabaseCreateCatalogState {
    pub fn phase(&self) -> DatabaseCreateCatalogPhase {
        DatabaseCreateCatalogPhase::from_u8(self.phase.load(std::sync::atomic::Ordering::Acquire))
    }

    pub fn set_phase(&self, phase: DatabaseCreateCatalogPhase) {
        self.phase.store(phase as u8, std::sync::atomic::Ordering::Release);
        self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
    }

    fn accepts_interruption(&self) -> bool {
        match self.phase() {
            DatabaseCreateCatalogPhase::Publish | DatabaseCreateCatalogPhase::Terminal => false,
            DatabaseCreateCatalogPhase::CloseWork | DatabaseCreateCatalogPhase::Revalidate | DatabaseCreateCatalogPhase::Retire => {
                !self.closing.load(std::sync::atomic::Ordering::Acquire) && self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            }
            _ => true,
        }
    }

    pub fn set_progress(&self, progress: DatabaseCreateCatalogProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn is_current(&self) -> bool {
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseCreateCatalogAdmission::is_current)
    }

    pub fn observed_generation(&self) -> u64 {
        DATABASE_CREATE_CATALOG_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots.get(self.slot).map_or(0, |entry| entry.generation)
    }

    pub(super) fn observed_capacity(&self, capacity: usize) -> usize {
        #[cfg(test)]
        {
            return capacity.saturating_add(self.controlled_capacity_overage.swap(0, std::sync::atomic::Ordering::AcqRel));
        }
        #[cfg(not(test))]
        capacity
    }

    pub(super) fn defer_catalog_contention(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.catalog_contention_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || {
            state.catalog_contention_armed.store(false, Ordering::Release);
            if !state.finished.load(Ordering::Acquire) {
                state.wake_requested.store(true, Ordering::Release);
                state.schedule();
            }
        });
    }

    pub fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire) {
            return;
        }
        if self.retry_closing.load(Ordering::Acquire) {
            self.arm_callback_close();
            return;
        }
        while self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Idle as u8, DatabaseCreateCatalogDriverAuthority::Queued as u8, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            self.wake_requested.store(true, Ordering::SeqCst);
            if self.driver_authority.load(Ordering::SeqCst) != DatabaseCreateCatalogDriverAuthority::Idle as u8 || !self.wake_requested.swap(false, Ordering::SeqCst) {
                return;
            }
        }
        self.wake_requested.swap(false, Ordering::SeqCst);
        let state = self.clone();
        let generation = self.generation;
        self.submit_exact(Box::new(move || state.drive_one(generation)), 0);
    }

    pub fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => {
                let next_attempt = worker_submit_retry_attempt(error.kind(), attempt, DATABASE_CREATE_CATALOG_RETRY_LIMIT).unwrap_or(DATABASE_CREATE_CATALOG_RETRY_LIMIT);
                #[cfg(test)]
                if next_attempt != attempt {
                    self.spent_submission_attempts.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                }
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), next_attempt));
                if self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Queued as u8, DatabaseCreateCatalogDriverAuthority::Retry as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
                    let state = self.clone();
                    self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.retry());
                }
            }
        }
    }

    pub fn retry(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        #[cfg(test)]
        self.callback_worker_thread.store(std::thread::current().name().is_some_and(|name| name.starts_with("semio-pool-worker-")), Ordering::Release);
        let Some((job, attempt)) = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() else { return };
        let interruptible = self.accepts_interruption();
        let cancelled = self.cancelled.load(Ordering::Acquire);
        let exhausted = attempt >= DATABASE_CREATE_CATALOG_RETRY_LIMIT;
        let terminal = if !self.is_current() {
            Some((DbError::StaleGeneration { expected: GenerationId(self.generation), actual: GenerationId(self.observed_generation()) }, DatabaseCreateCatalogProgress::Fault))
        } else if cancelled && interruptible {
            Some((DbError::Closed, DatabaseCreateCatalogProgress::Cancelled))
        } else if self.pool.now_ms() >= self.deadline_ms.load(Ordering::Acquire) && interruptible {
            Some((DbError::Timeout(String::from("database create-catalog retry deadline")), DatabaseCreateCatalogProgress::Fault))
        } else if exhausted && interruptible {
            Some((DbError::LimitExceeded("database create-catalog retry exhausted"), DatabaseCreateCatalogProgress::Fault))
        } else {
            None
        };
        if terminal.is_some() || cancelled || exhausted {
            if self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Retry as u8, DatabaseCreateCatalogDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(job);
                self.retry_closing.store(true, Ordering::Release);
                if let Some((error, progress)) = terminal {
                    self.stage_error(error, progress);
                }
                self.drive_callback_close_claimed(true);
            } else {
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            }
            return;
        }
        if self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Retry as u8, DatabaseCreateCatalogDriverAuthority::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            self.submit_exact(job, attempt);
        } else {
            *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
        }
    }

    pub(super) fn arm_callback_close(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.callback_close_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || state.callback_close_one());
    }

    pub(super) fn callback_close_one(self: Arc<Self>) {
        use std::sync::atomic::Ordering;
        #[cfg(test)]
        self.callback_worker_thread.store(std::thread::current().name().is_some_and(|name| name.starts_with("semio-pool-worker-")), Ordering::Release);
        self.callback_close_armed.store(false, Ordering::Release);
        if !self.retry_closing.load(Ordering::Acquire) || self.finished.load(Ordering::Acquire) {
            return;
        }
        if self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Idle as u8, DatabaseCreateCatalogDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            self.arm_callback_close();
            return;
        }
        self.drive_callback_close_claimed(true);
    }

    fn drive_callback_close_claimed(self: Arc<Self>, autonomous: bool) {
        use std::sync::atomic::Ordering;
        self.opportunities.fetch_add(1, Ordering::AcqRel);
        let driven = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.drive_claimed(self.generation)));
        if driven.is_err() {
            self.stage_error(DbError::LimitExceeded("database create-catalog callback close panic"), DatabaseCreateCatalogProgress::Fault);
        }
        self.wake_requested.store(false, Ordering::Release);
        self.driver_authority.store(DatabaseCreateCatalogDriverAuthority::Idle as u8, Ordering::Release);
        let keep_closing = !self.finished.load(Ordering::Acquire) && (self.phase() != DatabaseCreateCatalogPhase::Terminal || self.closing.load(Ordering::Acquire));
        if keep_closing && autonomous {
            self.arm_callback_close();
        } else {
            self.retry_closing.store(false, Ordering::Release);
            if !keep_closing {
                self.callback_close_armed.store(false, Ordering::Release);
            }
        }
    }

    pub(super) fn drive_explicit_close_one(self: &Arc<Self>) -> bool {
        use std::sync::atomic::Ordering;
        self.retry_closing.store(true, Ordering::Release);
        if self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Idle as u8, DatabaseCreateCatalogDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return false;
        }
        self.clone().drive_callback_close_claimed(false);
        true
    }

    pub(super) fn begin_callback_close(self: &Arc<Self>) {
        self.retry_closing.store(true, std::sync::atomic::Ordering::Release);
        self.arm_callback_close();
    }

    pub fn drive_one(self: Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        if self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Queued as u8, DatabaseCreateCatalogDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        #[cfg(test)]
        {
            let active = self.active_drivers.fetch_add(1, Ordering::AcqRel) + 1;
            self.max_active_drivers.fetch_max(active, Ordering::AcqRel);
        }
        self.opportunities.fetch_add(1, Ordering::AcqRel);
        let driven = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            #[cfg(test)]
            if let Some(hook) = self.controlled_driver_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone() {
                hook(self.phase());
            }
            self.drive_claimed(generation);
        }));
        if driven.is_err() {
            self.stage_error(DbError::LimitExceeded("database create-catalog worker panic"), DatabaseCreateCatalogProgress::Fault);
        }
        #[cfg(test)]
        self.active_drivers.fetch_sub(1, Ordering::AcqRel);
        if self.driver_authority.compare_exchange(DatabaseCreateCatalogDriverAuthority::Driving as u8, DatabaseCreateCatalogDriverAuthority::Idle as u8, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            self.stage_error(DbError::LimitExceeded("database create-catalog driver release"), DatabaseCreateCatalogProgress::Fault);
        }
        if self.wake_requested.swap(false, Ordering::SeqCst) {
            self.schedule();
        }
    }

    pub fn drive_claimed(self: &Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire) {
            return;
        }
        if self.phase() == DatabaseCreateCatalogPhase::Terminal {
            if self.closing.load(Ordering::Acquire) {
                self.retire_terminal_one();
            }
            return;
        }
        if !self.retry_closing.load(Ordering::Acquire) && (generation != self.generation || !self.is_current()) {
            self.stage_error(DbError::StaleGeneration { expected: GenerationId(self.generation), actual: GenerationId(self.observed_generation()) }, DatabaseCreateCatalogProgress::Fault);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) && self.accepts_interruption() {
            self.stage_error(DbError::Closed, DatabaseCreateCatalogProgress::Cancelled);
            return;
        }
        if self.pool.now_ms() >= self.deadline_ms.load(Ordering::Acquire) && self.accepts_interruption() {
            self.stage_error(DbError::Timeout(String::from("database create-catalog deadline")), DatabaseCreateCatalogProgress::Fault);
            return;
        }
        match self.phase() {
            DatabaseCreateCatalogPhase::Scan => self.scan_one(),
            DatabaseCreateCatalogPhase::Reserve => self.reserve_candidate_one(),
            DatabaseCreateCatalogPhase::Clone => self.clone_one(),
            DatabaseCreateCatalogPhase::Snapshot => self.snapshot_one(),
            DatabaseCreateCatalogPhase::Encode => self.encode_one(),
            DatabaseCreateCatalogPhase::Seal => self.seal_one(),
            DatabaseCreateCatalogPhase::Claim => self.claim_one(),
            DatabaseCreateCatalogPhase::Handoff => self.handoff_one(),
            DatabaseCreateCatalogPhase::Poll => self.poll_backend_once(generation),
            DatabaseCreateCatalogPhase::CloseWork => self.close_work_one(),
            DatabaseCreateCatalogPhase::Revalidate => self.revalidate_one(),
            DatabaseCreateCatalogPhase::Retire => self.retire_intermediate_one(),
            DatabaseCreateCatalogPhase::Publish => self.publish_one(),
            DatabaseCreateCatalogPhase::Terminal => {}
        }
    }

    pub fn stage_error(&self, error: DbError, progress: DatabaseCreateCatalogProgress) {
        let mut outcome = self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if outcome.is_none() || outcome.as_ref().is_some_and(Result::is_ok) {
            *outcome = Some(Err(error));
        }
        drop(outcome);
        self.set_progress(progress);
        self.set_phase(DatabaseCreateCatalogPhase::Retire);
    }

}
//#endregion 🔖️CreateDocumentCatalogCas
