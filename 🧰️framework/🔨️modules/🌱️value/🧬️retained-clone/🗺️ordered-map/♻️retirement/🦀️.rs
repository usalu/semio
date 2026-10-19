//! ♻️ Preserves original editing cursors inside separately admitted retirement frames.

use super::*;
use crate::retirement::{RetirementCursor,RetirementStep};
use std::mem::ManuallyDrop;

trait EditOwner:Send+'static {
 fn demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>;
 fn begin(&mut self);
 fn close(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>;
 fn empty(&self)->bool;
}
struct EditRetirement<C:EditOwner>{cursor:ManuallyDrop<C>}
impl<C:EditOwner> RetirementCursor for EditRetirement<C> {
 fn next_demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{self.cursor.demand(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{match self.cursor.close(grant){Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress),Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.cursor.empty()}
 fn next_depth_demand(&self)->Result<usize,crate::ValueError>{Ok(self.cursor.demand(0)?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,crate::ValueError>{Ok(self.cursor.demand(0)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.cursor.demand(0).ok().map(|d|d.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.cursor.demand(body).ok().map(|d|d.capacity_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.cursor.empty().then_some(size_of::<Self>())}
}
impl<C:EditOwner> Drop for EditRetirement<C>{fn drop(&mut self){if self.cursor.empty(){unsafe{ManuallyDrop::drop(&mut self.cursor)}}else if !std::thread::panicking(){panic!("original ordered edit frame requires physical closure")}}}
macro_rules! edit_owner {
 ($cursor:ident)=>{
  impl<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync> EditOwner for $cursor<K,V>{
   fn demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{self.close_demands(body)}
   fn begin(&mut self){self.begin_close();}
   fn close(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{self.close_step(grant)}
   fn empty(&self)->bool{self.terminal_is_empty()}
  }
  impl<K:BoundedOrd+RetireOwned,V:RetireOwned+Sync> RetireOwned for $cursor<K,V>{
   fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin();Box::new(EditRetirement{cursor:ManuallyDrop::new(self)})}
   fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<EditRetirement<Self>>())}
   fn controlled_retirement_supported()->bool{K::controlled_retirement_supported()&&V::controlled_retirement_supported()}
  }
 };
}
edit_owner!(RetainedOrderedMapInsertCursor);
edit_owner!(RetainedOrderedMapRemoveCursor);

impl<K:RetainedClone,V:RetainedClone> EditOwner for RetainedOrderedMapCloneCursor<K,V>{
 fn demand(&self,body:usize)->Result<crate::RetirementDemand,crate::ValueError>{self.close_demands(body)}
 fn begin(&mut self){self.begin_close();}
 fn close(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{self.close_step(grant)}
 fn empty(&self)->bool{self.terminal_is_empty()}
}
impl<K:RetainedClone,V:RetainedClone> RetireOwned for RetainedOrderedMapCloneCursor<K,V>{
 fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin();Box::new(EditRetirement{cursor:ManuallyDrop::new(self)})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<EditRetirement<Self>>())}
 fn controlled_retirement_supported()->bool{K::controlled_retirement_supported()&&V::controlled_retirement_supported()}
}
