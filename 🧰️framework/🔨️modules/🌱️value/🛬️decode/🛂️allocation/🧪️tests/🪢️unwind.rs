//! 🪢️ Original allocation callback boundaries retain actual backing and recorded recipient verdicts.
use super::super::{NativeAllocationRequest as Request,NativeAllocationEvent as Event,NativeDecodeAllocationPhase as Phase};
use crate::eff;
use crate::native_decoding::NativeDecodeControl;
use crate::{ValueError,ValueRefusalKind};
use std::sync::{Mutex,atomic::{AtomicUsize,Ordering}};
#[derive(Default)]struct Receiver{pending:Mutex<Option<Request>>,submitted:AtomicUsize}
impl Receiver{fn accept(&self,event:Event,target:&str)->bool{let name=match event{Event::Admit(_)=>"admit",Event::Quote(request)=>{assert!(self.pending.lock().unwrap().replace(request).is_none());"quote"},Event::Post{request,actual}=>{assert_eq!(self.pending.lock().unwrap().take(),Some(request));self.submitted.fetch_add(actual,Ordering::Relaxed);"post"},Event::Cancel(request)=>{assert_eq!(self.pending.lock().unwrap().take(),Some(request));"cancel"}};name==target}}
struct Interrupting<'a>{receiver:&'a Receiver,target:&'a str}
impl crate::NativeAllocationAuthorityPort for Interrupting<'_>{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn receive<'reply,'turn,'source>(&mut self,_:&mut (),event:Event,reply:crate::NativeAllocationReply<'reply>)->Result<crate::NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{let hit=self.receiver.accept(event,self.target);let reply=reply.complete(Ok(()));if hit{std::hint::black_box(&reply);panic!("original recipient interrupted after recording its verdict")}Ok(reply)}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut (),_:Event,reply:crate::NativeAllocationReply<'reply>)->Result<crate::NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{Ok(reply.pending())}
}
fn birth(bytes:usize,output:&mut Option<Vec<u8>>,actual:&mut Option<usize>)->Result<(),ValueError>{let layout=std::alloc::Layout::array::<u8>(bytes).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"fixture layout"))?;let pointer=unsafe{std::alloc::alloc(layout)};if pointer.is_null(){*actual=Some(0);return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"fixture allocation"))}*output=Some(unsafe{Vec::from_raw_parts(pointer,0,bytes)});*actual=Some(bytes);Ok(())}
#[test]fn original_port_panic_retains_actual_verdict_and_backing(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🛫️encode/🛂️allocation/🧪️tests/🧫️fixtures/🪢️port-unwind/🔣️.json")).unwrap();let bytes=fixture["bytes"].as_u64().unwrap()as usize;
 for case in fixture["cases"].as_array().unwrap().iter().filter(|row|row["port"]=="forwarded"){
  let target=case["event"].as_str().unwrap();let forwarded=Receiver::default();let mut callback=|_|true;let mut authority=crate::NativeAllocationAuthority::new(Interrupting{receiver:&forwarded,target});
  let(state,output)=authority.with_send_loan(&mut (),|loan|{
   let mut control=NativeDecodeControl::new_forwarded(bytes,&mut callback,loan);let mut actual=None;let mut retained=None;let mut output=None;
   let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||control.allocate_effect(bytes,&mut actual,&mut retained,&mut |actual|{if target=="cancel"{Err(ValueError::literal(ValueRefusalKind::Canceled,"fixture allocation failed"))}else{birth(bytes,&mut output,actual)}})));assert!(result.is_err());
   let state=retained.as_mut().unwrap();assert_eq!(state.request().bytes,bytes);assert_eq!(state.outcome().request,Some(state.request()));assert_eq!(state.outcome().actual,case["actual"].as_u64().map(|v|v as usize));assert_eq!(state.phase(),if target=="quote"{Phase::QuoteStarted}else{Phase::SettlementStarted});assert!(crate::retirement::RetireOwned::retirement_birth_bytes(state).is_none());
   assert!(!control.settle_allocation_progress(state,actual).unwrap_or(false));assert!(control.recover_allocation_callback(state).unwrap());
   if state.phase()!=Phase::Settled{assert!(control.settle_allocation_progress(state,actual).unwrap())}
   let state=retained.take().unwrap();drop(control);(state,output)
  }).unwrap();
  assert!(authority.into_port().is_ok());assert!(forwarded.pending.lock().unwrap().is_none());assert_eq!(forwarded.submitted.load(Ordering::Relaxed),if target=="post"{bytes}else{0});
  eff::close(state);if let Some(output)=output{eff::close(output)}
 }
}
#[test]fn operation_unwind_settles_real_backing_through_original_port(){
 let receiver=Receiver::default();let mut callback=|_|true;let mut authority=crate::NativeAllocationAuthority::new(Interrupting{receiver:&receiver,target:""});
 let(state,output)=authority.with_send_loan(&mut (),|loan|{let mut control=NativeDecodeControl::new_forwarded(8,&mut callback,loan);let mut actual=None;let mut retained=None;let mut output=None;let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||control.allocate_effect(8,&mut actual,&mut retained,&mut |actual|{birth(8,&mut output,actual)?;panic!("operation interrupted after real allocation")})));assert!(result.is_err());let state=retained.as_mut().unwrap();assert_eq!(state.outcome().actual,Some(8));assert_eq!(state.phase(),Phase::QuoteAccepted);assert!(control.settle_allocation_progress(state,actual).unwrap());let state=retained.take().unwrap();drop(control);(state,output)}).unwrap();
 assert!(authority.into_port().is_ok());assert_eq!(receiver.submitted.load(Ordering::Relaxed),8);assert!(receiver.pending.lock().unwrap().is_none());eff::close(state);eff::close(output.unwrap());
}

#[test]
fn partial_actual_refusal_reports_the_committed_extent(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🛫️encode/🛂️allocation/🧪️tests/🧫️fixtures/🪢️port-unwind/🔣️.json")).unwrap();let request=fixture["partial"]["requestBytes"].as_u64().unwrap() as usize;let actual_bytes=fixture["partial"]["actualBytes"].as_u64().unwrap() as usize;
 let port=|event:Event|match event{Event::Post{..}=>Err(ValueError::literal(ValueRefusalKind::Canceled,"actual post refused")),_=>Ok(())};let mut authority=crate::NativeAllocationAuthority::new(eff::Port(port));let mut observer=|_|true;let mut output=None;
 let failure=authority.with_send_loan(&mut (),|loan|{let mut native=NativeDecodeControl::new_forwarded(request,&mut observer,loan);let mut actual=None;let mut progress=None;let outcome=native.allocate_effect(request,&mut actual,&mut progress,&mut |actual|birth(actual_bytes,&mut output,actual));assert_eq!(native.owned_bytes(),request);assert_eq!(outcome.actual,Some(actual_bytes));let failure=outcome.into_allocation_failure(0).unwrap();assert_eq!(failure.progress.admitted_bytes,request);assert_eq!(failure.progress.requested_bytes,Some(request));assert_eq!(failure.progress.actual_capacity_bytes,Some(actual_bytes));assert!(failure.forwarded_post.is_some());assert!(progress.is_none());drop(native);failure}).unwrap();
 assert!(authority.into_port().is_ok());eff::close(failure);eff::close(output.take().unwrap());
}

#[test]
fn forwarded_frame_retains_owned_output_and_pending_original_work(){
 use crate::native_decoding::{NativeForwardedDecodeFrame,NativeDecodeRetirementRecipient};
 let mut recipient=NativeDecodeRetirementRecipient::new();let mut receipt=None;let mut observer=|_|true;let mut allocation=crate::NativeAllocationAuthority::new(eff::Port(|_:Event|Ok(())));let mut work=crate::OriginalWorkAuthority::new(eff::Producer{maximum:u64::MAX});
 let output=Box::new(73u64);let pointer=std::ptr::from_ref(output.as_ref());
 let failure=allocation.with_send_loan(&mut (),|allocation_loan|work.with_loan(&mut (),|work_loan|{let mut frame=NativeForwardedDecodeFrame::begin(&mut receipt,128,&mut observer,allocation_loan,&mut recipient).unwrap();
  let answer=frame.scope(|control|control.with_work_recipient(work_loan,|control|{assert!(control.prepare_work(1)?);control.depth=1;Ok::<_,ValueError>(output)}));
  let failure=answer.unwrap_err();assert_eq!(std::ptr::from_ref(failure.output.as_ref().unwrap().as_ref()),pointer);assert!(failure.operation.is_none());assert!(failure.context.is_some());assert!(failure.receipt_refusal.is_none());
  drop(frame);failure}).unwrap()).unwrap();
 assert!(receipt.as_ref().unwrap().has_pending_work());
 allocation.with_send_loan(&mut (),|allocation_loan|work.with_loan(&mut (),|work_loan|{let mut frame=NativeForwardedDecodeFrame::begin(&mut receipt,128,&mut observer,allocation_loan,&mut recipient).unwrap();
  frame.scope(|control|control.with_work_recipient(work_loan,|control|control.cancel_work())).unwrap();drop(frame);}).unwrap()).unwrap();
 assert!(!receipt.as_ref().unwrap().has_pending_work());drop(failure);assert!(allocation.into_port().is_ok());assert!(work.into_recipient().is_ok());
}

#[test]
fn forwarded_frame_unwind_returns_the_original_admission_receipt(){
 use crate::native_decoding::{NativeForwardedDecodeFrame,NativeDecodeRetirementRecipient};
 let receiver=Receiver::default();let mut authority=crate::NativeAllocationAuthority::new(Interrupting{receiver:&receiver,target:""});
 let mut observer=|_|true;let mut recipient=NativeDecodeRetirementRecipient::new();let mut receipt=None;
 let mut output=None;let mut actual=None;let mut allocation=None;
 let outcome=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
  authority.with_send_loan(&mut (),|loan|{let mut frame=NativeForwardedDecodeFrame::begin(&mut receipt,8,&mut observer,loan,&mut recipient).unwrap();
   let _:Result<(),crate::native_decoding::NativeCallFailure<(),ValueError>>=frame.scope(|control|{
    let outcome=control.allocate_effect(8,&mut actual,&mut allocation,&mut |actual|birth(8,&mut output,actual));
    assert!(outcome.primary.is_none());panic!("original allocated output remains in its caller slot");
   });}).unwrap();
 }));
 assert!(outcome.is_err());assert_eq!(receipt.as_ref().unwrap().owned_bytes(),8);assert!(receiver.pending.lock().unwrap().is_none());assert_eq!(receiver.submitted.load(Ordering::Relaxed),8);assert_eq!(actual,Some(8));assert!(authority.into_port().is_ok());eff::close(output.take().unwrap());
}
