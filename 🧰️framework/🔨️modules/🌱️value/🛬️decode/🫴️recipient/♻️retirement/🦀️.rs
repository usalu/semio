//! 🧺️ Retires the actual decoder recipient and its retained original child.
use super::NativeDecodeRetirementRecipient;
use crate::{ErasedSnapshotRetirement,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use std::mem::size_of;

impl RetireOwned for NativeDecodeRetirementRecipient {
    fn retirement_ready(&self)->bool{self.funding.failure_allocation.is_none()&&self.funding.failure_port.is_none()&&self.funding.failure_ledger.is_none()&&self.funding.failure_verdict.is_none()&&self.funding.failure_protocol_refusal.is_none()&&self.funding.failure_birth.terminal_is_empty()}
    type RetirementCursor=Self;fn retirement_seed(self)->Self::RetirementCursor{self}
    fn retirement_birth_bytes(&self)->Option<usize>{self.retirement_ready().then_some(size_of::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for NativeDecodeRetirementRecipient {
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{ErasedSnapshotRetirement::next_computational_work(self,body)}
 fn requires_original_control(&self)->bool{ErasedSnapshotRetirement::requires_original_control(self)}
 fn requires_original_birth(&self)->bool{ErasedSnapshotRetirement::requires_original_birth(self)}
 fn close_original_birth_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{semio_framework_value_retirement::retirement::erased_cursor_turn(ErasedSnapshotRetirement::close_original_birth_step(self,original,grant))}
 fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{semio_framework_value_retirement::retirement::erased_cursor_turn(ErasedSnapshotRetirement::close_original_step(self,original,effect))}
 fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value_retirement::OriginalRetirementControl)->RetirementStep{semio_framework_value_retirement::retirement::erased_cursor_turn(ErasedSnapshotRetirement::close_step_controlled(self,effect,control))}
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{ErasedSnapshotRetirement::next_demand(self,body)}
    fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{semio_framework_value_retirement::retirement::erased_cursor_turn(ErasedSnapshotRetirement::close_step(self,effect))}
    fn terminal_is_empty(&self)->bool{ErasedSnapshotRetirement::terminal_is_empty(self)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{ErasedSnapshotRetirement::next_depth_demand(self)}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{ErasedSnapshotRetirement::next_copy_byte_demand(self)}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn next_close_byte_demand(&self)->Option<usize>{ErasedSnapshotRetirement::next_release_byte_demand(self).ok()}
    fn next_birth_bytes(&self,body:usize)->Option<usize>{ErasedSnapshotRetirement::next_capacity_byte_demand(self,body).ok()}
    fn terminal_release_bytes(&self)->Option<usize>{ErasedSnapshotRetirement::terminal_is_empty(self).then_some(size_of::<Self>())}
}
