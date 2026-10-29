//! 🏛️ The process owns its registered factories; declaration sources lend paid mounted aliases without rebuilding that registry.
use super::ActionBus;
use semio_framework_value::{
    RetirementDemand, ValueError,
    retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep},
    retirement::{RetireOwned, RetirementCursor, RetirementStep},
};
use std::{
    mem::{ManuallyDrop, size_of},
    sync::Arc,
};
/// 🧾️ Declaration assembly captures this source after registering its actual factories in the process-owned bus.
pub struct RegisteredActionBusSource {
    bus: ActionBus,
}
impl RegisteredActionBusSource {
    pub fn production() -> Self {
        Self { bus: ActionBus::production() }
    }
    /// 🧾️ Reference construction names the explicit bus its factories are registered in.
    pub fn reference(bus: ActionBus) -> Self {
        Self { bus }
    }
    pub fn identity(&self) -> usize {
        Arc::as_ptr(&self.bus.inner) as usize
    }
    pub fn bus(&self) -> &ActionBus {
        &self.bus
    }
    pub fn alias_demands(&self) -> RetirementDemand {
        RetirementDemand { copy_bytes: 2 * size_of::<RegisteredActionBusAlias>() + size_of::<ActionBus>(), depth: 1, ..Default::default() }
    }
    pub fn admit_alias(&self, grant: RetainedCloneGrant) -> Result<Option<(RegisteredActionBusAlias, RetainedCloneProgress)>, ValueError> {
        let d = self.alias_demands();
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, d) {
            return Ok(None);
        }
        Ok(Some((RegisteredActionBusAlias { original: ManuallyDrop::new(Some(self.bus.clone())) }, RetainedCloneProgress { copied_items: 1, copied_bytes: d.copy_bytes, ..Default::default() })))
    }
}
/// 🪢️ This lease returns only its process-root alias; it does not claim ownership or physical closure of global factories.
pub struct RegisteredActionBusAlias {
    original: ManuallyDrop<Option<ActionBus>>,
}
impl RegisteredActionBusAlias {
    pub fn bus(&self) -> &ActionBus {
        self.original.as_ref().expect("registered bus lease already returned")
    }
    pub fn identity(&self) -> Option<usize> {
        self.original.as_ref().map(|bus| Arc::as_ptr(&bus.inner) as usize)
    }
    pub fn close_demands(&self) -> RetirementDemand {
        if self.original.is_some() { RetirementDemand { copy_bytes: size_of::<Option<ActionBus>>(), depth: 1, ..Default::default() } } else { Default::default() }
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> RetainedCloneStep {
        if self.original.is_none() {
            return RetainedCloneStep::Complete(Default::default());
        }
        let d = self.close_demands();
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, d) {
            return RetainedCloneStep::Progress(Default::default());
        }
        drop(self.original.take());
        RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: d.copy_bytes, ..Default::default() })
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.original.is_none()
    }
}
impl Drop for RegisteredActionBusAlias {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "mounted registered bus alias requires its original paid return")
    }
}
struct RegisteredSourceRetirement {
    source: ManuallyDrop<Option<RegisteredActionBusSource>>,
}
impl RetirementCursor for RegisteredSourceRetirement {
    fn next_demand(&self, _: usize) -> Result<RetirementDemand, ValueError> {
        Ok(if self.source.is_some() { RetirementDemand { copy_bytes: size_of::<Option<RegisteredActionBusSource>>(), depth: 1, ..Default::default() } } else { Default::default() })
    }
    fn close_step(&mut self, g: RetainedCloneGrant) -> RetirementStep {
        if self.source.is_none() {
            return RetirementStep::Complete;
        }
        let d = self.next_demand(g.maximum_copy_bytes).expect("fixed registered source return quote");
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(g, d) {
            return RetirementStep::BudgetExhausted;
        }
        drop(self.source.take());
        RetirementStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: d.copy_bytes, ..Default::default() })
    }
    fn terminal_is_empty(&self) -> bool {
        self.source.is_none()
    }
    fn terminal_release_bytes(&self) -> Option<usize> {
        self.terminal_is_empty().then_some(size_of::<Self>())
    }
}
impl Drop for RegisteredSourceRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "registered declaration source return retains its original process alias")
    }
}
impl RetireOwned for RegisteredActionBusSource {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(RegisteredSourceRetirement { source: ManuallyDrop::new(Some(self)) })
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        Some(size_of::<RegisteredSourceRetirement>())
    }
    fn controlled_retirement_supported() -> bool {
        true
    }
}
