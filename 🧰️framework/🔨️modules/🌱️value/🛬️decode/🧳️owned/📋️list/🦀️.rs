//! 🧳️ Original typed list children become persistent native rows through mandatory receiving and funded ownership transfers.
use crate::OriginalReceivingPrimary;
use crate::RetainedWorkOutcome;
use super::{NativeReceiver,NativeReceiverSeed,receiving_child_body_budget};
use crate::{DslValue,NativeDecodeControl,OriginalEffectReply,OriginalEffectReplyComplete,OriginalRetirementFailure,OriginalSourceTurnWitness,RetirementDemand,ValueError,ValueRefusalKind,list::persistent::{PersistentList,PersistentListBuilder},native_decoding::NativeReceivingFailure,retained_clone::{RetainedCloneBinding,RetainedCloneClose,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits},shared_issuance::SealedShared};
use semio_framework_value_native_receiver_protocol::{NativeAdvanceSelection,NativeReceiverBackingClose,native_record_pure_take};
use semio_framework_value_native_shared_engine::NativeSharedReceiver;
use semio_framework_value_native_storage::NativeOwnedStorage;
use std::mem::ManuallyDrop;
pub trait NativeListSource:Sync+'static{type Row:Sync+'static+?Sized;fn row_count(&self)->Result<usize,ValueError>;fn row(&self,index:usize)->Result<&Self::Row,ValueError>;}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"native persistent receiving requires the exact original ranked typed list")}
fn phase_error()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native persistent receiving has closed or transferred its required owner")}
impl<I:Sync+'static> NativeListSource for Vec<I>{type Row=I;fn row_count(&self)->Result<usize,ValueError>{Ok(self.len())}fn row(&self,index:usize)->Result<&I,ValueError>{self.get(index).ok_or_else(invalid)}}
impl NativeListSource for DslValue{type Row=Self;fn row_count(&self)->Result<usize,ValueError>{match self{Self::Array(rows)=>Ok(rows.len()),_=>Err(invalid())}}fn row(&self,index:usize)->Result<&Self,ValueError>{match self{Self::Array(rows)=>rows.get(index).ok_or_else(invalid),_=>Err(invalid())}}}
impl NativeListSource for dyn semio_framework_value_native_schema_engine::source::NativeValueSource{type Row=dyn semio_framework_value_native_schema_engine::source::NativeValueSource;fn row_count(&self)->Result<usize,ValueError>{self.sequence_len()}fn row(&self,index:usize)->Result<&Self::Row,ValueError>{self.sequence_child(index)}}
type Reply<'r>=OriginalEffectReply<'r,RetainedCloneStep,OriginalReceivingPrimary>;
type Complete<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalReceivingPrimary>;
type CloseReply<'r>=OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>;
type CloseComplete<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>;
type Row<I,T,C,S>=NativeSharedReceiver<<I as NativeListSource>::Row,T,C,S,SealedShared<T>>;
#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase{Bind,Count,Initialize,RowBirth,RowAdvance,RowTake,RowClose,RowAdopt,Build,Finish,FinalTake,Ready,Spent}
/// 🧳️ Every row is received by its own shared receiver, sealed, adopted by the builder and released before the next row.
pub struct NativePersistentListReceiver<I:NativeListSource+?Sized,T:crate::retirement::RetireOwned+Send+Sync+'static,C:NativeReceiver<I::Row,T>,S:NativeReceiverSeed<I::Row,T,C>>{
 binding:Option<RetainedCloneBinding>,builder:NativeOwnedStorage<PersistentListBuilder<T>>,builder_retired:Option<Box<PersistentListBuilder<T>>>,child:NativeOwnedStorage<Row<I,T,C,S>>,child_backing:NativeReceiverBackingClose,child_retired:Option<Box<Row<I,T,C,S>>>,sealed:ManuallyDrop<Option<SealedShared<T>>>,output:ManuallyDrop<Option<PersistentList<T>>>,retirement:RetainedCloneClose,count:usize,index:usize,phase:Phase,closing:bool,input:std::marker::PhantomData<fn(&I)->S>,
}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn nested(mut d:RetirementDemand,extra:usize)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(extra).ok_or_else(phase_error)?;d.depth=d.depth.checked_add(1).ok_or_else(phase_error)?;Ok(d)}
fn child_grant(g:RetainedCloneGrant,extra:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes.saturating_sub(extra),maximum_depth:g.maximum_depth.saturating_sub(1),..g}}
fn relabel(step:RetainedCloneStep)->RetainedCloneStep{RetainedCloneStep::Progress(step.progress())}
fn relay<'r,E:Into<OriginalReceivingPrimary>>(reply:Reply<'r>,answer:Option<Result<RetainedCloneStep,E>>)->Complete<'r>{match answer{Some(result)=>reply.complete(result.map(relabel).map_err(Into::into)),None=>reply.pending()}}
impl<I:NativeListSource+?Sized,T:crate::retirement::RetireOwned+Send+Sync+'static,C:NativeReceiver<I::Row,T>,S:NativeReceiverSeed<I::Row,T,C>> NativePersistentListReceiver<I,T,C,S>{
 pub fn new()->Self{NativePersistentListReceiver{binding:None,builder:NativeOwnedStorage::empty(),builder_retired:None,child:NativeOwnedStorage::empty(),child_backing:NativeReceiverBackingClose::vacant(),child_retired:None,sealed:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),count:0,index:0,phase:Phase::Bind,closing:false,input:std::marker::PhantomData}}
 fn row_source<'a>(&self,original:RetainedCloneRef<'a,I>)->Result<RetainedCloneRef<'a,I::Row>,ValueError>{original.get().row(self.index)?;let index=self.index;Ok(original.project(index,move|input|input.row(index).expect("original validated native list child")))}
 fn builder(&self)->Result<&PersistentListBuilder<T>,ValueError>{self.builder.get().ok_or_else(phase_error)}
 fn child(&self)->Result<&Row<I,T,C,S>,ValueError>{self.child.get().ok_or_else(phase_error)}
 fn next_row_phase(&self)->Result<Phase,ValueError>{Ok(if !self.builder()?.accepts_row(){Phase::Build}else if self.index<self.count{Phase::RowBirth}else{Phase::Finish})}
 fn child_needs_close(&self)->bool{self.child.reserved()||self.child_retired.is_some()||!self.child_backing.terminal_is_empty()}
 fn builder_needs_close(&self)->bool{self.builder.reserved()||self.builder_retired.is_some()}
 fn child_seed_demand(&self)->Result<RetirementDemand,ValueError>{match <Row<I,T,C,S> as NativeReceiver<I::Row,SealedShared<T>>>::seed_template(false){Some(template)=>self.child.template_seed_demand(template),None=>Ok(NativeOwnedStorage::<Row<I,T,C,S>>::seed_demand())}}
 fn builder_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.builder_retired.is_some(){return Ok(RetirementDemand{release_bytes:std::mem::size_of::<PersistentListBuilder<T>>(),depth:1,..Default::default()})}
  if let Some(builder)=self.builder.get(){return if builder.terminal_is_empty(){Ok(NativeOwnedStorage::<PersistentListBuilder<T>>::box_demand())}else if !builder.is_closing(){Ok(builder.begin_close_demand())}else{nested(builder.close_demand(body)?,0)}}
  Ok(self.builder.release_demand())
 }
 fn builder_close_work(&self,body:usize)->Result<u64,ValueError>{
  if self.builder_retired.is_some(){return Ok(1)}
  if let Some(builder)=self.builder.get(){return if builder.terminal_is_empty(){Ok(self.builder.box_work_demand())}else if !builder.is_closing(){Ok(builder.begin_close_work_demand())}else{builder.close_work_demand(body)}}
  Ok(self.builder.release_work_demand())
 }
 fn close_dispatch<'r>(&mut self,mut original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>,recover:bool)->Result<CloseComplete<'r>,ValueError>{
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  if self.child_needs_close(){return match(original.as_deref_mut(),recover){(Some(loan),false)=>self.child_backing.close_original_into::<I::Row,SealedShared<T>,Row<I,T,C,S>>(&mut self.child,&mut self.child_retired,body,loan,effect,reply),(Some(loan),true)=>self.child_backing.recover_close_original_into::<I::Row,SealedShared<T>,Row<I,T,C,S>>(&mut self.child,&mut self.child_retired,body,loan,effect,reply),(None,false)=>self.child_backing.close_into::<I::Row,SealedShared<T>,Row<I,T,C,S>>(&mut self.child,&mut self.child_retired,body,effect,reply),(None,true)=>self.child_backing.recover_close_into::<I::Row,SealedShared<T>,Row<I,T,C,S>>(&mut self.child,&mut self.child_retired,body,effect,reply)}}
  if self.builder_needs_close(){return self.close_builder(original,body,effect,reply,recover)}
  if !self.retirement.is_empty(){return match(original.as_deref_mut(),recover){(Some(loan),false)=>self.retirement.step_original_into(loan,body,effect,reply),(Some(loan),true)=>self.retirement.recover_step_original_into(loan,body,effect,reply),(None,false)=>self.retirement.step_into(body,effect,reply),(None,true)=>self.retirement.recover_step_into(body,effect,reply)}}
  if recover{return Err(phase_error())}
  if self.sealed.is_some(){let turn=self.retirement.begin_granted(&mut self.sealed,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}
  if self.output.is_some(){let turn=self.retirement.begin_granted(&mut self.output,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}
  Ok(match RetainedCloneBinding::close_one(&mut self.binding,body,effect){crate::RetainedWorkPoll::Pending{..}=>reply.pending(),crate::RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result.map_err(Into::into))})
 }
 fn close_builder<'r>(&mut self,original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>,recover:bool)->Result<CloseComplete<'r>,ValueError>{
  let d=self.builder_close_demand(body)?;if !retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units()?<self.builder_close_work(body)?{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if let Some(retired)=self.builder_retired.take(){if recover||!retired.terminal_is_empty(){self.builder_retired=Some(retired);return Err(phase_error())}if !effect.performed(1)?{self.builder_retired=Some(retired);return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"builder header release exceeded original Work").into())))}let layout=std::alloc::Layout::new::<PersistentListBuilder<T>>();let pointer=Box::into_raw(retired);if layout.size()!=0{unsafe{std::alloc::dealloc(pointer.cast(),layout)}}return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()}))))}
  if let Some(builder)=self.builder.get_mut(){
   if builder.terminal_is_empty(){let turn=self.builder.take_box_into(&mut self.builder_retired,effect);return Ok(match turn{crate::RetainedWorkPoll::Pending{..}=>reply.pending(),crate::RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result.map(relabel).map_err(Into::into))})}
   if !builder.is_closing(){let turn=builder.begin_close_paid(effect);return Ok(reply.complete(turn.result.map(relabel).map_err(Into::into)))}
   return match(original,recover){(Some(loan),false)=>builder.close_original_into(loan,body,effect,reply),(Some(loan),true)=>builder.recover_close_original_into(loan,body,effect,reply),(None,false)=>builder.close_into(body,effect,reply),(None,true)=>builder.recover_close_into(body,effect,reply)}
  }
  let turn=self.builder.release(effect);Ok(reply.complete(turn.result.map(relabel).map_err(Into::into)))
 }
}
impl<I:NativeListSource+?Sized,T:crate::retirement::RetireOwned+Send+Sync+'static,C:NativeReceiver<I::Row,T>,S:NativeReceiverSeed<I::Row,T,C>> NativeReceiver<I,PersistentList<T>> for NativePersistentListReceiver<I,T,C,S>{
 fn demands(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||matches!(self.phase,Phase::Ready|Phase::Spent){return Err(phase_error())}original.check_binding(&self.binding)?;let metadata=||Ok(fixed(0));match self.phase{
  Phase::Bind=>if original.check_binding(&self.binding)?{metadata()}else{Ok(fixed(original.binding_copy_bytes()))},Phase::Count=>metadata(),
  Phase::Initialize=>if !self.builder.reserved(){Ok(NativeOwnedStorage::<PersistentListBuilder<T>>::reserve_demand())}else if !self.builder.initialized(){Ok(NativeOwnedStorage::<PersistentListBuilder<T>>::seed_demand())}else{metadata()},
  Phase::RowBirth=>if !self.child.reserved(){Ok(NativeOwnedStorage::<Row<I,T,C,S>>::reserve_demand())}else if !self.child.initialized(){self.child_seed_demand()}else{metadata()},
  Phase::RowAdvance=>if self.child()?.output_ready(){metadata()}else{nested(self.child()?.demands(self.row_source(original)?,body,maximum_work_units)?,0)},
  Phase::RowTake=>if self.sealed.is_some(){metadata()}else{nested(self.child()?.take_demand()?,0)},
  Phase::RowClose=>if self.child_needs_close(){self.child_backing.demands::<I::Row,SealedShared<T>,Row<I,T,C,S>>(&self.child,&self.child_retired,body)}else{metadata()},
  Phase::RowAdopt=>if self.sealed.is_none(){metadata()}else{nested(self.builder()?.adopt_demand()?,0)},
  Phase::Build=>if self.builder()?.output_ready()||self.builder()?.accepts_row(){metadata()}else{nested(self.builder()?.demand()?,0)},
  Phase::Finish=>if self.builder()?.finishing(){metadata()}else{nested(self.builder()?.finish_demand()?,0)},
  Phase::FinalTake=>nested(self.builder()?.take_demand()?,0),_=>Err(phase_error())}}
 fn select_advance(&self,original:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{use NativeAdvanceSelection::Local;self.demands(original,body,maximum_work_units)?;match self.phase{
  Phase::Bind=>Ok(Local{work:if original.check_binding(&self.binding)?{1}else{original.binding_work_demand(&self.binding)?}}),
  Phase::Count|Phase::Initialize|Phase::RowBirth|Phase::RowAdopt|Phase::Finish=>Ok(Local{work:1}),
  Phase::RowAdvance=>if self.child()?.output_ready(){Ok(Local{work:1})}else{self.child()?.select_advance(self.row_source(original)?,body,maximum_work_units)},
  Phase::RowTake=>Ok(Local{work:if self.sealed.is_some(){1}else{self.child()?.take_work_demand()?}}),
  Phase::RowClose=>Ok(Local{work:if self.child_needs_close(){self.child_backing.work_demand::<I::Row,SealedShared<T>,Row<I,T,C,S>>(&self.child,&self.child_retired,body)?}else{1}}),
  Phase::Build=>Ok(Local{work:if self.builder()?.output_ready()||self.builder()?.accepts_row(){1}else{self.builder()?.advance_work_demand()?}}),
  Phase::FinalTake=>Ok(Local{work:self.builder()?.take_work_demand()?}),_=>Err(phase_error())}}
 fn advance_nested(&mut self,original:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->crate::RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{
  use crate::RetainedWorkPoll::Complete;let refuse=|error:ValueError|Complete(RetainedWorkOutcome{result:Err(error.into()),work:0});
  match self.select_advance(original,body,maximum_work_units){Ok(NativeAdvanceSelection::Nested)=>{},Ok(_)=>return refuse(phase_error()),Err(error)=>return refuse(error)}
  let source=match self.row_source(original){Ok(source)=>source,Err(error)=>return refuse(error)};let Some(child)=self.child.get_mut()else{return refuse(phase_error())};
  child.advance_nested(source,control,witness,body,maximum_work_units,child_grant(g,0)).map(|step|RetainedCloneStep::Progress(step.progress()))
 }
 fn advance_local<'r>(&mut self,original:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{
  if !matches!(self.select_advance(original,body,effect.maximum_work_units())?,NativeAdvanceSelection::Local{..}){return Ok(reply.complete(Err(phase_error().into())))}
  let d=self.demands(original,body,effect.maximum_work_units())?;if !retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units()?==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  match self.phase{
   Phase::Bind if !original.check_binding(&self.binding)?=>{let turn=original.bind(&mut self.binding,effect);return Ok(reply.complete(turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())).map_err(Into::into)))},
   Phase::Initialize if !self.builder.reserved()=>{let turn=self.builder.reserve_empty(effect);return native_record_pure_take(turn.map_err(Into::into),reply)},
   Phase::Initialize if !self.builder.initialized()=>{let count=self.count;let turn=self.builder.initialize_seed(effect,||Ok(PersistentListBuilder::new(count)));return native_record_pure_take(turn.map_err(Into::into),reply)},
   Phase::RowBirth if !self.child.reserved()=>{let turn=self.child.reserve_empty(effect);return native_record_pure_take(turn.map_err(Into::into),reply)},
   Phase::RowBirth if !self.child.initialized()=>{if let Some(template)=<Row<I,T,C,S> as NativeReceiver<I::Row,SealedShared<T>>>::seed_template(false){let turn=self.child.initialize_template(template,effect);return Ok(reply.complete(turn.result.map(relabel).map_err(Into::into)))}let turn=self.child.initialize_seed(effect,||Ok(NativeSharedReceiver::new(false)));return native_record_pure_take(turn.map_err(Into::into),reply)},
   Phase::RowAdvance if !self.child()?.output_ready()=>{let source=self.row_source(original)?;return self.child.get_mut().ok_or_else(phase_error)?.advance_local(source,body,effect,reply.progress())},
   Phase::RowTake if self.sealed.is_none()=>return self.child.get_mut().ok_or_else(phase_error)?.take(&mut self.sealed,effect,reply.progress()),
   Phase::RowClose if self.child_needs_close()=>{let mut answer=None;self.child_backing.close_into::<I::Row,SealedShared<T>,Row<I,T,C,S>>(&mut self.child,&mut self.child_retired,body,effect,OriginalEffectReply::borrow(&mut answer)?)?;return Ok(relay(reply,answer))},
   Phase::RowAdopt if self.sealed.is_some()=>{let mut answer=None;self.builder.get_mut().ok_or_else(phase_error)?.adopt_into(&mut self.sealed,effect,OriginalEffectReply::borrow(&mut answer)?)?;return Ok(relay(reply,answer))},
   Phase::Build if !self.builder()?.output_ready()&&!self.builder()?.accepts_row()=>{let mut answer=None;self.builder.get_mut().ok_or_else(phase_error)?.advance_into(effect,OriginalEffectReply::borrow(&mut answer)?)?;return Ok(relay(reply,answer))},
   Phase::Finish if !self.builder()?.finishing()=>{let mut answer=None;self.builder.get_mut().ok_or_else(phase_error)?.finish_into(effect,OriginalEffectReply::borrow(&mut answer)?)?;return Ok(relay(reply,answer))},
   Phase::FinalTake=>{let mut answer=None;self.builder.get_mut().ok_or_else(phase_error)?.take_into(&mut self.output,effect,OriginalEffectReply::borrow(&mut answer)?)?;if self.output.is_some(){self.phase=Phase::Ready}return Ok(match answer{Some(Ok(step))=>reply.complete(Ok(if self.phase==Phase::Ready{RetainedCloneStep::Complete(step.progress())}else{relabel(step)})),other=>relay(reply,other)})},
   _=>{}}
  if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"native list phase transition exceeded its Work").into())))}
  match self.phase{
   Phase::Bind=>self.phase=Phase::Count,
   Phase::Count=>{self.count=original.get().row_count()?;self.phase=Phase::Initialize},
   Phase::Initialize=>self.phase=if self.count==0{Phase::Finish}else{Phase::RowBirth},
   Phase::RowBirth=>self.phase=Phase::RowAdvance,
   Phase::RowAdvance=>self.phase=Phase::RowTake,
   Phase::RowTake=>self.phase=Phase::RowClose,
   Phase::RowClose=>self.phase=Phase::RowAdopt,
   Phase::RowAdopt=>{self.index+=1;self.phase=self.next_row_phase()?},
   Phase::Build=>self.phase=if self.builder()?.output_ready(){Phase::FinalTake}else if self.index<self.count{Phase::RowBirth}else{Phase::Finish},
   Phase::Finish=>self.phase=Phase::Build,
   _=>return Err(phase_error())}
  Ok(reply.complete(Ok(RetainedCloneStep::Progress(receipt(d)))))
 }
 fn recover_advance_local<'r>(&mut self,original:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{match self.phase{
  Phase::RowAdvance=>{let source=self.row_source(original)?;match self.child.get_mut(){Some(child)=>child.recover_advance_local(source,body,effect,reply.progress()),None=>Ok(reply.pending())}},
  Phase::RowTake=>match self.child.get_mut(){Some(child)=>child.recover_take(&mut self.sealed,effect,reply.progress()),None=>Ok(reply.pending())},
  Phase::RowClose if !self.child_backing.terminal_is_empty()=>{let mut answer=None;self.child_backing.recover_close_into::<I::Row,SealedShared<T>,Row<I,T,C,S>>(&mut self.child,&mut self.child_retired,body,effect,OriginalEffectReply::borrow(&mut answer)?)?;Ok(relay(reply,answer))},
  _=>Ok(reply.pending())}}
 fn advance_pending(&self)->bool{!self.child_backing.terminal_is_empty()||(matches!(self.phase,Phase::RowAdvance|Phase::RowTake)&&self.child.get().is_some_and(NativeReceiver::advance_pending))}
 fn output_ready(&self)->bool{!self.closing&&self.phase==Phase::Ready&&self.output.is_some()}
 semio_framework_value_native_receiver_protocol::native_owned_output_state!(PersistentList<T>,phase_error,phase,Phase::Spent);
 semio_framework_value_native_receiver_protocol::native_receiver_close_start!(closing);
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.child_needs_close(){return self.child_backing.demands::<I::Row,SealedShared<T>,Row<I,T,C,S>>(&self.child,&self.child_retired,body)}
  if self.builder_needs_close(){return self.builder_close_demand(body)}
  if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<T>(false,&None,body)}
  if self.sealed.is_some(){return self.retirement.next_owner_demands::<SealedShared<T>>(true,&None,body)}
  if self.output.is_some(){return self.retirement.next_owner_demands::<PersistentList<T>>(true,&None,body)}
  RetainedCloneBinding::close_demand(&self.binding,body)
 }
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  if self.child_needs_close(){return self.child_backing.work_demand::<I::Row,SealedShared<T>,Row<I,T,C,S>>(&self.child,&self.child_retired,body)}
  if self.builder_needs_close(){return self.builder_close_work(body)}
  if !self.retirement.is_empty(){return self.retirement.next_owner_work::<T>(false,&None,body)}
  if self.sealed.is_some(){return self.retirement.next_owner_work::<SealedShared<T>>(true,&None,body)}
  if self.output.is_some(){return self.retirement.next_owner_work::<PersistentList<T>>(true,&None,body)}
  RetainedCloneBinding::close_work_demand(&self.binding,body)
 }
 fn close_step<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{self.close_dispatch(None,body,effect,reply,false)}
 fn recover_close_step<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{self.close_dispatch(None,body,effect,reply,true)}
 fn requires_original_birth(&self)->bool{if let Some(child)=self.child.get().filter(|child|!child.terminal_is_empty()){return child.requires_original_birth()}if let Some(builder)=self.builder.get().filter(|builder|!builder.terminal_is_empty()){return builder.requires_original_birth()}self.retirement.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->crate::OriginalRetirementPoll{
  if grant.maximum_items==0||grant.maximum_depth==0{return crate::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0})}
  let child=child_grant(grant,0);
  if let Some(row)=self.child.get_mut().filter(|row|!row.terminal_is_empty()&&row.requires_original_birth()){return row.close_original_birth_step(original,child)}
  if let Some(builder)=self.builder.get_mut().filter(|builder|!builder.terminal_is_empty()&&builder.requires_original_birth()){return builder.close_original_birth_step(original,child)}
  self.retirement.step_original_birth(original,grant)
 }
 fn requires_original_control(&self)->bool{if let Some(original)=self.child_backing.original_route(){return original}if let Some(child)=self.child.get().filter(|child|!child.terminal_is_empty()){return child.requires_original_control()}if let Some(builder)=self.builder.get().filter(|builder|!builder.terminal_is_empty()){return builder.requires_original_control()}self.retirement.requires_original_control()}
 fn close_original_step<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{self.close_dispatch(Some(original),body,effect,reply,false)}
 fn recover_close_original_step<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CloseReply<'r>)->Result<CloseComplete<'r>,ValueError>{self.close_dispatch(Some(original),body,effect,reply,true)}
 fn terminal_is_empty(&self)->bool{self.child_backing.terminal_is_empty()&&self.child.terminal_is_empty()&&self.child_retired.is_none()&&self.builder.terminal_is_empty()&&self.builder_retired.is_none()&&self.sealed.is_none()&&self.output.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
 fn native_terminal_release(&self)->Option<semio_framework_value_native_receiver_protocol::RetainedCursorRelease<Self>>{self.terminal_is_empty().then(||unsafe{semio_framework_value_native_receiver_protocol::RetainedCursorRelease::native(|owner:&Self|owner.terminal_is_empty())})}
}
impl<I:NativeListSource+?Sized,T:crate::retirement::RetireOwned+Send+Sync+'static,C:NativeReceiver<I::Row,T>,S:NativeReceiverSeed<I::Row,T,C>> Drop for NativePersistentListReceiver<I,T,C,S>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"native persistent receiving retains original physical owners until funded closure");}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
