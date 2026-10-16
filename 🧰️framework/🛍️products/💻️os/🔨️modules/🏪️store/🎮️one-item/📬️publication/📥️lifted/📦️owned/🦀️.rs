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

pub struct SourceRetirementIssuer<M: Send + 'static> {
    pub birth_bytes: usize,
    pub begin: fn(std::collections::VecDeque<M>, RetainedCloneGrant) -> BatchAdmission<std::collections::VecDeque<M>>,
}
