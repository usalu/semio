//! 🧹️ Bounded direct-leaf ownership transfer for Flow mutation retirement.

use super::{FlowMutation, FlowOwner, FlowRetirement};
use crate::os_store::{ErasedSnapshotRetirement, SnapshotRetirementStep};
use std::mem::ManuallyDrop;

//#region 🧹️MutationFrontier
#[must_use = "Retained Flow mutations must be closed to an empty frontier"]
pub(super) struct FlowMutationRetirementFrontier {
    mutation: ManuallyDrop<Option<FlowMutation>>,
    frontier: FlowRetirement,
}

impl FlowMutationRetirementFrontier {
    pub(super) fn new(mutation: FlowMutation) -> Self {
        Self { mutation: ManuallyDrop::new(Some(mutation)), frontier: FlowRetirement::default() }
    }

    fn handoff(&mut self, mutation: FlowMutation) {
        match mutation {
            FlowMutation::AddWidget(value) => self.frontier.push(FlowOwner::Widget(value.widget)),
            FlowMutation::RemoveWidget(value) => self.frontier.text(value.id),
            FlowMutation::MoveWidget(value) => self.frontier.text(value.id),
            FlowMutation::ChangeWidget(value) => {
                self.frontier.text(value.id);
                self.frontier.push(FlowOwner::Widget(value.widget));
            }
            FlowMutation::AddSynapse(value) => self.frontier.push(FlowOwner::Specs(vec![value.synapse])),
            FlowMutation::RemoveSynapse(value) => self.frontier.text(value.id),
            FlowMutation::MoveSynapse(value) => self.frontier.text(value.id),
            FlowMutation::ChangeSynapse(value) => {
                self.frontier.text(value.id);
                self.frontier.push(FlowOwner::Specs(vec![value.synapse]));
            }
            FlowMutation::ChangeLayout(value) => self.frontier.push(FlowOwner::Layout(value.entries)),
            FlowMutation::ReplaceFlowFixture(value) => self.frontier.push(FlowOwner::Fixture(value.fixture)),
        }
    }

    pub(super) fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        self.close_step_with(maximum_items, maximum_bytes, |frontier, items, bytes| frontier.close_step(items, bytes))
    }

    #[cfg(test)]
    pub(super) fn close_step_with_injected<F>(&mut self, maximum_items: usize, maximum_bytes: usize, close: F) -> Result<SnapshotRetirementStep, String>
    where F: FnOnce(&mut FlowRetirement, usize, usize) -> Result<SnapshotRetirementStep, String> {
        self.close_step_with(maximum_items, maximum_bytes, close)
    }

    fn close_step_with<F>(&mut self, maximum_items: usize, maximum_bytes: usize, close: F) -> Result<SnapshotRetirementStep, String>
    where F: FnOnce(&mut FlowRetirement, usize, usize) -> Result<SnapshotRetirementStep, String> {
        if self.terminal_is_empty() {
            return Ok(SnapshotRetirementStep::Complete);
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return Ok(SnapshotRetirementStep::Blocked);
        }
        if let Some(mutation) = self.mutation.take() {
            self.handoff(mutation);
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        let step = close(&mut self.frontier, maximum_items, maximum_bytes)?;
        if matches!(step, SnapshotRetirementStep::Complete) && !self.terminal_is_empty() {
            return Err("flow mutation retirement frontier reported Complete before terminal-empty".into());
        }
        Ok(step)
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.mutation.is_none() && self.frontier.is_empty()
    }
}

impl Drop for FlowMutationRetirementFrontier {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            if !std::thread::panicking() {
                panic!("Flow mutation retirement frontier dropped before terminal-empty");
            }
            return;
        }
        unsafe { ManuallyDrop::drop(&mut self.mutation); }
    }
}
//#endregion 🧹️MutationFrontier

//#region 🧪️NativeTests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️NativeTests
