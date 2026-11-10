//! 🧬️ Owns one pending tick presentation until its joint candidate is fully published or physically closed.
use super::{ToolRunPresentation,ToolRunPresentationBody};
use semio_framework_tool_run::{ToolRunProgress,ToolRunTick,ToolRunState,ToolRunStep,ToolRunStepRing,ToolRunCounter,progress_clone::ToolRunProgressClone,step_ring_insert::{ToolRunStepRingInsert,ToolRunStepRingInsertGrant}};
use semio_framework_tool_run_entities::{denied,duplicate_sealed,local_step,route_fault,source_hold::SourceHold,EntityComplete as Complete,EntityReply as Reply,ToolRunEntityEdit,ToolRunEntityEditGrant,provenance::{ToolRunEntityProvenance,ToolRunEntityProvenanceEdit}};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep as Step,retained_clone_grant_admits},retirement::{RetireOwned,controlled::ControlledRetirement}};
use semio_framework_value_shared_issuance::SealedShared;
use semio_framework_tool_run::payload_stream::{ToolRunPayloadStream,ToolRunPayloadUpdate,update::ToolRunPayloadStreamUpdate};
use std::{mem::{ManuallyDrop,size_of},collections::VecDeque};

type Hold=SourceHold<ToolRunProgress,ToolRunPresentation>;
const HOLD:u8=1;const CLONE_CLOSE:u8=4;const COUNTER_CLOSE:u8=6;const RING_CLOSE:u8=9;const PROVENANCE:u8=12;const PROVENANCE_CLOSE:u8=13;const PAYLOAD:u8=15;const PAYLOAD_CLOSE:u8=17;

/// 🧩️ Shares one slot between the children whose phases never overlap, so the update pays for the largest live child instead of their sum.
#[derive(semio_framework_value::RetireOwned)]
pub enum Live{Idle,Hold(Hold),Clone(ToolRunProgressClone<ToolRunPresentation>),CloneClose(ControlledRetirement<ToolRunProgressClone<ToolRunPresentation>>),Edit(ToolRunEntityProvenanceEdit),Close(ControlledRetirement<ToolRunEntityProvenanceEdit>),Payload(ToolRunPayloadStreamUpdate)}
macro_rules! live_accessors{($($get:ident,$get_mut:ident,$variant:ident,$ty:ty);*)=>{impl Live{$(fn $get(&self)->Option<&$ty>{if let Self::$variant(child)=self{Some(child)}else{None}} fn $get_mut(&mut self)->Option<&mut $ty>{if let Self::$variant(child)=self{Some(child)}else{None}})*}}}
live_accessors!(hold,hold_mut,Hold,Hold;clone,clone_mut,Clone,ToolRunProgressClone<ToolRunPresentation>;clone_close,clone_close_mut,CloneClose,ControlledRetirement<ToolRunProgressClone<ToolRunPresentation>>;edit,edit_mut,Edit,ToolRunEntityProvenanceEdit;close,close_mut,Close,ControlledRetirement<ToolRunEntityProvenanceEdit>;payload,payload_mut,Payload,ToolRunPayloadStreamUpdate);
#[derive(semio_framework_value::RetireOwned)]
#[value(boxed_original,send_only)]
#[doc(hidden)]
pub struct State{original:Option<ToolRunPresentation>,tick:ToolRunTick,end:u32,state:ToolRunState,rate:f32,conflicts:u32,capture:Option<ToolRunPresentation>,live:Live,progress:Option<ToolRunProgress>,counter_close:Option<ControlledRetirement<Vec<ToolRunCounter>>>,steps:VecDeque<ToolRunStep>,ring:Option<ToolRunStepRingInsert>,ring_close:Option<ControlledRetirement<ToolRunStepRingInsert>>,provenance_capture:Option<ToolRunEntityProvenance>,projected:Option<ToolRunEntityProvenance>,tool_id:Option<SealedShared<String>>,payload:Option<SealedShared<ToolRunPayloadStream>>,payload_phase:u8,output:Option<ToolRunPresentation>,step_sequence:u64,phase:u8,route:Option<u8>,cancelled:bool}
pub struct ToolRunPresentationUpdate{state:ManuallyDrop<Box<State>>,transferred:bool}
fn control_demands<T:RetireOwned>(owner:&ControlledRetirement<T>,body:usize)->Result<RetirementDemand,ValueError>{if owner.terminal_is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}owner.selected_demand(body)}
fn item(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}}
fn settled<'r>(reply:Reply<'r>,result:Result<RetainedCloneProgress,ValueError>)->Result<Complete<'r>,ValueError>{Ok(reply.complete(result.map(Step::Progress).map_err(Into::into)))}
macro_rules! close_child{($state:ident,$field:ident,$key:expr,$next:expr,$d:ident,$body:ident,$effect:ident,$reply:ident,$recover:ident)=>{{
 if $state.$field.as_ref().is_some_and(|child|child.terminal_is_empty()){return local_step($effect,$d,1,$reply,|_|{$state.$field=None;$state.phase=$next;Ok(item($d))});}
 let child=$state.$field.as_mut().ok_or_else(||route_fault("presentation close owner is absent"))?;
 let mut scope=$reply.scope(&mut $state.route,$key)?;
 drop(if $recover{child.recover_step_into($body,$effect,scope.borrow()?)}else{child.step_into($body,$effect,scope.borrow()?)}?);
 Ok(scope.forwarded())
}}}
impl ToolRunPresentationUpdate{
 pub fn birth_demands()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<State>()+size_of::<ToolRunPresentation>()+size_of::<ToolRunTick>()+size_of::<Self>(),capacity_bytes:size_of::<State>(),depth:1,..Default::default()}}
 pub fn admit(original:ToolRunPresentation,tick:ToolRunTick,end:u32,state:ToolRunState,rate:f32,conflicts:u32,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(ValueError,ToolRunPresentation,ToolRunTick)>{let demand=Self::birth_demands();if !retained_clone_grant_admits(grant,demand){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"original presentation update birth is not funded"),original,tick));}let step_sequence=original.body().progress.steps.newest().map_or(0,|step|step.sequence);let output=Self{state:ManuallyDrop::new(Box::write(Box::new_uninit(),State{original:Some(original),tick,end,state,rate,conflicts,capture:None,live:Live::Idle,progress:None,counter_close:None,steps:VecDeque::new(),ring:None,ring_close:None,provenance_capture:None,projected:None,tool_id:None,payload:None,payload_phase:0,output:None,step_sequence,phase:0,route:None,cancelled:false})),transferred:false};Ok((output,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))}
 pub fn complete(&self)->bool{self.state.phase==17&&!self.state.cancelled}
 pub fn cancel(&mut self){self.state.cancelled=true;if let Some(child)=self.state.live.clone_mut(){child.cancel();}if let Some(child)=self.state.ring.as_mut(){child.cancel();}if let Some(child)=self.state.live.edit_mut(){child.cancel();}}
 pub fn edit_capacity_bound(&self)->Result<usize,ValueError>{if let Some(child)=self.state.live.edit(){return child.edit_capacity_bound();}let count=self.state.original.as_ref().unwrap().body().provenance.marks().len().checked_add(self.state.tick.append_entities.len()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"presentation provenance cardinality overflow"))?;ToolRunEntityEdit::<u128>::capacity_bound_for(0,count).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"presentation provenance capacity overflow"))}
 fn phase_now(&self)->u8{let state=&**self.state;if state.phase==HOLD&&state.live.hold().is_some_and(Hold::ready){2}else{state.phase}}
 pub fn next_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  let state=&**self.state;if state.cancelled||state.phase>=17{return Ok(Default::default());}let original=state.original.as_ref().unwrap().body();if state.tick.identity.id!=original.progress.identity.id{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"pending presentation belongs to another original run"));}
  let mut quote=RetirementDemand{depth:1,..Default::default()};match self.phase_now(){
   0=>quote.copy_bytes=ToolRunPresentation::capture_copy_bytes(),
   1=>quote=match state.live.hold(){None=>RetirementDemand{copy_bytes:size_of::<Hold>(),depth:1,..Default::default()},Some(hold)=>hold.quote(body)?},
   2=>quote.copy_bytes=size_of::<ToolRunProgressClone<ToolRunPresentation>>(),
   3=>{let child=state.live.clone().unwrap();quote=if child.complete(){RetirementDemand{copy_bytes:size_of::<ToolRunProgress>()+size_of::<ToolRunProgressClone<ToolRunPresentation>>(),depth:1,..Default::default()}}else{child.next_demands()?};},
   4=>quote=control_demands(state.live.clone_close().unwrap(),body)?,
   5=>quote.copy_bytes=size_of::<ToolRunProgress>()+2*size_of::<Vec<ToolRunCounter>>(),
   6=>if let Some(child)=state.counter_close.as_ref(){quote=control_demands(child,body)?;},
   7=>quote.copy_bytes=size_of::<Vec<ToolRunStep>>()+size_of::<VecDeque<ToolRunStep>>(),
   8=>if let Some(child)=state.ring.as_ref(){quote=if child.complete(){RetirementDemand{copy_bytes:size_of::<ToolRunStepRing>()+size_of::<ToolRunStepRingInsert>(),depth:1,..Default::default()}}else{child.next_demands()?.retirement};}else if !state.steps.is_empty(){quote.copy_bytes=size_of::<ToolRunStepRing>()+size_of::<ToolRunStep>();},
   9=>quote=control_demands(state.ring_close.as_ref().unwrap(),body)?,
   10=>quote.copy_bytes=ToolRunEntityProvenance::capture_copy_bytes(),
   11=>quote.copy_bytes=size_of::<ToolRunEntityProvenance>()+size_of::<Vec<u64>>()+size_of::<Option<u32>>()+size_of::<u32>(),
   12=>{let child=state.live.edit().unwrap();quote=if child.complete(){let mut quote=child.publication_demands();quote.copy_bytes+=size_of::<ToolRunEntityProvenanceEdit>();quote}else{child.next_demands(body)?};},
   13=>quote=control_demands(state.live.close().unwrap(),body)?,
   14=>quote.copy_bytes=SealedShared::<String>::duplicate_copy_bytes(),
   15=>{quote=match state.payload_phase{
    0=>RetirementDemand{copy_bytes:SealedShared::<ToolRunPayloadStream>::duplicate_copy_bytes(),depth:1,..Default::default()},
    1=>{let mut demand=ToolRunPayloadStreamUpdate::birth_demand();demand.copy_bytes+=size_of::<Option<ToolRunPayloadStreamUpdate>>();demand},
    2=>{let child=state.live.payload().unwrap();if child.complete(){child.take_demand()}else{child.demand(body)?}},
    3=>{let child=state.live.payload().unwrap();if child.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<Option<ToolRunPayloadStreamUpdate>>(),depth:1,..Default::default()}}else{child.close_demand(body)?}},
    _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"presentation payload phase is invalid")),
   };},
   16=>quote=ToolRunPresentation::birth_demands()?,
   _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"pending presentation phase is invalid")),
  }Ok(quote)
 }
 pub fn advance_into<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.drive(body,grant,effect,reply,false)}
 pub fn recover_advance_into<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.drive(body,grant,effect,reply,true)}
 fn drive<'r>(&mut self,body:usize,grant:ToolRunEntityEditGrant,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>{
  if self.state.route.is_some()!=recover{return Err(route_fault("presentation update changed its original recovery route"));}
  self.state.phase=self.phase_now();
  if !recover{if self.state.cancelled||self.state.phase>=17||grant.retirement.maximum_items==0{return denied(reply);}}
  let d=if recover{Default::default()}else{self.next_demands(body)?};
  if !recover&&!retained_clone_grant_admits(grant.retirement,d){return denied(reply);}
  let state=&mut**self.state;let phase=state.phase;
  match phase{
   0=>settled(reply,state.original.as_ref().unwrap().capture(effect).map(|(capture,progress)|{state.capture=Some(capture);state.phase=1;progress})),
   1=>if state.live.hold().is_none(){
    local_step(effect,d,1,reply,|_|{state.live=Live::Hold(SourceHold::new(state.capture.take().ok_or_else(||ValueError::invariant("capture absent"))?));Ok(item(d))})
   }else{
    let hold=state.live.hold_mut().unwrap();let mut scope=reply.scope(&mut state.route,HOLD)?;drop(hold.step_into(body,effect,scope.borrow()?,recover)?);Ok(scope.forwarded())
   },
   2=>local_step(effect,d,1,reply,|_|{let Live::Hold(hold)=std::mem::replace(&mut state.live,Live::Idle) else{return Err(ValueError::invariant("hold absent"))};state.live=Live::Clone(ToolRunProgressClone::new(hold));state.phase=3;Ok(item(d))}),
   3=>if state.live.clone().unwrap().complete(){
    local_step(effect,d,1,reply,|effect|{
     let child=state.live.clone_mut().unwrap();state.progress=Some(child.take_output(effect.grant())?.unwrap().0);let Live::Clone(child)=std::mem::replace(&mut state.live,Live::Idle) else{return Err(ValueError::invariant("clone absent"))};
     match ControlledRetirement::new(child){Ok(close)=>state.live=Live::CloneClose(close),Err((error,child))=>{state.live=Live::Clone(child);return Err(error);}}state.phase=4;Ok(item(d))
    })
   }else{settled(reply,state.live.clone_mut().unwrap().advance(effect))},
   4=>{
    if state.live.clone_close().is_some_and(|child|child.terminal_is_empty()){return local_step(effect,d,1,reply,|_|{state.live=Live::Idle;state.phase=5;Ok(item(d))});}
    let child=state.live.clone_close_mut().ok_or_else(||route_fault("presentation close owner is absent"))?;
    let mut scope=reply.scope(&mut state.route,CLONE_CLOSE)?;
    drop(if recover{child.recover_step_into(body,effect,scope.borrow()?)}else{child.step_into(body,effect,scope.borrow()?)}?);
    Ok(scope.forwarded())
   },
   5=>local_step(effect,d,1,reply,|_|{
    let progress=state.progress.as_mut().unwrap();progress.identity=state.tick.identity;progress.sequence=progress.sequence.max(state.tick.sequence);progress.state=state.state;progress.units_per_second=state.rate;progress.conflicts=state.conflicts;
    if let Some(update)=state.tick.progress.as_mut(){progress.stage=update.stage;progress.total=update.total;progress.completed=update.total.map_or(update.completed,|total|update.completed.min(total));let counters=std::mem::replace(&mut progress.counters,std::mem::take(&mut update.counters));match ControlledRetirement::new(counters){Ok(close)=>state.counter_close=Some(close),Err((error,counters))=>{update.counters=std::mem::replace(&mut progress.counters,counters);return Err(error);}}}
    state.phase=6;Ok(item(d))
   }),
   6=>if state.counter_close.is_none(){local_step(effect,d,1,reply,|_|{state.phase=7;Ok(item(d))})}else{close_child!(state,counter_close,COUNTER_CLOSE,7,d,body,effect,reply,recover)},
   7=>local_step(effect,d,1,reply,|_|{state.steps=std::mem::take(&mut state.tick.steps).into();state.phase=8;Ok(item(d))}),
   8=>if state.ring.as_ref().is_some_and(ToolRunStepRingInsert::complete){
    local_step(effect,d,1,reply,|effect|{
     let child=state.ring.as_mut().unwrap();state.progress.as_mut().unwrap().steps=child.take_output(effect.grant())?.unwrap().0;let child=state.ring.take().unwrap();
     match ControlledRetirement::new(child){Ok(close)=>state.ring_close=Some(close),Err((error,child))=>{state.ring=Some(child);return Err(error);}}state.phase=9;Ok(item(d))
    })
   }else if let Some(child)=state.ring.as_mut(){
    settled(reply,child.advance(ToolRunStepRingInsertGrant{retirement:grant.retirement,comparison:grant.comparison,maximum_moved_items:grant.maximum_moved_items,maximum_moved_bytes:grant.maximum_moved_bytes},effect).map(|step|step.retirement))
   }else if !state.steps.is_empty(){
    local_step(effect,d,1,reply,|_|{let mut step=state.steps.pop_front().unwrap();state.step_sequence=state.step_sequence.saturating_add(1);step.sequence=state.step_sequence;let ring=std::mem::take(&mut state.progress.as_mut().unwrap().steps);state.ring=Some(ToolRunStepRingInsert::new(ring,step));Ok(item(d))})
   }else{local_step(effect,d,1,reply,|_|{state.phase=10;Ok(item(d))})},
   9=>close_child!(state,ring_close,RING_CLOSE,8,d,body,effect,reply,recover),
   10=>settled(reply,state.original.as_ref().unwrap().body().provenance.capture(effect).map(|(capture,progress)|{state.provenance_capture=Some(capture);state.phase=11;progress})),
   11=>local_step(effect,d,1,reply,|_|{state.live=Live::Edit(ToolRunEntityProvenanceEdit::new(state.provenance_capture.take().unwrap(),state.tick.retract_to,state.end,std::mem::take(&mut state.tick.append_entities)));state.phase=12;Ok(item(d))}),
   12=>if state.live.edit().unwrap().complete(){
    let child=state.live.edit_mut().unwrap();
    match child.take_output(effect){Ok(Some((projected,progress)))=>{state.projected=Some(projected);let Live::Edit(child)=std::mem::replace(&mut state.live,Live::Idle) else{return Err(route_fault("presentation provenance edit is absent"))};match ControlledRetirement::new(child){Ok(close)=>state.live=Live::Close(close),Err((error,child))=>{state.live=Live::Edit(child);return settled(reply,Err(error));}}state.phase=13;settled(reply,Ok(RetainedCloneProgress{copied_bytes:d.copy_bytes,..progress}))},Ok(None)=>denied(reply),Err(error)=>settled(reply,Err(error))}
   }else{
    let child=state.live.edit_mut().unwrap();let mut scope=reply.scope(&mut state.route,PROVENANCE)?;drop(if recover{child.recover_advance_into(body,grant,effect,scope.borrow()?)}else{child.advance_into(body,grant,effect,scope.borrow()?)}?);Ok(scope.forwarded())
   },
   13=>{
    if state.live.close().is_some_and(|child|child.terminal_is_empty()){return local_step(effect,d,1,reply,|_|{state.live=Live::Idle;state.phase=14;Ok(item(d))});}
    let child=state.live.close_mut().ok_or_else(||route_fault("presentation close owner is absent"))?;
    let mut scope=reply.scope(&mut state.route,PROVENANCE_CLOSE)?;
    drop(if recover{child.recover_step_into(body,effect,scope.borrow()?)}else{child.step_into(body,effect,scope.borrow()?)}?);
    Ok(scope.forwarded())
   },
   14=>settled(reply,duplicate_sealed(&state.original.as_ref().ok_or_else(||ValueError::invariant("original absent"))?.body().tool_id,effect).map(|(tool_id,progress)|{state.tool_id=Some(tool_id);state.phase=15;progress})),
   15=>match state.payload_phase{
    0=>{
     let duplicated=match state.original.as_ref().ok_or_else(||ValueError::invariant("original absent"))?.body().payload.as_ref(){Some(payload)=>Some(duplicate_sealed(payload,effect)),None=>None};
     let progress=match duplicated{Some(Ok((payload,mut progress)))=>{progress.copied_bytes=d.copy_bytes;state.payload=Some(payload);progress},Some(Err(error))=>return settled(reply,Err(error)),None=>Default::default()};
     if state.tick.payload.is_some(){state.payload_phase=1;}else{state.phase=16;}
     settled(reply,Ok(progress))
    },
    1=>local_step(effect,d,1,reply,|effect|{let mut update=ToolRunPayloadStreamUpdate::empty();let progress=update.admit(&mut state.payload,&mut state.tick.payload,state.tick.identity,state.tick.payload_update.unwrap_or(ToolRunPayloadUpdate::Replace),effect.grant())?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"funded original payload admission refused"))?;state.live=Live::Payload(update);state.payload_phase=2;Ok(RetainedCloneProgress{copied_bytes:d.copy_bytes,..progress})}),
    2=>if state.live.payload().unwrap().complete(){
     local_step(effect,d,1,reply,|effect|{let child=state.live.payload_mut().unwrap();let(payload,progress)=child.take(effect.grant())?.unwrap();state.payload=Some(payload);child.begin_close();state.payload_phase=3;Ok(progress)})
    }else{
     let child=state.live.payload_mut().unwrap();let mut scope=reply.scope(&mut state.route,PAYLOAD)?;drop(if recover{child.recover_advance_into(body,effect,scope.borrow()?)}else{child.advance_into(body,effect,scope.borrow()?)}?);Ok(scope.forwarded())
    },
    3=>if state.live.payload().unwrap().terminal_is_empty(){
     local_step(effect,d,1,reply,|_|{state.live=Live::Idle;state.phase=16;Ok(item(d))})
    }else{
     let child=state.live.payload_mut().unwrap();let mut scope=reply.scope(&mut state.route,PAYLOAD_CLOSE)?;drop(if recover{child.recover_close_into(body,effect,scope.borrow()?)}else{child.close_into(body,effect,scope.borrow()?)}?);Ok(scope.forwarded())
    },
    _=>Err(route_fault("presentation payload phase is invalid")),
   },
   16=>{let body=ToolRunPresentationBody{tool_id:state.tool_id.take().unwrap(),progress:state.progress.take().unwrap(),provenance:state.projected.take().unwrap(),payload:state.payload.take()};match ToolRunPresentation::admit(body,effect){Ok((output,progress))=>{state.output=Some(output);state.phase=17;settled(reply,Ok(progress))},Err((error,body))=>{state.tool_id=Some(body.tool_id);state.progress=Some(body.progress);state.projected=Some(body.provenance);state.payload=body.payload;settled(reply,Err(error))}}},
   _=>Err(route_fault("pending presentation phase is invalid")),
  }
 }
 pub fn take_output(&mut self,grant:RetainedCloneGrant)->Result<Option<(ToolRunPresentation,RetainedCloneProgress)>,ValueError>{if !self.complete()||grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<ToolRunPresentation>()||grant.maximum_depth==0{return Ok(None);}state_take_output(&mut self.state,grant)}
}
fn state_take_output(state:&mut State,_grant:RetainedCloneGrant)->Result<Option<(ToolRunPresentation,RetainedCloneProgress)>,ValueError>{let output=state.output.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"completed presentation lost its unpublished output"))?;state.phase=18;Ok(Some((output,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<ToolRunPresentation>(),..Default::default()})))}
impl RetireOwned for ToolRunPresentationUpdate{type RetirementCursor=<Box<State> as RetireOwned>::RetirementCursor;fn retirement_seed(mut self)->Self::RetirementCursor{self.transferred=true;unsafe{ManuallyDrop::take(&mut self.state)}.retirement_seed()}fn retirement_birth_bytes(&self)->Option<usize>{RetireOwned::retirement_birth_bytes(&*self.state)}fn controlled_retirement_supported()->bool{true}fn retirement_element_copy_bytes()->usize{size_of::<Self>()}}
impl Drop for ToolRunPresentationUpdate{fn drop(&mut self){if !self.transferred&&!std::thread::panicking(){panic!("pending original presentation requires controlled retirement")}}}
