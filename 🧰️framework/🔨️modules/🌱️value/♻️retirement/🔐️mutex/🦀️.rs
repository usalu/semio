//! 🔐️ Original owned mutex and cell payloads preserve custody through typed funded retirement.
use super::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use crate::{ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};

#[doc(hidden)]
pub struct Cursor<T:RetireOwned>{owner:ControlledRetirement<T>}
impl<T:RetireOwned> RetireOwned for std::sync::Mutex<T>{
 type RetirementCursor=Cursor<T>;fn retirement_seed(self)->Self::RetirementCursor{let original=self.into_inner().unwrap_or_else(|poisoned|poisoned.into_inner());Cursor{owner:ControlledRetirement::retain(original)}}
 fn retirement_birth_bytes(&self)->Option<usize>{T::controlled_retirement_supported().then_some(size_of::<Cursor<T>>())}
 fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
}
impl<T:RetireOwned> RetireOwned for std::cell::Cell<T>{
 type RetirementCursor=Cursor<T>;fn retirement_seed(self)->Self::RetirementCursor{Cursor{owner:ControlledRetirement::retain(self.into_inner())}}
 fn retirement_birth_bytes(&self)->Option<usize>{T::controlled_retirement_supported().then_some(size_of::<Cursor<T>>())}
 fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
}
impl<T:RetireOwned> RetirementCursor for Cursor<T>{
 native_retirement_reply!();
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.owner.next_computational_work(body)}
 fn requires_original_control(&self)->bool{crate::ErasedSnapshotRetirement::requires_original_control(&self.owner)}
 fn close_step_controlled(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->RetirementStep{let grant=effect.grant();super::erased_cursor_turn(crate::ErasedSnapshotRetirement::close_step_controlled(&mut self.owner,body,&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect"),control))}
 fn close_original_step(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let grant=effect.grant();super::erased_cursor_turn(crate::ErasedSnapshotRetirement::close_original_step(&mut self.owner,original,body,&mut effect.reborrow(grant,effect.maximum_work_units()).expect("retirement child exceeds original effect")))}
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{self.owner.next_demand(body)}
 
 fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.owner.next_capacity_byte_demand(body).ok()}
 fn next_close_byte_demand(&self)->Option<usize>{self.owner.next_release_byte_demand().ok()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner.next_depth_demand()}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned> Cursor<T>{
 fn close_step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{super::erased_cursor_turn(self.owner.step(body,effect))}
}


#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;


