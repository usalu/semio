//! 🚚️ Document-archive load: the load state and status wire types, and the `DocumentArchiveLoadHost` step machine with its outcomes.

use super::{AppCommand, AppFrame, DocumentArchivePack, malformed};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentArchiveLoadState {
    Pending,
    Running,
    Ready,
    Cancelled,
    Fault,
}

impl DocumentArchiveLoadState {
    pub(super) fn wire(self) -> u8 {
        match self {
            Self::Pending => 0,
            Self::Running => 1,
            Self::Ready => 2,
            Self::Cancelled => 3,
            Self::Fault => 4,
        }
    }

    pub(super) fn from_wire(value: u8) -> Result<Self, protocol::ProtocolError> {
        match value {
            0 => Ok(Self::Pending),
            1 => Ok(Self::Running),
            2 => Ok(Self::Ready),
            3 => Ok(Self::Cancelled),
            4 => Ok(Self::Fault),
            _ => Err(malformed("document archive load state", 0, "unknown state")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentArchiveLoadStatus {
    pub operation: u64,
    pub state: DocumentArchiveLoadState,
    pub completed: u64,
    pub total: u64,
    /// ⏭️ Events this program holds that the archive lacks: 0 for a replacement, the merge's own count otherwise.
    pub ahead: u64,
    pub fault: Vec<u8>,
}
//#region 🔖️DocumentArchiveLoadHost
/// 🏁️ How one host-driven whole-document load ended (`📓️api-stepped-document-load.md` §2.3, ticket
/// 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING). `Cancelled` and `Fault` both left the previous document exactly as it was;
/// `Fault` carries the guest's encoded fault.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentArchiveLoadOutcome {
    Ready,
    Cancelled,
    Fault(Vec<u8>),
}

/// 🧭️ What a host does next with one [`DocumentArchiveLoadHost`]: send this command, stamped with the sequence the host minted
/// for it, or stop, because the load reached its acknowledged terminal or was cancelled before it was ever admitted.
#[derive(Debug, PartialEq)]
pub enum DocumentArchiveLoadStep {
    Send { seq: u64, command: AppCommand },
    Finished(DocumentArchiveLoadOutcome),
}

/// 🚧️ An answer a load cannot go on from: the guest refused the admission, a poll or an acknowledgement (its encoded fault), or the
/// frame handed back answers some other command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DocumentArchiveLoadRefusal {
    Refused(Vec<u8>),
    Unanswered,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DocumentArchiveLoadSent {
    Admit,
    Poll,
    Cancel,
    Acknowledge,
}

/// 🗃️ The host half of the stepped, ACK-owned whole-document load (`📓️api-stepped-document-load.md` §2): a transport-free state
/// machine every Rust host drives with its own exchange — the MCP gateway, the `🏃️run` batch runner, the wgpu bridge — so
/// admit → poll* → (cancel → poll*) → acknowledge exists once. TS twin: `AppChannelClient.loadDocumentArchive` (`💻️os/🟦️.ts`).
/// Corpus: `🧫️fixtures/🧫️document-archive-load-host/🔣️.json` (schema `🧬️schema/🔣️document-archive-load-host/🔣️.json`).
///
/// 🔢️ The operation is the admission's own sequence, which is how the guest keys the load. A host's sequence is minted only for
/// a command that is sent, never for the step that finishes. Steps and answers alternate: every [`Self::step`] that sends is
/// answered by exactly one [`Self::answer`] under the same sequence.
///
/// 🛑️ A cancel requested before admission sends nothing and finishes `Cancelled`. After admission it sends
/// `CancelDocumentArchiveLoad` once and keeps polling: only the guest's terminal status says whether the previous document was
/// restored or the load won the race (a refused cancel is that race, and the next poll answers it). A cancelled terminal whose
/// acknowledgement is refused still owns retained input, so it is polled again before the next acknowledgement.
pub struct DocumentArchiveLoadHost {
    pub(super) archive: Option<DocumentArchivePack>,
    pub(super) merge: bool,
    pub(super) ahead: u64,
    pub(super) operation: Option<u64>,
    sent: Option<(u64, DocumentArchiveLoadSent)>,
    cancel_requested: bool,
    cancel_sent: bool,
    pub(super) terminal: Option<DocumentArchiveLoadStatus>,
    outcome: Option<DocumentArchiveLoadOutcome>,
}

impl DocumentArchiveLoadHost {
    pub fn new(archive: DocumentArchivePack) -> Self {
        Self { archive: Some(archive), merge: false, ahead: 0, operation: None, sent: None, cancel_requested: false, cancel_sent: false, terminal: None, outcome: None }
    }

    /// 🔀️ A read-back merge of the document the program already shows (design §22.22): admitted with
    /// `AppCommand::MergeDocumentArchive`, then polled, cancelled and acknowledged exactly like a load. Its events join the
    /// program's log and nothing is replaced; [`Self::ahead`] answers what the program then holds that the archive lacks.
    pub fn merging(archive: DocumentArchivePack) -> Self {
        Self { merge: true, ..Self::new(archive) }
    }

    /// 🎞️ A load the guest admitted itself under `operation`, the host sequence of the command that started it: a `MediaIn` of a whole
    /// document of the app's own schema answers `AppFrame::DocumentArchiveLoad` (pending) for its own sequence
    /// (`📓️api-stepped-document-load.md` §4). Driven from its first poll exactly like an admission this driver sent.
    pub fn admitted(operation: u64) -> Self {
        Self { archive: None, merge: false, ahead: 0, operation: Some(operation), sent: None, cancel_requested: false, cancel_sent: false, terminal: None, outcome: None }
    }

    /// 🔢️ The admitted operation, once the admission was sent.
    pub fn operation(&self) -> Option<u64> {
        self.operation
    }

    /// ⏭️ The events the program holds that the archive lacks, as its acknowledged `Ready` terminal counted them: what a
    /// writer persists again for. 0 for a replacement and before the operation ended ready.
    pub fn ahead(&self) -> u64 {
        self.ahead
    }

    /// 🛑️ Asks for the load to stop; idempotent, and a no-op once a terminal status is known.
    pub fn request_cancel(&mut self) {
        self.cancel_requested = true;
        if self.operation.is_none() && self.outcome.is_none() {
            self.archive = None;
            self.outcome = Some(DocumentArchiveLoadOutcome::Cancelled);
        }
    }

    /// ➡️ The next command, stamped with a sequence from `next_seq`, or the outcome once the load is over.
    pub fn step(&mut self, next_seq: impl FnOnce() -> u64) -> DocumentArchiveLoadStep {
        if let Some(outcome) = &self.outcome {
            return DocumentArchiveLoadStep::Finished(outcome.clone());
        }
        let seq = next_seq();
        let (sent, command) = match (self.operation, &self.terminal) {
            (None, _) if self.merge => (DocumentArchiveLoadSent::Admit, AppCommand::MergeDocumentArchive { seq, archive: self.archive.take().unwrap_or_default() }),
            (None, _) => (DocumentArchiveLoadSent::Admit, AppCommand::LoadDocumentArchive { seq, archive: self.archive.take().unwrap_or_default() }),
            (Some(operation), Some(_)) => (DocumentArchiveLoadSent::Acknowledge, AppCommand::AcknowledgeDocumentArchiveLoad { seq, operation }),
            (Some(operation), None) if self.cancel_requested && !self.cancel_sent => (DocumentArchiveLoadSent::Cancel, AppCommand::CancelDocumentArchiveLoad { seq, operation }),
            (Some(operation), None) => (DocumentArchiveLoadSent::Poll, AppCommand::PollDocumentArchiveLoad { seq, operation }),
        };
        if sent == DocumentArchiveLoadSent::Admit {
            self.operation = Some(seq);
        }
        self.sent = Some((seq, sent));
        DocumentArchiveLoadStep::Send { seq, command }
    }

    /// 📬️ Takes the guest's answer to the command sent under `seq`; a poll answers its status for the host's progress.
    pub fn answer(&mut self, seq: u64, frame: &AppFrame) -> Result<Option<DocumentArchiveLoadStatus>, DocumentArchiveLoadRefusal> {
        let (sent, operation) = match (self.sent.take(), self.operation) {
            (Some((sent_seq, sent)), Some(operation)) if sent_seq == seq => (sent, operation),
            _ => return Err(DocumentArchiveLoadRefusal::Unanswered),
        };
        match (sent, frame) {
            (DocumentArchiveLoadSent::Admit, AppFrame::Done { in_reply_to }) if *in_reply_to == seq => Ok(None),
            (DocumentArchiveLoadSent::Cancel, AppFrame::Done { in_reply_to }) if *in_reply_to == seq => {
                self.cancel_sent = true;
                Ok(None)
            }
            (DocumentArchiveLoadSent::Poll, AppFrame::DocumentArchiveLoad { in_reply_to, status }) if *in_reply_to == seq && status.operation == operation => {
                if matches!(status.state, DocumentArchiveLoadState::Ready | DocumentArchiveLoadState::Cancelled | DocumentArchiveLoadState::Fault) {
                    self.terminal = Some(status.clone());
                }
                Ok(Some(status.clone()))
            }
            (DocumentArchiveLoadSent::Acknowledge, AppFrame::Done { in_reply_to }) if *in_reply_to == seq => {
                let terminal = self.terminal.take().ok_or(DocumentArchiveLoadRefusal::Unanswered)?;
                if terminal.state == DocumentArchiveLoadState::Ready {
                    self.ahead = terminal.ahead;
                }
                self.outcome = Some(match terminal.state {
                    DocumentArchiveLoadState::Ready => DocumentArchiveLoadOutcome::Ready,
                    DocumentArchiveLoadState::Cancelled => DocumentArchiveLoadOutcome::Cancelled,
                    _ => DocumentArchiveLoadOutcome::Fault(terminal.fault),
                });
                Ok(None)
            }
            (DocumentArchiveLoadSent::Cancel, AppFrame::Error { in_reply_to: Some(reply), .. }) if *reply == seq => {
                self.cancel_sent = true;
                Ok(None)
            }
            (DocumentArchiveLoadSent::Acknowledge, AppFrame::Error { in_reply_to: Some(reply), .. }) if *reply == seq && self.terminal.as_ref().is_some_and(|terminal| terminal.state == DocumentArchiveLoadState::Cancelled) => {
                self.terminal = None;
                Ok(None)
            }
            (_, AppFrame::Error { in_reply_to: Some(reply), fault, .. }) if *reply == seq => Err(DocumentArchiveLoadRefusal::Refused(fault.clone())),
            _ => Err(DocumentArchiveLoadRefusal::Unanswered),
        }
    }
}
//#endregion 🔖️DocumentArchiveLoadHost
