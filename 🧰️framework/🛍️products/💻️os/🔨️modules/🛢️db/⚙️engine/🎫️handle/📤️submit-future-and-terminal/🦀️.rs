//! 📤️ Submit future and terminal: the submit future and its terminal job and work. See [`super`].

use super::*;

//#region 🔖️ArtifactHandle
impl SubmitFuture {
    pub fn submit(handle: &ArtifactHandle, batch: db_artifact_contract::CommandBatch, options: db_artifact_contract::SubmitOptions) -> Self {
        let credit = artifact_submit_credit(&batch).and_then(|(items, bytes)| ArtifactSubmitAdmission::try_claim(items, bytes));
        let admission_error = credit.as_ref().err().cloned();
        let generation = credit.as_ref().map_or(0, |admission| admission.generation);
        let request = ArtifactSubmitWorkOwner::Request { batch, options, submitted_at_ms: handle.pool.now_ms() };
        let (work, terminal_work) = if generation == 0 { (None, Some(request)) } else { (Some(request), None) };
        let state = Arc::new(ArtifactSubmitState {
            pool: handle.pool.as_ref().clone(),
            authority: Mutex::new(Some(handle.authority.clone())),
            document: handle.document.clone(),
            generation,
            authority_generation: handle.authority.generation(),
            admission: Mutex::new(credit.ok()),
            work: Mutex::new(work),
            completion: Mutex::new(None),
            terminal_work: Mutex::new(terminal_work),
            terminal_result: Mutex::new(None),
            retry_job: Mutex::new(None),
            terminal_job: Mutex::new(None),
            waker: Mutex::new(None),
            retry_armed: std::sync::atomic::AtomicBool::new(false),
            retry_generation: std::sync::atomic::AtomicU64::new(1),
            scheduled: std::sync::atomic::AtomicBool::new(false),
            cancelled: std::sync::atomic::AtomicBool::new(false),
            abandoned: std::sync::atomic::AtomicBool::new(false),
            finished: std::sync::atomic::AtomicBool::new(false),
            progress: std::sync::atomic::AtomicU8::new(if generation == 0 { SubmitProgress::Fault as u8 } else { SubmitProgress::Admitted as u8 }),
        });
        if generation == 0 {
            *state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(Err(admission_error.unwrap_or_else(|| DbError::Unavailable("artifact submit admission exhausted".to_string()))));
        } else {
            state.schedule();
        }
        Self { state, resolved: false }
    }

    pub fn generation(&self) -> u64 {
        self.state.generation
    }

    pub fn progress(&self) -> SubmitProgress {
        match self.state.progress.load(std::sync::atomic::Ordering::Acquire) {
            0 => SubmitProgress::Admitted,
            1 => SubmitProgress::Scheduled,
            2 => SubmitProgress::Waiting,
            3 => SubmitProgress::Completed,
            4 => SubmitProgress::Cancelled,
            _ => SubmitProgress::Fault,
        }
    }

    pub fn cancel(&self) {
        if matches!(self.progress(), SubmitProgress::Completed | SubmitProgress::Cancelled | SubmitProgress::Fault) {
            return;
        }
        self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
        self.state.schedule();
    }

    pub fn take_terminal_job(&self) -> Option<ArtifactSubmitTerminalJob> {
        self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactSubmitTerminalJob { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_work(&self) -> Option<ArtifactSubmitTerminalWork> {
        self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().map(|owner| ArtifactSubmitTerminalWork { state: self.state.clone(), owner: Some(owner) })
    }

    pub fn take_terminal_result(&self) -> Option<ArtifactSubmitOutcome> {
        let result = self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if result.is_some() {
            self.state.finish_if_terminal_empty();
        }
        result
    }

    pub fn take_actor_terminal_job(&self) -> Option<db_artifact::ArtifactRunnerTerminalJob> {
        self.state.authority().and_then(|authority| authority.take_terminal_job())
    }

    pub fn close_step(&self) -> bool {
        let progressed = self.state.close_one() || self.state.authority().is_some_and(|authority| authority.close_step());
        self.state.finish_if_terminal_empty();
        progressed
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.state.terminal_is_empty() && self.state.authority().is_none_or(|authority| authority.terminal_is_empty())
    }
}

impl Future for SubmitFuture {
    type Output = ArtifactSubmitOutcome;

    fn poll(mut self: std::pin::Pin<&mut Self>, context: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let result = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(result) = result {
            self.resolved = true;
            self.state.finish_if_terminal_empty();
            return std::task::Poll::Ready(result);
        }
        *self.state.waker.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(context.waker().clone());
        let result = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
        if let Some(result) = result {
            self.resolved = true;
            self.state.finish_if_terminal_empty();
            return std::task::Poll::Ready(result);
        }
        std::task::Poll::Pending
    }
}

impl Drop for SubmitFuture {
    fn drop(&mut self) {
        if !self.resolved {
            self.state.abandoned.store(true, std::sync::atomic::Ordering::Release);
            self.state.cancelled.store(true, std::sync::atomic::Ordering::Release);
            self.state.schedule();
        }
        self.state.close_one();
        self.state.finish_if_terminal_empty();
    }
}

impl ArtifactSubmitTerminalJob {
    pub fn reason(&self) -> semio_framework_async::WorkerSubmitErrorKind {
        self.owner.as_ref().expect("terminal artifact submit job already resolved").0
    }

    pub fn resume(mut self) {
        let (_, job) = self.owner.take().expect("terminal artifact submit job already resolved");
        if self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            let work = self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take();
            *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = work;
        }
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.cancelled.store(false, std::sync::atomic::Ordering::Release);
        self.state.set_progress(SubmitProgress::Scheduled);
        self.state.scheduled.store(true, std::sync::atomic::Ordering::Release);
        self.state.submit_exact(job, 0);
    }

    pub fn close(mut self) {
        let (_, job) = self.owner.take().expect("terminal artifact submit job already resolved");
        drop(job);
        self.state.finish_if_terminal_empty();
    }
}

impl Drop for ArtifactSubmitTerminalJob {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
    }
}

impl ArtifactSubmitTerminalWork {
    pub fn resume(mut self) -> Result<(), Self> {
        if self.state.generation == 0 {
            return Err(self);
        }
        let owner = self.owner.take().expect("terminal artifact submit work already resolved");
        *self.state.work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        if let Some(result) = self.state.completion.lock().unwrap_or_else(std::sync::PoisonError::into_inner).take() {
            *self.state.terminal_result.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(result);
        }
        self.state.cancelled.store(false, std::sync::atomic::Ordering::Release);
        self.state.set_progress(SubmitProgress::Admitted);
        if self.state.terminal_job.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none() {
            self.state.schedule();
        }
        Ok(())
    }

    pub fn close(mut self) {
        drop(self.owner.take().expect("terminal artifact submit work already resolved"));
        self.state.finish_if_terminal_empty();
    }
}

impl Drop for ArtifactSubmitTerminalWork {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            *self.state.terminal_work.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(owner);
        }
    }
}
//#endregion 🔖️ArtifactHandle
