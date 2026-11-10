use super::*;
use crate::allocation_testing::observe_retirement_allocations as observe;
use semio_framework_value_control_contract::{OriginalEffectAuthority,OriginalEffectProducer,OriginalEffectReply,OriginalEffectReplyComplete,OriginalSourceTurnIdentity,RetainedAllocationEffect,RetainedWorkPoll};
use std::marker::PhantomData;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};

fn bank()->EffectBank{let law:serde_json::Value=serde_json::from_str(include_str!("../🎟️work/📐️.json")).unwrap();EffectBank::new(Wallet::from_profile(&law["stock"]))}
fn run<E>(bank:&mut EffectBank,grant:RetainedCloneGrant,work:u64,slot:&mut Option<Result<RetainedCloneStep,E>>,call:impl for<'a,'r> FnOnce(&mut RetainedAllocationEffect<'a>,OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>,error_progress:impl FnOnce(&E)->RetainedCloneProgress)->(RetainedCloneProgress,u128,(usize,usize)){
 let((poll,actual),heap)=observe(||bank.run(grant,work,|effect|{let before=effect.performed_work();let complete=call(effect,OriginalEffectReply::borrow(slot).unwrap()).unwrap();assert!(complete.is_recorded());RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work:effect.performed_work()-before})}));
 let RetainedWorkPoll::Complete(turn)=poll else{panic!("original vector turn remained pending")};turn.result.unwrap();
 let progress=match slot.as_ref().unwrap(){Ok(step)=>step.progress(),Err(error)=>error_progress(error)};
 if let Some(actual)=actual{assert_eq!(actual,heap.0)}assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));let _=bank.take_admitted();bank.post(progress,grant,heap);(progress,turn.work,heap)
}
fn clear_reply<E>(bank:&mut EffectBank,slot:&mut Option<Result<RetainedCloneStep,E>>){let bytes=size_of::<Option<Result<RetainedCloneStep,E>>>();let grant=bank.loan(1,bytes);let((poll,actual),heap)=observe(||bank.run(grant,1,|effect|{assert!(effect.performed(1).unwrap());drop(slot.take());RetainedWorkPoll::Complete(RetainedWorkOutcome::<RetainedCloneProgress,ValueError>{result:Ok(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}),work:1})}));let RetainedWorkPoll::Complete(turn)=poll else{panic!("reply header clear pending")};assert_eq!(actual,None);let _=bank.take_admitted();bank.post(turn.result.unwrap(),grant,heap)}
type Failure=crate::OriginalRetirementFailure;
struct VectorDriver<T:RetireOwned>(PhantomData<T>);
impl<T:RetireOwned> OriginalEffectProducer for VectorDriver<T>{type Output=RetainedCloneStep;type Failure=Failure;type Context<'a,'b>=&'a mut OriginalVectorCursor<T>where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{None}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,Failure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,Failure>,ValueError>where 'b:'a{c.close_into(4096,e,r)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,Failure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,Failure>,ValueError>where 'b:'a{c.recover_close_into(4096,e,r)}
}
struct RetireDriver<T:RetireOwned>(PhantomData<T>);
impl<T:RetireOwned> OriginalEffectProducer for RetireDriver<T>{type Output=RetainedCloneStep;type Failure=Failure;type Context<'a,'b>=&'a mut ControlledRetirement<T>where 'b:'a;
 fn validate_context<'a,'b>(&self,_:&Self::Context<'a,'b>)->Result<(),ValueError>where 'b:'a{Ok(())}
 fn source_turn<'a,'b>(&self,_:&Self::Context<'a,'b>)->Option<OriginalSourceTurnIdentity>where 'b:'a{None}
 fn receive<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,Failure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,Failure>,ValueError>where 'b:'a{c.step_into(4096,e,r)}
 fn recover<'r,'a,'b>(&mut self,c:&mut Self::Context<'a,'b>,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetainedCloneStep,Failure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,Failure>,ValueError>where 'b:'a{c.recover_step_into(4096,e,r)}
}
fn driven<'a,'b,P>(bank:&mut EffectBank,context:&mut P::Context<'a,'b>,producer:P,g:RetainedCloneGrant,units:u64)->(RetainedCloneProgress,u128,(usize,usize))where 'b:'a,P:OriginalEffectProducer<Output=RetainedCloneStep,Failure=Failure>{
 let mut authority=OriginalEffectAuthority::new(producer);let before_work=*bank.wallet().original_work();let(mut action,mut output)=(None,None);let mut heap=(0usize,0usize);let mut done=false;
 for _ in 0..128{let((poll,actual),observed)=observe(||bank.run(g,units,|effect|{let before=effect.performed_boundary();let ready=authority.drive(context,effect,&mut action).unwrap();let work=effect.performed_since(before).unwrap();if ready{RetainedWorkPoll::Complete(RetainedWorkOutcome::<(),ValueError>{result:Ok(()),work})}else{RetainedWorkPoll::Pending{work}}}));heap.0+=observed.0;heap.1+=observed.1;if let Some(actual)=actual{assert_eq!(actual,heap.0)}if matches!(poll,RetainedWorkPoll::Pending{..}){continue}done=true;break}
 assert!(done,"original vector driven action exceeded its finite bound");
 let progress=match action.as_ref().unwrap().recorded_reply().unwrap(){Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};assert!(progress.fits(g));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));let _=bank.take_admitted();bank.post(progress,g,heap);
 let work=before_work-*bank.wallet().original_work();
 let demand=OriginalEffectAuthority::<P>::transfer_reply_demand().unwrap();let tg=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_depth:demand.depth,..Default::default()};
 let((poll,_),theap)=observe(||bank.run(tg,1,|effect|RetainedWorkPoll::Complete(authority.transfer_reply_into(context,&mut action,&mut output,effect))));let RetainedWorkPoll::Complete(turn)=poll else{panic!("original vector reply transfer pending")};let _=bank.take_admitted();bank.post(turn.result.unwrap().unwrap(),tg,theap);
 drop(output);assert!(authority.into_producer().is_ok());(progress,work,heap)
}
fn vstep<T:RetireOwned>(bank:&mut EffectBank,owner:&mut OriginalVectorCursor<T>,g:RetainedCloneGrant,units:u64)->(RetainedCloneProgress,u128,(usize,usize)){let mut c=owner;driven(bank,&mut c,VectorDriver::<T>(PhantomData),g,units)}
fn cstep<T:RetireOwned>(bank:&mut EffectBank,cursor:&mut ControlledRetirement<T>,g:RetainedCloneGrant,units:u64)->(RetainedCloneProgress,u128,(usize,usize)){let mut c=cursor;driven(bank,&mut c,RetireDriver::<T>(PhantomData),g,units)}
fn retire_value<T:RetireOwned>(bank:&mut EffectBank,value:T,limit:u64)->(usize,usize){
 let mut cursor=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("native custody"));let(mut births,mut releases)=(0,0);
 for _ in 0..limit{if cursor.terminal_is_empty(){break}let work=cursor.next_computational_work(4096).unwrap().max(1);let g=bank.loan(1,4096);let(p,_,heap)=cstep(bank,&mut cursor,g,work);assert!(p.fits(g));births+=heap.0;releases+=heap.1;}
 assert!(cursor.terminal_is_empty());let(_,heap)=observe(||drop(cursor));assert_eq!(heap,(0,0));(births,releases)
}

#[test]
fn original_vector_native_ordered_handoffs_cancel_every_original_frontier_and_pay_final_backing(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for stop in law["cancelAfter"].as_array().unwrap(){
  let mut bank=bank();
  let(source,birth)=observe(||{let mut values=Vec::with_capacity(37);for value in law["values"].as_array().unwrap(){values.push(value.as_str().unwrap().repeat(8192));}values});
  let pointers:Vec<_>=source.iter().map(|value|value.as_ptr()).collect();let backing=source.as_ptr();let backing_bytes=source.capacity()*size_of::<String>();
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:262144,maximum_depth:64};
  let(mut input,mut owner,mut reply)=(Some(source),None,None);
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(p,_,heap)=run(&mut bank,denied,1,&mut reply,|e,r|OriginalVectorCursor::admit_into(&mut input,&mut owner,e,r),ValueError::retained_progress);assert_eq!(p,Default::default());assert_eq!(heap,(0,0));assert!(owner.is_none());assert_eq!(input.as_ref().unwrap().as_ptr(),backing);assert_eq!(input.as_ref().unwrap()[0].as_ptr(),pointers[0]);clear_reply(&mut bank,&mut reply);}
  let(p,_,heap)=run(&mut bank,grant,1,&mut reply,|e,r|OriginalVectorCursor::admit_into(&mut input,&mut owner,e,r),ValueError::retained_progress);assert_eq!(heap,(0,0));assert!(p.fits(grant));assert_eq!(owner.as_ref().unwrap().remaining().as_ptr(),backing);clear_reply(&mut bank,&mut reply);
  let(mut returned,mut row)=(Vec::new(),None);
  for index in 0..stop.as_u64().unwrap()as usize{
   let(p,work,heap)=run(&mut bank,RetainedCloneGrant{maximum_items:0,..grant},1,&mut reply,|e,r|owner.as_mut().unwrap().take_front_into(&mut row,e,r),ValueError::retained_progress);assert_eq!((p,work,heap),(Default::default(),0,(0,0)));assert!(row.is_none());assert_eq!(owner.as_ref().unwrap().remaining()[0].as_ptr(),pointers[index]);clear_reply(&mut bank,&mut reply);
   let(p,_,heap)=run(&mut bank,grant,1,&mut reply,|e,r|owner.as_mut().unwrap().take_front_into(&mut row,e,r),ValueError::retained_progress);assert_eq!(heap,(0,0));assert!(p.fits(grant));let value=row.take().unwrap();assert_eq!(value.as_ptr(),pointers[index]);returned.push(value);clear_reply(&mut bank,&mut reply);
  }
  run(&mut bank,grant,1,&mut reply,|e,r|owner.as_mut().unwrap().begin_close_into(e,r),ValueError::retained_progress);clear_reply(&mut bank,&mut reply);
  if owner.as_ref().unwrap().remaining().is_empty(){let(p,_,heap)=vstep(&mut bank,owner.as_mut().unwrap(),RetainedCloneGrant{maximum_release_bytes:backing_bytes-1,..grant},1);assert_eq!(p,Default::default());assert_eq!(heap,(0,0));assert!(!owner.as_ref().unwrap().terminal_is_empty());}
  let(mut births,mut releases)=(0,0);
  for _ in 0..100000{
   if owner.as_ref().unwrap().terminal_is_empty(){break}
   let demand=owner.as_ref().unwrap().next_demand(4096).unwrap();assert!(demand.depth<=grant.maximum_depth);let work=owner.as_ref().unwrap().next_computational_work(4096).unwrap().max(1);
   let denied=RetainedCloneGrant{maximum_release_bytes:0,maximum_capacity_bytes:0,maximum_copy_bytes:0,maximum_depth:0,..grant};
   let(p,_,heap)=vstep(&mut bank,owner.as_mut().unwrap(),denied,work);assert_eq!(p,Default::default());assert_eq!(heap,(0,0));
   let(p,_,heap)=vstep(&mut bank,owner.as_mut().unwrap(),grant,work);assert!(p.fits(grant));births+=heap.0;releases+=heap.1;
  }
  assert!(owner.as_ref().unwrap().terminal_is_empty());let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));
  for value in returned{let(b,r)=retire_value(&mut bank,value,1000);births+=b;releases+=r;}
  assert_eq!(releases,birth.0+births);assert!(releases>=backing_bytes);
 }
}

#[test]
fn original_vector_actual_backing_release_is_independent_of_cached_completion(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let mut bank=bank();
 let source=Vec::<u64>::with_capacity(2);
 let bytes=source.capacity()*size_of::<u64>();
 let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:bytes,maximum_depth:64};
 let(mut input,mut owner,mut reply)=(Some(source),None,None);
 run(&mut bank,g,1,&mut reply,|e,r|OriginalVectorCursor::admit_into(&mut input,&mut owner,e,r),ValueError::retained_progress);clear_reply(&mut bank,&mut reply);
 run(&mut bank,g,1,&mut reply,|e,r|owner.as_mut().unwrap().begin_close_into(e,r),ValueError::retained_progress);clear_reply(&mut bank,&mut reply);
 let mut actual=Vec::new();
 for(label,grant)in[("deniedRelease",RetainedCloneGrant{maximum_release_bytes:bytes-1,..g}),("paidBackingRelease",g),("cachedComplete",g)]{
  let(p,work,heap)=vstep(&mut bank,owner.as_mut().unwrap(),grant,1);
  assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));
  actual.push(serde_json::json!({"label":label,"work":work as u64}));
 }
 assert_eq!(serde_json::Value::Array(actual),law["actualCloseWork"]);
 assert!(owner.as_ref().unwrap().terminal_is_empty());
}

#[test]
fn actual_original_vector_normal_moves_preserve_input_identity_with_separate_cpu_and_exact_quotes(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🎟️work/📐️.json")).unwrap();
 let mut bank=bank();let values:Vec<String>=serde_json::from_value(law["values"].clone()).unwrap();let pointer=values.as_ptr();let text=values[0].as_ptr();
 let(mut input,mut owner,mut reply,mut row)=(Some(values),None,None,None);
 let d=OriginalVectorCursor::<String>::constructor_demand();let g=bank.loan(1,d.copy_bytes);
 let(p,work,heap)=run(&mut bank,RetainedCloneGrant{maximum_items:0,..g},1,&mut reply,|e,r|OriginalVectorCursor::admit_into(&mut input,&mut owner,e,r),ValueError::retained_progress);assert_eq!((p,work,heap),(Default::default(),0,(0,0)));assert_eq!(input.as_ref().unwrap().as_ptr(),pointer);assert_eq!(input.as_ref().unwrap()[0].as_ptr(),text);clear_reply(&mut bank,&mut reply);
 let(p,work,heap)=run(&mut bank,g,1,&mut reply,|e,r|OriginalVectorCursor::admit_into(&mut input,&mut owner,e,r),ValueError::retained_progress);assert_eq!(work,1);assert_eq!(p.copied_bytes,d.copy_bytes);assert_eq!(heap,(0,0));assert_eq!(owner.as_ref().unwrap().remaining().as_ptr(),pointer);clear_reply(&mut bank,&mut reply);
 let d=owner.as_ref().unwrap().next_take_demand();let g=bank.loan(1,d.copy_bytes);
 let(p,work,_)=run(&mut bank,RetainedCloneGrant{maximum_items:0,..g},1,&mut reply,|e,r|owner.as_mut().unwrap().take_front_into(&mut row,e,r),ValueError::retained_progress);assert_eq!((p,work),(Default::default(),0));assert!(row.is_none());clear_reply(&mut bank,&mut reply);
 let(p,work,heap)=run(&mut bank,g,1,&mut reply,|e,r|owner.as_mut().unwrap().take_front_into(&mut row,e,r),ValueError::retained_progress);assert_eq!(work,1);assert_eq!(p.copied_bytes,d.copy_bytes);assert_eq!(heap,(0,0));clear_reply(&mut bank,&mut reply);
 let value=row.take().unwrap();assert_eq!(value.as_ptr(),text);assert_eq!(serde_json::to_value(&value).unwrap(),law["values"][0]);
 retire_value(&mut bank,value,law["stock"]["maximumTurns"].as_u64().unwrap());
 let g=bank.loan(1,4096);run(&mut bank,g,1,&mut reply,|e,r|owner.as_mut().unwrap().begin_close_into(e,r),ValueError::retained_progress);clear_reply(&mut bank,&mut reply);
 for _ in 0..law["stock"]["maximumTurns"].as_u64().unwrap(){if owner.as_ref().unwrap().terminal_is_empty(){break}let work=owner.as_ref().unwrap().next_computational_work(4096).unwrap().max(1);let g=bank.loan(1,4096);let(p,_,_)=vstep(&mut bank,owner.as_mut().unwrap(),g,work);assert!(p.fits(g));}
 assert!(owner.as_ref().unwrap().terminal_is_empty());
 let(_,work,_)=vstep(&mut bank,owner.as_mut().unwrap(),Default::default(),1);assert_eq!(work,0);
 let(p,work,_)=run(&mut bank,Default::default(),1,&mut reply,|e,r|owner.as_mut().unwrap().take_front_into(&mut row,e,r),ValueError::retained_progress);assert_eq!((p,work),(Default::default(),0));assert!(row.is_none());
}
