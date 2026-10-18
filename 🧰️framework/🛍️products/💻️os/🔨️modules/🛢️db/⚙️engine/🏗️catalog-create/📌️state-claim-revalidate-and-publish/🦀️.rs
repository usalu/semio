//! 📌️ State claim, revalidate and publish: the claim, handoff, backend poll, revalidation, intermediate retirement, publish and terminal retirement of the state. See [`super`].

use super::*;

//#region 🔖️CreateDocumentCatalogCas
impl DatabaseCreateCatalogState {
    pub(super) fn claim_one(self: &Arc<Self>) {
        let cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let token = DatabaseCreateCatalogToken { slot: self.slot, generation: self.generation, revision: cursor.base_revision };
        let mut catalog = match self.catalog.try_lock() {
            Ok(catalog) => catalog,
            Err(std::sync::TryLockError::Poisoned(error)) => error.into_inner(),
            Err(std::sync::TryLockError::WouldBlock) => {
                drop(cursor);
                self.defer_catalog_contention();
                return;
            }
        };
        if catalog.pending.is_some() && catalog.pending != Some(token) {
            drop(catalog);
            drop(cursor);
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if catalog.revision != cursor.base_revision || catalog.epoch != cursor.base_epoch || Arc::as_ptr(&catalog.entries) as usize != cursor.base_identity {
            let actual = catalog.epoch.epoch;
            let expected = cursor.base_epoch.epoch;
            drop(catalog);
            drop(cursor);
            self.stage_error(DbError::Fenced { expected, actual }, DatabaseCreateCatalogProgress::Fault);
            return;
        }
        catalog.pending = Some(token);
        self.pending_owned.store(true, std::sync::atomic::Ordering::Release);
        drop(catalog);
        drop(cursor);
        self.set_phase(DatabaseCreateCatalogPhase::Handoff);
    }

    pub fn handoff_one(&self) {
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let pages = cursor.pages.take();
        let expected = cursor.base_epoch;
        let storage = self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map(Arc::clone);
        match (storage, pages) {
            (Some(storage), Some(pages)) => {
                *self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(DatabaseCreateCatalogWork::new(storage, pages, expected));
                self.set_phase(DatabaseCreateCatalogPhase::Poll);
            }
            (storage, pages) => {
                drop(storage);
                cursor.pages = pages;
                drop(cursor);
                self.stage_error(DbError::LimitExceeded("database create-catalog handoff owner"), DatabaseCreateCatalogProgress::Fault);
            }
        }
    }

    pub fn poll_backend_once(self: &Arc<Self>, generation: u64) {
        use std::sync::atomic::Ordering;
        let Some(mut work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()) else {
            self.stage_error(DbError::LimitExceeded("database create-catalog poll owner"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        if self.polling.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            self.wake_requested.store(true, Ordering::Release);
            return;
        }
        if self.cancelled.load(Ordering::Acquire) {
            *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            self.polling.store(false, Ordering::Release);
            self.stage_error(DbError::Closed, DatabaseCreateCatalogProgress::Cancelled);
            return;
        }
        #[cfg(test)]
        self.poll_worker_thread.store(std::thread::current().name().is_some_and(|name| name.starts_with("semio-pool-worker-")), Ordering::Release);
        self.set_progress(DatabaseCreateCatalogProgress::Polling);
        let wake = std::task::Waker::from(Arc::new(DatabaseCreateCatalogWake { state: Arc::downgrade(self), generation }));
        let mut context = std::task::Context::from_waker(&wake);
        #[cfg(test)]
        self.backend_polls.fetch_add(1, Ordering::AcqRel);
        let polled = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| work.poll(&mut context)));
        match polled {
            Ok(std::task::Poll::Pending) => {
                *self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.polling.store(false, Ordering::Release);
                if self.cancelled.load(Ordering::Acquire) {
                    self.stage_error(DbError::Closed, DatabaseCreateCatalogProgress::Cancelled);
                }
            }
            Ok(std::task::Poll::Ready(actual)) => {
                *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                *self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(actual);
                self.polling.store(false, Ordering::Release);
                self.set_phase(DatabaseCreateCatalogPhase::CloseWork);
            }
            Err(_) => {
                *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
                self.polling.store(false, Ordering::Release);
                self.stage_error(DbError::LimitExceeded("database create-catalog backend poll panic"), DatabaseCreateCatalogProgress::Fault);
            }
        }
    }

    pub(super) fn close_work_one(&self) {
        let mut work = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let step = work.as_mut().map(DatabaseCreateCatalogWork::close_one);
        let empty = work.as_ref().is_some_and(DatabaseCreateCatalogWork::terminal_is_empty);
        if empty {
            work.take();
        }
        drop(work);
        match step {
            Some(Ok(_)) if empty => {
                if self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(Result::is_ok) {
                    self.set_phase(DatabaseCreateCatalogPhase::Revalidate);
                } else {
                    self.set_phase(DatabaseCreateCatalogPhase::Retire);
                }
            }
            Some(Ok(true)) => self.wake_requested.store(true, std::sync::atomic::Ordering::Release),
            Some(Err(error)) => self.stage_error(error, DatabaseCreateCatalogProgress::Fault),
            _ => self.stage_error(DbError::LimitExceeded("database create-catalog close work"), DatabaseCreateCatalogProgress::Fault),
        }
    }

    pub(super) fn revalidate_one(self: &Arc<Self>) {
        self.set_progress(DatabaseCreateCatalogProgress::Revalidating);
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let token = DatabaseCreateCatalogToken { slot: self.slot, generation: self.generation, revision: cursor.base_revision };
        let actual = self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().and_then(|actual| actual.as_ref().ok()).copied();
        let expected_next = cursor.base_epoch.epoch.checked_add(1);
        let mut catalog = match self.catalog.try_lock() {
            Ok(catalog) => catalog,
            Err(std::sync::TryLockError::Poisoned(error)) => error.into_inner(),
            Err(std::sync::TryLockError::WouldBlock) => {
                drop(cursor);
                self.defer_catalog_contention();
                return;
            }
        };
        if catalog.pending != Some(token) || catalog.revision != cursor.base_revision || catalog.epoch != cursor.base_epoch || Arc::as_ptr(&catalog.entries) as usize != cursor.base_identity || actual.map(|actual| actual.epoch) != expected_next {
            let expected = cursor.base_epoch.epoch;
            let actual = actual.map_or(catalog.epoch.epoch, |actual| actual.epoch);
            if catalog.pending == Some(token) {
                catalog.pending = None;
            }
            self.pending_owned.store(false, std::sync::atomic::Ordering::Release);
            drop(catalog);
            drop(cursor);
            self.stage_error(DbError::Fenced { expected, actual }, DatabaseCreateCatalogProgress::Fault);
            return;
        }
        let Some(snapshot) = cursor.snapshot.take() else {
            catalog.pending = None;
            self.pending_owned.store(false, std::sync::atomic::Ordering::Release);
            drop(catalog);
            drop(cursor);
            self.stage_error(DbError::LimitExceeded("database create-catalog publication snapshot"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        let Some(revision) = catalog.revision.checked_add(1) else {
            catalog.pending = None;
            self.pending_owned.store(false, std::sync::atomic::Ordering::Release);
            drop(catalog);
            cursor.snapshot = Some(snapshot);
            drop(cursor);
            self.stage_error(DbError::LimitExceeded("database create-catalog publication revision"), DatabaseCreateCatalogProgress::Fault);
            return;
        };
        catalog.entries = snapshot;
        catalog.epoch = actual.unwrap_or(cursor.base_epoch);
        catalog.revision = revision;
        catalog.pending = None;
        self.pending_owned.store(false, std::sync::atomic::Ordering::Release);
        drop(catalog);
        cursor.base.take();
        self.set_progress(DatabaseCreateCatalogProgress::Completed);
        self.set_phase(DatabaseCreateCatalogPhase::Publish);
    }

    pub(super) fn retire_intermediate_one(self: &Arc<Self>) {
        if self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            #[cfg(test)]
            self.terminal_job_retirements.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if self.pending_owned.load(std::sync::atomic::Ordering::Acquire) {
            let token = {
                let cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                DatabaseCreateCatalogToken { slot: self.slot, generation: self.generation, revision: cursor.base_revision }
            };
            let mut catalog = match self.catalog.try_lock() {
                Ok(catalog) => catalog,
                Err(std::sync::TryLockError::Poisoned(error)) => error.into_inner(),
                Err(std::sync::TryLockError::WouldBlock) => {
                    self.defer_catalog_contention();
                    return;
                }
            };
            if catalog.pending == Some(token) {
                catalog.pending = None;
            }
            self.pending_owned.store(false, std::sync::atomic::Ordering::Release);
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().or_else(|| self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take()) {
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        let mut work = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(owner) = work.as_mut() {
            let step = owner.close_one();
            let empty = owner.terminal_is_empty();
            if empty {
                work.take();
            }
            drop(work);
            match step {
                Ok(_) => self.wake_requested.store(true, std::sync::atomic::Ordering::Release),
                Err(error) => {
                    *self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Err(error));
                    self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
                }
            }
            return;
        }
        drop(work);
        let mut cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(pages) = cursor.pages.as_mut() {
            match pages.close_step() {
                Ok(Some(_)) => {}
                Ok(None) if pages.terminal_is_empty() => {
                    cursor.pages.take();
                }
                Ok(None) => {
                    drop(cursor);
                    self.stage_error(DbError::LimitExceeded("database create-catalog page retirement"), DatabaseCreateCatalogProgress::Fault);
                    return;
                }
                Err(error) => {
                    drop(cursor);
                    self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
                    return;
                }
            }
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if let Some(writer) = cursor.writer.as_mut() {
            match writer.close_step() {
                Ok(Some(_)) => {}
                Ok(None) if writer.terminal_is_empty() => {
                    cursor.writer.take();
                }
                Ok(None) => {
                    drop(cursor);
                    self.stage_error(DbError::LimitExceeded("database create-catalog writer retirement"), DatabaseCreateCatalogProgress::Fault);
                    return;
                }
                Err(error) => {
                    drop(cursor);
                    self.stage_error(error, DatabaseCreateCatalogProgress::Fault);
                    return;
                }
            }
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if let Some(snapshot) = cursor.snapshot.take() {
            match Arc::try_unwrap(snapshot) {
                Ok(candidate) => cursor.candidate = Some(candidate),
                Err(snapshot) => {
                    cursor.snapshot = Some(snapshot);
                    drop(cursor);
                    self.stage_error(DbError::LimitExceeded("database create-catalog snapshot retirement"), DatabaseCreateCatalogProgress::Fault);
                    return;
                }
            }
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if let Some(candidate) = cursor.candidate.as_mut() {
            if candidate.pop().is_some() {
                self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
                return;
            }
            cursor.candidate.take();
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if cursor.clone_text.take().is_some() {
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        if cursor.base.take().is_some() {
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        drop(cursor);
        let unpublished = self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
            || self.document.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
            || self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        if self.closing.load(std::sync::atomic::Ordering::Acquire) && !unpublished {
            self.set_phase(DatabaseCreateCatalogPhase::Terminal);
            return;
        }
        self.set_phase(DatabaseCreateCatalogPhase::Publish);
    }

    pub fn publish_one(self: &Arc<Self>) {
        let storage = self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let document = self.document.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let actual = self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        let expected = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner).base_epoch;
        match (storage, document, actual) {
            (Some(storage), Some(document), Some(actual)) => {
                let result = Ok(DatabaseCreateCatalogResult { state: Some(self.clone()), storage: Some(storage), document: Some(document), expected, actual: Some(actual) });
                if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
                    *self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
                } else {
                    *self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
                }
                self.set_phase(DatabaseCreateCatalogPhase::Terminal);
                if let Some(waker) = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
                    waker.wake();
                }
            }
            (storage, document, actual) => {
                *self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = storage;
                *self.document.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = document;
                *self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = actual;
                self.stage_error(DbError::LimitExceeded("database create-catalog result publication"), DatabaseCreateCatalogProgress::Fault);
            }
        }
    }

    pub fn roots_are_empty(&self) -> bool {
        let storage_empty = self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none();
        let document_empty = self.document.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none();
        let cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        storage_empty
            && document_empty
            && cursor.base.is_none()
            && cursor.candidate.is_none()
            && cursor.clone_text.is_none()
            && cursor.snapshot.is_none()
            && cursor.writer.is_none()
            && cursor.pages.is_none()
            && self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.pending_owned.load(std::sync::atomic::Ordering::Acquire)
    }

    pub fn retained_owner_count(&self) -> usize {
        let storage = usize::from(self.storage.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
        let document = usize::from(self.document.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some());
        let cursor = self.cursor.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        storage
            + document
            + usize::from(cursor.base.is_some())
            + cursor.candidate.as_ref().map_or(0, |entries| entries.len() + 1)
            + usize::from(cursor.clone_text.is_some())
            + cursor.snapshot.as_ref().map_or(0, |entries| entries.len() + 1)
            + usize::from(cursor.writer.is_some())
            + usize::from(cursor.pages.is_some())
            + usize::from(self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.poll_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.outcome.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map_or(0, DatabaseCreateCatalogResult::retained_owner_count)
            + self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().map_or(0, DatabaseCreateCatalogResult::retained_owner_count)
            + usize::from(self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some())
            + usize::from(self.pending_owned.load(std::sync::atomic::Ordering::Acquire))
    }

    pub(super) fn retire_terminal_one(self: &Arc<Self>) {
        if !self.roots_are_empty() {
            self.set_phase(DatabaseCreateCatalogPhase::Retire);
            self.retire_intermediate_one();
            return;
        }
        let mut result = self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(Ok(owner)) = result.as_mut() {
            if owner.close_one() {
                if owner.terminal_is_empty() {
                    result.take();
                }
                drop(result);
                self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
                return;
            }
        }
        if result.take().is_some() {
            drop(result);
            self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
            return;
        }
        drop(result);
        self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        self.pool_use.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        self.finished.store(true, std::sync::atomic::Ordering::Release);
        let mut registry = database_create_catalog_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if registry.get(self.slot).and_then(Option::as_ref).is_some_and(|state| state.generation == self.generation) {
            registry[self.slot] = None;
        }
    }

    pub fn release_success(self: &Arc<Self>) {
        if self.roots_are_empty() && self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.retire_terminal_one();
            return;
        }
        self.closing.store(true, std::sync::atomic::Ordering::Release);
        self.wake_requested.store(true, std::sync::atomic::Ordering::Release);
        self.begin_callback_close();
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.finished.load(std::sync::atomic::Ordering::Acquire)
            && self.roots_are_empty()
            && self.terminal_completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.retry_closing.load(std::sync::atomic::Ordering::Acquire)
            && !self.callback_close_armed.load(std::sync::atomic::Ordering::Acquire)
            && self.driver_authority.load(std::sync::atomic::Ordering::Acquire) == DatabaseCreateCatalogDriverAuthority::Idle as u8
    }
}
//#endregion 🔖️CreateDocumentCatalogCas
