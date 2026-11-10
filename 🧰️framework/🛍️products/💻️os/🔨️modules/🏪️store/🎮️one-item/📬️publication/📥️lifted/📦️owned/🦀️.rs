//! 📥️ Items of `member_owned_batch` that belong to `os-store-one-item-publication` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧩️composition/📨️emission/📦️owned/🦀️.rs`.

use semio_framework_value::ValueError;

use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_value::{RetainedAllocationEffect, OriginalRetirementPoll};

use semio_framework_value::retirement::RetireOwned;

use semio_framework_value::retirement::controlled::ControlledRetirement;
pub trait BatchRetirement: Send {
    fn step(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll;
    fn terminal_is_empty(&self) -> bool;
    fn demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;
}

impl<T: RetireOwned> BatchRetirement for ControlledRetirement<T> {
    fn step(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll { ControlledRetirement::step(self, body, effect) }
    fn terminal_is_empty(&self) -> bool { self.terminal_is_empty() }
    fn demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { Ok(semio_framework_value::RetirementDemand { copy_bytes: self.next_copy_byte_demand()?, capacity_bytes: self.next_capacity_byte_demand(maximum_body_bytes)?, release_bytes: self.next_release_byte_demand()?, depth: self.next_depth_demand()? }) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { ControlledRetirement::next_copy_byte_demand(self) }
}

pub type BatchAdmission = semio_framework_value::RetainedWorkOutcome<Option<(Box<dyn BatchRetirement>, RetainedCloneProgress)>, ValueError>;

pub struct SourceRetirementIssuer<M> {
    pub birth_bytes: usize,
    pub begin: fn(&mut Option<std::collections::VecDeque<M>>, &mut RetainedAllocationEffect<'_>) -> BatchAdmission,
}

/// 🎟️ The concrete native vocabulary lends its exact vector and source retirement constructors.
pub struct BatchRetirementIssuer<M> {
    pub birth_bytes: usize,
    pub begin: fn(&mut Option<Vec<M>>, &mut RetainedAllocationEffect<'_>) -> BatchAdmission,
    pub source: SourceRetirementIssuer<M>,
}
impl<M> Copy for SourceRetirementIssuer<M> {}
impl<M> Clone for SourceRetirementIssuer<M> { fn clone(&self) -> Self { *self } }
impl<M> Copy for BatchRetirementIssuer<M> {}
impl<M> Clone for BatchRetirementIssuer<M> { fn clone(&self) -> Self { *self } }
impl<M: RetireOwned> BatchRetirementIssuer<M> {
    /// 🪪️ Captures only defining typed retirement authority; no factory is substituted later.
    pub fn typed() -> Self {
        fn admit<T: RetireOwned>(value: &mut Option<T>, effect: &mut RetainedAllocationEffect<'_>) -> BatchAdmission {
            let outcome = semio_framework_value::retirement::controlled::admit_typed_controlled_retirement(value, effect);
            semio_framework_value::RetainedWorkOutcome { work: outcome.work, result: outcome.result.map(|admitted| admitted.map(|(owner, progress)| (owner as Box<dyn BatchRetirement>, progress))) }
        }
        Self { birth_bytes: std::mem::size_of::<ControlledRetirement<Vec<M>>>(), begin: admit::<Vec<M>>, source: SourceRetirementIssuer { birth_bytes: std::mem::size_of::<ControlledRetirement<std::collections::VecDeque<M>>>(), begin: admit::<std::collections::VecDeque<M>> } }
    }
}
