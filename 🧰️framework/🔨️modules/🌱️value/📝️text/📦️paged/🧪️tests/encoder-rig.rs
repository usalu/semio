//! 🏦️ Paged encoder laws retain one genuine Source and finite wallet through every phase.
use semio_framework_value_paged_text as value;
use value::*;
use semio_framework_value_retirement::retirement::RetireOwned;
use semio_framework_value_native_encoding::{self as native,*};
use native::native_receiving::span::*;
use std::sync::{Arc,Mutex};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(crate) struct Stock{pub(crate) grant:RetainedCloneGrant,pub(crate) work:u128,pub(crate) submitted:RetainedCloneProgress,pub(crate) partials:usize,pub(crate) posts:usize}
impl Stock{
 /// 📐️ Bounds both byte passes, advancing reserves, their reply/rearm payments and the final loan/projection/header tail.
 ///
 /// Each reserve adds a slot, so at most one reserve per output byte is possible. The defining list quote already bounds every branch probe and allocator call. Each native payment has a 4096-byte copy ceiling and spends at least one original Work unit.
 pub(crate) fn for_outputs<const N:usize>(outputs:&[(usize,u64)],capacity:usize,depth:usize)->Result<Self,ValueError>{
  let invalid=||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original encoder funding bound overflow");let mut work=0u128;
  for &(bytes,bind) in outputs{let reserve=value::list::PagedList::<u8,N>::empty().reserve_capacity_one_work_demand(bytes).map_err(|_|invalid())?;let bytes=bytes as u128;let reserves=bytes.max(1);let operations=bytes.checked_mul(2).and_then(|n|n.checked_add(reserves)).and_then(|n|n.checked_add(4)).ok_or_else(invalid)?;let body=bytes.checked_mul(6).and_then(|n|reserves.checked_mul(u128::from(reserve)+2).and_then(|r|n.checked_add(r))).and_then(|n|n.checked_add(u128::from(bind)+11)).ok_or_else(invalid)?;let metadata=operations.checked_mul(2).ok_or_else(invalid)?;work=work.checked_add(body).and_then(|n|n.checked_add(metadata)).and_then(|n|n.checked_add(8)).ok_or_else(invalid)?;}
  let items=usize::try_from(work).map_err(|_|invalid())?;let copy=usize::try_from(work.checked_mul(4096).ok_or_else(invalid)?).map_err(|_|invalid())?;Ok(Self{grant:RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:copy,maximum_capacity_bytes:capacity,maximum_release_bytes:capacity,maximum_depth:depth},work,submitted:Default::default(),partials:0,posts:0})
 }
}
#[derive(Clone)]
struct Port{source:Arc<OriginalSourceTurnIssuer>,stock:Arc<Mutex<Stock>>,interrupt:Arc<std::sync::atomic::AtomicBool>,interrupt_post:Arc<std::sync::atomic::AtomicBool>,allocation_posts:Arc<std::sync::atomic::AtomicUsize>}
macro_rules! bound{()=>{type Context<'a,'b>=&'a OriginalSourceTurn where 'b:'a;fn validate_context<'a,'b>(&self,c:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{self.source.validate(c).map_err(Into::into)}fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.identity())}}}
impl NativeAllocationAuthorityPort for Port{
 bound!();
 fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:NativeAllocationEvent,r:NativeAllocationReply<'r>)->Result<NativeAllocationReplyComplete<'r>,ValueError>where 'b:'a{Ok(r.complete(Ok(())))}
 fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:NativeAllocationEvent,r:NativeAllocationReply<'r>)->Result<NativeAllocationReplyComplete<'r>,ValueError>where 'b:'a{Ok(r.pending())}
}
impl OriginalWorkRecipient for Port{
 bound!();
 fn admits<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,r:OriginalWorkQuote<'r>,_observed:u64,additional:u64)->OriginalWorkQuoteComplete<'r>where 'b:'a{let stock=self.stock.lock().unwrap();r.complete(Ok(u128::from(additional)<=stock.work))}
 fn settle_partial<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,r:OriginalWorkPartialCheck<'r>)->OriginalWorkPartialCheckComplete<'r>where 'b:'a{let valid=OriginalWorkRecipient::validate_context(self,c);let mut stock=self.stock.lock().unwrap();stock.work=stock.work.checked_sub(r.delta()).expect("original encoder finite Work");stock.partials+=1;r.complete(valid)}
 fn recover_partial<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,r:OriginalWorkPartialCheck<'r>)->OriginalWorkPartialCheckComplete<'r>where 'b:'a{r.pending()}
}
impl OriginalConstructionRecipient for Port{
 bound!();
 fn submitted<'a,'b>(&self,_:&Self::Context<'a,'b>)->RetainedCloneProgress where 'b:'a{self.stock.lock().unwrap().submitted}
 fn grant<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,r:OriginalConstructionGrant<'r>)->OriginalConstructionGrantComplete<'r>where 'b:'a{let d=r.demand();let left=self.stock.lock().unwrap().grant;let grant=if retained_clone_grant_admits(left,d){RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}else{Default::default()};let complete=r.complete(Ok(grant));if self.interrupt.swap(false,std::sync::atomic::Ordering::SeqCst){std::hint::black_box(&complete);panic!("original encoder grant recorded before unwind")}complete}
 fn accept<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,r:OriginalConstructionAccept<'r>)->OriginalConstructionAcceptComplete<'r>where 'b:'a{let p=r.progress();if p.retained_capacity_bytes!=0{self.allocation_posts.fetch_add(1,std::sync::atomic::Ordering::SeqCst);}let mut stock=self.stock.lock().unwrap();assert!(p.fits(r.grant())&&p.fits(stock.grant));stock.grant.maximum_items-=p.copied_items;stock.grant.maximum_copy_bytes-=p.copied_bytes;stock.grant.maximum_capacity_bytes-=p.retained_capacity_bytes;stock.grant.maximum_release_bytes-=p.released_bytes;stock.submitted=stock.submitted.checked_add(p).unwrap();stock.posts+=1;drop(stock);let done=r.complete(Ok(()));if self.interrupt_post.swap(false,std::sync::atomic::Ordering::SeqCst){std::hint::black_box(&done);panic!("original encoder final receipt recorded before unwind")}done}
}
struct Span{source:Arc<OriginalSourceTurnIssuer>}
impl OriginalRecordedProducer for Span{
 type Request=OriginalSpanRequest;type Reply=();bound!();
 fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:Self::Request,r:OriginalRecordedReply<'r,()>)->Result<OriginalRecordedReplyComplete<'r,()>,ValueError>where 'b:'a{Ok(r.complete(Ok(())))}
 fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:Self::Request,r:OriginalRecordedReply<'r,()>)->Result<OriginalRecordedReplyComplete<'r,()>,ValueError>where 'b:'a{Ok(r.pending())}
}
struct Check{source:Arc<OriginalSourceTurnIssuer>}
impl OriginalRecordedProducer for Check{
 type Request=OriginalCheckpointRequest;type Reply=bool;bound!();
 fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:Self::Request,r:OriginalRecordedReply<'r,bool>)->Result<OriginalRecordedReplyComplete<'r,bool>,ValueError>where 'b:'a{Ok(r.complete(Ok(true)))}
 fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,_:Self::Request,r:OriginalRecordedReply<'r,bool>)->Result<OriginalRecordedReplyComplete<'r,bool>,ValueError>where 'b:'a{Ok(r.pending())}
}
pub(crate) struct EncodeBank{interrupt_grant:Arc<std::sync::atomic::AtomicBool>,allocation_posts:Arc<std::sync::atomic::AtomicUsize>,interrupt_post:Arc<std::sync::atomic::AtomicBool>,turn:OriginalSourceTurn,stock:Arc<Mutex<Stock>>,allocation:NativeAllocationAuthority<Port>,work:OriginalWorkAuthority<Port>,physical:OriginalConstructionAuthority<Port>,span:OriginalRecordedAuthority<Span>,checkpoint:OriginalRecordedAuthority<Check>}
impl EncodeBank{
 pub(crate) fn new(stock:Stock,interrupt:bool)->Self{let source=Arc::new(OriginalSourceTurnIssuer::new());let turn=source.issue().unwrap();let stock=Arc::new(Mutex::new(stock));let interrupt_post=Arc::new(std::sync::atomic::AtomicBool::new(false));let interrupt_grant=Arc::new(std::sync::atomic::AtomicBool::new(interrupt));let allocation_posts=Arc::new(std::sync::atomic::AtomicUsize::new(0));let port=Port{source:source.clone(),stock:stock.clone(),interrupt:interrupt_grant.clone(),interrupt_post:interrupt_post.clone(),allocation_posts:allocation_posts.clone()};Self{interrupt_grant,allocation_posts,interrupt_post,turn,stock,allocation:NativeAllocationAuthority::new(port.clone()),work:OriginalWorkAuthority::new(port.clone()),physical:OriginalConstructionAuthority::new(port),span:OriginalRecordedAuthority::new(Span{source:source.clone()}),checkpoint:OriginalRecordedAuthority::new(Check{source})}}
 pub(crate) fn grant_interrupt(&self)->Arc<std::sync::atomic::AtomicBool>{self.interrupt_grant.clone()}
 pub(crate) fn allocation_posts(&self)->Arc<std::sync::atomic::AtomicUsize>{self.allocation_posts.clone()}
 pub(crate) fn post_interrupt(&self)->Arc<std::sync::atomic::AtomicBool>{self.interrupt_post.clone()}
 pub(crate) fn stock(&self)->Stock{*self.stock.lock().unwrap()}
 pub(crate) fn finish(self)->Stock{let stock=self.stock();assert!(self.work.into_recipient().is_ok());assert!(self.physical.into_recipient().is_ok());assert!(self.allocation.into_port().is_ok());assert!(self.span.into_port().is_ok());assert!(self.checkpoint.into_port().is_ok());stock}
 pub(crate) fn run_cancelable<T>(&mut self,capacity:usize,depth:usize,cancel:Option<&mut dyn FnMut(NativeEncodeProgress)->bool>,operation:impl FnOnce(&OriginalSourceTurn,&mut NativeEncodeControl<'_>)->Result<T,ValueError>)->Result<T,ValueError>{let turn=&self.turn;self.allocation.with_send_loan(&mut turn.clone(),|allocation|self.work.with_loan(&mut turn.clone(),|work|self.physical.with_loan(&mut turn.clone(),|physical|self.span.with_send_loan(&mut turn.clone(),|span|self.checkpoint.with_send_loan(&mut turn.clone(),|checkpoint|{let mut control=NativeEncodeControl::new_recorded_source(capacity,depth,turn.witness(),allocation,span,checkpoint)?;if let Some(cancel)=cancel{control.bind_cancel(cancel);}control.with_work_recipient(work,|control|control.with_construction_recipient(physical,|control|operation(turn,control)))}))?)?)?)??}
 pub(crate) fn run<T>(&mut self,capacity:usize,depth:usize,operation:impl FnOnce(&OriginalSourceTurn,&mut NativeEncodeControl<'_>)->Result<T,ValueError>)->Result<T,ValueError>{self.run_cancelable(capacity,depth,None,operation)}
}
#[derive(Default)]
pub(crate) struct Payment{pub(crate) physical:Option<OriginalConstructionAction>,pub(crate) work:Option<OriginalWorkActionIdentity>,receipt:Option<OriginalWorkPerformedReceipt>,claim:Option<Result<u128,ValueError>>,partial:Option<OriginalWorkPartialOutcome>,pub(crate) proof:Option<OriginalWorkActionProof>,actual:Option<usize>,progress:Option<RetainedCloneProgress>,maximum:u64}
impl Payment{
 pub(crate) fn begin(&mut self,control:&mut NativeEncodeControl<'_>,demand:RetirementDemand,maximum:u64)->Result<(),ValueError>{assert!(demand.copy_bytes<=4096);assert!(maximum<=4096);self.maximum=maximum;control.with_original_effect_work_and_construction(|work,physical|{physical.begin(demand,&mut self.physical,work)?;physical.quote(self.physical.as_mut().unwrap())?;assert!(physical.recover_grant(self.physical.as_mut().unwrap())?.unwrap().is_ok());assert_eq!(work.prepare(maximum,&mut self.work)?,Some(true));Ok(())})}
 pub(crate) fn stage(&mut self,control:&mut NativeEncodeControl<'_>,operation:impl FnOnce(&mut OriginalConstructionLoan<'_>,&OriginalConstructionAction,&mut RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneProgress>,ValueError>)->Result<(),ValueError>{control.with_original_effect_work_and_construction(|work,physical|{let mut scope=work.performed_scope(self.work,&mut self.receipt,&mut self.claim)?;let mut effect=scope.effect(self.physical.as_ref().unwrap().grant().unwrap(),self.maximum,&mut self.actual)?;self.progress=operation(physical,self.physical.as_ref().unwrap(),&mut effect)?;assert!(self.progress.is_some());Ok(())})}
 pub(crate) fn post(&mut self,control:&mut NativeEncodeControl<'_>)->Result<bool,ValueError>{control.with_original_effect_work_and_construction(|work,physical|{if self.proof.is_none(){if let Some(claim)=self.claim.take(){claim?;}if work.partial_pending(self.work)?{assert!(work.settle_partial_into(self.work,&mut self.partial)?);assert!(self.partial.as_ref().unwrap().source_refusal().is_none());drop(self.partial.take())}let actual=work.performed_total(self.work)?;assert!(work.commit(actual,self.work,&mut self.proof)?)}let action=self.physical.as_mut().unwrap();if action.phase()==OriginalConstructionPhase::Granted{physical.post(action,self.progress.unwrap(),self.actual)?}if action.phase()==OriginalConstructionPhase::AcceptStarted&&physical.recover_accept(action)?.is_none(){return Ok(false)}Ok(action.recorded_accept_status()==Some(OriginalConstructionAcceptStatus::Accepted))})}
 pub(crate) fn finish(&mut self,control:&mut NativeEncodeControl<'_>)->Result<(),ValueError>{control.with_original_effect_work_and_construction(|work,physical|{work.finish(&mut self.proof,&mut self.work)?;physical.finish_accepted(&mut self.physical)})}
}

fn selected(d:RetirementDemand)->RetainedCloneGrant{assert!(d.copy_bytes<=4096,"original paged metadata turn: {d:?}");RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
struct ProducerClose<'p,'i,'o,S:value::paged_text::TextReadSource+Sync+?Sized,const N:usize>{producer:&'p mut value::paged_text::PagedTextEncodeProducer<'i,'o,S,N>}
impl<S:value::paged_text::TextReadSource+Sync+?Sized,const N:usize> OriginalEffectProducer for ProducerClose<'_,'_,'_,S,N>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=OriginalSourceTurnWitness<'a> where 'b:'a;
 fn validate_context<'a,'b>(&self,c:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{if self.producer.source_identity()==c.identity(){Ok(())}else{Err(ValueError::invariant("paged close original Source changed"))}}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.identity())}
 fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>where 'b:'a{self.producer.close_into(4096,e,r)}
 fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>where 'b:'a{self.producer.recover_close_into(4096,e,r)}
}
/// 🧵️ Retains the actual Source loan through formatter publication and fully paid producer close.
fn encode_output<S:value::paged_text::TextReadSource+value::RetireOwned+Sync,const N:usize>(original:&semio_framework_value_clone_source::RetainedCloneSource<S>,control:&mut NativeEncodeControl<'_>,turn:&OriginalSourceTurn,maximum:usize,prefix:&'static str,suffix:&'static str,inline_mode:bool)->Result<(Option<value::paged_text::PagedText<N>>,Option<value::paged_text::InlineTextBuffer>),ValueError>{
 use value::paged_text::{PagedTextEncodeProducer,PagedTextEncodeFrame,PagedTextEncodeOperation,PagedTextLoanReturn};
 let mut output=None;let mut inline=None;
  let mut producer=if inline_mode{PagedTextEncodeProducer::<_,N>::new_inline(turn.witness(),original.borrow(),&mut output,&mut inline,maximum,prefix,suffix)?}else{PagedTextEncodeProducer::<_,N>::new(turn.witness(),original.borrow(),&mut output,maximum,prefix,suffix)?};
  let mut frame=PagedTextEncodeFrame::new(control,turn,&mut producer)?;
  drive_frame(&mut frame,control,turn)?;return_loan(&mut frame,control,turn)?;drop(frame);close_producer(&mut producer,control,turn)?;drop(producer);
Ok((output,inline))
}
/// 🔁️ Drives the actual same-Source body, reply and funded rearm sequence.
pub(crate) fn drive_one_snapshot<S:value::paged_text::TextReadSource+Sync+?Sized,const N:usize>(frame:&mut value::paged_text::PagedTextEncodeFrame<'_,'_,'_,S,N>,control:&mut NativeEncodeControl<'_>,turn:&OriginalSourceTurn)->Result<(value::paged_text::PagedTextEncodeOperation,bool,Option<NativeEncodeProducerOutcome>),ValueError>{use value::paged_text::PagedTextEncodeOperation;
   let operation=frame.operation()?;let (d,_,_)=frame.demand()?;let grant=selected(d);let mut ready=false;
   for _ in 0..1000{let step=frame.advance_body(control,turn,grant)?;frame.acknowledge_body_partial_success()?;frame.claim_body_work()?;if matches!(step,NativeEncodeSourcePreparation::Ready){ready=true;break}if matches!(step,NativeEncodeSourcePreparation::Refused)&&frame.failure().is_some_and(|failure|failure.primary.is_some()){return Ok((operation,false,None))}assert!(!matches!(step,NativeEncodeSourcePreparation::Denied|NativeEncodeSourcePreparation::Refused))}
   assert!(ready);let grant=selected(frame.reply_demand()?);let mut published=false;for _ in 0..1000{let step=frame.advance_reply(control,turn,grant)?;frame.acknowledge_reply_partial_success()?;frame.claim_reply_work()?;if matches!(step,NativeEncodeProducerReplyStep::Transferred(_)){published=true;break}}assert!(published);if frame.reply().unwrap().is_err(){return Ok((operation,false,None))}let outcome=*frame.reply().unwrap().as_ref().unwrap();assert!(frame.failure().is_none_or(|failure|failure.is_empty()));
   if operation!=PagedTextEncodeOperation::Publish{
   let grant=selected(frame.rearm_demand()?);let mut rearmed=false;for _ in 0..1000{let step=frame.advance_rearm(control,turn,grant)?;frame.acknowledge_rearm_partial_success()?;frame.claim_rearm_work()?;if matches!(step,NativeEncodeProducerReplyStep::Transferred(_)){rearmed=true;break}}assert!(rearmed);
   }Ok((operation,true,Some(outcome)))
}
/// 🪟️ Keeps the owned refusal resident while reporting only its native completion status.
pub(crate) fn drive_one_result<S:value::paged_text::TextReadSource+Sync+?Sized,const N:usize>(frame:&mut value::paged_text::PagedTextEncodeFrame<'_,'_,'_,S,N>,control:&mut NativeEncodeControl<'_>,turn:&OriginalSourceTurn)->Result<(value::paged_text::PagedTextEncodeOperation,bool),ValueError>{let(operation,success,_)=drive_one_snapshot(frame,control,turn)?;Ok((operation,success))}
/// 🧭️ Settles one actual successful producer operation before selecting another.
pub(crate) fn drive_one<S:value::paged_text::TextReadSource+Sync+?Sized,const N:usize>(frame:&mut value::paged_text::PagedTextEncodeFrame<'_,'_,'_,S,N>,control:&mut NativeEncodeControl<'_>,turn:&OriginalSourceTurn)->Result<value::paged_text::PagedTextEncodeOperation,ValueError>{let(operation,success)=drive_one_result(frame,control,turn)?;assert!(success);Ok(operation)}
/// 📤️ Hands back the exact published result under the original physical Post.
pub(crate) fn take_frame_reply<S:value::paged_text::TextReadSource+Sync+?Sized,const N:usize>(frame:&mut value::paged_text::PagedTextEncodeFrame<'_,'_,'_,S,N>,control:&mut NativeEncodeControl<'_>,turn:&OriginalSourceTurn,output:&mut Option<Result<NativeEncodeProducerOutcome,NativeEncodeProducerFailure>>)->Result<(),ValueError>{if frame.reply().is_none(){let pointer=frame.failure().and_then(|failure|failure.primary.as_ref()).expect("original body refusal absent").message.as_ptr();let mut error=None;{let mut payout=value::paged_text::PagedTextReplyPayout::new_body_failure(frame,turn,NativeEncodeEffectFailureSlot::Primary,&mut error)?;let mut pay=Payment::default();pay.begin(control,payout.demand()?,1)?;pay.stage(control,|physical,action,effect|payout.stage(physical,action,effect).result)?;assert!(pay.post(control)?);assert!(payout.finish(control,turn,&mut pay.physical,&mut pay.proof,&mut pay.work)?);}assert_eq!(error.as_ref().unwrap().message.as_ptr(),pointer);assert!(frame.failure().is_none_or(|failure|failure.is_empty()));retire_value(&mut error,control,turn)?;return Ok(())}let mut payout=value::paged_text::PagedTextReplyPayout::new(frame,turn,output)?;let mut pay=Payment::default();pay.begin(control,payout.demand()?,1)?;pay.stage(control,|physical,action,effect|payout.stage(physical,action,effect).result)?;assert!(pay.post(control)?);assert!(payout.finish(control,turn,&mut pay.physical,&mut pay.proof,&mut pay.work)?);Ok(())}
/// 🔁️ Completes the original paid body, reply and rearm actions.
pub(crate) fn drive_frame<S:value::paged_text::TextReadSource+Sync+?Sized,const N:usize>(frame:&mut value::paged_text::PagedTextEncodeFrame<'_,'_,'_,S,N>,control:&mut NativeEncodeControl<'_>,turn:&OriginalSourceTurn)->Result<(),ValueError>{for _ in 0..100000{if drive_one(frame,control,turn)?==value::paged_text::PagedTextEncodeOperation::Publish{return Ok(())}}panic!("original encoder did not complete within its maintained bound")}

/// 🧷️ Returns the original exclusive loan only after both physical receipts settle.
pub(crate) fn return_loan<S:value::paged_text::TextReadSource+Sync+?Sized,const N:usize>(frame:&mut value::paged_text::PagedTextEncodeFrame<'_,'_,'_,S,N>,control:&mut NativeEncodeControl<'_>,turn:&OriginalSourceTurn)->Result<(),ValueError>{use value::paged_text::PagedTextLoanReturn;
let mut returned=PagedTextLoanReturn::new(frame,turn)?;
  let mut transfer=Payment::default();transfer.begin(control,returned.transfer_demand()?,1)?;transfer.stage(control,|physical,payout,effect|returned.stage_transfer(turn,physical,payout,effect).result)?;assert!(transfer.post(control)?);assert!(returned.finish(control,turn,&mut transfer.physical,&mut transfer.proof,&mut transfer.work)?);
  let mut release=Payment::default();release.begin(control,returned.release_demand()?,1)?;release.stage(control,|physical,payout,effect|returned.stage_release(physical,payout,effect).result)?;assert!(release.post(control)?);assert!(returned.finish(control,turn,&mut release.physical,&mut release.proof,&mut release.work)?);drop(returned);Ok(())

}
/// 🧹️ Pays the actual retained projection and native producer-borrow header.
pub(crate) fn close_producer<S:value::paged_text::TextReadSource+Sync+?Sized,const N:usize>(producer:&mut value::paged_text::PagedTextEncodeProducer<'_,'_,S,N>,control:&mut NativeEncodeControl<'_>,turn:&OriginalSourceTurn)->Result<(),ValueError>{close_producer_observed(producer,control,turn,|_,_|{})}
/// 👁️ Reports only settled original close progress to corpus observers.
pub(crate) fn close_producer_observed<S:value::paged_text::TextReadSource+Sync+?Sized,const N:usize>(producer:&mut value::paged_text::PagedTextEncodeProducer<'_,'_,S,N>,control:&mut NativeEncodeControl<'_>,turn:&OriginalSourceTurn,observed:impl FnMut(RetainedCloneProgress,u64))->Result<(),ValueError>{close_producer_observed_with_limit(producer,control,turn,1000,observed)}
/// 🧮️ Preserves each maintained caller's original finite close-turn ceiling.
pub(crate) fn close_producer_with_limit<S:value::paged_text::TextReadSource+Sync+?Sized,const N:usize>(producer:&mut value::paged_text::PagedTextEncodeProducer<'_,'_,S,N>,control:&mut NativeEncodeControl<'_>,turn:&OriginalSourceTurn,maximum:usize)->Result<(),ValueError>{close_producer_observed_with_limit(producer,control,turn,maximum,|_,_|{})}
fn close_producer_observed_with_limit<S:value::paged_text::TextReadSource+Sync+?Sized,const N:usize>(producer:&mut value::paged_text::PagedTextEncodeProducer<'_,'_,S,N>,control:&mut NativeEncodeControl<'_>,turn:&OriginalSourceTurn,maximum_turns:usize,mut observed:impl FnMut(RetainedCloneProgress,u64))->Result<(),ValueError>{
producer.begin_close();let mut close=Some(OriginalEffectAuthority::new(ProducerClose{producer:&mut *producer}));
  for _ in 0..maximum_turns{if close.as_ref().unwrap().inspect(|p|p.producer.terminal_is_empty()){break}let (d,maximum)=close.as_ref().unwrap().inspect(|p|Ok::<_,ValueError>((p.producer.close_demand(4096)?,p.producer.close_work_demand(4096)?)))?;let mut pay=Payment::default();pay.begin(control,d,maximum)?;let mut action=None;let mut reply=None;let mut settled=None;let mut performed=0;pay.stage(control,|_,_,effect|{let before=effect.performed_boundary();assert!(close.as_mut().unwrap().drive(&mut turn.witness(),effect,&mut action)?);assert!(action.as_mut().unwrap().transfer_reply(&mut reply)?);close.as_mut().unwrap().finish(&mut action)?;let progress=reply.take().unwrap().unwrap().progress();settled=Some(progress);performed=u64::try_from(effect.performed_since(before)?).map_err(|_|ValueError::invariant("close work extent overflow"))?;Ok(Some(progress))})?;assert!(pay.post(control)?);pay.finish(control)?;observed(settled.unwrap(),performed)}
  assert!(close.as_ref().unwrap().inspect(|p|p.producer.terminal_is_empty()));let mut holder=None;let d=OriginalEffectAuthority::<ProducerClose<'_, '_, '_,S,N>>::producer_transfer_demand()?;let mut pay=Payment::default();pay.begin(control,d,1)?;pay.stage(control,|_,_,effect|OriginalEffectAuthority::transfer_producer(&mut close,&turn.witness(),&mut holder,effect).result)?;assert!(pay.post(control)?);pay.finish(control)?;
  let d=RetirementDemand{copy_bytes:std::mem::size_of_val(&holder),depth:1,..Default::default()};let mut pay=Payment::default();pay.begin(control,d,1)?;pay.stage(control,|_,_,effect|{assert!(effect.performed(1)?);drop(holder.take());Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;assert!(pay.post(control)?);pay.finish(control)?;drop(holder);drop(close);assert!(producer.terminal_is_empty());Ok(())
}
/// 📝️ Returns the actual paged output after the original producer loan and projection close.
pub(crate) fn encode_success<S:value::paged_text::TextReadSource+value::RetireOwned+Sync,const N:usize>(original:&semio_framework_value_clone_source::RetainedCloneSource<S>,control:&mut NativeEncodeControl<'_>,turn:&OriginalSourceTurn,maximum:usize,prefix:&'static str,suffix:&'static str)->Result<value::paged_text::PagedText<N>,ValueError>{encode_output(original,control,turn,maximum,prefix,suffix,false)?.0.ok_or_else(||ValueError::invariant("actual paged output is absent"))}
/// 🛑️ Drives the same inline or paged encode but returns the partially built owner with the refusal, so a cancelled encode keeps its physical allocations for the caller to retain.
pub(crate) fn encode_inline_retaining<S:value::paged_text::TextReadSource+value::RetireOwned+Sync,const N:usize>(original:&semio_framework_value_clone_source::RetainedCloneSource<S>,control:&mut NativeEncodeControl<'_>,turn:&OriginalSourceTurn,maximum:usize,prefix:&'static str,suffix:&'static str)->((Option<value::paged_text::PagedText<N>>,Option<value::paged_text::InlineTextBuffer>),Option<ValueError>){
 use value::paged_text::{PagedTextEncodeProducer,PagedTextEncodeFrame};
 let mut output=None;let mut inline=None;let mut failure=None;
 {
  let mut producer=match PagedTextEncodeProducer::<_,N>::new_inline(turn.witness(),original.borrow(),&mut output,&mut inline,maximum,prefix,suffix){Ok(producer)=>producer,Err(error)=>return((None,None),Some(error))};
  {
   match PagedTextEncodeFrame::new(control,turn,&mut producer){
    Ok(mut frame)=>{if let Err(error)=drive_frame(&mut frame,control,turn).and_then(|()|return_loan(&mut frame,control,turn)){failure=Some(error);}drop(frame);}
    Err(error)=>failure=Some(error),
   }
  }
  if let Err(error)=close_producer(&mut producer,control,turn){failure=failure.or(Some(error));}
 }
 ((output,inline),failure)
}
/// 🧩️ Returns the selected real inline or paged output after paid producer return.
pub(crate) fn encode_inline_success<S:value::paged_text::TextReadSource+value::RetireOwned+Sync,const N:usize>(original:&semio_framework_value_clone_source::RetainedCloneSource<S>,control:&mut NativeEncodeControl<'_>,turn:&OriginalSourceTurn,maximum:usize,prefix:&'static str,suffix:&'static str)->Result<(Option<value::paged_text::PagedText<N>>,Option<value::paged_text::InlineTextBuffer>),ValueError>{encode_output(original,control,turn,maximum,prefix,suffix,true)}

/// 🧭️ Quotes the real empty projection admission without reading semantic input.
pub(crate) fn projection_work<T:Sync+?Sized>(source:semio_framework_value_clone_source::RetainedCloneRef<'_,T>)->Result<u64,ValueError>{semio_framework_value_clone_source::RetainedOwnedProjectionAdmission::<T>::default().work_demand(source)}

struct FailureRetirement<T:RetireOwned>{owner:Option<T>,close:semio_framework_value_clone_source::RetainedCloneClose}
struct FailureLoan<'a,T:RetireOwned>{source:OriginalSourceTurnIdentity,frame:&'a mut FailureRetirement<T>}
impl<T:RetireOwned> OriginalEffectProducer for FailureLoan<'_,T>{
 type Output=RetainedCloneStep;type Failure=OriginalRetirementFailure;type Context<'a,'b>=OriginalSourceTurnWitness<'a> where 'b:'a;
 fn validate_context<'a,'b>(&self,c:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{if c.identity()==self.source{Ok(())}else{Err(ValueError::invariant("failure retirement Source changed"))}}
 fn source_turn<'a,'b>(&self,c:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{Some(c.identity())}
 fn receive<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>where 'b:'a{if self.frame.owner.is_some(){let turn=self.frame.close.begin_granted(&mut self.frame.owner,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}self.frame.close.step_into(4096,effect,reply)}
 fn recover<'r,'a,'b>(&mut self,_:&mut Self::Context<'a,'b>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>where 'b:'a{self.frame.close.recover_step_into(4096,effect,reply)}
}
pub(crate) fn retire_value<T:RetireOwned>(slot:&mut Option<T>,control:&mut NativeEncodeControl<'_>,turn:&OriginalSourceTurn)->Result<(),ValueError>{
 let mut frame=FailureRetirement{owner:None,close:Default::default()};let demand=RetirementDemand{copy_bytes:std::mem::size_of_val(slot)+std::mem::size_of_val(&frame.owner),depth:1,..Default::default()};assert!(demand.copy_bytes<=4096);let mut payment=Payment::default();payment.begin(control,demand,1)?;payment.stage(control,|_,_,effect|{assert!(effect.performed(1)?);frame.owner=slot.take();Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})?;assert!(payment.post(control)?);payment.finish(control)?;
 let mut authority=Some(OriginalEffectAuthority::new(FailureLoan{source:turn.identity(),frame:&mut frame}));
 for _ in 0..1000{if authority.as_ref().unwrap().inspect(|p|p.frame.owner.is_none()&&p.frame.close.is_empty()){break}
  let (d,work)=authority.as_ref().unwrap().inspect(|p|Ok::<_,ValueError>((p.frame.close.next_owner_demands::<T>(p.frame.owner.is_some(),&None,4096)?,p.frame.close.next_owner_work::<T>(p.frame.owner.is_some(),&None,4096)?)))?;assert!(d.copy_bytes<=4096);let mut action=None;let mut pay=Payment::default();pay.begin(control,d,work)?;pay.stage(control,|_,_,effect|{assert!(authority.as_mut().unwrap().drive(&mut turn.witness(),effect,&mut action)?);let step=action.as_ref().unwrap().recorded_reply().unwrap().as_ref().map_err(|_|ValueError::invariant("actual failure close refused"))?;Ok(Some(step.progress()))})?;assert!(pay.post(control)?);pay.finish(control)?;
  let mut reply=None;let d=OriginalEffectAuthority::<FailureLoan<'_,T>>::transfer_reply_demand()?;assert!(d.copy_bytes<=4096);let mut pay=Payment::default();pay.begin(control,d,1)?;pay.stage(control,|_,_,effect|authority.as_mut().unwrap().transfer_reply_into(&turn.witness(),&mut action,&mut reply,effect).result)?;assert!(pay.post(control)?);pay.finish(control)?;assert!(reply.as_ref().unwrap().is_ok());let d=RetirementDemand{copy_bytes:std::mem::size_of_val(&reply),depth:1,..Default::default()};let mut pay=Payment::default();pay.begin(control,d,1)?;pay.stage(control,|_,_,effect|{assert!(effect.performed(1)?);drop(reply.take());Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;assert!(pay.post(control)?);pay.finish(control)?;
 }
 assert!(authority.as_ref().unwrap().inspect(|p|p.frame.owner.is_none()&&p.frame.close.is_empty()));let mut returned=None;let d=OriginalEffectAuthority::<FailureLoan<'_,T>>::producer_transfer_demand()?;let mut pay=Payment::default();pay.begin(control,d,1)?;pay.stage(control,|_,_,effect|OriginalEffectAuthority::transfer_producer(&mut authority,&turn.witness(),&mut returned,effect).result)?;assert!(pay.post(control)?);pay.finish(control)?;let d=RetirementDemand{copy_bytes:std::mem::size_of_val(&returned),depth:1,..Default::default()};let mut pay=Payment::default();pay.begin(control,d,1)?;pay.stage(control,|_,_,effect|{assert!(effect.performed(1)?);drop(returned.take());Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;assert!(pay.post(control)?);pay.finish(control)?;drop(returned);drop(authority);assert!(frame.owner.is_none()&&frame.close.is_empty());Ok(())
}

/// 🗃️ Moves the published failure into its actual paid retirement owner.
pub(crate) fn retire_published_failure(output:&mut Option<Result<NativeEncodeProducerOutcome,NativeEncodeProducerFailure>>,control:&mut NativeEncodeControl<'_>,turn:&OriginalSourceTurn)->Result<(),ValueError>{let mut failure=None;let d=RetirementDemand{copy_bytes:std::mem::size_of_val(output)+std::mem::size_of_val(&failure),depth:1,..Default::default()};let mut pay=Payment::default();pay.begin(control,d,1)?;pay.stage(control,|_,_,effect|{assert!(effect.performed(1)?);let Err(error)=output.take().expect("actual published failure")else{panic!("failed operation published success")};failure=Some(error);Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;assert!(pay.post(control)?);pay.finish(control)?;retire_value(&mut failure,control,turn)}
