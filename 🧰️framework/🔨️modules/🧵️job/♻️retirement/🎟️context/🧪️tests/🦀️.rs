//! 🔬️ One genuine finite caller preserves physical custody, direct work and complete refusals.
use semio_framework_value_testing_allocator as _;
use semio_framework_job::{close_original_quoted_retained_step,close_original_effect_step,close_original_granted_step,close_original_nested_step,close_original_retained_step,InteractiveJobCloseStep,OriginalJobFixtureCaller,RetainedCloneGrant,RetainedCloneProgress,Generation,OperationId,StepBudget};
use semio_framework_value::{RetainedWorkOutcome,ValueError,ValueRefusalKind,retirement::controlled::ControlledRetirement};
fn clock()->Option<u64>{Some(1)}
fn bank()->RetainedCloneGrant{let row:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let s=&row["originalLifetime"];RetainedCloneGrant{maximum_items:s["items"].as_u64().unwrap()as usize,maximum_copy_bytes:s["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:s["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:s["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:s["depth"].as_u64().unwrap()as usize}}
fn caller()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),bank(),clock,12345,100000)}
fn delta(a:RetainedCloneProgress,b:RetainedCloneProgress)->RetainedCloneProgress{RetainedCloneProgress{copied_items:a.copied_items-b.copied_items,copied_bytes:a.copied_bytes-b.copied_bytes,retained_capacity_bytes:a.retained_capacity_bytes-b.retained_capacity_bytes,released_bytes:a.released_bytes-b.released_bytes}}
#[test]
fn original_close_context_denied_work_and_deadline_preserve_owned_source(){
 let source=vec![7u8;1799];let pointer=source.as_ptr();let mut owner=ControlledRetirement::retain(source);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:512,maximum_capacity_bytes:262144,maximum_release_bytes:262144,maximum_depth:128};let mut caller=caller();
 for budget in [StepBudget::new(0,12345,grant),StepBudget::new(1,0,grant)]{caller.turn(grant,|cx|{let before=cx.retained_progress();cx.with_original_work_loan(budget.fuel,budget.deadline_us,|cx|{let body=cx.retained_grant().maximum_copy_bytes;let demand=owner.selected_demand(body)?;let(result,heap)=observe_heap(||close_original_effect_step(cx,demand,|effect|owner.step(body,effect)));assert_eq!(result?.progress(),Default::default());assert_eq!(cx.retained_progress(),before);assert_eq!(owner.original().unwrap().as_ptr(),pointer);assert_eq!((heap.0,heap.1),(0,0));Ok(())})}).unwrap();}
 let mut freed=0;let mut born=0;caller.cancel().cancel_now();
 for turn in 0..100000{if owner.terminal_is_empty(){break}caller.turn(grant,|cx|{cx.with_original_work_loan(1,12345,|cx|{assert!(cx.is_cancelled());let before=cx.retained_progress();let body=cx.retained_grant().maximum_copy_bytes;let demand=owner.selected_demand(body)?;let(result,heap)=observe_heap(||close_original_effect_step(cx,demand,|effect|owner.step(body,effect)));let step=result?;let actual=delta(cx.retained_progress(),before);assert_eq!(step.progress(),actual);assert!(actual.fits(grant));assert_eq!((heap.0,heap.1),(actual.retained_capacity_bytes,actual.released_bytes));assert!(cx.original_work_units_spent()<=1);born+=heap.0;freed+=heap.1;Ok(())})}).unwrap();assert!(turn<99999,"same original source close did not complete");}assert!(owner.terminal_is_empty());assert_eq!(1799+born,freed);let(_,heap)=observe_heap(||drop(owner));assert_eq!((heap.0,heap.1),(0,0));caller.retire(bank());
}
#[test]
fn granted_close_body_is_debited_once_and_its_refusal_keeps_the_whole_error(){
 let grant=RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:64,maximum_capacity_bytes:262144,maximum_release_bytes:262144,maximum_depth:128};let mut caller=caller();let mut buffer=Some(vec![0u8;8]);
 caller.turn(grant,|cx|{cx.with_original_work_loan(3,12345,|cx|{
  let before=cx.retained_progress();let progress=RetainedCloneProgress{copied_items:1,released_bytes:8,..Default::default()};let mut seen=0;let(step,heap)=observe_heap(||close_original_granted_step(cx,|granted|{seen=granted.maximum_items;drop(buffer.take());RetainedWorkOutcome::<_,ValueError>{result:Ok(InteractiveJobCloseStep::Pending{progress}),work:1}}));let step=step?;assert_eq!((heap.0,heap.1),(0,8));assert_eq!((step,seen,delta(cx.retained_progress(),before),cx.fuel_remaining()),(InteractiveJobCloseStep::Pending{progress},4,progress,2));
  let refused=RetainedCloneProgress{copied_items:1,..Default::default()};let error=ValueError::literal(ValueRefusalKind::InvariantViolated,"granted close refused").with_retained_progress(refused);let mut refused_seen=false;
  let failure=close_original_granted_step(cx,|granted|{seen=granted.maximum_items;refused_seen=true;RetainedWorkOutcome{result:Err(error),work:1}}).unwrap_err();assert!(refused_seen);assert_eq!((seen,failure.primary_value().map(|error|error.kind),failure.receipt_refusal.is_none(),cx.fuel_remaining()),(3,Some(ValueRefusalKind::InvariantViolated),true,1));assert_eq!(delta(cx.retained_progress(),before).copied_items,2);
  let blocked=close_original_granted_step(cx,|g|{assert!(g.maximum_depth>0);RetainedWorkOutcome::<_,ValueError>{result:Ok(InteractiveJobCloseStep::Blocked),work:1}})?;assert_eq!((blocked,cx.fuel_remaining()),(InteractiveJobCloseStep::Blocked,0));let mut ran=false;let yielded=close_original_granted_step(cx,|_|{ran=true;RetainedWorkOutcome::<_,ValueError>{result:Ok(InteractiveJobCloseStep::Blocked),work:0}})?;assert_eq!((yielded,ran,delta(cx.retained_progress(),before).copied_items),(InteractiveJobCloseStep::Pending{progress:Default::default()},false,2));Ok(())
 })}).unwrap();caller.retire(bank());
}
#[test]
fn nested_close_body_debits_only_the_remainder_its_nested_callers_did_not_post(){
 let grant=RetainedCloneGrant{maximum_items:4,maximum_copy_bytes:64,maximum_capacity_bytes:262144,maximum_release_bytes:262144,maximum_depth:128};let mut caller=caller();let mut buffers=Some((vec![0u8;8],vec![0u8;8]));
 caller.turn(grant,|cx|{cx.with_original_work_loan(3,12345,|cx|{let before=cx.retained_progress();let reported=RetainedCloneProgress{copied_items:2,released_bytes:16,..Default::default()};let posted=RetainedCloneProgress{copied_items:1,..Default::default()};let(step,heap)=observe_heap(||close_original_nested_step(cx,|nested,_|{nested.consume_retained(posted)?;drop(buffers.take());nested.consume_fuel(1);Ok(InteractiveJobCloseStep::Pending{progress:reported})}));assert_eq!((heap.0,heap.1),(0,16));assert_eq!((step?,delta(cx.retained_progress(),before),cx.fuel_remaining()),(InteractiveJobCloseStep::Pending{progress:reported},reported,2));let failure=close_original_nested_step(cx,|nested,_|{nested.consume_retained(posted)?;nested.consume_fuel(1);Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}).unwrap_err();assert_eq!(failure.primary_value().map(|error|error.kind),Some(ValueRefusalKind::InvariantViolated));Ok(())})}).unwrap();caller.retire(bank());
}
#[test]
fn genuine_close_context_lends_exactly_its_grant_and_returns_the_body_outcome(){
 let grant=RetainedCloneGrant{maximum_items:2,maximum_copy_bytes:3,maximum_capacity_bytes:5,maximum_release_bytes:7,maximum_depth:11};let mut caller=caller();
 let value=caller.turn(grant,|cx|{let lent=cx.retained_grant();assert_eq!((lent.maximum_items,lent.maximum_copy_bytes,lent.maximum_capacity_bytes,lent.maximum_release_bytes,lent.maximum_depth),(2,3,5,7,11));assert!(!cx.should_yield());Ok(41+1)}).unwrap();assert_eq!(value,42);
 let(first,second)=(caller.turn(grant,|cx|Ok(cx.retained_grant().maximum_depth)).unwrap(),caller.turn(grant,|cx|Ok(cx.retained_grant().maximum_depth)).unwrap());assert_eq!((first,second),(11,11));let failure=caller.turn(grant,|_|Err::<(),_>(ValueError::literal(ValueRefusalKind::InvariantViolated,"original body refused").into())).unwrap_err();assert_eq!(failure.primary_value().map(|error|error.kind),Some(ValueRefusalKind::InvariantViolated));caller.retire(bank());
}

#[test]
fn quoted_close_never_returns_zero_work_pending_forever(){
 let stock=bank();let fuel=1000u64;
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),stock,clock,12345,fuel);
 let advance=|called:&mut u32|{*called+=1;RetainedWorkOutcome{result:Ok::<_,semio_framework_value::ValueError>(semio_framework_value::RetainedCloneStep::Progress(RetainedCloneProgress::default())),work:1}};
 let mut called=0;
 for _ in 0..8{caller.turn(stock,|cx|{if called==0{close_original_quoted_retained_step(cx,semio_framework_value::RetirementDemand::default(),fuel+1,|_|advance(&mut called))?;}Ok(())}).unwrap();}
 assert_eq!(called,1,"a quote above the whole turn fuel must be admitted at the start of a turn");
 caller.turn(stock,|cx|{cx.consume_fuel(1);let before=called;close_original_quoted_retained_step(cx,semio_framework_value::RetirementDemand::default(),fuel,|_|advance(&mut called))?;assert_eq!(before,called,"a quote above the remaining fuel of a spent turn waits");Ok(())}).unwrap();
 let too_big=semio_framework_value::RetirementDemand{copy_bytes:stock.maximum_copy_bytes+1,..Default::default()};
 let mut fresh=OriginalJobFixtureCaller::admit(OperationId(72),Generation(3),stock,clock,12345,fuel);
 fresh.turn(stock,|cx|{assert!(cx.retained_turn_untouched());let before=called;let refused=close_original_quoted_retained_step(cx,too_big,1,|_|advance(&mut called));assert!(refused.is_err(),"a demand above the whole grant of an untouched turn is refused typed");assert_eq!(before,called);Ok(())}).unwrap();fresh.retire(stock);
 caller.retire(stock);
}
#[test]
fn effect_close_yields_on_a_narrowed_turn_that_cannot_fit_then_completes_on_a_wider_one(){
 let small=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:262144,maximum_release_bytes:262144,maximum_depth:128};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(73),Generation(3),bank(),clock,12345,1000);
 let mut owner=ControlledRetirement::retain(vec![7u8;1799]);let mut called=0u32;
 caller.turn(small,|cx|{let body=cx.retained_grant().maximum_copy_bytes;let demand=owner.selected_demand(body)?;assert!(demand.copy_bytes>small.maximum_copy_bytes);let step=close_original_effect_step(cx,demand,|effect|{called+=1;owner.step(body,effect)})?;assert_eq!(step.progress(),RetainedCloneProgress::default());Ok(())}).unwrap();
 assert_eq!(called,0,"a narrowed first turn that cannot fit yields without running the cursor");
 for turn in 0..10000{if owner.terminal_is_empty(){break}caller.turn(bank(),|cx|{let body=cx.retained_grant().maximum_copy_bytes;let demand=owner.selected_demand(body)?;close_original_effect_step(cx,demand,|effect|owner.step(body,effect))?;Ok(())}).unwrap();assert!(turn<9999,"a wider turn must complete the close");}
 assert!(owner.terminal_is_empty());
 caller.retire(bank());
}
#[test]
fn an_error_inside_the_effect_exchange_cancels_its_action_instead_of_leaving_it_live(){
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(75),Generation(3),bank(),clock,12345,1000);
 caller.turn(bank(),|cx|{
  let fuel=cx.fuel_remaining();let grant=cx.retained_grant();
  let failed=cx.with_original_effect(grant,|_,effect|{let _=effect.performed(fuel+1);});
  assert!(failed.is_err(),"work beyond the ceiling must be a typed error");
  let again=cx.with_original_effect(grant,|_,effect|effect.performed(1).unwrap())?;
  assert!(again.is_some(),"the failed exchange left no live action behind");Ok(())
 }).unwrap();
 caller.retire(bank());
}
fn observe_heap<T>(run:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);(value,(heap.requested_bytes,heap.released_bytes))}
