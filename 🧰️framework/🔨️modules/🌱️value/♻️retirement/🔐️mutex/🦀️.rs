//! 🔐️ Original owned mutex payloads preserve custody through typed funded retirement.
use super::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use crate::{ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};

struct Cursor<T:RetireOwned>{owner:ControlledRetirement<T>}
impl<T:RetireOwned> RetireOwned for std::sync::Mutex<T>{
 fn retirement(self)->Box<dyn RetirementCursor>{let original=self.into_inner().unwrap_or_else(|poisoned|poisoned.into_inner());Box::new(Cursor{owner:ControlledRetirement::new(original).unwrap_or_else(|_|panic!("admitted original mutex payload refused typed custody"))})}
 fn retirement_birth_bytes(&self)->Option<usize>{T::controlled_retirement_supported().then_some(size_of::<Cursor<T>>())}
 fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()}
}
impl<T:RetireOwned> RetirementCursor for Cursor<T>{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{match self.owner.step(grant){Ok(step)if step.progress()==RetainedCloneProgress::default()&&self.owner.terminal_is_empty()=>RetirementStep::Complete,Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.owner.next_capacity_byte_demand(body).ok()}
 fn next_close_byte_demand(&self)->Option<usize>{self.owner.next_release_byte_demand().ok()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner.next_depth_demand()}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
