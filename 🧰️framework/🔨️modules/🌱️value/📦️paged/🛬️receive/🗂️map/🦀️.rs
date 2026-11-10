//! 🗂️ Original map keys are admitted through a paged scalar hash index before typed native entries receive custody.
use super::*;
use super::super::PagedMap;
use semio_framework_value_persistent::list::PagedList;
use semio_framework_value_native_owned::native_decoding::owned::tree::{NativeSequenceReceiver,RecordEntries};
use semio_framework_value_native_owned::native_decoding::{OriginalEffectReply,OriginalEffectReplyComplete};
use crate::RetainedWorkOutcome;
use semio_framework_value_persistent::RetainedWorkPoll;
use semio_framework_value_native_owned::native_decoding::owned::{native_relay,native_output_demand,native_output_take,native_record_pure_take,NativeAdvanceSelection,receiving_capacity};
type Index=PagedList<Option<(u64,usize)>,{usize::MAX}>;
type Rows<V,const N:usize>=PagedList<(PagedUtf8<{usize::MAX}>,V),N>;
type Child<I,V,const N:usize>=NativeSequenceReceiver<I,(PagedUtf8<{usize::MAX}>,V),Rows<V,N>,RecordEntries>;
type Retiring=crate::OriginalRetirementFailure;
pub struct PagedMapReceiver<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync,const N:usize>{binding:Option<RetainedCloneBinding>,index:ManuallyDrop<Option<Index>>,child:ManuallyDrop<Option<Child<I,V,N>>>,entries:ManuallyDrop<Option<Rows<V,N>>>,output:ManuallyDrop<Option<PagedMap<V,N>>>,retirement:RetainedCloneClose,count:usize,slots:usize,item:usize,offset:usize,probe:usize,comparison:usize,hash:u64,phase:u8,closing:bool,fault:bool}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync,const N:usize> PagedMapReceiver<I,V,N>{
 fn rows(&self)->Result<&Index,ValueError>{self.index.as_ref().ok_or_else(invalid)}
 fn key<'a>(&self,input:&'a I,index:usize)->Result<&'a str,ValueError>{input.record_name(index)}
 fn duplicate()->ValueError{ValueError::literal(crate::ValueRefusalKind::InvalidValue,"native original map contains a duplicate key")}
 fn relay_close<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>,recover:bool)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  let begin=self.stage()==12;let g=effect.grant();let child=self.child.as_mut().ok_or_else(invalid)?;
  let answer=native_relay(reply.progress(),|inner|if begin{if recover{child.recover_begin_close_into(effect,inner)}else{child.begin_close_into(effect,inner)}}else if recover{child.recover_close_step(body,&mut effect.nested(child_grant(g,0)),inner)}else{child.close_step(body,&mut effect.nested(child_grant(g,0)),inner)})?;
  if begin&&child.is_closing(){self.phase=7}Ok(answer)
 }
 fn stage(&self)->u8{if self.phase==6&&self.entries.is_some(){12}else{self.phase}}
}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync,const N:usize> NativeReceive<I> for PagedMap<V,N>{
 type Receiver=PagedMapReceiver<I,V,N>;
 fn receiver()->Self::Receiver{PagedMapReceiver{binding:None,index:ManuallyDrop::new(None),child:ManuallyDrop::new(None),entries:ManuallyDrop::new(None),output:ManuallyDrop::new(None),retirement:Default::default(),count:0,slots:0,item:0,offset:0,probe:usize::MAX,comparison:0,hash:0xcbf29ce484222325,phase:0,closing:false,fault:false}}
}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync,const N:usize> NativeReceiver<I,PagedMap<V,N>> for PagedMapReceiver<I,V,N>{
 fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<RetirementDemand,ValueError>{if self.closing||self.phase==8||self.phase==9{return Err(invalid())}if self.fault{return Err(Self::duplicate())}input.check_binding(&self.binding)?;match self.stage(){
 0=>Ok(fixed(input.binding_copy_bytes())),1|4|10|11=>Ok(fixed(0)),
 2=>{let rows=self.rows()?;if rows.len()==self.slots{return Ok(fixed(0))}if !rows.has_reserved_slot(){nested(RetirementDemand{copy_bytes:rows.next_reserve_copy_byte_demand().map_err(|_|invalid())?,capacity_bytes:rows.next_allocation_bytes().map_err(|_|invalid())?,depth:rows.next_reserve_depth_demand().map_err(|_|invalid())?,..Default::default()},0)}else{Ok(RetirementDemand{copy_bytes:size_of::<Option<(u64,usize)>>(),depth:rows.next_push_depth_demand().map_err(|_|invalid())?.checked_add(1).ok_or_else(invalid)?,..Default::default()})}},
 3=>{if self.item==self.count||self.probe==usize::MAX{return Ok(fixed(0))}Ok(RetirementDemand{depth:self.rows()?.next_get_depth_demand(self.probe).map_err(|_|invalid())?.checked_add(1).ok_or_else(invalid)?,..Default::default()})},
  5=>{let child=self.child.as_ref().ok_or_else(invalid)?;if child.output_ready(){Ok(fixed(0))}else{nested(child.demands(input,body,maximum_work_units)?,0)}},6=>nested(self.child.as_ref().ok_or_else(invalid)?.take_demand()?,0),12=>self.child.as_ref().ok_or_else(invalid)?.begin_close_demand(),
  7=>if self.child.as_ref().ok_or_else(invalid)?.terminal_is_empty(){Ok(fixed(0))}else{self.child.as_ref().ok_or_else(invalid)?.close_demands(body)},_=>Err(invalid())}}
 fn select_advance(&self,input:RetainedCloneRef<'_,I>,body:usize,maximum_work_units:u64)->Result<NativeAdvanceSelection,ValueError>{
  if self.closing||self.phase==8||self.phase==9{return Err(invalid())}if self.fault{return Err(Self::duplicate())}
  if !input.check_binding(&self.binding)?{return Ok(NativeAdvanceSelection::Local{work:input.binding_work_demand(&self.binding)?})}
  Ok(match self.stage(){
   0|1|3|4|10|11=>NativeAdvanceSelection::Local{work:1},
   12=>NativeAdvanceSelection::Local{work:self.child.as_ref().ok_or_else(invalid)?.begin_close_work_demand()?},
   2=>{let rows=self.rows()?;NativeAdvanceSelection::Local{work:if rows.len()==self.slots||rows.has_reserved_slot(){1}else{rows.reserve_one_work_demand().map_err(|_|invalid())?}}},
   5=>{let child=self.child.as_ref().ok_or_else(invalid)?;if child.output_ready(){NativeAdvanceSelection::Local{work:1}}else{child.select_advance(input,body,maximum_work_units)?}},
   6=>NativeAdvanceSelection::Local{work:self.child.as_ref().ok_or_else(invalid)?.take_work_demand()?},
   7=>NativeAdvanceSelection::Local{work:{let child=self.child.as_ref().ok_or_else(invalid)?;if child.terminal_is_empty(){1}else{child.close_work_demand(body)?}}},
   _=>return Err(invalid())})
 }
 fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:OriginalSourceTurnWitness<'_>,body:usize,maximum_work_units:u64,g:RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>{
  use RetainedWorkPoll::Complete;
  let result:Result<RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>,NativeReceivingFailure>=(||{
   if maximum_work_units==0||self.closing||self.binding.is_none(){return Err(invalid().into())}
   let d=self.demands(input,body,maximum_work_units)?;if !retained_clone_grant_admits(g,d){return Ok(Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}))}
   
   Ok(match self.stage(){
    5=>self.child.as_mut().ok_or_else(invalid)?.advance_nested(input,control,witness,body,maximum_work_units,child_grant(g,0)).map(|s|RetainedCloneStep::Progress(s.progress())),
    _=>return Err(invalid().into())})
  })();
  match result{Ok(poll)=>poll,Err(error)=>Complete(RetainedWorkOutcome{result:Err(error),work:0})}
 }
 fn advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{
  if self.binding.is_some(){let child=self.child.as_mut().ok_or_else(invalid)?;if !child.output_ready(){return child.advance_local(input,body,effect,reply)}}
  if self.binding.is_some()&&self.phase==6&&self.entries.is_none(){return self.child.as_mut().ok_or_else(invalid)?.take(&mut self.entries,effect,reply)}
  if self.binding.is_some()&&matches!(self.stage(),7|12)&&self.child.as_ref().is_some_and(|child|!child.terminal_is_empty()){
   let d=match self.demands(input,body,effect.maximum_work_units()){Ok(d)=>d,Err(error)=>return Ok(reply.complete(Err(error.into())))};if !retained_clone_grant_admits(effect.grant(),d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   return self.relay_close(body,effect,reply,false)
  }
  let result:Result<RetainedCloneStep,OriginalReceivingPrimary>=(||{let d=self.demands(input,body,effect.maximum_work_units())?;if !retained_clone_grant_admits(effect.grant(),d){return Ok(RetainedCloneStep::Progress(Default::default()))}
   if self.binding.is_none(){let turn=input.bind(&mut self.binding,effect);return turn.result.map(|p|RetainedCloneStep::Progress(p.unwrap_or_default())).map_err(Into::into)}
   if self.stage()==2&&self.rows()?.len()!=self.slots&&!self.rows()?.has_reserved_slot(){let rows=self.index.as_mut().ok_or_else(invalid)?;let turn=rows.reserve_one_funded(effect);return match turn.result{Ok(semio_framework_value_persistent::list::PagedListReservationOutcome::Ready(progress))=>Ok(RetainedCloneStep::Progress(progress)),Ok(semio_framework_value_persistent::list::PagedListReservationOutcome::Denied)=>Ok(RetainedCloneStep::Progress(Default::default())),Err(semio_framework_value_persistent::list::PagedListReservationFailure::Work(error))=>Err(error.into()),Err(semio_framework_value_persistent::list::PagedListReservationFailure::Allocation{error,..})=>Err(invalid().with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:error.allocated_bytes,..Default::default()}).into())}}
   if !effect.performed(1)?{return Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"native paged map action exceeded its original work").into())}
   match self.stage(){
    0=>self.phase=1,
    1=>{let count=input.get().record_len()?;if count>N{return Err(invalid().with_retained_progress(receipt(d)).into())}let slots=if count==0{0}else{count.checked_mul(2).and_then(usize::checked_next_power_of_two).ok_or_else(invalid)?};self.count=count;self.slots=slots;self.phase=10;},
    10=>{*self.index=Some(Index::new());self.phase=2;},
    2=>{if self.rows()?.len()==self.slots{self.phase=3;}else{let rows=self.index.as_mut().ok_or_else(invalid)?;if rows.push_reserved(None).is_err(){return Err(invalid().with_retained_progress(receipt(d)).into())}}},
    3=>{if self.item==self.count{self.phase=4;}else{let key=self.key(input.get(),self.item)?;if self.probe==usize::MAX{if self.offset<key.len(){self.hash=(self.hash^u64::from(key.as_bytes()[self.offset])).wrapping_mul(0x100000001b3);self.offset+=1;}else{self.probe=self.hash as usize&(self.slots-1);self.comparison=0;}}else{let candidate=*self.rows()?.get(self.probe).ok_or_else(invalid)?;if let Some((hash,index))=candidate{let held=self.key(input.get(),index)?;if hash!=self.hash||held.len()!=key.len(){self.probe=(self.probe+1)&(self.slots-1);self.comparison=0;}else if self.comparison==key.len(){self.fault=true;return Err(Self::duplicate().with_retained_progress(receipt(d)).into())}else if key.as_bytes()[self.comparison]==held.as_bytes()[self.comparison]{self.comparison+=1;}else{self.probe=(self.probe+1)&(self.slots-1);self.comparison=0;}}else{let probe=self.probe;*self.index.as_mut().ok_or_else(invalid)?.get_mut(probe).ok_or_else(invalid)?=Some((self.hash,self.item));self.item+=1;self.offset=0;self.probe=usize::MAX;self.comparison=0;self.hash=0xcbf29ce484222325;}}}},
    4=>{*self.child=Some(Child::<I,V,N>::new());self.phase=5;},
    5=>self.phase=6,
    7=>{self.child.take();self.phase=11;},
    11=>{*self.output=Some(PagedMap::from_retained_entries_cloned(self.entries.take().ok_or_else(invalid)?));self.phase=8;},_=>return Err(invalid().into())}
   Ok(if self.phase==8{RetainedCloneStep::Complete(receipt(d))}else{RetainedCloneStep::Progress(receipt(d))})})();
  Ok(reply.complete(result))
 }
 fn recover_advance_local<'reply>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{if matches!(self.stage(),7|12)&&self.child.as_ref().is_some_and(|child|!child.terminal_is_empty()){return self.relay_close(body,effect,reply,true)}if self.phase==5{if let Some(child)=self.child.as_mut(){return child.recover_advance_local(input,body,effect,reply)}}Ok(reply.pending())}
 fn output_ready(&self)->bool{!self.closing&&self.phase==8&&self.output.is_some()}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.output_ready(){return Err(invalid())}native_output_demand::<PagedMap<V,N>>(size_of::<u8>())}
 fn take_work_demand(&self)->Result<u64,ValueError>{self.take_demand().map(|_|1)}
 fn take<'reply>(&mut self,output:&mut Option<PagedMap<V,N>>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,OriginalReceivingPrimary>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,OriginalReceivingPrimary>,ValueError>{if !self.output_ready(){return Ok(reply.complete(Err(invalid().into())))}let turn=native_output_take(&mut self.output,output,size_of::<u8>(),effect);if matches!(&turn,RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(_)),..})){self.phase=9}native_record_pure_take(turn,reply)}
 fn advance_pending(&self)->bool{self.child.as_ref().is_some_and(|child|child.advance_pending())}
 fn is_closing(&self)->bool{self.closing}
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{Ok(if self.closing{Default::default()}else{fixed(size_of::<bool>())})}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(!self.closing))}
 fn begin_close_into<'reply>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,Retiring>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,Retiring>,ValueError>{if self.closing{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}let d=fixed(size_of::<bool>());if !retained_clone_grant_admits(effect.grant(),d)||effect.remaining_work_units()?<1{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"native paged map close start exceeded its work authority").into())))}self.closing=true;Ok(reply.complete(Ok(RetainedCloneStep::Complete(receipt(d)))))}
 fn recover_begin_close_into<'reply>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,Retiring>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,Retiring>,ValueError>{self.begin_close_into(effect,reply)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.closing{return Err(invalid())}if let Some(child)=self.child.as_ref(){return if child.terminal_is_empty(){Ok(fixed(0))}else if child.is_closing(){child.close_demands(body)}else{child.begin_close_demand()}}if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<PagedMap<V,N>>(false,&None,body)}if self.output.is_some(){return self.retirement.next_owner_demands::<PagedMap<V,N>>(true,&None,body)}if self.entries.is_some(){return self.retirement.next_owner_demands::<Rows<V,N>>(true,&None,body)}if self.index.is_some(){return self.retirement.next_owner_demands::<Index>(true,&None,body)}if self.fault{return Ok(fixed(0))}RetainedCloneBinding::close_demand(&self.binding,body)}
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{if !self.closing{return Err(invalid())}if let Some(child)=self.child.as_ref(){return if child.terminal_is_empty(){Ok(1)}else if child.is_closing(){child.close_work_demand(body)}else{child.begin_close_work_demand()}}if !self.retirement.is_empty(){return self.retirement.next_owner_work::<PagedMap<V,N>>(false,&None,body)}if self.output.is_some(){return self.retirement.next_owner_work::<PagedMap<V,N>>(true,&None,body)}if self.entries.is_some(){return self.retirement.next_owner_work::<Rows<V,N>>(true,&None,body)}if self.index.is_some(){return self.retirement.next_owner_work::<Index>(true,&None,body)}if self.fault{return Ok(1)}RetainedCloneBinding::close_work_demand(&self.binding,body)}
 fn close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,Retiring>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,Retiring>,ValueError>{
  if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
  let g=effect.grant();let d=self.close_demands(body)?;if !retained_clone_grant_admits(g,d){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
  if let Some(child)=self.child.as_mut(){
   if !child.terminal_is_empty(){return if child.is_closing(){child.close_step(body,&mut effect.nested(g),reply.progress())}else{child.begin_close_into(effect,reply.progress())}}
   if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"native paged map child release exceeded its work authority").into())))}
   self.child.take();return Ok(reply.complete(Ok(RetainedCloneStep::Progress(receipt(d)))))
  }
  if !self.retirement.is_empty(){return self.retirement.step_into(body,effect,reply)}
  macro_rules! close{($field:ident)=>{if self.$field.is_some(){let turn=self.retirement.begin_granted(&mut self.$field,effect);return Ok(reply.complete(turn.result.map(|s|s.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}}}
  close!(output);close!(entries);close!(index);
  if self.fault{if !effect.performed(1)?{return Ok(reply.complete(Err(ValueError::literal(crate::ValueRefusalKind::WorkLimit,"native paged map fault release exceeded its work authority").into())))}self.fault=false;return Ok(reply.complete(Ok(RetainedCloneStep::Progress(receipt(d)))))}
  Ok(match RetainedCloneBinding::close_one(&mut self.binding,body,effect){RetainedWorkPoll::Pending{..}=>reply.pending(),RetainedWorkPoll::Complete(turn)=>reply.complete(turn.result)})
 }
 fn recover_close_step<'reply>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,Retiring>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,Retiring>,ValueError>{
  let g=effect.grant();
  if let Some(child)=self.child.as_mut(){if !child.terminal_is_empty(){return if child.is_closing(){child.recover_close_step(body,&mut effect.nested(g),reply.progress())}else{child.recover_begin_close_into(effect,reply.progress())}}}
  if !self.retirement.is_empty(){return self.retirement.recover_step_into(body,effect,reply)}
  Ok(reply.pending())
 }
 fn requires_original_birth(&self)->bool{if let Some(child)=self.child.as_ref(){return !child.terminal_is_empty()&&child.requires_original_birth()}self.retirement.requires_original_birth()}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value::OriginalRetirementPoll{if let Some(child)=self.child.as_mut(){return child.close_original_birth_step(original,grant)}self.retirement.step_original_birth(original,grant)}
 fn requires_original_control(&self)->bool{if let Some(child)=self.child.as_ref(){return !child.terminal_is_empty()&&child.requires_original_control()}self.retirement.requires_original_control()}
 fn close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,Retiring>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,Retiring>,ValueError>{
  if self.requires_original_birth(){return Err(ValueError::literal(crate::ValueRefusalKind::UnsupportedOwner,"settle original birth first"))}
  let g=effect.grant();
  if let Some(child)=self.child.as_mut(){if !child.terminal_is_empty()&&child.is_closing()&&child.requires_original_control(){return child.close_original_step(original,body,&mut effect.nested(g),reply.progress())}}else if self.retirement.requires_original_control(){return self.retirement.step_original_into(original,body,effect,reply)}
  self.close_step(body,effect,reply)
 }
 fn recover_close_original_step<'reply>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'reply,RetainedCloneStep,Retiring>)->Result<OriginalEffectReplyComplete<'reply,RetainedCloneStep,Retiring>,ValueError>{
  let g=effect.grant();
  if let Some(child)=self.child.as_mut(){if !child.terminal_is_empty()&&child.is_closing()&&child.requires_original_control(){return child.recover_close_original_step(original,body,&mut effect.nested(g),reply.progress())}}else if self.retirement.requires_original_control(){return self.retirement.recover_step_original_into(original,body,effect,reply)}
  self.recover_close_step(body,effect,reply)
 }
 fn terminal_is_empty(&self)->bool{!self.fault&&self.entries.is_none()&&self.child.is_none()&&self.output.is_none()&&self.index.is_none()&&self.retirement.is_empty()&&self.binding.is_none()}
}
impl<I:NativeValueSource+?Sized,V:NativeReceive<dyn NativeValueSource>+Sync,const N:usize> Drop for PagedMapReceiver<I,V,N>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native original map receiving requires funded closure")}}
