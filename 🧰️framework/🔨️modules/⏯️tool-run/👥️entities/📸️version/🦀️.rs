//! 📸️ Original immutable entity versions publish through caller-funded private edit workspaces.
use super::{publication_demand,admit_sealed,denied,duplicate_sealed,funded,local_step,publish_sealed,route_fault,source_hold::SourceHold,EntityComplete,EntityMap,EntityReply,ToolRunEntityEdit,ToolRunEntityEditGrant,ToolRunEntityEditKind};
use semio_framework_value::{OriginalRetirementFailure,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{OriginalEffectReply,OriginalEffectReplyComplete,OriginalProjectionReply,OriginalSourceProjection,RetainedAllocationEffect,RetainedClone,RetainedCloneCursor,RetainedCloneProgress,retained_clone_grant_admits},retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value::retained_clone::ordered_map::RetainedOrderedMapCloneCursor;
use std::ptr::NonNull;
use semio_framework_value_shared_issuance::SealedShared;
use std::mem::{ManuallyDrop,size_of};

#[derive(semio_framework_value::RetireOwned)]
pub struct ToolRunEntityVersion {entities:SealedShared<EntityMap>,revision:u64}
impl ToolRunEntityVersion {
 pub fn birth_bytes()->usize{SealedShared::<EntityMap>::birth_bytes()}
 pub fn capture_copy_bytes()->usize{size_of::<Self>()}
 pub fn admit(map:EntityMap,revision:u64,effect:&mut RetainedAllocationEffect<'_>)->Result<(Self,RetainedCloneProgress),(ValueError,EntityMap)>{admit_sealed(map,effect).map(|(entities,progress)|(Self{entities,revision},progress))}
 pub fn capture(&self,effect:&mut RetainedAllocationEffect<'_>)->Result<(Self,RetainedCloneProgress),ValueError>{let grant=effect.grant();if grant.maximum_copy_bytes<Self::capture_copy_bytes(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"entity version capture requires its original fixed-body copy"));}let(entities,mut progress)=duplicate_sealed(&self.entities,effect)?;progress.copied_bytes=Self::capture_copy_bytes();Ok((Self{entities,revision:self.revision},progress))}
 pub fn entities(&self)->&EntityMap{self.entities.get()}
 pub fn revision(&self)->u64{self.revision}
 pub fn identity(&self)->usize{self.entities.identity()}
}
impl OriginalSourceProjection<EntityMap> for ToolRunEntityVersion{
 fn projection_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn project_source<'reply,'source>(&'source self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,EntityMap>)->Result<OriginalEffectReplyComplete<'reply,NonNull<EntityMap>,ValueError>,ValueError>{if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"entity version projection exceeded its original work"))));}Ok(reply.complete(Ok(self.entities())))}
 fn recover_source_projection<'reply,'source>(&'source self,_:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,EntityMap>)->Result<OriginalEffectReplyComplete<'reply,NonNull<EntityMap>,ValueError>,ValueError>{Ok(reply.pending())}
}
const ADMIT:u8=0;const CLONE:u8=1;const TAKE_CLONE:u8=2;const BEGIN_CLONE:u8=3;const CLOSE_CLONE:u8=4;const DROP_CLONE:u8=5;const CONSTRUCT:u8=6;const ADVANCE:u8=7;const TAKE:u8=8;const WRAP:u8=9;const CLOSE:u8=10;const DROP:u8=11;const DONE:u8=12;const PUBLISHED:u8=13;
type Hold=SourceHold<EntityMap,ToolRunEntityVersion>;
#[doc(hidden)]
pub struct State {hold:Hold,clone:Option<RetainedOrderedMapCloneCursor<u64,()>>,base:Option<EntityMap>,targets:Option<Vec<u64>>,edit:Option<ToolRunEntityEdit>,edit_close:Option<ControlledRetirement<ToolRunEntityEdit>>,candidate:Option<EntityMap>,kind:ToolRunEntityEditKind,next_revision:Option<u64>,capacity_bound:usize,phase:u8,route:Option<u8>,cancelled:bool}
semio_framework_value::artifact_retire_struct!(State{hold,clone,base,targets,edit,edit_close,candidate,kind,next_revision,capacity_bound,phase,route,cancelled});
impl State {
 fn phase_now(&self)->u8{match self.phase{
  ADMIT if self.hold.ready()=>CLONE,
  CLONE if self.clone.as_ref().is_some_and(RetainedCloneCursor::output_ready)=>TAKE_CLONE,
  TAKE_CLONE if self.base.is_some()=>BEGIN_CLONE,
  BEGIN_CLONE if self.clone.as_ref().is_some_and(RetainedCloneCursor::close_started)=>CLOSE_CLONE,
  CLOSE_CLONE if self.clone.as_ref().is_some_and(RetainedCloneCursor::terminal_is_empty)=>DROP_CLONE,
  ADVANCE if self.edit.as_ref().is_some_and(ToolRunEntityEdit::complete)=>TAKE,
  TAKE if self.candidate.is_some()=>WRAP,
  CLOSE if self.edit_close.as_ref().is_some_and(ControlledRetirement::terminal_is_empty)=>DROP,
  phase=>phase}}
}
/// 🏭️ Keeps each original version under its admitted Source until real publication or controlled closure; the candidate is the retained clone of the original, edited by the targets alone.
pub struct ToolRunEntityVersionEdit {state:ManuallyDrop<State>,transferred:bool}
impl ToolRunEntityVersionEdit {
 pub fn new(original:ToolRunEntityVersion,targets:Vec<u64>,kind:ToolRunEntityEditKind)->Self{
  let capacity_bound=ToolRunEntityEdit::<u64>::capacity_bound_for(original.entities().len(),targets.len()).unwrap_or(usize::MAX);let next_revision=original.revision.checked_add(1);
  Self{state:ManuallyDrop::new(State{hold:SourceHold::new(original),clone:Some(EntityMap::retained_clone_cursor()),base:None,targets:Some(targets),edit:None,edit_close:None,candidate:None,kind,next_revision,capacity_bound,phase:ADMIT,route:None,cancelled:false}),transferred:false}
 }
 pub fn complete(&self)->bool{self.state.phase_now()==DONE&&!self.state.cancelled}
 /// ✏️ Reports whether the retained clone is complete and only the target edit remains.
 pub fn editing(&self)->bool{self.state.phase_now()>=CONSTRUCT}
 pub fn cancel(&mut self){self.state.cancelled=true;if let Some(edit)=self.state.edit.as_mut(){edit.cancel();}}
 pub fn edit_capacity_bound(&self)->Result<usize,ValueError>{if self.state.capacity_bound==usize::MAX{Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"entity version edit capacity overflow"))}else{Ok(self.state.capacity_bound)}}
 pub fn publication_demands(&self)->RetirementDemand{publication_demand::<EntityMap>().unwrap_or_default()}
 pub fn next_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let state=&*self.state;let phase=state.phase_now();if state.cancelled||phase>=DONE{return Ok(Default::default());}
  if state.next_revision.is_none(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"entity version revision overflow"));}
  let clone=||state.clone.as_ref().ok_or_else(||route_fault("entity version clone is absent"));let edit=||state.edit.as_ref().ok_or_else(||route_fault("entity version edit is absent"));
  Ok(match phase{
   ADMIT=>state.hold.quote(body)?,
   CLONE=>clone()?.advance_demands(state.hold.original()?,body,u64::MAX)?,
   TAKE_CLONE=>clone()?.take_demand()?,
   BEGIN_CLONE=>clone()?.begin_close_demand()?,
   CLOSE_CLONE=>clone()?.close_demands(body)?,
   DROP_CLONE=>RetirementDemand{depth:1,..Default::default()},
   CONSTRUCT=>RetirementDemand{copy_bytes:size_of::<EntityMap>()+size_of::<Vec<u64>>()+size_of::<ToolRunEntityEditKind>(),depth:1,..Default::default()},
   ADVANCE=>{let quote=edit()?.next_demands(body)?;edit()?.next_demands(body.max(quote.copy_bytes))?},
   TAKE=>ToolRunEntityEdit::<u64>::take_output_demand(),
   WRAP=>RetirementDemand{copy_bytes:size_of::<EntityMap>()+size_of::<ToolRunEntityEdit>(),depth:1,..Default::default()},
   CLOSE=>state.edit_close.as_ref().ok_or_else(||route_fault("entity version edit close is absent"))?.selected_demand(body)?,
   DROP=>RetirementDemand{depth:1,..Default::default()},
   _=>return Err(route_fault("entity version publisher phase is invalid"))
  })
 }
 pub fn advance_into<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>{self.drive(body,grant,effect,reply,false)}
 pub fn recover_advance_into<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>{self.drive(body,grant,effect,reply,true)}
 fn drive<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>,recover:bool)->Result<EntityComplete<'r>,ValueError>{
  if self.state.route.is_some()!=recover{return Err(route_fault("entity version changed its original recovery route"));}
  self.state.phase=self.state.phase_now();
  let demand=if recover{Default::default()}else{
   if self.state.cancelled||self.state.phase>=DONE||grant.retirement.maximum_items==0{return denied(reply);}
   let demand=self.next_demands(body)?;if !retained_clone_grant_admits(grant.retirement,demand){return denied(reply);}demand
  };
  let state=&mut *self.state;let phase=state.phase;
  match phase{
   ADMIT=>{let mut scope=reply.scope(&mut state.route,phase)?;drop(state.hold.step_into(body,effect,scope.borrow()?,recover)?);Ok(scope.forwarded())},
   CLONE=>{let clone=state.clone.as_mut().ok_or_else(||route_fault("entity version clone is absent"))?;let source=state.hold.original()?;let mut scope=reply.scope(&mut state.route,phase)?;drop(if recover{clone.recover_advance_into(source,body,u64::MAX,effect,scope.borrow()?)?}else{clone.advance_into(source,body,u64::MAX,effect,scope.borrow()?)?});Ok(scope.forwarded())},
   TAKE_CLONE=>{if recover{return Err(route_fault("entity version clone take has no unrecorded callback"));}let clone=state.clone.as_mut().ok_or_else(||route_fault("entity version clone is absent"))?;let mut inner=None;drop(clone.take(&mut state.base,effect,OriginalEffectReply::borrow(&mut inner)?)?);match inner.take(){Some(result)=>Ok(reply.progress().complete(result.map_err(OriginalRetirementFailure::from))),None=>Ok(reply.pending())}},
   BEGIN_CLONE=>{if recover{return Err(route_fault("entity version clone close start has no unrecorded callback"));}let clone=state.clone.as_mut().ok_or_else(||route_fault("entity version clone is absent"))?;let mut scope=reply.scope(&mut state.route,phase)?;drop(clone.begin_close_into(effect,scope.borrow()?)?);Ok(scope.forwarded())},
   CLOSE_CLONE=>{let clone=state.clone.as_mut().ok_or_else(||route_fault("entity version clone is absent"))?;let mut scope=reply.scope(&mut state.route,phase)?;drop(if recover{clone.recover_close_into(body,effect,scope.borrow()?)?}else{clone.close_into(body,effect,scope.borrow()?)?});Ok(scope.forwarded())},
   DROP_CLONE=>local_step(effect,demand,1,reply,|_|{state.clone=None;state.phase=CONSTRUCT;Ok(RetainedCloneProgress{copied_items:1,..Default::default()})}),
   CONSTRUCT=>local_step(effect,demand,1,reply,|_|{
    let base=state.base.take().ok_or_else(||route_fault("entity version clone output is absent"))?;
    state.edit=Some(ToolRunEntityEdit::new(base,state.targets.take().ok_or_else(||route_fault("entity version targets are absent"))?,state.kind));
    state.phase=ADVANCE;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})
   }),
   ADVANCE=>{let edit=state.edit.as_mut().ok_or_else(||route_fault("entity version edit is absent"))?;let mut scope=reply.scope(&mut state.route,phase)?;drop(if recover{edit.recover_advance_into(body,grant,effect,scope.borrow()?)?}else{edit.advance_into(body,grant,effect,scope.borrow()?)?});Ok(scope.forwarded())},
   TAKE=>{if recover{return Err(route_fault("entity version take has no unrecorded callback"));}let edit=state.edit.as_mut().ok_or_else(||route_fault("entity version edit is absent"))?;let mut scope=reply.scope(&mut state.route,phase)?;drop(edit.take_output_into(&mut state.candidate,effect,scope.borrow()?)?);Ok(scope.forwarded())},
   WRAP=>local_step(effect,demand,1,reply,|_|{let edit=state.edit.take().ok_or_else(||route_fault("entity version edit is absent"))?;match ControlledRetirement::new(edit){Ok(owner)=>state.edit_close=Some(owner),Err((error,edit))=>{state.edit=Some(edit);return Err(error);}}state.phase=CLOSE;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})}),
   CLOSE=>{let child=state.edit_close.as_mut().ok_or_else(||route_fault("entity version edit close is absent"))?;let mut scope=reply.scope(&mut state.route,phase)?;drop(if recover{child.recover_step_into(body,effect,scope.borrow()?)?}else{child.step_into(body,effect,scope.borrow()?)?});Ok(scope.forwarded())},
   DROP=>local_step(effect,demand,1,reply,|_|{state.edit_close=None;state.phase=DONE;Ok(RetainedCloneProgress{copied_items:1,..Default::default()})}),
   _=>Err(route_fault("entity version publisher phase is invalid"))
  }
 }
 pub fn take_output(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(ToolRunEntityVersion,RetainedCloneProgress)>,ValueError>{
  if !self.complete(){return Ok(None);}let demand=self.publication_demands();if !funded(effect,demand,0)?{return Ok(None);}
  let Some((entities,mut progress))=publish_sealed(effect,||self.state.candidate.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"entity publication has no private candidate")))?else{return Ok(None);};
  progress.copied_bytes=demand.copy_bytes;self.state.phase=PUBLISHED;
  Ok(Some((ToolRunEntityVersion{entities,revision:self.state.next_revision.ok_or_else(||ValueError::invariant("next_revision absent"))?},progress)))
 }
}
impl RetireOwned for ToolRunEntityVersionEdit {
 type RetirementCursor=<State as RetireOwned>::RetirementCursor;
 fn retirement_seed(mut self)->Self::RetirementCursor{self.transferred=true;unsafe{ManuallyDrop::take(&mut self.state)}.retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.state.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{true}
 fn retirement_element_copy_bytes()->usize{size_of::<Self>()}
}
impl Drop for ToolRunEntityVersionEdit{fn drop(&mut self){assert!(self.transferred,"original entity version publisher requires controlled retirement");}}
