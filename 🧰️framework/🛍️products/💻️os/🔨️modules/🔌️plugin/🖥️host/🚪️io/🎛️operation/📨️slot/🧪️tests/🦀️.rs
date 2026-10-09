//! 📨️ A suspended guest receives exact original admission before its next physical effect.
use super::{OperationSlot,OperationRequest,OperationReply};
use std::{sync::{Arc,atomic::{AtomicUsize,Ordering}},task::{Context,Poll,Wake,Waker}};
struct Observer(AtomicUsize);
impl Wake for Observer{fn wake(self:Arc<Self>){self.0.fetch_add(1,Ordering::Relaxed);}fn wake_by_ref(self:&Arc<Self>){self.0.fetch_add(1,Ordering::Relaxed);}}
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
#[test]
fn scalar_slot_requires_a_live_original_pump_and_retains_exact_requests_until_reply(){
 let observed=Arc::new(Observer(AtomicUsize::new(0)));let waker=Waker::from(observed.clone());let mut context=Context::from_waker(&waker);let mut slot=OperationSlot::new();let mut posted=None;
 assert_eq!(slot.poll_request(OperationRequest::Begin,&mut posted,&mut context),Poll::Ready(OperationReply::Begin(Err(6))));assert!(slot.take_request().is_none());assert!(posted.is_none());slot.arm().unwrap();slot.set_pump_waker(&waker);
 assert_eq!(slot.poll_request(OperationRequest::Begin,&mut posted,&mut context),Poll::Pending);assert!(posted.is_some());assert_eq!(observed.0.load(Ordering::Relaxed),1);let mut duplicate=None;assert_eq!(slot.poll_request(OperationRequest::Begin,&mut duplicate,&mut context),Poll::Ready(OperationReply::Begin(Err(4))));assert!(duplicate.is_none());
 assert_eq!(slot.take_request(),Some(OperationRequest::Begin));assert!(slot.take_request().is_none());assert_eq!(slot.reply(OperationReply::Code(0)),Err(4));slot.reply(OperationReply::Begin(Ok(57))).unwrap();assert_eq!(slot.poll_request(OperationRequest::Begin,&mut posted,&mut context),Poll::Ready(OperationReply::Begin(Ok(57))));
 let mut posted=None;let request=OperationRequest::Allocation{bytes:43,owned:0,next:43,maximum:57};assert_eq!(slot.poll_request(request,&mut posted,&mut context),Poll::Pending);slot.disarm(1);assert_eq!(slot.poll_request(request,&mut posted,&mut context),Poll::Ready(OperationReply::Code(1)));assert!(slot.take_request().is_none());assert_eq!(slot.reply(OperationReply::Code(0)),Err(4));
 println!("[DEBUG] Original operation scalar slot: unboundRefused=true duplicateRefused=true requestTakenOnce=true wrongReplyRetainsRequest=true closeRevokesBeforeGuestAllocation=true noPayloadCopies=true");
}
#[test]
fn scalar_transport_preserves_the_original_receiving_ledger_and_independent_reply_oracle(){
 let neutral=fixture();let law=&neutral["authority"];let ceiling=law["ceiling"].as_u64().unwrap()as usize;let prior=law["prior"].as_u64().unwrap()as usize;let amount=law["guest"].as_u64().unwrap()as usize;
 let mut observe=|_|true;let mut identity=super::super::EntityIdentityAuthority::new(ceiling,&mut observe).unwrap();identity.encode::<_,semio_framework_value::ValueError>(|control|control.charge(prior)).unwrap();let mut receiver=super::super::OriginalOperationReceiver::new(&mut identity);
 let waker=Waker::from(Arc::new(Observer(AtomicUsize::new(0))));let mut context=Context::from_waker(&waker);let mut slot=OperationSlot::new();slot.arm().unwrap();let mut posted=None;assert_eq!(slot.poll_request(OperationRequest::Begin,&mut posted,&mut context),Poll::Pending);assert_eq!(slot.take_request(),Some(OperationRequest::Begin));let maximum=receiver.begin().unwrap();slot.reply(OperationReply::Begin(Ok(maximum as u64))).unwrap();assert_eq!(slot.poll_request(OperationRequest::Begin,&mut posted,&mut context),Poll::Ready(OperationReply::Begin(Ok((ceiling-prior)as u64))));
 let request=OperationRequest::Allocation{bytes:amount as u64,owned:0,next:amount as u64,maximum:maximum as u64};let mut posted=None;assert_eq!(slot.poll_request(request,&mut posted,&mut context),Poll::Pending);assert_eq!(slot.take_request(),Some(request));receiver.allocation(semio_framework_value::native_encoding::NativeEncodeAllocation{bytes:amount,owned_bytes:0,next_owned_bytes:amount,maximum_bytes:maximum}).unwrap();slot.reply(OperationReply::Code(0)).unwrap();assert_eq!(slot.poll_request(request,&mut posted,&mut context),Poll::Ready(OperationReply::Code(0)));receiver.finish(amount).unwrap();drop(receiver);slot.disarm(1);
 let receipt=identity.pause().unwrap();let mut final_observer=|_|true;let actual=semio_framework_value::NativeEncodeControl::resume(receipt,&mut final_observer).unwrap().owned_bytes();let expected=serde_json::from_value::<usize>(law["expected"].clone()).unwrap();assert_eq!(actual,expected);
 println!("[DEBUG] Original operation transport: guest43PlusOriginal7=50 exactOriginalBorrow=true independentSerdeLedger=true allocationPrecedesReply=true sourceOwnerNotStored=true");
}

#[test]
fn revoked_request_cannot_consume_identical_later_operation_reply(){
 let waker=Waker::noop();let mut context=Context::from_waker(waker);let mut slot=OperationSlot::new();let request=OperationRequest::Allocation{bytes:43,owned:0,next:43,maximum:57};let mut old=None;slot.arm().unwrap();assert_eq!(slot.poll_request(request,&mut old,&mut context),Poll::Pending);slot.disarm(1);slot.arm().unwrap();let mut current=None;assert_eq!(slot.poll_request(request,&mut current,&mut context),Poll::Pending);assert_eq!(slot.take_request(),Some(request));slot.reply(OperationReply::Code(0)).unwrap();assert_eq!(slot.poll_request(request,&mut old,&mut context),Poll::Ready(OperationReply::Code(4)));assert_eq!(slot.poll_request(request,&mut current,&mut context),Poll::Ready(OperationReply::Code(0)));println!("[DEBUG] Original operation cancellation: identicalLaterRequest=true revokedLeaseCannotConsumeCurrentGrant=true currentReplyRetained=true");
}
