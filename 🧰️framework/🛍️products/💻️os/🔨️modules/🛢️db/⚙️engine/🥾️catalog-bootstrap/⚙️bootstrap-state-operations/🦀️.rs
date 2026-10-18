//! ⚙️ Catalog bootstrap state operations: the drive, submit, retry and completion operations of the shared bootstrap state. See [`super`].

use super::*;

//#region 🔖️CatalogBootstrapCas
impl DatabaseCatalogBootstrapState {
    pub fn phase(&self) -> DatabaseCatalogBootstrapPhase {
        DatabaseCatalogBootstrapPhase::from_u8(self.phase.load(std::sync::atomic::Ordering::Acquire))
    }

    pub fn set_phase(&self, phase: DatabaseCatalogBootstrapPhase) {
        self.phase.store(phase as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn set_progress(&self, progress: DatabaseCatalogBootstrapProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn is_current(&self) -> bool {
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(DatabaseCatalogBootstrapAdmission::is_current)
    }

    pub fn observed_generation(&self) -> u64 {
        DATABASE_CATALOG_BOOTSTRAP_ADMISSION.lock().unwrap_or_else(std::sync::PoisonError::into_inner).slots.get(self.slot).map_or(0, |entry| entry.generation)
    }

    pub fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire) {
            return;
        }
        while self.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Idle as u8, DatabaseCatalogBootstrapDriverAuthority::Queued as u8, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            self.wake_requested.store(true, Ordering::SeqCst);
            if self.driver_authority.load(Ordering::SeqCst) != DatabaseCatalogBootstrapDriverAuthority::Idle as u8 || !self.wake_requested.swap(false, Ordering::SeqCst) {
                return;
            }
        }
        self.wake_requested.swap(false, Ordering::SeqCst);
        self.scheduled.store(true, Ordering::Release);
        self.set_progress(DatabaseCatalogBootstrapProgress::Scheduled);
        let state = self.clone();
        let generation = self.generation;
        let job: semio_framework_async::Job = Box::new(move || state.drive_one(generation));
        #[cfg(test)]
        let job = if let Some(submit) = self.controlled_submit_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone() {
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
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => {
                let next_attempt = attempt.checked_add(1).map_or(DATABASE_CATALOG_BOOTSTRAP_RETRY_LIMIT, |next| next.min(DATABASE_CATALOG_BOOTSTRAP_RETRY_LIMIT));
                if next_attempt == DATABASE_CATALOG_BOOTSTRAP_RETRY_LIMIT {
                    self.retry_pressure.store(true, std::sync::atomic::Ordering::Release);
                }
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), next_attempt));
                self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                if self.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Queued as u8, DatabaseCatalogBootstrapDriverAuthority::Retry as u8, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok()
                {
                    self.arm_retry();
                } else {
                    *self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DbError::LimitExceeded("database catalog-bootstrap driver queue authority"));
                    self.retry_pressure.store(true, std::sync::atomic::Ordering::Release);
                    self.closing.store(true, std::sync::atomic::Ordering::Release);
                    self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
                    self.schedule();
                }
            }
        }
    }

    pub fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let state = self.clone();
        self.pool.callback_at(self.pool.now_ms().saturating_add(1), move || {
            state.retry_armed.store(false, Ordering::Release);
            let retained = state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some((job, attempt)) = retained {
                if state.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Retry as u8, DatabaseCatalogBootstrapDriverAuthority::Queued as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                    state.scheduled.store(true, Ordering::Release);
                    state.submit_exact(job, attempt);
                } else {
                    *state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
                    *state.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DbError::LimitExceeded("database catalog-bootstrap retry authority"));
                    state.retry_pressure.store(true, Ordering::Release);
                    state.closing.store(true, Ordering::Release);
                    state.wake_requested.store(true, Ordering::Release);
                    state.schedule();
                }
            } else if state.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Retry as u8, DatabaseCatalogBootstrapDriverAuthority::Idle as u8, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                state.schedule();
            }
        });
    }

    pub fn drive_one(self: Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        if self.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Queued as u8, DatabaseCatalogBootstrapDriverAuthority::Driving as u8, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        self.scheduled.store(false, Ordering::Release);
        #[cfg(test)]
        {
            self.driver_grants.fetch_add(1, Ordering::AcqRel);
            let active = self.active_drivers.fetch_add(1, Ordering::AcqRel) + 1;
            self.max_active_drivers.fetch_max(active, Ordering::AcqRel);
            if let Some(hook) = self.controlled_driver_claim_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone() {
                hook(self.phase());
            }
        }
        self.drive_claimed(generation);
        #[cfg(test)]
        if let Some(hook) = self.controlled_driver_release_hook.lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone() {
            hook(self.phase());
        }
        #[cfg(test)]
        self.active_drivers.fetch_sub(1, Ordering::AcqRel);
        if self.driver_authority.compare_exchange(DatabaseCatalogBootstrapDriverAuthority::Driving as u8, DatabaseCatalogBootstrapDriverAuthority::Idle as u8, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            *self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DbError::LimitExceeded("database catalog-bootstrap driver release authority"));
            self.retry_pressure.store(true, Ordering::Release);
            self.closing.store(true, Ordering::Release);
        }
        if self.wake_requested.swap(false, Ordering::SeqCst) {
            self.schedule();
        }
    }

    pub fn drive_claimed(self: &Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        if generation != self.generation {
            return;
        }
        if self.closing.load(Ordering::Acquire) {
            self.retire_one();
            if !self.finished.load(Ordering::Acquire) || !self.roots_are_empty() {
                self.schedule();
            }
            return;
        }
        if !self.is_current() && matches!(self.phase(), DatabaseCatalogBootstrapPhase::Handoff | DatabaseCatalogBootstrapPhase::Poll) {
            self.stage_error(DbError::StaleGeneration { expected: GenerationId(self.generation), actual: GenerationId(self.observed_generation()) }, DatabaseCatalogBootstrapProgress::Fault);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) && !matches!(self.phase(), DatabaseCatalogBootstrapPhase::Validate | DatabaseCatalogBootstrapPhase::Publish | DatabaseCatalogBootstrapPhase::Terminal) {
            let mut terminal = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if terminal.is_none() {
                *terminal = Some(DbError::Closed);
            }
            drop(terminal);
            self.set_progress(DatabaseCatalogBootstrapProgress::Cancelled);
            if matches!(self.phase(), DatabaseCatalogBootstrapPhase::Handoff | DatabaseCatalogBootstrapPhase::Poll) {
                self.set_phase(DatabaseCatalogBootstrapPhase::RetainWork);
                self.schedule();
                return;
            }
        }
        match self.phase() {
            DatabaseCatalogBootstrapPhase::Handoff => self.handoff_one(),
            DatabaseCatalogBootstrapPhase::Poll => self.poll_backend_once(generation),
            DatabaseCatalogBootstrapPhase::RetainWork => {
                let work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take());
                if let Some(work) = work {
                    *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                    self.set_phase(DatabaseCatalogBootstrapPhase::CloseWork);
                } else {
                    self.set_phase(DatabaseCatalogBootstrapPhase::RetireInput);
                }
                self.schedule();
            }
            DatabaseCatalogBootstrapPhase::CloseWork => {
                let mut work = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let step = work.as_mut().map(DatabaseCatalogBootstrapWork::close_one);
                let empty = work.as_ref().is_some_and(DatabaseCatalogBootstrapWork::terminal_is_empty);
                if empty {
                    work.take();
                }
                drop(work);
                if empty {
                    self.set_phase(DatabaseCatalogBootstrapPhase::RetireInput);
                    self.schedule();
                } else if matches!(step, Some(Ok(true))) {
                    self.schedule();
                } else if let Some(Err(error)) = step {
                    *self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(error);
                    self.set_progress(DatabaseCatalogBootstrapProgress::Fault);
                    self.closing.store(true, Ordering::Release);
                    self.schedule();
                } else {
                    self.stage_error(DbError::LimitExceeded("database catalog-bootstrap work witness"), DatabaseCatalogBootstrapProgress::Fault);
                }
            }
            DatabaseCatalogBootstrapPhase::RetireInput => self.retire_input_one(),
            DatabaseCatalogBootstrapPhase::Validate => self.validate_one(),
            DatabaseCatalogBootstrapPhase::Publish => self.publish_one(),
            DatabaseCatalogBootstrapPhase::Terminal => {}
        }
    }

    pub fn handoff_one(self: &Arc<Self>) {
        if self.expected != EpochFence::INITIAL {
            self.stage_error(DbError::Fenced { expected: EpochFence::INITIAL.epoch, actual: self.expected.epoch }, DatabaseCatalogBootstrapProgress::Fault);
            return;
        }
        let pages = self.pages.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let storage = self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map(Arc::clone);
        match (storage, pages) {
            (Some(storage), Some(pages)) => {
                *self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCatalogBootstrapWork::new(storage, pages, self.expected));
                self.set_phase(DatabaseCatalogBootstrapPhase::Poll);
                self.schedule();
            }
            (storage, pages) => {
                if let Some(pages) = pages {
                    *self.pages.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(pages);
                }
                drop(storage);
                self.stage_error(DbError::LimitExceeded("database catalog-bootstrap handoff owner"), DatabaseCatalogBootstrapProgress::Fault);
            }
        }
    }

    pub fn poll_backend_once(self: &Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        let Some(mut work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()) else {
            self.stage_error(DbError::LimitExceeded("database catalog-bootstrap poll owner"), DatabaseCatalogBootstrapProgress::Fault);
            return;
        };
        if self.polling.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            self.wake_requested.store(true, Ordering::Release);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) {
            *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            self.publish_poll_error(DbError::Closed, DatabaseCatalogBootstrapProgress::Cancelled);
            self.release_poll();
            return;
        }
        self.set_progress(DatabaseCatalogBootstrapProgress::Polling);
        #[cfg(test)]
        self.poll_worker_thread.store(std::thread::current().name().is_some_and(|name| name.starts_with("semio-pool-worker-")), Ordering::Release);
        let wake = std::task::Waker::from(Arc::new(DatabaseCatalogBootstrapWake { state: Arc::downgrade(self), generation }));
        let mut context = std::task::Context::from_waker(&wake);
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| work.poll(&mut context)));
        match polled {
            Ok(std::task::Poll::Pending) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                if !self.is_current() {
                    self.publish_poll_error(DbError::StaleGeneration { expected: GenerationId(self.generation), actual: GenerationId(self.observed_generation()) }, DatabaseCatalogBootstrapProgress::Fault);
                } else if self.cancelled.load(Ordering::Acquire) {
                    self.publish_poll_error(DbError::Closed, DatabaseCatalogBootstrapProgress::Cancelled);
                } else {
                    self.set_progress(DatabaseCatalogBootstrapProgress::Pending);
                }
                self.release_poll();
            }
            Ok(std::task::Poll::Ready(actual)) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                *self.staged_actual.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(actual);
                self.set_phase(DatabaseCatalogBootstrapPhase::RetainWork);
                if !self.is_current() {
                    self.publish_poll_error(DbError::StaleGeneration { expected: GenerationId(self.generation), actual: GenerationId(self.observed_generation()) }, DatabaseCatalogBootstrapProgress::Fault);
                } else if self.cancelled.load(Ordering::Acquire) {
                    self.publish_poll_error(DbError::Closed, DatabaseCatalogBootstrapProgress::Cancelled);
                }
                self.release_poll();
            }
            Err(_) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.publish_poll_error(DbError::LimitExceeded("database catalog-bootstrap backend poll panic"), DatabaseCatalogBootstrapProgress::Fault);
                self.release_poll();
            }
        }
    }

    fn publish_poll_error(&self, error: DbError, progress: DatabaseCatalogBootstrapProgress) {
        self.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.set_progress(progress);
        let mut terminal = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal.is_none() {
            *terminal = Some(error);
        }
        self.set_phase(DatabaseCatalogBootstrapPhase::RetainWork);
    }

    fn release_poll(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        self.polling.store(false, Ordering::Release);
        if self.phase() != DatabaseCatalogBootstrapPhase::Poll {
            self.wake_requested.store(true, Ordering::Release);
        }
    }

    pub fn stage_error(self: &Arc<Self>, error: DbError, progress: DatabaseCatalogBootstrapProgress) {
        self.publish_poll_error(error, progress);
        self.schedule();
    }

    fn retire_input_one(self: &Arc<Self>) {
        let mut pages = self.pages.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(owner) = pages.as_mut() {
            match owner.close_step() {
                Ok(Some(_)) => {
                    drop(pages);
                    self.schedule();
                    return;
                }
                Ok(None) if owner.terminal_is_empty() => {
                    pages.take();
                    drop(pages);
                    self.schedule();
                    return;
                }
                Ok(None) => {
                    drop(pages);
                    self.stage_error(DbError::LimitExceeded("database catalog-bootstrap input retirement"), DatabaseCatalogBootstrapProgress::Fault);
                    return;
                }
                Err(error) => {
                    drop(pages);
                    self.stage_error(error, DatabaseCatalogBootstrapProgress::Fault);
                    return;
                }
            }
        }
        drop(pages);
        self.set_phase(DatabaseCatalogBootstrapPhase::Validate);
        self.schedule();
    }

    fn validate_one(self: &Arc<Self>) {
        let actual = if let Some(error) = self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            Err(error)
        } else if let Some(actual) = self.staged_actual.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            match actual {
                Ok(actual) => match self.expected.epoch.checked_add(1) {
                    Some(expected) if actual.epoch == expected => Ok(actual),
                    Some(expected) => Err(DbError::Fenced { expected, actual: actual.epoch }),
                    None => Err(DbError::LimitExceeded("database catalog-bootstrap expected revision")),
                },
                Err(error) => Err(error),
            }
        } else {
            Err(DbError::LimitExceeded("database catalog-bootstrap result owner"))
        };
        *self.staged_actual.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(actual);
        self.set_phase(DatabaseCatalogBootstrapPhase::Publish);
        self.schedule();
    }

    pub fn publish_one(self: &Arc<Self>) {
        if !self.is_current() && self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            *self.staged_actual.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Err(DbError::StaleGeneration { expected: GenerationId(self.generation), actual: GenerationId(self.observed_generation()) }));
            self.set_progress(DatabaseCatalogBootstrapProgress::Fault);
        }
        let storage = self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let key = self.key.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let actual = self.staged_actual.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let completion = match (storage, key, actual) {
            (Some(storage), Some(key), Some(actual)) => {
                self.set_progress(match actual {
                    Ok(_) => DatabaseCatalogBootstrapProgress::Completed,
                    Err(DbError::Closed) => DatabaseCatalogBootstrapProgress::Cancelled,
                    Err(_) => DatabaseCatalogBootstrapProgress::Fault,
                });
                Ok(DatabaseCatalogBootstrapResult { state: Some(self.clone()), storage: Some(storage), key: Some(key), expected: self.expected, actual: Some(actual) })
            }
            (storage, key, actual) => {
                *self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = storage;
                *self.key.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = key;
                *self.staged_actual.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = actual;
                self.set_progress(DatabaseCatalogBootstrapProgress::Fault);
                Err(DbError::LimitExceeded("database catalog-bootstrap publication owner"))
            }
        };
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            *self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(completion);
        } else {
            *self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(completion);
            if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                waker.wake();
            }
        }
        self.set_phase(DatabaseCatalogBootstrapPhase::Terminal);
    }

    pub fn retained_owner_count(&self) -> usize {
        usize::from(self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.pages.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.key.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.staged_actual.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
    }

    pub fn roots_are_empty(&self) -> bool {
        self.retained_owner_count() == 0
    }

    fn retire_one(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() || self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return;
        }
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()) {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            return;
        }
        {
            let mut work = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(step) = work.as_mut().map(DatabaseCatalogBootstrapWork::close_one) {
                match step {
                    Ok(true) => return,
                    Ok(false) => {}
                    Err(error) => {
                        *self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(error);
                        self.set_progress(DatabaseCatalogBootstrapProgress::Fault);
                        return;
                    }
                }
            }
            if work.as_ref().is_some_and(DatabaseCatalogBootstrapWork::terminal_is_empty) {
                work.take();
                return;
            }
        }
        {
            let mut pages = self.pages.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if let Some(owner) = pages.as_mut() {
                if owner.close_step().ok().flatten().is_some() {
                    return;
                }
                if owner.terminal_is_empty() {
                    pages.take();
                    return;
                }
            }
        }
        let retained = self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take());
        if let Some(mut result) = retained {
            if let Ok(owner) = result.as_mut() {
                owner.close_one();
                if !owner.terminal_is_empty() {
                    *self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
                }
            }
            return;
        }
        if self.staged_actual.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some()
            || self.terminal_error.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some()
            || self.key.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some()
            || self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some()
            || self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some()
        {
            return;
        }
        if self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            let mut registry = database_catalog_bootstrap_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if registry.get(self.slot).and_then(Option::as_ref).is_some_and(|state| state.generation == self.generation) {
                registry[self.slot] = None;
            }
            return;
        }
        self.finished.store(true, Ordering::Release);
    }

    pub fn release_success(&self) {
        if !self.roots_are_empty() && self.retained_owner_count() != 1 {
            return;
        }
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        self.finished.store(true, std::sync::atomic::Ordering::Release);
        let mut registry = database_catalog_bootstrap_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if registry.get(self.slot).and_then(Option::as_ref).is_some_and(|state| state.generation == self.generation) {
            registry[self.slot] = None;
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.finished.load(std::sync::atomic::Ordering::Acquire) && self.roots_are_empty() && self.driver_authority.load(std::sync::atomic::Ordering::Acquire) == DatabaseCatalogBootstrapDriverAuthority::Idle as u8
    }
}
//#endregion 🔖️CatalogBootstrapCas
