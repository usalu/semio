//! 📥️ Items of `member_owned_batch` that belong to `os-store-one-item-publication` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧩️composition/📨️emission/📦️owned/🦀️.rs`.

use semio_framework_value::ValueError;

use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};

use semio_framework_value::retirement::RetireOwned;

use semio_framework_value::retirement::controlled::ControlledRetirement;
pub trait BatchRetirement: Send {
    fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    fn terminal_is_empty(&self) -> bool;
    fn demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError>;
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;
}

impl<T: RetireOwned> BatchRetirement for ControlledRetirement<T> {
    fn step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.step(grant) }
    fn terminal_is_empty(&self) -> bool { self.terminal_is_empty() }
    fn demands(&self, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { Ok(semio_framework_value::RetirementDemand { copy_bytes: self.next_copy_byte_demand()?, capacity_bytes: self.next_capacity_byte_demand(maximum_body_bytes)?, release_bytes: self.next_release_byte_demand()?, depth: self.next_depth_demand()? }) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { ControlledRetirement::next_copy_byte_demand(self) }
}

pub type BatchAdmission<T> = Result<(Box<dyn BatchRetirement>, RetainedCloneProgress), (ValueError, T)>;

pub struct SourceRetirementIssuer<M> {
    pub birth_bytes: usize,
    pub begin: fn(std::collections::VecDeque<M>, RetainedCloneGrant) -> BatchAdmission<std::collections::VecDeque<M>>,
}

/// 🎟️ The concrete native vocabulary lends its exact vector and source retirement constructors.
pub struct BatchRetirementIssuer<M> {
    pub birth_bytes: usize,
    pub begin: fn(Vec<M>, RetainedCloneGrant) -> BatchAdmission<Vec<M>>,
    pub source: SourceRetirementIssuer<M>,
}
impl<M> Copy for SourceRetirementIssuer<M> {}
impl<M> Clone for SourceRetirementIssuer<M> { fn clone(&self) -> Self { *self } }
impl<M> Copy for BatchRetirementIssuer<M> {}
impl<M> Clone for BatchRetirementIssuer<M> { fn clone(&self) -> Self { *self } }
impl<M: RetireOwned> BatchRetirementIssuer<M> {
    /// 🪪️ Captures only defining typed retirement authority; no factory is substituted later.
    pub fn typed() -> Self {
        fn admit<T: RetireOwned>(value: T, grant: RetainedCloneGrant) -> BatchAdmission<T> {
            semio_framework_value::retirement::controlled::admit_typed_controlled_retirement(value, grant).map(|(owner,p)| (owner as Box<dyn BatchRetirement>,p))
        }
        Self { birth_bytes: std::mem::size_of::<ControlledRetirement<Vec<M>>>(), begin: admit::<Vec<M>>, source: SourceRetirementIssuer { birth_bytes: std::mem::size_of::<ControlledRetirement<std::collections::VecDeque<M>>>(), begin: admit::<std::collections::VecDeque<M>> } }
    }
}
