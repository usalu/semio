//! ✂️ Cooperative message selection and owned retirement for one replayed edit.
use super::{ArtifactStoreMessageLedgerRetirement, ErasedSnapshotRetirement, SnapshotRetirementStep, ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES, ARTIFACT_EDIT_MESSAGE_OWNER_BYTES, ARTIFACT_EDIT_MESSAGE_TARGET_BYTES};
use crate::os_spr::MutationMessage;
use semio_framework_diagnostic::Severity;

#[path = "🔁️settlement/🦀️.rs"]
pub(crate) mod settlement;

enum Phase { Measure, Rank, Drain, Restore, Summary, Finish, Finished }

pub(super) struct EditMessageClamp {
    phase: Phase,
    retained: Vec<MutationMessage>,
    active: Option<Box<dyn ErasedSnapshotRetirement>>,
    index: usize,
    target: usize,
    measuring: Option<usize>,
    total: usize,
    original_len: usize,
    budget: usize,
    used: usize,
    level: usize,
    threshold: Option<(Severity, usize)>,
    worst: Option<(Severity, usize)>,
    first_dropped: Option<Option<u32>>,
    kept: usize,
    changed: bool,
    work: u64,
}

impl EditMessageClamp {
    pub(super) fn new(edit_id: &str, messages: &[MutationMessage]) -> Self {
        let summary_bytes = ARTIFACT_EDIT_MESSAGE_OWNER_BYTES + "mutation.cascade".len() + format!("{} more messages", messages.len()).len();
        Self {
            phase: Phase::Measure, retained: Vec::with_capacity(ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES / ARTIFACT_EDIT_MESSAGE_OWNER_BYTES), active: None,
            index: 0, target: 0, measuring: None, total: edit_id.len(), original_len: messages.len(),
            budget: ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES.saturating_sub(edit_id.len()).saturating_sub(summary_bytes),
            used: 0, level: 0, threshold: None, worst: None, first_dropped: None, kept: 0, changed: false, work: 0,
        }
    }

    pub(super) fn changed(&self) -> bool { self.changed }
    pub(super) fn completed_work(&self) -> u64 { self.work }
    pub(super) fn is_finished(&self) -> bool { matches!(self.phase, Phase::Finished) }

    /// ⏳️ Executes one row, target segment or granted retirement step.
    pub(super) fn step(&mut self, messages: &mut Vec<MutationMessage>, maximum_bytes: usize) -> Result<bool, semio_framework_value::ValueError> {
        if self.is_finished() { return Ok(true); }
        self.work = self.work.saturating_add(1);
        if let Some(active) = self.active.as_mut() {
            if matches!(active.close_step(1, maximum_bytes)?, SnapshotRetirementStep::Complete) {
                assert!(active.terminal_is_empty(), "message clamp retirement child is terminal");
                drop(self.active.take());
            }
            return Ok(false);
        }
        match self.phase {
            Phase::Measure => {
                let Some(message) = messages.get(self.index) else {
                    self.changed = self.total > ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES;
                    self.phase = if self.changed { Phase::Rank } else { Phase::Finish };
                    self.index = 0;
                    return Ok(false);
                };
                if self.measuring.is_none() {
                    self.measuring = Some(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES.saturating_add(message.code.0.len()).saturating_add(message.message.len()));
                    if self.worst.is_none_or(|(level, _)| message.level > level) { self.worst = Some((message.level, self.index)); }
                    self.target = 0;
                    return Ok(false);
                }
                if let Some(part) = message.target.get(self.target) {
                    self.measuring = Some(self.measuring.unwrap().saturating_add(ARTIFACT_EDIT_MESSAGE_TARGET_BYTES).saturating_add(part.len()));
                    self.target += 1;
                    return Ok(false);
                }
                let size = self.measuring.take().expect("active message measurement");
                self.total = self.total.saturating_add(size);
                self.index += 1;
            }
            Phase::Rank => {
                if self.level == 4 {
                    self.phase = Phase::Drain;
                    return Ok(false);
                }
                let Some(message) = messages.get(self.index) else {
                    self.level += 1;
                    self.index = 0;
                    return Ok(false);
                };
                let level = [Severity::Fatal, Severity::Error, Severity::Warning, Severity::Info][self.level];
                if message.level == level {
                    if self.measuring.is_none() {
                        self.measuring = Some(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES.saturating_add(message.code.0.len()).saturating_add(message.message.len()));
                        self.target = 0;
                        return Ok(false);
                    }
                    if let Some(part) = message.target.get(self.target) {
                        self.measuring = Some(self.measuring.unwrap().saturating_add(ARTIFACT_EDIT_MESSAGE_TARGET_BYTES).saturating_add(part.len()));
                        self.target += 1;
                        return Ok(false);
                    }
                    let size = self.measuring.take().expect("active severity-row measurement");
                    if self.used.saturating_add(size) > self.budget {
                        self.phase = Phase::Drain;
                        return Ok(false);
                    }
                    self.used += size;
                    self.kept += 1;
                    self.threshold = Some((level, self.index));
                }
                self.index += 1;
            }
            Phase::Drain => {
                let Some(mut message) = messages.pop() else {
                    self.phase = Phase::Restore;
                    return Ok(false);
                };
                let index = messages.len();
                let keep = self.threshold.is_some_and(|(level, last)| message.level > level || (message.level == level && index <= last));
                let truncate = self.kept == 0 && self.worst.is_some_and(|(_, worst)| worst == index);
                if truncate {
                    let mut cut = self.budget.saturating_sub(ARTIFACT_EDIT_MESSAGE_OWNER_BYTES).saturating_sub(message.code.0.len()).min(message.message.len());
                    while !message.message.is_char_boundary(cut) { cut -= 1; }
                    let replacement = message.message[..cut].to_string();
                    let retired = MutationMessage { level: message.level, code: String::new().into(), message: std::mem::replace(&mut message.message, replacement), target: std::mem::take(&mut message.target), op_index: message.op_index };
                    self.active = Some(Box::new(ArtifactStoreMessageLedgerRetirement::new(String::new(), vec![retired])));
                    self.retained.push(message);
                } else if keep {
                    self.retained.push(message);
                } else {
                    self.first_dropped = Some(message.op_index);
                    self.active = Some(Box::new(ArtifactStoreMessageLedgerRetirement::new(String::new(), vec![message])));
                }
            }
            Phase::Restore => {
                if let Some(message) = self.retained.pop() { messages.push(message); }
                else { self.phase = Phase::Summary; }
            }
            Phase::Summary => {
                if let Some(op_index) = self.first_dropped.take() {
                    let mut summary = MutationMessage::info("mutation.cascade", format!("{} more messages", self.original_len - messages.len()));
                    summary.op_index = op_index;
                    messages.push(summary);
                }
                self.phase = Phase::Finish;
            }
            Phase::Finish => {
                self.phase = Phase::Finished;
            }
            Phase::Finished => {}
        }
        Ok(self.is_finished())
    }

    /// 🛑️ Transfers one partial owner to the existing registered history-retirement lane.
    pub(super) fn retire_item(&mut self) -> Option<Box<dyn ErasedSnapshotRetirement>> {
        if let Some(active) = self.active.take() { return Some(active); }
        if let Some(message) = self.retained.pop() {
            return Some(Box::new(ArtifactStoreMessageLedgerRetirement::new(String::new(), vec![message])));
        }
        None
    }

    pub(super) fn finish_retirement(&mut self) { self.phase = Phase::Finished; }

    pub(super) fn close_cold(&mut self) {
        while let Some(mut child) = self.retire_item() {
            while !matches!(child.close_step(1, 4096).expect("cold message retirement"), SnapshotRetirementStep::Complete) {}
            assert!(child.terminal_is_empty());
        }
        self.finish_retirement();
    }
}

impl Drop for EditMessageClamp {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.is_finished() && self.active.is_none() && self.retained.is_empty()), "message clamp reached Drop before settlement or exact partial retirement");
    }
}
