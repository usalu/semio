//! 📝️ Original retained dictionary entry producer.
use super::{Dictionary,Value};
use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError};
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value_ordered::{OriginalEffectReply,ordered::{OriginalOwnedEntry,SharedOwner}};

type EntryMap=semio_framework_value_ordered::ordered::OrderedMap<Value>;
type EntryAnswer=Option<Result<RetainedCloneStep,OriginalRetirementFailure>>;
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
#[doc(hidden)]
pub struct EntryOwners{entry:OriginalOwnedEntry<Value>,output:Option<EntryMap>}

/// 📝️ Dictionary entry construction uses the canonical original ordered-map producer and keeps its reply slot across suspended turns.
pub struct BudgetedOwnedEntry{owners:EntryOwners,answer:EntryAnswer,suspended:bool,progress:RetainedCloneProgress}
impl BudgetedOwnedEntry{
 fn seed(entry:OriginalOwnedEntry<Value>)->Self{Self{owners:EntryOwners{entry,output:None},answer:None,suspended:false,progress:Default::default()}}
 /// 🌱️ Moves the actual original dictionary and entry into the shared producer.
 pub fn new(base:Dictionary,key:String,value:Value)->Self{Self::seed(OriginalOwnedEntry::new(base.pairs,key,value))}
 /// 🔗️ Moves the exact source value lease while admitting its original key header.
 pub fn from_shared_value(base:Dictionary,key:String,value:SharedOwner<Value>)->Self{Self::seed(OriginalOwnedEntry::from_shared_value(base.pairs,key,value))}
 /// 📥️ Moves the actual original shared entry into the same update owner.
 pub fn from_shared(base:Dictionary,key:SharedOwner<String>,value:SharedOwner<Value>)->Self{Self::seed(OriginalOwnedEntry::from_shared(base.pairs,key,value))}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn next_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{self.owners.entry.next_demands(copy)}
 pub fn next_work_demand(&self,copy:usize)->Result<u64,ValueError>{self.owners.entry.next_work_demand(copy)}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{self.owners.entry.next_close_demands(copy)}
 pub fn next_close_work_demand(&self,copy:usize)->Result<u64,ValueError>{self.owners.entry.next_close_work_demand(copy)}
 fn settle<R>(&mut self,work:u128,result:Result<(),ValueError>,done:impl FnOnce(&mut Self,RetainedCloneStep)->R)->RetainedWorkPoll<R,OriginalRetirementFailure>{
  let outcome=|result:Result<R,OriginalRetirementFailure>|RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work});
  if let Err(error)=result{return outcome(Err(error.into()))}
  match self.answer.take(){None=>{self.suspended=true;RetainedWorkPoll::Pending{work}},Some(Err(failure))=>{self.suspended=false;outcome(Err(failure.with_retained_progress(self.progress)))},Some(Ok(step))=>{self.suspended=false;self.progress=step.progress();outcome(Ok(done(self,step)))}}
 }
 /// ⏱️ Publishes the exact map as a dictionary without cloning source or replacement ownership; a suspended turn keeps the same producer reply for recovery.
 pub fn step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<Option<Dictionary>,OriginalRetirementFailure>{
  self.progress=Default::default();let before=effect.performed_boundary();
  let call=(||->Result<(),ValueError>{if self.answer.is_none(){let reply=OriginalEffectReply::borrow(&mut self.answer)?;let owners=&mut self.owners;drop(if self.suspended{owners.entry.recover_advance_into(body,&mut owners.output,effect,reply)?}else{owners.entry.advance_into(body,&mut owners.output,effect,reply)?});}Ok(())})();
  let work=effect.performed_since(before).expect("same entry receipt");self.settle(work,call,|this,step|if matches!(step,RetainedCloneStep::Complete(_)){this.owners.output.take().map(|pairs|Dictionary{pairs})}else{None})
 }
 /// 🧹️ Closes every remaining original owner of the entry, keeping the producer reply across suspended turns.
 pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  self.progress=Default::default();let before=effect.performed_boundary();
  let call=(||->Result<(),ValueError>{if self.answer.is_none(){let reply=OriginalEffectReply::borrow(&mut self.answer)?;let entry=&mut self.owners.entry;drop(if self.suspended{entry.recover_close_into(body,effect,reply)?}else{entry.close_into(body,effect,reply)?});}Ok(())})();
  let work=effect.performed_since(before).expect("same entry receipt");self.settle(work,call,|_,step|step)
 }
 pub fn terminal_is_empty(&self)->bool{self.owners.entry.terminal_is_empty()&&self.owners.output.is_none()&&self.answer.is_none()}
}
impl RetireOwned for BudgetedOwnedEntry{
 type RetirementCursor=<EntryOwners as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{self.owners.retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.owners.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{<EntryOwners as RetireOwned>::controlled_retirement_supported()}
}

