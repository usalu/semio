//! 🧪️ Typed encoding owners preserve actual callback custody and expose only admitted retirement births.
use semio_framework_value_native_encoding::*;
use semio_framework_value_retirement::retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use semio_framework_value_testing_allocator::{observe_retirement_allocations as observe,refuse_one_allocation_on_this_thread};
use std::{mem::size_of,sync::{Arc,Mutex},panic::{catch_unwind,AssertUnwindSafe}};
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64}}
fn row(id:&str)->serde_json::Value{let f:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();f["cases"].as_array().unwrap().iter().find(|row|row["id"]==id).unwrap().clone()}
fn close<T:RetireOwned>(value:T,bank:&mut RetainedCloneGrant,fuel:&mut u64){
 let mut owner=ControlledRetirement::retain(value);let(mut born,mut released)=(0,0);
 for _ in 0..4096{
  if owner.terminal_is_empty(){break}
  let quote=owner.next_computational_work(65536).unwrap();assert!(quote>0&&quote<=*fuel);let g=RetainedCloneGrant{maximum_items:bank.maximum_items.min(1),..*bank};let before=owner.next_demand(65536).unwrap();let mut marker=None;
  let(denied,heap)=observe(||owner.step(RetainedCloneGrant{maximum_items:0,..g},&mut marker));assert_eq!(denied.unwrap().progress(),Default::default());assert_eq!(heap,(0,0));assert_eq!(marker,None);assert_eq!(owner.next_demand(65536).unwrap(),before);
  let(turn,heap)=observe(||owner.step(g,&mut marker));let p=turn.unwrap().progress();let work=owner.last_turn_work();assert!(p.fits(g));assert!(work<=quote);assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));assert_eq!(marker,if p.retained_capacity_bytes==0{None}else{Some(p.retained_capacity_bytes)});born+=heap.0;released+=heap.1;bank.maximum_items-=p.copied_items;bank.maximum_copy_bytes-=p.copied_bytes;bank.maximum_capacity_bytes-=p.retained_capacity_bytes;bank.maximum_release_bytes-=p.released_bytes;*fuel-=work;
 }
 assert!(owner.terminal_is_empty());assert_eq!(born,released);
}
fn error(name:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,name)}
#[test]
fn failure_and_allocation_cursors_have_only_the_admitted_native_backing(){
 let mut bank=stock();let mut fuel=32768;assert_eq!(row("allocation-settled")["closeAllowed"],true);assert_eq!(row("operation-failure")["closeAllowed"],true);assert_eq!(row("effect-failure")["birthHeap"],0);
 close(NativeEncodeOperationFailure{primary:Some(error("primary")),work_refusal:Some(error("work"))},&mut bank,&mut fuel);
 close(NativeEncodeAllocationOutcome{request:Some(NativeEncodeAllocation{bytes:16,owned_bytes:0,next_owned_bytes:16,maximum_bytes:64}),admitted:true,primary:Some(error("primary")),forwarded_quote:Some(error("forwarded quote")),scoped_quote:Some(error("scoped quote")),extent_refusal:Some(error("extent")),forwarded_refusal:Some(error("forwarded")),scoped_refusal:Some(error("scoped")),actual:Some(0)},&mut bank,&mut fuel);
 close(NativeEncodeEffectFailure{primary:Some(error("primary")),invariant:Some(error("invariant")),physical_invariant:Some(error("physical invariant")),capacity_invariant:Some(error("capacity invariant")),work:Some(error("work")),local:Some(error("local")),physical:Some(error("physical")),allocation:None},&mut bank,&mut fuel);
}
#[test]
fn consuming_continuation_constructs_its_typed_cursor_without_a_hidden_allocation(){
 let mut callback=|_|true;let mut original=NativeEncodeControl::new(64,&mut callback);original.charge(16).unwrap();let continuation=original.pause().unwrap();assert!(continuation.retirement_ready());assert_eq!(continuation.retirement_birth_bytes(),Some(size_of::<<NativeEncodeContinuation as RetireOwned>::Retirement>()));
 let(mut cursor,heap)=observe(||continuation.retirement());assert_eq!(heap,(row("continuation-settled")["birthHeap"].as_u64().unwrap()as usize,0));let mut actual=Some(73);assert!(matches!(cursor.close_step(Default::default(),&mut actual),RetirementStep::BudgetExhausted));assert_eq!(actual,Some(73));assert!(!cursor.terminal_is_empty());assert!(matches!(cursor.close_step(stock(),&mut actual),RetirementStep::Complete(1)));assert_eq!(actual,Some(73));assert!(cursor.terminal_is_empty());
}
#[test]
fn failed_typed_cursor_birth_preserves_both_original_failures_and_its_real_marker(){
 let mut bank=stock();let mut fuel=32768;let mut source=Some(NativeEncodeOperationFailure{primary:Some(error("primary")),work_refusal:Some(error("work"))});let mut cursor=None;let mut actual=None;
 let((turn,attempts),heap)=observe(||refuse_one_allocation_on_this_thread(||semio_framework_value_retirement::retirement::admit_retirement_cursor(&mut source,&mut cursor,stock(),&mut actual)));
 assert_eq!(attempts,1);assert_eq!(heap,(0,0));assert_eq!(actual,Some(0));assert_eq!(turn.work,1);assert_eq!(turn.result.unwrap_err().kind,ValueRefusalKind::AllocationFailed);assert!(cursor.is_none());let source=source.take().unwrap();assert_eq!(source.primary.as_ref().unwrap().message.as_ref(),"primary");assert_eq!(source.work_refusal.as_ref().unwrap().message.as_ref(),"work");close(source,&mut bank,&mut fuel);
}
#[test]
fn actual_pending_allocation_returns_to_its_original_callback_before_retirement(){
 let mut bank=stock();let mut fuel=32768;assert_eq!(row("allocation-pending")["closeAllowed"],false);
 let response=Arc::new(Mutex::new(None::<Result<(),ValueError>>));let write=response.clone();let mut port=move|event:NativeEncodeAllocationEvent|{if matches!(event,NativeEncodeAllocationEvent::Quote(_)){*write.lock().unwrap()=Some(Err(error("original quote refused")));panic!("actual original quote callback interrupted")}Ok(())};
 let mut issuer=NativeEncodeAllocationIssuer::new();let mut loan=issuer.loan(&mut port);let mut callback=|_|true;let mut native=NativeEncodeControl::new_forwarded(64,&mut callback,&mut loan);let mut actual=None;let mut progress=None;let mut operation=|_:&mut Option<usize>|->Result<(),ValueError>{panic!("refused original quote must not run an allocator")};
 let interrupted=catch_unwind(AssertUnwindSafe(||native.allocate_effect(16,&mut actual,&mut progress,&mut operation)));assert!(interrupted.is_err());assert_eq!(actual,None);assert!(progress.as_ref().unwrap().is_pending());assert!(!progress.as_ref().unwrap().retirement_ready());assert_eq!(progress.as_ref().unwrap().retirement_birth_bytes(),None);
 let(original,heap)=observe(||progress.take().unwrap().retirement());assert_eq!(heap,(0,0));let mut retired=original;assert!(retired.requires_original_control());let request=retired.pending_original().unwrap().request();let mut marker=Some(73);let(step,heap)=observe(||retired.close_step(stock(),&mut marker));let refusal=match step{RetirementStep::Failure(refusal,0)=>refusal,_=>panic!("pending original was not retained")};assert_eq!(marker,Some(73));assert_eq!(heap,(0,0));assert_eq!(retired.pending_original().unwrap().request(),request);close(refusal,&mut bank,&mut fuel);
 let quote=NativeEncodeRetirement::<NativeEncodeAllocationProgress,<NativeEncodeAllocationOutcome as RetireOwned>::Retirement>::pending_return_work_demand().unwrap();let demand=NativeEncodeRetirement::<NativeEncodeAllocationProgress,<NativeEncodeAllocationOutcome as RetireOwned>::Retirement>::pending_return_demand().unwrap();let g=RetainedCloneGrant{maximum_items:bank.maximum_items.min(1),maximum_copy_bytes:bank.maximum_copy_bytes.min(demand.copy_bytes),maximum_depth:bank.maximum_depth.min(1),..bank};assert!(quote<=fuel);
 let mut output=None;for work in [0,quote-1]{let(turn,heap)=observe(||retired.return_pending(&mut output,g,work));assert_eq!(turn.work,0);assert_eq!(heap,(0,0));assert!(output.is_none());assert_eq!(retired.pending_original().unwrap().request(),request);}
 let(turn,heap)=observe(||retired.return_pending(&mut output,g,quote));assert_eq!(heap,(0,0));assert_eq!(turn.work,quote);let p=turn.result.unwrap().progress();assert_eq!(p.copied_bytes,demand.copy_bytes);assert!(p.fits(g));bank.maximum_items-=p.copied_items;bank.maximum_copy_bytes-=p.copied_bytes;bank.maximum_capacity_bytes-=p.retained_capacity_bytes;bank.maximum_release_bytes-=p.released_bytes;fuel-=turn.work;assert!(retired.terminal_is_empty());assert_eq!(output.as_ref().unwrap().request(),request);
 let result=response.lock().unwrap().take().unwrap();native.resolve_forwarded_allocation_callback(output.as_mut().unwrap(),result).unwrap();native.settle_allocation_progress(output.as_mut().unwrap(),None).unwrap();assert!(!output.as_ref().unwrap().is_pending());assert_eq!(output.as_ref().unwrap().outcome().forwarded_quote.as_ref().unwrap().message.as_ref(),"original quote refused");assert!(output.as_ref().unwrap().retirement_ready());drop(native);close(output.take().unwrap(),&mut bank,&mut fuel);
}

struct SettlementWork{interrupt:bool,response:Arc<Mutex<Option<Result<(),ValueError>>>>,remaining:u64,posted:u64,quote:u64,actual:u64}
impl NativeEncodeWorkRecipient for SettlementWork{
 fn admits(&mut self,_:u64,additional:u64)->Result<bool,ValueError>{assert_eq!(additional,self.quote);Ok(additional<=self.remaining)}
 fn committed(&mut self,actual:u64)->Result<(),ValueError>{assert_eq!(actual,self.actual);self.remaining=self.remaining.checked_sub(actual).unwrap();self.posted+=actual;let result=Err(error("original work post"));if self.interrupt{*self.response.lock().unwrap()=Some(result);panic!("original work post interrupted")}result}
}
struct SettlementPhysical{interrupt:bool,response:Arc<Mutex<Option<Result<(),ValueError>>>>,submitted:RetainedCloneProgress,remaining:RetainedCloneGrant}
impl NativeEncodeConstructionRecipient for SettlementPhysical{
 fn submitted(&self)->RetainedCloneProgress{self.submitted}
 fn grant(&mut self,demand:RetirementDemand)->Result<RetainedCloneGrant,ValueError>{assert_eq!(demand.copy_bytes,3);Ok(self.remaining)}
 fn accept(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>{assert!(progress.fits(grant)&&progress.fits(self.remaining));self.remaining.maximum_items-=progress.copied_items;self.remaining.maximum_copy_bytes-=progress.copied_bytes;self.remaining.maximum_capacity_bytes-=progress.retained_capacity_bytes;self.remaining.maximum_release_bytes-=progress.released_bytes;self.submitted=self.submitted.checked_add(progress)?;let result=Err(error("original physical post"));if self.interrupt{*self.response.lock().unwrap()=Some(result);panic!("original physical post interrupted")}result}
}
#[test]
fn settled_and_interrupted_effects_keep_their_original_post_owners_until_reconciled(){
 for (id,interrupt_work,interrupt_physical) in [("settlement-settled",false,false),("settlement-work-pending",true,false),("settlement-physical-pending",false,true)]{
  let expected=row(id);let quote=expected["selectedWork"].as_u64().unwrap();let actual=expected["actualWork"].as_u64().unwrap();assert!(actual<quote);let mut bank=stock();let mut fuel=32768;let effect_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:3,maximum_depth:1,..Default::default()};assert!(effect_grant.maximum_items<=bank.maximum_items&&effect_grant.maximum_copy_bytes<=bank.maximum_copy_bytes&&quote<=fuel);bank.maximum_items-=effect_grant.maximum_items;bank.maximum_copy_bytes-=effect_grant.maximum_copy_bytes;fuel-=quote;let response=Arc::new(Mutex::new(None));let mut payer=SettlementWork{interrupt:interrupt_work,response:response.clone(),remaining:quote,posted:0,quote,actual};let mut physical=SettlementPhysical{interrupt:interrupt_physical,response:response.clone(),submitted:Default::default(),remaining:effect_grant};
  let mut work_issuer=NativeEncodeWorkIssuer::new();let mut physical_issuer=NativeEncodeConstructionIssuer::new();let mut callback=|_|true;let mut native=NativeEncodeControl::new(0,&mut callback);let mut slot=None;let mut output=[0u8;3];{let mut work_loan=work_issuer.loan(&mut payer);let mut physical_loan=physical_issuer.loan(&mut physical);
  native.with_work_recipient(&mut work_loan,|native|native.with_construction_recipient(&mut physical_loan,|native|{
   native.begin_effect_settlement(&mut slot,RetirementDemand{copy_bytes:3,depth:1,..Default::default()},quote)?;assert_eq!(actual,output.len()as u64);assert_eq!(output,[0;3]);for byte in &mut output{*byte=1;}slot.as_mut().unwrap().record_effect(RetainedCloneProgress{copied_items:1,copied_bytes:3,..Default::default()},actual)?;
   let interrupted=catch_unwind(AssertUnwindSafe(||native.settle_effect(&mut slot)));assert_eq!(interrupted.is_err(),interrupt_work||interrupt_physical);
   let owner=slot.as_ref().unwrap();assert_eq!(owner.retirement_ready(),expected["closeAllowed"].as_bool().unwrap());assert_eq!(owner.retirement_birth_bytes().is_none(),expected["pending"].as_bool().unwrap());
   if interrupt_work||interrupt_physical{
    let identity=owner.identity();let(cursor,heap)=observe(||slot.take().unwrap().retirement());assert_eq!(heap,(expected["birthHeap"].as_u64().unwrap()as usize,0));let mut cursor=cursor;let mut marker=Some(73);let(turn,heap)=observe(||cursor.close_step(stock(),&mut marker));assert_eq!(heap,(0,0));assert_eq!(marker,Some(73));let refusal=match turn{RetirementStep::Failure(error,0)=>error,_=>panic!("pending original callback was consumed")};close(refusal,&mut bank,&mut fuel);assert_eq!(cursor.pending_original().unwrap().identity(),identity);
    let demand=NativeEncodeRetirement::<NativeEncodeEffectSettlement,<(Option<ValueError>,(Option<ValueError>,(Option<ValueError>,()))) as RetireOwned>::Retirement>::pending_return_demand()?;
    let quote=NativeEncodeRetirement::<NativeEncodeEffectSettlement,<(Option<ValueError>,(Option<ValueError>,(Option<ValueError>,()))) as RetireOwned>::Retirement>::pending_return_work_demand()?;
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_depth:1,..Default::default()};assert!(grant.maximum_copy_bytes<=bank.maximum_copy_bytes&&quote<=fuel);
    let(denied,heap)=observe(||cursor.return_pending(&mut slot,grant,quote-1));assert_eq!(heap,(0,0));assert_eq!(denied.work,0);assert!(slot.is_none());
    let(turn,heap)=observe(||cursor.return_pending(&mut slot,grant,quote));assert_eq!(heap,(0,0));let progress=turn.result?.progress();assert_eq!(turn.work,quote);bank.maximum_items-=progress.copied_items;bank.maximum_copy_bytes-=progress.copied_bytes;fuel-=turn.work;assert!(cursor.terminal_is_empty());assert_eq!(slot.as_ref().unwrap().identity(),identity);
    let mut result=response.lock().unwrap().take();native.resolve_effect_post(&mut slot,interrupt_work,&mut result)?;assert!(result.is_none());native.settle_effect(&mut slot)?;
   }else{interrupted.unwrap()?;}
   let owner=slot.as_ref().unwrap();assert_eq!(owner.work_quote(),quote);assert_eq!(owner.actual_work(),Some(actual));assert!(owner.retirement_ready());let(work,physical,invariant)=owner.failures();assert_eq!(work.unwrap().message.as_ref(),"original work post");assert_eq!(physical.unwrap().message.as_ref(),"original physical post");assert!(invariant.is_none());Ok::<(),ValueError>(())
  })).unwrap();}
  assert_eq!(output,[1;3]);assert_eq!(payer.posted,actual);assert_eq!(payer.remaining,expected["unspentWork"].as_u64().unwrap());assert_eq!(physical.submitted.copied_items,1);assert_eq!(physical.submitted.copied_bytes,3);bank.maximum_items+=physical.remaining.maximum_items;bank.maximum_copy_bytes+=physical.remaining.maximum_copy_bytes;bank.maximum_capacity_bytes+=physical.remaining.maximum_capacity_bytes;bank.maximum_release_bytes+=physical.remaining.maximum_release_bytes;fuel+=payer.remaining;
  close(slot.take().unwrap(),&mut bank,&mut fuel);
 }
}
#[test]
fn scoped_pause_refuses_the_pending_original_and_recovers_the_same_control(){
 let expected=row("continuation-pause-refused");assert_eq!(expected["closeAllowed"],false);assert!(expected["birthHeap"].is_null());let mut bank=stock();let mut fuel=32768;let response=Arc::new(Mutex::new(None));let write=response.clone();let mut port=move|event:NativeEncodeAllocationEvent|{if matches!(event,NativeEncodeAllocationEvent::Quote(_)){*write.lock().unwrap()=Some(Err(error("original scoped quote refused")));panic!("original scoped quote interrupted")}Ok(())};
 let mut issuer=NativeEncodeAllocationIssuer::new();let mut loan=issuer.scoped_loan(&mut port);let mut recipient=NativeEncodeRetirementRecipient::new();let identity=recipient.identity();let mut callback=|_|true;let mut work=|_,_|Ok(());let mut native=NativeEncodeControl::new_scoped_source(64,16,&mut callback,&mut loan,&mut work);native.install_retirement_recipient(&mut recipient).unwrap();let mut actual=None;let mut pending=None;
 assert!(catch_unwind(AssertUnwindSafe(||native.allocate_effect(16,&mut actual,&mut pending,&mut |_|panic!("refused quote cannot allocate")))).is_err());assert_eq!(actual,None);
 let(failure,mut native)=match native.pause_scoped_source(&pending){Err(original)=>original,Ok(_)=>panic!("pending scoped owner detached")};assert_eq!(native.retirement_recipient_identity(),Some(identity));assert!(pending.as_ref().unwrap().is_pending());close(failure,&mut bank,&mut fuel);
 native.resolve_scoped_allocation_callback(pending.as_mut().unwrap(),response.lock().unwrap().take().unwrap()).unwrap();native.settle_allocation_progress(pending.as_mut().unwrap(),None).unwrap();assert!(pending.as_ref().unwrap().retirement_ready());close(pending.take().unwrap(),&mut bank,&mut fuel);
 let receipt=match native.pause_scoped_source(&pending){Ok(receipt)=>receipt,Err(_)=>panic!("reconciled original cannot detach")};assert_eq!(receipt.owned_bytes(),0);close(receipt,&mut bank,&mut fuel);
}
