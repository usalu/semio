//! ♻️ Byte-accounted retirement for complete interaction roots and whole-state history mutations.

use crate::app::InteractionConfigMutation;
use protocol::{DomainHover, DomainSelection, InteractionState, SelectionMode};
use semio_framework_value::{
    RetirementDemand, ValueError, ValueRefusalKind,
    retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep},
};
use std::{mem::ManuallyDrop, sync::Arc};
use store::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};

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

/// 🧭️ The single next physical move of the frontier; quote and execution read the same variant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Move {
    TruncateBytes,
    ReleaseBytes(usize),
    TextToBytes,
    SelectionId,
    SelectionIdsRelease(usize),
    SelectionGranularity,
    SelectionAnchor,
    SelectionDone,
    HoverId,
    HoverIdsRelease(usize),
    HoverChannel,
    HoverDone,
    Unshare,
    StateSelection,
    StateHover,
    StateMode,
    StateGranularity,
    StateDone,
}

/// ♻️ Fixed-width frontier; payload vectors and strings are detached and drained separately.
pub(crate) struct InteractionRetirement {
    owned: ManuallyDrop<RetirementState>,
}

impl InteractionRetirement {
    pub(crate) fn owned(state: InteractionState) -> Self {
        Self { owned: ManuallyDrop::new(RetirementState { state: Some(state), ..Default::default() }) }
    }
    fn shared(state: Arc<InteractionState>) -> Self {
        Self { owned: ManuallyDrop::new(RetirementState { shared: Some(state), ..Default::default() }) }
    }

    fn next_move(&self) -> Option<Move> {
        let owned: &RetirementState = &self.owned;
        if !owned.bytes.is_empty() {
            return Some(Move::TruncateBytes);
        }
        if owned.bytes.capacity() != 0 {
            return Some(Move::ReleaseBytes(owned.bytes.capacity()));
        }
        if owned.text.is_some() {
            return Some(Move::TextToBytes);
        }
        if let Some(selection) = owned.selection.as_ref() {
            return Some(if !selection.ids.is_empty() {
                Move::SelectionId
            } else if selection.ids.capacity() != 0 {
                Move::SelectionIdsRelease(selection.ids.capacity() * size_of::<String>())
            } else if selection.granularity.capacity() != 0 {
                Move::SelectionGranularity
            } else if selection.anchor_id.is_some() {
                Move::SelectionAnchor
            } else {
                Move::SelectionDone
            });
        }
        if let Some(hover) = owned.hover.as_ref() {
            return Some(if !hover.ids.is_empty() {
                Move::HoverId
            } else if hover.ids.capacity() != 0 {
                Move::HoverIdsRelease(hover.ids.capacity() * size_of::<String>())
            } else if hover.channel.capacity() != 0 {
                Move::HoverChannel
            } else {
                Move::HoverDone
            });
        }
        if owned.shared.is_some() {
            return Some(Move::Unshare);
        }
        owned.state.as_ref().map(|state| {
            if !state.selection.is_empty() {
                Move::StateSelection
            } else if !state.hover.is_empty() {
                Move::StateHover
            } else if !state.active_mode.is_empty() {
                Move::StateMode
            } else if !state.active_granularity.is_empty() {
                Move::StateGranularity
            } else {
                Move::StateDone
            }
        })
    }

    fn demand_of(movement: Move) -> RetirementDemand {
        let string = size_of::<String>();
        let (copy_bytes, release_bytes) = match movement {
            Move::TruncateBytes => (1, 0),
            Move::ReleaseBytes(bytes) | Move::SelectionIdsRelease(bytes) | Move::HoverIdsRelease(bytes) => (0, bytes),
            Move::TextToBytes | Move::SelectionId | Move::SelectionGranularity | Move::SelectionAnchor | Move::HoverId | Move::HoverChannel => (string, 0),
            Move::StateSelection => (size_of::<(String, DomainSelection)>(), 0),
            Move::StateHover => (size_of::<(String, DomainHover)>(), 0),
            Move::StateMode => (size_of::<(String, SelectionMode)>(), 0),
            Move::StateGranularity => (size_of::<(String, String)>(), 0),
            Move::SelectionDone | Move::HoverDone | Move::Unshare | Move::StateDone => (0, 0),
        };
        RetirementDemand { copy_bytes, release_bytes, depth: 1, ..Default::default() }
    }

    fn demands(&self) -> RetirementDemand {
        self.next_move().map_or_else(Default::default, Self::demand_of)
    }
}

impl ErasedSnapshotRetirement for InteractionRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        let Some(movement) = self.next_move() else {
            return Ok(RetainedCloneStep::Complete(idle));
        };
        let demand = Self::demand_of(movement);
        if grant.maximum_depth < demand.depth {
            return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "interaction retirement exceeds admitted depth"));
        }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_release_bytes < demand.release_bytes {
            return Ok(RetainedCloneStep::Progress(idle));
        }
        let owned: &mut RetirementState = &mut self.owned;
        let mut copied_bytes = demand.copy_bytes;
        match movement {
            Move::TruncateBytes => {
                copied_bytes = grant.maximum_copy_bytes.min(owned.bytes.len());
                let next = owned.bytes.len() - copied_bytes;
                owned.bytes.truncate(next);
            }
            Move::ReleaseBytes(_) => owned.bytes = Vec::new(),
            Move::TextToBytes => owned.bytes = owned.text.take().expect("quoted text owner").into_bytes(),
            Move::SelectionId => owned.text = owned.selection.as_mut().and_then(|selection| selection.ids.pop()),
            Move::SelectionIdsRelease(_) => owned.selection.as_mut().expect("quoted selection owner").ids = Vec::new(),
            Move::SelectionGranularity => owned.text = owned.selection.as_mut().map(|selection| std::mem::take(&mut selection.granularity)),
            Move::SelectionAnchor => owned.text = owned.selection.as_mut().and_then(|selection| selection.anchor_id.take()),
            Move::SelectionDone => owned.selection = None,
            Move::HoverId => owned.text = owned.hover.as_mut().and_then(|hover| hover.ids.pop()),
            Move::HoverIdsRelease(_) => owned.hover.as_mut().expect("quoted hover owner").ids = Vec::new(),
            Move::HoverChannel => owned.text = owned.hover.as_mut().map(|hover| std::mem::take(&mut hover.channel)),
            Move::HoverDone => owned.hover = None,
            Move::Unshare => owned.state = Arc::into_inner(owned.shared.take().expect("quoted shared owner")),
            Move::StateSelection => {
                let (domain, selection) = owned.state.as_mut().and_then(|state| state.selection.pop_first()).expect("quoted selection entry");
                owned.text = Some(domain);
                owned.selection = Some(selection);
            }
            Move::StateHover => {
                let (domain, hover) = owned.state.as_mut().and_then(|state| state.hover.pop_first()).expect("quoted hover entry");
                owned.text = Some(domain);
                owned.hover = Some(hover);
            }
            Move::StateMode => owned.text = owned.state.as_mut().and_then(|state| state.active_mode.pop_first()).map(|(domain, _)| domain),
            Move::StateGranularity => {
                let (domain, value) = owned.state.as_mut().and_then(|state| state.active_granularity.pop_first()).expect("quoted granularity entry");
                owned.bytes = domain.into_bytes();
                owned.text = Some(value);
            }
            Move::StateDone => owned.state = None,
        }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes, released_bytes: demand.release_bytes, ..idle };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    fn terminal_is_empty(&self) -> bool {
        self.owned.shared.is_none() && self.owned.state.is_none() && self.owned.selection.is_none() && self.owned.hover.is_none() && self.owned.text.is_none() && self.owned.bytes.is_empty() && self.owned.bytes.capacity() == 0
    }

    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.demands().copy_bytes)
    }

    fn next_capacity_byte_demand(&self, _maximum_body_bytes: usize) -> Result<usize, ValueError> {
        Ok(self.demands().capacity_bytes)
    }

    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(self.demands().release_bytes)
    }

    fn next_depth_demand(&self) -> Result<usize, ValueError> {
        Ok(self.demands().depth)
    }
}

impl Drop for InteractionRetirement {
    fn drop(&mut self) {
        if !self.terminal_is_empty() {
            if !std::thread::panicking() {
                panic!("interaction retirement dropped before exact byte and allocation emptiness");
            }
            return;
        }
        unsafe {
            ManuallyDrop::drop(&mut self.owned);
        }
    }
}
//#endregion 📦️OwnedFrontier

//#region 🏪️StoreOwners
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(crate) struct InteractionRetirementFactory;

fn admit_birth(grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
    RetainedCloneBirthDemand { capacity_bytes: size_of::<InteractionRetirement>(), depth: 1 }.admit(grant)
}

impl SnapshotRetirementFactory<InteractionState> for InteractionRetirementFactory {
    fn retirement_birth_bytes(&self, _snapshot: &Arc<InteractionState>) -> usize { size_of::<InteractionRetirement>() }

    fn retire(&self, root: Arc<InteractionState>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Arc<InteractionState>)> {
        match admit_birth(grant) {
            Ok(progress) => Ok((Box::new(InteractionRetirement::shared(root)), progress)),
            Err(error) => Err((error, root)),
        }
    }
}

impl ArtifactOwnedValueRetirementFactory<InteractionState> for InteractionRetirementFactory {
    fn retirement_birth_bytes(&self, _root: &InteractionState) -> usize { size_of::<InteractionRetirement>() }

    fn retire_owned(&self, root: InteractionState, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, InteractionState)> {
        match admit_birth(grant) {
            Ok(progress) => Ok((Box::new(InteractionRetirement::owned(root)), progress)),
            Err(error) => Err((error, root)),
        }
    }
}

impl ArtifactOwnedValueRetirementFactory<InteractionConfigMutation> for InteractionRetirementFactory {
    fn retirement_birth_bytes(&self, _mutation: &InteractionConfigMutation) -> usize { size_of::<InteractionRetirement>() }

    fn retire_owned(&self, mutation: InteractionConfigMutation, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, InteractionConfigMutation)> {
        match admit_birth(grant) {
            Ok(progress) => {
                let InteractionConfigMutation::SetInteractionState(state) = mutation;
                Ok((Box::new(InteractionRetirement::owned(state.state)), progress))
            }
            Err(error) => Err((error, mutation)),
        }
    }
}

pub(crate) fn interaction_store_owners(grant: RetainedCloneGrant) -> Result<(store::DocumentStoreOwners<InteractionState, InteractionConfigMutation>, RetainedCloneProgress), store::DocumentStoreOwnersAdmissionError<InteractionState, InteractionConfigMutation>> {
    store::DocumentStoreOwners::admit_source_constructor(grant, || (InteractionRetirementFactory, InteractionRetirementFactory, InteractionRetirementFactory, store::ArtifactStoreCursorDisposer::<InteractionState, InteractionConfigMutation>::new()))
}

pub(crate) fn interaction_store_owners_source_demands()->Result<semio_framework_value::RetirementDemand,ValueError>{
 type Owners=store::DocumentStoreOwners<InteractionState,InteractionConfigMutation>;
 type Disposer=store::ArtifactStoreCursorDisposer<InteractionState,InteractionConfigMutation>;
 Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Owners>()+size_of::<(InteractionRetirementFactory,InteractionRetirementFactory,InteractionRetirementFactory,Disposer)>(),capacity_bytes:Owners::source_birth_bytes::<InteractionRetirementFactory,InteractionRetirementFactory,InteractionRetirementFactory,Disposer>()?,depth:1,..Default::default()})
}

/// 🎟️ Funds the interaction catalog from its own quoted source and ticket demands.
pub(crate) fn funded_interaction_store_owners() -> Result<store::DocumentStoreOwners<InteractionState, InteractionConfigMutation>, semio_framework_value::ValueError> {
    let capacity = store::DocumentStoreOwners::<InteractionState, InteractionConfigMutation>::source_birth_bytes::<InteractionRetirementFactory, InteractionRetirementFactory, InteractionRetirementFactory, store::ArtifactStoreCursorDisposer<InteractionState, InteractionConfigMutation>>()?;
    store::complete_unscheduled_catalog(interaction_store_owners(RetainedCloneGrant { maximum_items: 1, maximum_capacity_bytes: capacity, maximum_depth: 1, ..Default::default() }))
}
//#endregion 🏪️StoreOwners

#[cfg(test)]
#[path = "🧪️tests/♻️retirement/🦀️.rs"]
mod tests;
