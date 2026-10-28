//! ♻️ Neutral retained-owner retirement and typed factories.
use std::sync::Arc;
use crate::{ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
pub use semio_framework_value_control_contract::RetirementDemand;
/// 🪪️ Original decoding custody supplies its unchanged identity and funded cleanup allocations.
pub trait OriginalRetirementControl {
    fn ledger_identity(&self) -> u64;
    fn retirement_recipient_identity(&self) -> Option<u64>;
    fn charge(&mut self, bytes: usize) -> Result<(), ValueError>;
}
pub trait ErasedSnapshotRetirement: Send {
    /// 🧷️ Native owners override this port to preserve the caller's actual decoder ledger and recipient.
    fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.next_demand(grant.maximum_copy_bytes)?;if !semio_framework_value_control_contract::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()))}original.charge(d.capacity_bytes)?;self.close_step(grant)}
    fn requires_original_control(&self) -> bool { false }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>;
    /// 🧵️ Composite owners forward actual caller decoding custody to decoder-bound children.
    fn close_step_controlled(&mut self, grant: RetainedCloneGrant, _control: &mut dyn OriginalRetirementControl) -> Result<RetainedCloneStep, ValueError> { self.close_step(grant) }
    fn terminal_is_empty(&self) -> bool;
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, ValueError>;
    fn next_release_byte_demand(&self) -> Result<usize, ValueError>;
    fn next_depth_demand(&self) -> Result<usize, ValueError>;
    /// 🎟️ Quotes the original next action once, preserving its actual body hint through every nested owner.
    fn next_demand(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand { copy_bytes: self.next_copy_byte_demand()?, capacity_bytes: self.next_capacity_byte_demand(maximum_body_bytes)?, release_bytes: self.next_release_byte_demand()?, depth: self.next_depth_demand()? })
    }
}

