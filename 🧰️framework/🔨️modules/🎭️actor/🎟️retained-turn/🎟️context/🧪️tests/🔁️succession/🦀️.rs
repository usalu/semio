//! 🧪️ One actor Source outlives its finite stocks only through verified succession; its identity and resident owners never change.
use super::*;
use semio_framework_job::{OriginalCallerReturnPlace,OriginalSourceBinding};
fn law()->serde_json::Value{serde_json::from_str(include_str!("../../../🔁️succession/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
fn clock()->Option<u64>{Some(1)}
fn other_clock()->Option<u64>{Some(2)}
fn stock(law:&serde_json::Value)->(RetainedCloneGrant,u64){(serde_json::from_value(law["stock"]["grant"].clone()).unwrap(),law["stock"]["fuel"].as_u64().unwrap())}
fn scope(input:RetainedTurnInput)->StepBudget{StepBudget::new(input.work.fuel,input.work.absolute_deadline_us,input.grant)}
fn next(input:&mut RetainedTurnInput,spent:RetainedCloneProgress,work:u64){let returned=input.return_original(spent,work).unwrap();input.epoch+=1;input.grant=returned.remaining;input.work.fuel=returned.remaining_fuel;}
fn admitted(law:&serde_json::Value)->(OriginalActorContextSlot,RetainedTurnInput){
 let(grant,fuel)=stock(law);let source=&law["source"];let mut input=RetainedTurnInput{operation:source["operation"].as_u64().unwrap(),generation:source["generation"].as_u64().unwrap(),epoch:1,work:OriginalActorTurnWork{clock_binding:source["clockBinding"].as_u64().unwrap(),absolute_deadline_us:source["deadlineUs"].as_u64().unwrap(),fuel},grant};
 let mut slot=OriginalActorContextSlot::empty();
 for _ in 0..64{let call=slot.admit_step(input,scope(input),clock);let ready=call.result.unwrap().ready;next(&mut input,call.receipt_spent,call.work_units_spent);if ready{return(slot,input)}}
 panic!("context admission stalled")
}
fn succession(law:&serde_json::Value,input:RetainedTurnInput,successor:u64)->OriginalActorSuccession{let(grant,fuel)=stock(law);OriginalActorSuccession{operation:input.operation,generation:input.generation,epoch:input.epoch,successor,grant,fuel}}
fn renewed(law:&serde_json::Value,input:RetainedTurnInput)->RetainedTurnInput{let(grant,fuel)=stock(law);RetainedTurnInput{epoch:input.epoch+1,grant,work:OriginalActorTurnWork{fuel,..input.work},..input}}
fn close(mut slot:OriginalActorContextSlot,mut input:RetainedTurnInput){for _ in 0..1024{if slot.terminal_is_empty(){return}let call=slot.close_step(input,scope(input));assert!(call.result.is_ok(),"{:?}",call.result);next(&mut input,call.receipt_spent,call.work_units_spent);}panic!("context close stalled")}
fn spend(slot:&mut OriginalActorContextSlot,input:&mut RetainedTurnInput,progress:RetainedCloneProgress,work:u64)->Result<(),OriginalActorCallError<OriginalTurnFailure>>{
 let mut spent=RetainedCloneProgress::default();let call=slot.dispatch_step(*input,scope(*input),&mut spent,clock,|cx|->Result<(),OriginalTurnFailure>{cx.consume_retained(progress)?;cx.consume_fuel(work);Ok(())});let outcome=call.result;next(input,call.receipt_spent,call.work_units_spent);outcome
}

#[test]
fn succession_admits_only_the_exact_predecessor_turn_and_an_unused_kernel_identity(){
 let law=law();
 for row in law["admissions"].as_array().unwrap(){
  let(mut slot,input)=admitted(&law);let mut offered=succession(&law,input,input.operation+1);let change=&row["change"];let field=|name:&str|change.get(name).and_then(serde_json::Value::as_u64);
  if let Some(value)=field("operation"){offered.operation=value}if let Some(value)=field("generation"){offered.generation=value}if let Some(value)=field("epoch"){offered.epoch=value}if let Some(value)=field("successor"){offered.successor=value}if let Some(value)=field("fuel"){offered.fuel=value}if let Some(value)=field("maximumItems"){offered.grant.maximum_items=value as usize}if let Some(value)=field("maximumDepth"){offered.grant.maximum_depth=value as usize}
  let before=slot.context.as_ref().unwrap().remaining_grant().unwrap();let call=slot.succeed_step(input,scope(input),clock,offered);
  assert_eq!(call.result.as_ref().is_ok_and(|admission|admission.ready),row["accepted"].as_bool().unwrap(),"{}",row["name"]);
  if row["accepted"]==false{assert_eq!(call.result.unwrap_err().primary_value().unwrap().kind,ValueRefusalKind::InvariantViolated);assert_eq!((call.receipt_spent,call.work_units_spent),(Default::default(),0));assert_eq!(slot.context.as_ref().unwrap().remaining_grant().unwrap(),before);assert_eq!(slot.tenure(),Some(input.operation));let mut input=input;spend(&mut slot,&mut input,RetainedCloneProgress{copied_items:1,..Default::default()},1).unwrap();close(slot,input)}
  else{let receipt=call.receipt().unwrap();assert_eq!(receipt.spent,RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<OriginalActorSuccession>(),..Default::default()});assert_eq!(receipt.work_units_spent,1);assert_eq!(slot.tenure(),Some(offered.successor));close(slot,renewed(&law,input))}
 }
}

#[test]
fn an_unnamed_renewal_stays_refused_and_every_succession_is_used_once(){
 let law=law();let(mut slot,mut input)=admitted(&law);
 spend(&mut slot,&mut input,RetainedCloneProgress{copied_items:1,copied_bytes:4096,..Default::default()},7).unwrap();
 let forged=renewed(&law,input);let mut spent=RetainedCloneProgress::default();let call=slot.dispatch_step(forged,scope(forged),&mut spent,clock,|_|Ok::<(),OriginalTurnFailure>(()));assert!(matches!(call.result,Err(OriginalActorCallError::Authority(_))));assert_eq!(spent,Default::default());
 let offered=succession(&law,input,90);assert!(slot.succeed_step(input,scope(input),other_clock,offered).result.is_err());
 let wider=StepBudget::new(input.work.fuel,input.work.absolute_deadline_us,RetainedCloneGrant{maximum_items:input.grant.maximum_items+1,..input.grant});assert!(slot.succeed_step(input,wider,clock,offered).result.is_err());
 assert!(slot.succeed_step(input,scope(input),clock,offered).result.unwrap().ready);
 assert!(slot.succeed_step(input,scope(input),clock,offered).result.is_err(),"the predecessor turn is spent");
 let mut current=renewed(&law,input);let repeated=slot.succeed_step(current,scope(current),clock,succession(&law,current,90));assert!(repeated.result.is_err(),"a tenure is admitted once");
 let mut stale=current;stale.grant.maximum_items+=1;let mut spent=RetainedCloneProgress::default();assert!(matches!(slot.dispatch_step(stale,scope(stale),&mut spent,clock,|_|Ok::<(),OriginalTurnFailure>(())).result,Err(OriginalActorCallError::Authority(_))));
 for moved in[RetainedTurnInput{work:OriginalActorTurnWork{absolute_deadline_us:current.work.absolute_deadline_us+1,..current.work},..current},RetainedTurnInput{work:OriginalActorTurnWork{clock_binding:current.work.clock_binding+1,..current.work},..current},RetainedTurnInput{operation:90,..current}]{let mut spent=RetainedCloneProgress::default();assert!(matches!(slot.dispatch_step(moved,scope(moved),&mut spent,clock,|_|Ok::<(),OriginalTurnFailure>(())).result,Err(OriginalActorCallError::Authority(_))))}
 spend(&mut slot,&mut current,RetainedCloneProgress{copied_items:1,..Default::default()},1).unwrap();close(slot,current);
}

#[test]
fn a_predecessor_that_cannot_pay_keeps_its_stock_and_its_tenure(){
 let law=law();let(mut slot,mut input)=admitted(&law);let offered=succession(&law,input,77);
 for budget in[StepBudget::new(0,input.work.absolute_deadline_us,input.grant),StepBudget::new(input.work.fuel,1,input.grant),StepBudget::new(input.work.fuel,input.work.absolute_deadline_us,RetainedCloneGrant{maximum_items:0,..input.grant}),StepBudget::new(input.work.fuel,input.work.absolute_deadline_us,RetainedCloneGrant{maximum_copy_bytes:std::mem::size_of::<OriginalActorSuccession>()-1,..input.grant}),StepBudget::new(input.work.fuel,input.work.absolute_deadline_us,RetainedCloneGrant{maximum_depth:0,..input.grant})]{let call=slot.succeed_step(input,budget,clock,offered);assert!(!call.result.unwrap().ready);assert_eq!((call.receipt_spent,call.work_units_spent),(Default::default(),0));assert_eq!(slot.tenure(),Some(input.operation));}
 let cold=OriginalActorContextSlot::empty().succeed_step(input,scope(input),clock,offered);assert!(cold.result.is_err());
 spend(&mut slot,&mut input,RetainedCloneProgress{copied_items:1,..Default::default()},1).unwrap();close(slot,input);
}

#[test]
fn an_actor_runs_past_several_exhausted_stocks_and_keeps_its_resident_owners_and_every_command(){
 let law=law();let(mut slot,mut input)=admitted(&law);let run=&law["run"];let stocks=run["stocks"].as_u64().unwrap();let row=&run["command"];let field=|name:&str|row[name].as_u64().unwrap();
 let command=RetainedCloneProgress{copied_items:field("copiedItems")as usize,copied_bytes:field("copiedBytes")as usize,retained_capacity_bytes:field("retainedCapacityBytes")as usize,released_bytes:field("releasedBytes")as usize};let work=field("work");
 let source=(input.operation,input.generation,input.work.clock_binding,input.work.absolute_deadline_us);let mut binding:Option<OriginalSourceBinding>=None;let mut document=Vec::new();let mut cell=0u64;let address=&mut cell as*mut u64 as usize;let mut place=OriginalCallerReturnPlace::empty();let(total,fuel)=stock(&law);let mut tenures=vec![slot.tenure().unwrap()];let mut copied=0usize;let mut issued=0u64;
 let low=|input:&RetainedTurnInput|input.grant.maximum_items<total.maximum_items/4||input.grant.maximum_copy_bytes<total.maximum_copy_bytes/4||input.grant.maximum_capacity_bytes<total.maximum_capacity_bytes/4||input.grant.maximum_release_bytes<total.maximum_release_bytes/4||input.work.fuel<fuel/4;
 while (tenures.len()as u64)<stocks||!low(&input){
  issued+=1;assert!(issued<200_000,"the run never exhausted its stocks");
  if low(&input){
   let offered=succession(&law,input,tenures.last().unwrap()+10);let call=slot.succeed_step(input,scope(input),clock,offered);assert!(call.result.as_ref().unwrap().ready);assert_eq!(call.receipt().unwrap().input,input);
   assert_eq!(slot.context.as_ref().unwrap().remaining_grant().unwrap(),total);assert_eq!(slot.context.as_ref().unwrap().remaining_work().unwrap().fuel,fuel);tenures.push(slot.tenure().unwrap());input=renewed(&law,input);continue
  }
  let mut spent=RetainedCloneProgress::default();let call=slot.dispatch_step(input,scope(input),&mut spent,clock,|cx|->Result<bool,OriginalTurnFailure>{
   if !place.is_admitted(){place.admit_sized(cx,address,8)?;return Ok(false)}
   if let Some(original)=binding{assert!(original.matches(cx),"a resident owner keeps its Source across stocks")}else{binding=Some(OriginalSourceBinding::borrow(cx))}
   cx.consume_retained(command)?;cx.consume_fuel(work);Ok(true)
  });
  if call.result.unwrap(){document.push(issued);}copied+=call.receipt_spent.copied_bytes;assert_eq!((input.operation,input.generation,input.work.clock_binding,input.work.absolute_deadline_us),source);next(&mut input,call.receipt_spent,call.work_units_spent);
 }
 assert_eq!(tenures,(0..stocks).map(|index|source.0+index*10).collect::<Vec<_>>());assert!(copied>0&&document.len()as u64>=100*stocks,"{copied} copied by {} commands",document.len());assert!(document.windows(2).all(|pair|pair[0]<pair[1]));assert!(place.is_admitted());
 let mut preissued=false;for _ in 0..64{if place.terminal_is_empty(){break}let mut spent=RetainedCloneProgress::default();let call=slot.dispatch_step(input,scope(input),&mut spent,clock,|cx|->Result<(),OriginalTurnFailure>{if !preissued{preissued=place.preissue_capture(cx,address,8)?;if preissued{cell=7}}else{place.settle_return(cx)?;}Ok(())});assert!(call.result.is_ok());next(&mut input,call.receipt_spent,call.work_units_spent)}
 assert!(place.terminal_is_empty(),"the owner admitted under the first stock settles under the last");assert_eq!(cell,7);close(slot,input);
}
