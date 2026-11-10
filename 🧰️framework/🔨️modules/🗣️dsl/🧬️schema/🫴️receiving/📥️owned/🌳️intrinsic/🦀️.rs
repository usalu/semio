//! 🌳️ The exact original source and native receiver remain in real funded storage through publication and cancellation.
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
use super::*;
use semio_framework_value::{DslValue,ErasedSnapshotRetirement,OriginalRetirementFailure,retained_clone::{OriginalEffectReply,RetainedCloneSource,RetainedCloneSourceAdmission},retirement::{RetirementCursor,RetirementStep}};
use semio_framework_value_native_receiving_original::StagedProjection;
use semio_framework_value_native_owned::native_decoding::{NativeReceive,NativeReceivingFailure,OriginalReceivingPrimary};
use semio_framework_value_native_storage::NativeOwnedStorage;
/// 🧬️ The declaration identifies the original intrinsic field without allocating a metadata roster.
pub trait NativeIntrinsicRecord:NativeReceive<DslValue>+Sync{const INTRINSIC_FIELD_ID:u16;}
/// 🏷️ An original source is any retired, shareable native tree the receiver reads in place.
pub trait SourceOwner:RetireOwned+Send+Sync+'static{}
impl<X:RetireOwned+Send+Sync+'static>SourceOwner for X{}
/// 🏁️ A received output is natively receivable from its source and retires through a paid cursor.
pub trait SourceReceived<I>:NativeReceive<I>+RetireOwned+Send+Sync+'static{}
impl<I,X:NativeReceive<I>+RetireOwned+Send+Sync+'static>SourceReceived<I>for X{}
/// 🧭️ The origin declares where the original source sits inside the moved field and moves it out without copying.
pub trait NativeSourceOrigin<I>:'static{
 /// 🔖️ The record field id holding the source, or none when the moved field is itself the source.
 const RECORD_FIELD:Option<u16>;
 fn check(field:&Option<FieldValue>)->Result<(),ValueError>;
 fn extract(held:&mut Option<FieldValue>)->Result<I,ValueError>;
}
/// 🌳️ The source is the intrinsic literal held by one declared field of a moved record.
pub struct IntrinsicOrigin<T>(std::marker::PhantomData<fn()->T>);
impl<T:NativeIntrinsicRecord>NativeSourceOrigin<DslValue>for IntrinsicOrigin<T>{
 const RECORD_FIELD:Option<u16>=Some(T::INTRINSIC_FIELD_ID);
 fn check(field:&Option<FieldValue>)->Result<(),ValueError>{if matches!(field,Some(FieldValue::Record(_))){Ok(())}else{Err(invalid())}}
 fn extract(held:&mut Option<FieldValue>)->Result<DslValue,ValueError>{let Some(FieldValue::Record(record))=held.as_mut()else{return Err(invariant())};let field=record.fields.get_mut(&T::INTRINSIC_FIELD_ID).ok_or_else(invalid)?;if !matches!(field,FieldValue::Value(_)){return Err(invalid())}let FieldValue::Value(value)=std::mem::replace(field,FieldValue::Absent)else{return Err(invariant())};Ok(value)}
}
/// 🧊️ The source is the whole moved field, so any native tree reads it directly through its field source port.
pub struct WholeOrigin;
impl NativeSourceOrigin<FieldValue>for WholeOrigin{
 const RECORD_FIELD:Option<u16>=None;
 fn check(field:&Option<FieldValue>)->Result<(),ValueError>{if field.is_some(){Ok(())}else{Err(invalid())}}
 fn extract(held:&mut Option<FieldValue>)->Result<FieldValue,ValueError>{held.take().ok_or_else(invariant)}
}
type Receiver<I,T>=NativeReceivingOriginal<I,T>;
/// 🎒️ Every source, unfinished native child and completed output remains in its authentic owner.
pub struct NativeSourceReceiving<I:SourceOwner,T:SourceReceived<I>,O:NativeSourceOrigin<I>>{record:OwnedSlot<FieldValue>,original:OwnedSlot<I>,admission:ManuallyDrop<Option<RetainedCloneSourceAdmission<I,I>>>,source:ManuallyDrop<Option<RetainedCloneSource<I>>>,receiver:NativeOwnedStorage<Receiver<I,T>>,retired:ManuallyDrop<Option<Box<Receiver<I,T>>>>,output:OwnedSlot<T>,shape:DeclaredRecordShape,projection:StagedProjection<I>,phase:u8,closing:bool,failed:bool,origin:std::marker::PhantomData<fn()->O>}
impl<I:SourceOwner,T:SourceReceived<I>,O:NativeSourceOrigin<I>>Default for NativeSourceReceiving<I,T,O>{fn default()->Self{Self{record:Default::default(),original:Default::default(),admission:ManuallyDrop::new(None),source:ManuallyDrop::new(None),receiver:NativeOwnedStorage::empty(),retired:ManuallyDrop::new(None),output:Default::default(),shape:Default::default(),projection:Default::default(),phase:0,closing:false,failed:false,origin:std::marker::PhantomData}}}
/// 🌳️ An intrinsic record receives the literal held by its one declared field.
pub type NativeIntrinsicReceiving<T>=NativeSourceReceiving<DslValue,T,IntrinsicOrigin<T>>;
/// 🪙️ Only a progress-bearing or complete close step may settle a receiver close turn.
fn step_progress(step:RetirementStep)->Result<RetainedCloneProgress,OriginalRetirementFailure>{match step{RetirementStep::Complete(_)|RetirementStep::Pending(_)|RetirementStep::BudgetExhausted|RetirementStep::Advanced=>Ok(Default::default()),RetirementStep::Progress(progress,_)=>Ok(progress),RetirementStep::Failure(failure,_)=>Err(failure),_=>Err(ValueError::invariant("original receiver close produced a foreign child step").into())}}
/// 🧯️ A control-driven child keeps every independent cause inside the complete native primary.
fn primary(failure:NativeReceivingFailure)->OriginalReceivingPrimary{match failure{NativeReceivingFailure::Primary(primary)=>primary,NativeReceivingFailure::ValueEffect(effect)=>{let mut receiving=effect.receiving;receiving.allocation_refusal=effect.child;receiving.work_refusal=effect.work_refusal;receiving},NativeReceivingFailure::RetirementEffect(effect)=>{let mut receiving=effect.receiving;receiving.retirement_failure=effect.child;receiving.work_refusal=effect.work_refusal;receiving},NativeReceivingFailure::ReceivingEffect(effect)=>{let mut receiving=effect.receiving;receiving.work_refusal=effect.work_refusal;receiving}}}
/// 🧾️ Settles one reply-shaped receiver close action into an owned closing turn; an unanswered action stays pending.
fn closed(effect:&RetainedAllocationEffect<'_>,ran:Result<(),ValueError>,answer:Option<Result<RetainedCloneProgress,OriginalRetirementFailure>>)->OwnedClose{match(ran,answer){(Err(error),_)=>done(effect,Err(error.into())),(Ok(()),Some(settled))=>done(effect,settled),(Ok(()),None)=>RetainedWorkPoll::Pending{work:effect.performed_work()}}}
/// ♻️ Pays the receiver close start once, then closes it through its reply-shaped retirement cursor.
fn receiver_close<I:SourceOwner,T:SourceReceived<I>>(child:&mut Receiver<I,T>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{
 if !child.is_closing(){let mut slot=None;let ran=OriginalEffectReply::borrow(&mut slot).and_then(|reply|child.begin_close_into(effect,reply).map(drop));return closed(effect,ran,slot.take().map(|settled|settled.map(|step|step.progress())))}
 let mut slot=None;let ran=OriginalEffectReply::borrow(&mut slot).and_then(|reply|RetirementCursor::close_into(child,body,effect,reply).map(drop));
 closed(effect,ran,slot.take().map(|settled|settled.and_then(step_progress)))
}
fn absent<R>(name:&'static str)->Result<R,ValueError>{Err(ValueError::invariant(name))}
impl<I:SourceOwner,T:SourceReceived<I>,O:NativeSourceOrigin<I>>NativeSourceReceiving<I,T,O>{
 fn taking(&self)->bool{self.admission.as_ref().is_some_and(|admission|!admission.terminal_is_empty())}
 fn admission_quote(&self)->Result<(RetirementDemand,u64),ValueError>{let admission=self.admission.as_ref().ok_or_else(||ValueError::invariant("admission absent"))?;match self.projection.quote(admission)?{Some(quote)=>Ok(quote),None=>Ok((admission.next_demand()?,admission.advance_work_demand()?))}}
 fn receiver_ref(&self)->Result<&Receiver<I,T>,ValueError>{self.receiver.get().ok_or_else(||ValueError::invariant("get target absent"))}
 fn receiver_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.retired.is_some(){return Ok(storage::header_release_demand::<Receiver<I,T>>())}if let Some(child)=self.receiver.get(){return if child.terminal_is_empty(){Ok(NativeOwnedStorage::<Receiver<I,T>>::box_demand())}else if !child.is_closing(){child.begin_close_demand().and_then(nested)}else{child.close_demands(body).and_then(nested)}}Ok(self.receiver.release_demand())}
 fn receiver_close_work(&self,body:usize)->Result<u64,ValueError>{if self.retired.is_some(){return Ok(1)}if let Some(child)=self.receiver.get(){return if child.terminal_is_empty(){Ok(self.receiver.box_work_demand())}else if !child.is_closing(){child.begin_close_work_demand()}else{child.close_work_demand(body)}}Ok(self.receiver.release_work_demand())}
 fn close_receiver(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{
  let g=effect.grant();
  if let Some(header)=self.retired.as_ref(){if !header.terminal_is_empty(){return done(effect,Err(invariant().into()))}return unsafe{storage::release_empty_header(&mut self.retired,effect)}}
  if let Some(child)=self.receiver.get_mut(){if !child.terminal_is_empty(){let poll=receiver_close(child,g.maximum_copy_bytes,&mut effect.nested(child_grant(g)));return settle(effect,poll)}let poll=closing(physical(self.receiver.take_box_into(&mut self.retired,effect)));return settle(effect,poll)}
  let poll=closing(physical(RetainedWorkPoll::Complete(self.receiver.release(effect))));settle(effect,poll)
 }
 fn selected(&mut self,input:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{
  if self.closing||self.failed{return done(effect,Err(invariant().into()))}if self.phase==13{return idle(effect)}
  let d=match self.demands(input,body){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};if !admits(effect,d){return idle(effect)}
  let g=effect.grant();
  match self.phase{
   0 if O::RECORD_FIELD.is_some()&&!self.shape.is_ready()=>{let (Some(FieldValue::Record(record)),Some(field))=(input.as_ref(),O::RECORD_FIELD)else{return done(effect,Err(invalid().into()))};let result=self.shape.step(record,&[(field,true)],effect).map_err(Into::into);return done(effect,result)},
   3 if self.admission.as_ref().is_some_and(|a|a.phase()!="ready")=>{let Self{admission,projection,..}=self;let Some(admission)=admission.as_mut()else{return done(effect,Err(ValueError::invariant("admission absent").into()))};let quote=projection.quote(admission);let poll=match quote{Ok(Some(_))=>match projection.advance(admission,effect){RetainedWorkPoll::Pending{..}=>RetainedWorkPoll::Pending{work:effect.performed_work()},RetainedWorkPoll::Complete(turn)=>done(effect,turn.result.map_err(Into::into))},Ok(None)=>normal(physical(RetainedWorkPoll::Complete(admission.advance(effect)))),Err(error)=>done(effect,Err(error.into()))};return settle(effect,poll)},
   4=>{let result=match self.admission.as_mut(){Some(admission)=>{admission.take_output(&mut self.source,effect).result.map(|step|step.progress())},None=>absent("admission absent")};return done(effect,result.map_err(Into::into))},
   6=>{let poll=normal(physical(self.receiver.reserve_empty(effect)));return settle(effect,poll)},
   7=>{let source=&mut self.source;let poll=normal(physical(self.receiver.initialize_seed(effect,||Ok(NativeReceivingOriginal::new(source.take().ok_or_else(||ValueError::invariant("source absent"))?)))));return settle(effect,poll)},
   9 if self.output.value.is_none()=>{let poll=match self.receiver.get_mut(){Some(receiver)=>receiver.take(&mut self.output.value,effect).map(|step|step.progress()),None=>return done(effect,Err(ValueError::invariant("get_mut target absent").into()))};return settle(effect,poll)},
   10 if self.receiver.get().is_some_and(|child|!child.is_closing()&&!child.terminal_is_empty())=>{let poll=match self.receiver.get_mut(){Some(child)=>receiver_close(child,g.maximum_copy_bytes,&mut effect.nested(child_grant(g))),None=>return done(effect,Err(invariant().into()))};return settle(effect,receiving(poll))},
   11 if !self.receiver.terminal_is_empty()||self.retired.is_some()=>{let poll=self.close_receiver(effect);return settle(effect,receiving(poll))},
   12 if !self.record.terminal_is_empty()=>{let poll=self.record.close_step(effect);return settle(effect,receiving(poll))},
   _=>{}
  }
  local(effect,d,||{match self.phase{
   0=>{*self.record.value=input.take();self.phase=1;},
   1=>{*self.original.value=Some(O::extract(&mut self.record.value)?);self.phase=2;},
   2=>{*self.admission=Some(RetainedCloneSource::admission_owned(self.original.value.take().ok_or_else(invariant)?,()));self.phase=3;},
   3=>self.phase=4,
   5=>{if !self.admission.as_ref().is_some_and(|a|a.terminal_is_empty()){return Err(invariant())}self.admission.take();self.phase=6;},
   8=>self.phase=9,
   10=>self.phase=11,
   11=>self.phase=12,
   12=>self.phase=13,
   _=>return Err(invariant())}Ok(())})
 }
 fn advanced(&mut self,poll:OwnedPoll)->OwnedPoll{
  if let RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(_),..})=&poll{match self.phase{4 if self.source.is_some()&&!self.taking()=>self.phase=5,6 if self.receiver.reserved()=>self.phase=7,7 if self.receiver.initialized()=>self.phase=8,9 if self.output.value.is_some()=>self.phase=10,_=>{}}}
  poison(&mut self.failed,&poll);poll
 }
}
impl<I:SourceOwner,T:SourceReceived<I>,O:NativeSourceOrigin<I>>OwnedFieldReceiving<T>for NativeSourceReceiving<I,T,O>{
 fn demands(&self,input:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.failed{return Err(invariant())}match self.phase{
  0=>{O::check(input)?;Ok(metadata::<()>())},
  3 if self.admission.as_ref().is_some_and(|a|a.phase()!="ready")=>self.admission_quote().map(|quote|quote.0),
  4 if self.taking()=>Ok(self.admission.as_ref().ok_or_else(||ValueError::invariant("admission absent"))?.take_output_demand()),
  6 if !self.receiver.reserved()=>Ok(NativeOwnedStorage::<Receiver<I,T>>::reserve_demand()),
  7 if !self.receiver.initialized()=>Ok(NativeOwnedStorage::<Receiver<I,T>>::seed_demand()),
  8 if !self.receiver_ref()?.output_ready()=>self.receiver_ref()?.demands(body,u64::MAX).and_then(nested),
  9 if self.output.value.is_none()=>self.receiver_ref()?.take_demand().and_then(nested),
  10 if self.receiver.get().is_some_and(|child|!child.is_closing()&&!child.terminal_is_empty())=>self.receiver_close_demand(body),
  11 if !self.receiver.terminal_is_empty()||self.retired.is_some()=>self.receiver_close_demand(body),
  12 if !self.record.terminal_is_empty()=>self.record.close_demands(body),13=>Ok(Default::default()),_=>Ok(metadata::<()>())}}
 fn select_advance(&self,_:&Option<FieldValue>,body:usize,_:u64)->Result<OwnedAdvanceSelection,ValueError>{if self.closing||self.failed{return Err(invariant())}let local=|work:u64|Ok(OwnedAdvanceSelection::Local{work});match self.phase{
  8 if !self.receiver_ref()?.output_ready()=>Ok(OwnedAdvanceSelection::Nested),
  3 if self.admission.as_ref().is_some_and(|a|a.phase()!="ready")=>local(self.admission_quote()?.1),
  6 if !self.receiver.reserved()=>local(self.receiver.reserve_work_demand()),7 if !self.receiver.initialized()=>local(self.receiver.seed_work_demand()),
  4 if self.taking()=>local(self.admission.as_ref().ok_or_else(||ValueError::invariant("admission absent"))?.take_output_work_demand()),
  9 if self.output.value.is_none()=>local(self.receiver_ref()?.take_work_demand()?),
  10 if self.receiver.get().is_some_and(|child|!child.is_closing()&&!child.terminal_is_empty())=>local(self.receiver_close_work(body)?),
  11 if !self.receiver.terminal_is_empty()||self.retired.is_some()=>local(self.receiver_close_work(body)?),
  12 if !self.record.terminal_is_empty()=>local(self.record.close_work_demand(body)?),13=>local(0),_=>local(1)}}
 fn advance_nested(&mut self,_:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->OwnedPoll{
  if self.closing||self.failed||self.phase!=8{return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invariant().into()),work:0})}
  let poll=match self.receiver.get_mut(){Some(receiver)if !receiver.output_ready()=>physical(receiver.advance(native,witness,body,maximum_work_units,child_grant(g)).map_err(primary)),_=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(invariant().into()),work:0})};self.advanced(poll)
 }
 fn advance_local(&mut self,input:&mut Option<FieldValue>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OwnedPoll{let poll=self.selected(input,body,effect);self.advanced(poll)}
 fn is_ready(&self)->bool{!self.closing&&!self.failed&&self.phase==13&&self.output.value.is_some()}
 fn take_into(&mut self,output:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->OwnedTake{if !self.is_ready(){return done(effect,Err(invariant()))}move_into(&mut self.output.value,output,effect)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if OwnedFieldReceiving::terminal_is_empty(self){return Ok(Default::default())}if !self.closing{return Ok(metadata::<()>())}if !self.receiver.terminal_is_empty()||self.retired.is_some(){return self.receiver_close_demand(body)}if let Some(a)=self.admission.as_ref(){return if a.terminal_is_empty(){Ok(metadata::<()>())}else if let Some((demand,_))=self.projection.close_quote(a)?{Ok(demand)}else if a.phase()=="ready"{Ok(a.take_output_demand())}else{a.next_return_demand()}}if let Some(s)=self.source.as_ref(){return if s.terminal_is_empty(){Ok(metadata::<()>())}else{s.next_close_demand(body)}}if !self.original.terminal_is_empty(){return self.original.close_demands(body)}if !self.output.terminal_is_empty(){return self.output.close_demands(body)}self.record.close_demands(body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if OwnedFieldReceiving::terminal_is_empty(self){return Ok(0)}if !self.closing{return Ok(1)}if !self.receiver.terminal_is_empty()||self.retired.is_some(){return self.receiver_close_work(body)}if let Some(a)=self.admission.as_ref(){return if a.terminal_is_empty(){Ok(1)}else if let Some((_,work))=self.projection.close_quote(a)?{Ok(work)}else if a.phase()=="ready"{Ok(a.take_output_work_demand())}else{a.return_work_demand()}}if let Some(s)=self.source.as_ref(){return if s.terminal_is_empty(){Ok(1)}else{s.next_close_work_demand(body)}}if !self.original.terminal_is_empty(){return self.original.close_work_demand(body)}if !self.output.terminal_is_empty(){return self.output.close_work_demand(body)}self.record.close_work_demand(body)}
 fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OwnedClose{
  if OwnedFieldReceiving::terminal_is_empty(self){return idle(effect)}let g=effect.grant();let d=match self.close_demands(g.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return done(effect,Err(error.into()))};if !admits(effect,d){return idle(effect)}
  if !self.closing{return local_close(effect,d,||{self.closing=true;Ok(())})}
  if !self.receiver.terminal_is_empty()||self.retired.is_some(){return self.close_receiver(effect)}
  if let Some(a)=self.admission.as_mut(){
   if a.terminal_is_empty(){return local_close(effect,d,||{self.admission.take();Ok(())})}
   match self.projection.close_quote(a){Ok(Some(_))=>{let result=self.projection.close(a,effect);return done(effect,result.map_err(Into::into))},Ok(None)=>{},Err(error)=>return done(effect,Err(error.into()))}
   if a.phase()=="ready"{let result=a.take_output(&mut self.source,effect).result.map(|step|step.progress()).map_err(Into::into);return done(effect,result)}
   let mut original=None;let turn=a.return_inputs(&mut original,effect);if let Some((value,()))=original{*self.original.value=Some(value)}return settle(effect,physical(RetainedWorkPoll::Complete(turn).map_err(Into::into)))
  }
  if let Some(s)=self.source.as_mut(){if s.terminal_is_empty(){return local_close(effect,d,||{self.source.take();Ok(())})}let poll=s.close_step(effect.grant().maximum_copy_bytes,effect);return settle(effect,physical(poll))}
  if !self.original.terminal_is_empty(){return self.original.close_step(effect)}if !self.output.terminal_is_empty(){return self.output.close_step(effect)}self.record.close_step(effect)
 }
 fn terminal_is_empty(&self)->bool{self.record.terminal_is_empty()&&self.original.terminal_is_empty()&&self.admission.is_none()&&self.projection.terminal_is_empty()&&self.source.is_none()&&self.receiver.terminal_is_empty()&&self.retired.is_none()&&self.output.terminal_is_empty()}
}
impl<I:SourceOwner,T:SourceReceived<I>,O:NativeSourceOrigin<I>>Drop for NativeSourceReceiving<I,T,O>{fn drop(&mut self){assert!(std::thread::panicking()||OwnedFieldReceiving::terminal_is_empty(self),"intrinsic receiving requires funded closure");}}
impl<I:SourceOwner,T:SourceReceived<I>,O:NativeSourceOrigin<I>>NativeSourceReceiving<I,T,O>{
 /// 🎯️ Selects the unchanged actual close frontier for this original owner.
 pub fn select_close(&mut self,body:usize)->Result<semio_framework_value::RetirementSelection<'_,Self>,ValueError>{semio_framework_value::select_retirement_cursor(self,body)}
}
impl<I:SourceOwner,T:SourceReceived<I>,O:NativeSourceOrigin<I>>ErasedSnapshotRetirement for NativeSourceReceiving<I,T,O>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{OwnedFieldReceiving::close_demands(self,body)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{OwnedFieldReceiving::close_work_demand(self,body)}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,usize::MAX)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,body)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,usize::MAX)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(OwnedFieldReceiving::close_demands(self,usize::MAX)?.depth)}
 fn close_step(&mut self,_:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{OwnedFieldReceiving::close_step(self,effect).map(RetainedCloneStep::Progress)}
 fn terminal_is_empty(&self)->bool{OwnedFieldReceiving::terminal_is_empty(self)}
}
