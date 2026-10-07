//! 🔁️ Settles one edit's diagnostic views without discarding mutation severity.
use super::EditMessageClamp;
use super::super::{ArtifactStoreMessageLedgerRetirement, ErasedSnapshotRetirement, MessageCopyCursor, MessageCopyGrant, MessageCopyStep, SnapshotRetirementStep, ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES};
use crate::os_spr::{MutationMessage, MutationReplayOutcome};
use semio_framework_value::ValueError;

enum Phase { Clamp, RetireOutcome, Copy, CloseCopy, NextOutcome, Finish, Finished }

pub(crate) struct EditMessageSettlement {
    clamp: EditMessageClamp,
    phase: Phase,
    outcome: usize,
    end: usize,
    ledger: usize,
    copy: Option<MessageCopyCursor>,
    active: Option<Box<dyn ErasedSnapshotRetirement>>,
    work: u64,
}

struct MessageCopyRetirement(MessageCopyCursor);

impl ErasedSnapshotRetirement for MessageCopyRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> { self.0.close_step(maximum_items, maximum_bytes) }
    fn terminal_is_empty(&self) -> bool { self.0.terminal_is_empty() }
}

impl EditMessageSettlement {
    pub(crate) fn new(edit_id: &str, messages: &[MutationMessage], first_outcome: usize, end: usize) -> Self {
        Self { clamp: EditMessageClamp::new(edit_id, messages), phase: Phase::Clamp, outcome: first_outcome, end, ledger: 0, copy: None, active: None, work: 0 }
    }

    pub(crate) fn completed_work(&self) -> u64 { self.work }
    pub(crate) fn is_finished(&self) -> bool { matches!(self.phase, Phase::Finished) }

    /// ⏳️ Visits one diagnostic segment, view entry or exact cleanup child.
    pub(crate) fn step(&mut self, messages: &mut Vec<MutationMessage>, outcomes: &mut [MutationReplayOutcome], maximum_bytes: usize) -> Result<bool, ValueError> {
        if self.is_finished() { return Ok(true); }
        self.work = self.work.saturating_add(1);
        if let Some(active) = self.active.as_mut() {
            if active.close_step(1, maximum_bytes)? == SnapshotRetirementStep::Complete {
                assert!(active.terminal_is_empty(), "settlement cleanup child is terminal");
                drop(self.active.take());
            }
            return Ok(false);
        }
        match self.phase {
            Phase::Clamp => {
                if self.clamp.step(messages, maximum_bytes)? {
                    self.phase = if self.clamp.changed() { Phase::RetireOutcome } else { Phase::Finish };
                }
            }
            Phase::RetireOutcome => {
                if self.outcome == self.end { self.phase = Phase::Finish; }
                else {
                    let original = std::mem::replace(&mut outcomes[self.outcome].messages, Vec::with_capacity(messages.len()));
                    self.active = Some(Box::new(ArtifactStoreMessageLedgerRetirement::new(String::new(), original)));
                    self.ledger = 0;
                    self.phase = Phase::Copy;
                }
            }
            Phase::Copy => {
                if let Some(source) = messages.get(self.ledger) {
                    if source.op_index != Some(outcomes[self.outcome].op_index) { self.ledger += 1; }
                    else if let Some(copy) = self.copy.as_mut() {
                        let grant = MessageCopyGrant { maximum_items: 1, maximum_copy_bytes: maximum_bytes, maximum_capacity_bytes: ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES };
                        if matches!(copy.advance(source, grant)?, MessageCopyStep::Complete(_)) {
                            outcomes[self.outcome].messages.push(copy.take().expect("completed message copy owns a diagnostic"));
                            copy.begin_close();
                            self.phase = Phase::CloseCopy;
                        }
                    } else { self.copy = Some(MessageCopyCursor::new()); }
                } else { self.phase = Phase::NextOutcome; }
            }
            Phase::CloseCopy => {
                let copy = self.copy.as_mut().expect("settlement owns its completed copy cursor");
                if copy.close_step(1, maximum_bytes)? == SnapshotRetirementStep::Complete {
                    assert!(copy.terminal_is_empty());
                    drop(self.copy.take());
                    self.ledger += 1;
                    self.phase = Phase::Copy;
                }
            }
            Phase::NextOutcome => { self.outcome += 1; self.phase = Phase::RetireOutcome; }
            Phase::Finish => { self.phase = Phase::Finished; }
            Phase::Finished => {}
        }
        Ok(self.is_finished())
    }

    /// 🛑️ Transfers each retained partial owner before the replay's existing rows retire.
    pub(crate) fn retire_item(&mut self) -> Option<Box<dyn ErasedSnapshotRetirement>> {
        if let Some(active) = self.active.take() { return Some(active); }
        if let Some(mut copy) = self.copy.take() {
            copy.cancel();
            copy.begin_close();
            return Some(Box::new(MessageCopyRetirement(copy)));
        }
        self.clamp.retire_item()
    }

    pub(crate) fn finish_retirement(&mut self) { self.clamp.finish_retirement(); self.phase = Phase::Finished; }

    pub(crate) fn close_cold(&mut self) {
        while let Some(mut child) = self.retire_item() {
            while child.close_step(1, ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES).expect("cold settlement retirement") != SnapshotRetirementStep::Complete {}
            assert!(child.terminal_is_empty());
        }
        self.finish_retirement();
    }
}

impl Drop for EditMessageSettlement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.is_finished() && self.copy.is_none() && self.active.is_none()), "message settlement dropped before terminal ownership");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(mut owner: Box<dyn ErasedSnapshotRetirement>, bytes: usize) {
        for _ in 0..1_000_000 {
            match owner.close_step(1, bytes).unwrap() {
                SnapshotRetirementStep::Complete => { assert!(owner.terminal_is_empty()); return; }
                SnapshotRetirementStep::Pending { released_items, released_bytes } => { assert!(released_items <= 1); assert!(released_bytes <= bytes); }
                SnapshotRetirementStep::Blocked => panic!("nonzero settlement cleanup grant blocked"),
            }
        }
        panic!("settlement cleanup did not terminate");
    }

    #[test]
    fn edit_message_clamp_settlement_cancels_at_every_owned_stage() {
        let law: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
        for bytes in [1, 7] {
            for stage in law["settlement"]["cancelStages"].as_array().unwrap() {
                let baseline = serde_json::json!({"artifact": "unchanged", "alternatives": ["original"]});
                let mut messages: Vec<_> = (0..6).map(|index| { let mut message = MutationMessage::fatal("mutation.invariant", "x".repeat(1000)); message.op_index = Some(index); message }).collect();
                let mut outcomes: Vec<_> = messages.iter().map(|message| MutationReplayOutcome { mutation_id: crate::os_spr::MutationId(format!("mutation:{}", message.op_index.unwrap())), edit_id: "owned-stages".into(), op_index: message.op_index.unwrap(), worst: Some(message.level), messages: vec![message.clone()], superseded: false, withdrawn: false }).collect();
                let original_status: Vec<_> = outcomes.iter().map(|outcome| outcome.worst).collect();
                let mut cursor = EditMessageSettlement::new("owned-stages", &messages, 0, outcomes.len());
                for turn in 0..1_000_000 {
                    let reached = match stage.as_str().unwrap() {
                        "clamp" => turn == 1,
                        "outcome-retirement" => matches!(cursor.phase, Phase::Copy) && cursor.active.is_some(),
                        "partial-copy" => matches!(cursor.phase, Phase::Copy) && cursor.copy.is_some(),
                        "finished" => cursor.is_finished(),
                        _ => unreachable!(),
                    };
                    if reached {
                        if stage == "partial-copy" {
                            for _ in 0..10 { cursor.step(&mut messages, &mut outcomes, bytes).unwrap(); }
                            assert!(cursor.copy.is_some());
                            assert!(matches!(cursor.phase, Phase::Copy));
                            assert!(outcomes[cursor.outcome].messages.is_empty());
                        }
                        break;
                    }
                    let before = cursor.completed_work();
                    cursor.step(&mut messages, &mut outcomes, bytes).unwrap();
                    assert_eq!(cursor.completed_work() - before, 1);
                    assert_eq!(outcomes.iter().map(|outcome| outcome.worst).collect::<Vec<_>>(), original_status);
                    assert!(turn < 999_999, "requested settlement stage was not reached");
                }
                while let Some(child) = cursor.retire_item() { close(child, bytes); }
                cursor.finish_retirement();
                assert!(cursor.is_finished());
                drop(cursor);
                close(Box::new(ArtifactStoreMessageLedgerRetirement::new(String::new(), messages)), bytes);
                for outcome in outcomes { close(Box::new(ArtifactStoreMessageLedgerRetirement::new(outcome.edit_id, outcome.messages)), bytes); }
                assert_eq!(baseline, serde_json::json!({"artifact": "unchanged", "alternatives": ["original"]}));
            }
        }
        eprintln!("[DEBUG] edit settlement cancellation covered clamp, exact outcome retirement, partial bounded copy and finished ownership with1/7-byte grants");
    }
}
