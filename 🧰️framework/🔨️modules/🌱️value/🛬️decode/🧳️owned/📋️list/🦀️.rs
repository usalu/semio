//! 🧳️ Original typed list children become persistent native rows through mandatory receiving and funded ownership transfers.
use crate::OriginalReceivingPrimary;
use crate::RetainedWorkOutcome;
use super::{NativeReceive,NativeReceiver,NativeAdvanceSelection,receiving_child_body_budget};
use crate::{OriginalEffectReply,OriginalEffectReplyComplete};
use crate::{DslValue,NativeDecodeControl,RetirementDemand,ValueError,ValueRefusalKind,list::persistent::{PersistentList,PersistentListBuilder},retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits},shared_issuance::{SealedShared,SharedIssuer}};
use std::mem::ManuallyDrop;
pub trait NativeListSource:Sync+'static{type Row:Sync+'static+?Sized;fn row_count(&self)->Result<usize,ValueError>;fn row(&self,index:usize)->Result<&Self::Row,ValueError>;}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"native persistent receiving requires the exact original ranked typed list")}
fn phase_error()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native persistent receiving has closed or transferred its required owner")}
impl<I:Sync+'static> NativeListSource for Vec<I>{type Row=I;fn row_count(&self)->Result<usize,ValueError>{Ok(self.len())}fn row(&self,index:usize)->Result<&I,ValueError>{self.get(index).ok_or_else(invalid)}}
impl NativeListSource for DslValue{type Row=Self;fn row_count(&self)->Result<usize,ValueError>{match self{Self::Array(rows)=>Ok(rows.len()),_=>Err(invalid())}}fn row(&self,index:usize)->Result<&Self,ValueError>{match self{Self::Array(rows)=>rows.get(index).ok_or_else(invalid),_=>Err(invalid())}}}
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Bind,Count,Initialize,RowAdvance,RowTake,RowClose,RowSeal,RowAdopt,Build,Finish,FinalTake,Ready,Spent}
pub struct NativePersistentListReceiver<I:NativeListSource+?Sized,T:NativeReceive<I::Row>+Sync>{
 binding:Option<RetainedCloneBinding>,builder:semio_framework_value_native_storage::NativeOwnedStorage<PersistentListBuilder<T>>,child:super::tree::NativeReceiveSlot<I::Row,T>,builder_retired:Option<Box<PersistentListBuilder<T>>>,raw:ManuallyDrop<Option<T>>,sealed:ManuallyDrop<Option<SealedShared<T>>>,output:ManuallyDrop<Option<PersistentList<T>>>,slots:super::NativeSharedSlots<T,SealedShared<T>>,retirement:RetainedCloneClose,frame:crate::native_decoding::NativeReceivingEffect,count:usize,index:usize,phase:Phase,closing:bool,input:std::marker::PhantomData<fn(&I)>,
}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn nested(mut d:RetirementDemand,extra:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(extra).ok_or_else(phase_error)?;d.depth=d.depth.checked_add(1).ok_or_else(phase_error)?;Ok(d)}
fn child_grant(g:RetainedCloneGrant,extra:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-extra,maximum_depth:g.maximum_depth-1,..g}}
impl<I:NativeListSource+?Sized,T:NativeReceive<I::Row>+Sync> NativePersistentListReceiver<I,T>{
 pub fn shared_admission(&mut self)->&mut crate::native_decoding::NativeSharedAdmission<T>{self.slots.admission()}
 fn row_source<'a>(&self,original:RetainedCloneRef<'a,I>)->Result<RetainedCloneRef<'a,I::Row>,ValueError>{original.get().row(self.index)?;let index=self.index;Ok(original.project(index,move|input|input.row(index).expect("original validated native list child")))}
 fn builder(&self)->Result<&PersistentListBuilder<T>,ValueError>{self.builder.get().ok_or_else(phase_error)}
 fn next_row_phase(&self)->Result<Phase,ValueError>{Ok(if !self.builder()?.accepts_row(){Phase::Build}else if self.index<self.count{Phase::RowAdvance}else{Phase::Finish})}
}
impl<I:NativeListSource+?Sized,T:NativeReceive<I::Row>+Sync>NativePersistentListReceiver<I,T>{
}

impl<I:NativeListSource+?Sized,T:NativeReceive<I::Row>+Sync> NativeReceive<I> for PersistentList<T>{
 type Receiver=NativePersistentListReceiver<I,T>;
 
 fn receiver()->Self::Receiver{NativePersistentListReceiver{binding:None,builder:semio_framework_value_native_storage::NativeOwnedStorage::empty(),child:Default::default(),builder_retired:None,raw:ManuallyDrop::new(None),sealed:ManuallyDrop::new(None),output:ManuallyDrop::new(None),slots:Default::default(),retirement:Default::default(),frame:Default::default(),count:0,index:0,phase:Phase::Bind,closing:false,input:std::marker::PhantomData}}
}
impl<I:NativeListSource+?Sized,T:NativeReceive<I::Row>+Sync> NativeReceiver<I,PersistentList<T>> for NativePersistentListReceiver<I,T>{
 fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||matches!(self.phase,Phase::Ready|Phase::Spent){return Err(phase_error())}original.check_binding(&self.binding)?;let metadata=||Ok(fixed(0));match self.phase{
  Phase::Bind=>if original.check_binding(&self.binding)?{metadata()}else{Ok(fixed(original.binding_copy_bytes()))},Phase::Count=>metadata(),
  Phase::Initialize=>if !self.builder.reserved(){Ok(semio_framework_value_native_storage::NativeOwnedStorage::<PersistentListBuilder<T>>::reserve_demand())}else if !self.builder.initialized(){Ok(semio_framework_value_native_storage::NativeOwnedStorage::<PersistentListBuilder<T>>::move_demand())}else{metadata()},
  Phase::RowAdvance=>if self.child.ready(){metadata()}else{nested(self.child.demands(self.row_source(original)?,body,maximum_work_units)?,0)},Phase::RowTake=>nested(self.child.take_demand()?,0),
  Phase::RowClose=>if self.child.terminal_is_empty(){metadata()}else{nested(self.child.close_demands(body)?,0)},
  Phase::RowSeal=>if self.sealed.is_some(){metadata()}else if self.raw.is_some(){Ok(super::NativeSharedSlots::<T,SealedShared<T>>::capture_demand())}else if self.slots.output_ready(){Ok(super::NativeSharedSlots::<T,SealedShared<T>>::take_demand())}else{Ok(self.slots.demand())},Phase::RowAdopt=>if self.sealed.is_none(){metadata()}else{nested(self.builder()?.adopt_demand()?,0)},
  Phase::Build=>if self.builder()?.output_ready()||self.builder()?.accepts_row(){metadata()}else{nested(self.builder()?.demand()?,0)},Phase::Finish=>if self.builder()?.finishing(){metadata()}else{nested(self.builder()?.finish_demand()?,0)},Phase::FinalTake=>nested(self.builder()?.take_demand()?,0),_=>Err(phase_error())}}
 fn select_advance(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{
  if self.closing||matches!(self.phase,Phase::Ready|Phase::Spent){return Err(phase_error())}
  if !original.check_binding(&self.binding)?{return Ok(NativeAdvanceSelection::Local{work:original.binding_work_demand(&self.binding)?})}
  let local=|work:u64|Ok(NativeAdvanceSelection::Local{work});
  let work=match self.phase{
   Phase::Bind|Phase::Count=>1,Phase::Initialize=>1,
   Phase::RowAdvance=>if self.child.ready(){1}else{return Ok(NativeAdvanceSelection::Nested)},Phase::RowTake=>self.child.take_work_demand()?,
   Phase::RowClose=>if self.child.terminal_is_empty(){1}else{self.child.close_work_demand(body)?},
   Phase::RowSeal=>if self.sealed.is_some(){1}else if self.raw.is_some(){self.slots.capture_work_demand(&self.raw)?}else if self.slots.output_ready(){self.slots.take_work_demand(&self.sealed)?}else{return Ok(NativeAdvanceSelection::Nested)},
   Phase::RowAdopt=>if self.sealed.is_none(){1}else{return Ok(NativeAdvanceSelection::Nested)},
   Phase::Build=>if self.builder()?.output_ready()||self.builder()?.accepts_row(){1}else{return Ok(NativeAdvanceSelection::Nested)},
   Phase::Finish=>if self.builder()?.finishing(){1}else{return Ok(NativeAdvanceSelection::Nested)},Phase::FinalTake=>return Ok(NativeAdvanceSelection::Nested),_=>return Err(phase_error())};
  if work>maximum_work_units{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native list action exceeds its original work ceiling"))}local(work)
 }
 fn advance_nested(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>{
  use crate::RetainedWorkPoll::Complete;
  let result:Result<crate::RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>,OriginalReceivingPrimary>=(||{
   if self.closing||self.binding.is_none(){return Err(phase_error().into())}
   let d=self.demands(original,body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}))}
   control.checkpoint()?;crate::owned::receiving_capacity(control,d.capacity_bytes)?;
   let Self{builder,child,sealed,output,slots,frame,phase,..}=self;
   let progress=|t:RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>|Complete(RetainedWorkOutcome{result:t.result.map(RetainedCloneStep::Progress),work:t.work});
   Ok(match *phase{
    Phase::RowAdvance=>{let index=self.index;let source=original.project(index,move|input|input.row(index).expect("original validated native list child"));child.advance(source,control,body,maximum_work_units,child_grant(g,0)).map(|s|RetainedCloneStep::Progress(s.progress()))},
    Phase::RowSeal=>{let t=slots.advance(control,SharedIssuer::owned());progress(t)},
    Phase::RowAdopt=>{let b=builder.get_mut().ok_or_else(phase_error)?;let q=1;progress(control.receive_allocation(d,frame,false,q,|grant,_|{let grant=grant.restricted_by(g);let t=b.adopt(sealed,grant);RetainedWorkOutcome{result:t.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())),work:t.work}}))},
    Phase::Build=>{let b=builder.get_mut().ok_or_else(phase_error)?;let q=b.advance_work_demand()?;progress(control.receive_allocation(d,frame,d.capacity_bytes!=0,q,|grant,actual|{let mut none=None;let grant=grant.restricted_by(g);b.advance(grant,actual.unwrap_or(&mut none))}))},
    Phase::Finish=>{let b=builder.get_mut().ok_or_else(phase_error)?;progress(control.receive_allocation(d,frame,false,1,|grant,_|{let grant=grant.restricted_by(g);let t=b.finish(grant);RetainedWorkOutcome{result:t.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())),work:t.work}}))},
    Phase::FinalTake=>{let b=builder.get_mut().ok_or_else(phase_error)?;let mut finished=false;let t=control.receive_allocation(d,frame,false,1,|grant,_|{let grant=grant.restricted_by(g);let t=b.take(grant);let work=t.work;let result=t.result.and_then(|value|match value{Some((list,p))=>{**output=Some(list);finished=true;Ok(RetainedCloneStep::Complete(p))},None=>Ok(RetainedCloneStep::Progress(Default::default()))});RetainedWorkOutcome{result,work}});if finished{*phase=Phase::Ready;}progress(t)},
    _=>return Err(phase_error().into())})
  })();
  match result{Ok(poll)=>poll,Err(error)=>Complete(RetainedWorkOutcome{result:Err(error),work:0})}
 }
 fn advance_local<'reply>(&mut self,original:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  let g=effect.grant();
  macro_rules! perform{()=>{if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native list action exceeded its original work").into())}}}
  let result:Result<Option<RetainedCloneStep>,OriginalReceivingPrimary>=(||{
   let d=self.demands(original,body,effect.maximum_work_units())?;if !retained_clone_grant_admits(g,d){return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
   if self.binding.is_none(){let turn=original.bind(&mut self.binding,effect);return turn.result.map(|p|Some(RetainedCloneStep::Progress(p.unwrap_or_default()))).map_err(Into::into)}
   match self.phase{
    Phase::Initialize if !self.builder.reserved()=>{return match self.builder.reserve_empty(effect){crate::RetainedWorkPoll::Pending{..}=>Ok(None),crate::RetainedWorkPoll::Complete(turn)=>turn.result.map(|s|Some(RetainedCloneStep::Progress(s.progress()))).map_err(Into::into)}},
    Phase::Initialize if !self.builder.initialized()=>{let mut header=Some(PersistentListBuilder::new(self.count));let turn=self.builder.put_from(&mut header,effect);return turn.result.map(|s|Some(RetainedCloneStep::Progress(s.progress()))).map_err(Into::into)},
    Phase::RowTake=>{return match self.child.take(&mut *self.raw,effect){crate::RetainedWorkPoll::Pending{..}=>Ok(None),crate::RetainedWorkPoll::Complete(turn)=>{let step=turn.result?;if self.raw.is_none(){return Err(ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"native take was admitted but published nothing").into())}self.child.begin_close();self.phase=Phase::RowClose;Ok(Some(RetainedCloneStep::Progress(step.progress())))}}},
    Phase::RowClose if !self.child.terminal_is_empty()=>{return match self.child.close_step(&mut effect.nested(child_grant(g,0))){crate::RetainedWorkPoll::Pending{..}=>Ok(None),crate::RetainedWorkPoll::Complete(turn)=>turn.result.map(|s|Some(RetainedCloneStep::Progress(s.progress()))).map_err(Into::into)}},
    Phase::RowSeal if self.sealed.is_none()&&self.raw.is_some()=>{let turn=self.slots.capture(&mut self.raw,effect);return turn.result.map(|step|Some(RetainedCloneStep::Progress(step.progress()))).map_err(Into::into)},
    Phase::RowSeal if self.sealed.is_none()&&self.slots.output_ready()=>{let turn=self.slots.take(&mut self.sealed,effect);return turn.result.map(|step|Some(RetainedCloneStep::Progress(step.progress()))).map_err(Into::into)},
    _=>{}}
   perform!();
   match self.phase{
    Phase::Bind=>self.phase=Phase::Count,
    Phase::Count=>{self.count=original.get().row_count()?;self.phase=Phase::Initialize},
    Phase::Initialize=>self.phase=if self.count==0{Phase::Finish}else{Phase::RowAdvance},
    Phase::RowAdvance=>{if !self.child.ready(){return Err(phase_error().into())}self.phase=Phase::RowTake},
    Phase::RowClose=>{self.child=Default::default();self.phase=Phase::RowSeal},
    Phase::RowSeal=>{if self.sealed.is_none(){return Err(phase_error().into())}self.phase=Phase::RowAdopt},
    Phase::RowAdopt=>{self.index+=1;self.phase=self.next_row_phase()?},
    Phase::Build=>{if self.builder()?.output_ready(){self.phase=Phase::FinalTake}else if self.builder()?.accepts_row(){self.phase=if self.index<self.count{Phase::RowAdvance}else{Phase::Finish}}else{return Err(phase_error().into())}},
    Phase::Finish=>self.phase=Phase::Build,
    _=>return Err(phase_error().into())}
   Ok(Some(RetainedCloneStep::Progress(receipt(fixed(0)))))
  })();
  Ok(match result{Ok(Some(step))=>reply.complete(Ok(step)),Ok(None)=>reply.pending(),Err(error)=>reply.complete(Err(error))})
 }
 fn output_ready(&self)->bool{!self.closing&&self.phase==Phase::Ready&&self.output.is_some()}
 semio_framework_value_native_receiver_protocol::native_owned_output_state!(PersistentList<T>,phase_error,phase,Phase::Spent);
 fn begin_close(&mut self){self.closing=true;self.child.begin_close();if let Some(builder)=self.builder.get_mut(){builder.begin_close();}}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.slots.terminal_is_empty(){return self.slots.close_demand(body)}if !self.child.terminal_is_empty(){return self.child.close_demands(body)}if self.builder.reserved()||self.builder_retired.is_some(){if let Some(builder)=self.builder_retired.as_ref(){if !builder.terminal_is_empty(){return Err(phase_error())}return Ok(RetirementDemand{release_bytes:std::alloc::Layout::new::<PersistentListBuilder<T>>().size(),depth:1,..Default::default()})}if let Some(builder)=self.builder.get(){if !builder.terminal_is_empty(){return nested(builder.close_demand(body)?,0)}return Ok(semio_framework_value_native_storage::NativeOwnedStorage::<PersistentListBuilder<T>>::box_demand())}return Ok(self.builder.release_demand())}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body)}if self.raw.is_some(){return self.retirement.next_owner_demands::<T>(true,&None,body)}if self.sealed.is_some(){return self.retirement.next_owner_demands::<SealedShared<T>>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_demands::<PersistentList<T>>(true,&None,body)}RetainedCloneBinding::close_demand(&self.binding,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.slots.terminal_is_empty(){return self.slots.close_work_demand(body)}if !self.child.terminal_is_empty(){return self.child.close_work_demand(body)}if let Some(builder)=self.builder.get(){if !builder.terminal_is_empty(){return builder.close_work_demand(body)}}if self.builder.reserved()||self.builder_retired.is_some(){return Ok(1)}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}if self.raw.is_some(){return self.retirement.next_owner_work::<T>(true,&None,body)}if self.sealed.is_some(){return self.retirement.next_owner_work::<SealedShared<T>>(true,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<PersistentList<T>>(true,&None,body)}RetainedCloneBinding::close_work_demand(&self.binding,body)}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{
  use crate::RetainedWorkPoll::{Complete,Pending};let g=effect.grant();
  if self.terminal_is_empty(){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0})}
  let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})};if !retained_clone_grant_admits(g,d){return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  if !self.slots.terminal_is_empty(){return self.slots.close_step(&mut effect.nested(g)).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if !self.child.terminal_is_empty(){return self.child.close_step(&mut effect.nested(g)).map(|step|RetainedCloneStep::Progress(step.progress()))}
  if self.builder_retired.is_some(){if !effect.performed(1).unwrap_or(false){return Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native list builder release exceeded its work authority").into()),work:1})}let layout=std::alloc::Layout::new::<PersistentListBuilder<T>>();let Some(owner)=self.builder_retired.take()else{return Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("builder_retired absent").into()),work:1})};let pointer=Box::into_raw(owner);if layout.size()!=0{unsafe{std::alloc::dealloc(pointer.cast(),layout)}}return Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:d.release_bytes,..Default::default()})),work:1})}
  if self.builder.reserved(){
   if let Some(builder)=self.builder.get_mut().filter(|builder|!builder.terminal_is_empty()){return builder.close_step(&mut effect.nested(child_grant(g,0))).map(|s|RetainedCloneStep::Progress(s.progress()))}
   if self.builder.initialized(){return match self.builder.take_box_into(&mut self.builder_retired,effect){Pending{work}=>Pending{work},Complete(turn)=>Complete(RetainedWorkOutcome{result:turn.result.map(|s|RetainedCloneStep::Progress(s.progress())).map_err(Into::into),work:turn.work})}}
   let turn=self.builder.release(effect);return Complete(RetainedWorkOutcome{result:turn.result.map(|s|RetainedCloneStep::Progress(s.progress())).map_err(Into::into),work:turn.work})}
  if !self.retirement.is_empty(){return self.retirement.step_granted(effect).map(|s|RetainedCloneStep::Progress(s.progress()))}
  macro_rules! close{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,effect);return Complete(RetainedWorkOutcome{result:turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into),work:turn.work})}}}
  close!(raw);close!(sealed);close!(output);
  RetainedCloneBinding::close_one(&mut self.binding,effect)}
 fn requires_original_birth(&self)->bool{if !self.slots.terminal_is_empty(){return self.slots.requires_original_birth()}if !self.child.terminal_is_empty(){return self.child.requires_original_birth()}if self.builder.reserved()||self.builder_retired.is_some(){return self.builder.get().is_some_and(|c|!c.terminal_is_empty()&&c.requires_original_birth())}self.retirement.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{if !self.slots.terminal_is_empty(){return self.slots.close_original_birth_step(original,grant)}if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}let inner=child_grant(grant,0);if !self.child.terminal_is_empty()&&self.child.requires_original_birth(){return self.child.close_original_birth_step(original,inner)}if let Some(c)=self.builder.get_mut(){if !c.terminal_is_empty()&&c.requires_original_birth(){return c.close_original_birth_step(original,inner)}}self.retirement.step_original_birth(original,grant)}
 fn requires_original_control(&self)->bool{if !self.slots.terminal_is_empty(){return self.slots.requires_original_control()}if !self.child.terminal_is_empty(){return self.child.requires_original_control()}if self.builder.reserved()||self.builder_retired.is_some(){return self.builder.get().is_some_and(|c|!c.terminal_is_empty()&&c.requires_original_control())}self.retirement.requires_original_control()}
 fn close_original_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{if self.requires_original_birth(){return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"settle original birth first").into()),work:0})}if !self.slots.terminal_is_empty()&&self.slots.requires_original_control(){return self.slots.close_original_step(original,effect)}let g=effect.grant();let inner=child_grant(g,0);if !self.child.terminal_is_empty()&&self.child.requires_original_control(){return self.child.close_original_step(original,&mut effect.nested(inner))}if let Some(c)=self.builder.get_mut(){if !c.terminal_is_empty()&&c.requires_original_control(){return c.close_original_step(original,&mut effect.nested(inner))}}if self.requires_original_control(){return self.retirement.step_original(original,effect)}self.close_step(effect)}
 fn terminal_is_empty(&self)->bool{self.slots.terminal_is_empty()&&self.child.terminal_is_empty()&&self.builder.terminal_is_empty()&&self.builder_retired.is_none()&&self.raw.is_none()&&self.sealed.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.frame.phase()==crate::native_decoding::construction::NativeReceivingEffectPhase::Empty&&self.binding.is_none()}
}
impl<I:NativeListSource+?Sized,T:NativeReceive<I::Row>+Sync> Drop for NativePersistentListReceiver<I,T>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"native persistent receiving retains original physical owners until funded closure");}}

impl NativeListSource for dyn super::tree::NativeValueSource{type Row=dyn super::tree::NativeValueSource;fn row_count(&self)->Result<usize,ValueError>{self.sequence_len()}fn row(&self,index:usize)->Result<&Self::Row,ValueError>{self.sequence_child(index)}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
