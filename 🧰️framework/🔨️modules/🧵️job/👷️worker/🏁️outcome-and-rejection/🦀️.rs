//! 🏁️ Outcome and rejection: the worker job outcome and rejection records with their drops and the rejection code and kind mapping. See [`super`].

use super::*;

//#region 🏭️RetainedSessions
pub struct WorkerJobOutcome<J> {
    pub(super) inner: session_return::SessionHandle<J>,
    pub(super) authority: Option<OriginalWorkerJobAuthority<J>>,
    pub(super) restore_phase: u8,
}

impl<J> WorkerJobOutcome<J> {
    /// 📥️ The receiving caller consumes the same actual turn receipt exactly once before semantic access.
    /// 🧾️ The original checked-out authority retains both borrowed fields until its unique recipient accepts them.
    pub fn retained_step_receipt(&self)->Option<(&RetainedCloneGrant,&RetainedCloneProgress)>{self.authority.as_ref().and_then(OriginalWorkerJobAuthority::retained_step_receipt)}

    pub fn take_retained_step_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.authority.as_mut().and_then(OriginalWorkerJobAuthority::take_retained_step_receipt)}

    /// 🧾️ Returns the exact latest actual worker turn receipt without changing its outcome owner.
    pub fn retained_step_progress(&self)->RetainedCloneProgress{self.authority.as_ref().expect("original outcome retains its authority").retained_step_progress()}
    /// ↩️ Returns the actual physical receipt accumulated by this original worker authority.
    pub fn retained_progress(&self)->RetainedCloneProgress{self.authority.as_ref().expect("original outcome retains its authority").retained_progress()}

    pub fn callback_verdict(&self) -> Option<&semio_framework_trace::CallbackVerdict> {
        self.authority.as_ref().and_then(OriginalWorkerJobAuthority::callback_verdict)
    }

    /// 📒️ This session's own overrun ledger — how many of its steps breached the interactive ceiling,
    /// the longest consecutive run, and the worst reading, for a host that wants the counters without
    /// waiting for a quarantine.
    pub fn overrun_ledger(&self) -> Option<&semio_framework_trace::StepOverrunLedger> {
        self.authority.as_ref().map(OriginalWorkerJobAuthority::overrun_ledger)
    }

    pub fn job(&self) -> &J {
        self.authority.as_ref().expect("checked-out worker outcome owns exact job").job()
    }

    pub fn job_mut(&mut self)->Result<&mut J,ValueError>{self.authority.as_mut().expect("original outcome retains its authority").job_mut()}
    pub fn retained_failure(&self)->Option<&OriginalTurnFailure>{self.authority.as_ref().and_then(OriginalWorkerJobAuthority::retained_failure)}
    /// 🪪️ Borrows original semantic metadata while its payload remains in this worker authority.
    pub fn outcome_descriptor(&self)->Option<&JobOutcomeDescriptor>{self.authority.as_ref().and_then(OriginalWorkerJobAuthority::outcome_descriptor)}
    pub fn outcome(&self)->Result<Option<JobOutcomeView<'_>>,ValueError>where J:InteractiveJob{self.authority.as_ref().expect("original outcome retains its authority").outcome()}
    pub fn acknowledge_outcome(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{self.authority.as_mut().expect("original outcome retains its authority").acknowledge_outcome(grant)}

    #[expect(clippy::result_large_err, reason = "Refused resumption returns the checked-out job authority and retained outcome without allocation or ownership loss.")]
    pub fn resume(mut self) -> Result<(), Self> {
        if self.restore_phase == SESSION_TERMINAL {
            return Err(self);
        }
        let authority = self.authority.as_ref().expect("checked-out worker outcome owns authority");
        if !authority.is_resumable() {
            return Err(self);
        }
        let authority = self.authority.take().expect("checked-out worker outcome owns authority");
        unsafe { self.inner.put_authority(authority, SESSION_IDLE) };
        Ok(())
    }

    pub fn begin_close(mut self) {
        let authority = self.authority.take().expect("checked-out worker outcome owns authority");
        unsafe { self.inner.put_authority(authority, SESSION_CLOSE) };
    }
}

impl<J> Drop for WorkerJobOutcome<J> {
    fn drop(&mut self) {
        if let Some(authority) = self.authority.take() {
            unsafe { self.inner.put_authority(authority, self.restore_phase) };
        }
    }
}

pub struct WorkerJobRejected<J> {
    pub(super) inner: session_return::SessionHandle<J>,
    pub(super) authority: Option<OriginalWorkerJobAuthority<J>>,
    pub(super) kind: semio_framework_async::WorkerSubmitErrorKind,
}

impl<J> WorkerJobRejected<J> {
    pub fn kind(&self) -> semio_framework_async::WorkerSubmitErrorKind {
        self.kind
    }

    pub fn job(&self) -> &J {
        self.authority.as_ref().expect("checked-out rejected worker owner remains exact").job()
    }

    pub fn resume(mut self) {
        let authority = self.authority.take().expect("checked-out rejected worker owner remains exact");
        self.inner.rejection_kind.store(u8::MAX, Ordering::Release);
        unsafe { self.inner.put_authority(authority, SESSION_IDLE) };
    }

    pub fn begin_close(mut self) {
        let authority = self.authority.take().expect("checked-out rejected worker owner remains exact");
        unsafe { self.inner.put_authority(authority, SESSION_CLOSE) };
    }
}

impl<J> Drop for WorkerJobRejected<J> {
    fn drop(&mut self) {
        if let Some(authority) = self.authority.take() {
            unsafe { self.inner.put_authority(authority, SESSION_REJECTED) };
        }
    }
}

pub(super) fn worker_rejection_code(kind: semio_framework_async::WorkerSubmitErrorKind) -> u8 {
    match kind {
        semio_framework_async::WorkerSubmitErrorKind::Shutdown => 0,
        semio_framework_async::WorkerSubmitErrorKind::Contended => 1,
        semio_framework_async::WorkerSubmitErrorKind::Poisoned => 2,
        semio_framework_async::WorkerSubmitErrorKind::Saturated => 3,
    }
}

pub(super) fn worker_rejection_kind(code: u8) -> semio_framework_async::WorkerSubmitErrorKind {
    match code {
        0 => semio_framework_async::WorkerSubmitErrorKind::Shutdown,
        1 => semio_framework_async::WorkerSubmitErrorKind::Contended,
        2 => semio_framework_async::WorkerSubmitErrorKind::Poisoned,
        _ => semio_framework_async::WorkerSubmitErrorKind::Saturated,
    }
}
//#endregion 🏭️RetainedSessions
