//! 🛫️ The actual Source admits one exact byte backing and every write span before a caller-owned receiver changes.
use semio_framework_job_context::{OriginalCallerSourceLoan,OriginalSourceBinding,OriginalSourceTurnIdentity,OriginalTurnFailure,RetainedCloneGrant,RetainedCloneProgress,RetirementDemand,StepContext};
use semio_framework_job_receiving::retained_delta;
use semio_framework_value::{RetainedWorkOutcome,ValueError,ValueRefusalKind};
use semio_framework_value::native_encoding::{NativeEncodeControl,NativeEncodeEffectFailure,NativeEncodeEffectState,NativeEncodeRetirementRecipient,OriginalCheckpointRequest,OriginalRecordedAuthority,OriginalRecordedProducer,OriginalRecordedReply,OriginalRecordedReplyComplete,OriginalSpanRequest};
use semio_framework_value::ErasedSnapshotRetirement;
use semio_framework_value_control_contract::{NativeAllocationAuthority,NativeAllocationAuthorityPort,NativeAllocationBudget,NativeAllocationEvent,NativeAllocationRefusal,NativeAllocationReply,NativeAllocationReplyComplete,OriginalConstructionAccept,OriginalConstructionAcceptComplete,OriginalConstructionAuthority,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionRecipient,OriginalWorkAuthority,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkRecipient,RetainedAllocationEffect};
use std::mem::ManuallyDrop;
use std::sync::{Mutex,MutexGuard};
/// ⚠️ Owns independent Source effect refusals retained across an error or unwind.
#[derive(semio_framework_value::RetireOwned)]
#[value(crate="::semio_framework_value",send_only)]
pub struct SourceEncodingRetainedFailure{pub receipt:Option<ValueError>,pub provenance:Option<ValueError>}
/// 🧳️ The caller retains the original Source recipient and its outstanding allocation reservation.
pub struct OriginalSourceEncodingOwner{retirement:NativeEncodeRetirementRecipient,allocation:EncodingCustody<NativeAllocationAuthority<EncodingAllocationProducer>>,span:EncodingCustody<OriginalRecordedAuthority<EncodingSpanProducer>>,checkpoint:EncodingCustody<OriginalRecordedAuthority<EncodingCheckpointProducer>>,work:EncodingCustody<OriginalWorkAuthority<EncodingWorkProducer>>,construction:EncodingCustody<OriginalConstructionAuthority<EncodingConstructionProducer>>,binding:Option<OriginalSourceBinding>,failure:ManuallyDrop<Option<SourceEncodingRetainedFailure>>}
impl OriginalSourceEncodingOwner{
 /// 🪹️ Creates the original Source encoding owner before any allocation is admitted.
 pub fn new()->Self{Self{retirement:NativeEncodeRetirementRecipient::new(),allocation:EncodingCustody::new(),span:EncodingCustody::new(),checkpoint:EncodingCustody::new(),work:EncodingCustody::new(),construction:EncodingCustody::new(),binding:None,failure:ManuallyDrop::new(None)}}
 /// 🎟️ Admits one durable authority per turn against the actual original caller before any callback loan.
 pub fn admit_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  cx.original_source_parent()?.validate_borrowed_context(cx)?;let identity=OriginalSourceBinding::borrow(cx);if self.binding.is_some_and(|binding|binding!=identity){return Err(encoding_refusal("Source encoding owner changed its original caller").into())}
  let admitted=self.allocation.admit(cx)?;self.binding=Some(identity);
  Ok(admitted&&self.span.admit(cx)?&&self.checkpoint.admit(cx)?&&self.work.admit(cx)?&&self.construction.admit(cx)?)
 }
 /// ♻️ Returns the durable authorities only after all original child custody has ended.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.custody_is_empty(){return Ok(true)}cx.original_source_parent()?.validate_borrowed_context(cx)?;if !self.binding.is_some_and(|binding|binding.matches(cx)){return Err(encoding_refusal("Source issuer close changed its original caller").into())}if self.allocation_pending()||self.failure.is_some()||!self.retirement.terminal_is_empty(){return Err(encoding_refusal("Source issuer retains original child custody").into())}let grant=cx.retained_grant();if cx.retirement_should_yield()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}
  Ok(self.construction.close(cx)?&&self.work.close(cx)?&&self.checkpoint.close(cx)?&&self.span.close(cx)?&&self.allocation.close(cx)?)
 }
 /// 🪪️ Identifies the original physical retirement recipient.
 pub fn retirement_identity(&self)->u64{self.retirement.identity()}
 /// ♻️ Lends the same recipient for its explicit paid retirement.
 pub fn retirement_recipient(&mut self)->Result<&mut NativeEncodeRetirementRecipient,ValueError>{if self.allocation_pending(){return Err(encoding_refusal("Source allocation is still pending"))}Ok(&mut self.retirement)}
 /// 👁️ Reports whether effect refusals remain in this original owner.
 pub fn has_retained_failure(&self)->bool{self.failure.is_some()}
 /// 🪫️ Reports whether original allocation and retirement custody have both ended.
 pub fn terminal_is_empty(&self)->bool{!self.allocation_pending()&&self.custody_is_empty()&&self.retirement.terminal_is_empty()&&self.failure.is_none()}
 /// 📬️ Transfers retained effect refusals only under the original caller's paid metadata grant.
 pub fn take_retained_failure(&mut self,cx:&mut StepContext<'_>,output:&mut Option<SourceEncodingRetainedFailure>)->Result<bool,OriginalTurnFailure>{
  if output.is_some(){return Err(encoding_refusal("Source failure destination is occupied").into())}if self.failure.is_none(){return Ok(false)}cx.original_source_parent()?.validate_borrowed_context(cx)?;if !self.binding.is_some_and(|binding|binding.matches(cx)){return Err(encoding_refusal("Source failure transfer changed its original caller").into())}if cx.retirement_should_yield()||cx.fuel_remaining()==0||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}
  *output=self.failure.take();charge(cx,RetainedCloneProgress{copied_items:1,..Default::default()}).map(|()|true).map_err(Into::into)
 }
 fn custody_is_empty(&self)->bool{self.allocation.is_empty()&&self.span.is_empty()&&self.checkpoint.is_empty()&&self.work.is_empty()&&self.construction.is_empty()}
 fn admitted(&self)->bool{self.allocation.admitted()&&self.span.admitted()&&self.checkpoint.admitted()&&self.work.admitted()&&self.construction.admitted()}
 fn allocation_pending(&self)->bool{self.allocation.authority.as_ref().is_some_and(|authority|!authority.quiet())}
}
impl Drop for OriginalSourceEncodingOwner{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Source encoding owner retains unsettled original custody")}}
fn charge(cx:&mut StepContext<'_>,progress:RetainedCloneProgress)->Result<(),ValueError>{let receipt=cx.consume_retained(progress);cx.consume_fuel(1);receipt}
/// 🔐️ One durable original authority and the Source loan it returns to.
trait EncodingAuthority:Sized{
 fn forge(source:OriginalCallerSourceLoan)->Self;
 fn validate(&self,cx:&StepContext<'_>)->Result<(),ValueError>;
 fn release(self)->Result<OriginalCallerSourceLoan,Self>;
 fn quiet(&self)->bool{true}
}
impl EncodingAuthority for NativeAllocationAuthority<EncodingAllocationProducer>{
 fn forge(source:OriginalCallerSourceLoan)->Self{Self::new(EncodingAllocationProducer{source,budget:Default::default()})}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),ValueError>{self.inspect_port(|producer|producer.source.validate_context(cx))}
 fn release(self)->Result<OriginalCallerSourceLoan,Self>{self.into_port().map(|producer|producer.source)}
 fn quiet(&self)->bool{!self.has_pending_transaction()&&self.inspect_port(|producer|producer.budget.terminal_is_empty())}
}
impl EncodingAuthority for OriginalRecordedAuthority<EncodingSpanProducer>{
 fn forge(source:OriginalCallerSourceLoan)->Self{Self::new(EncodingSpanProducer{source})}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),ValueError>{self.inspect_port(|producer|producer.source.validate_context(cx))}
 fn release(self)->Result<OriginalCallerSourceLoan,Self>{self.into_port().map(|producer|producer.source)}
 fn quiet(&self)->bool{!self.has_pending_action()}
}
impl EncodingAuthority for OriginalRecordedAuthority<EncodingCheckpointProducer>{
 fn forge(source:OriginalCallerSourceLoan)->Self{Self::new(EncodingCheckpointProducer{source})}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),ValueError>{self.inspect_port(|producer|producer.source.validate_context(cx))}
 fn release(self)->Result<OriginalCallerSourceLoan,Self>{self.into_port().map(|producer|producer.source)}
 fn quiet(&self)->bool{!self.has_pending_action()}
}
impl EncodingAuthority for OriginalWorkAuthority<EncodingWorkProducer>{
 fn forge(source:OriginalCallerSourceLoan)->Self{Self::new(EncodingWorkProducer{source})}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),ValueError>{self.inspect(|producer|producer.source.validate_context(cx))}
 fn release(self)->Result<OriginalCallerSourceLoan,Self>{self.into_recipient().map(|producer|producer.source)}
 fn quiet(&self)->bool{!self.has_live_actions()}
}
impl EncodingAuthority for OriginalConstructionAuthority<EncodingConstructionProducer>{
 fn forge(source:OriginalCallerSourceLoan)->Self{Self::new(EncodingConstructionProducer{source,submitted:Default::default()})}
 fn validate(&self,cx:&StepContext<'_>)->Result<(),ValueError>{self.inspect(|producer|producer.source.validate_context(cx))}
 fn release(self)->Result<OriginalCallerSourceLoan,Self>{self.into_recipient().map(|producer|producer.source)}
 fn quiet(&self)->bool{!self.has_live_actions()}
}
/// 🧷️ Holds one authority, or the Source loan awaiting its authority, across turns.
struct EncodingCustody<A>{source:ManuallyDrop<Option<OriginalCallerSourceLoan>>,authority:ManuallyDrop<Option<A>>,accepted:bool}
impl<A:EncodingAuthority> EncodingCustody<A>{
 fn new()->Self{Self{source:ManuallyDrop::new(None),authority:ManuallyDrop::new(None),accepted:false}}
 fn is_empty(&self)->bool{self.source.is_none()&&self.authority.is_none()}
 fn admitted(&self)->bool{self.authority.is_some()&&self.accepted}
 fn admit(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.is_empty(){cx.original_source_parent()?.admit_loan(&mut self.source,cx)?;return Ok(false)}
  if self.authority.is_none(){let bytes=size_of::<A>();let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}let source=self.source.take().ok_or_else(||encoding_refusal("Source encoding loan is absent"))?;*self.authority=Some(A::forge(source));charge(cx,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;self.accepted=true;return Ok(false)}
  if !self.accepted{return Err(encoding_refusal("Source encoding authority admission was refused").into())}
  self.authority.as_ref().ok_or_else(||encoding_refusal("Source encoding authority is absent"))?.validate(cx)?;Ok(true)
 }
 fn close(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if let Some(authority)=self.authority.take(){let accepted=std::mem::take(&mut self.accepted);let released=if authority.quiet(){authority.release()}else{Err(authority)};match released{Ok(source)=>*self.source=Some(source),Err(authority)=>{*self.authority=Some(authority);self.accepted=accepted;return Err(encoding_refusal("Source encoding authority retains its original action").into())}}charge(cx,RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(false)}
  if let Some(source)=self.source.as_mut(){let parent=cx.original_source_parent()?;source.close_step(parent,cx)?;if source.terminal_is_empty(){self.source.take();}return Ok(false)}
  Ok(true)
 }
}
/// 🧰️ Lends typed Source effects to the original native and physical recipients.
pub struct OriginalSourceEncoding<'owner,'control>{native:&'owner mut NativeEncodeControl<'control>,grant:RetainedCloneGrant,local:&'owner mut RetainedCloneProgress}
impl OriginalSourceEncoding<'_,'_>{
 /// 🧺️ Quotes the fresh byte-vector header and its exact backing before birth.
 pub fn allocate_bytes(&mut self,count:usize,output:&mut Option<Vec<u8>>,marker:&mut NativeEncodeEffectState,unwind:&mut Option<NativeEncodeEffectFailure>)->RetainedWorkOutcome<Option<()>,NativeEncodeEffectFailure>{
  if output.is_some(){return RetainedWorkOutcome{work:0,result:Err(NativeEncodeEffectFailure{primary:Some(encoding_refusal("Source byte destination is occupied")),..Default::default()})}}let layout=match std::alloc::Layout::array::<u8>(count){Ok(layout)=>layout,Err(_)=>return RetainedWorkOutcome{work:0,result:Err(NativeEncodeEffectFailure{primary:Some(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Source byte layout exceeds the native limit")),..Default::default()})}};let bytes=layout.size();let header=size_of::<Vec<u8>>();let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:header,maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()};
  self.native.effect_allocated(self.grant,self.local,demand,(bytes!=0).then_some(bytes),&mut marker.view(),unwind,&mut |physical,effect|{performed_once(effect)?;if bytes==0{*output=Some(Vec::new());*physical=RetainedCloneProgress{copied_items:1,copied_bytes:header,..Default::default()};return Ok(())}let pointer=unsafe{std::alloc::alloc(layout)};if pointer.is_null(){*effect.actual()=Some(0);return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"Source byte backing allocation failed"))}*output=Some(unsafe{Vec::from_raw_parts(pointer,0,count)});*effect.actual()=Some(bytes);*physical=RetainedCloneProgress{copied_items:1,copied_bytes:header,retained_capacity_bytes:bytes,..Default::default()};Ok(())})
 }
 /// ✍️ Quotes one bounded byte write and retains its output on every refusal.
 pub fn append_bytes(&mut self,output:&mut Vec<u8>,input:&[u8],consumed:&mut usize,marker:&mut NativeEncodeEffectState,unwind:&mut Option<NativeEncodeEffectFailure>)->RetainedWorkOutcome<Option<()>,NativeEncodeEffectFailure>{
  if *consumed!=0{return RetainedWorkOutcome{work:0,result:Err(NativeEncodeEffectFailure{primary:Some(encoding_refusal("Source byte progress destination is occupied")),..Default::default()})}}if input.is_empty(){return RetainedWorkOutcome{work:0,result:Ok(Some(()))}}let span=&input[..input.len().min(65536)];let demand=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:span.len(),maximum_depth:1,..Default::default()};self.native.effect_allocated(self.grant,self.local,demand,None,&mut marker.view(),unwind,&mut |physical,effect|{performed_once(effect)?;let Some(end)=output.len().checked_add(span.len())else{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Source byte extent overflow"))};if end>output.capacity(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Source byte output has insufficient admitted backing"))}output.extend_from_slice(span);*consumed=span.len();*physical=RetainedCloneProgress{copied_items:1,copied_bytes:span.len(),..Default::default()};Ok(())})
 }
}
fn performed_once(effect:&mut RetainedAllocationEffect<'_>)->Result<(),ValueError>{if effect.performed(1)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Source byte effect exceeds its work ceiling"))}}
/// 🧾️ Physical backing and write extent observed by the recorded producers of one encoding turn.
#[derive(Default)]
struct EncodingHop{planned:Option<usize>,observed:u128,permits:usize}
/// 🧵️ Every recorded Source producer reads the actual original caller through this one view.
trait EncodingTurnContext:Sync{
 fn identity(&self)->Option<OriginalSourceTurnIdentity>;
 fn validate(&self,source:&OriginalCallerSourceLoan)->Result<(),ValueError>;
 fn live(&self)->bool;
 fn admit_work(&self,observed:u64,additional:u64)->bool;
 fn allocate(&self,budget:&mut NativeAllocationBudget,event:NativeAllocationEvent)->Result<(),ValueError>;
 fn span(&self,request:OriginalSpanRequest)->Result<(),ValueError>;
 fn grant(&self,demand:RetirementDemand)->Result<RetainedCloneGrant,ValueError>;
 fn accept(&self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>;
}
fn locked<T>(mutex:&Mutex<T>)->MutexGuard<'_,T>{mutex.lock().unwrap_or_else(|poison|poison.into_inner())}
struct EncodingTurn<'view,'context,'source>{source:&'view Mutex<&'context mut StepContext<'source>>,binding:OriginalSourceBinding,hop:&'view Mutex<EncodingHop>}
impl<'view,'context,'source> EncodingTurn<'view,'context,'source>{
 fn cx(&self)->MutexGuard<'view,&'context mut StepContext<'source>>{locked(self.source)}
 fn hop(&self)->MutexGuard<'view,EncodingHop>{locked(self.hop)}
}
impl EncodingTurnContext for EncodingTurn<'_,'_,'_>{
 fn identity(&self)->Option<OriginalSourceTurnIdentity>{self.cx().original_source_turn_identity()}
 fn validate(&self,source:&OriginalCallerSourceLoan)->Result<(),ValueError>{source.validate_context(&self.cx())}
 fn live(&self)->bool{let source=self.cx();self.binding.matches(&source)&&!source.should_yield()&&!source.is_cancelled()}
 fn admit_work(&self,observed:u64,additional:u64)->bool{let source=self.cx();!source.should_yield()&&!source.is_cancelled()&&observed.checked_add(additional).is_some_and(|work|work<=source.fuel_remaining())}
 fn allocate(&self,budget:&mut NativeAllocationBudget,event:NativeAllocationEvent)->Result<(),ValueError>{
  let grant=self.cx().retained_grant();let live=self.live();
  if let NativeAllocationEvent::Quote(request)=event{if self.hop().planned.is_some()||request.owned_bytes!=0||grant.maximum_items==0{return Err(encoding_refusal("Source byte encoding requires one exact admitted backing"))}}
  let verdict=budget.receive(event,grant,live);
  if let NativeAllocationEvent::Post{actual,..}=event{let mut hop=self.hop();let total=hop.observed.checked_add(actual as u128).ok_or_else(||encoding_refusal("Source allocation observation overflow"))?;hop.observed=total;hop.planned=usize::try_from(total).ok();verdict?;if !live{return Err(ValueError::literal(ValueRefusalKind::Canceled,"Source allocation post was refused"))}return Ok(())}
  verdict
 }
 fn span(&self,request:OriginalSpanRequest)->Result<(),ValueError>{
  if !self.live(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"Source byte span no longer has its admitting work"))}if request.owned_bytes==0{return Ok(())}
  let maximum=self.cx().retained_grant().maximum_copy_bytes;let mut hop=self.hop();if hop.planned!=Some(request.owned_bytes){return Err(encoding_refusal("Source byte span changed its admitted backing"))}let copied=hop.permits.checked_add(request.units).ok_or_else(||encoding_refusal("Source byte span extent overflow"))?;if copied>maximum{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Source byte span exceeds remaining copy currency"))}hop.permits=copied;Ok(())
 }
 fn grant(&self,demand:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{let source=self.cx();if source.should_yield()||source.is_cancelled(){return Err(encoding_refusal("Source construction lost its original caller"))}let grant=source.retained_grant();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Source construction exceeds original physical grant"))}Ok(grant)}
 fn accept(&self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{let mut source=self.cx();let admitted=progress.fits(grant);source.consume_retained(progress)?;if !admitted{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Source construction exceeded its quoted physical grant").with_retained_progress(progress))}Ok(())}
}
struct EncodingAllocationProducer{source:OriginalCallerSourceLoan,budget:NativeAllocationBudget}
impl NativeAllocationAuthorityPort for EncodingAllocationProducer{
 type Context<'turn,'source>=&'turn(dyn EncodingTurnContext+'turn) where 'source:'turn;
 fn validate_context<'turn,'source>(&self,turn:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn{turn.validate(&self.source)}
 fn source_turn<'turn,'source>(&self,turn:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity> where 'source:'turn{turn.identity()}
 fn receive<'reply,'turn,'source>(&mut self,turn:&mut Self::Context<'turn,'source>,event:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{let result=turn.validate(&self.source).and_then(|()|turn.allocate(&mut self.budget,event)).map_err(NativeAllocationRefusal::from);Ok(reply.complete(result))}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,_:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{Ok(reply.pending())}
}
struct EncodingSpanProducer{source:OriginalCallerSourceLoan}
impl OriginalRecordedProducer for EncodingSpanProducer{
 type Request=OriginalSpanRequest;type Reply=();
 type Context<'turn,'source>=&'turn(dyn EncodingTurnContext+'turn) where 'source:'turn;
 fn validate_context<'turn,'source>(&self,turn:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn{turn.validate(&self.source)}
 fn source_turn<'turn,'source>(&self,turn:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity> where 'source:'turn{turn.identity()}
 fn receive<'reply,'turn,'source>(&mut self,turn:&mut Self::Context<'turn,'source>,request:Self::Request,reply:OriginalRecordedReply<'reply,()>)->Result<OriginalRecordedReplyComplete<'reply,()>,ValueError> where 'source:'turn{let result=turn.validate(&self.source).and_then(|()|turn.span(request));Ok(reply.complete(result))}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,_:Self::Request,reply:OriginalRecordedReply<'reply,()>)->Result<OriginalRecordedReplyComplete<'reply,()>,ValueError> where 'source:'turn{Ok(reply.pending())}
}
struct EncodingCheckpointProducer{source:OriginalCallerSourceLoan}
impl OriginalRecordedProducer for EncodingCheckpointProducer{
 type Request=OriginalCheckpointRequest;type Reply=bool;
 type Context<'turn,'source>=&'turn(dyn EncodingTurnContext+'turn) where 'source:'turn;
 fn validate_context<'turn,'source>(&self,turn:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn{turn.validate(&self.source)}
 fn source_turn<'turn,'source>(&self,turn:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity> where 'source:'turn{turn.identity()}
 fn receive<'reply,'turn,'source>(&mut self,turn:&mut Self::Context<'turn,'source>,_:Self::Request,reply:OriginalRecordedReply<'reply,bool>)->Result<OriginalRecordedReplyComplete<'reply,bool>,ValueError> where 'source:'turn{let result=turn.validate(&self.source).map(|()|turn.live());Ok(reply.complete(result))}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,_:Self::Request,reply:OriginalRecordedReply<'reply,bool>)->Result<OriginalRecordedReplyComplete<'reply,bool>,ValueError> where 'source:'turn{Ok(reply.pending())}
}
struct EncodingWorkProducer{source:OriginalCallerSourceLoan}
impl OriginalWorkRecipient for EncodingWorkProducer{
 type Context<'turn,'source>=&'turn(dyn EncodingTurnContext+'turn) where 'source:'turn;
 fn validate_context<'turn,'source>(&self,turn:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn{turn.validate(&self.source)}
 fn source_turn<'turn,'source>(&self,turn:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity> where 'source:'turn{turn.identity()}
 fn admits<'reply,'turn,'source>(&mut self,turn:&mut Self::Context<'turn,'source>,quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply> where 'source:'turn{let result=turn.validate(&self.source).map(|()|turn.admit_work(observed,additional));quote.complete(result)}
 fn settle_partial<'reply,'turn,'source>(&mut self,turn:&mut Self::Context<'turn,'source>,check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(turn.validate(&self.source))}
 fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.pending()}
}
struct EncodingConstructionProducer{source:OriginalCallerSourceLoan,submitted:RetainedCloneProgress}
impl OriginalConstructionRecipient for EncodingConstructionProducer{
 type Context<'turn,'source>=&'turn(dyn EncodingTurnContext+'turn) where 'source:'turn;
 fn validate_context<'turn,'source>(&self,turn:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn{turn.validate(&self.source)}
 fn source_turn<'turn,'source>(&self,turn:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity> where 'source:'turn{turn.identity()}
 fn submitted<'turn,'source>(&self,_:&Self::Context<'turn,'source>)->RetainedCloneProgress where 'source:'turn{self.submitted}
 fn grant<'reply,'turn,'source>(&mut self,turn:&mut Self::Context<'turn,'source>,quote:OriginalConstructionGrant<'reply>)->OriginalConstructionGrantComplete<'reply> where 'source:'turn{let result=turn.grant(quote.demand());quote.complete(result)}
 fn accept<'reply,'turn,'source>(&mut self,turn:&mut Self::Context<'turn,'source>,receipt:OriginalConstructionAccept<'reply>)->OriginalConstructionAcceptComplete<'reply> where 'source:'turn{let progress=receipt.progress();let result=(||{let submitted=self.submitted.checked_add(progress)?;let result=turn.accept(receipt.grant(),progress);self.submitted=submitted;result})();receipt.complete(result)}
}
fn settle_encoding(cx:&mut StepContext<'_>,output:&Option<Vec<u8>>,hop:&Mutex<EncodingHop>,start:RetainedCloneProgress)->(RetainedCloneProgress,Option<ValueError>,Option<ValueError>){let hop=locked(hop);let capacity=output.as_ref().map_or(0,Vec::capacity);let copied=output.as_ref().map_or(0,Vec::len);let provenance=if output.is_some()&&((capacity!=0&&hop.planned!=Some(capacity))||(capacity==0&&hop.planned.is_some())||copied>hop.permits){Some(encoding_refusal("Source byte receiver differs from its admitted physical backing or writes"))}else if output.is_none()&&hop.permits!=0{Some(encoding_refusal("Source byte receiver lost its admitted partial backing"))}else{None};let receipt=charge(cx,RetainedCloneProgress{copied_items:1,copied_bytes:copied,retained_capacity_bytes:capacity,..Default::default()});(retained_delta(cx.retained_progress(),start),receipt.err(),provenance)}
/// 🧵️ Borrows source-authenticated native encoding from the actual job context.
pub trait OriginalSourceEncodingContext {
 /// 🎟️ Encodes one measured flat byte backing while retaining partial original output.
 fn with_native_encoding(&mut self,owner:&mut OriginalSourceEncodingOwner,output:&mut Option<Vec<u8>>,encode:impl FnOnce(&mut NativeEncodeControl<'_>,&mut Option<Vec<u8>>)->Result<(),ValueError>)->Result<(),OriginalTurnFailure>;
}
impl OriginalSourceEncodingContext for StepContext<'_>{
 fn with_native_encoding(&mut self,owner:&mut OriginalSourceEncodingOwner,output:&mut Option<Vec<u8>>,encode:impl FnOnce(&mut NativeEncodeControl<'_>,&mut Option<Vec<u8>>)->Result<(),ValueError>)->Result<(),OriginalTurnFailure>{
  self.original_source_parent()?.validate_borrowed_context(self)?;if !owner.admitted(){return Err(encoding_refusal("Source encoding requires its admitted original authorities").into())}if owner.allocation_pending(){return Err(encoding_refusal("Source encoding retains an unsettled allocation").into())}if owner.failure.is_some(){return Err(encoding_refusal("Source encoding retains unsettled effect failures").into())}if !owner.retirement.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"Source encoding requires its empty original retirement recipient").into())}if output.is_some(){return Err(encoding_refusal("Source encoding requires an empty original byte receiver").into())}if self.should_yield()||self.is_cancelled()||self.fuel_remaining()<2||self.retained_grant().maximum_items<2||self.retained_grant().maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Source encoding requires its funded live work and receiver metadata").into())}
  let start=self.retained_progress();let identity=OriginalSourceBinding::borrow(self);if owner.binding.is_some_and(|binding|binding!=identity){return Err(encoding_refusal("Source encoding owner changed its original caller").into())}self.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;self.consume_fuel(1);owner.binding=Some(identity);
  let maximum=self.retained_grant().maximum_capacity_bytes;let depth=self.retained_grant().maximum_depth;let turn_identity=self.original_source_turn_identity().ok_or_else(||encoding_refusal("Source encoding requires its original dispatch"))?;
  let missing=||encoding_refusal("Source encoding lost an admitted original authority");let retirement=&mut owner.retirement;let allocation=owner.allocation.authority.as_mut().ok_or_else(missing)?;let span=owner.span.authority.as_mut().ok_or_else(missing)?;let checkpoint=owner.checkpoint.authority.as_mut().ok_or_else(missing)?;let work=owner.work.authority.as_mut().ok_or_else(missing)?;let construction=owner.construction.authority.as_mut().ok_or_else(missing)?;
  let hop=Mutex::new(EncodingHop::default());let source=Mutex::new(&mut*self);
  let turn=EncodingTurn{source:&source,binding:identity,hop:&hop};let context:&dyn EncodingTurnContext=&turn;let(mut allocation_context,mut span_context,mut checkpoint_context,mut work_context,mut construction_context)=(context,context,context,context,context);
  let attempt=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||allocation.with_send_loan(&mut allocation_context,|allocate|span.with_send_loan(&mut span_context,|span_loan|checkpoint.with_send_loan(&mut checkpoint_context,|checkpoint_loan|{
   let mut control=NativeEncodeControl::new_recorded_source(maximum,depth,turn_identity,allocate,span_loan,checkpoint_loan)?;control.install_retirement_recipient(retirement)?;
   work.with_loan(&mut work_context,|work_loan|construction.with_loan(&mut construction_context,|construction_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(construction_loan,|control|encode(control,output)))).and_then(|result|result)).and_then(|result|result)
  }).and_then(|result|result)).and_then(|result|result)).and_then(|result|result)));
  let cx=source.into_inner().unwrap_or_else(|poison|poison.into_inner());let (effects,mut receipt,provenance)=settle_encoding(cx,output,&hop,start);
  let result=match attempt{Ok(result)=>result,Err(payload)=>{if receipt.is_some()||provenance.is_some(){*owner.failure=Some(SourceEncodingRetainedFailure{receipt,provenance})}std::panic::resume_unwind(payload)}};let mut primary=result.err();
  if let Some(provenance)=provenance{if primary.is_some(){*owner.failure=Some(SourceEncodingRetainedFailure{receipt:None,provenance:Some(provenance)})}else{primary=Some(provenance)}}
  let mut context=if identity.matches(self){None}else{Some(encoding_refusal("Source encoding changed its original recipient"))};
  if primary.is_none(){primary=receipt.take().or_else(||context.take())}match primary{None=>Ok(()),Some(primary)=>{let mut failure=OriginalTurnFailure::from(primary.with_retained_progress(effects));failure.receipt_refusal=receipt;failure.context_refusal=context;Err(failure)}}
 }
}
fn encoding_refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
