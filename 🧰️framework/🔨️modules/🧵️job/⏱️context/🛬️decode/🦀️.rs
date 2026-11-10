//! 🛬️ A paid original source lends actual native decode imports and receives the concrete producer's effects.
use semio_framework_job::{OriginalCallerSourceLoan,OriginalTurnFailure,OriginalTurnPrimary,StepContext};
use semio_framework_value::{ErasedSnapshotRetirement, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::{native_decoding::NativeDecodeControl, native_decoding::{OriginalConstructionAuthority,OriginalConstructionRecipient,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionAccept,OriginalConstructionAcceptComplete,NativeDecodeRetirementRecipient,OriginalWorkAuthority,OriginalWorkRecipient,OriginalWorkQuote,OriginalWorkQuoteComplete,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalSourceTurnIdentity}};
use semio_framework_value_native_owned::{native_decoding::{NativeAllocationAuthority,NativeAllocationAuthorityPort,NativeAllocationEvent,NativeAllocationRefusal,NativeAllocationReply,NativeAllocationReplyComplete,NativeAllocationRequest}, native_decoding::NativeForwardedDecodeFrame, RetainedWorkOutcome, RetainedWorkPoll};
use semio_framework_value_native_owned::native_decoding::allocation::NativeForwardedDecodeReceipt;
use std::{cell::UnsafeCell,mem::ManuallyDrop,sync::atomic::{AtomicBool,Ordering}};

/// 📋️ The original caller authors a fixed cumulative ownership ceiling before decoding begins.
#[derive(Clone,Copy,Debug)]
pub struct OriginalNativeDecodePolicy{pub maximum_owned_bytes:usize}
/// 🧾️ The actual bounded producer returns its unchanged supplied grant, physical receipt and raw work.
pub struct OriginalNativeDecodeIssued<T,E=ValueError>{pub value:Result<T,E>,pub grant:RetainedCloneGrant,pub progress:RetainedCloneProgress,pub work:u64}
/// 🎫️ Names the receiving refusal that the issuing Source holds in its own funded slot and receipted in the failing turn.
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq,semio_framework_value::RetireOwned)]
#[value(crate="::semio_framework_value",send_only)]
pub struct OriginalReceivingRefusal{pub progress:RetainedCloneProgress}
/// 🧳️ A receiving refusal stays beside the original outcome without destroying either source; the Source-issued ticket resolves through that Source and a Sourceless observation keeps the failure inline.
#[derive(semio_framework_value::RetireOwned)]
#[value(crate="::semio_framework_value",send_only)]
pub struct OriginalNativeDecodeOutcome<T,E=ValueError,R=OriginalReceivingRefusal>{pub value:Result<T,E>,pub progress:RetainedCloneProgress,pub work:u64,pub reported_work:u64,pub observed_native_work:u64,pub receiving_refusal:Option<R>}

#[derive(Default)]
struct AllocationLedger{pending:Option<NativeAllocationRequest>,actual:u128}
struct ActualTurn<'a,'source>{actual:UnsafeCell<&'a mut StepContext<'source>>,reserved:UnsafeCell<u128>,borrowed:AtomicBool}
/// 🔒️ Every mutable caller access is excluded by the atomic gate; its declared context supports transfer.
unsafe impl<'a,'source> Sync for ActualTurn<'a,'source> where StepContext<'source>:Send{}
struct ActualGuard<'a>(&'a AtomicBool);
impl Drop for ActualGuard<'_>{fn drop(&mut self){self.0.store(false,Ordering::Release)}}
impl<'a,'source> ActualTurn<'a,'source>{
 fn new(actual:&'a mut StepContext<'source>)->Self{Self{actual:UnsafeCell::new(actual),reserved:UnsafeCell::new(0),borrowed:AtomicBool::new(false)}}
 fn with<R>(&self,run:impl FnOnce(&mut StepContext<'source>,&mut u128)->R)->Result<R,ValueError>{self.borrowed.compare_exchange(false,true,Ordering::Acquire,Ordering::Relaxed).map_err(|_|refusal("original native decoder caller is already borrowed"))?;let guard=ActualGuard(&self.borrowed);let result=unsafe{run(&mut **self.actual.get(),&mut *self.reserved.get())};drop(guard);Ok(result)}
}
/// 🏗️ The calling turn's own physical grant is the original construction recipient of every native frontier.
struct ActualPhysical{grant:RetainedCloneGrant,turn:Option<OriginalSourceTurnIdentity>}
struct ActualPhysicalDriver;
impl OriginalConstructionRecipient for ActualPhysicalDriver{
 type Context<'t,'s>=ActualPhysical where 's:'t;
 fn validate_context<'t,'s>(&self,_:&ActualPhysical)->Result<(),ValueError> where 's:'t{Ok(())}
 fn source_turn<'t,'s>(&self,physical:&ActualPhysical)->Option<OriginalSourceTurnIdentity> where 's:'t{physical.turn}
 fn grant<'r,'t,'s>(&mut self,physical:&mut ActualPhysical,quote:OriginalConstructionGrant<'r>)->OriginalConstructionGrantComplete<'r> where 's:'t{quote.complete(Ok(physical.grant))}
 fn accept<'r,'t,'s>(&mut self,physical:&mut ActualPhysical,receipt:OriginalConstructionAccept<'r>)->OriginalConstructionAcceptComplete<'r> where 's:'t{let(grant,progress)=(receipt.grant(),receipt.progress());let result=if !progress.fits(grant)||!progress.fits(physical.grant){Err(refusal("native construction receipt exceeds its physical grant"))}else{let left=&mut physical.grant;left.maximum_items-=progress.copied_items;left.maximum_copy_bytes-=progress.copied_bytes;left.maximum_capacity_bytes-=progress.retained_capacity_bytes;left.maximum_release_bytes-=progress.released_bytes;Ok(())};receipt.complete(result)}
 fn submitted<'t,'s>(&self,_:&ActualPhysical)->RetainedCloneProgress where 's:'t{RetainedCloneProgress::default()}
}
struct ActualDecodeFrame<'slot,'control,'turn,'call,'source>{frame:NativeForwardedDecodeFrame<'slot,'control>,turn:&'turn ActualTurn<'call,'source>,before:u128}
impl Drop for ActualDecodeFrame<'_,'_,'_,'_,'_>{fn drop(&mut self){if std::thread::panicking(){let work=self.frame.control().actual_work().saturating_sub(self.before);let work=u64::try_from(work).unwrap_or(u64::MAX);self.turn.with(|cx,_|cx.consume_fuel(work)).expect("original decoder work return requires its same unborrowed caller")}}}
struct ActualWork;
struct ActualWorkTurn<'a,'turn,'source>{turn:&'a ActualTurn<'turn,'source>,closing:bool}
trait ActualWorkContext:Sync{fn work_turn(&self)->Option<OriginalSourceTurnIdentity>;fn validate_work(&self)->Result<(),ValueError>;fn admit_work(&self,observed:u64,additional:u64)->bool;fn allocate(&self,ledger:&mut AllocationLedger,event:NativeAllocationEvent)->Result<(),ValueError>;}
impl ActualWorkContext for ActualWorkTurn<'_,'_,'_>{
 fn work_turn(&self)->Option<OriginalSourceTurnIdentity>{self.turn.with(|cx,_|cx.original_source_turn_identity()).ok().flatten()}
 fn validate_work(&self)->Result<(),ValueError>{self.turn.with(|cx,_|cx.original_source_parent().and_then(|parent|parent.validate_borrowed_context(cx)))?}
 fn admit_work(&self,observed:u64,additional:u64)->bool{self.turn.with(|cx,_|!paused(cx,self.closing)&&(self.closing||!cx.is_cancelled())&&observed.checked_add(additional).is_some_and(|work|work<=cx.fuel_remaining())).unwrap_or(false)}
 fn allocate(&self,ledger:&mut AllocationLedger,event:NativeAllocationEvent)->Result<(),ValueError>{let closing=self.closing;self.turn.with(|actual,reserved|match event{
  NativeAllocationEvent::Admit(request)=>{if request.owned_bytes.checked_add(request.bytes)!=Some(request.next_owned_bytes)||request.next_owned_bytes>request.maximum_bytes{return Err(refusal("original native decoder allocation coordinates are invalid"))}Ok(())}
  NativeAllocationEvent::Quote(request)=>{
   if ledger.pending.is_some(){return Err(refusal("original native decoder already has a pending allocation"))}
   if paused(actual,closing)||(!closing&&actual.is_cancelled()){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original native decode caller yielded"))}
   if request.owned_bytes as u128!=ledger.actual||request.owned_bytes.checked_add(request.bytes)!=Some(request.next_owned_bytes)||request.next_owned_bytes>request.maximum_bytes{return Err(refusal("original native decoder allocation coordinates are invalid"))}
   if reserved.checked_add(request.bytes as u128).is_none_or(|bytes|bytes>actual.retained_grant().maximum_capacity_bytes as u128){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original native allocation exceeds the actual turn capacity"))}
   ledger.pending=Some(request);Ok(())
  }
  NativeAllocationEvent::Post{request,actual:born}=>{
   ledger.actual=ledger.actual.checked_add(born as u128).ok_or_else(||refusal("original native allocation observation overflow"))?;
   *reserved=reserved.checked_add(born as u128).ok_or_else(||refusal("original native turn allocation overflow"))?;
   if ledger.pending!=Some(request){return Err(refusal("original native allocation post does not match its reservation"))}
   ledger.pending=None;
   if born>request.bytes||*reserved>actual.retained_grant().maximum_capacity_bytes as u128{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original native allocation exceeded its reservation"))}
   if paused(actual,closing)||(!closing&&actual.is_cancelled()){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original native allocation post was refused"))}Ok(())
  }
  NativeAllocationEvent::Cancel(request)=>{if ledger.pending!=Some(request){return Err(refusal("original native allocation cancellation does not match its reservation"))}ledger.pending=None;Ok(())}
 }).and_then(|answer|answer)}
}
struct DecodeAllocationProducer{ledger:AllocationLedger}
impl NativeAllocationAuthorityPort for DecodeAllocationProducer{
 type Context<'turn,'source>=&'turn(dyn ActualWorkContext+'turn) where 'source:'turn;
 fn validate_context<'turn,'source>(&self,turn:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn{turn.validate_work()}
 fn source_turn<'turn,'source>(&self,turn:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity> where 'source:'turn{turn.work_turn()}
 fn receive<'reply,'turn,'source>(&mut self,turn:&mut Self::Context<'turn,'source>,event:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{let result=turn.validate_work().and_then(|()|turn.allocate(&mut self.ledger,event)).map_err(NativeAllocationRefusal::from);Ok(reply.complete(result))}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,_:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{Ok(reply.pending())}
}
impl OriginalWorkRecipient for ActualWork{
 type Context<'turn,'source>=&'turn(dyn ActualWorkContext+'turn) where 'source:'turn;
 fn validate_context<'turn,'source>(&self,turn:&Self::Context<'turn,'source>)->Result<(),ValueError> where 'source:'turn{turn.validate_work()}
 fn source_turn<'turn,'source>(&self,turn:&Self::Context<'turn,'source>)->Option<OriginalSourceTurnIdentity> where 'source:'turn{turn.work_turn()}
 fn admits<'reply,'turn,'source>(&mut self,turn:&mut Self::Context<'turn,'source>,quote:OriginalWorkQuote<'reply>,observed:u64,additional:u64)->OriginalWorkQuoteComplete<'reply> where 'source:'turn{let result=turn.validate_work().map(|()|turn.admit_work(observed,additional));quote.complete(result)}
 fn settle_partial<'reply,'turn,'source>(&mut self,turn:&mut Self::Context<'turn,'source>,check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(turn.validate_work())}
 fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut Self::Context<'turn,'source>,check:OriginalWorkPartialCheck<'reply>)->OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.pending()}
}
fn refusal(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn paused(cx:&StepContext<'_>,closing:bool)->bool{if closing{cx.retirement_should_yield()}else{cx.should_yield()}}
fn metadata(cx:&mut StepContext<'_>,closing:bool)->Result<bool,OriginalTurnFailure>{if paused(cx,closing)||cx.retained_grant().maximum_items==0||cx.retained_grant().maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);Ok(true)}

/// 🎟️ Owns the genuine source loan, original retirement recipient and consuming forwarded decoder receipt.
pub struct OriginalNativeDecodeSource{policy:OriginalNativeDecodePolicy,source:ManuallyDrop<Option<OriginalCallerSourceLoan>>,original_recipient:ManuallyDrop<Option<NativeDecodeRetirementRecipient>>,receipt:Option<NativeForwardedDecodeReceipt>,allocation_authority:ManuallyDrop<Option<NativeAllocationAuthority<DecodeAllocationProducer>>>,issuer_admitted:bool,work_authority:ManuallyDrop<Option<OriginalWorkAuthority<ActualWork>>>,work_admitted:bool,construction_authority:ManuallyDrop<Option<OriginalConstructionAuthority<ActualPhysicalDriver>>>,construction_admitted:bool,refusal:ManuallyDrop<Option<OriginalTurnFailure>>,closing:bool,returning:bool,closed:bool}
impl OriginalNativeDecodeSource{
 /// 🪹️ Keeps only an authored declaration; no source, recipient, controller or allocation is minted here.
 pub const fn empty(policy:OriginalNativeDecodePolicy)->Self{Self{policy,source:ManuallyDrop::new(None),original_recipient:ManuallyDrop::new(None),receipt:None,allocation_authority:ManuallyDrop::new(None),issuer_admitted:false,work_authority:ManuallyDrop::new(None),work_admitted:false,construction_authority:ManuallyDrop::new(None),construction_admitted:false,refusal:ManuallyDrop::new(None),closing:false,returning:false,closed:false}}
 /// 🪪️ Admits the actual source before recipient birth and before any native decode import can be lent.
 pub fn admit_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.closing||self.closed{return Err(refusal("closing original native decode source cannot admit work").into())}
  if let Some(source)=self.source.as_ref(){source.validate_context(cx)?}else{cx.original_source_parent()?.admit_loan(&mut self.source,cx)?;return Ok(false)}
  if self.allocation_authority.is_none(){let bytes=std::mem::size_of::<NativeAllocationAuthority<DecodeAllocationProducer>>();let grant=cx.retained_grant();if paused(cx,false)||cx.is_cancelled()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}*self.allocation_authority=Some(NativeAllocationAuthority::new(DecodeAllocationProducer{ledger:Default::default()}));let receipt=cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()});cx.consume_fuel(1);receipt?;self.issuer_admitted=true;return Ok(false)}
  if !self.issuer_admitted{return Ok(false)}
  if self.work_authority.is_none(){let bytes=std::mem::size_of::<OriginalWorkAuthority<ActualWork>>();let grant=cx.retained_grant();if paused(cx,false)||cx.is_cancelled()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}*self.work_authority=Some(OriginalWorkAuthority::new(ActualWork));let receipt=cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()});cx.consume_fuel(1);receipt?;self.work_admitted=true;return Ok(false)}if !self.work_admitted{return Ok(false)}
  if self.construction_authority.is_none(){let bytes=std::mem::size_of::<OriginalConstructionAuthority<ActualPhysicalDriver>>();let grant=cx.retained_grant();if paused(cx,false)||cx.is_cancelled()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}*self.construction_authority=Some(OriginalConstructionAuthority::new(ActualPhysicalDriver));let receipt=cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()});cx.consume_fuel(1);receipt?;self.construction_admitted=true;return Ok(false)}if !self.construction_admitted{return Ok(false)}
  if self.original_recipient.is_none(){if !metadata(cx,false)?{return Ok(false)}*self.original_recipient=Some(NativeDecodeRetirementRecipient::new());return Ok(false)}
  if self.receipt.is_none(){if !metadata(cx,false)?{return Ok(false)}let _=self.with_control(cx,false,|_|())?;return Ok(false)}
  Ok(true)
 }
 /// 🧵️ The Source dispatch token stays inside the exclusively borrowed caller context for the whole control scope, so its witness is lent to the native control without a second borrow of that context.
 fn with_control<T>(&mut self,cx:&mut StepContext<'_>,closing:bool,run:impl FnOnce(&mut NativeDecodeControl<'_>)->T)->Result<(T,u64),OriginalTurnFailure>{
  self.source.as_ref().ok_or_else(||refusal("original native decoder has no admitted source"))?.validate_context(cx)?;if !self.issuer_admitted||!self.work_admitted||!self.construction_admitted{return Err(refusal("original native decoder issuers have not been accepted").into())}
  let grant=cx.retained_grant();let turn_identity=cx.original_source_turn_identity();let source_turn=cx.original_source_turn().ok().map(std::ptr::from_ref);
  let borrowed=ActualTurn::new(&mut *cx);
  let mut progress=|_|borrowed.with(|actual,_|!paused(actual,closing)&&(closing||!actual.is_cancelled())).unwrap_or(false);
  let allocation=self.allocation_authority.as_mut().ok_or_else(||refusal("original native decoder has no allocation authority"))?;
  let original_recipient=self.original_recipient.as_mut().ok_or_else(||refusal("original native decoder has no admitted recipient"))?;
  let authority=self.work_authority.as_mut().ok_or_else(||refusal("original native decoder has no admitted work authority"))?;
  let construction=self.construction_authority.as_mut().ok_or_else(||refusal("original native decoder has no admitted construction authority"))?;
  let receipt=&mut self.receipt;let maximum=self.policy.maximum_owned_bytes;
  let work_turn=ActualWorkTurn{turn:&borrowed,closing};let context:&dyn ActualWorkContext=&work_turn;let(mut allocation_context,mut work_context)=(context,context);
  let answer=allocation.with_send_loan(&mut allocation_context,|allocate|{
   let frame=NativeForwardedDecodeFrame::begin(receipt,maximum,&mut progress,allocate,original_recipient)?;
   let mut frame=ActualDecodeFrame{before:0,frame,turn:&borrowed};frame.before=frame.frame.control().actual_work();let before_work=frame.before;
   let answer=authority.with_loan(&mut work_context,|work_loan|frame.frame.control().with_work_recipient(work_loan,|control|control.with_source_witness(source_turn.map(|token|unsafe{&*token}.witness()),|control|{let mut physical=ActualPhysical{grant,turn:turn_identity};construction.with_loan(&mut physical,|physical|control.with_construction_recipient(physical,|control|Ok::<_,ValueError>(run(control)))).and_then(|answer|answer)}))).and_then(|answer|answer);
   let observed=u64::try_from(frame.frame.control().actual_work().saturating_sub(before_work)).unwrap_or(u64::MAX);drop(frame);
   answer.map(|value|(value,observed))
  }).and_then(|answer|answer);
  answer.map_err(OriginalTurnFailure::from)
 }
 fn allocation_pending(&self)->bool{self.allocation_authority.as_ref().is_some_and(|authority|authority.has_pending_transaction()||authority.inspect_port(|producer|producer.ledger.pending.is_some()))}
 fn close_recipient_step(&mut self,cx:&mut StepContext<'_>)->Result<RetainedCloneStep,OriginalTurnFailure>{
  let grant=cx.retained_grant();
  let Some((polled,performed))=cx.with_original_effect(grant,|cx,effect|self.with_control(cx,true,|control|control.close_retirement_recipient(effect.grant().maximum_copy_bytes,effect)))?else{return Ok(RetainedCloneStep::Progress(Default::default()))};
  let (poll,_observed)=polled?;
  let turn=match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work}};
  let overrun=u128::from(turn.work)<performed;
  let step=semio_framework_job::receive_original_retained_work(cx,turn)?;
  if overrun{return Err(refusal("original recipient close performed more work than it settled").into())}Ok(step)
 }
 /// 🧮️ Reservations never debit birth; the concrete bounded producer's actual receipt returns to this exact caller.
 pub fn receive_step<T,E>(&mut self,cx:&mut StepContext<'_>,run:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->OriginalNativeDecodeIssued<T,E>)->Result<Option<OriginalNativeDecodeOutcome<T,E>>,OriginalTurnFailure>{
  if !self.admit_step(cx)?||cx.should_yield()||cx.is_cancelled(){return Ok(None)}
  let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}let fuel=cx.fuel_remaining();let (issued,observed)=self.with_control(cx,false,|control|run(control,grant))?;
  Ok(Some(self.hold(Self::receive_observed_work(cx,grant,fuel,issued,observed),cx)))
 }
 /// ♻️ A retained producer borrows its authentic decoder during paid close even after original cancellation.
 pub fn close_producer_step<T,E>(&mut self,cx:&mut StepContext<'_>,run:impl FnOnce(&mut NativeDecodeControl<'_>,RetainedCloneGrant)->OriginalNativeDecodeIssued<T,E>)->Result<Option<OriginalNativeDecodeOutcome<T,E>>,OriginalTurnFailure>{
  if self.closing{return Err(refusal("original producer close cannot resume after native source return began").into())}if self.source.is_none()||cx.retirement_should_yield(){return Ok(None)}let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}let fuel=cx.fuel_remaining();let (issued,observed)=self.with_control(cx,true,|control|run(control,grant))?;Ok(Some(self.hold(Self::receive_observed_work(cx,grant,fuel,issued,observed),cx)))
 }
 /// ♻️ A retained producer closes under the calling turn's own original effect and its authentic decoder.
 pub fn close_producer_effect_step<T,E>(&mut self,cx:&mut StepContext<'_>,run:impl FnOnce(&mut NativeDecodeControl<'_>,&mut RetainedAllocationEffect<'_>)->OriginalNativeDecodeIssued<T,E>)->Result<Option<OriginalNativeDecodeOutcome<T,E>>,OriginalTurnFailure>{
  if self.closing{return Err(refusal("original producer close cannot resume after native source return began").into())}if self.source.is_none()||cx.retirement_should_yield(){return Ok(None)}let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}let fuel=cx.fuel_remaining();
  let Some((answer,performed))=cx.with_original_effect(grant,|cx,effect|self.with_control(cx,true,|control|run(control,effect)))?else{return Ok(None)};
  let (issued,observed)=answer?;
  if u128::from(issued.work)<performed{return Err(refusal("original producer close performed more work than it settled").into())}
  Ok(Some(self.hold(Self::receive_observed_work(cx,grant,fuel,issued,observed),cx)))
 }
 fn receive_issued<T,E>(cx:&mut StepContext<'_>,grant:RetainedCloneGrant,fuel:u64,issued:OriginalNativeDecodeIssued<T,E>)->OriginalNativeDecodeOutcome<T,E,OriginalTurnFailure>{
  Self::receive_observed_work(cx,grant,fuel,issued,0)
 }
 /// 🧾️ Posts the actual original producer observation once while retaining its independent value and receiving refusal.
 pub fn receive_observed_work<T,E>(cx:&mut StepContext<'_>,grant:RetainedCloneGrant,fuel:u64,issued:OriginalNativeDecodeIssued<T,E>,observed:u64)->OriginalNativeDecodeOutcome<T,E,OriginalTurnFailure>{
  let mut receiving_refusal=None;
  if issued.grant!=grant||!issued.progress.fits(grant){receiving_refusal=Some(OriginalTurnFailure::from(refusal("original native producer changed its supplied grant or exceeded its physical receipt").with_retained_progress(issued.progress)))}
  if issued.work>fuel&&receiving_refusal.is_none(){receiving_refusal=Some(ValueError::literal(ValueRefusalKind::WorkLimit,"original native producer exceeded the supplied caller work").with_retained_progress(issued.progress).into())}
  if issued.work<observed{let error=ValueError::literal(ValueRefusalKind::InvariantViolated,"original native producer omitted observed native work");if let Some(failure)=receiving_refusal.as_mut(){failure.context_refusal=Some(error)}else{receiving_refusal=Some(error.into())}}
  let posted_work=issued.work.max(observed);cx.consume_fuel(posted_work);
  if let Err(error)=cx.consume_retained(issued.progress){if let Some(failure)=receiving_refusal.as_mut(){failure.receipt_refusal=Some(error)}else{receiving_refusal=Some(error.into())}}
  OriginalNativeDecodeOutcome{value:issued.value,progress:issued.progress,work:posted_work,reported_work:issued.work,observed_native_work:observed,receiving_refusal}
 }
 /// 📏️ Quotes the retained recipient or original source metadata return without constructing an authority.
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.closed{return Ok(Default::default())}if let Some(recipient)=self.original_recipient.as_ref().filter(|recipient|!recipient.terminal_is_empty()){return Ok(RetirementDemand{copy_bytes:recipient.next_copy_byte_demand()?,capacity_bytes:recipient.next_capacity_byte_demand(body)?,release_bytes:recipient.next_release_byte_demand()?,depth:recipient.next_depth_demand()?})}Ok(RetirementDemand{depth:1,..Default::default()})}
 /// 📪️ Returns recipient owners and the consuming receipt before the genuine source loan's last close phase.
 pub fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<RetainedCloneStep,OriginalTurnFailure>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}
  if self.allocation_pending(){return Err(refusal("original native decoder still owns a pending allocation").into())}
  if self.receipt.as_ref().is_some_and(NativeForwardedDecodeReceipt::has_pending_work){return Err(refusal("original native decoder still owns unresolved original work").into())}
  self.closing=true;
  if cx.retirement_should_yield(){return Ok(RetainedCloneStep::Progress(Default::default()))}
  if self.original_recipient.as_ref().is_some_and(|recipient|!recipient.terminal_is_empty()){
   let grant=cx.retained_grant();let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
   if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}
   return self.close_recipient_step(cx)
  }
  if self.refusal.is_some(){if !metadata(cx,true)?{return Ok(RetainedCloneStep::Progress(Default::default()))}self.refusal.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
  if self.receipt.is_some(){if !metadata(cx,true)?{return Ok(RetainedCloneStep::Progress(Default::default()))}self.receipt.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
  if self.original_recipient.is_some(){if !metadata(cx,true)?{return Ok(RetainedCloneStep::Progress(Default::default()))}self.original_recipient.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}))}
  if self.construction_authority.is_some(){if self.construction_authority.as_ref().is_some_and(OriginalConstructionAuthority::has_live_actions){return Err(refusal("original native construction issuer retains an action").into())}let grant=cx.retained_grant();if cx.retirement_should_yield()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}self.construction_authority.take();self.construction_admitted=false;let progress=RetainedCloneProgress{copied_items:1,..Default::default()};let receipt=cx.consume_retained(progress);cx.consume_fuel(1);receipt?;return Ok(RetainedCloneStep::Progress(progress))}
  if self.work_authority.is_some(){if self.work_authority.as_ref().is_some_and(OriginalWorkAuthority::has_live_actions){return Err(refusal("original native work issuer retains a pending quote").into())}let grant=cx.retained_grant();if cx.retirement_should_yield()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}self.work_authority.take();self.work_admitted=false;let progress=RetainedCloneProgress{copied_items:1,..Default::default()};let receipt=cx.consume_retained(progress);cx.consume_fuel(1);receipt?;return Ok(RetainedCloneStep::Progress(progress))}
  if self.allocation_authority.is_some(){if self.allocation_pending(){return Err(refusal("original native decoder still owns a pending allocation").into())}let grant=cx.retained_grant();if cx.retirement_should_yield()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_depth==0{return Ok(RetainedCloneStep::Progress(Default::default()))}let authority=self.allocation_authority.take().ok_or_else(||refusal("original native decoder lost its allocation authority"))?;if authority.into_port().is_err(){return Err(refusal("original native decoder still owns a pending allocation").into())}self.issuer_admitted=false;let progress=RetainedCloneProgress{copied_items:1,..Default::default()};let receipt=cx.consume_retained(progress);cx.consume_fuel(1);receipt?;return Ok(RetainedCloneStep::Progress(progress))}
  if let Some(source)=self.source.as_mut(){self.returning=true;let parent=cx.original_source_parent()?;let before=cx.retained_progress();source.close_step(parent,cx)?;if source.terminal_is_empty(){self.source.take();}let after=cx.retained_progress();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:after.copied_items-before.copied_items,copied_bytes:after.copied_bytes-before.copied_bytes,retained_capacity_bytes:after.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:after.released_bytes-before.released_bytes}))}
  if !metadata(cx,true)?{return Ok(RetainedCloneStep::Progress(Default::default()))}self.closed=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
 }
 /// 📊️ Borrows the retained consuming decoder ledger without creating another admission.
 pub fn owned_bytes(&self)->Option<usize>{self.receipt.as_ref().map(NativeForwardedDecodeReceipt::owned_bytes)}
 /// 🪫️ No original recipient, decoder receipt or source alias remains after the final paid turn.
 pub fn terminal_is_empty(&self)->bool{self.closed&&self.source.is_none()&&self.original_recipient.is_none()&&self.receipt.is_none()&&self.allocation_authority.is_none()&&self.work_authority.is_none()&&self.construction_authority.is_none()&&self.refusal.is_none()}
 /// 🎫️ Moves one receiving refusal into the funded slot, receipting the move in the failing turn; a later refusal adds its semantic cause to the held one.
 fn hold<T,E>(&mut self,outcome:OriginalNativeDecodeOutcome<T,E,OriginalTurnFailure>,cx:&mut StepContext<'_>)->OriginalNativeDecodeOutcome<T,E>{
  let OriginalNativeDecodeOutcome{value,progress,work,reported_work,observed_native_work,receiving_refusal}=outcome;
  let receiving_refusal=receiving_refusal.map(|mut failure|{
   let moved=RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Option<OriginalTurnFailure>>(),..Default::default()};
   if let Err(error)=cx.consume_retained(moved){failure.receipt_refusal.get_or_insert(error);}
   match self.refusal.as_mut(){
    Some(held)=>{if let OriginalTurnPrimary::Value(error)=failure.primary{held.context_refusal.get_or_insert(error);}}
    None=>*self.refusal=Some(failure)
   }
   OriginalReceivingRefusal{progress:moved}
  });
  OriginalNativeDecodeOutcome{value,progress,work,reported_work,observed_native_work,receiving_refusal}
 }
 /// 🔎️ Borrows the held receiving refusal that an issued ticket names.
 pub fn receiving_refusal(&self,_ticket:&OriginalReceivingRefusal)->Option<&OriginalTurnFailure>{self.refusal.as_ref()}
 /// 🫳️ Takes the held receiving refusal out of the slot; the slot retires through the close ladder only while it is still held.
 pub fn take_receiving_refusal(&mut self,_ticket:OriginalReceivingRefusal)->OriginalTurnFailure{self.refusal.take().unwrap_or_else(||refusal("original native decode source holds no receiving refusal for this ticket").into())}
}
impl Drop for OriginalNativeDecodeSource{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"original native decode source requires explicit terminal source return")}}

#[cfg(test)]
#[path="🎟️transport/🧪️tests/🦀️.rs"]
mod transport_tests;

#[cfg(test)]#[path="🎟️work/🧪️tests/🦀️.rs"]mod actual_work_tests;
