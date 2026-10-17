//! 📊️ Progress records of the codec ABI: phases, step states, steps, the cancel outcome, the progress counter and the pending input page. See [`super`].

use super::*;

//#region ⏳️RetainedOperation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OsHostCodecPhase {
    Input = 1,
    Decode = 2,
    Output = 3,
    AwaitingAcknowledgement = 4,
    Reply = 5,
    Closing = 6,
    Closed = 7,
    Cancelled = 8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OsHostCodecStepState {
    Progress,
    InputAcknowledged,
    OutputPage,
    AwaitingAcknowledgement,
    Reply,
    Idle,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OsHostCodecStep {
    pub state: OsHostCodecStepState,
    pub event: AbiEvent,
    pub input_acknowledgement: Option<AbiControl>,
    pub page: Option<AbiPage>,
    pub reply: Option<AbiReply>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OsHostCodecCancelOutcome {
    pub page: Option<AbiPage>,
    pub admitted_byte_credits: usize,
    pub copied_bytes: usize,
}

#[derive(Clone, Copy)]
pub(super) struct OsHostCodecProgress {
    pub(super) phase: OsHostCodecPhase,
    pub(super) completed: usize,
    pub(super) total: usize,
}

pub(super) struct PendingInputPage {
    pub(super) page: AbiPage,
    pub(super) cursor: usize,
    pub(super) retire_cursor: usize,
}
//#endregion ⏳️RetainedOperation
