//! 🔭️ History future and terminals: the history future with its terminal handle, job, work, reservation and construction. See [`super`].

use super::*;

//#region 🔖️ArtifactHandle
impl HistoryFuture {
    pub fn take_completion(&mut self) -> Option<ArtifactHistoryOutcome> {
        let result = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if result.is_some() {
            self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            self.resolved = true;
            self.state.finish_if_terminal_empty();
        }
        result
    }

    pub fn submit(handle: &ArtifactHandle) -> Self {
        let future = Self::prepare(handle);
        let ready = future.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_some();
        if ready {
            future.state.schedule();
        }
        future
    }

    pub fn prepare(handle: &ArtifactHandle) -> Self {
        let (admission, construction_fault, rejected_error, generation) = match ArtifactHistoryAdmission::try_claim() {
            Ok(admission) => {
                let generation = admission.generation;
                (Some(admission), None, None, generation)
            }
            Err(ArtifactHistoryAdmissionError::Construction { admission, fault }) => {
                let generation = admission.generation;
                (Some(admission), Some(fault), None, generation)
            }
            Err(ArtifactHistoryAdmissionError::Rejected(error)) => (None, None, Some(error), 0),
        };
        let ready = construction_fault.is_none() && rejected_error.is_none();
        let state = Arc::new(ArtifactHistoryState {
            pool: handle.pool.as_ref().clone(),
            authority: handle.authority.clone(),
            generation,
            authority_generation: handle.authority.generation(),
            admission: Mutex::new(admission),
            work: Mutex::new(ready.then_some(ArtifactHistoryWorkOwner::Request)),
            completion: Mutex::new(None),
            terminal_work: Mutex::new(None),
            terminal_result: Mutex::new(None),
            terminal_reservation: Mutex::new(None),
            reservation_checked_out: std::sync::atomic::AtomicBool::new(false),
            terminal_construction: Mutex::new(construction_fault),
            construction_checked_out: std::sync::atomic::AtomicBool::new(false),
            retry_job: Mutex::new(None),
            terminal_job: Mutex::new(None),
            waker: Mutex::new(None),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            retry_generation: std::sync::atomic::AtomicU64::new(1),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            cancelled: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            abandoned: std::sync::atomic::AtomicBool::new(false),
            finished: std::sync::atomic::AtomicBool::new(false),
            progress: std::sync::atomic::AtomicU8::new(if generation == 0 { HistoryProgress::Fault as u8 } else { HistoryProgress::Admitted as u8 }),
            #[cfg(test)]
            controlled_completion_hook: Mutex::new(None),
            #[cfg(any(test, feature = "testkit"))]
            controlled_publication_before_waker_hook: Mutex::new(None),
        });
        register_artifact_history(&state);
        let construction_error = state.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_mut().and_then(db_artifact_history::HistoryReplayReservationConstructionFault::take_error);
        if let Some(error) = rejected_error.or(construction_error) {
            *state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Err(error));
        }
        Self { state, resolved: false }
    }

    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn terminal_handle(&self) -> ArtifactHistoryTerminalHandle {
        ArtifactHistoryTerminalHandle { state: self.state.clone() }
    }

    pub fn progress(&self) -> HistoryProgress {
        match self.state.progress.load(std::sync::atomic::Ordering::Acquire) {
            0 => HistoryProgress::Admitted,
            1 => HistoryProgress::Scheduled,
            2 => HistoryProgress::Waiting,
            3 => HistoryProgress::Mapping,
            4 => HistoryProgress::Completed,
            5 => HistoryProgress::Cancelled,
            _ => HistoryProgress::Fault,
        }
    }

    pub fn cancel(&self) {
        if !matches!(self.progress(), HistoryProgress::Completed | HistoryProgress::Cancelled | HistoryProgress::Fault) {
            self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            if !self.state.terminalize_unhanded_request(Err(DbError::Closed), HistoryProgress::Cancelled) {
                self.state.schedule();
            }
        }
    }

    pub fn take_terminal_job(&self) -> Option<ArtifactHistoryTerminalJob> {
        self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactHistoryTerminalJob { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_work(&self) -> Option<ArtifactHistoryTerminalWork> {
        self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactHistoryTerminalWork { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_reservation(&self) -> Option<ArtifactHistoryTerminalReservation> {
        self.state.take_terminal_reservation()
    }

    pub fn take_terminal_construction_fault(&self) -> Option<ArtifactHistoryTerminalConstructionFault> {
        self.state.take_terminal_construction_fault()
    }

    pub fn take_terminal_result(&self) -> Option<ArtifactHistoryOutcome> {
        let mut result = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(Ok(view)) = result.as_mut() {
            view.rearm_terminal_return();
        }
        if result.is_some() {
            self.state.finish_if_terminal_empty();
        }
        result
    }

    pub fn take_actor_terminal_job(&self) -> Option<db_artifact::ArtifactRunnerTerminalJob> {
        self.state.authority.take_terminal_job()
    }

    pub fn close_step(&self) -> bool {
        if self.state.close_one() {
            return true;
        }
        if self.state.authority.close_step() {
            return true;
        }
        self.state.finish_if_terminal_empty()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty() && self.state.authority.terminal_is_empty()
    }
}

impl ArtifactHistoryTerminalHandle {
    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn take_terminal_job(&self) -> Option<ArtifactHistoryTerminalJob> {
        self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactHistoryTerminalJob { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_work(&self) -> Option<ArtifactHistoryTerminalWork> {
        self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactHistoryTerminalWork { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_reservation(&self) -> Option<ArtifactHistoryTerminalReservation> {
        self.state.take_terminal_reservation()
    }

    pub fn take_terminal_construction_fault(&self) -> Option<ArtifactHistoryTerminalConstructionFault> {
        self.state.take_terminal_construction_fault()
    }

    pub fn take_terminal_result(&self) -> Option<ArtifactHistoryOutcome> {
        let mut result = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(Ok(view)) = result.as_mut() {
            view.rearm_terminal_return();
        }
        if result.is_some() {
            self.state.finish_if_terminal_empty();
        }
        result
    }

    pub fn take_actor_terminal_job(&self) -> Option<db_artifact::ArtifactRunnerTerminalJob> {
        self.state.authority.take_terminal_job()
    }

    pub fn close_step(&self) -> bool {
        if self.state.close_one() {
            return true;
        }
        if self.state.authority.close_step() {
            return true;
        }
        self.state.finish_if_terminal_empty()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty() && self.state.authority.terminal_is_empty()
    }
}

impl Future for HistoryFuture {
    type Output = ArtifactHistoryOutcome;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if let Some(result) = self.take_completion() {
            return std::task::Poll::Ready(result);
        }
        #[cfg(test)]
        {
            let hook = self.state.controlled_publication_before_waker_hook.lock().unwrap().take();
            if let Some(hook) = hook {
                hook();
            }
        }
        *self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        self.take_completion().map_or(std::task::Poll::Pending, std::task::Poll::Ready)
    }
}

impl Drop for HistoryFuture {
    fn drop(&mut self) {
        if !self.resolved {
            let mut completion = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
            self.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            if let Some(result) = completion.take() {
                *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
            }
            drop(completion);
            self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            if !self.state.terminalize_unhanded_request(Err(DbError::Closed), HistoryProgress::Cancelled) {
                self.state.schedule();
            }
        }
        self.state.finish_if_terminal_empty();
    }
}

impl ArtifactHistoryTerminalJob {
    pub fn reason(&self) -> semio_framework_async::WorkerSubmitErrorKind {
        self.owner.as_ref().expect("terminal artifact history job already resolved").0
    }

    pub fn resume(mut self) {
        let request = self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).as_ref().is_some_and(|owner| matches!(owner, ArtifactHistoryWorkOwner::Request));
        if request && !self.state.resume_unhanded_reservation() {
            return;
        }
        let (_, job) = self.owner.take().expect("terminal artifact history job already resolved");
        if self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            let work = self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = work;
        }
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.cancelled.store(false, std::sync::atomic::Ordering::Release);
        self.state.scheduled.store(true, std::sync::atomic::Ordering::Release);
        self.state.submit_exact(job, 0);
    }

    pub fn close(mut self) {
        drop(self.owner.take().expect("terminal artifact history job already resolved"));
        self.state.finish_if_terminal_empty();
    }
}

impl Drop for ArtifactHistoryTerminalJob {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
    }
}

impl ArtifactHistoryTerminalWork {
    pub fn resume(mut self) -> Result<(), Self> {
        if self.state.generation == 0 {
            return Err(self);
        }
        if self.owner.as_ref().is_some_and(|owner| matches!(owner, ArtifactHistoryWorkOwner::Request)) && !self.state.resume_unhanded_reservation() {
            return Err(self);
        }
        let owner = self.owner.take().expect("terminal artifact history work already resolved");
        *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.cancelled.store(false, std::sync::atomic::Ordering::Release);
        self.state.set_progress(HistoryProgress::Admitted);
        if self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.state.schedule();
        }
        Ok(())
    }

    pub fn close(mut self) {
        let owner = self.owner.take().expect("terminal artifact history work already resolved");
        match owner {
            ArtifactHistoryWorkOwner::Request => {
                self.state.begin_unhanded_reservation_close();
                self.state.finish_if_terminal_empty();
            }
            ArtifactHistoryWorkOwner::Actor(owner) => {
                *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ArtifactHistoryWorkOwner::Actor(owner));
                self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
                self.state.schedule();
            }
        }
    }
}

impl ArtifactHistoryTerminalReservation {
    pub fn resume(mut self) -> Result<(), Self> {
        let mut admission = self.state.admission.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if admission.as_ref().is_none_or(|owner| owner.reservation.is_some()) {
            drop(admission);
            return Err(self);
        }
        let Some(cursor) = self.owner.as_mut() else {
            drop(admission);
            return Err(self);
        };
        let Some(reservation) = cursor.resume() else {
            drop(admission);
            return Err(self);
        };
        let admission = admission.as_mut().expect("artifact history admission changed after reservation preflight");
        admission.restore_reservation(reservation);
        self.owner = None;
        self.state.reservation_checked_out.store(false, std::sync::atomic::Ordering::Release);
        Ok(())
    }

    pub fn close_step(&mut self) -> bool {
        let Some(cursor) = self.owner.as_mut() else {
            return false;
        };
        if cursor.close_step() {
            return true;
        }
        if cursor.terminal_is_empty() {
            self.owner = None;
            self.state.reservation_checked_out.store(false, std::sync::atomic::Ordering::Release);
            self.state.finish_if_terminal_empty();
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.owner.as_ref().is_none_or(db_artifact_history::HistoryReplayReservationCloseCursor::terminal_is_empty)
    }
}

impl Drop for ArtifactHistoryTerminalReservation {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_reservation.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
        self.state.reservation_checked_out.store(false, std::sync::atomic::Ordering::Release);
    }
}

impl ArtifactHistoryTerminalConstructionFault {
    pub fn resume(mut self) -> Result<(), Self> {
        let mut terminal = self.state.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if terminal.is_some() {
            drop(terminal);
            return Err(self);
        }
        *terminal = self.owner.take();
        drop(terminal);
        self.state.construction_checked_out.store(false, std::sync::atomic::Ordering::Release);
        Ok(())
    }

    pub fn close_step(&mut self) -> bool {
        let Some(cursor) = self.owner.as_mut() else {
            return false;
        };
        if cursor.close_step() {
            return true;
        }
        if cursor.terminal_is_empty() {
            self.owner = None;
            self.state.construction_checked_out.store(false, std::sync::atomic::Ordering::Release);
            return true;
        }
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.owner.as_ref().is_none_or(db_artifact_history::HistoryReplayReservationConstructionFault::terminal_is_empty)
    }
}

impl Drop for ArtifactHistoryTerminalConstructionFault {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_construction.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
        self.state.construction_checked_out.store(false, std::sync::atomic::Ordering::Release);
    }
}

impl Drop for ArtifactHistoryTerminalWork {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
    }
}
//#endregion 🔖️ArtifactHandle
