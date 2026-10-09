//! 📥️ Original budgeted neuron input assembly with retained immutable dictionary entries.

use super::{Dictionary,Value,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use protocol::value::ordered::UpdateCursor;
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
    pub fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.progress=Default::default();let result=self.advance(grant);if let Ok(step)=&result{self.progress=step.progress()}result.map_err(|error|error.with_retained_progress(self.progress))}
    fn advance(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original input merge is closing"))}if self.is_complete(){return Ok(RetainedCloneStep::Complete(empty))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}
        if let Some(owner)=self.child.as_mut(){let result=owner.step(grant);self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.child=None;}return Ok(RetainedCloneStep::Progress(step.progress()))}
        let demand=self.next_demands(grant.maximum_copy_bytes)?;if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original input merge requires admitted depth"))}if grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_copy_bytes<demand.copy_bytes{return Ok(RetainedCloneStep::Progress(empty))}
        let progress=RetainedCloneProgress{copied_items:1,..empty};
        match self.phase {
            0=>{let overlay=self.owners.overlay.as_ref().unwrap();if self.rank==overlay.len(){self.phase=4;}else{let(key,value)=overlay.pairs.entry_shared_at_rank(self.rank).unwrap();self.owners.update=Some(self.owners.output.as_ref().unwrap().pairs.begin_set_shared(key,value));self.phase=1;}},
            1=>{let update=self.owners.update.as_mut().unwrap();let step=update.advance(grant)?;if update.is_complete(){self.phase=2;}return Ok(RetainedCloneStep::Progress(step.progress()));},
            2=>{let result=self.owners.update.as_mut().unwrap().take_result().unwrap();let displaced=std::mem::replace(&mut self.owners.output.as_mut().unwrap().pairs,result);self.child=Some(ControlledRetirement::new(InputClose::Dictionary(Dictionary{pairs:displaced})).unwrap_or_else(|_|unreachable!("original dictionary fields declare retirement")));self.phase=3;},
            3=>{self.child=Some(ControlledRetirement::new(InputClose::Update(self.owners.update.take().unwrap())).unwrap_or_else(|_|unreachable!("original update declares retirement")));self.rank+=1;self.phase=0;},
            _=>unreachable!(),
        }
        Ok(if self.is_complete(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)})
    }
    /// 📤️ Moves the same completed output to the original caller under an admitted handoff item.
    pub fn take_output(&mut self,grant:RetainedCloneGrant)->Result<(Option<Dictionary>,RetainedCloneProgress),ValueError>{if !self.is_complete()||grant.maximum_items==0{return Ok((None,Default::default()))}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original output handoff requires admitted depth"))}Ok((self.owners.output.take(),RetainedCloneProgress{copied_items:1,..Default::default()}))}
    /// 🛑️ Requests closure without decomposing or dropping any original source.
    pub fn begin_close(&mut self){self.closing=true;}
    pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=&self.child{return Self::demands(owner,copy)}if let Some(owner)=&self.closure{return Self::demands(owner,copy)}Ok(RetirementDemand{depth:usize::from(!self.terminal_is_empty()),..Default::default()})}
    /// 🧹️ Closes an actual child before handing off all remaining original fields.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.progress=Default::default();let result=self.close_source(grant);if let Ok(step)=&result{self.progress=step.progress()}result.map_err(|error|error.with_retained_progress(self.progress))}
    fn close_source(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original input close was not requested"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty))}
        if let Some(owner)=self.child.as_mut(){let result=owner.step(grant);self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.child=None;}return Ok(RetainedCloneStep::Progress(step.progress()))}
        if let Some(owner)=self.closure.as_mut(){let result=owner.step(grant);self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.closure=None;}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original input close handoff requires admitted depth"))}
        let fields=InputOwners{output:self.owners.output.take(),overlay:self.owners.overlay.take(),update:self.owners.update.take()};self.closure=Some(ControlledRetirement::new(fields).unwrap_or_else(|_|unreachable!("original input schema declares every field")));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))
    }
    pub fn terminal_is_empty(&self)->bool{self.closing&&self.child.is_none()&&self.closure.is_none()&&self.owners.output.is_none()&&self.owners.overlay.is_none()&&self.owners.update.is_none()}
}
impl Drop for BudgetedInputMerge{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original input merge must reach physical terminal emptiness");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.owners)}}}}
struct InputRetirement(BudgetedInputMerge);
impl RetirementCursor for InputRetirement {
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{match self.0.close_step(grant){Ok(step) if step.progress()!=RetainedCloneProgress::default()=>RetirementStep::Progress(step.progress()),Ok(RetainedCloneStep::Complete(_))=>RetirementStep::Complete,Ok(_)=>RetirementStep::BudgetExhausted,Err(error)=>RetirementStep::Failure(error)}}
    fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.copy_bytes)}
    fn allows_admitted_narrow_work(&self)->bool{true}
    fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.0.next_close_demands(copy).ok().map(|demand|demand.capacity_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.0.next_close_demands(0).ok().map(|demand|demand.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.depth)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl RetireOwned for BudgetedInputMerge {
    fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(InputRetirement(self))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<InputRetirement>())}
    fn controlled_retirement_supported()->bool{true}
}

#[derive(semio_framework_value::RetireOwned)]
struct OwnedEntryFields{base:Option<Dictionary>,key:Option<String>,value:Option<Value>,shared_key:Option<protocol::value::ordered::SharedOwner<String>>,shared_value:Option<protocol::value::ordered::SharedOwner<Value>>,update:Option<UpdateCursor<Value>>,child:Option<ControlledRetirement<InputClose>>}
/// 📝️ Admits one new original dictionary entry before its existing ordered update frontier.
pub struct BudgetedOwnedEntry{fields:ManuallyDrop<Option<OwnedEntryFields>>,retirement:Option<ControlledRetirement<OwnedEntryFields>>,phase:u8,closing:bool,progress:RetainedCloneProgress}
impl BudgetedOwnedEntry{
 /// 🌱️ Moves the original dictionary, key, and value inline with no allocation or destruction.
 pub fn new(base:Dictionary,key:String,value:Value)->Self{Self{fields:ManuallyDrop::new(Some(OwnedEntryFields{base:Some(base),key:Some(key),value:Some(value),shared_key:None,shared_value:None,update:None,child:None})),retirement:None,phase:0,closing:false,progress:Default::default()}}
 /// 🔗️ Moves an exact immutable source value lease while admitting only its new key header.
 pub fn from_shared_value(base:Dictionary,key:String,value:protocol::value::ordered::SharedOwner<Value>)->Self{Self{fields:ManuallyDrop::new(Some(OwnedEntryFields{base:Some(base),key:Some(key),value:None,shared_key:None,shared_value:Some(value),update:None,child:None})),retirement:None,phase:0,closing:false,progress:Default::default()}}
 /// 📥️ Moves the exact original shared key and value into the existing ordered update owner.
 pub fn from_shared(base:Dictionary,key:protocol::value::ordered::SharedOwner<String>,value:protocol::value::ordered::SharedOwner<Value>)->Self{Self{fields:ManuallyDrop::new(Some(OwnedEntryFields{base:Some(base),key:None,value:None,shared_key:Some(key),shared_value:Some(value),update:None,child:None})),retirement:None,phase:2,closing:false,progress:Default::default()}}
 pub fn step_progress(&self)->RetainedCloneProgress{self.progress}
 pub fn next_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if self.closing{return self.next_close_demands(copy)}let fields=self.fields.as_ref().unwrap();if let Some(child)=fields.child.as_ref(){return BudgetedInputMerge::demands(child,copy)}if self.phase==3{let update=fields.update.as_ref().unwrap();return Ok(RetirementDemand{copy_bytes:update.next_copy_byte_demand(),capacity_bytes:update.next_capacity_byte_demand()?,depth:update.next_depth_demand(),..Default::default()})}Ok(RetirementDemand{capacity_bytes:match self.phase{0=>protocol::value::ordered::SharedOwner::<String>::allocation_bytes(),1=>if fields.shared_value.is_none(){protocol::value::ordered::SharedOwner::<Value>::allocation_bytes()}else{0},_=>0},depth:1,..Default::default()})}
 /// ⏱️ Forwards the actual update or child receipt without cold header creation or discarded owners.
 pub fn step(&mut self,grant:RetainedCloneGrant)->Result<Option<Dictionary>,ValueError>{self.progress=Default::default();let result=self.advance(grant);result.map_err(|error|error.with_retained_progress(self.progress))}
 fn advance(&mut self,grant:RetainedCloneGrant)->Result<Option<Dictionary>,ValueError>{if self.closing{return Err(ValueError::literal(ValueRefusalKind::Canceled,"original owned entry is closing"))}if grant.maximum_items==0{return Ok(None)}let demand=self.next_demands(grant.maximum_copy_bytes)?;if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original owned entry exceeds admitted depth"))}if self.phase!=5&&self.phase!=7&&(grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes){return Ok(None)}let fields=self.fields.as_mut().unwrap();self.progress.copied_items=1;match self.phase{
  0=>match protocol::value::ordered::SharedOwner::admit(fields.key.take().unwrap(),grant){Ok((key,progress))=>{fields.shared_key=Some(key);self.progress=progress;self.phase=1;},Err((error,key))=>{fields.key=Some(key);self.progress=error.retained_progress();return Err(error)}},
  1=>if fields.shared_value.is_some(){self.phase=2;}else{match protocol::value::ordered::SharedOwner::admit(fields.value.take().unwrap(),grant){Ok((value,progress))=>{fields.shared_value=Some(value);self.progress=progress;self.phase=2;},Err((error,value))=>{fields.value=Some(value);self.progress=error.retained_progress();return Err(error)}}},
  2=>{fields.update=Some(fields.base.as_ref().unwrap().pairs.begin_set_shared(fields.shared_key.take().unwrap(),fields.shared_value.take().unwrap()));self.phase=3;},
  3=>{let update=fields.update.as_mut().unwrap();let result=update.advance(grant);self.progress=result.as_ref().map_or_else(|error|error.retained_progress(),|step|step.progress());result?;if update.is_complete(){self.phase=4;}},
  4=>{let result=fields.update.as_mut().unwrap().take_result().unwrap();let previous=std::mem::replace(&mut fields.base.as_mut().unwrap().pairs,result);fields.child=Some(ControlledRetirement::new(InputClose::Dictionary(Dictionary{pairs:previous})).unwrap_or_else(|_|unreachable!("original entry dictionary fields declare closure")));self.phase=5;},
  5|7=>{let child=fields.child.as_mut().unwrap();let result=child.step(grant);self.progress=child.step_progress();result?;if child.terminal_is_empty(){fields.child=None;self.phase+=1;}},
  6=>{fields.child=Some(ControlledRetirement::new(InputClose::Update(fields.update.take().unwrap())).unwrap_or_else(|_|unreachable!("original entry update fields declare closure")));self.phase=7;},
  8=>{self.phase=9;return Ok(fields.base.take())},
  _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original owned entry output was already handed off")),
 }Ok(None)}
 /// 🛑️ Preserves all accepted source owners for the original typed close frontier.
 pub fn begin_close(&mut self){self.closing=true;}
 pub fn next_close_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{if let Some(owner)=self.retirement.as_ref(){return BudgetedInputMerge::demands(owner,copy)}Ok(RetirementDemand{depth:usize::from(self.fields.is_some()),..Default::default()})}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.progress=Default::default();let result=self.close_source(grant);if let Ok(step)=&result{self.progress=step.progress()}result.map_err(|error|error.with_retained_progress(self.progress))}
 fn close_source(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.closing{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original owned entry close was not requested"))}if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(owner)=self.retirement.as_mut(){let result=owner.step(grant);self.progress=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.retirement=None;}return Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original owned entry handoff requires depth"))}self.retirement=Some(ControlledRetirement::new(self.fields.take().unwrap()).unwrap_or_else(|_|unreachable!("original entry source declares every field")));Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.fields.is_none()&&self.retirement.is_none()}
}
impl Drop for BudgetedOwnedEntry{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original owned entry must finish explicit closure");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.fields)}}}}
struct OwnedEntryRetirement(BudgetedOwnedEntry);
impl RetirementCursor for OwnedEntryRetirement{
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{match self.0.close_step(grant){Ok(step) if step.progress()!=RetainedCloneProgress::default()=>RetirementStep::Progress(step.progress()),Ok(RetainedCloneStep::Complete(_))=>RetirementStep::Complete,Ok(_)=>RetirementStep::BudgetExhausted,Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.0.terminal_is_empty()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.copy_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.0.next_close_demands(copy).ok().map(|demand|demand.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.0.next_close_demands(0).ok().map(|demand|demand.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.next_close_demands(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl RetireOwned for BudgetedOwnedEntry{
 fn retirement(mut self)->Box<dyn RetirementCursor>{self.begin_close();Box::new(OwnedEntryRetirement(self))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<OwnedEntryRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
