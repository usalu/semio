use crate::{ErasedSnapshotRetirement,NativeDecodeControl,NativeDecodeProgress,NativeDecodeRetirementRecipient,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::{controlled::ControlledRetirement,RetirementCursor}};
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn grant(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)}}
fn observe<T>(operation:impl FnOnce()->T)->(T,(usize,usize)){crate::observe_retirement_allocations(operation)}

struct RetirementFuel{maximum:u64}
impl crate::native_decoding::OriginalWorkRecipient for RetirementFuel{
 type Context<'turn,'source>=() where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn admits<'reply,'turn,'source>(&mut self,_:&mut (),quote:crate::native_decoding::OriginalWorkQuote<'reply>,observed:u64,additional:u64)->crate::native_decoding::OriginalWorkQuoteComplete<'reply> where 'source:'turn{quote.complete(Ok(observed.checked_add(additional).is_some_and(|sum|sum<=self.maximum)))}
}
fn operation_refusal<T>(failure:crate::native_decoding::NativeCallFailure<T,ValueError>)->ValueError{let crate::native_decoding::NativeCallFailure{output,operation,context,receipt_refusal}=failure;assert!(output.is_none());assert!(context.is_none());assert!(receipt_refusal.is_none());operation.expect("original operation refusal retained")}

/// 🧾️ The actual original receipt and recipient survive cancelled work while retirement receives only its paid hop allowance.
#[test]
fn native_retirement_loan_preserves_original_cancelled_ledger_and_grant(){
 let mut authority=crate::native_decoding::OriginalWorkAuthority::new(RetirementFuel{maximum:1_000_000});let mut source=();authority.with_loan(&mut source,|work|{
 let law=fixture();let mut accepted=|_|true;let mut recipient=NativeDecodeRetirementRecipient::new();let mut control=NativeDecodeControl::new(128,&mut accepted);control.install_retirement_recipient(&mut recipient).unwrap();control.charge(law["originalBytes"].as_u64().unwrap()as usize).unwrap();let ledger=control.ledger_identity();let recipient_id=control.retirement_recipient_identity();let mut owner=control.pause_with_retirement().unwrap();let calls=std::cell::Cell::new(0usize);let mut cancelled=|_:NativeDecodeProgress|{calls.set(calls.get()+1);false};assert_eq!(operation_refusal(owner.decode(&mut cancelled,|control|control.checkpoint()).unwrap_err()).kind,ValueRefusalKind::Canceled);let capacity=law["charges"].as_array().unwrap().iter().map(|n|n.as_u64().unwrap()as usize).sum();let g=grant(RetirementDemand{capacity_bytes:capacity,depth:1,..Default::default()});let before=owner.owned_bytes();let mut visits=0;let denied=RetainedCloneGrant{maximum_items:0,..g};assert!(owner.retire(denied,&mut cancelled,work,|_|{visits+=1;Ok(())}).is_err());assert_eq!(visits,0);assert_eq!(owner.owned_bytes(),before);
 owner.retire(g,&mut cancelled,work,|loan|{assert_eq!(loan.ledger_identity(),ledger);assert_eq!(loan.retirement_recipient_identity(),recipient_id);{let close:&mut dyn crate::native_decoding::NativeDecodeCloseControl=loan;assert_eq!(close.ledger_identity(),ledger);assert_eq!(close.retirement_recipient_identity(),recipient_id);for n in law["charges"].as_array().unwrap(){close.charge(n.as_u64().unwrap()as usize)?;}}let before=calls.get();assert_eq!(loan.charge(1).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(calls.get(),before);assert_eq!(loan.owned_bytes(),law["expectedBytes"].as_u64().unwrap()as usize);Ok(())}).unwrap();assert_eq!(owner.owned_bytes(),35);assert_eq!(operation_refusal(owner.decode(&mut cancelled,|control|control.checkpoint()).unwrap_err()).kind,ValueRefusalKind::Canceled);let(detached,_)=owner.detach();assert_eq!(detached.owned_bytes(),35);assert!(crate::ErasedSnapshotRetirement::terminal_is_empty(&recipient));
 }).unwrap();
}

/// 🧳️ Real returned payloads close through their exact original recipient after cancellation at every authored frontier.
#[test]
fn native_retirement_loan_closes_original_returned_payload_after_cancellation(){
 use crate::native_decoding::{OriginalConstructionAuthority,OriginalConstructionRecipient,OriginalConstructionGrant,OriginalConstructionGrantComplete,OriginalConstructionAccept,OriginalConstructionAcceptComplete,OriginalWorkAuthority,NativeReceivingEffect,NativeReceivingEffectPoll};
 struct Physical{source:std::sync::Arc<u8>,submitted:RetainedCloneProgress}
 impl OriginalConstructionRecipient for Physical{
 type Context<'turn,'source>=&'turn u8 where 'source:'turn;
 fn validate_context<'turn,'source>(&self,cx:&&'turn u8)->Result<(),ValueError>where 'source:'turn{if std::ptr::eq(self.source.as_ref(),*cx){Ok(())}else{Err(ValueError::invariant("original cleanup source replaced"))}}
 fn grant<'a,'turn,'source>(&mut self,_:&mut &'turn u8,q:OriginalConstructionGrant<'a>)->OriginalConstructionGrantComplete<'a>where 'source:'turn{let demand=q.demand();q.complete(Ok(grant(demand)))}
 fn accept<'a,'turn,'source>(&mut self,_:&mut &'turn u8,r:OriginalConstructionAccept<'a>)->OriginalConstructionAcceptComplete<'a>where 'source:'turn{let posted=self.submitted.checked_add(r.progress()).map(|p|self.submitted=p);r.complete(posted)}
 fn submitted<'turn,'source>(&self,_:&&'turn u8)->RetainedCloneProgress where 'source:'turn{self.submitted}
 }
 let law=fixture();for length in law["payloadBytes"].as_array().unwrap(){let length=length.as_u64().unwrap()as usize;for cut in law["cuts"].as_array().unwrap(){let cut=cut.as_u64().unwrap()as usize;
 let mut work=OriginalWorkAuthority::new(RetirementFuel{maximum:1_000_000});work.with_loan(&mut (),|work|{
 let text="x".repeat(length);let mut accepted=|_|true;let mut recipient=NativeDecodeRetirementRecipient::new();let mut control=NativeDecodeControl::new(1<<30,&mut accepted);control.install_retirement_recipient(&mut recipient).unwrap();let identity=control.retirement_recipient_identity();let ledger=control.ledger_identity();let wrapper=std::mem::size_of::<ControlledRetirement<String>>();
 let(error,mut total)=observe(||control.with_retirement_owner::<(),ValueError>(wrapper,|control|{let owned=control.copy_text(&text).unwrap();let owner=Box::new(ControlledRetirement::new(owned).unwrap());(Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original payload retained for funded closure")),Some(owner))}).unwrap_err());assert_eq!(error.kind,ValueRefusalKind::InvalidValue);assert_eq!(control.owned_bytes(),total.0);
 let mut owner=control.pause_with_retirement().unwrap();let mut canceled=|_|false;let source=std::sync::Arc::new(7);let mut physical=OriginalConstructionAuthority::new(Physical{source:source.clone(),submitted:Default::default()});let mut context=source.as_ref();let mut frame=NativeReceivingEffect::<semio_framework_value_retirement::OriginalRetirementFailure>::default();let mut turns=0;
 loop{
 let state=owner.retire(grant(RetirementDemand{depth:1,..Default::default()}),&mut canceled,work,|loan|Ok::<_,ValueError>((loan.has_retirement_owner(),loan.requires_original_birth(),loan.retirement_demands(4096)?))).unwrap();if !state.0{break}let d=state.2;assert!(d.copy_bytes<=4096);let g=grant(d);
 if state.1{let(turn,heap)=observe(||owner.retire(g,&mut canceled,work,|loan|Ok::<_,ValueError>(loan.close_original_birth_step(g))).unwrap());let p=turn.result.unwrap().progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));total.0+=heap.0;total.1+=heap.1;
 }else{
 let units=owner.retire(g,&mut canceled,work,|loan|loan.retirement_work_demand(4096)).unwrap();
 let(poll,heap)=observe(||{owner.decode(&mut canceled,|native|{native.with_work_recipient(work,|native|{let turn=physical.with_loan(&mut context,|physical|{native.with_construction_recipient(physical,|native|{native.with_retirement(g,|loan|{loan.receive_effect(d,units,&mut frame,|loan,effect|{
 assert_eq!(loan.ledger_identity(),ledger);assert_eq!(loan.retirement_recipient_identity(),identity);
 for axis in 0..5{let mut denied=g;match axis{0=>denied.maximum_items=0,1 if d.copy_bytes>0=>denied.maximum_copy_bytes=d.copy_bytes-1,2 if d.capacity_bytes>0=>denied.maximum_capacity_bytes=d.capacity_bytes-1,3 if d.release_bytes>0=>denied.maximum_release_bytes=d.release_bytes-1,4 if d.depth>0=>denied.maximum_depth=d.depth-1,_=>continue}
 let(turn,heap)=observe(||loan.close_retirement_recipient(&mut effect.reborrow(denied,units).expect("denial narrows the original effect")));assert_eq!(turn.work,0);assert_eq!(turn.result.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));}
 loan.close_retirement_recipient(effect)
 })})})})?;turn})}).unwrap()});let NativeReceivingEffectPoll::Complete(turn)=poll else{panic!("admitted original cleanup remained pending")};let p=turn.result.unwrap().progress();assert!(p.fits(g));assert!(p==Default::default()||turn.work>0);assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));total.0+=heap.0;total.1+=heap.1;
 }turns+=1;if turns==cut{assert_eq!(operation_refusal(owner.decode(&mut canceled,|control|control.checkpoint()).unwrap_err()).kind,ValueRefusalKind::Canceled)}assert!(turns<10000);
 }
 assert_eq!(owner.owned_bytes(),total.0);let(detached,_)=owner.detach();assert_eq!(detached.owned_bytes(),total.0);assert!(crate::ErasedSnapshotRetirement::terminal_is_empty(&recipient));assert_eq!(total.0,total.1);assert_eq!(observe(||drop(recipient)).1,(0,0));assert!(physical.into_recipient().is_ok());
 }).unwrap();assert!(work.into_recipient().is_ok());
 }}
}

/// 🧷️ Neutral erasure retains the cancelled decoder's actual original ledger and recipient, including rejected capacity.
#[test]
fn neutral_original_close_loan_forwards_actual_cancelled_native_authority(){
 let mut authority=crate::native_decoding::OriginalWorkAuthority::new(RetirementFuel{maximum:1_000_000});let mut source=();authority.with_loan(&mut source,|work|{
 let mut accepted=|_|true;let mut recipient=NativeDecodeRetirementRecipient::new();let mut control=NativeDecodeControl::new(128,&mut accepted);control.install_retirement_recipient(&mut recipient).unwrap();control.charge(32).unwrap();let ledger=control.ledger_identity();let identity=control.retirement_recipient_identity();let mut owned=control.pause_with_retirement().unwrap();let mut cancelled=|_|false;assert_eq!(operation_refusal(owned.decode(&mut cancelled,|c|c.checkpoint()).unwrap_err()).kind,ValueRefusalKind::Canceled);let g=grant(RetirementDemand{capacity_bytes:3,depth:1,..Default::default()});owned.retire(g,&mut cancelled,work,|retirement|retirement.with_original_close(|original|{assert_eq!(original.ledger_identity(),ledger);assert_eq!(original.retirement_recipient_identity(),identity);original.charge(1)?;original.charge(2)?;assert_eq!(original.charge(1).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);Ok(())})).unwrap();assert_eq!(owned.owned_bytes(),35);let(actual,_)=owned.detach();assert_eq!(actual.owned_bytes(),35);assert!(crate::ErasedSnapshotRetirement::terminal_is_empty(&recipient));
 }).unwrap();
}

/// 🪢️ Encoding loans spend the original retirement hop through success, retained refusal and unwind.
#[test]
fn native_retirement_encoding_receiver_keeps_one_original_ledger_and_hop(){
 let mut initial=|_|true;let mut recipient=NativeDecodeRetirementRecipient::new();let mut original=NativeDecodeControl::new(128,&mut initial);original.install_retirement_recipient(&mut recipient).unwrap();original.charge(32).unwrap();let ledger=original.ledger_identity();let identity=original.retirement_recipient_identity();let mut owner=original.pause_with_retirement().unwrap();
 let calls=std::cell::Cell::new(0);let mut permitted=|_:NativeDecodeProgress|{calls.set(calls.get()+1);true};
 let mut partial=None;let mut unwound=None;let mut outputs=Vec::new();
 owner.retire(grant(RetirementDemand{capacity_bytes:8,depth:1,..Default::default()}),&mut permitted,|loan|{
  assert_eq!(loan.ledger_identity(),ledger);assert_eq!(loan.retirement_recipient_identity(),identity);loan.charge(2)?;
  let(result,heap)=observe(||loan.with_encoding_receiver(|maximum,observer,allocation|{
   assert_eq!(maximum,6);let mut encoder=crate::native_encoding::NativeEncodeControl::new_forwarded(maximum,observer,allocation);encoder.copy_text("abc")
  }));assert_eq!(heap,(3,0));outputs.push(result?);assert_eq!(loan.spent,5);
  let(result,heap)=observe(||loan.with_encoding_receiver::<(),ValueError>(|maximum,observer,allocation|{
   assert_eq!(maximum,3);let mut encoder=crate::native_encoding::NativeEncodeControl::new_forwarded(maximum,observer,allocation);partial=Some(encoder.copy_text("xy")?);Err(ValueError::literal(ValueRefusalKind::InvalidValue,"retained encoding refusal"))
  }));assert_eq!(heap,(2,0));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::InvalidValue);assert_eq!(loan.spent,7);
  let panic=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
   let _=loan.with_encoding_receiver::<(),ValueError>(|maximum,observer,allocation|{
    assert_eq!(maximum,1);let mut encoder=crate::native_encoding::NativeEncodeControl::new_forwarded(maximum,observer,allocation);unwound=Some(encoder.copy_text("z")?);panic!("retirement encoding unwind");
   });
  }));assert!(panic.is_err());assert_eq!(loan.spent,8);assert_eq!(loan.owned_bytes(),40);
  let before=calls.get();assert_eq!(loan.charge(1).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(calls.get(),before);
  let(result,heap)=observe(||loan.with_encoding_receiver(|maximum,observer,allocation|{
   assert_eq!(maximum,0);let mut encoder=crate::native_encoding::NativeEncodeControl::new_forwarded(maximum,observer,allocation);encoder.copy_text("denied")
  }));assert_eq!(heap,(0,0));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(loan.spent,8);Ok(())
 }).unwrap();
 assert!(calls.get()>0);assert_eq!(owner.owned_bytes(),40);
 let mut refused=|_:NativeDecodeProgress|false;let before=owner.owned_bytes();let(result,heap)=observe(||owner.retire(grant(RetirementDemand{capacity_bytes:4,depth:1,..Default::default()}),&mut refused,|loan|loan.with_encoding_receiver(|maximum,observer,allocation|{
  let mut encoder=crate::native_encoding::NativeEncodeControl::new_forwarded(maximum,observer,allocation);encoder.copy_text("no")
 })));assert_eq!(heap,(0,0));assert_eq!(result.unwrap_err().kind,ValueRefusalKind::Canceled);assert_eq!(owner.owned_bytes(),before);
 let(detached,_)=owner.detach();assert_eq!(detached.owned_bytes(),40);assert!(crate::ErasedSnapshotRetirement::terminal_is_empty(&recipient));
 outputs.push(partial.take().unwrap());outputs.push(unwound.take().unwrap());assert_eq!(outputs.iter().map(String::as_str).collect::<Vec<_>>(),["abc","xy","z"]);
 for output in outputs{let mut closing=ControlledRetirement::retain(output);for _ in 0..64{if closing.terminal_is_empty(){break}let d=closing.next_demand(4096).unwrap();let(step,heap)=observe(||closing.step(grant(d)).unwrap());let p=step.progress();assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));}assert!(closing.terminal_is_empty());}
}

/// 🧾️ The real original allocation port observes quote and actual post before spare-node cleanup.
#[test]
fn original_failure_storage_posts_actual_birth_to_the_same_forwarded_port(){
 use crate::native_decoding::{allocation::NativeAllocationEvent,NativeDecodeWorkRecipient};
 struct Work{allow:bool}
 impl NativeDecodeWorkRecipient for Work{fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{if self.allow{1_000_000u64.checked_sub(observed).ok_or_else(||ValueError::invariant("observed test work exceeds its finite scope"))}else{Ok(0)}}fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(self.allow&&observed.checked_add(additional).is_some_and(|total|total<=1_000_000))}}
 let f:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/failure-events.json")).unwrap();let bytes=crate::OriginalFailureStorage::birth_bytes();
 for mode in ["accepted","postRefused","quoteRefused","workDenied","allocationFailed"]{
  let mut events=Vec::with_capacity(2);let mut port=|event:NativeAllocationEvent|{events.push(event);match event{NativeAllocationEvent::Quote(_)if mode=="quoteRefused"=>Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"fixture quote refused")),NativeAllocationEvent::Post{..}if mode=="postRefused"=>Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"fixture post refused")),_=>Ok(())}};let mut progress=|_|true;let mut recipient=NativeDecodeRetirementRecipient::new();let mut allocation_issuer=crate::native_decoding::NativeDecodeAllocationIssuer::new();let mut allocation_loan=allocation_issuer.loan(&mut port);let mut native=NativeDecodeControl::new_forwarded(1<<20,&mut progress,&mut allocation_loan);native.install_retirement_recipient(&mut recipient).unwrap();let ledger=native.ledger_identity();let identity=native.retirement_recipient_identity();let mut work=Work{allow:mode!="workDenied"};let mut work_issuer=crate::native_decoding::OriginalWorkActionIssuer::new();
  let g=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:bytes,maximum_release_bytes:bytes,maximum_depth:1,..Default::default()};let((turn,refused),heap)=observe(||{let mut invoke=||{let mut work_loan=work_issuer.loan(&mut work);native.with_work_recipient(&mut work_loan,|native|native.with_retirement::<_,ValueError>(g,|loan|{assert_eq!(loan.ledger_identity(),ledger);assert_eq!(loan.retirement_recipient_identity(),identity);let funding:&mut dyn semio_framework_value_retirement::OriginalRetirementControl=loan;Ok(semio_framework_value_retirement::refill_original_failure_storage(funding,g))})).unwrap()};if mode=="allocationFailed"{crate::allocation_testing::refuse_one_allocation_on_this_thread(&mut invoke)}else{(invoke(),0)}});assert_eq!(refused,usize::from(mode=="allocationFailed"));
  assert_eq!(turn.work,u64::from(mode!="quoteRefused"&&mode!="workDenied"));assert_eq!(heap,(if mode=="quoteRefused"||mode=="workDenied"||mode=="allocationFailed"{0}else{bytes},0));if mode=="accepted"{assert_eq!(turn.result.unwrap().progress().retained_capacity_bytes,bytes)}else if mode=="workDenied"{assert_eq!(turn.result.unwrap().progress(),Default::default())}else{let error=turn.result.unwrap_err();if mode=="allocationFailed"{assert_eq!(error.primary().unwrap().kind,ValueRefusalKind::AllocationFailed)}if mode=="postRefused"{assert_eq!(error.receipt_refusal().unwrap().message,f["postRefusal"].as_str().unwrap());assert_eq!(error.retained_progress().retained_capacity_bytes,bytes)}}
  if native.has_retirement_owner(){let release=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:bytes,maximum_depth:1,..Default::default()};let(turn,heap)=observe(||{let mut work_loan=work_issuer.loan(&mut work);native.with_work_recipient(&mut work_loan,|native|Ok::<_,ValueError>(native.close_retirement_recipient(release))).unwrap()});assert_eq!(turn.work,1);assert_eq!(turn.result.unwrap().progress().released_bytes,bytes);assert_eq!(heap,(0,bytes));}assert!(!native.has_retirement_owner());drop(native);drop(allocation_loan);drop(port);let labels:Vec<&str>=events.iter().map(|event|match event{NativeAllocationEvent::Quote(_)=>"quote",NativeAllocationEvent::Post{..}=>"post",NativeAllocationEvent::Cancel(_)=>"cancel"}).collect();assert_eq!(serde_json::to_value(labels).unwrap(),f[mode]);if mode=="allocationFailed"{assert!(matches!(events.last(),Some(NativeAllocationEvent::Post{actual:0,..})))}
 }
}


/// 🛟️ A cancelled original finishes its actual pending receipt; another issuer returns the reply donor.
#[test]
fn retirement_recovery_preserves_callback_origin_work_and_complete_child_role(){
 use crate::native_decoding::{NativeConstructionRecipientIssuer,NativeDecodeConstructionRecipient,NativeDecodeWorkIssuer,NativeDecodeWorkRecipient,NativeReceivingEffect,NativeReceivingEffectPhase};
 use std::cell::{Cell,RefCell};
 struct Recipient<'a>{calls:&'a Cell<u64>,reply:&'a RefCell<Option<Result<(),ValueError>>>}
 impl NativeDecodeConstructionRecipient for Recipient<'_>{
  fn submitted(&self)->RetainedCloneProgress{Default::default()}
  fn grant(&mut self,d:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{Ok(RetainedCloneGrant{maximum_items:1,maximum_depth:d.depth,..Default::default()})}
  fn accept(&mut self,_:RetainedCloneGrant,_:RetainedCloneProgress)->Result<(),ValueError>{self.calls.set(self.calls.get()+1);*self.reply.borrow_mut()=Some(Ok(()));panic!("actual receipt callback boundary")}
 }
 struct Work{maximum:u64}
 impl NativeDecodeWorkRecipient for Work{
  fn available_work(&mut self,observed:u64)->Result<u64,ValueError>{self.maximum.checked_sub(observed).ok_or_else(||ValueError::invariant("recovery fixture exhausted its original work"))}
  fn admits(&mut self,observed:u64,additional:u64)->Result<bool,ValueError>{Ok(observed.checked_add(additional).is_some_and(|n|n<=self.maximum))}
 }
 let law:serde_json::Value=serde_json::from_str(include_str!("recovery.json")).unwrap();
 for role in law["roles"].as_array().unwrap(){
  let cancelled=Cell::new(false);let calls=Cell::new(0);let replies=RefCell::new(None);let child_calls=Cell::new(0);let mut observer=|_:NativeDecodeProgress|!cancelled.get();let mut native=NativeDecodeControl::new(0,&mut observer);let mut retirement_recipient=NativeDecodeRetirementRecipient::new();native.install_retirement_recipient(&mut retirement_recipient).unwrap();let ledger=native.ledger_identity();let recipient_id=native.retirement_recipient_identity();let mut recipient=Recipient{calls:&calls,reply:&replies};let mut issuer=NativeConstructionRecipientIssuer::new();let mut loan=issuer.loan(&mut recipient);let mut work=Work{maximum:law["maximumWork"].as_u64().unwrap()};let mut work_issuer=NativeDecodeWorkIssuer::new();let mut work_loan=work_issuer.loan(&mut work);let mut value_frame=NativeReceivingEffect::<ValueError>::default();let mut retirement_frame=NativeReceivingEffect::<semio_framework_value_retirement::OriginalRetirementFailure>::default();let mut held=None;let mut foreign_refusal=None;let is_retirement=role.as_str()==Some("retirement");
  native.with_work_recipient(&mut work_loan,|native|native.with_construction_recipient(&mut loan,|native|{
   let demand=RetirementDemand{depth:1,..Default::default()};
   let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{
    if is_retirement{let _=native.receive_retirement(demand,&mut retirement_frame,false,law["work"].as_u64().unwrap(),|_,_|{child_calls.set(child_calls.get()+1);crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original retirement child remains whole").into()),work:law["work"].as_u64().unwrap()}});}else{let _=native.receive_allocation(demand,&mut value_frame,false,law["work"].as_u64().unwrap(),|_,_|{child_calls.set(child_calls.get()+1);crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original value child remains whole")),work:law["work"].as_u64().unwrap()}});}
   }));assert!(interrupted.is_err());cancelled.set(true);assert_eq!(if is_retirement{retirement_frame.phase()}else{value_frame.phase()},NativeReceivingEffectPhase::ReceiptStarted);
   let foreign_calls=Cell::new(0);let foreign_replies=RefCell::new(None);let mut foreign_recipient=Recipient{calls:&foreign_calls,reply:&foreign_replies};let mut foreign_issuer=NativeConstructionRecipientIssuer::new();let mut foreign_loan=foreign_issuer.loan(&mut foreign_recipient);let mut foreign_observer=|_|true;let mut foreign_native=NativeDecodeControl::new(0,&mut foreign_observer);let mut foreign_retirement=NativeDecodeRetirementRecipient::new();foreign_native.install_retirement_recipient(&mut foreign_retirement)?;
   foreign_native.with_construction_recipient(&mut foreign_loan,|foreign|foreign.with_retirement(grant(demand),|close|{
    let reply=replies.borrow_mut().take().unwrap();let denied=if is_retirement{close.resolve_receiving_receipt(&mut retirement_frame,reply)}else{close.resolve_receiving_receipt(&mut value_frame,reply)};let(refusal,reply)=denied.unwrap_err();assert_eq!(refusal.kind,ValueRefusalKind::InvariantViolated);foreign_refusal=Some(refusal);assert!(reply.is_ok());*replies.borrow_mut()=Some(reply);Ok::<_,ValueError>(())
   }))?;assert_eq!(foreign_calls.get(),0);
   native.with_retirement(grant(demand),|close|{
    assert_eq!(close.ledger_identity(),ledger);assert_eq!(close.retirement_recipient_identity(),recipient_id);assert_eq!(close.owned_bytes(),law["newCapacity"].as_u64().unwrap()as usize);let reply=replies.borrow_mut().take().unwrap();
    if is_retirement{close.resolve_receiving_receipt(&mut retirement_frame,reply).unwrap();let turn=close.finish_retirement_receiving(&mut retirement_frame);assert_eq!(turn.work,law["work"].as_u64().unwrap());let failure=turn.result.unwrap_err();assert!(failure.retirement_failure.is_some());assert!(failure.allocation_refusal.is_none());held=Some(failure);}else{close.resolve_receiving_receipt(&mut value_frame,reply).unwrap();let turn=close.finish_recovery(crate::native_decoding::construction::NativeRecoveryLoan::Value(&mut value_frame));assert_eq!(turn.work,law["work"].as_u64().unwrap());let failure=turn.result.unwrap_err();assert!(failure.allocation_refusal.is_some());assert!(failure.retirement_failure.is_none());held=Some(failure);}
    Ok::<_,ValueError>(())
   })?;assert_eq!(native.checkpoint().unwrap_err().kind,ValueRefusalKind::Canceled);Ok::<_,ValueError>(())
  })).unwrap();assert_eq!(child_calls.get(),1);assert_eq!(calls.get(),1);assert_eq!(value_frame.phase(),NativeReceivingEffectPhase::Empty);assert_eq!(retirement_frame.phase(),NativeReceivingEffectPhase::Empty);assert_eq!(native.actual_work(),law["work"].as_u64().unwrap());drop(work_loan);drop(loan);assert_eq!(issuer.outstanding(),0);assert!(!work_issuer.has_pending_work());
  let mut owner=ControlledRetirement::retain((held,foreign_refusal));let mut stock=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:128};let mut fuel=4096u64;
  for _ in 0..128{if owner.terminal_is_empty(){break}let needed=owner.next_computational_work(stock.maximum_copy_bytes).unwrap();assert!(needed<=fuel);let mut actual=None;let(result,heap)=observe(||owner.step(stock,&mut actual));let progress=result.unwrap().progress();assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(actual.unwrap_or(0),heap.0);assert!(progress.fits(stock));fuel=fuel.checked_sub(owner.last_turn_work()).unwrap();stock.maximum_items-=progress.copied_items;stock.maximum_copy_bytes-=progress.copied_bytes;stock.maximum_capacity_bytes-=progress.retained_capacity_bytes;stock.maximum_release_bytes-=progress.released_bytes;}assert!(owner.terminal_is_empty());
 }
}
