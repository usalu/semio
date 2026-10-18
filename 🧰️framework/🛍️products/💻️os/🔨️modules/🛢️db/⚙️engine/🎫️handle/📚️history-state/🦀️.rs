//! 📚️ History state: the history state machine. See [`super`].

use super::*;

//#region 🔖️ArtifactHandle
impl ArtifactHistoryState {
    pub fn set_progress(&self, progress: HistoryProgress) {
        self.progress.store(progress as u8, std::sync::atomic::Ordering::Release);
    }

    pub fn wake_waiter(&self) {
        let waker = self.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(waker) = waker {
            waker.wake();
        }
    }

    pub(super) fn take_terminal_reservation(self: &Arc<Self>) -> Option<ArtifactHistoryTerminalReservation> {
        use std::sync::atomic::Ordering;
        if self.reservation_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        let owner = self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if owner.is_none() {
            self.reservation_checked_out.store(false, Ordering::Release);
            return None;
        }
        Some(ArtifactHistoryTerminalReservation { state: self.clone(), owner })
    }

    pub(super) fn take_terminal_construction_fault(self: &Arc<Self>) -> Option<ArtifactHistoryTerminalConstructionFault> {
        use std::sync::atomic::Ordering;
        if self.construction_checked_out.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        let owner = self.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if owner.is_none() {
            self.construction_checked_out.store(false, Ordering::Release);
            return None;
        }
        Some(ArtifactHistoryTerminalConstructionFault { state: self.clone(), owner })
    }

    pub fn terminal_roots_are_empty(&self) -> bool {
        self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.reservation_checked_out.load(std::sync::atomic::Ordering::Acquire)
            && self.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && !self.construction_checked_out.load(std::sync::atomic::Ordering::Acquire)
            && self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.finished.load(std::sync::atomic::Ordering::Acquire)
            && self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()
            && self.terminal_roots_are_empty()
            && !self.scheduled.load(std::sync::atomic::Ordering::Acquire)
            && !self.retry_armed.load(std::sync::atomic::Ordering::Acquire)
    }

    pub(super) fn finish_if_terminal_empty(&self) -> bool {
        use std::sync::atomic::Ordering;
        if !self.terminal_roots_are_empty() || self.scheduled.load(Ordering::Acquire) || self.retry_armed.load(Ordering::Acquire) {
            return false;
        }
        let mut admission = self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.finished.load(Ordering::Acquire) {
            return false;
        }
        let owner = admission.take();
        assert!(owner.as_ref().is_none_or(|owner| owner.reservation.is_none()), "artifact history admission finished before reservation retirement");
        drop(owner);
        unregister_artifact_history(self.generation);
        self.finished.store(true, Ordering::Release);
        true
    }

    pub(super) fn begin_unhanded_reservation_close(&self) {
        if self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some() || self.reservation_checked_out.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        let cursor = self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_mut().and_then(ArtifactHistoryAdmission::begin_reservation_close);
        if let Some(cursor) = cursor {
            *self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(cursor);
        }
    }

    pub(super) fn resume_unhanded_reservation(&self) -> bool {
        if self.reservation_checked_out.load(std::sync::atomic::Ordering::Acquire) {
            return false;
        }
        let mut terminal = self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let Some(cursor) = terminal.as_mut() else {
            return self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|owner| owner.reservation.is_some());
        };
        let mut admission = self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if admission.as_ref().is_none_or(|owner| owner.reservation.is_some()) {
            return false;
        }
        let Some(reservation) = cursor.resume() else {
            return false;
        };
        let admission = admission.as_mut().expect("artifact history reservation admission disappeared after preflight");
        admission.restore_reservation(reservation);
        terminal.take();
        true
    }

    pub fn complete(&self, result: ArtifactHistoryOutcome, progress: HistoryProgress) {
        self.set_progress(progress);
        #[cfg(test)]
        {
            let hook = self.controlled_completion_hook.lock().unwrap().take();
            if let Some(hook) = hook {
                hook(result, progress);
                return;
            }
        }
        let mut completion = self.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if self.abandoned.load(std::sync::atomic::Ordering::Acquire) {
            drop(completion);
            *self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        } else {
            *completion = Some(result);
            drop(completion);
            self.wake_waiter();
        }
    }

    pub(super) fn terminalize_work(&self, result: ArtifactHistoryOutcome, progress: HistoryProgress) {
        if let Some(work) = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            if matches!(&work, ArtifactHistoryWorkOwner::Request) {
                self.begin_unhanded_reservation_close();
            }
            *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(work);
        }
        self.complete(result, progress);
    }

    pub(super) fn terminalize_unhanded_request(&self, result: ArtifactHistoryOutcome, progress: HistoryProgress) -> bool {
        let mut work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !work.as_ref().is_some_and(|owner| matches!(owner, ArtifactHistoryWorkOwner::Request)) {
            return false;
        }
        let request = work.take().expect("artifact history request disappeared after retained preflight");
        drop(work);
        self.begin_unhanded_reservation_close();
        *self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(request);
        self.complete(result, progress);
        true
    }

    pub fn schedule(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.finished.load(Ordering::Acquire)
            || self.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some()
            || self.construction_checked_out.load(Ordering::Acquire)
            || self.scheduled.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err()
        {
            return;
        }
        self.set_progress(HistoryProgress::Scheduled);
        let state = self.clone();
        let generation = self.generation;
        self.submit_exact(Box::new(move || state.drive_one(generation)), 0);
    }

    pub fn submit_exact(self: &Arc<Self>, job: semio_framework_async::Job, attempt: u8) {
        match self.pool.try_submit(Lane::Io, job) {
            Ok(()) => {}
            Err(error) => match worker_submit_retry_attempt(error.kind(), attempt, ARTIFACT_SUBMIT_RETRY_LIMIT) {
                Some(next) => {
                    *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((error.into_job(), next));
                    self.arm_retry();
                }
                None => {
                    let kind = error.kind();
                    let job = error.into_job();
                    self.scheduled.store(false, std::sync::atomic::Ordering::Release);
                    *self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((kind, job));
                    self.terminalize_work(Err(DbError::Unavailable(format!("artifact history WorkerPool submission failed: {kind:?}"))), HistoryProgress::Fault);
                }
            },
        }
    }

    pub fn arm_retry(self: &Arc<Self>) {
        use std::sync::atomic::Ordering;
        if self.retry_armed.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return;
        }
        let generation = match self.retry_generation.try_update(Ordering::AcqRel, Ordering::Acquire, |generation| generation.checked_add(1).filter(|next| *next != 0)) {
            Ok(previous) => match previous.checked_add(1) {
                Some(generation) => generation,
                None => {
                    self.terminalize_retry_authority("artifact history retry generation exhausted");
                    return;
                }
            },
            Err(_) => {
                self.terminalize_retry_authority("artifact history retry generation exhausted");
                return;
            }
        };
        let Some(deadline) = self.pool.now_ms().checked_add(ARTIFACT_SUBMIT_RETRY_MS) else {
            self.terminalize_retry_authority("artifact history retry deadline exhausted");
            return;
        };
        let state = self.clone();
        self.pool.callback_at(deadline, move || {
            if generation != state.retry_generation.load(Ordering::Acquire) {
                return;
            }
            state.retry_armed.store(false, Ordering::Release);
            let retry = state.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            if let Some((job, attempt)) = retry {
                if state.cancelled.load(Ordering::Acquire) {
                    state.scheduled.store(false, Ordering::Release);
                    *state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, job));
                    state.terminalize_work(Err(DbError::Closed), HistoryProgress::Cancelled);
                } else {
                    state.submit_exact(job, attempt);
                }
            }
        });
    }

    pub(super) fn terminalize_retry_authority(&self, detail: &'static str) {
        self.retry_armed.store(false, std::sync::atomic::Ordering::Release);
        self.scheduled.store(false, std::sync::atomic::Ordering::Release);
        let retained = self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some((job, attempt)) = retained {
            let mut terminal = self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            if terminal.is_none() {
                *terminal = Some((semio_framework_async::WorkerSubmitErrorKind::Saturated, job));
            } else {
                drop(terminal);
                *self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some((job, attempt));
            }
        }
        self.terminalize_work(Err(DbError::Unavailable(detail.to_string())), HistoryProgress::Fault);
    }

    pub fn drive_one(self: Arc<Self>, generation: u64) {
        use std::future::Future as _;
        use std::sync::atomic::Ordering;

        if generation != self.generation {
            return;
        }
        self.scheduled.store(false, Ordering::Release);
        if self.authority.generation() != self.authority_generation {
            self.cancelled.store(true, Ordering::Release);
            let actor_active = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|owner| matches!(owner, ArtifactHistoryWorkOwner::Actor(_)));
            if !actor_active {
                self.terminalize_work(Err(DbError::StaleGeneration { expected: self.authority.generation(), actual: self.authority_generation }), HistoryProgress::Fault);
                return;
            }
        }
        if self.cancelled.load(Ordering::Acquire) {
            let actor_active = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|owner| matches!(owner, ArtifactHistoryWorkOwner::Actor(_)));
            if !actor_active {
                self.terminalize_work(Err(DbError::Closed), HistoryProgress::Cancelled);
                return;
            }
        }

        let mut work = self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        match work.as_mut() {
            Some(ArtifactHistoryWorkOwner::Request) => {
                let reservation = self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_mut().and_then(ArtifactHistoryAdmission::take_reservation);
                let Some(reservation) = reservation else {
                    drop(work);
                    self.terminalize_work(Err(DbError::Unavailable("artifact history fixed reservation owner is missing".to_string())), HistoryProgress::Fault);
                    return;
                };
                *work = Some(ArtifactHistoryWorkOwner::Actor(self.authority.history_retained(self.generation, self.cancelled.clone(), reservation)));
                drop(work);
                self.schedule();
            }
            Some(ArtifactHistoryWorkOwner::Actor(future)) => {
                let waker = std::task::Waker::from(Arc::new(ArtifactHistoryWake { state: Arc::downgrade(&self), generation }));
                let mut context = std::task::Context::from_waker(&waker);
                match std::pin::Pin::new(future).poll(&mut context) {
                    std::task::Poll::Pending => self.set_progress(HistoryProgress::Waiting),
                    std::task::Poll::Ready(Ok(Ok(view))) => {
                        work.take();
                        drop(work);
                        let admission = self.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
                        match admission {
                            Some(admission) => self.complete(Ok(HistoryView::new(view, Some(admission), self.clone())), HistoryProgress::Completed),
                            None => self.complete(Ok(HistoryView::new(view, None, self.clone())), HistoryProgress::Fault),
                        }
                    }
                    std::task::Poll::Ready(Ok(Err(error))) => {
                        work.take();
                        drop(work);
                        self.complete(Err(error), HistoryProgress::Fault);
                    }
                    std::task::Poll::Ready(Err(error)) => {
                        work.take();
                        drop(work);
                        self.complete(Err(error), HistoryProgress::Fault);
                    }
                }
            }
            None => {}
        }
    }

    pub fn close_one(self: &Arc<Self>) -> bool {
        if self.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return true;
        }
        if self.retry_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().is_some() {
            return true;
        }
        if let Some(work) = self.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            match work {
                ArtifactHistoryWorkOwner::Request => self.begin_unhanded_reservation_close(),
                ArtifactHistoryWorkOwner::Actor(owner) => {
                    *self.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ArtifactHistoryWorkOwner::Actor(owner));
                    self.cancelled.store(true, std::sync::atomic::Ordering::Release);
                    self.schedule();
                }
            }
            return true;
        }
        let mut reservation = self.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(cursor) = reservation.as_mut() {
            if cursor.close_step() {
                return true;
            }
            if cursor.terminal_is_empty() {
                reservation.take();
                return true;
            }
        }
        drop(reservation);
        let mut construction = self.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(cursor) = construction.as_mut() {
            if cursor.close_step() {
                return true;
            }
            if cursor.terminal_is_empty() {
                construction.take();
                return true;
            }
        }
        drop(construction);
        let mut terminal = self.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(result) = terminal.as_mut() {
            match result {
                Ok(view) => {
                    if view.close_step() {
                        return true;
                    }
                    terminal.take();
                    return true;
                }
                Err(_) => {
                    terminal.take();
                    return true;
                }
            }
        }
        false
    }
}
//#endregion 🔖️ArtifactHandle
