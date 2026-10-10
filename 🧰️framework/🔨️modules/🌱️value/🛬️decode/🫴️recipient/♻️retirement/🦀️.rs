//! 🧺️ Retires the actual decoder recipient and its retained original child.
use super::NativeDecodeRetirementRecipient;
use crate::{ErasedSnapshotRetirement,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use std::mem::size_of;

impl RetireOwned for NativeDecodeRetirementRecipient {
    fn retirement(self)->Box<dyn RetirementCursor>{Box::new(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
    fn controlled_retirement_supported()->bool{true}
}
impl RetirementCursor for NativeDecodeRetirementRecipient {
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{
        match ErasedSnapshotRetirement::close_step(self,grant){
            Err(error)=>RetirementStep::Failure(error),
            Ok(RetainedCloneStep::Complete(progress)|RetainedCloneStep::Progress(progress))=>RetirementStep::Progress(progress),
        }
    }
    fn terminal_is_empty(&self)->bool{ErasedSnapshotRetirement::terminal_is_empty(self)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{ErasedSnapshotRetirement::next_depth_demand(self)}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{ErasedSnapshotRetirement::next_copy_byte_demand(self)}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn next_close_byte_demand(&self)->Option<usize>{ErasedSnapshotRetirement::next_release_byte_demand(self).ok()}
    fn next_birth_bytes(&self,body:usize)->Option<usize>{ErasedSnapshotRetirement::next_capacity_byte_demand(self,body).ok()}
    fn terminal_release_bytes(&self)->Option<usize>{ErasedSnapshotRetirement::terminal_is_empty(self).then_some(size_of::<Self>())}
}
