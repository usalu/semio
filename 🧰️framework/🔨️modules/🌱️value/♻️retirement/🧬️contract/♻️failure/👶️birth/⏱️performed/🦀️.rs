//! ⏱️ Failure birth records actual work in its original Core scope before unwinding caller loans.
use super::{OriginalFailureStorageBirth,OriginalFailureStorageBirthPhase,invalid};
use crate::{OriginalWorkState,OriginalFailureStoragePool,OriginalFailureStorage,RetainedAllocationEffect,ValueError};
/// 🛡️ Borrows the actual original work, resident birth and original spare pool together.
pub struct OriginalFailurePerformedScope<'a>{work:&'a mut OriginalWorkState,birth:&'a mut OriginalFailureStorageBirth,pool:&'a mut OriginalFailureStoragePool}
impl<'a> OriginalFailurePerformedScope<'a>{
 /// 🎟️ Issues only the unchanged original birth's first performed-work receipt.
 pub fn new(work:&'a mut OriginalWorkState,birth:&'a mut OriginalFailureStorageBirth,pool:&'a mut OriginalFailureStoragePool)->Result<Self,ValueError>{
  if birth.phase!=OriginalFailureStorageBirthPhase::Allocate||birth.quoted.is_none()||birth.performed.is_some()||birth.work_claim.is_some()||birth.work!=0||work.reserved()!=1||work.performed_total_for_action(birth.action)?!=0{return Err(invalid("failure work requires its original untouched action"))}
  work.issue_performed_for_action(birth.action,&mut birth.performed)?;birth.phase=OriginalFailureStorageBirthPhase::Allocating;Ok(Self{work,birth,pool})
 }
 /// 🧵️ Lends the same sealed work receipt and original physical marker to the actual leaf.
 pub fn effect(&mut self)->RetainedAllocationEffect<'_>{let grant=self.birth.quoted.expect("original failure grant retained");RetainedAllocationEffect::new(grant,1,&mut self.birth.actual,self.birth.performed.as_mut().expect("original failure work receipt retained"))}
 /// 🪢️ Publishes into already borrowed original storage before any Source callback can run.
 pub fn allocate_and_publish(&mut self){
  let turn=OriginalFailureStorage::admit(&mut self.effect());match turn.result{Ok((node,mut progress))=>{self.pool.put(node);progress.copied_bytes+=OriginalFailureStorage::birth_demand().copy_bytes-OriginalFailureStorage::admission_demand().copy_bytes;self.birth.progress=progress},Err(error)=>{self.birth.progress=error.retained_progress();self.birth.primary=Some(error)}}self.birth.phase=OriginalFailureStorageBirthPhase::WorkPartial;
 }
}
impl Drop for OriginalFailurePerformedScope<'_>{
 fn drop(&mut self){let result=self.work.claim_performed_for_action(self.birth.action,&mut self.birth.performed).and_then(|()|self.work.performed_total_for_action(self.birth.action));if let Ok(total)=&result{self.birth.work=*total}*self.birth.work_claim=Some(result);if self.birth.phase==OriginalFailureStorageBirthPhase::Allocating&&self.birth.actual.is_none(){self.birth.phase=OriginalFailureStorageBirthPhase::WorkPartial}}
}
