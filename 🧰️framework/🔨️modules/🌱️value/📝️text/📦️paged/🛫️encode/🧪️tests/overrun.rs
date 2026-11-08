//! 🧪️ An original paged Source overrun keeps its heap refusal and physical receipt independent.
use semio_framework_value_paged_text::{self as value,RetireOwned};
use value::*;
use value::paged_text::{PagedText,PagedTextEncodeProducer,PagedTextEncodeFrame,PagedTextEncodeOperation,PagedTextReplyPayout,TextReadSource,TextSourceByteReply,TextSourceByteComplete};
use semio_framework_value_native_encoding::{self as native,*};
use native::native_receiving::span::*;
use semio_framework_value_clone_source::RetainedCloneSource;
use semio_framework_value_clone_source_testing::FixtureSourceBirth;
use std::sync::{Mutex,atomic::{AtomicUsize,Ordering}};
#[path="../../🧪️tests/encoder-rig.rs"]mod rig;
use rig::{EncodeBank,Stock,Payment,return_loan,close_producer,retire_value};

struct Probe{body:String,cause:Mutex<Option<ValueError>>,calls:AtomicUsize,recoveries:AtomicUsize}
impl TextReadSource for Probe{
 fn byte_len(&self)->usize{self.body.len()}
 fn byte_at(&self,index:usize)->Option<u8>{self.body.as_bytes().get(index).copied()}
 fn read_recorded(&self,mut reply:TextSourceByteReply<'_,'_>)->TextSourceByteComplete{self.calls.fetch_add(1,Ordering::SeqCst);assert!(!reply.performed(3).unwrap());let cause=self.cause.lock().unwrap().take().expect("original source error");reply.complete(Err(cause),3)}
 fn recover_recorded(&self,_:TextSourceByteReply<'_,'_>)->Option<TextSourceByteComplete>{self.recoveries.fetch_add(1,Ordering::SeqCst);None}
}
impl RetireOwned for Probe{
 type RetirementCursor=<(String,Option<ValueError>,usize,usize) as RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{self.body.retirement_ready()&&self.cause.lock().unwrap().retirement_ready()}
 fn retirement_seed(self)->Self::RetirementCursor{(self.body,self.cause.into_inner().unwrap(),self.calls.into_inner(),self.recoveries.into_inner()).retirement_seed()}
 fn controlled_retirement_supported()->bool{<(String,Option<ValueError>,usize,usize) as RetireOwned>::controlled_retirement_supported()}
}
fn selected(d:RetirementDemand)->RetainedCloneGrant{assert!(d.copy_bytes<=4096);RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
#[test]
fn original_paged_source_overrun_retains_heap_error_and_override(){
 let law:serde_json::Value=serde_json::from_str(include_str!("overrun.json")).unwrap();let override_progress=RetainedCloneProgress{copied_items:law["originalItems"].as_u64().unwrap()as usize,copied_bytes:law["originalBytes"].as_u64().unwrap()as usize,..Default::default()};
 let message="original paged source heap error".to_owned();let message_bytes=message.capacity();let cause=ValueError::new(ValueRefusalKind::InvalidValue,message).with_retained_progress(override_progress);let pointer=cause.message.as_ptr();let mut original=RetainedCloneSource::from_owner(Probe{body:"x".to_owned(),cause:Mutex::new(Some(cause)),calls:AtomicUsize::new(0),recoveries:AtomicUsize::new(0)});
 let stock=Stock{grant:RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64},work:10000,submitted:Default::default(),partials:0,posts:0};let mut bank=EncodeBank::new(stock,false);let mut output:Option<PagedText<1024>>=None;let mut inline=None;
 let (result,heap)=semio_framework_value_testing_allocator::observe_retirement_allocations(||bank.run(4096,8,|turn,control|{
  let mut producer=PagedTextEncodeProducer::new_inline(turn.witness(),original.borrow(),&mut output,&mut inline,1024,"","")?;let mut frame=PagedTextEncodeFrame::new(control,turn,&mut producer)?;
  assert_eq!(frame.producer().unwrap().operation()?,PagedTextEncodeOperation::Bind);let(d,_,_)=frame.demand()?;let grant=selected(d);let mut ready=false;for _ in 0..32{let status=frame.advance_body(control,turn,grant)?;frame.acknowledge_body_partial_success()?;frame.claim_body_work()?;if matches!(status,NativeEncodeSourcePreparation::Ready){ready=true;break}}assert!(ready);
  let grant=selected(frame.reply_demand()?);let mut ready=false;for _ in 0..32{let status=frame.advance_reply(control,turn,grant)?;frame.acknowledge_reply_partial_success()?;frame.claim_reply_work()?;if matches!(status,NativeEncodeProducerReplyStep::Transferred(_)){ready=true;break}}assert!(ready);
  let grant=selected(frame.rearm_demand()?);let mut ready=false;for _ in 0..32{let status=frame.advance_rearm(control,turn,grant)?;frame.acknowledge_rearm_partial_success()?;frame.claim_rearm_work()?;if matches!(status,NativeEncodeProducerReplyStep::Transferred(_)){ready=true;break}}assert!(ready);
  assert_eq!(frame.producer().unwrap().operation()?,PagedTextEncodeOperation::Validate);let(d,work,_)=frame.demand()?;assert_eq!(work,law["bodyQuote"].as_u64().unwrap());let grant=selected(d);let before=control.actual_work();let mut pending=false;
  for _ in 0..32{let status=frame.advance_body(control,turn,grant)?;frame.acknowledge_body_partial_success()?;frame.claim_body_work()?;if matches!(status,NativeEncodeSourcePreparation::Pending)&&frame.producer().unwrap().resident_reply_facts().is_some(){pending=true;break}}assert!(pending);assert_eq!(control.actual_work()-before,u128::from(law["reportedWork"].as_u64().unwrap()));assert_eq!(original.borrow().get().calls.load(Ordering::SeqCst),1);assert_eq!(original.borrow().get().recoveries.load(Ordering::SeqCst),0);assert!(frame.reply().is_none());
  let(d,work)=frame.resident_reply_recovery_demand()?;assert_eq!(work,law["recoveryWork"].as_u64().unwrap());let grant=selected(d);let mut short=grant;short.maximum_copy_bytes-=1;let before=control.actual_work();assert!(matches!(frame.advance_resident_reply_recovery(control,turn,short)?,NativeEncodeProducerReplyStep::Denied));assert_eq!(control.actual_work(),before);assert_eq!(original.borrow().get().calls.load(Ordering::SeqCst),1);
  let mut recovered=false;for _ in 0..32{let step=frame.advance_resident_reply_recovery(control,turn,grant)?;frame.acknowledge_resident_recovery_partial_success()?;frame.claim_resident_recovery_work()?;if matches!(step,NativeEncodeProducerReplyStep::Transferred(_)){recovered=true;break}}assert!(recovered);assert_eq!(control.actual_work()-before,u128::from(law["recoveryWork"].as_u64().unwrap()));assert_eq!(original.borrow().get().calls.load(Ordering::SeqCst),1);assert_eq!(original.borrow().get().recoveries.load(Ordering::SeqCst),0);assert!(frame.failure().unwrap().work_overrun.is_some());
  let mut overrun=None;{let mut payout=PagedTextReplyPayout::new_body_failure(&mut frame,turn,NativeEncodeEffectFailureSlot::WorkOverrun,&mut overrun)?;let mut pay=Payment::default();pay.begin(control,payout.demand()?,1)?;pay.stage(control,|physical,action,effect|payout.stage(physical,action,effect).result)?;assert!(pay.post(control)?);assert!(payout.finish(control,turn,&mut pay.physical,&mut pay.proof,&mut pay.work)?)}assert_eq!(overrun.as_ref().unwrap().kind,ValueRefusalKind::WorkLimit);
  let grant=selected(frame.reply_demand()?);let mut ready=false;for _ in 0..32{let step=frame.advance_reply(control,turn,grant)?;frame.acknowledge_reply_partial_success()?;frame.claim_reply_work()?;if matches!(step,NativeEncodeProducerReplyStep::Transferred(_)){ready=true;break}}assert!(ready);let error=frame.reply().unwrap().as_ref().unwrap_err();assert_eq!(error.cause.message.as_ptr(),pointer);assert_eq!(error.cause.retained_progress(),override_progress);assert_ne!(error.progress,override_progress);
  let mut published=None;{let mut payout=PagedTextReplyPayout::new(&mut frame,turn,&mut published)?;let mut pay=Payment::default();pay.begin(control,payout.demand()?,1)?;pay.stage(control,|physical,action,effect|payout.stage(physical,action,effect).result)?;assert!(pay.post(control)?);assert!(payout.finish(control,turn,&mut pay.physical,&mut pay.proof,&mut pay.work)?)}let error=published.as_ref().unwrap().as_ref().unwrap_err();assert_eq!(error.cause.message.as_ptr(),pointer);assert_eq!(error.cause.retained_progress(),override_progress);assert_ne!(error.progress,override_progress);
  return_loan(&mut frame,control,turn)?;drop(frame);close_producer(&mut producer,control,turn)?;drop(producer);assert!(output.is_none()&&inline.is_none());
  let mut failure=None;let d=RetirementDemand{copy_bytes:std::mem::size_of_val(&published)+std::mem::size_of_val(&failure),depth:1,..Default::default()};let mut pay=Payment::default();pay.begin(control,d,1)?;pay.stage(control,|_,_,effect|{assert!(effect.performed(1)?);let Err(error)=published.take().expect("original published result")else{panic!("original child error changed")};failure=Some(error);Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))})?;assert!(pay.post(control)?);pay.finish(control)?;assert_eq!(failure.as_ref().unwrap().cause.message.as_ptr(),pointer);assert_eq!(failure.as_ref().unwrap().cause.retained_progress(),override_progress);retire_value(&mut failure,control,turn)?;retire_value(&mut overrun,control,turn)?;assert!(failure.is_none()&&overrun.is_none());Ok(())
 }));result.unwrap();let remaining=bank.finish();assert_eq!(heap.0,remaining.submitted.retained_capacity_bytes);assert_eq!(heap.1,remaining.submitted.released_bytes);assert!(remaining.work<stock.work);assert!(message_bytes>0);for _ in 0..10000{if original.terminal_is_empty(){break}original.close_once().unwrap();}assert!(original.terminal_is_empty());
}
