//! ♻️ Byte-accounted retirement for complete interaction roots and whole-state history mutations.

use std::{mem::ManuallyDrop, sync::Arc};
use protocol::{DomainHover, DomainSelection, InteractionState};
use store::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory, SnapshotRetirementStep};
use crate::app::InteractionConfigMutation;

//#region 📦️OwnedFrontier
#[derive(Default)]
struct RetirementState {
    shared: Option<Arc<InteractionState>>,
    state: Option<InteractionState>,
    selection: Option<DomainSelection>,
    hover: Option<DomainHover>,
    text: Option<String>,
    bytes: Vec<u8>,
}

/// ♻️ Fixed-width frontier; payload vectors and strings are detached and drained separately.
pub(crate) struct InteractionRetirement { owned: ManuallyDrop<RetirementState> }

impl InteractionRetirement {
    pub(crate) fn owned(state: InteractionState) -> Self { Self { owned: ManuallyDrop::new(RetirementState { state: Some(state), ..Default::default() }) } }
    fn shared(state: Arc<InteractionState>) -> Self { Self { owned: ManuallyDrop::new(RetirementState { shared: Some(state), ..Default::default() }) } }
}

impl ErasedSnapshotRetirement for InteractionRetirement {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if self.terminal_is_empty() { return Ok(SnapshotRetirementStep::Complete); }
        if maximum_items == 0 { return Ok(SnapshotRetirementStep::Blocked); }
        let owned: &mut RetirementState = &mut self.owned;
        if !owned.bytes.is_empty() {
            if maximum_bytes == 0 { return Ok(SnapshotRetirementStep::Blocked); }
            let released_bytes = maximum_bytes.min(owned.bytes.len());
            let next = owned.bytes.len() - released_bytes;
            owned.bytes.truncate(next);
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes });
        }
        if owned.bytes.capacity() != 0 {
            owned.bytes = Vec::new();
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(text) = owned.text.take() {
            owned.bytes = text.into_bytes();
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(selection) = owned.selection.as_mut() {
            if let Some(id) = selection.ids.pop() { owned.text = Some(id); }
            else if selection.ids.capacity() != 0 { selection.ids = Vec::new(); }
            else if selection.granularity.capacity() != 0 { owned.text = Some(std::mem::take(&mut selection.granularity)); }
            else if let Some(anchor) = selection.anchor_id.take() { owned.text = Some(anchor); }
            else { owned.selection = None; }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(hover) = owned.hover.as_mut() {
            if let Some(id) = hover.ids.pop() { owned.text = Some(id); }
            else if hover.ids.capacity() != 0 { hover.ids = Vec::new(); }
            else if hover.channel.capacity() != 0 { owned.text = Some(std::mem::take(&mut hover.channel)); }
            else { owned.hover = None; }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(root) = owned.shared.take() {
            owned.state = Arc::into_inner(root);
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(state) = owned.state.as_mut() {
            if let Some((domain, selection)) = state.selection.pop_first() { owned.text = Some(domain); owned.selection = Some(selection); }
            else if let Some((domain, hover)) = state.hover.pop_first() { owned.text = Some(domain); owned.hover = Some(hover); }
            else if let Some((domain, _)) = state.active_mode.pop_first() { owned.text = Some(domain); }
            else if let Some((domain, value)) = state.active_granularity.pop_first() { owned.bytes = domain.into_bytes(); owned.text = Some(value); }
            else { owned.state = None; }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owned.shared.is_none() && self.owned.state.is_none() && self.owned.selection.is_none() && self.owned.hover.is_none() && self.owned.text.is_none() && self.owned.bytes.is_empty() && self.owned.bytes.capacity() == 0
    }
}

impl Drop for InteractionRetirement {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            if !std::thread::panicking() { panic!("interaction retirement dropped before exact byte and allocation emptiness"); }
            return;
        }
        unsafe { ManuallyDrop::drop(&mut self.owned); }
    }
}
//#endregion 📦️OwnedFrontier

//#region 🏪️StoreOwners
pub(crate) struct InteractionRetirementFactory;

impl SnapshotRetirementFactory<InteractionState> for InteractionRetirementFactory {
    fn retire(&self, root: Arc<InteractionState>) -> Box<dyn ErasedSnapshotRetirement> { Box::new(InteractionRetirement::shared(root)) }
}

impl ArtifactOwnedValueRetirementFactory<InteractionState> for InteractionRetirementFactory {
    fn retire_owned(&self, root: InteractionState) -> Box<dyn ErasedSnapshotRetirement> { Box::new(InteractionRetirement::owned(root)) }
}

impl ArtifactOwnedValueRetirementFactory<InteractionConfigMutation> for InteractionRetirementFactory {
    fn retire_owned(&self, mutation: InteractionConfigMutation) -> Box<dyn ErasedSnapshotRetirement> {
        let InteractionConfigMutation::SetState(state) = mutation;
        Box::new(InteractionRetirement::owned(state.state))
    }
}

pub(crate) fn interaction_store_owners() -> store::MemberStoreOwners<InteractionState, InteractionConfigMutation> {
    store::MemberStoreOwners::new(Arc::new(InteractionRetirementFactory), Arc::new(InteractionRetirementFactory), Arc::new(InteractionRetirementFactory), Box::new(store::ArtifactStoreCursorDisposer::<InteractionState, InteractionConfigMutation>::new()))
}
//#endregion 🏪️StoreOwners

#[cfg(test)]
#[path = "🧪️component.rs"]
mod tests;
