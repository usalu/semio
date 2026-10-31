//! 🎫️ Worker job vocabulary: the worker job ticket, contention, demand, submit, poll, take and close phase records, the batch session and the mounted session pump fault. See [`super`].

use super::*;

//#region 🏭️RetainedSessions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorkerJobTicket {
    pub generation: Generation,
    pub step_sequence: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobContention {
    Submitted(WorkerJobTicket),
    Outcome(WorkerJobTicket),
    Terminal(WorkerJobTicket),
    Rejected(Generation),
    CheckedOut(Generation),
    Closing(Generation),
    WakeExhausted(Generation),
    TerminalEmpty,
}

/// 🧭️ Keeps contention separate from a refused original ownership demand.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum WorkerJobDemandError {Contention(WorkerJobContention),Refused(ValueError)}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobSubmitFault {
    Contention(WorkerJobContention),
    Pool(semio_framework_async::WorkerSubmitErrorKind),
    SequenceExhausted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobPoll {
    Idle,
    Submitted,
    Outcome,
    Terminal,
    Rejected,
    CheckedOut,
    Closing,
    TerminalEmpty,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobTakeFault {
    Pending,
    Stale,
    WrongPhase,
}

/// 🔭️ Which named phase a worker session's bounded close cursor is parked in, read without
/// spending a turn.
///
/// `close_step` reports HOW MUCH a turn released, never WHERE the cursor is, so a close that
/// stops advancing is indistinguishable from one that is merely slow: both answer
/// `Pending { 0, 0 }`. A host draining a session on a short grant, and every law that asserts a
/// session reaches terminal, needs the phase name to tell "the job is still releasing its own
/// owners" from "the cursor cannot leave the pre-admitted fault page" — the two that looked
/// identical while the mounted presence capture law spun 4096 times (ticket 26/09/18).
///
/// The phases are the exact ladder [`WorkerJobSession::close_step`] walks, in order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkerJobClosePhase {
    OriginalFailure,
    /// 🚦️ Not closing: the session is idle, submitted, or holding an outcome nobody closed.
    Open,
    /// 🤝️ The authority is checked out, so the cursor has not been handed the session yet.
    CheckedOut,
    /// 📦️ Releasing the checked-out step outcome's retained payload pages.
    Outcome,
    /// 🔔️ The turn that hands `begin_close` to the job itself.
    BeginClose,
    /// 🧨️ Releasing the session's prepared fault payload and its actual source receiver.
    Payload,
    /// 🧩️ The job is releasing its own owners through `InteractiveJob::close_step`.
    Job,
    /// 🗑️ Dropping the released job.
    JobRelease,
    /// 🎛️ Dropping the batch parameters.
    ParamsRelease,
    /// 🧺️ Releasing the original worker authority backing.
    AuthorityRelease,
    /// 🎟️ Returning the pre-admitted retirement slot.
    RetirementSlot,
    SessionArc,
    /// 🕳️ Terminal-empty: every owner is released and the slot is returned.
    Empty,
}

pub struct BatchJobSession<J: InteractiveJob + 'static> {
    pub(super) session: WorkerJobSession<J>,
    pub(super) ticket: Option<WorkerJobTicket>,
    pub(super) checked_out: Option<WorkerJobOutcome<J>>,
}

pub enum MountedWorkerJobPumpFault {
    Submit(WorkerJobSubmitFault),
    Take(WorkerJobTakeFault),
    MissingTicket,
    CheckedOut,
}

pub struct MountedWorkerJobSession<J: InteractiveJob + 'static> {
    pub(super) session: WorkerJobSession<J>,
    pub(super) ticket: Option<WorkerJobTicket>,
    pub(super) checked_out: Option<WorkerJobOutcome<J>>,
}
//#endregion 🏭️RetainedSessions
