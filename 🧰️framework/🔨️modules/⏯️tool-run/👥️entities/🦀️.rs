//! 👥️ Caller-owned ToolRun entity edit workspace and exact original retirement.
use semio_framework_value::{OriginalRetirementFailure,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,RetainedAllocationEffect,RetainedWorkPoll,RetainedClone,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,retained_clone_grant_admits,ordered_map::{BoundedOrd,BoundedOrdGrant,BoundedOrdProgress,RetainedOrderedMap,RetainedOrderedMapInsertCursor,RetainedOrderedMapInsertGrant,RetainedOrderedMapRemoveCursor,RetainedOrderedMapRemoveGrant,RETAINED_ORDERED_MAP_PAGE_CAPACITY}},retirement::RetireOwned};
use semio_framework_value_shared_issuance::{SealedShared,SharedIssuer};
use std::mem::{ManuallyDrop,size_of};

pub type EntityMap=RetainedOrderedMap<u64,()>;
pub type EntityReply<'r>=OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>;
pub type EntityComplete<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>;

/// 🏁️ Settles one finished retirement poll into its physical receipt; a refused or suspended close is a fault of this workspace.
pub fn settle_close(poll:semio_framework_value::OriginalRetirementPoll)->Result<RetainedCloneProgress,ValueError>{match poll{RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(step)=>Ok(step.progress()),Err(failure)=>{let progress=failure.retained_progress();drop(failure);Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"entity workspace close was refused").with_retained_progress(progress))}},RetainedWorkPoll::Pending{..}=>Err(ValueError::invariant("entity workspace close stayed pending"))}}

/// 🧯️ Records the fault of a replay, a changed route or a misplaced phase as an invariant refusal.
pub fn route_fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}

/// 💤️ Completes a reply with a denied, unpaid frontier so that the caller retries under a larger grant.
pub fn denied<'r>(reply:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>{Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}

/// ⏱️ Reports whether the original effect admits one quoted demand and its original work.
pub fn funded(effect:&RetainedAllocationEffect<'_>,demand:RetirementDemand,work:u64)->Result<bool,ValueError>{Ok(retained_clone_grant_admits(effect.grant(),demand)&&effect.remaining_work_units()?>=work)}

/// 🪜️ Performs one funded local transition on the original effect and records its physical receipt in the reply.
pub fn local_step<'r>(effect:&mut RetainedAllocationEffect<'_>,demand:RetirementDemand,work:u64,reply:EntityReply<'r>,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>)->Result<EntityComplete<'r>,ValueError>{
 if !funded(effect,demand,work)?{return denied(reply);}
 if !effect.performed(work)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"entity workspace transition exceeded original work").into())));}
 Ok(reply.complete(run(effect).map(RetainedCloneStep::Progress).map_err(Into::into)))
}

/// 🧱️ Reserves one exact vector backing as the single allocation attempt of this effect and records its genuine size.
pub fn reserve_vec<T>(count:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<Vec<T>,ValueError>{
 let actual=effect.actual();if actual.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"entity workspace allocation attempt is occupied"));}
 let mut vector=Vec::new();let reserved=vector.try_reserve_exact(count);*actual=Some(if reserved.is_ok(){vector.capacity()*size_of::<T>()}else{0});
 reserved.map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"entity workspace backing allocation failed"))?;Ok(vector)
}

/// 🏷️ Admits one sealed original from its caller slot under the original effect; an unfunded or refused admission hands the value back.
pub fn admit_sealed<T:RetireOwned+Send+Sync+'static>(value:T,effect:&mut RetainedAllocationEffect<'_>)->Result<(SealedShared<T>,RetainedCloneProgress),(ValueError,T)>{
 let mut input=Some(value);let mut output=None;
 let turn=SealedShared::admit_from(&mut input,SharedIssuer::owned(),&mut output,effect);
 match(turn.result,output){
  (Ok(step),Some(sealed))=>Ok((sealed,step.progress())),
  (Ok(_),None)=>Err((ValueError::literal(ValueRefusalKind::WorkLimit,"entity sealed admission is not funded"),input.take().expect("unfunded sealed admission retains its input"))),
  (Err(error),output)=>{drop(output);Err((error,input.take().expect("refused sealed admission retains its input")))}
 }
}

/// 🪞️ Duplicates one sealed alias; the alias turn debits its own quoted work on the effect exactly once, a refused quote duplicates nothing.
pub fn duplicate_sealed<T:Send+Sync+'static>(source:&SealedShared<T>,effect:&mut RetainedAllocationEffect<'_>)->Result<(SealedShared<T>,RetainedCloneProgress),ValueError>{
 source.try_duplicate(effect).result
}

/// 📏️ Quotes the single frontier of an empty birth followed by its publication.
pub fn publication_demand<T:Send+Sync+'static>()->Result<RetirementDemand,ValueError>{let birth=SealedShared::<T>::prepare_birth_demand()?;let publication=semio_framework_value_shared_issuance::SealedSharedBirth::<T>::publication_demand();Ok(RetirementDemand{copy_bytes:birth.copy_bytes.max(publication.copy_bytes),capacity_bytes:birth.capacity_bytes,release_bytes:birth.release_bytes.max(publication.release_bytes),depth:birth.depth.max(publication.depth)})}

/// 📤️ Publishes one sealed original through a funded birth and publication; an unfunded frontier yields nothing and builds no value.
pub fn publish_sealed<T:RetireOwned+Send+Sync+'static>(effect:&mut RetainedAllocationEffect<'_>,value:impl FnOnce()->Result<T,ValueError>)->Result<Option<(SealedShared<T>,RetainedCloneProgress)>,ValueError>{
 let demand=publication_demand::<T>()?;
 if !funded(effect,demand,SealedShared::<T>::prepare_birth_work_demand()+semio_framework_value_shared_issuance::SealedSharedBirth::<T>::publication_work_demand())?{return Ok(None);}
 let mut slot=None;let turn=SealedShared::<T>::prepare_birth(&mut slot,effect);let born=turn.result?.progress();
 let Some(mut birth)=slot else{return Ok(None);};
 let mut input=match value(){Ok(value)=>Some(value),Err(error)=>{close_birth(&mut birth,effect)?;return Err(error);}};
 let mut output=None;let turn=birth.publish(&mut input,SharedIssuer::owned(),&mut output,effect);
 match(turn.result,output){
  (Ok(step),Some(shared))=>{let mut progress=step.progress();progress.retained_capacity_bytes=born.retained_capacity_bytes;Ok(Some((shared,progress)))},
  (Ok(_),None)=>{close_birth(&mut birth,effect)?;Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"sealed publication is not funded"))},
  (Err(error),_)=>{close_birth(&mut birth,effect)?;Err(error)}
 }
}

fn close_birth<T:Send+Sync+'static>(birth:&mut semio_framework_value_shared_issuance::SealedSharedBirth<T>,effect:&mut RetainedAllocationEffect<'_>)->Result<(),ValueError>{birth.close_step(0,effect).result.map(drop)}

pub use semio_framework_value_source_hold as source_hold;
#[path="📸️version/🦀️.rs"]
pub mod version;
#[path="🧾️provenance/🦀️.rs"]
pub mod provenance;
#[derive(Clone,Copy,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub enum ToolRunEntityEditKind {Append,Retract}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct ToolRunEntityEditGrant {pub retirement:RetainedCloneGrant,pub comparison:BoundedOrdGrant,pub maximum_moved_items:usize,pub maximum_moved_bytes:usize,pub maximum_capacity_bytes:usize}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct ToolRunEntityEditProgress {pub retirement:RetainedCloneProgress,pub comparison:BoundedOrdProgress,pub moved_items:usize,pub moved_bytes:usize}
impl ToolRunEntityEditProgress {pub fn fits(self,grant:ToolRunEntityEditGrant)->bool {self.retirement.fits(grant.retirement)&&self.comparison.fits(grant.comparison)&&self.moved_items<=grant.maximum_moved_items&&self.moved_bytes<=grant.maximum_moved_bytes}}
const IDLE:u8=0;const ADVANCING:u8=1;const TAKING:u8=2;const CLOSING_BEGIN:u8=3;const CLOSING:u8=4;const DONE:u8=5;
#[derive(semio_framework_value::RetireOwned)]
enum Editor<K:BoundedOrd+RetainedClone+Copy> {Append(RetainedOrderedMapInsertCursor<K,()>),Retract(RetainedOrderedMapRemoveCursor<K,()>)}
impl<K:BoundedOrd+RetainedClone+Copy> Editor<K> {
 fn insert(grant:ToolRunEntityEditGrant)->RetainedOrderedMapInsertGrant{RetainedOrderedMapInsertGrant{retirement:grant.retirement,comparison:grant.comparison,maximum_moved_items:grant.maximum_moved_items,maximum_moved_bytes:grant.maximum_moved_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes}}
 fn remove(grant:ToolRunEntityEditGrant)->RetainedOrderedMapRemoveGrant{RetainedOrderedMapRemoveGrant{retirement:grant.retirement,comparison:grant.comparison,maximum_moved_items:grant.maximum_moved_items,maximum_moved_bytes:grant.maximum_moved_bytes}}
 fn output_ready(&self)->bool{match self{Self::Append(owner)=>owner.output_ready()||owner.refused_workspace_ready(),Self::Retract(owner)=>owner.output_ready()}}
 fn closing(&self)->bool{self.begin_close_work_demand()==0}
 fn terminal_is_empty(&self)->bool{match self{Self::Append(owner)=>owner.terminal_is_empty(),Self::Retract(owner)=>owner.terminal_is_empty()}}
 fn take_demand(&self)->RetirementDemand{match self{Self::Append(owner)=>owner.take_output_demand(),Self::Retract(owner)=>owner.take_output_demand()}}
 fn begin_close_demand(&self)->RetirementDemand{match self{Self::Append(owner)=>owner.begin_close_demand(),Self::Retract(owner)=>owner.begin_close_demand()}}
 fn begin_close_work_demand(&self)->u64{match self{Self::Append(owner)=>owner.begin_close_work_demand(),Self::Retract(owner)=>owner.begin_close_work_demand()}}
 fn advance_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let quote=|body:usize|match self{Self::Append(owner)=>owner.advance_demands(body,owner.next_advance_demands(body)?),Self::Retract(owner)=>owner.advance_demands(body,owner.next_advance_demands(body)?)};
  let first=quote(body)?;quote(body.max(first.copy_bytes))
 }
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{match self{Self::Append(owner)=>owner.close_demands(body),Self::Retract(owner)=>owner.close_demands(body)}}
 fn advance_into<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>,recover:bool)->Result<EntityComplete<'r>,ValueError>{match self{
  Self::Append(owner)=>if recover{owner.recover_advance_into(body,Self::insert(grant),effect,reply)}else{owner.advance_into(body,Self::insert(grant),effect,reply)},
  Self::Retract(owner)=>if recover{owner.recover_advance_into(body,Self::remove(grant),effect,reply)}else{owner.advance_into(body,Self::remove(grant),effect,reply)}}}
 fn take_into<'r>(&mut self,output:&mut Option<RetainedOrderedMap<K,()>>,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>{match self{Self::Append(owner)=>if owner.refused_workspace_ready(){owner.take_refused_workspace_into(output,effect,reply)}else{owner.take_into(output,effect,reply)},Self::Retract(owner)=>owner.take_into(output,effect,reply)}}
 fn begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>{match self{Self::Append(owner)=>owner.begin_close_into(effect,reply),Self::Retract(owner)=>owner.begin_close_into(effect,reply)}}
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>,recover:bool)->Result<EntityComplete<'r>,ValueError>{match self{
  Self::Append(owner)=>if recover{owner.recover_close_into(body,effect,reply)}else{owner.close_into(body,effect,reply)},
  Self::Retract(owner)=>if recover{owner.recover_close_into(body,effect,reply)}else{owner.close_into(body,effect,reply)}}}
}
#[derive(semio_framework_value::RetireOwned)]
#[doc(hidden)]
pub struct State<K:BoundedOrd+RetainedClone+Copy> {map:Option<RetainedOrderedMap<K,()>>,targets:Vec<K>,index:usize,kind:ToolRunEntityEditKind,editor:Option<Editor<K>>,phase:u8,route:Option<u8>,capacity_bound:usize,cancelled:bool}
impl<K:BoundedOrd+RetainedClone+Copy> State<K> {
 fn phase_now(&self)->u8{match self.phase{ADVANCING if self.editor.as_ref().is_some_and(Editor::output_ready)=>TAKING,TAKING if self.map.is_some()=>CLOSING_BEGIN,CLOSING_BEGIN if self.editor.as_ref().is_some_and(Editor::closing)=>CLOSING,phase=>phase}}
}
/// 👥️ Owns genuine map leaves and target backing until a funded transfer or close; every turn is a reply-shaped original effect.
pub struct ToolRunEntityEdit<K:BoundedOrd+RetainedClone+Copy=u64> {state:ManuallyDrop<State<K>>,transferred:bool}
impl<K:BoundedOrd+RetainedClone+Copy> ToolRunEntityEdit<K> {
 pub fn capacity_bound_for(entries:usize,targets:usize)->Option<usize>{entries.checked_add(targets).and_then(|len|len.checked_add(RETAINED_ORDERED_MAP_PAGE_CAPACITY-1)).map(|len|len/RETAINED_ORDERED_MAP_PAGE_CAPACITY).and_then(|pages|pages.checked_mul(2)).and_then(|pages|pages.checked_add(1)).and_then(|pages|pages.checked_mul(size_of::<Vec<(K,())>>())).map(|bytes|bytes.max(RETAINED_ORDERED_MAP_PAGE_CAPACITY*size_of::<(K,())>()))}
 pub fn new(map:RetainedOrderedMap<K,()>,targets:Vec<K>,kind:ToolRunEntityEditKind)->Self{let capacity_bound=Self::capacity_bound_for(map.len(),targets.len()).unwrap_or(usize::MAX);Self{state:ManuallyDrop::new(State{map:Some(map),targets,index:0,kind,editor:None,phase:IDLE,route:None,capacity_bound,cancelled:false}),transferred:false}}
 pub fn targets_identity(&self)->usize{self.state.targets.as_ptr()as usize}
 pub fn edit_capacity_bound(&self)->Result<usize,ValueError>{if self.state.capacity_bound==usize::MAX{Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"entity edit capacity overflow"))}else{Ok(self.state.capacity_bound)}}
 pub fn complete(&self)->bool{self.state.phase_now()==DONE&&!self.state.cancelled}
 pub fn cancel(&mut self){self.state.cancelled=true;}
 pub fn construct_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<RetainedOrderedMap<K,()>>()+size_of::<K>(),depth:1,..Default::default()}}
 pub fn take_output_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<RetainedOrderedMap<K,()>>()+size_of::<Option<RetainedOrderedMap<K,()>>>(),depth:1,..Default::default()}}
 pub fn next_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let state=&*self.state;let phase=state.phase_now();if state.cancelled||phase==DONE{return Ok(Default::default());}
  let editor=||state.editor.as_ref().ok_or_else(||route_fault("entity editor is absent"));
  match phase{
   IDLE=>Ok(if state.index==state.targets.len(){RetirementDemand{depth:1,..Default::default()}}else{Self::construct_demand()}),
   ADVANCING=>editor()?.advance_demands(body),
   TAKING=>Ok(editor()?.take_demand()),
   CLOSING_BEGIN=>Ok(editor()?.begin_close_demand()),
   CLOSING=>{let owner=editor()?;if owner.terminal_is_empty(){Ok(RetirementDemand{depth:1,..Default::default()})}else{owner.close_demands(body)}},
   _=>Err(route_fault("entity edit phase is invalid"))
  }
 }
 pub fn advance_into<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>{self.drive(body,grant,effect,reply,false)}
 pub fn recover_advance_into<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>{self.drive(body,grant,effect,reply,true)}
 fn drive<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>,recover:bool)->Result<EntityComplete<'r>,ValueError>{
  if self.state.route.is_some()!=recover{return Err(route_fault("entity edit changed its original recovery route"));}
  self.state.phase=self.state.phase_now();
  if !recover{
   if grant.retirement.maximum_items==0||self.state.cancelled||self.state.phase==DONE{return denied(reply);}
   let demand=self.next_demands(body)?;if !retained_clone_grant_admits(grant.retirement,demand){return denied(reply);}
  }
  let state=&mut *self.state;let phase=state.phase;
  match phase{
   IDLE=>if state.index==state.targets.len(){
    local_step(effect,RetirementDemand{depth:1,..Default::default()},1,reply,|_|{state.phase=DONE;Ok(RetainedCloneProgress{copied_items:1,..Default::default()})})
   }else{
    let demand=Self::construct_demand();
    local_step(effect,demand,1,reply,|_|{
     let target=state.targets[state.index];let map=state.map.take().ok_or_else(||route_fault("entity editor requires its original map"))?;
     state.editor=Some(match state.kind{ToolRunEntityEditKind::Append=>Editor::Append(RetainedOrderedMapInsertCursor::new(map,target,())),ToolRunEntityEditKind::Retract=>Editor::Retract(RetainedOrderedMapRemoveCursor::new(map,target))});
     state.phase=ADVANCING;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})
    })
   },
   ADVANCING=>{let editor=state.editor.as_mut().ok_or_else(||route_fault("entity editor is absent"))?;let mut scope=reply.scope(&mut state.route,phase)?;drop(editor.advance_into(body,grant,effect,scope.borrow()?,recover)?);Ok(scope.forwarded())},
   TAKING=>{if recover{return Err(route_fault("entity editor take has no unrecorded callback"));}let editor=state.editor.as_mut().ok_or_else(||route_fault("entity editor is absent"))?;let mut scope=reply.scope(&mut state.route,phase)?;drop(editor.take_into(&mut state.map,effect,scope.borrow()?)?);Ok(scope.forwarded())},
   CLOSING_BEGIN=>{if recover{return Err(route_fault("entity editor close start has no unrecorded callback"));}let editor=state.editor.as_mut().ok_or_else(||route_fault("entity editor is absent"))?;let mut scope=reply.scope(&mut state.route,phase)?;drop(editor.begin_close_into(effect,scope.borrow()?)?);Ok(scope.forwarded())},
   CLOSING=>{
    let editor=state.editor.as_mut().ok_or_else(||route_fault("entity editor is absent"))?;
    if editor.terminal_is_empty(){local_step(effect,RetirementDemand{depth:1,..Default::default()},1,reply,|_|{state.editor=None;state.index+=1;state.phase=IDLE;Ok(RetainedCloneProgress{copied_items:1,..Default::default()})})}
    else{let mut scope=reply.scope(&mut state.route,phase)?;drop(editor.close_into(body,effect,scope.borrow()?,recover)?);Ok(scope.forwarded())}
   },
   _=>Err(route_fault("entity edit phase is invalid"))
  }
 }
 pub fn take_output_into<'r>(&mut self,output:&mut Option<RetainedOrderedMap<K,()>>,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>{
  if !self.complete()||self.state.map.is_none()||output.is_some()||self.state.route.is_some(){return Err(route_fault("entity edit output places changed"));}
  let state=&mut *self.state;let demand=Self::take_output_demand();
  local_step(effect,demand,1,reply,|_|{*output=state.map.take();Ok(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<RetainedOrderedMap<K,()>>(),..Default::default()})})
 }
}
impl<K:BoundedOrd+RetainedClone+Copy> RetireOwned for ToolRunEntityEdit<K> {
 type RetirementCursor=<State<K> as RetireOwned>::RetirementCursor;
 fn retirement_seed(mut self)->Self::RetirementCursor{self.transferred=true;unsafe{ManuallyDrop::take(&mut self.state)}.retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.state.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{true}
 fn retirement_element_copy_bytes()->usize{size_of::<Self>()}
}
impl<K:BoundedOrd+RetainedClone+Copy> Drop for ToolRunEntityEdit<K> {fn drop(&mut self){assert!(self.transferred,"original entity edit workspace requires controlled retirement");}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[cfg(test)]
#[global_allocator]
static TOOL_RUN_ENTITIES_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
