//! 🧪️ Recorded original quote and cancel replies survive unwind without a physical post.
use super::*;
use semio_framework_value_control_contract::{NativeAllocationAuthority,NativeAllocationAuthorityPort,NativeAllocationEvent,NativeAllocationReply,NativeAllocationReplyComplete,OriginalSourceTurnIdentity};
use std::sync::{Arc,atomic::{AtomicUsize,Ordering}};
struct Producer{quotes:Arc<AtomicUsize>,cancels:Arc<AtomicUsize>,recoveries:Arc<AtomicUsize>}
impl NativeAllocationAuthorityPort for Producer{
 type Context<'turn,'source>=()where 'source:'turn;
 fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{None}
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
 fn receive<'reply,'turn,'source>(&mut self,_:&mut (),event:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError>where 'source:'turn{match event{NativeAllocationEvent::Quote(_)=>{self.quotes.fetch_add(1,Ordering::Relaxed);},NativeAllocationEvent::Cancel(_)=>{self.cancels.fetch_add(1,Ordering::Relaxed);},_=>panic!("canceled original allocation cannot post a physical effect")}let completed=reply.complete(Ok(()));std::hint::black_box(&completed);panic!("original allocation reply recorded")}
 fn recover<'reply,'turn,'source>(&mut self,_:&mut (),_:NativeAllocationEvent,reply:NativeAllocationReply<'reply>)->Result<NativeAllocationReplyComplete<'reply>,ValueError>where 'source:'turn{self.recoveries.fetch_add(1,Ordering::Relaxed);Ok(reply.pending())}
}
#[test]
fn recorded_original_quote_and_cancel_recover_without_replaying_the_producer(){
 let law:serde_json::Value=serde_json::from_str(include_str!("reply.json")).unwrap();let quotes=Arc::new(AtomicUsize::new(0));let cancels=Arc::new(AtomicUsize::new(0));let recoveries=Arc::new(AtomicUsize::new(0));let mut authority=NativeAllocationAuthority::new(Producer{quotes:quotes.clone(),cancels:cancels.clone(),recoveries:recoveries.clone()});let mut progress=None;let mut ledger=Some(semio_framework_value_receiving::NativeAllocationLedger::new(0));let request=NativeAllocationRequest{bytes:7,owned_bytes:0,next_owned_bytes:7,maximum_bytes:7};
 let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||authority.with_send_loan(&mut (),|loan|NativeDecodeAllocationProgress::begin(request,&mut NativeDecodeAllocationBinding::Forwarded(loan),&mut progress))));assert!(interrupted.is_err());assert_eq!(progress.as_ref().unwrap().phase(),NativeDecodeAllocationPhase::QuoteStarted);
 authority.with_send_loan(&mut (),|loan|assert!(progress.as_mut().unwrap().recover(&mut NativeDecodeAllocationBinding::Forwarded(loan)).unwrap())).unwrap();
 let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||authority.with_send_loan(&mut (),|loan|progress.as_mut().unwrap().settle(&mut ledger,&mut NativeDecodeAllocationBinding::Forwarded(loan),None))));assert!(interrupted.is_err());assert_eq!(progress.as_ref().unwrap().phase(),NativeDecodeAllocationPhase::SettlementStarted);assert_eq!(ledger.as_ref().unwrap().owned_bytes(),law["admittedBytes"].as_u64().unwrap()as usize);
 authority.with_send_loan(&mut (),|loan|assert!(progress.as_mut().unwrap().recover(&mut NativeDecodeAllocationBinding::Forwarded(loan)).unwrap())).unwrap();
 let outcome=progress.take().unwrap().into_outcome().ok().expect("same original allocation fully settled");assert!(!outcome.admitted);assert_eq!(outcome.actual,None);assert_eq!(quotes.load(Ordering::Relaxed),law["quotes"].as_u64().unwrap()as usize);assert_eq!(cancels.load(Ordering::Relaxed),law["cancels"].as_u64().unwrap()as usize);assert_eq!(recoveries.load(Ordering::Relaxed),law["producerRecoveries"].as_u64().unwrap()as usize);assert!(authority.into_port().is_ok());
}
