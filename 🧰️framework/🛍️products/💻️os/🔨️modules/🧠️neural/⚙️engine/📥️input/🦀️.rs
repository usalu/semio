//! 📥️ Original budgeted neuron input assembly with retained immutable dictionary entries.

use semio_os_kernel_neural_engine_model::retirement::{close_reply,CloseReply,CloseComplete};
use super::{Dictionary,Value,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,RetirementPollExit,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,controlled::ControlledRetirement};
use semio_framework_value_ordered::{OriginalEffectReply,ordered::{OriginalOwnedEntry,SharedOwner,UpdateCursor}};
use std::mem::ManuallyDrop;

#[derive(semio_framework_value::RetireOwned)]
enum InputClose {Dictionary(Dictionary),Update(UpdateCursor<Value>)}
#[derive(semio_framework_value::RetireOwned)]
struct InputOwners {output:Option<Dictionary>,overlay:Option<Dictionary>,update:Option<UpdateCursor<Value>>}

/// 📥️ Merges original immutable entries one admitted update event at a time.
pub struct BudgetedInputMerge {owners:ManuallyDrop<InputOwners>,child:Option<ControlledRetirement<InputClose>>,closure:Option<ControlledRetirement<InputOwners>>,rank:usize,phase:u8,closing:bool,progress:RetainedCloneProgress}
impl BudgetedInputMerge {
    /// 🌱️ Moves the exact original dictionary owners inline without creating or releasing allocations.
    pub fn new(base:Dictionary,overlay:Dictionary)->Self{Self{owners:ManuallyDrop::new(InputOwners{output:Some(base),overlay:Some(overlay),update:None}),child:None,closure:None,rank:0,phase:0,closing:false,progress:Default::default()}}
    pub fn output(&self)->Option<&Dictionary>{self.owners.output.as_ref()}
    pub fn overlay(&self)->Option<&Dictionary>{self.owners.overlay.as_ref()}
    pub fn is_complete(&self)->bool{!self.closing&&self.phase==4}
    pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
    fn demands<T:RetireOwned>(owner:&ControlledRetirement<T>,copy:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}
    /// 🪙️ Quotes the next event from the same retained input and update owners.
    pub fn next_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if self.closing{return self.next_close_demands(copy)}
        if let Some(owner)=&self.child{return Self::demands(owner,copy)}
        if let Some(update)=self.owners.update.as_ref(){if self.phase==1{return Ok(RetirementDemand{copy_bytes:update.next_copy_byte_demand(),capacity_bytes:update.next_capacity_byte_demand()?,depth:update.next_depth_demand(),..Default::default()})}}
        Ok(RetirementDemand{depth:usize::from(self.phase!=4),..Default::default()})
    }
    /// ⏱️ Advances one existing merge event and saves its actual physical receipt through refusal.
    pub fn step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{self.progress=Default::default();let result=self.advance(effect);if let Ok(step)=&result{self.progress=step.progress()}result.map_err(|exit|match exit{RetirementPollExit::Failed(failure)=>RetirementPollExit::Failed(failure.with_retained_progress(self.progress)),pending=>pending})}
    fn advance(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
        let grant=effect.grant();let empty=RetainedCloneProgress::default();if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original input merge is closing").into())}if self.is_complete(){return Ok(RetainedCloneStep::Complete(empty))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}
        if let Some(owner)=self.child.as_mut(){let result=RetirementPollExit::resume(owner.step(grant.maximum_copy_bytes,effect));self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.child=None;}return Ok(RetainedCloneStep::Progress(step.progress()))}
        let demand=self.next_demands(grant.maximum_copy_bytes)?;if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original input merge requires admitted depth").into())}if grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_copy_bytes<demand.copy_bytes{return Ok(RetainedCloneStep::Progress(empty))}
        let progress=RetainedCloneProgress{copied_items:1,..empty};
        let absent=|message:&'static str|RetirementPollExit::from(ValueError::invariant(message));
        match self.phase {
            0=>{let overlay=self.owners.overlay.as_ref().ok_or_else(||absent("overlay absent"))?;if self.rank==overlay.len(){self.phase=4;}else{let(key,value)=overlay.pairs().entry_shared_at_rank(self.rank).ok_or_else(||absent("overlay entry absent"))?;let output=self.owners.output.as_ref().ok_or_else(||absent("output absent"))?;self.owners.update=Some(output.pairs().begin_set_shared(key,value));self.phase=1;}},
            1=>{let update=self.owners.update.as_mut().ok_or_else(||absent("update absent"))?;let turn=update.advance(grant,effect.remaining_work_units()?);RetirementPollExit::pay(effect,turn.work)?;let step=turn.result?;if update.is_complete(){self.phase=2;}return Ok(RetainedCloneStep::Progress(step.progress()));},
            2=>{let result=self.owners.update.as_mut().and_then(UpdateCursor::take_result).ok_or_else(||absent("update result absent"))?;let output=self.owners.output.as_mut().ok_or_else(||absent("output absent"))?;let displaced=std::mem::replace(&mut output.pairs,result);self.child=Some(ControlledRetirement::retain(InputClose::Dictionary(Dictionary{pairs:displaced})));self.phase=3;},
            3=>{let update=self.owners.update.take().ok_or_else(||absent("update absent"))?;self.child=Some(ControlledRetirement::retain(InputClose::Update(update)));self.rank+=1;self.phase=0;},
            _=>return Err(absent("original input merge phase")),
        }
        Ok(if self.is_complete(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
    /// 📤️ Moves the same completed output to the original caller under an admitted handoff item.
    pub fn take_output(&mut self,grant:RetainedCloneGrant)->Result<(Option<Dictionary>,RetainedCloneProgress),ValueError>{if !self.is_complete()||grant.maximum_items==0{return Ok((None,Default::default()))}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original output handoff requires admitted depth"))}Ok((self.owners.output.take(),RetainedCloneProgress{copied_items:1,..Default::default()}))}
    /// 🛑️ Requests closure without decomposing or dropping any original source.
    pub fn begin_close(&mut self){self.closing=true;}
    pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=&self.child{return Self::demands(owner,copy)}if let Some(owner)=&self.closure{return Self::demands(owner,copy)}Ok(RetirementDemand{depth:usize::from(!self.terminal_is_empty()),..Default::default()})}
    /// ⛽️ Quotes the selected child step or the one field handoff.
    pub fn next_close_work(&self,body:usize)->Result<u64,ValueError>{if let Some(owner)=&self.child{return owner.next_computational_work(body)}if let Some(owner)=&self.closure{return owner.next_computational_work(body)}Ok(u64::from(!self.terminal_is_empty()))}
    /// 🧹️ Closes an actual child before handing off all remaining original fields.
    pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
        self.progress=Default::default();let grant=effect.grant();let empty=RetainedCloneProgress::default();
        let result:Result<RetainedCloneStep,RetirementPollExit>=(||{
            if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original input close was not requested").into())}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}
            if let Some(owner)=self.child.as_mut(){let result=RetirementPollExit::resume(owner.step(body,effect));self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.child=None;}return Ok(RetainedCloneStep::Progress(step.progress()))}
            if let Some(owner)=self.closure.as_mut(){let result=RetirementPollExit::resume(owner.step(body,effect));self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.closure=None;}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
            if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original input close handoff requires admitted depth").into())}
            if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original input close handoff exceeds its work ceiling").into())}
            let fields=InputOwners{output:self.owners.output.take(),overlay:self.owners.overlay.take(),update:self.owners.update.take()};self.closure=Some(ControlledRetirement::retain(fields));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))
        })();
        let poll=RetirementPollExit::settle(effect,result);
        if let semio_framework_value::RetainedWorkPoll::Complete(turn)=&poll{self.progress=turn.result.as_ref().map_or_else(|failure|failure.retained_progress(),|step|step.progress())}
        poll
    }
    pub fn terminal_is_empty(&self)->bool{self.closing&&self.child.is_none()&&self.closure.is_none()&&self.owners.output.is_none()&&self.owners.overlay.is_none()&&self.owners.update.is_none()}
}
impl Drop for BudgetedInputMerge{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original input merge must reach physical terminal emptiness");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owners)}}}}
/// 🧹️ Owns a closing input merge as its retirement cursor.
pub struct InputRetirement(BudgetedInputMerge);
impl RetirementCursor for InputRetirement {
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, semio_framework_value::ValueError> { self.0.next_close_demands(body) }
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.0.next_close_work(body)}
    fn close_into<'reply>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:CloseReply<'reply>)->Result<CloseComplete<'reply>,ValueError>{close_reply(self.0.close_step(body,effect),effect,reply)}
    fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.copy_bytes)}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.0.next_close_demands(copy).ok().map(|demand|demand.capacity_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.0.next_close_demands(0).ok().map(|demand|demand.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.depth)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl RetireOwned for BudgetedInputMerge {
    type RetirementCursor=InputRetirement;
    fn retirement_seed(mut self)->InputRetirement{self.begin_close();InputRetirement(self)}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<InputRetirement>())}
    fn controlled_retirement_supported()->bool{true}
}

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

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
