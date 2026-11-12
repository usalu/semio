//! 🧫️ Actual source callers, native decoder receipts and heap witnesses exercise same-allocation factory admission.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,RevisionId,InteractiveJobCloseStep,JobOutcomeBorrow,JobOutcomeDescriptor,JobOutcomeView};
#[derive(semio_framework_value::RetireOwned)]#[value(crate="::semio_framework_value")]
struct Job{identity:Option<(u64,u64)>,body:[u8;70000]}
impl InteractiveJob for Job{
 fn step<'a>(&'a mut self,_:&mut StepContext<'_>,_:&mut semio_framework_job::OriginalJobReturnClaim<'_>)->semio_framework_job::InteractiveJobStep<'a>{semio_framework_job::InteractiveJobStep::from_callback(Ok(None))}
 fn borrow_outcome<'a>(&'a self,_:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"fixture has no publication"))}
 fn begin_close(&mut self){}
 fn close_step(&mut self,_:&mut StepContext<'_>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{Ok(InteractiveJobCloseStep::Complete{progress:Default::default()})}
 fn terminal_is_empty(&self)->bool{true}
}
pub(crate) fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1<<24,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:64}}
pub(crate) fn clock()->Option<u64>{Some(1)}
pub(crate) fn source<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,value:T)->Box<T>{
 let mut value=Some(value);let mut admission=Admission::default();let mut slot=None;
 for _ in 0..64{if caller.turn_with_decoder(grant(),decoder,|cx|admit_original_construction_settled(cx,&mut admission,&mut slot,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:size_of::<T>(),release_bytes:0,depth:1},1,|_,actual|{*actual=Some(size_of::<T>());semio_framework_value::RetainedWorkOutcome{result:Ok((Box::new(value.take().expect("source value is consumed once")),RetainedCloneProgress{copied_items:1,retained_capacity_bytes:size_of::<T>(),..Default::default()})),work:1}})).unwrap(){adopt_credit(caller,decoder);return slot.unwrap()}}panic!("source construction must settle")
}
fn metadata(cx:&mut StepContext<'_>,bytes:usize,items:usize)->bool{let progress=RetainedCloneProgress{copied_items:items,copied_bytes:bytes,..Default::default()};if cx.retirement_should_yield()||cx.fuel_remaining()<items as u64||cx.retained_grant().maximum_depth==0||!progress.fits(cx.retained_grant()){return false}cx.consume_retained(progress).unwrap();cx.consume_fuel(items as u64);true}
pub(crate) fn retire<T:semio_framework_value::retirement::RetireOwned>(caller:&mut OriginalJobFixtureCaller,original:&mut Option<T>){
 if original.is_none(){return}
 let mut owner=None;caller.turn(grant(),|cx|{let bytes=size_of::<Option<T>>()+size_of::<Option<semio_framework_value::retirement::controlled::ControlledRetirement<T>>>();assert!(metadata(cx,bytes,2));owner=Some(semio_framework_value::retirement::controlled::ControlledRetirement::retain(original.take().unwrap()));Ok(())}).unwrap();
 for _ in 0..10000{if owner.as_ref().unwrap().terminal_is_empty(){break}caller.turn(grant(),|cx|{let demand=owner.as_ref().unwrap().next_demand(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_effect_step(cx,demand,|effect|owner.as_mut().unwrap().step(effect.grant().maximum_copy_bytes,effect)).map(|_|())}).unwrap();}
 assert!(owner.as_ref().unwrap().terminal_is_empty());caller.turn(grant(),|cx|{assert!(metadata(cx,size_of_val(&owner),1));owner=None;Ok(())}).unwrap();
}
fn release<T:semio_framework_value::retirement::RetireOwned>(caller:&mut OriginalJobFixtureCaller,slot:&mut Option<Box<T>>){retire(caller,slot)}
pub(crate) fn close(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){adopt_credit(caller,decoder);settle_credits(caller,decoder);while !decoder.terminal_is_empty(){caller.turn(grant(),|cx|{decoder.close_on_source(cx)?;Ok(())}).unwrap();}}
pub(crate) fn admitted_decoder(caller:&mut OriginalJobFixtureCaller)->OriginalJobDecodeOwner{let mut decoder=OriginalJobDecodeOwner::empty();for _ in 0..64{if caller.turn(grant(),|cx|Ok(decoder.admit_step(cx)?)).unwrap(){return decoder}}panic!("original decoder must admit its Source")}
type Status=semio_framework_job::OriginalConstructionStatus;
type Admission=semio_framework_job::OriginalConstructionAdmission;
thread_local!{static CREDITS:std::cell::RefCell<Vec<std::mem::ManuallyDrop<semio_framework_job::OriginalCreditedOutput<()>>>>=const{std::cell::RefCell::new(Vec::new())};}
fn adopt_credit(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){if !decoder.source_loan_admitted(){return}if let Ok(credit)=caller.turn_with_decoder(grant(),decoder,|cx|cx.adopt_original_credit(())).unwrap(){CREDITS.with(|credits|credits.borrow_mut().push(std::mem::ManuallyDrop::new(credit)))}}
pub(crate) fn settle_credits(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){let mut credits=CREDITS.with(|credits|std::mem::take(&mut *credits.borrow_mut()));for credit in credits.iter_mut(){let mut settled=false;for _ in 0..16{if caller.turn(grant(),|cx|cx.settle_original_credit(credit)).unwrap(){settled=true;break}}assert!(settled,"adopted scope credit must settle");unsafe{std::mem::ManuallyDrop::drop(credit)}}}
fn drive(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,admission:&mut Admission,mut step:impl FnMut(&mut StepContext<'_>,&mut Admission)->Result<Status,OriginalTurnFailure>)->Status{
 for _ in 0..16{let status=caller.turn_with_decoder(grant(),decoder,|cx|step(cx,admission)).unwrap();if status!=Status::Pending{adopt_credit(caller,decoder);return status}}panic!("multi-turn original construction must settle")
}
fn finish(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,admission:&mut Admission,done:bool){
 if !done{let mut failure=None;caller.turn_with_decoder(grant(),decoder,|cx|{admission.take_failure_into_step(&mut failure,cx)?;Ok(())}).unwrap();retire(caller,&mut failure);}
 for _ in 0..16{if admission.terminal_is_empty(){return}caller.turn_with_decoder(grant(),decoder,|cx|{if done{admission.reset_step(cx)?;}admission.close_step(cx)}).unwrap();}panic!("construction control must retire")
}
#[test]
fn actual_original_factory_denied_axes_keep_same_box_and_admitted_reuse_has_zero_heap_effects(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();for case in corpus["cases"].as_array().unwrap(){let operation=Operation::new(OperationId(case["operation"].as_u64().unwrap()),RevisionId(0),Generation(case["generation"].as_u64().unwrap()),0);let mut caller=OriginalJobFixtureCaller::admit(operation.operation,operation.generation,grant(),clock,u64::MAX,10000);let mut decoder=admitted_decoder(&mut caller);let mut payload=Some(source(&mut caller,&mut decoder,Job{identity:None,body:[7;70000]}));let original=payload.as_deref().unwrap() as *const Job;let mut input=None;let mut checkpoint=None;let mut output=None;let mut admission=Admission::default();
 for axis in 0..2{let denied=if axis==0{RetainedCloneGrant{maximum_items:0,..grant()}}else{RetainedCloneGrant{maximum_depth:0,..grant()}};let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(denied,&mut decoder,|cx|admit_original_tool_job(cx,&mut admission,&operation,&mut payload,&mut input,&mut checkpoint,&mut output,ToolJobAdmissionDemand::reused_job(),|identity,mut job,_,_|{job.identity=Some((identity.operation().0,identity.generation().0));job})));assert_eq!(result.unwrap(),Status::Pending);assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,0);assert_eq!(payload.as_deref().unwrap()as*const Job,original);assert!(output.is_none());}
 let(status,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drive(&mut caller,&mut decoder,&mut admission,|cx,admission|admit_original_tool_job(cx,admission,&operation,&mut payload,&mut input,&mut checkpoint,&mut output,ToolJobAdmissionDemand::reused_job(),|identity,mut job,_,_|{job.identity=Some((identity.operation().0,identity.generation().0));job})));assert_eq!(status,Status::Ready);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(output.as_deref().unwrap()as*const Job,original);let identity=output.as_deref().unwrap().identity.unwrap();assert_eq!(identity,(operation.operation.0,operation.generation.0));assert_eq!(output.as_deref().unwrap().body[69999],7);finish(&mut caller,&mut decoder,&mut admission,true);release(&mut caller,&mut output);close(&mut caller,&mut decoder);caller.retire(grant());}
}
#[test]
fn actual_original_factory_post_birth_cancellation_retains_candidate_and_whole_physical_receipt(){
 let operation=Operation::new(OperationId(93),RevisionId(0),Generation(11),0);let mut caller=OriginalJobFixtureCaller::admit(operation.operation,operation.generation,grant(),clock,u64::MAX,10000);let cancel=caller.admit_alias(grant());let mut decoder=admitted_decoder(&mut caller);let mut payload=Some(source(&mut caller,&mut decoder,9u64));let mut input=None;let mut checkpoint=None;let mut output=None;let demand=ToolJobAdmissionDemand::boxed_job::<u64,Job>();let mut admission=Admission::default();
 let status=drive(&mut caller,&mut decoder,&mut admission,|cx,admission|admit_original_tool_job(cx,admission,&operation,&mut payload,&mut input,&mut checkpoint,&mut output,demand,|identity,payload,_,_|{let job=Box::new(Job{identity:Some((identity.operation().0,identity.generation().0)),body:[*payload as u8;70000]});drop(payload);cancel.cancel_now();job}));
 assert_eq!(status,Status::Failed);assert!(admission.is_failed());assert!(payload.is_none());assert_eq!(output.as_deref().unwrap().body[69999],9);finish(&mut caller,&mut decoder,&mut admission,false);release(&mut caller,&mut output);close(&mut caller,&mut decoder);caller.return_alias(cancel,grant());caller.retire(grant());
}

#[test]
fn actual_constructor_primary_and_physical_receipt_survive_secondary_original_cancellation(){
 let operation=Operation::new(OperationId(107),RevisionId(0),Generation(37),0);let mut caller=OriginalJobFixtureCaller::admit(operation.operation,operation.generation,grant(),clock,u64::MAX,10000);let cancel=caller.admit_alias(grant());let mut decoder=admitted_decoder(&mut caller);let mut payload=Some(source(&mut caller,&mut decoder,13u64));let pointer=payload.as_deref().unwrap()as*const u64;let mut input=None;let mut checkpoint=None;let mut output:Option<Box<Job>>=None;let demand=ToolJobAdmissionDemand{items:1,copy_bytes:0,capacity_bytes:70000,release_bytes:70000,depth:1};let progress=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:70000,released_bytes:70000,..Default::default()};let mut admission=Admission::default();
 let status=drive(&mut caller,&mut decoder,&mut admission,|cx,admission|admit_original_tool_job_controlled(cx,admission,&operation,&mut payload,&mut input,&mut checkpoint,&mut output,demand,|_,_,_,_|{let temporary=Vec::<u8>::with_capacity(70000);assert_eq!(temporary.capacity(),70000);drop(temporary);cancel.cancel_now();Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original constructor refusal retained whole").with_retained_progress(progress))}));
 assert_eq!(status,Status::Failed);assert!(admission.is_failed());assert_eq!(payload.as_deref().unwrap()as*const u64,pointer);assert!(output.is_none());finish(&mut caller,&mut decoder,&mut admission,false);release(&mut caller,&mut payload);close(&mut caller,&mut decoder);caller.return_alias(cancel,grant());caller.retire(grant());
}

#[test]
fn scoped_nested_owner_refusal_is_atomic_and_allocator_over_quote_keeps_actual_decoder_truth(){
 let operation=Operation::new(OperationId(109),RevisionId(0),Generation(41),0);let mut caller=OriginalJobFixtureCaller::admit(operation.operation,operation.generation,grant(),clock,u64::MAX,10000);let mut decoder=admitted_decoder(&mut caller);let mut output:Option<Box<[u8;70000]>>=None;let mut admission=Admission::default();
 let demand=ToolJobAdmissionDemand{items:1,copy_bytes:0,capacity_bytes:69999,release_bytes:0,depth:1};let actual=RetainedCloneProgress{copied_items:1,retained_capacity_bytes:70000,..Default::default()};
 let status=drive(&mut caller,&mut decoder,&mut admission,|cx,admission|admit_original_tool_construction_scoped(cx,admission,&mut output,demand,|scope|{assert!(scope.admit_owner(RetirementDemand{capacity_bytes:70000,depth:1,..Default::default()}).is_err());scope.admit_owner(RetirementDemand{capacity_bytes:69999,depth:1,..Default::default()})?;assert!(scope.admit_owner(RetirementDemand{capacity_bytes:1,depth:1,..Default::default()}).is_err());Ok((Box::new([3;70000]),actual))}));
 assert_eq!(status,Status::Failed);assert!(admission.is_failed());assert_eq!(output.as_deref().unwrap()[69999],3);finish(&mut caller,&mut decoder,&mut admission,false);release(&mut caller,&mut output);close(&mut caller,&mut decoder);caller.retire(grant());
}

#[test]
fn original_scoped_nested_owner_admission_matches_every_neutral_quote(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🧰️scope/🔣️.json")).unwrap();let operation=Operation::new(OperationId(113),RevisionId(0),Generation(43),0);let mut caller=OriginalJobFixtureCaller::admit(operation.operation,operation.generation,grant(),clock,u64::MAX,10000);let mut decoder=admitted_decoder(&mut caller);
 for row in corpus["cases"].as_array().unwrap(){let numbers=|key:&str|->[usize;4]{std::array::from_fn(|axis|row[key][axis].as_u64().unwrap()as usize)};let quote=numbers("quote");let mut admission=Admission::default();let mut output=None;let mut admitted=false;for _ in 0..64{admitted=caller.turn_with_decoder(grant(),&mut decoder,|cx|admit_original_tool_construction_settled(cx,&mut admission,&mut output,ToolJobAdmissionDemand{items:1,copy_bytes:quote[0],capacity_bytes:quote[1],release_bytes:quote[2],depth:quote[3]},|scope|{for(index,owner)in row["owners"].as_array().unwrap().iter().enumerate(){let value=|axis:usize|owner[axis].as_u64().unwrap()as usize;let result=scope.admit_owner(RetirementDemand{copy_bytes:value(0),capacity_bytes:value(1),release_bytes:value(2),depth:value(3)});assert_eq!(result.is_ok(),row["accepted"][index].as_bool().unwrap());}let remaining=numbers("remaining");assert_eq!([scope.remaining.copy_bytes,scope.remaining.capacity_bytes,scope.remaining.release_bytes,scope.remaining.depth],remaining);Ok(((),RetainedCloneProgress{copied_items:1,..Default::default()}))})).unwrap();if admitted{break}}assert!(admitted&&output.is_some());assert_eq!(decoder.owned_bytes(),0);}
 close(&mut caller,&mut decoder);caller.retire(grant());
}

#[test]
fn actual_original_close_denials_preserve_each_cursor_and_exact_physical_receipt(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(107),Generation(13),grant(),clock,u64::MAX,100000);let mut decoder=admitted_decoder(&mut caller);let mut output=None;let mut admission=Admission::default();
 for _ in 0..64{if caller.turn_with_decoder(grant(),&mut decoder,|cx|admit_original_tool_construction_settled(cx,&mut admission,&mut output,ToolJobAdmissionDemand{capacity_bytes:128,..ToolJobAdmissionDemand::reused_job()},|_|{let mut bytes=Vec::<u8>::with_capacity(128);bytes.extend_from_slice(&[7;128]);let actual=bytes.capacity();let owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(bytes).map_err(|(error,_)|error)?;Ok((owner,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:actual,..Default::default()}))})).unwrap(){break}}
 let owner=output.as_mut().unwrap();for _ in 0..10000{if owner.terminal_is_empty(){break}let demand=owner.next_demand(grant().maximum_copy_bytes).unwrap();let funded=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
 for axis in 0..5{let mut denied=funded;let extent=match axis{0=>&mut denied.maximum_items,1=>&mut denied.maximum_copy_bytes,2=>&mut denied.maximum_capacity_bytes,3=>&mut denied.maximum_release_bytes,_=>&mut denied.maximum_depth};if *extent==0{continue}*extent-=1;let before=caller.cumulative_receipt();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(denied,|cx|semio_framework_job::close_original_effect_step(cx,demand,|effect|owner.step(effect.grant().maximum_copy_bytes,effect))));assert_eq!(result.unwrap().progress(),Default::default());assert_eq!(heap,Default::default());assert_eq!(caller.cumulative_receipt(),before);}
 let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(funded,|cx|semio_framework_job::close_original_effect_step(cx,demand,|effect|owner.step(effect.grant().maximum_copy_bytes,effect))));let receipt=result.unwrap().progress();assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));assert_eq!(caller.receipt(),receipt);
 }assert!(owner.terminal_is_empty());output.take();close(&mut caller,&mut decoder);caller.retire(grant());
}
#[test]
fn actual_original_close_failure_keeps_whole_primary_and_actual_released_backing(){
 for diagnostic in [false,true]{let mut caller=OriginalJobFixtureCaller::admit(OperationId(108),Generation(13),grant(),clock,u64::MAX,10000);let mut decoder=admitted_decoder(&mut caller);let mut original=None;let mut admission=Admission::default();
 for _ in 0..64{if caller.turn_with_decoder(grant(),&mut decoder,|cx|admit_original_tool_construction_settled(cx,&mut admission,&mut original,ToolJobAdmissionDemand{capacity_bytes:128,..ToolJobAdmissionDemand::reused_job()},|_|{let bytes=Vec::<u8>::with_capacity(128);let actual=bytes.capacity();Ok((bytes,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:actual,..Default::default()}))})).unwrap(){break}}let released=original.as_ref().unwrap().capacity();let receipt=RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()};
 let mut primary=Some(if diagnostic{let mut fault=semio_framework_diagnostic::Fault::from("whole original diagnostic close refusal");fault.retained_progress=receipt;semio_framework_job::OriginalTurnPrimary::from(fault)}else{ValueError::literal(ValueRefusalKind::InvariantViolated,"whole original value close refusal").with_retained_progress(receipt).into()});
 let mut refresh=None;let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant(),|cx|{refresh=Some(semio_framework_job::original_return_refresh_quote(cx)?);semio_framework_job::close_original_retained_turn(cx,RetirementDemand{release_bytes:released,depth:1,..Default::default()},|_|{drop(original.take());semio_framework_value::RetainedWorkOutcome{result:Err(primary.take().unwrap()),work:1}})}));let failure=result.unwrap_err();assert_eq!((heap.requested_bytes,heap.released_bytes),(0,released));assert_eq!(failure.primary.retained_progress(),receipt);assert_eq!(caller.receipt(),receipt.checked_add(refresh.unwrap()).unwrap());assert!(failure.receipt_refusal.is_none());if diagnostic{assert!(failure.primary_fault().is_some())}else{assert!(failure.primary_value().is_some())}let mut failure_owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(failure).map_err(|(error,_)|error).unwrap();for _ in 0..10000{if failure_owner.terminal_is_empty(){break}let demand=failure_owner.next_demand(grant().maximum_copy_bytes).unwrap();caller.turn(grant(),|cx|semio_framework_job::close_original_effect_step(cx,demand,|effect|failure_owner.step(effect.grant().maximum_copy_bytes,effect))).unwrap();}assert!(failure_owner.terminal_is_empty());close(&mut caller,&mut decoder);caller.retire(grant());}
}

#[test]
fn settled_construction_refuses_a_quote_no_turn_can_fund_and_backpressures_a_narrow_turn(){
 let operation=Operation::new(OperationId(117),RevisionId(0),Generation(47),0);let mut caller=OriginalJobFixtureCaller::admit(operation.operation,operation.generation,grant(),clock,u64::MAX,10000);let mut decoder=admitted_decoder(&mut caller);let mut output:Option<Box<u8>>=None;
 let over=ToolJobAdmissionDemand{capacity_bytes:grant().maximum_capacity_bytes+1,..ToolJobAdmissionDemand::reused_job()};
 let refused=caller.turn_with_decoder(grant(),&mut decoder,|cx|admit_original_tool_construction_settled(cx,&mut Default::default(),&mut output,over,|_|panic!("unfundable construction cannot run"))).unwrap_err();assert_eq!(refused.primary_value().unwrap().kind,ValueRefusalKind::OwnershipLimit);assert!(output.is_none());
 let fits=ToolJobAdmissionDemand{capacity_bytes:1,..ToolJobAdmissionDemand::reused_job()};
 let narrow=RetainedCloneGrant{maximum_items:0,..grant()};let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(narrow,&mut decoder,|cx|admit_original_tool_construction_settled(cx,&mut Default::default(),&mut output,fits,|_|panic!("narrow turn cannot run construction"))));assert!(!result.unwrap());assert!(output.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 let mut admission=Admission::default();let mut settled=false;for _ in 0..64{if caller.turn_with_decoder(grant(),&mut decoder,|cx|admit_original_tool_construction_settled(cx,&mut admission,&mut output,fits,|_|Ok((Box::new(1u8),RetainedCloneProgress{copied_items:1,retained_capacity_bytes:1,..Default::default()})))).unwrap(){settled=true;break}}assert!(settled);
 assert!(output.is_some());output.take();close(&mut caller,&mut decoder);caller.retire(grant());
}

fn warm_constructor(_:&mut semio_framework_value_native_owned::native_decoding::construction::NativeConstructionScope<'_,'_>,actual:&mut Option<usize>)->semio_framework_value::RetainedWorkOutcome<(Box<u8>,RetainedCloneProgress)>{*actual=Some(1);semio_framework_value::RetainedWorkOutcome{result:Ok((Box::new(1u8),RetainedCloneProgress{copied_items:1,retained_capacity_bytes:1,..Default::default()})),work:1}}
fn below(grant:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:grant.maximum_items-1,..grant}}
fn funded_prerequisites(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,grant:RetainedCloneGrant,turns:usize)->Vec<(bool,RetainedCloneProgress)>{
 (0..turns).map(|_|{let ready=caller.admit_decoder_step(decoder,grant);(ready,caller.receipt())}).collect()
}
fn teardown(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){
 let steps=semio_framework_job::ORIGINAL_DECODER_TEARDOWN_STEPS;
 for _ in 0..steps{let(quote,_)=semio_framework_job::original_decoder_teardown_quote(decoder).unwrap();caller.turn(quote,|cx|{decoder.close_on_source(cx)?;Ok(())}).unwrap();}
 assert!(decoder.terminal_is_empty(),"decoder teardown exceeded its {steps} quoted steps");
}
#[test]
fn cold_decoder_prerequisites_are_all_or_nothing_at_their_quote(){
 let(prerequisite,fuel)=semio_framework_job::original_decoder_prerequisite_quote().unwrap();
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(119),Generation(49),grant(),clock,u64::MAX,10000);let mut decoder=OriginalJobDecodeOwner::empty();
 let starved=funded_prerequisites(&mut caller,&mut decoder,below(prerequisite),8);
 assert!(starved.iter().all(|(ready,_)|!ready));assert!(starved[3..].iter().all(|(_,receipt)|*receipt==RetainedCloneProgress::default()));
 teardown(&mut caller,&mut decoder);caller.retire(grant());
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(120),Generation(49),grant(),clock,u64::MAX,10000);let mut decoder=OriginalJobDecodeOwner::empty();
 let funded=funded_prerequisites(&mut caller,&mut decoder,prerequisite,4);
 assert_eq!(funded.iter().position(|(ready,_)|*ready),Some(3));assert!(funded.iter().all(|(_,receipt)|receipt.fits(prerequisite)));assert!(fuel>0);
 teardown(&mut caller,&mut decoder);caller.retire(grant());
}
#[test]
fn settled_construction_admits_exactly_its_turn_quote_and_refuses_one_below_without_state_change(){
 let demand=semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:0,capacity_bytes:1,release_bytes:0,depth:1};
 let(quote,_)=semio_framework_job::original_construction_settled_turn_quote::<Box<u8>>(demand,1).unwrap();
 let(prerequisite,_)=semio_framework_job::original_decoder_prerequisite_quote().unwrap();
 let attempt=|turn_grant:RetainedCloneGrant|{
  let operation=Operation::new(OperationId(118),RevisionId(0),Generation(48),0);let mut caller=OriginalJobFixtureCaller::admit(operation.operation,operation.generation,grant(),clock,u64::MAX,10000);let mut decoder=OriginalJobDecodeOwner::empty();
  assert!(funded_prerequisites(&mut caller,&mut decoder,prerequisite,4).last().unwrap().0);
  let mut output:Option<Box<u8>>=None;let mut admission=Admission::default();
  let(settled,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(turn_grant,&mut decoder,|cx|admit_original_construction_settled(cx,&mut admission,&mut output,demand,1,warm_constructor)));
  let state=(output.is_some(),admission.terminal_is_empty());if !settled.as_ref().is_ok_and(|done|*done){assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0))}
  if settled.is_ok(){output.take();}teardown(&mut caller,&mut decoder);caller.retire(grant());(settled,state)
 };
 let(settled,state)=attempt(quote);assert!(settled.unwrap());assert_eq!(state,(true,false));
 for starved in[below(quote),RetainedCloneGrant{maximum_copy_bytes:quote.maximum_copy_bytes-1,..quote}]{let(settled,state)=attempt(starved);assert!(!settled.unwrap());assert_eq!(state,(false,true))}
}
