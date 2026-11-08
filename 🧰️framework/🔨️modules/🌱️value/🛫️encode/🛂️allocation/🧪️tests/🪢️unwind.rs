//! 🪢️ Original allocation callback boundaries retain actual backing and recorded recipient verdicts.
use super::super::{NativeAllocationRequest as Request,NativeAllocationEvent as Event,NativeEncodeAllocationPhase as Phase};
use crate::eff;
use crate::native_encoding::NativeEncodeControl;
use crate::{ValueError,ValueRefusalKind};
use std::sync::{Mutex,atomic::{AtomicUsize,Ordering}};
#[derive(Default)]struct Receiver{pending:Mutex<Option<Request>>,submitted:AtomicUsize}
impl Receiver{fn accept(&self,event:Event,target:&str)->bool{let name=match event{Event::Quote(request)=>{assert!(self.pending.lock().unwrap().replace(request).is_none());"quote"},Event::Post{request,actual}=>{assert_eq!(self.pending.lock().unwrap().take(),Some(request));self.submitted.fetch_add(actual,Ordering::Relaxed);"post"},Event::Cancel(request)=>{assert_eq!(self.pending.lock().unwrap().take(),Some(request));"cancel"},Event::Admit(_)=>panic!("unexpected budget admission")};name==target}}
struct Interrupting<'a>{receiver:&'a Receiver,target:&'a str}
impl crate::NativeAllocationAuthorityPort for Interrupting<'_>{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&())->Option<crate::OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn receive<'reply,'turn,'source>(&mut self,_:&mut (),event:Event,reply:crate::NativeAllocationReply<'reply>)->Result<crate::NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{let hit=self.receiver.accept(event,self.target);let reply=reply.complete(Ok(()));if hit{std::hint::black_box(&reply);panic!("original recipient interrupted after recording its verdict")}Ok(reply)}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut (),_:Event,reply:crate::NativeAllocationReply<'reply>)->Result<crate::NativeAllocationReplyComplete<'reply>,ValueError> where 'source:'turn{Ok(reply.pending())}
}
fn birth(bytes:usize,output:&mut Option<Vec<u8>>,actual:&mut Option<usize>)->Result<bool,ValueError>{let layout=std::alloc::Layout::array::<u8>(bytes).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"fixture layout"))?;let pointer=unsafe{std::alloc::alloc(layout)};if pointer.is_null(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"fixture allocation"))}*output=Some(unsafe{Vec::from_raw_parts(pointer,0,bytes)});*actual=Some(bytes);Ok(true)}
#[test]fn original_port_panic_retains_actual_verdict_and_backing(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🪢️port-unwind/🔣️.json")).unwrap();let bytes=fixture["bytes"].as_u64().unwrap()as usize;
 for case in fixture["cases"].as_array().unwrap().iter(){
  let selected=case["port"].as_str().unwrap();let target=case["event"].as_str().unwrap();let forwarded=Receiver::default();let scoped=Receiver::default();let mut callback=|_|true;
  let mut forwarded_authority=crate::NativeAllocationAuthority::new(Interrupting{receiver:&forwarded,target:if selected=="forwarded"{target}else{""}});let mut scoped_authority=crate::NativeAllocationAuthority::new(Interrupting{receiver:&scoped,target:if selected=="scoped"{target}else{""}});
  let(state,output)=forwarded_authority.with_send_loan(&mut (),|forwarded_loan|scoped_authority.with_send_loan(&mut (),|scoped_loan|{
   let mut control=NativeEncodeControl::new_forwarded(bytes,&mut callback,forwarded_loan);control.scoped_allocation=Some(scoped_loan);let mut actual=None;let mut retained=None;let mut output=None;
   let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||control.allocate_effect(bytes,&mut actual,&mut retained,&mut |actual,_|{if target=="cancel"{Err(ValueError::literal(ValueRefusalKind::Canceled,"fixture allocation failed"))}else{birth(bytes,&mut output,actual)}})));assert!(result.is_err());
   let state=retained.as_mut().unwrap();assert_eq!(state.request().bytes,bytes);assert_eq!(state.outcome().request,Some(state.request()));assert_eq!(state.outcome().actual,case["actual"].as_u64().map(|v|v as usize));assert_eq!(if selected=="forwarded"{state.forwarded_phase()}else{state.scoped_phase()},if target=="quote"{Phase::QuoteStarted}else{Phase::SettlementStarted});assert!(crate::retirement::RetireOwned::retirement_birth_bytes(state).is_none());
   if target=="quote"{assert!(control.settle_allocation_progress(state,actual).is_err());assert!(control.recover_allocation_quotes(state).unwrap())}
   assert!(control.settle_allocation_progress(state,actual).unwrap());
   let state=retained.take().unwrap();control.scoped_allocation=None;drop(control);(state,output)
  }).unwrap()).unwrap();
  assert!(forwarded_authority.into_port().is_ok());assert!(scoped_authority.into_port().is_ok());assert!(forwarded.pending.lock().unwrap().is_none());assert!(scoped.pending.lock().unwrap().is_none());if target=="post"{assert_eq!(forwarded.submitted.load(Ordering::Relaxed),bytes);assert_eq!(scoped.submitted.load(Ordering::Relaxed),bytes)}
  eff::close(state);if let Some(output)=output{eff::close(output)}
 }
}
#[test]fn operation_unwind_settles_real_backing_through_original_port(){
 let receiver=Receiver::default();let mut callback=|_|true;let mut authority=crate::NativeAllocationAuthority::new(Interrupting{receiver:&receiver,target:""});
 let(state,output)=authority.with_send_loan(&mut (),|allocation_loan|{let mut control=NativeEncodeControl::new_forwarded(8,&mut callback,allocation_loan);let mut actual=None;let mut retained=None;let mut output=None;let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||control.allocate_effect(8,&mut actual,&mut retained,&mut |actual,_|{birth(8,&mut output,actual)?;panic!("operation interrupted after real allocation")})));assert!(result.is_err());let state=retained.as_mut().unwrap();assert_eq!(state.outcome().actual,Some(8));assert_eq!(state.forwarded_phase(),Phase::QuoteAccepted);assert!(control.settle_allocation_progress(state,actual).unwrap());let state=retained.take().unwrap();drop(control);(state,output)}).unwrap();
 assert!(authority.into_port().is_ok());assert_eq!(receiver.submitted.load(Ordering::Relaxed),8);assert!(receiver.pending.lock().unwrap().is_none());eff::close(state);eff::close(output.unwrap());
}

#[test]
fn partial_actual_refusal_reports_the_committed_extent(){
 let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../🛂️allocation/🧪️tests/🧫️fixtures/🪢️port-unwind/🔣️.json")).unwrap();let request=fixture["partial"]["requestBytes"].as_u64().unwrap() as usize;let actual_bytes=fixture["partial"]["actualBytes"].as_u64().unwrap() as usize;
 let port=|event:Event|match event{Event::Post{..}=>Err(ValueError::literal(ValueRefusalKind::Canceled,"actual post refused")),_=>Ok(())};let mut authority=crate::NativeAllocationAuthority::new(eff::Port(port));let mut observer=|_|true;let mut output=None;
 let failure=authority.with_send_loan(&mut (),|loan|{let mut native=NativeEncodeControl::new_forwarded(request,&mut observer,loan);let mut actual=None;let mut progress=None;let outcome=native.allocate_effect(request,&mut actual,&mut progress,&mut |actual,_|birth(actual_bytes,&mut output,actual)).unwrap().expect("original allocation settled");assert_eq!(native.owned_bytes(),request);assert_eq!(outcome.actual,Some(actual_bytes));let failure=outcome.into_allocation_failure(0).unwrap();assert_eq!(failure.progress.admitted_bytes,request);assert_eq!(failure.progress.requested_bytes,Some(request));assert_eq!(failure.progress.actual_capacity_bytes,Some(actual_bytes));assert!(failure.forwarded_post.is_some());assert!(progress.is_none());drop(native);failure}).unwrap();
 assert!(authority.into_port().is_ok());eff::close(failure);eff::close(output.take().unwrap());
}
