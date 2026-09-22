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
            FlowMutation::ReplaceFlowHostSnapshot(value) => self.frontier.push(FlowOwner::HostSnapshot(value.host_snapshot)),
        }
    }

    pub(super) fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        self.close_step_with(maximum_items, maximum_bytes, |frontier, items, bytes| frontier.close_page(items, bytes))
    }

    #[cfg(test)]
    pub(super) fn close_step_with_injected<F>(&mut self, maximum_items: usize, maximum_bytes: usize, close: F) -> Result<SnapshotRetirementStep, String>
    where F: FnOnce(&mut FlowRetirement, usize, usize) -> Result<SnapshotRetirementStep, String> {
        self.close_step_with(maximum_items, maximum_bytes, close)
    }

    /// 📏️ A heap allocation is freed WHOLE or not at all, so this frontier READS the demand its
    /// domain publishes and grants it out of its own allocation currency, then charges the caller's
    /// payload page only what fits in it. Granting only the caller's page left a five-byte mutation
    /// id unfreeable at grant 1 forever (ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END).
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
        let demand = self.frontier.next_close_byte_demand().map_err(str::to_owned)?;
        let step = close(&mut self.frontier, maximum_items, maximum_bytes.max(demand))?;
        if matches!(step, SnapshotRetirementStep::Complete) && !self.terminal_is_empty() {
            return Err("flow mutation retirement frontier reported Complete before terminal-empty".into());
        }
        Ok(match step {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => SnapshotRetirementStep::Pending { released_items, released_bytes: released_bytes.min(maximum_bytes) },
            step => step,
        })
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.mutation.is_none() && self.frontier.terminal_is_empty()
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
#[path = "🧪️tests/🧹️retirement/🦀️.rs"]
mod tests;
//#endregion 🧪️NativeTests
