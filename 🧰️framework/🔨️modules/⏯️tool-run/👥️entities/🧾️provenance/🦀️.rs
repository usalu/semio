//! 🧾️ Publishes deduplicated original operation provenance and its entity projection atomically.
use super::{publication_demand,source_hold::SourceHold,admit_sealed,denied,duplicate_sealed,funded,local_step,publish_sealed,reserve_vec,route_fault,EntityComplete,EntityMap,EntityReply,ToolRunEntityEdit,ToolRunEntityEditGrant,ToolRunEntityEditKind};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{OriginalEffectReplyComplete,OriginalProjectionReply,OriginalSourceProjection,RetainedAllocationEffect,RetainedCloneProgress,retained_clone_grant_admits,ordered_map::RetainedOrderedMap},retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_shared_issuance::SealedShared;
use std::mem::{ManuallyDrop,size_of};
use std::ptr::NonNull;

pub type ProvenanceMap=RetainedOrderedMap<u128,()>;
#[derive(semio_framework_value::RetireOwned)]
struct Root {marks:ProvenanceMap,entities:EntityMap}
#[derive(semio_framework_value::RetireOwned)]
pub struct ToolRunEntityProvenance {root:SealedShared<Root>,revision:u64}
impl ToolRunEntityProvenance {
 pub fn birth_bytes()->usize{SealedShared::<Root>::birth_bytes()}
 pub fn birth_copy_bytes()->usize{size_of::<Root>()}
 /// 📏️ Quotes the single frontier of publishing the empty provenance root.
 pub fn empty_birth_demand()->Result<RetirementDemand,ValueError>{publication_demand::<Root>()}
 pub fn admit_empty(effect:&mut RetainedAllocationEffect<'_>)->Result<(Self,RetainedCloneProgress),ValueError>{let grant=effect.grant();if grant.maximum_copy_bytes<Self::birth_copy_bytes(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"empty provenance fixed-body admission is not funded"));}let(root,mut progress)=publish_sealed(effect,||Ok(Root{marks:Default::default(),entities:Default::default()}))?.ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"empty provenance publication is not funded"))?;progress.copied_bytes=Self::birth_copy_bytes();Ok((Self{root,revision:0},progress))}
 pub fn capture_copy_bytes()->usize{size_of::<Self>().max(SealedShared::<Root>::duplicate_copy_bytes())}
 pub fn admit(marks:ProvenanceMap,entities:EntityMap,revision:u64,effect:&mut RetainedAllocationEffect<'_>)->Result<(Self,RetainedCloneProgress),(ValueError,ProvenanceMap,EntityMap)>{admit_sealed(Root{marks,entities},effect).map(|(root,progress)|(Self{root,revision},progress)).map_err(|(error,root)|(error,root.marks,root.entities))}
 pub fn capture(&self,effect:&mut RetainedAllocationEffect<'_>)->Result<(Self,RetainedCloneProgress),ValueError>{let grant=effect.grant();if grant.maximum_copy_bytes<Self::capture_copy_bytes(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original provenance capture requires its fixed-body copy"));}let(root,mut progress)=duplicate_sealed(&self.root,effect)?;progress.copied_bytes=Self::capture_copy_bytes();Ok((Self{root,revision:self.revision},progress))}
 pub fn marks(&self)->&ProvenanceMap{&self.root.get().marks}
 pub fn entities(&self)->&EntityMap{&self.root.get().entities}
 pub fn revision(&self)->u64{self.revision}
 pub fn identity(&self)->usize{self.root.identity()}
}
impl OriginalSourceProjection<ProvenanceMap> for ToolRunEntityProvenance{
 fn projection_work_demand(&self)->Result<u64,ValueError>{Ok(1)}
 fn project_source<'reply,'source>(&'source self,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,ProvenanceMap>)->Result<OriginalEffectReplyComplete<'reply,NonNull<ProvenanceMap>,ValueError>,ValueError>{if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"provenance projection exceeded its original work"))));}Ok(reply.complete(Ok(self.marks())))}
 fn recover_source_projection<'reply,'source>(&'source self,_:&mut RetainedAllocationEffect<'_>,reply:OriginalProjectionReply<'reply,'source,ProvenanceMap>)->Result<OriginalEffectReplyComplete<'reply,NonNull<ProvenanceMap>,ValueError>,ValueError>{Ok(reply.pending())}
}
const ADMIT:u8=0;const RESERVE_KEYS:u8=1;const SCAN_MARKS:u8=2;const PUSH_APPENDED:u8=3;const CONSTRUCT_MARKS:u8=4;const ADVANCE_MARKS:u8=5;const TAKE_MARKS:u8=6;const WRAP_MARKS:u8=7;const CLOSE_MARKS:u8=8;const DROP_MARKS:u8=9;
const RESERVE_TARGETS:u8=10;const SCAN_TARGETS:u8=11;const CONSTRUCT_ENTITIES:u8=12;const ADVANCE_ENTITIES:u8=13;const TAKE_ENTITIES:u8=14;const WRAP_ENTITIES:u8=15;const CLOSE_ENTITIES:u8=16;const DROP_ENTITIES:u8=17;const DONE:u8=18;const PUBLISHED:u8=19;

type Hold=SourceHold<ProvenanceMap,ToolRunEntityProvenance>;
#[doc(hidden)]
pub struct State {hold:Hold,append:Vec<u64>,keys:Option<Vec<u128>>,targets:Option<Vec<u64>>,marks:Option<ProvenanceMap>,entities:Option<EntityMap>,marks_edit:Option<ToolRunEntityEdit<u128>>,entities_edit:Option<ToolRunEntityEdit>,marks_close:Option<ControlledRetirement<ToolRunEntityEdit<u128>>>,entities_close:Option<ControlledRetirement<ToolRunEntityEdit>>,retract:Option<u32>,end:u32,index:usize,phase:u8,route:Option<u8>,next_revision:Option<u64>,key_count:Option<usize>,cancelled:bool}
semio_framework_value::artifact_retire_struct!(State{hold,append,keys,targets,marks,entities,marks_edit,entities_edit,marks_close,entities_close,retract,end,index,phase,route,next_revision,key_count,cancelled});
impl State {
 fn phase_now(&self)->u8{match self.phase{
  ADMIT if self.hold.ready()=>RESERVE_KEYS,
  ADVANCE_MARKS if self.marks_edit.as_ref().is_some_and(ToolRunEntityEdit::complete)=>TAKE_MARKS,
  TAKE_MARKS if self.marks.is_some()=>WRAP_MARKS,
  CLOSE_MARKS if self.marks_close.as_ref().is_some_and(ControlledRetirement::terminal_is_empty)=>DROP_MARKS,
  ADVANCE_ENTITIES if self.entities_edit.as_ref().is_some_and(ToolRunEntityEdit::complete)=>TAKE_ENTITIES,
  TAKE_ENTITIES if self.entities.is_some()=>WRAP_ENTITIES,
  CLOSE_ENTITIES if self.entities_close.as_ref().is_some_and(ControlledRetirement::terminal_is_empty)=>DROP_ENTITIES,
  phase=>phase}}
}
pub struct ToolRunEntityProvenanceEdit {state:ManuallyDrop<State>,transferred:bool}
fn control_demands<T:RetireOwned>(child:&ControlledRetirement<T>,body:usize)->Result<RetirementDemand,ValueError>{if child.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}child.selected_demand(body)}
fn absent(message:&'static str)->ValueError{route_fault(message)}
impl ToolRunEntityProvenanceEdit {
 pub fn new(original:ToolRunEntityProvenance,retract:Option<u32>,end:u32,append:Vec<u64>)->Self{let key_count=original.marks().len().checked_add(append.len());let next_revision=original.revision.checked_add(1);Self{state:ManuallyDrop::new(State{hold:SourceHold::new(original),append,keys:None,targets:None,marks:None,entities:None,marks_edit:None,entities_edit:None,marks_close:None,entities_close:None,retract,end,index:0,phase:ADMIT,route:None,next_revision,key_count,cancelled:false}),transferred:false}}
 pub fn complete(&self)->bool{self.state.phase_now()==DONE&&!self.state.cancelled}
 pub fn cancel(&mut self){self.state.cancelled=true;if let Some(child)=self.state.marks_edit.as_mut(){child.cancel();}if let Some(child)=self.state.entities_edit.as_mut(){child.cancel();}}
 pub fn edit_capacity_bound(&self)->Result<usize,ValueError>{let count=self.state.key_count.ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"provenance candidate cardinality overflow"))?;Ok(ToolRunEntityEdit::<u128>::capacity_bound_for(0,count).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"provenance candidate page capacity overflow"))?.max(ToolRunEntityEdit::<u64>::capacity_bound_for(0,count).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"entity candidate page capacity overflow"))?))}
 pub fn publication_demands(&self)->RetirementDemand{publication_demand::<Root>().unwrap_or_default()}
 pub fn next_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let state=&*self.state;let phase=state.phase_now();if state.cancelled||phase>=DONE{return Ok(Default::default());}
  let count=state.key_count.ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"provenance cardinality overflow"))?;if state.next_revision.is_none(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"provenance revision overflow"));}
  let marks_edit=||state.marks_edit.as_ref().ok_or_else(||absent("provenance marks edit is absent"));let entities_edit=||state.entities_edit.as_ref().ok_or_else(||absent("provenance entities edit is absent"));
  let twice=|child:&ToolRunEntityEdit<_>|-> Result<RetirementDemand,ValueError>{let quote=child.next_demands(body)?;child.next_demands(body.max(quote.copy_bytes))};
  Ok(match phase{
   ADMIT=>state.hold.quote(body)?,
   RESERVE_KEYS=>RetirementDemand{capacity_bytes:count.checked_mul(size_of::<u128>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"provenance target capacity overflow"))?,depth:1,..Default::default()},
   SCAN_MARKS|PUSH_APPENDED=>RetirementDemand{copy_bytes:size_of::<u128>(),depth:1,..Default::default()},
   CONSTRUCT_MARKS=>RetirementDemand{copy_bytes:size_of::<ProvenanceMap>()+size_of::<Vec<u128>>()+size_of::<ToolRunEntityEditKind>(),depth:1,..Default::default()},
   ADVANCE_MARKS=>{let child=marks_edit()?;let quote=child.next_demands(body)?;child.next_demands(body.max(quote.copy_bytes))?},
   TAKE_MARKS=>ToolRunEntityEdit::<u128>::take_output_demand(),
   WRAP_MARKS=>RetirementDemand{copy_bytes:size_of::<ProvenanceMap>()+size_of::<ToolRunEntityEdit<u128>>(),depth:1,..Default::default()},
   CLOSE_MARKS=>control_demands(state.marks_close.as_ref().ok_or_else(||absent("provenance marks close is absent"))?,body)?,
   DROP_MARKS|DROP_ENTITIES=>RetirementDemand{depth:1,..Default::default()},
   RESERVE_TARGETS=>RetirementDemand{capacity_bytes:state.marks.as_ref().ok_or_else(||absent("provenance marks are absent"))?.len().checked_mul(size_of::<u64>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"entity target capacity overflow"))?,depth:1,..Default::default()},
   SCAN_TARGETS=>RetirementDemand{copy_bytes:size_of::<u64>(),depth:1,..Default::default()},
   CONSTRUCT_ENTITIES=>RetirementDemand{copy_bytes:size_of::<EntityMap>()+size_of::<Vec<u64>>()+size_of::<ToolRunEntityEditKind>(),depth:1,..Default::default()},
   ADVANCE_ENTITIES=>twice(entities_edit()?)?,
   TAKE_ENTITIES=>ToolRunEntityEdit::<u64>::take_output_demand(),
   WRAP_ENTITIES=>RetirementDemand{copy_bytes:size_of::<EntityMap>()+size_of::<ToolRunEntityEdit>(),depth:1,..Default::default()},
   CLOSE_ENTITIES=>control_demands(state.entities_close.as_ref().ok_or_else(||absent("provenance entities close is absent"))?,body)?,
   _=>return Err(route_fault("provenance producer phase is invalid"))
  })
 }
 pub fn advance_into<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>{self.drive(body,grant,effect,reply,false)}
 pub fn recover_advance_into<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>{self.drive(body,grant,effect,reply,true)}
 fn drive<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>,recover:bool)->Result<EntityComplete<'r>,ValueError>{
  if self.state.route.is_some()!=recover{return Err(route_fault("provenance changed its original recovery route"));}
  self.state.phase=self.state.phase_now();
  let demand=if recover{Default::default()}else{
   if self.state.cancelled||self.state.phase>=DONE||grant.retirement.maximum_items==0{return denied(reply);}
   let demand=self.next_demands(body)?;if !retained_clone_grant_admits(grant.retirement,demand){return denied(reply);}demand
  };
  let state=&mut *self.state;let phase=state.phase;
  let item=RetainedCloneProgress{copied_items:1,..Default::default()};
  match phase{
   ADMIT=>{let mut scope=reply.scope(&mut state.route,phase)?;drop(state.hold.step_into(body,effect,scope.borrow()?,recover)?);Ok(scope.forwarded())},
   RESERVE_KEYS=>{let count=state.key_count.ok_or_else(||route_fault("provenance cardinality overflow"))?;local_step(effect,demand,1,reply,|effect|{let keys=reserve_vec::<u128>(count,effect)?;let capacity=keys.capacity()*size_of::<u128>();state.keys=Some(keys);state.phase=SCAN_MARKS;Ok(RetainedCloneProgress{retained_capacity_bytes:capacity,..item})})},
   SCAN_MARKS=>{
    let source=state.hold.original()?;let marks=source.get();
    if state.index==marks.len(){local_step(effect,demand,1,reply,|_|{state.index=0;state.phase=PUSH_APPENDED;Ok(item)})}
    else{let key=*marks.get_index(state.index).ok_or_else(||absent("provenance mark index is absent"))?.0;let kept=state.retract.is_none_or(|end|(key>>64)<=end as u128);local_step(effect,demand,1,reply,|_|{if kept{state.keys.as_mut().ok_or_else(||absent("provenance keys are absent"))?.push(key);}state.index+=1;Ok(RetainedCloneProgress{copied_bytes:if kept{size_of::<u128>()}else{0},..item})})}
   },
   PUSH_APPENDED=>if state.index==state.append.len(){local_step(effect,demand,1,reply,|_|{state.index=0;state.phase=CONSTRUCT_MARKS;Ok(item)})}else{let key=((state.end as u128)<<64)|state.append[state.index]as u128;local_step(effect,demand,1,reply,|_|{state.keys.as_mut().ok_or_else(||absent("provenance keys are absent"))?.push(key);state.index+=1;Ok(RetainedCloneProgress{copied_bytes:size_of::<u128>(),..item})})},
   CONSTRUCT_MARKS=>local_step(effect,demand,1,reply,|_|{state.marks_edit=Some(ToolRunEntityEdit::<u128>::new(ProvenanceMap::default(),state.keys.take().ok_or_else(||absent("provenance keys are absent"))?,ToolRunEntityEditKind::Append));state.phase=ADVANCE_MARKS;Ok(RetainedCloneProgress{copied_bytes:demand.copy_bytes,..item})}),
   ADVANCE_MARKS=>{let child=state.marks_edit.as_mut().ok_or_else(||absent("provenance marks edit is absent"))?;let mut scope=reply.scope(&mut state.route,phase)?;drop(if recover{child.recover_advance_into(body,grant,effect,scope.borrow()?)?}else{child.advance_into(body,grant,effect,scope.borrow()?)?});Ok(scope.forwarded())},
   TAKE_MARKS=>{if recover{return Err(route_fault("provenance take has no unrecorded callback"));}let child=state.marks_edit.as_mut().ok_or_else(||absent("provenance marks edit is absent"))?;let mut scope=reply.scope(&mut state.route,phase)?;drop(child.take_output_into(&mut state.marks,effect,scope.borrow()?)?);Ok(scope.forwarded())},
   WRAP_MARKS=>local_step(effect,demand,1,reply,|_|{let child=state.marks_edit.take().ok_or_else(||absent("provenance marks edit is absent"))?;match ControlledRetirement::new(child){Ok(owner)=>state.marks_close=Some(owner),Err((error,child))=>{state.marks_edit=Some(child);return Err(error);}}state.phase=CLOSE_MARKS;Ok(RetainedCloneProgress{copied_bytes:demand.copy_bytes,..item})}),
   CLOSE_MARKS=>{let child=state.marks_close.as_mut().ok_or_else(||absent("provenance marks close is absent"))?;let mut scope=reply.scope(&mut state.route,phase)?;drop(if recover{child.recover_step_into(body,effect,scope.borrow()?)?}else{child.step_into(body,effect,scope.borrow()?)?});Ok(scope.forwarded())},
   DROP_MARKS=>local_step(effect,demand,1,reply,|_|{state.marks_close=None;state.phase=RESERVE_TARGETS;Ok(item)}),
   RESERVE_TARGETS=>{let count=state.marks.as_ref().ok_or_else(||absent("provenance marks are absent"))?.len();local_step(effect,demand,1,reply,|effect|{let targets=reserve_vec::<u64>(count,effect)?;let capacity=targets.capacity()*size_of::<u64>();state.targets=Some(targets);state.phase=SCAN_TARGETS;Ok(RetainedCloneProgress{retained_capacity_bytes:capacity,..item})})},
   SCAN_TARGETS=>{
    let marks=state.marks.as_ref().ok_or_else(||absent("provenance marks are absent"))?;
    if state.index==marks.len(){local_step(effect,demand,1,reply,|_|{state.index=0;state.phase=CONSTRUCT_ENTITIES;Ok(item)})}
    else{let key=*marks.get_index(state.index).ok_or_else(||absent("provenance mark index is absent"))?.0 as u64;local_step(effect,demand,1,reply,|_|{state.targets.as_mut().ok_or_else(||absent("provenance targets are absent"))?.push(key);state.index+=1;Ok(RetainedCloneProgress{copied_bytes:size_of::<u64>(),..item})})}
   },
   CONSTRUCT_ENTITIES=>local_step(effect,demand,1,reply,|_|{state.entities_edit=Some(ToolRunEntityEdit::new(EntityMap::default(),state.targets.take().ok_or_else(||absent("provenance targets are absent"))?,ToolRunEntityEditKind::Append));state.phase=ADVANCE_ENTITIES;Ok(RetainedCloneProgress{copied_bytes:demand.copy_bytes,..item})}),
   ADVANCE_ENTITIES=>{let child=state.entities_edit.as_mut().ok_or_else(||absent("provenance entities edit is absent"))?;let mut scope=reply.scope(&mut state.route,phase)?;drop(if recover{child.recover_advance_into(body,grant,effect,scope.borrow()?)?}else{child.advance_into(body,grant,effect,scope.borrow()?)?});Ok(scope.forwarded())},
   TAKE_ENTITIES=>{if recover{return Err(route_fault("provenance take has no unrecorded callback"));}let child=state.entities_edit.as_mut().ok_or_else(||absent("provenance entities edit is absent"))?;let mut scope=reply.scope(&mut state.route,phase)?;drop(child.take_output_into(&mut state.entities,effect,scope.borrow()?)?);Ok(scope.forwarded())},
   WRAP_ENTITIES=>local_step(effect,demand,1,reply,|_|{let child=state.entities_edit.take().ok_or_else(||absent("provenance entities edit is absent"))?;match ControlledRetirement::new(child){Ok(owner)=>state.entities_close=Some(owner),Err((error,child))=>{state.entities_edit=Some(child);return Err(error);}}state.phase=CLOSE_ENTITIES;Ok(RetainedCloneProgress{copied_bytes:demand.copy_bytes,..item})}),
   CLOSE_ENTITIES=>{let child=state.entities_close.as_mut().ok_or_else(||absent("provenance entities close is absent"))?;let mut scope=reply.scope(&mut state.route,phase)?;drop(if recover{child.recover_step_into(body,effect,scope.borrow()?)?}else{child.step_into(body,effect,scope.borrow()?)?});Ok(scope.forwarded())},
   DROP_ENTITIES=>local_step(effect,demand,1,reply,|_|{state.entities_close=None;state.phase=DONE;Ok(item)}),
   _=>Err(route_fault("provenance producer phase is invalid"))
  }
 }
 pub fn take_output(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<(ToolRunEntityProvenance,RetainedCloneProgress)>,ValueError>{
  if !self.complete(){return Ok(None);}let quote=self.publication_demands();if !funded(effect,quote,0)?{return Ok(None);}
  let Some((root,mut receipt))=publish_sealed(effect,||Ok(Root{marks:self.state.marks.take().ok_or_else(||ValueError::invariant("marks absent"))?,entities:self.state.entities.take().ok_or_else(||ValueError::invariant("entities absent"))?}))?else{return Ok(None);};
  receipt.copied_bytes=quote.copy_bytes;self.state.phase=PUBLISHED;
  Ok(Some((ToolRunEntityProvenance{root,revision:self.state.next_revision.ok_or_else(||ValueError::invariant("next_revision absent"))?},receipt)))
 }
}
impl RetireOwned for ToolRunEntityProvenanceEdit {type RetirementCursor=<State as RetireOwned>::RetirementCursor;fn retirement_seed(mut self)->Self::RetirementCursor{self.transferred=true;unsafe{ManuallyDrop::take(&mut self.state)}.retirement_seed()}fn retirement_birth_bytes(&self)->Option<usize>{self.state.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}fn retirement_element_copy_bytes()->usize{size_of::<Self>()}}
impl Drop for ToolRunEntityProvenanceEdit{fn drop(&mut self){assert!(self.transferred,"original provenance producer requires controlled retirement")}}
