//! 🧵️ Resident paged-text producer retains its original Source, projection and output loans.
use super::*;
use semio_framework_value_native_encoding::{NativeEncodeProducerOutcome,NativeEncodeProducerFailure};

/// 🧭️ One frozen semantic operation of an original paged-text producer.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PagedTextEncodeOperation{Bind,Validate,Reserve,Copy,Finish,Publish}

/// 🏠️ The actual Source projection and destination remain in one producer across every paid action.
pub struct PagedTextEncodeProducer<'input,'output,S:TextReadSource+Sync+?Sized,const N:usize>{
 source:crate::OriginalSourceTurnIdentity,
 input:RetainedCloneRef<'input,S>,
 input_key:(u64,usize,usize,usize),
 projection:semio_framework_value_clone_source::RetainedOwnedProjectionAdmission<S>,
 projection_reply:Option<Result<RetainedCloneStep,ValueError>>,
 child_result:Option<Result<RetainedCloneProgress,ValueError>>,
 child_physical:RetainedCloneProgress,
 child_route:Option<PagedTextChildRoute>,
 metadata_copy:Option<usize>,
 owner:Option<PagedText<N>>,
 output:&'output mut Option<PagedText<N>>,
 inline:Option<InlineTextBuffer>,
 inline_output:Option<&'output mut Option<InlineTextBuffer>>,
 paged_active:bool,
 read:TextSourceReadFrame,
 phase:u8,
 position:usize,
 length:Option<usize>,
 maximum:usize,
 prefix:&'static str,
 suffix:&'static str,
 active:Option<PagedTextEncodeOperation>,
 published:bool,
 entered:bool,
 closing:bool,
 close_route:Option<(crate::OriginalEffectInvocationIdentity,u8,usize,Option<(crate::NativeAllocationIdentity,Option<u64>)>)>,
}
impl<'input,'output,S:TextReadSource+Sync+?Sized,const N:usize> PagedTextEncodeProducer<'input,'output,S,N>{
 /// 🪢️ Binds the actual original input, Source and exclusive output before any producer callback.
 pub fn new(source:crate::OriginalSourceTurnWitness<'_>,input:RetainedCloneRef<'input,S>,output:&'output mut Option<PagedText<N>>,maximum:usize,prefix:&'static str,suffix:&'static str)->Result<Self,ValueError>{
  if output.is_some(){return Err(ValueError::invariant("original text output is occupied"))}
  if maximum>N{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"text formatter exceeds its original capacity"))}
  Ok(Self{source:source.identity(),input,input_key:input.reference_identity(),projection:Default::default(),projection_reply:None,child_result:None,child_physical:Default::default(),child_route:None,metadata_copy:None,owner:Some(PagedText::empty()),output,inline:None,inline_output:None,paged_active:false,read:Default::default(),phase:0,position:0,length:None,maximum,prefix,suffix,active:None,published:false,entered:false,closing:false,close_route:None})
 }
 /// 🧩️ Binds the actual inline output as part of the same exclusive destination.
 pub fn new_inline(source:crate::OriginalSourceTurnWitness<'_>,input:RetainedCloneRef<'input,S>,output:&'output mut Option<PagedText<N>>,inline_output:&'output mut Option<InlineTextBuffer>,maximum:usize,prefix:&'static str,suffix:&'static str)->Result<Self,ValueError>{
  if inline_output.is_some(){return Err(ValueError::invariant("original inline text output is occupied"))}
  let mut producer=Self::new(source,input,output,maximum,prefix,suffix)?;producer.inline=Some(InlineTextBuffer::empty());producer.inline_output=Some(inline_output);Ok(producer)
 }
 /// 👁️ Observes the exact originally issued input identity without granting a replacement source.
 pub fn input_identity(&self)->(u64,usize,usize,usize){self.input_key}
 /// 🪪️ Observes the actual Source dispatch retained by this producer.
 pub fn source_identity(&self)->crate::OriginalSourceTurnIdentity{self.source}
 /// 💾️ Observes the actual page backing retained by this original producer.
 pub fn retained_capacity(&self)->usize{self.owner.as_ref().or(self.output.as_ref()).map_or(0,PagedText::allocated_bytes)}
 /// 📊️ Observes actual retained text bytes, including the original selected output.
 pub fn retained_len(&self)->usize{self.owner.as_ref().or(self.output.as_ref()).map_or(0,PagedText::byte_len)+self.inline.as_ref().or_else(||self.inline_output.as_ref().and_then(|slot|slot.as_ref())).map_or(0,|cell|cell.length)}
 /// 🧭️ Observes the actual selected semantic phase.
 pub fn stage(&self)->PagedTextStage{if self.closing{return PagedTextStage::Closing}if self.published{return PagedTextStage::Complete}if self.phase==3&&self.active.is_some(){return PagedTextStage::Settling}match self.phase{0=>PagedTextStage::Binding,1=>PagedTextStage::Validating,2=>PagedTextStage::Copying,_=>PagedTextStage::Publishing}}
 /// 🔢️ Observes the resident text phase without releasing its owner.
 pub fn phase_progress(&self)->(usize,Option<usize>){(self.position,self.length)}
 /// 🧭️ Selects the next action from the same actual source and destination state.
 pub fn operation(&self)->Result<PagedTextEncodeOperation,ValueError>{
  if self.closing{return Err(ValueError::invariant("original text producer is closing"))}if self.input.reference_identity()!=self.input_key{return Err(ValueError::invariant("original text input changed"))}
  if let Some(active)=self.active{return Ok(active)}
  if self.published{return Err(ValueError::invariant("original text output already published"))}
  if self.phase==0{return Ok(PagedTextEncodeOperation::Bind)}
  if self.phase==1{return Ok(PagedTextEncodeOperation::Validate)}
  if self.phase==2{let size=self.length.ok_or_else(||ValueError::invariant("original text extent is absent"))?;if self.position==size{return Ok(PagedTextEncodeOperation::Finish)}let owner=self.owner.as_ref().ok_or_else(||ValueError::invariant("original text owner is absent"))?;if !(self.inline.is_some()&&size<=128)&&!owner.bytes.has_reserved_slot(){return Ok(PagedTextEncodeOperation::Reserve)}return Ok(PagedTextEncodeOperation::Copy)}
  Ok(PagedTextEncodeOperation::Publish)
 }
 fn publication_copy(&self)->Result<usize,ValueError>{let paged=size_of::<Option<PagedText<N>>>();let inline=size_of::<Option<InlineTextBuffer>>();if self.inline.is_some()&&!self.paged_active{paged.checked_add(inline.checked_mul(2).ok_or_else(||ValueError::invariant("text publication extent overflow"))?).ok_or_else(||ValueError::invariant("text publication extent overflow"))}else{paged.checked_mul(2).and_then(|n|n.checked_add(if self.inline.is_some(){inline}else{0})).ok_or_else(||ValueError::invariant("text publication extent overflow"))}}
 /// 📏️ Quotes real original projection, page, byte or destination metadata.
 pub fn demand(&self)->Result<(crate::RetirementDemand,u64,Option<usize>),ValueError>{
  let operation=self.operation()?;let mut demand=crate::RetirementDemand{depth:1,..Default::default()};let mut work=1;let mut allocation=None;
  match operation{
   PagedTextEncodeOperation::Bind=>{demand=self.projection.demand(self.input)?;work=self.projection.work_demand(self.input)?}
   PagedTextEncodeOperation::Reserve=>{let owner=self.owner.as_ref().ok_or_else(||ValueError::invariant("original text owner is absent"))?;let limit=self.length.ok_or_else(||ValueError::invariant("original text extent is absent"))?;let bytes=owner.bytes.next_exact_capacity_allocation_bytes(limit).map_err(error)?.ok_or_else(||ValueError::invariant("original text page is absent"))?;demand.copy_bytes=owner.bytes.next_capacity_copy_byte_demand(limit).map_err(error)?;demand.depth=owner.bytes.next_capacity_depth_demand(limit).map_err(error)?;demand.capacity_bytes=bytes;work=owner.bytes.reserve_capacity_one_work_demand(limit).map_err(error)?;allocation=Some(bytes)}
   PagedTextEncodeOperation::Copy=>demand.copy_bytes=1,
   PagedTextEncodeOperation::Publish=>demand.copy_bytes=self.publication_copy()?,
   _=>{}
  }
  demand.copy_bytes=demand.copy_bytes.checked_add(self.transfer_copy(operation)?).ok_or_else(||ValueError::invariant("original text demand overflow"))?;work=work.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original text Work quote overflow"))?;Ok((demand,work,allocation))
 }
}

#[derive(Clone,Copy)]
enum PagedTextChildRoute{Byte(usize),Finish}
impl<S:TextReadSource+Sync+?Sized,const N:usize> PagedTextEncodeProducer<'_,'_,S,N>{
 fn transfer_copy(&self,selected:PagedTextEncodeOperation)->Result<usize,ValueError>{
  let child=match selected{
   PagedTextEncodeOperation::Bind=>size_of::<Option<Result<RetainedCloneStep,ValueError>>>(),
   PagedTextEncodeOperation::Validate if self.length==Some(self.position)=>size_of::<Option<Result<RetainedCloneProgress,ValueError>>>(),
   PagedTextEncodeOperation::Validate|PagedTextEncodeOperation::Copy=>size_of::<Option<TextRecordedByte>>(),
   PagedTextEncodeOperation::Finish=>size_of::<Option<TextRecordedFinish>>(),
   PagedTextEncodeOperation::Reserve|PagedTextEncodeOperation::Publish=>size_of::<Option<Result<RetainedCloneProgress,ValueError>>>(),
  };
  child.max(size_of::<Option<Result<RetainedCloneProgress,ValueError>>>()).checked_add(size_of::<RetainedCloneProgress>()).and_then(|n|n.checked_add(size_of::<Option<Result<NativeEncodeProducerOutcome,NativeEncodeProducerFailure>>>())).ok_or_else(||ValueError::invariant("original text reply extent overflow"))
 }
 fn actual_transfer_copy(&self,selected:PagedTextEncodeOperation)->Result<usize,ValueError>{let child=if self.child_result.is_some(){size_of::<Option<Result<RetainedCloneProgress,ValueError>>>()}else{match selected{PagedTextEncodeOperation::Bind=>size_of::<Option<Result<RetainedCloneStep,ValueError>>>(),PagedTextEncodeOperation::Validate|PagedTextEncodeOperation::Copy=>size_of::<Option<TextRecordedByte>>(),PagedTextEncodeOperation::Finish=>size_of::<Option<TextRecordedFinish>>(),_=>return Err(ValueError::invariant("original text child reply is absent"))}};child.checked_add(size_of::<RetainedCloneProgress>()).and_then(|n|n.checked_add(size_of::<Option<Result<NativeEncodeProducerOutcome,NativeEncodeProducerFailure>>>())).ok_or_else(||ValueError::invariant("original text reply extent overflow"))}
 fn start_child(&mut self,selected:PagedTextEncodeOperation,effect:&mut crate::RetainedAllocationEffect<'_>)->Result<(),ValueError>{
  let result=(||->Result<Option<RetainedCloneProgress>,ValueError>{match selected{
   PagedTextEncodeOperation::Bind=>{let reply=crate::OriginalEffectReply::borrow(&mut self.projection_reply)?;let done=self.projection.admit(self.input,effect,reply)?;drop(done);Ok(None)}
   PagedTextEncodeOperation::Validate|PagedTextEncodeOperation::Copy=>{
    let input=self.projection.borrow()?.get();let body=input.byte_len();let size=self.prefix.len().checked_add(body).and_then(|n|n.checked_add(self.suffix.len())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original formatter extent overflow"))?;
    if size>N||size>self.maximum{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"paged text exceeds its original capacity"))}
    if self.length.is_some_and(|prior|prior!=size){return Err(ValueError::invariant("original formatter extent changed"))}self.length=Some(size);self.paged_active=self.inline.is_some()&&size>128;
    if selected==PagedTextEncodeOperation::Validate&&self.position==size{if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"text validation exceeds original Work"))}if self.read.have!=0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"truncated paged UTF8"))}self.phase=2;self.position=0;return Ok(Some(RetainedCloneProgress{copied_items:1,..Default::default()}))}
    if self.read.started||self.read.reply.is_some(){return Err(ValueError::invariant("original byte already selected"))}self.read.started=true;self.read.index=Some(self.position);
    if self.position<self.prefix.len(){if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"text prefix exceeds original Work"))}self.read.reply=Some(TextRecordedByte{result:Ok(self.prefix.as_bytes().get(self.position).copied()),work:1})}
    else{let offset=self.position-self.prefix.len();if offset<body{self.child_route=Some(PagedTextChildRoute::Byte(offset));input.read_recorded(TextSourceByteReply{index:offset,slot:&mut self.read.reply,effect});}else{if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"text suffix exceeds original Work"))}self.read.reply=Some(TextRecordedByte{result:Ok(self.suffix.as_bytes().get(offset-body).copied()),work:1})}}
    Ok(None)
   }
   PagedTextEncodeOperation::Reserve=>{let length=self.length.ok_or_else(||ValueError::invariant("original text extent is absent"))?;let owner=self.owner.as_mut().ok_or_else(||ValueError::invariant("original text owner is absent"))?;let required=owner.bytes.next_exact_capacity_allocation_bytes(length).map_err(error)?.ok_or_else(||ValueError::invariant("original text page is absent"))?;{use crate::list::{PagedListReservationOutcome,PagedListReservationFailure};let turn=owner.bytes.reserve_exact_capacity_one(length,effect);match turn.result{Ok(PagedListReservationOutcome::Denied)=>Ok(Some(Default::default())),Ok(PagedListReservationOutcome::Ready(step))=>{let progress=RetainedCloneProgress{copied_items:usize::from(step.progressed),copied_bytes:step.placed_bytes,retained_capacity_bytes:step.allocated_bytes,released_bytes:step.released_allocation_bytes};if !step.progressed||step.allocated_bytes==required{Ok(Some(progress))}else{self.child_physical=progress;Err(ValueError::invariant("original text page differs from its admitted extent"))}},Err(PagedListReservationFailure::Work(error))=>Err(error),Err(PagedListReservationFailure::Allocation{error:failure,copied_bytes})=>{self.child_physical=RetainedCloneProgress{copied_items:usize::from(copied_bytes!=0||failure.allocated_bytes!=0),copied_bytes,retained_capacity_bytes:failure.allocated_bytes,..Default::default()};Err(error(failure.refusal()))}}}}
   PagedTextEncodeOperation::Finish=>{if self.read.have!=0{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original text ends mid-scalar"))}let input=self.projection.borrow()?.get();if self.prefix.len().checked_add(input.byte_len()).and_then(|n|n.checked_add(self.suffix.len()))!=self.length{return Err(ValueError::invariant("original formatter extent changed before completion"))}if self.read.finish_started||self.read.finish.is_some(){return Err(ValueError::invariant("original finish already selected"))}self.read.finish_started=true;self.child_route=Some(PagedTextChildRoute::Finish);input.finish_recorded(TextSourceFinishReply{slot:&mut self.read.finish,effect});Ok(None)}
   PagedTextEncodeOperation::Publish=>{if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"text publication exceeds original Work"))}let inline=self.inline.is_some()&&!self.paged_active;let copy=self.publication_copy()?;if inline{let slot=self.inline_output.as_mut().ok_or_else(||ValueError::invariant("original inline output is absent"))?;if slot.is_some(){return Err(ValueError::invariant("original inline output is occupied"))}if self.owner.as_ref().is_none_or(|owner|!owner.terminal_is_empty()){return Err(ValueError::invariant("original inline publication retains pages"))}let owner=self.owner.take().expect("original empty owner");drop(owner);**slot=self.inline.take()}else{if self.output.is_some(){return Err(ValueError::invariant("original text output is occupied"))}if self.inline.as_ref().is_some_and(|cell|!cell.terminal_is_empty()){return Err(ValueError::invariant("original text publication retains inline bytes"))}*self.output=self.owner.take();self.inline=None}self.published=true;Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))}
  }})();
  match result{Ok(Some(progress))=>self.child_result=Some(Ok(progress)),Ok(None)=>{},Err(error)=>{if self.read.reply.is_none(){self.read.started=false;self.read.index=None}if self.read.finish.is_none(){self.read.finish_started=false}self.child_route=None;self.child_result=Some(Err(error))}}Ok(())
 }
 fn recover_child(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->Result<(),ValueError>{
  match self.active.ok_or_else(||ValueError::invariant("original text action is absent"))?{
   PagedTextEncodeOperation::Bind if self.projection_reply.is_none()=>{let reply=crate::OriginalEffectReply::borrow(&mut self.projection_reply)?;let done=self.projection.recover_admit(reply)?;drop(done)}
   PagedTextEncodeOperation::Validate|PagedTextEncodeOperation::Copy if self.read.reply.is_none()&&self.child_result.is_none()=>{if let Some(PagedTextChildRoute::Byte(index))=self.child_route{let input=self.projection.borrow()?.get();let _=input.recover_recorded(TextSourceByteReply{index,slot:&mut self.read.reply,effect});}}
   PagedTextEncodeOperation::Finish if self.read.finish.is_none()&&self.child_result.is_none()=>{if let Some(PagedTextChildRoute::Finish)=self.child_route{let input=self.projection.borrow()?.get();let _=input.recover_finish_recorded(TextSourceFinishReply{slot:&mut self.read.finish,effect});}}
   _=>{}
  }Ok(())
 }
 fn child_ready(&self,selected:PagedTextEncodeOperation)->bool{self.child_result.is_some()||match selected{PagedTextEncodeOperation::Bind=>self.projection_reply.is_some(),PagedTextEncodeOperation::Validate|PagedTextEncodeOperation::Copy=>self.read.reply.is_some(),PagedTextEncodeOperation::Finish=>self.read.finish.is_some(),_=>false}}
 fn take_child(&mut self,selected:PagedTextEncodeOperation)->Result<RetainedCloneProgress,ValueError>{
  if let Some(result)=self.child_result.take(){return result}
  match selected{
   PagedTextEncodeOperation::Bind=>{let step=self.projection_reply.take().ok_or_else(||ValueError::invariant("original projection reply is absent"))??;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=1}Ok(step.progress())}
   PagedTextEncodeOperation::Validate|PagedTextEncodeOperation::Copy=>{let result=self.read.reply.take().ok_or_else(||ValueError::invariant("original byte reply is absent"))?.result;self.read.started=false;self.read.index=None;self.child_route=None;let byte=result?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"original text byte is absent"))?;self.read.accept_utf8(byte)?;if selected==PagedTextEncodeOperation::Copy{let size=self.length.ok_or_else(||ValueError::invariant("original text extent is absent"))?;if self.inline.is_some()&&size<=128{let inline=self.inline.as_mut().expect("original inline owner retained");inline.bytes[inline.length]=byte;inline.length+=1}else{self.owner.as_mut().ok_or_else(||ValueError::invariant("original text owner is absent"))?.bytes.push_reserved(byte).map_err(|_|ValueError::invariant("original text page rejected its funded byte"))?}}self.position+=1;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:usize::from(selected==PagedTextEncodeOperation::Copy),..Default::default()})}
   PagedTextEncodeOperation::Finish=>{let result=self.read.finish.take().ok_or_else(||ValueError::invariant("original finish reply is absent"))?.result;self.read.finish_started=false;self.child_route=None;result?;if self.inline.is_some()&&!self.paged_active{self.inline.as_mut().expect("original inline owner retained").complete=true}else{self.owner.as_mut().ok_or_else(||ValueError::invariant("original text owner is absent"))?.complete=true}self.phase=3;Ok(RetainedCloneProgress{copied_items:1,..Default::default()})}
   _=>Err(ValueError::invariant("original text child reply is absent"))
  }
 }
 fn run<'r>(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,NativeEncodeProducerOutcome,NativeEncodeProducerFailure>,recovering:bool)->Result<crate::OriginalEffectReplyComplete<'r,NativeEncodeProducerOutcome,NativeEncodeProducerFailure>,ValueError>{
  self.entered=true;
  if !recovering{if self.active.is_some(){return Err(ValueError::invariant("original text action already started"))}let selected=self.operation()?;self.metadata_copy=Some(self.transfer_copy(selected)?);self.active=Some(selected);self.start_child(selected,effect)?;return Ok(reply.pending())}
  let selected=self.active.ok_or_else(||ValueError::invariant("original text action is absent"))?;if !self.child_ready(selected){self.recover_child(effect)?;if !self.child_ready(selected){return Ok(reply.pending())}}
  let quoted=self.metadata_copy.ok_or_else(||ValueError::invariant("original text reply quote is absent"))?;let copy=self.actual_transfer_copy(selected)?;if copy>quoted{return Err(ValueError::invariant("original text reply exceeds its quote"))}let demand=self.resident_reply_demand()?;if !crate::retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.pending())}if effect.remaining_work_units()?==0{return Ok(reply.pending())}if !effect.performed(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"text child publication exceeded original Work"))}
  let result=self.take_child(selected);let child_physical=self.child_physical;self.child_physical=Default::default();self.active=None;self.metadata_copy=None;
  Ok(reply.complete(match result{Ok(mut progress)=>{progress.copied_items=progress.copied_items.max(1);progress.copied_bytes=progress.copied_bytes.checked_add(copy).expect("preflight original text reply extent");Ok(NativeEncodeProducerOutcome{performed:true,progress})},Err(cause)=>{let mut progress=child_physical;progress.copied_items=progress.copied_items.max(1);progress.copied_bytes=progress.copied_bytes.checked_add(copy).expect("preflight original text refusal extent");Err(NativeEncodeProducerFailure{performed:true,progress,cause})}}))
 }
}
impl<S:TextReadSource+Sync+?Sized,const N:usize> crate::OriginalEffectProducer for PagedTextEncodeProducer<'_,'_,S,N>{
 type Output=NativeEncodeProducerOutcome;
 type Failure=NativeEncodeProducerFailure;
 type Context<'turn,'source>=crate::OriginalSourceTurnWitness<'turn> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{if context.identity()!=self.source||self.input.reference_identity()!=self.input_key{return Err(ValueError::invariant("original text Source or projection changed"))}Ok(())}
 fn source_turn<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{Some(context.identity())}
 fn receive<'r,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<crate::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'source:'turn{self.run(effect,reply,false)}
 fn recover<'r,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<crate::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'source:'turn{self.run(effect,reply,true)}
}

/// 🪢️ Exclusive loan of the same caller-resident paged producer through every action.
pub struct PagedTextProducerLoan<'owner,'input,'output,S:TextReadSource+Sync+?Sized,const N:usize>{producer:&'owner mut PagedTextEncodeProducer<'input,'output,S,N>}
impl<'owner,'input,'output,S:TextReadSource+Sync+?Sized,const N:usize> PagedTextProducerLoan<'owner,'input,'output,S,N>{
 /// 🪢️ Borrows the same actual resident producer and its original Source.
 pub fn new(source:crate::OriginalSourceTurnWitness<'_>,producer:&'owner mut PagedTextEncodeProducer<'input,'output,S,N>)->Result<Self,ValueError>{if producer.source!=source.identity()||producer.active.is_some()||producer.closing{return Err(ValueError::invariant("original text producer cannot be loaned"))}Ok(Self{producer})}
 /// 👁️ Observes the same original producer without lending mutable custody.
 pub fn producer(&self)->&PagedTextEncodeProducer<'input,'output,S,N>{self.producer}
}
impl<S:TextReadSource+Sync+?Sized,const N:usize> crate::OriginalEffectProducer for PagedTextProducerLoan<'_,'_,'_,S,N>{
 type Output=NativeEncodeProducerOutcome;type Failure=NativeEncodeProducerFailure;
 type Context<'turn,'source>=crate::OriginalSourceTurnWitness<'turn> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Result<(),ValueError>where 'source:'turn{crate::OriginalEffectProducer::validate_context(self.producer,context)}
 fn source_turn<'turn,'source>(&self,context:&Self::Context<'turn,'source>)->Option<crate::OriginalSourceTurnIdentity>where 'source:'turn{crate::OriginalEffectProducer::source_turn(self.producer,context)}
 fn receive<'r,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<crate::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'source:'turn{crate::OriginalEffectProducer::receive(self.producer,context,effect,reply)}
 fn recover<'r,'turn,'source>(&mut self,context:&mut Self::Context<'turn,'source>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,Self::Output,Self::Failure>)->Result<crate::OriginalEffectReplyComplete<'r,Self::Output,Self::Failure>,ValueError>where 'source:'turn{crate::OriginalEffectProducer::recover(self.producer,context,effect,reply)}
}
/// 🧵️ One actual encoder frame owns its Source producer throughout all paid text actions.
pub struct PagedTextEncodeFrame<'owner,'input,'output,S:TextReadSource+Sync+?Sized,const N:usize>{
 frame:crate::native_encoding::NativeEncodeOwnedProducerFrame<PagedTextProducerLoan<'owner,'input,'output,S,N>>,
 request:Option<(PagedTextEncodeOperation,RetainedCloneGrant,RetainedCloneGrant,u64,Option<usize>)>,
 rearming:bool,
}
impl<'owner,'input,'output,S:TextReadSource+Sync+?Sized,const N:usize> PagedTextEncodeFrame<'owner,'input,'output,S,N>{
 /// 🌱️ Exclusively borrows the caller's actual Source-bound producer before any quote.
 pub fn new(control:&mut crate::NativeEncodeControl<'_>,source:&crate::OriginalSourceTurn,producer:&'owner mut PagedTextEncodeProducer<'input,'output,S,N>)->Result<Self,ValueError>{
  if producer.source!=source.identity(){return Err(ValueError::invariant("original text producer Source changed"))}
  let loan=PagedTextProducerLoan::new(source.witness(),producer)?;
  Ok(Self{frame:crate::native_encoding::NativeEncodeOwnedProducerFrame::new(control,source.witness(),loan)?,request:None,rearming:false})
 }
 /// 👁️ Observes the original producer without lending mutable Source or output authority.
 pub fn producer(&self)->Option<&PagedTextEncodeProducer<'input,'output,S,N>>{self.frame.producer().map(|loan|&*loan.producer)}
 /// 📏️ Quotes the actual selected projection, page or metadata action and real Work ceiling.
 /// 🧭️ Reports the original selected operation until its action settles.
 pub fn operation(&self)->Result<PagedTextEncodeOperation,ValueError>{if let Some((operation,_,_,_,_))=self.request{return Ok(operation)}self.producer().ok_or_else(||ValueError::invariant("original text producer returned"))?.operation()}
 pub fn demand(&self)->Result<(crate::RetirementDemand,u64,Option<usize>),ValueError>{if let Some((_,_,selected,work,bytes))=self.request{return Ok((crate::RetirementDemand{copy_bytes:selected.maximum_copy_bytes,capacity_bytes:selected.maximum_capacity_bytes,release_bytes:selected.maximum_release_bytes,depth:selected.maximum_depth},work,bytes))}self.producer().ok_or_else(||ValueError::invariant("original text producer returned"))?.demand()}
 /// 🧵️ Advances or recovers the same original semantic action under its frozen quote.
 pub fn advance_body(&mut self,control:&mut crate::NativeEncodeControl<'_>,source:&crate::OriginalSourceTurn,grant:RetainedCloneGrant)->Result<crate::native_encoding::NativeEncodeSourcePreparation,ValueError>{
  if self.rearming{return Err(ValueError::invariant("original text rearm has not settled"))}if self.request.is_none(){let (demand,work,bytes)=self.demand()?;let selected=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};self.request=Some((self.operation()?,grant,selected,work,bytes))}
  let (_,original,selected,work,bytes)=self.request.expect("original text request retained");if grant!=original{return Err(ValueError::invariant("original text grant changed"))}
  let mut context=source.witness();let status=self.frame.advance_body(control,source.witness(),&mut context,grant,selected,work,bytes)?;if matches!(status,crate::native_encoding::NativeEncodeSourcePreparation::Denied){self.request=None}Ok(status)
 }
 /// 🧾️ Settles the actual original semantic Work reply before producer reentry.
 pub fn acknowledge_body_partial_success(&mut self)->Result<Option<u128>,ValueError>{self.frame.acknowledge_body_partial_success()}
 /// 🧮️ Claims only newly invoiced original semantic Work.
 pub fn claim_body_work(&mut self)->Result<Option<u128>,ValueError>{self.frame.claim_body_work()}
 /// 📤️ Pays and settles the original recorded producer reply before observation.
 pub fn advance_reply(&mut self,control:&mut crate::NativeEncodeControl<'_>,source:&crate::OriginalSourceTurn,grant:RetainedCloneGrant)->Result<crate::native_encoding::NativeEncodeProducerReplyStep,ValueError>{let mut context=source.witness();self.frame.advance_reply(control,source.witness(),&mut context,grant)}
 /// 📐️ Quotes the genuine original producer reply header.
 pub fn reply_demand(&self)->Result<crate::RetirementDemand,ValueError>{self.frame.reply_demand()}
 /// 🧾️ Settles the actual paid reply header Work without substituting a verdict.
 pub fn acknowledge_reply_partial_success(&mut self)->Result<Option<u128>,ValueError>{self.frame.acknowledge_reply_partial_success()}
 /// 🧮️ Claims only newly invoiced reply header Work.
 pub fn claim_reply_work(&mut self)->Result<Option<u128>,ValueError>{self.frame.claim_reply_work()}
 /// 📭️ Borrows the actual result only after its physical Post settled.
 pub fn reply(&self)->Option<&Result<NativeEncodeProducerOutcome,NativeEncodeProducerFailure>>{self.frame.reply()}
 /// 🧳️ Borrows the independent original settlement refusal, if any.
 pub fn failure(&self)->Option<&crate::native_encoding::NativeEncodeEffectFailure>{self.frame.failure()}
 /// 📏️ Quotes separately funded publication of the same completed resident child.
 pub fn resident_reply_recovery_demand(&self)->Result<(crate::RetirementDemand,u64),ValueError>{self.frame.resident_reply_recovery_demand()}
 /// 🧵️ Recovers the exact original child reply under the same Source and a real new payment.
 pub fn advance_resident_reply_recovery(&mut self,control:&mut crate::NativeEncodeControl<'_>,source:&crate::OriginalSourceTurn,grant:RetainedCloneGrant)->Result<crate::native_encoding::NativeEncodeProducerReplyStep,ValueError>{if self.producer().is_none_or(|producer|producer.source!=source.identity()){return Err(ValueError::invariant("original text recovery Source changed"))}let mut context=source.witness();self.frame.advance_resident_reply_recovery(control,source.witness(),&mut context,grant)}
 /// 🧾️ Borrows an independently recorded recovery payment refusal.
 pub fn resident_recovery_failure(&self)->Option<&crate::native_encoding::NativeEncodeEffectFailure>{self.frame.resident_recovery_failure()}
 /// ✅️ Acknowledges only the actual successful recovery Work partial.
 pub fn acknowledge_resident_recovery_partial_success(&mut self)->Result<Option<u128>,ValueError>{self.frame.acknowledge_resident_recovery_partial_success()}
 /// 🧮️ Claims only newly invoiced resident-recovery Work.
 pub fn claim_resident_recovery_work(&mut self)->Result<Option<u128>,ValueError>{self.frame.claim_resident_recovery_work()}
 /// 🧷️ Borrows the same original body while settling its real physical receipt.
 pub fn begin_body_receipt_payout<'a>(&'a mut self,control:&mut crate::NativeEncodeControl<'_>,source:&crate::OriginalSourceTurn)->Result<crate::native_encoding::NativeEncodeReceiptPayout<'a>,ValueError>{self.frame.begin_body_receipt_payout(control,source.witness())}
 /// 📐️ Quotes separately paid rearm of this same resident producer.
 pub fn rearm_demand(&self)->Result<crate::RetirementDemand,ValueError>{self.frame.rearm_success_demand()}
 /// 🔁️ Drives the real same-Source rearm payment and hides its owner until Post settles.
 pub fn advance_rearm(&mut self,control:&mut crate::NativeEncodeControl<'_>,source:&crate::OriginalSourceTurn,grant:RetainedCloneGrant)->Result<crate::native_encoding::NativeEncodeProducerReplyStep,ValueError>{
  if self.producer().is_none_or(|producer|producer.source!=source.identity()){return Err(ValueError::invariant("original text rearm Source changed"))}self.rearming=true;let step=self.frame.advance_rearm(control,source.witness(),grant)?;if matches!(step,crate::native_encoding::NativeEncodeProducerReplyStep::Transferred(_)){self.request=None;self.rearming=false}Ok(step)
 }
 /// 🧾️ Borrows the actual pending rearm Source partial.
 pub fn rearm_partial_outcome(&self)->Option<&crate::OriginalWorkPartialOutcome>{self.frame.rearm_partial_outcome()}
 /// ✅️ Acknowledges only the actual successful rearm Work partial.
 pub fn acknowledge_rearm_partial_success(&mut self)->Result<Option<u128>,ValueError>{self.frame.acknowledge_rearm_partial_success()}
 /// 🧮️ Claims only newly invoiced rearm Work.
 pub fn claim_rearm_work(&mut self)->Result<Option<u128>,ValueError>{self.frame.claim_rearm_work()}
 /// 🧳️ Borrows any separately retained rearm refusal.
 pub fn rearm_failure(&self)->Option<&crate::native_encoding::NativeEncodeEffectFailure>{self.frame.rearm_failure()}

}
impl<S:TextReadSource+Sync+?Sized,const N:usize> PagedTextEncodeProducer<'_,'_,S,N>{
 /// 🧷️ Stops new semantic work while retaining every original child for paid closure.
 pub fn begin_close(&mut self){self.closing=true}
 /// 📏️ Quotes the exact remaining projection, UTF8 state or owned page.
 pub fn close_demand(&self,body:usize)->Result<crate::RetirementDemand,ValueError>{
  if !self.closing||self.active.is_some()||self.projection_reply.is_some()||self.child_result.is_some()||self.child_physical!=Default::default()||self.child_route.is_some()||self.metadata_copy.is_some()||self.read.started||self.read.reply.is_some()||self.read.finish_started||self.read.finish.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original text producer still has a selected action"))}
  if !self.projection.terminal_is_empty(){return self.projection.bound_close_demand(self.input,body)}
  if self.read.have!=0{return Ok(crate::RetirementDemand{copy_bytes:text_read_tail_copy(&self.read),depth:1,..Default::default()})}
  text_frame_tail_demand(&self.owner,&self.inline,body)
 }
 /// ⛽️ Quotes the genuine next producer closure Work.
 pub fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{self.close_demand(body)?;if !self.projection.terminal_is_empty(){return self.projection.bound_close_work_demand(self.input,body)}if self.read.have!=0{return Ok(1)}if let Some(owner)=self.owner.as_ref(){if !owner.terminal_is_empty(){return owner.next_computational_work(body)}}Ok(u64::from(self.owner.is_some()||self.inline.is_some()))}
 /// 🌱️ Reports the actual next child birth before opening a normal close effect.
 pub fn requires_original_birth(&self)->bool{self.close_route.is_none()&&!self.projection.terminal_is_empty()&&self.projection.requires_original_birth()}
 /// 🪢️ Reports whether the retained projection needs its actual original close authority.
 pub fn requires_original_control(&self)->bool{!self.projection.terminal_is_empty()&&self.projection.requires_original_control()}
 /// 👶️ Advances only the retained projection birth under its original close loan.
 pub fn close_original_birth_step(&mut self,original:&mut semio_framework_value_retirement::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementPoll{self.projection.close_original_birth_step(original,grant)}
 /// 👁️ Reports only actual returned child owners and a settled selected close route.
 pub fn terminal_is_empty(&self)->bool{self.active.is_none()&&self.projection_reply.is_none()&&self.child_result.is_none()&&self.child_physical==Default::default()&&self.child_route.is_none()&&self.metadata_copy.is_none()&&self.projection.terminal_is_empty()&&!self.read.started&&self.read.reply.is_none()&&!self.read.finish_started&&self.read.finish.is_none()&&self.read.have==0&&self.owner.is_none()&&self.inline.is_none()&&self.close_route.is_none()}
 fn close_selected<'r>(&mut self,original:Option<&mut semio_framework_value_retirement::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<crate::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{
  let invocation=effect.invocation().ok_or_else(||ValueError::invariant("original text close lacks its producer invocation"))?;let mode=if recover{crate::OriginalEffectCallbackMode::Recover}else{crate::OriginalEffectCallbackMode::Receive};if invocation.mode()!=mode{return Err(ValueError::invariant("original text close changed invocation mode"))}
  let identity=invocation.identity();let original_route=original.as_ref().map(|loan|(loan.ledger_identity(),loan.retirement_recipient_identity()));let route=if recover{let (key,route,selected_body,was_original)=self.close_route.ok_or_else(||ValueError::invariant("original text close has no pending route"))?;if key!=identity||selected_body!=body||was_original!=original_route{return Err(ValueError::invariant("original text close changed its route"))}route}else{self.close_demand(body)?;if self.close_route.is_some(){return Err(ValueError::invariant("original text close already pending"))}if !self.projection.terminal_is_empty(){1}else if self.read.have!=0{2}else if self.owner.as_ref().is_some_and(|owner|!owner.terminal_is_empty()){3}else{4}};
  if route==2{return close_text_read_tail(&mut self.read,effect,reply.progress())}if route==4{return close_text_frame_tail(&mut self.owner,&mut self.inline,effect,reply)}
  let mut scope=reply.progress().scope(&mut self.close_route,(identity,route,body,original_route))?;
  let done=if route==1{match (original,recover){(Some(original),false)=>self.projection.close_original_into(original,body,effect,scope.borrow()?)?,(Some(original),true)=>self.projection.recover_close_original_into(original,body,effect,scope.borrow()?)?,(None,false)=>self.projection.close_bound_into(self.input,body,effect,scope.borrow()?)?,(None,true)=>self.projection.recover_close_bound_into(self.input,body,effect,scope.borrow()?)?}}else{let owner=self.owner.as_mut().ok_or_else(||ValueError::invariant("original text page owner is absent"))?;match (original,recover){(Some(original),false)=>ErasedSnapshotRetirement::close_original_into(owner,original,body,effect,scope.borrow()?)?,(Some(original),true)=>ErasedSnapshotRetirement::recover_close_original_into(owner,original,body,effect,scope.borrow()?)?,(None,false)=>ErasedSnapshotRetirement::close_into(owner,body,effect,scope.borrow()?)?,(None,true)=>ErasedSnapshotRetirement::recover_close_into(owner,body,effect,scope.borrow()?)?}};drop(done);Ok(scope.forwarded())
 }
 /// 📥️ Lends the actual ordinary close reply into the retained owner.
 pub fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_selected(None,body,effect,reply,false)}
 /// 🔁️ Recovers the same ordinary child close after interruption.
 pub fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_selected(None,body,effect,reply,true)}
 /// 📥️ Lends the actual original-control close reply into the retained owner.
 pub fn close_original_into<'r>(&mut self,original:&mut semio_framework_value_retirement::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_selected(Some(original),body,effect,reply,false)}
 /// 🔁️ Recovers the same original-control close after interruption.
 pub fn recover_close_original_into<'r>(&mut self,original:&mut semio_framework_value_retirement::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:crate::OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>)->Result<crate::OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{self.close_selected(Some(original),body,effect,reply,true)}
}
impl<S:TextReadSource+Sync+?Sized,const N:usize> Drop for PagedTextEncodeProducer<'_,'_,S,N>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty()||(!self.entered&&self.projection.terminal_is_empty()&&self.owner.as_ref().is_some_and(PagedText::terminal_is_empty)&&self.inline.as_ref().is_none_or(InlineTextBuffer::terminal_is_empty)),"original text producer requires paid child closure")}}

/// 🔐️ Keeps the exclusive original producer loan hidden until each actual return Post settles.
pub struct PagedTextLoanReturn<'frame,'owner,'input,'output,S:TextReadSource+Sync+?Sized,const N:usize>{
 frame:&'frame mut PagedTextEncodeFrame<'owner,'input,'output,S,N>,
 source:crate::OriginalSourceTurnIdentity,
 binding:Option<crate::OriginalWorkBindingIdentity>,
 issuer:Option<crate::OriginalConstructionIssuerIdentity>,
 returned:Option<PagedTextProducerLoan<'owner,'input,'output,S,N>>,
 pending:Option<PagedTextLoanReturnPending>,
 moved:bool,
 released:bool,
 done:bool,
}
struct PagedTextLoanReturnPending{binding:crate::OriginalWorkBindingIdentity,physical:crate::OriginalConstructionActionIdentity,work:crate::OriginalWorkActionIdentity,progress:RetainedCloneProgress,physical_finished:bool}
impl<'frame,'owner,'input,'output,S:TextReadSource+Sync+?Sized,const N:usize> PagedTextLoanReturn<'frame,'owner,'input,'output,S,N>{
 /// 🪢️ Retains the one actual frame and its Source until paid return completes.
 pub fn new(frame:&'frame mut PagedTextEncodeFrame<'owner,'input,'output,S,N>,source:&crate::OriginalSourceTurn)->Result<Self,ValueError>{if frame.rearming||frame.producer().is_none_or(|producer|producer.source!=source.identity()){return Err(ValueError::invariant("original text loan return changed Source or action"))}Ok(Self{frame,source:source.identity(),binding:None,issuer:None,returned:None,pending:None,moved:false,released:false,done:false})}
 /// ⏸️ Cancels only before any original loan or payout action has moved.
 pub fn cancel_unstarted(&mut self)->bool{if self.moved||self.pending.is_some(){return false}self.done=true;true}
 /// 📏️ Quotes only the actual loan, reply and selected request metadata being moved.
 pub fn transfer_demand(&self)->Result<crate::RetirementDemand,ValueError>{if self.moved||self.pending.is_some(){return Err(ValueError::invariant("original text loan return already started"))}let mut demand=self.frame.frame.owner_transfer_demand()?;if self.frame.request.is_some(){demand.copy_bytes=demand.copy_bytes.checked_add(size_of_val(&self.frame.request)).ok_or_else(||ValueError::invariant("original text loan return extent overflow"))?}Ok(demand)}
 fn validates(&self,port:&crate::OriginalConstructionLoan<'_>,payout:&crate::OriginalConstructionAction,effect:&crate::RetainedAllocationEffect<'_>,demand:crate::RetirementDemand)->Result<crate::OriginalWorkBindingIdentity,ValueError>{let binding=crate::OriginalWorkBindingIdentity::from_effect(effect);let witness=port.witness(payout)?;if port.source_turn()!=Some(self.source)||binding.source_turn()!=Some(self.source)||self.binding.is_some_and(|original|original!=binding)||self.issuer.is_some_and(|original|original!=port.identity())||witness.work_binding()!=binding||witness.phase()!=crate::OriginalConstructionPhase::Granted||witness.demand()!=demand||witness.grant()!=Some(effect.grant()){return Err(ValueError::invariant("original text loan return authority changed"))}Ok(binding)}
 /// 📥️ Moves the actual loan into this private slot under its original paid action.
 pub fn stage_transfer(&mut self,source:&crate::OriginalSourceTurn,port:&crate::OriginalConstructionLoan<'_>,payout:&crate::OriginalConstructionAction,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
  let fail=|error|crate::RetainedWorkOutcome{result:Err(error),work:0};if source.identity()!=self.source||effect.actual().is_some(){return fail(ValueError::invariant("original text loan Source changed"))}let demand=match self.transfer_demand(){Ok(demand)=>demand,Err(error)=>return fail(error)};let binding=match self.validates(port,payout,effect,demand){Ok(binding)=>binding,Err(error)=>return fail(error)};if !crate::retained_clone_grant_admits(effect.grant(),demand){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}
  let work=effect.work_identity().action_identity();let mut turn=self.frame.frame.transfer_owner_into(&source.witness(),&mut self.returned,effect);if let Ok(Some(progress))=&mut turn.result{progress.copied_bytes=demand.copy_bytes;self.frame.request=None;self.binding=Some(binding);self.issuer=Some(port.identity());self.pending=Some(PagedTextLoanReturnPending{binding,physical:payout.identity(),work,progress:*progress,physical_finished:false});self.moved=true}turn
 }
 /// 📏️ Quotes the actual small exclusive-loan slot rather than the caller-owned producer.
 pub fn release_demand(&self)->Result<crate::RetirementDemand,ValueError>{if !self.moved||self.released||self.pending.is_some()||self.returned.is_none(){return Err(ValueError::invariant("original text loan is not ready for release"))}Ok(crate::RetirementDemand{copy_bytes:size_of_val(&self.returned),depth:1,..Default::default()})}
 /// 🧹️ Clears the private loan only under its separately paid original Source action.
 pub fn stage_release(&mut self,port:&crate::OriginalConstructionLoan<'_>,payout:&crate::OriginalConstructionAction,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
  let fail=|error|crate::RetainedWorkOutcome{result:Err(error),work:0};if effect.actual().is_some(){return fail(ValueError::invariant("original text loan release invoked an allocator"))}let demand=match self.release_demand(){Ok(demand)=>demand,Err(error)=>return fail(error)};let binding=match self.validates(port,payout,effect,demand){Ok(binding)=>binding,Err(error)=>return fail(error)};if !crate::retained_clone_grant_admits(effect.grant(),demand){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}match effect.remaining_work_units(){Ok(0)=>return crate::RetainedWorkOutcome{result:Ok(None),work:0},Ok(_)=>{},Err(error)=>return fail(error)}match effect.performed(1){Ok(true)=>{},Ok(false)=>return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"text loan release exceeded original Work")),work:1},Err(error)=>return fail(error)}let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};drop(self.returned.take());self.released=true;self.pending=Some(PagedTextLoanReturnPending{binding,physical:payout.identity(),work:effect.work_identity().action_identity(),progress,physical_finished:false});crate::RetainedWorkOutcome{result:Ok(Some(progress)),work:1}
 }
 /// 📏️ Quotes a metadata-only retry of the already performed return or release.
 pub fn retry_demand()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:size_of::<PagedTextLoanReturnPending>(),depth:1,..Default::default()}}
 /// 🔁️ Retries only after preserving the exact separately published original Post refusal.
 pub fn retry(&mut self,port:&crate::OriginalConstructionLoan<'_>,refusal:&crate::OriginalConstructionCompletion,payout:&crate::OriginalConstructionAction,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
  let fail=|error|crate::RetainedWorkOutcome{result:Err(error),work:0};let Some(prior)=self.pending.as_ref()else{return fail(ValueError::invariant("original text loan return is absent"))};if prior.physical_finished||!prior.binding.matches_effect(effect)||refusal.action_identity()!=Some(prior.physical)||refusal.status()!=Some(crate::OriginalConstructionAcceptStatus::Refused)||refusal.work_binding()!=Some(prior.binding){return fail(ValueError::invariant("original text loan refusal changed"))}match port.validates_completion(refusal){Ok(true)=>{},Ok(false)=>return fail(ValueError::invariant("original text loan issuer changed")),Err(error)=>return fail(error)}
  let demand=Self::retry_demand();if let Err(error)=self.validates(port,payout,effect,demand){return fail(error)}if effect.actual().is_some(){return fail(ValueError::invariant("original text loan retry invoked an allocator"))}match effect.remaining_work_units(){Ok(0)=>return crate::RetainedWorkOutcome{result:Ok(None),work:0},Ok(_)=>{},Err(error)=>return fail(error)}if !crate::retained_clone_grant_admits(effect.grant(),demand){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"text loan retry exceeded original Work")),work:1},Err(error)=>return fail(error)}let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};let prior=self.pending.as_mut().expect("original return retained");prior.physical=payout.identity();prior.work=effect.work_identity().action_identity();prior.progress=progress;crate::RetainedWorkOutcome{result:Ok(Some(progress)),work:1}
 }
 /// ✅️ Finishes the exact physical action and its original Work proof before exposing a next phase.
 pub fn finish(&mut self,control:&mut crate::NativeEncodeControl<'_>,source:&crate::OriginalSourceTurn,payout:&mut Option<crate::OriginalConstructionAction>,proof:&mut Option<crate::OriginalWorkActionProof>,work_action:&mut Option<crate::OriginalWorkActionIdentity>)->Result<bool,ValueError>{
  if source.identity()!=self.source{return Err(ValueError::invariant("original text loan return Source changed"))}let pending=self.pending.as_mut().ok_or_else(||ValueError::invariant("original text loan return is absent"))?;
  let ready=control.with_original_effect_work_and_construction(|work,port|{if work.binding_identity()?!=pending.binding||port.source_turn()!=Some(self.source)||self.issuer!=Some(port.identity()){return Err(ValueError::invariant("original text loan return authorities changed"))}let actual=proof.as_ref().ok_or_else(||ValueError::invariant("original text loan return Work absent"))?;if actual.action_identity()!=pending.work||*work_action!=Some(pending.work)||actual.actual_work()==0||!work.validates(actual,actual.actual_work())?{return Err(ValueError::invariant("original text loan return Work changed"))}if !pending.physical_finished{let action=payout.as_ref().ok_or_else(||ValueError::invariant("original text loan Post absent"))?;if action.identity()!=pending.physical||!port.validates_action(action)?||action.work_binding()!=pending.binding||action.progress()!=Some(pending.progress)||action.actual().is_some(){return Err(ValueError::invariant("original text loan Post changed"))}match action.recorded_accept_status(){None|Some(crate::OriginalConstructionAcceptStatus::Refused)=>return Ok(false),Some(crate::OriginalConstructionAcceptStatus::Accepted)=>{}}port.finish_accepted(payout)?;pending.physical_finished=true}work.finish(proof,work_action)?;Ok(true)})?;
  if ready{self.pending=None;if self.released{self.done=true}}Ok(ready)
 }
}
impl<S:TextReadSource+Sync+?Sized,const N:usize> Drop for PagedTextLoanReturn<'_,'_,'_,'_,S,N>{fn drop(&mut self){assert!(std::thread::panicking()||self.done,"original text producer loan requires paid return")}}

enum PagedTextPayoutSlot<'slot>{Reply(&'slot mut Option<Result<NativeEncodeProducerOutcome,NativeEncodeProducerFailure>>),BodyFailure{slot:crate::native_encoding::NativeEncodeEffectFailureSlot,output:&'slot mut Option<ValueError>}}
/// 🔐️ Holds the exact published child result and caller destination through its paid Post.
pub struct PagedTextReplyPayout<'frame,'slot,'owner,'input,'output,S:TextReadSource+Sync+?Sized,const N:usize>{
 frame:&'frame mut PagedTextEncodeFrame<'owner,'input,'output,S,N>,
 output:PagedTextPayoutSlot<'slot>,
 source:crate::OriginalSourceTurnIdentity,
 issuer:Option<crate::OriginalConstructionIssuerIdentity>,
 pending:Option<PagedTextLoanReturnPending>,
 done:bool,
}
impl<'frame,'slot,'owner,'input,'output,S:TextReadSource+Sync+?Sized,const N:usize> PagedTextReplyPayout<'frame,'slot,'owner,'input,'output,S,N>{
 /// 🪢️ Exclusively binds the original Source frame and vacant actual reply slot.
 pub fn new(frame:&'frame mut PagedTextEncodeFrame<'owner,'input,'output,S,N>,source:&crate::OriginalSourceTurn,output:&'slot mut Option<Result<NativeEncodeProducerOutcome,NativeEncodeProducerFailure>>)->Result<Self,ValueError>{if output.is_some()||frame.rearming||frame.producer().is_none_or(|producer|producer.source!=source.identity()){return Err(ValueError::invariant("original text reply payout changed"))}Ok(Self{frame,output:PagedTextPayoutSlot::Reply(output),source:source.identity(),issuer:None,pending:None,done:false})}
 /// 🪢️ Exclusively binds one independent original body refusal and its vacant destination.
 pub fn new_body_failure(frame:&'frame mut PagedTextEncodeFrame<'owner,'input,'output,S,N>,source:&crate::OriginalSourceTurn,slot:crate::native_encoding::NativeEncodeEffectFailureSlot,output:&'slot mut Option<ValueError>)->Result<Self,ValueError>{if output.is_some()||frame.rearming||frame.producer().is_none_or(|producer|producer.source!=source.identity()){return Err(ValueError::invariant("original text body payout changed"))}Ok(Self{frame,output:PagedTextPayoutSlot::BodyFailure{slot,output},source:source.identity(),issuer:None,pending:None,done:false})}
 /// ⏸️ Cancels only while the actual result and destination remain untouched.
 pub fn cancel_unstarted(&mut self)->bool{if self.pending.is_some()||match &self.output{PagedTextPayoutSlot::Reply(output)=>output.is_some(),PagedTextPayoutSlot::BodyFailure{output,..}=>output.is_some()}{return false}self.done=true;true}
 /// 📏️ Quotes the actual complete typed original result and destination slots.
 pub fn demand(&self)->Result<crate::RetirementDemand,ValueError>{if self.pending.is_some()||self.done{return Err(ValueError::invariant("original text reply payout already started"))}match &self.output{PagedTextPayoutSlot::Reply(_)=>self.frame.frame.published_reply_transfer_demand(),PagedTextPayoutSlot::BodyFailure{..}=>self.frame.frame.body_value_failure_transfer_demand()}}
 /// 📥️ Moves the actual original result under its authenticated Source payment.
 pub fn stage(&mut self,port:&crate::OriginalConstructionLoan<'_>,payout:&crate::OriginalConstructionAction,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
  let fail=|error|crate::RetainedWorkOutcome{result:Err(error),work:0};if effect.actual().is_some(){return fail(ValueError::invariant("original text reply payout invoked an allocator"))}let demand=match self.demand(){Ok(demand)=>demand,Err(error)=>return fail(error)};let binding=crate::OriginalWorkBindingIdentity::from_effect(effect);let witness=match port.witness(payout){Ok(witness)=>witness,Err(error)=>return fail(error)};
  if port.source_turn()!=Some(self.source)||binding.source_turn()!=Some(self.source)||witness.work_binding()!=binding||witness.phase()!=crate::OriginalConstructionPhase::Granted||witness.demand()!=demand||witness.grant()!=Some(effect.grant()){return fail(ValueError::invariant("original text reply payout authority changed"))}
  let work=effect.work_identity().action_identity();let turn=match &mut self.output{PagedTextPayoutSlot::Reply(output)=>self.frame.frame.transfer_published_reply_into(output,effect),PagedTextPayoutSlot::BodyFailure{slot,output}=>self.frame.frame.transfer_body_value_failure_into(*slot,output,effect)};if let Ok(Some(progress))=&turn.result{self.issuer=Some(port.identity());self.pending=Some(PagedTextLoanReturnPending{binding,physical:payout.identity(),work,progress:*progress,physical_finished:false})}turn
 }
 /// 📏️ Quotes retry metadata without moving or replacing the retained original result.
 pub fn retry_demand()->crate::RetirementDemand{crate::RetirementDemand{copy_bytes:size_of::<PagedTextLoanReturnPending>(),depth:1,..Default::default()}}
 /// 🔁️ Retries the same payout after its exact independently retained physical refusal.
 pub fn retry(&mut self,port:&crate::OriginalConstructionLoan<'_>,refusal:&crate::OriginalConstructionCompletion,payout:&crate::OriginalConstructionAction,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<Option<RetainedCloneProgress>,ValueError>{
  let fail=|error|crate::RetainedWorkOutcome{result:Err(error),work:0};let Some(prior)=self.pending.as_ref()else{return fail(ValueError::invariant("original text reply payout absent"))};if prior.physical_finished||self.issuer!=Some(port.identity())||!prior.binding.matches_effect(effect)||refusal.action_identity()!=Some(prior.physical)||refusal.status()!=Some(crate::OriginalConstructionAcceptStatus::Refused)||refusal.work_binding()!=Some(prior.binding){return fail(ValueError::invariant("original text reply refusal changed"))}match port.validates_completion(refusal){Ok(true)=>{},Ok(false)=>return fail(ValueError::invariant("original text reply issuer changed")),Err(error)=>return fail(error)}
  let demand=Self::retry_demand();let witness=match port.witness(payout){Ok(witness)=>witness,Err(error)=>return fail(error)};if port.source_turn()!=Some(self.source)||witness.phase()!=crate::OriginalConstructionPhase::Granted||witness.demand()!=demand||witness.grant()!=Some(effect.grant())||witness.work_binding()!=prior.binding||effect.actual().is_some(){return fail(ValueError::invariant("original text reply retry changed"))}match effect.remaining_work_units(){Ok(0)=>return crate::RetainedWorkOutcome{result:Ok(None),work:0},Ok(_)=>{},Err(error)=>return fail(error)}if !crate::retained_clone_grant_admits(effect.grant(),demand){return crate::RetainedWorkOutcome{result:Ok(None),work:0}}match effect.performed(1){Ok(true)=>{},Ok(false)=>return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"text reply retry exceeded original Work")),work:1},Err(error)=>return fail(error)}let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};let prior=self.pending.as_mut().expect("original reply payout retained");prior.physical=payout.identity();prior.work=effect.work_identity().action_identity();prior.progress=progress;crate::RetainedWorkOutcome{result:Ok(Some(progress)),work:1}
 }
 /// ✅️ Returns the exact original Post and Work proof before releasing reply custody.
 pub fn finish(&mut self,control:&mut crate::NativeEncodeControl<'_>,source:&crate::OriginalSourceTurn,payout:&mut Option<crate::OriginalConstructionAction>,proof:&mut Option<crate::OriginalWorkActionProof>,work_action:&mut Option<crate::OriginalWorkActionIdentity>)->Result<bool,ValueError>{
  if self.done||source.identity()!=self.source{return Err(ValueError::invariant("original text reply payout Source changed"))}let pending=self.pending.as_mut().ok_or_else(||ValueError::invariant("original text reply payout absent"))?;let ready=control.with_original_effect_work_and_construction(|work,port|{
   if work.binding_identity()?!=pending.binding||port.source_turn()!=Some(self.source)||self.issuer!=Some(port.identity()){return Err(ValueError::invariant("original text reply payout authority changed"))}let actual=proof.as_ref().ok_or_else(||ValueError::invariant("original text reply payout Work absent"))?;if actual.action_identity()!=pending.work||*work_action!=Some(pending.work)||actual.actual_work()==0||!work.validates(actual,actual.actual_work())?{return Err(ValueError::invariant("original text reply payout Work changed"))}
   if !pending.physical_finished{let action=payout.as_ref().ok_or_else(||ValueError::invariant("original text reply Post absent"))?;if action.identity()!=pending.physical||!port.validates_action(action)?||action.work_binding()!=pending.binding||action.progress()!=Some(pending.progress)||action.actual().is_some(){return Err(ValueError::invariant("original text reply Post changed"))}match action.recorded_accept_status(){None|Some(crate::OriginalConstructionAcceptStatus::Refused)=>return Ok(false),Some(crate::OriginalConstructionAcceptStatus::Accepted)=>{}}port.finish_accepted(payout)?;pending.physical_finished=true}work.finish(proof,work_action)?;Ok(true)
  })?;if ready{self.pending=None;self.done=true}Ok(ready)
 }
}
impl<S:TextReadSource+Sync+?Sized,const N:usize> Drop for PagedTextReplyPayout<'_,'_,'_,'_,'_,S,N>{fn drop(&mut self){assert!(std::thread::panicking()||self.done,"original text reply requires its paid payout")}}
impl<S:TextReadSource+Sync+?Sized,const N:usize> PagedTextEncodeProducer<'_,'_,S,N>{
 /// 👁️ Observes only the original child result that is already resident.
 pub fn resident_reply_facts(&self)->Option<NativeEncodeProducerOutcome>{
  let selected=self.active?;
  let (performed,progress)=if let Some(result)=self.child_result.as_ref(){(true,match result{Ok(progress)=>*progress,Err(_)=>self.child_physical})}
   else{match selected{
    PagedTextEncodeOperation::Bind=>{let result=self.projection_reply.as_ref()?;(true,match result{Ok(step)=>step.progress(),Err(_)=>Default::default()})}
    PagedTextEncodeOperation::Validate|PagedTextEncodeOperation::Copy=>{let result=self.read.reply.as_ref()?;(result.work!=0,Default::default())}
    PagedTextEncodeOperation::Finish=>{let result=self.read.finish.as_ref()?;(result.work!=0,Default::default())}
    _=>return None
   }};
  Some(NativeEncodeProducerOutcome{performed,progress})
 }
 /// 📏️ Quotes only moving the retained child into its original native reply.
 pub fn resident_reply_demand(&self)->Result<crate::RetirementDemand,ValueError>{
  let selected=self.active.ok_or_else(||ValueError::invariant("original text child is absent"))?;
  if self.resident_reply_facts().is_none(){return Err(ValueError::invariant("original text child reply is not recorded"))}
  let mut copy=self.actual_transfer_copy(selected)?;
  if selected==PagedTextEncodeOperation::Copy&&self.child_result.is_none()&&self.read.reply.as_ref().is_some_and(|reply|matches!(reply.result,Ok(Some(_)))){copy=copy.checked_add(1).ok_or_else(||ValueError::invariant("original text reply extent overflow"))?}
  Ok(crate::RetirementDemand{copy_bytes:copy,depth:1,..Default::default()})
 }
 /// ⛽️ Quotes the one actual native child-result publication step.
 pub fn resident_reply_work_demand(&self)->Result<u64,ValueError>{self.resident_reply_demand()?;Ok(1)}
}

impl<S:TextReadSource+Sync+?Sized,const N:usize> crate::native_encoding::NativeEncodeResidentReplyProducer for PagedTextEncodeProducer<'_,'_,S,N>{
 fn resident_reply_facts(&self)->Option<NativeEncodeProducerOutcome>{PagedTextEncodeProducer::resident_reply_facts(self)}
 fn resident_reply_demand(&self)->Result<crate::RetirementDemand,ValueError>{PagedTextEncodeProducer::resident_reply_demand(self)}
 fn resident_reply_work_demand(&self)->Result<u64,ValueError>{PagedTextEncodeProducer::resident_reply_work_demand(self)}
}
impl<S:TextReadSource+Sync+?Sized,const N:usize> crate::native_encoding::NativeEncodeResidentReplyProducer for PagedTextProducerLoan<'_,'_,'_,S,N>{
 fn resident_reply_facts(&self)->Option<NativeEncodeProducerOutcome>{self.producer.resident_reply_facts()}
 fn resident_reply_demand(&self)->Result<crate::RetirementDemand,ValueError>{self.producer.resident_reply_demand()}
 fn resident_reply_work_demand(&self)->Result<u64,ValueError>{self.producer.resident_reply_work_demand()}
}
