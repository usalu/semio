use semio_framework_job_context::{Generation,OperationId,RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_value_testing_allocator as _;
use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
use semio_framework_value::ValueError;
use semio_framework_job_native_encoding::{OriginalSourceEncodingContext,OriginalSourceEncodingOwner};
fn allocate_bytes(control:&mut semio_framework_value::native_encoding::NativeEncodeControl<'_>,output:&mut Option<Vec<u8>>,count:usize)->Result<(),ValueError>{
 if count==0{*output=Some(Vec::new());return Ok(())}assert!(control.prepare_work(1).unwrap());let mut performed=false;let mut actual=None;let mut progress=None;let settled=control.allocate_effect(count,&mut actual,&mut progress,&mut |actual,_|{performed=true;let mut bytes=Vec::<u8>::new();bytes.try_reserve_exact(count).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"fixture allocation refused"))?;let capacity=bytes.capacity();*output=Some(bytes);*actual=Some(capacity);Ok(true)});
 if performed{control.commit_work(1).unwrap()}else{control.cancel_work().unwrap()}let mut outcome=settled?.expect("fixture allocation settles in its own turn");assert!(progress.is_none());assert!(outcome.forwarded_quote.is_none());assert!(outcome.scoped_quote.is_none());assert!(outcome.extent_refusal.is_none());assert!(outcome.forwarded_refusal.is_none());assert!(outcome.scoped_refusal.is_none());match outcome.primary.take(){Some(error)=>Err(error),None=>Ok(())}
}
fn clock()->Option<u64>{Some(1)}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1048576,maximum_capacity_bytes:1048576,maximum_release_bytes:1048576,maximum_depth:128}}
fn source_owner(caller:&mut OriginalJobFixtureCaller,grant:RetainedCloneGrant)->OriginalSourceEncodingOwner{let mut owner=OriginalSourceEncodingOwner::new();for _ in 0..128{if caller.turn(grant,|cx|owner.admit_step(cx)).unwrap(){return owner}}panic!("original Source issuers did not admit")}
fn close_source_owner(caller:&mut OriginalJobFixtureCaller,owner:&mut OriginalSourceEncodingOwner,grant:RetainedCloneGrant){for _ in 0..128{if owner.terminal_is_empty(){return}caller.turn(grant,|cx|cx.with_original_retirement(|cx|owner.close_step(cx))).unwrap();}panic!("original Source issuer did not close")}
#[test]
fn actual_source_prices_nested_work_and_keeps_the_partial_receiver(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("cases.json")).unwrap();let grant=stock();
 for (index,row) in corpus["cases"].as_array().unwrap().iter().enumerate(){
  let chunks:Vec<Vec<u8>>=serde_json::from_value(row["chunks"].clone()).unwrap();let total=chunks.iter().map(Vec::len).sum::<usize>();let mut caller=OriginalJobFixtureCaller::admit(OperationId(710+index as u64),Generation(1),grant,clock,100000,100000);let mut output=None;let mut retirement=source_owner(&mut caller,grant);let retirement_id=retirement.retirement_identity();let before=caller.fuel_remaining();
  let policy=RetainedCloneGrant{maximum_copy_bytes:row["maximumCopy"].as_u64().unwrap()as usize,maximum_capacity_bytes:row["maximumCapacity"].as_u64().unwrap()as usize,..grant};
  let (result,heap)=observe_heap(||caller.turn(policy,|cx|cx.with_native_encoding(&mut retirement,&mut output,|control,output|{assert_eq!(control.retirement_recipient_identity(),Some(retirement_id));allocate_bytes(control,output,total)?;for chunk in &chunks{control.append_bytes(output.as_mut().unwrap(),chunk)?;}Ok(())})));
  assert_eq!(result.is_ok(),row["valid"].as_bool().unwrap(),"{:?}",result.as_ref().err().map(|failure|failure.primary_value()));assert_eq!(before-caller.fuel_remaining()-2,row["expectedWork"].as_u64().unwrap());assert_eq!(output.as_ref().map_or(&[][..],Vec::as_slice),serde_json::from_value::<Vec<u8>>(row["expectedBytes"].clone()).unwrap());let capacity=output.as_ref().map_or(0,Vec::capacity);assert_eq!(capacity,row["expectedCapacity"].as_u64().unwrap()as usize);assert_eq!((heap.0,heap.1),(capacity,0));assert_eq!(caller.receipt().retained_capacity_bytes,capacity);
  if let Err(error)=result{assert_eq!(error.retained_progress(),caller.receipt());}
  let (_,close)=observe_heap(||caller.turn(grant,|cx|{cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:capacity,..Default::default()})?;cx.consume_fuel(1);drop(output.take());Ok(())}).unwrap());assert_eq!((close.0,close.1),(0,capacity));close_source_owner(&mut caller,&mut retirement,grant);caller.retire(grant);
 }
 let bytes:Vec<u8>=serde_json::from_value(corpus["unwind"]["bytes"].clone()).unwrap();let mut caller=OriginalJobFixtureCaller::admit(OperationId(719),Generation(1),grant,clock,100000,100000);let mut output=None;let mut retirement=source_owner(&mut caller,grant);
 let (panic,heap)=observe_heap(||std::panic::catch_unwind(std::panic::AssertUnwindSafe(||caller.turn(grant,|cx|cx.with_native_encoding(&mut retirement,&mut output,|control,output|{allocate_bytes(control,output,bytes.len())?;control.append_bytes(output.as_mut().unwrap(),&bytes)?;panic!("original encoding unwind law")})))));
 assert!(panic.is_err());assert_eq!(caller.receipt().retained_capacity_bytes,corpus["unwind"]["expectedCapacity"].as_u64().unwrap()as usize);assert_eq!(caller.receipt().copied_bytes,corpus["unwind"]["expectedCopies"].as_u64().unwrap()as usize);assert!(heap.0>=bytes.len());assert_eq!(output.as_ref().unwrap(),&bytes);caller.turn(grant,|cx|{cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:output.as_ref().unwrap().capacity(),..Default::default()})?;cx.consume_fuel(1);drop(output.take());Ok(())}).unwrap();
 let policy=RetainedCloneGrant{maximum_depth:corpus["depth"]["source"].as_u64().unwrap()as usize,..grant};let result=caller.turn(policy,|cx|cx.with_native_encoding(&mut retirement,&mut output,|control,_|control.scoped_depth(corpus["depth"]["consumer"].as_u64().unwrap()as usize,|control|control.scoped_depth(corpus["depth"]["consumer"].as_u64().unwrap()as usize,|_|Ok::<(),ValueError>(())))));assert_eq!(result.is_ok(),corpus["depth"]["admitted"].as_bool().unwrap());assert!(output.is_none());close_source_owner(&mut caller,&mut retirement,grant);caller.retire(grant);
}

fn close_original<T:semio_framework_value::retirement::RetireOwned>(caller:&mut OriginalJobFixtureCaller,value:T,grant:RetainedCloneGrant){
 let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::retain(value);
 for _ in 0..65536{if owner.terminal_is_empty(){break}let(result,heap)=observe_heap(||caller.turn(grant,|cx|cx.with_original_retirement(|cx|{let demand=owner.selected_demand(grant.maximum_copy_bytes)?;semio_framework_job_context::close_original_effect_step(cx,demand,|effect|owner.step(grant.maximum_copy_bytes,effect))?;Ok(())})));let receipt=caller.receipt();assert_eq!((heap.0,heap.1),(receipt.retained_capacity_bytes,receipt.released_bytes));result.unwrap();}
 assert!(owner.terminal_is_empty());
}
#[test]
fn original_source_commits_one_action_before_post_effect_cancellation(){
 let f:serde_json::Value=serde_json::from_str(include_str!("source-cancel.json")).unwrap();let grant=stock();let prefix:Vec<u8>=serde_json::from_value(f["prefix"].clone()).unwrap();let tail:Vec<u8>=serde_json::from_value(f["tail"].clone()).unwrap();let mut caller=OriginalJobFixtureCaller::admit(OperationId(720),Generation(1),grant,clock,100000,100000);let cancel=caller.admit_alias(grant);let mut output=None;let mut retirement=source_owner(&mut caller,grant);let before=caller.fuel_remaining();let retirement_id=retirement.retirement_identity();
 let(result,heap)=observe_heap(||caller.turn(grant,|cx|{
  let identity=(cx.operation(),cx.generation(),cx.original_recipient_identity(),cx.payload_authority_identity());
  let result=cx.with_native_encoding(&mut retirement,&mut output,|control,output|{
   assert_eq!(control.retirement_recipient_identity(),Some(retirement_id));allocate_bytes(control,output,prefix.len()+tail.len())?;control.append_bytes(output.as_mut().unwrap(),&prefix)?;assert_eq!(control.actual_work(),u128::from(f["expectedActionWork"].as_u64().unwrap()));cancel.cancel_now();control.append_bytes(output.as_mut().unwrap(),&tail)
  });
  assert_eq!((cx.operation(),cx.generation(),cx.original_recipient_identity(),cx.payload_authority_identity()),identity);result
 }));
 let failure=result.unwrap_err();assert_eq!(failure.primary_value().unwrap().kind.as_str(),f["expectedKind"].as_str().unwrap());assert_eq!(before-caller.fuel_remaining(),f["expectedFuel"].as_u64().unwrap());assert_eq!(before-caller.fuel_remaining()-2,f["expectedActionWork"].as_u64().unwrap());assert_eq!(output.as_ref().unwrap(),&prefix);assert_eq!(caller.receipt().copied_bytes,f["expectedCopiedBytes"].as_u64().unwrap()as usize);assert_eq!(caller.receipt().retained_capacity_bytes,f["expectedCapacityBytes"].as_u64().unwrap()as usize);assert_eq!((heap.0,heap.1),(output.as_ref().unwrap().capacity(),0));
 close_original(&mut caller,failure,grant);close_original(&mut caller,output.take().unwrap(),grant);caller.return_alias(cancel,grant);close_source_owner(&mut caller,&mut retirement,grant);caller.retire(grant);
}

#[test]
fn source_encoder_construction_posts_to_the_original_caller(){
 let grant=stock();let mut caller=OriginalJobFixtureCaller::admit(OperationId(721),Generation(1),grant,clock,100000,100000);let mut output=None;let mut retirement=source_owner(&mut caller,grant);let mut slot=0u8;let before=caller.fuel_remaining();
 caller.turn(grant,|cx|cx.with_native_encoding(&mut retirement,&mut output,|control,_|{
  let demand=semio_framework_value::RetirementDemand{copy_bytes:1,depth:1,..Default::default()};let quote=control.construction_grant(demand)?;assert!(control.prepare_work(1)?);slot=7;control.commit_work(1)?;let actual=RetainedCloneProgress{copied_items:1,copied_bytes:1,..Default::default()};control.submit_constructed_progress(quote,actual)?;assert_eq!(control.construction_submitted()?,actual);Ok(())
 })).unwrap();
 assert_eq!(slot,7);assert_eq!(caller.receipt().copied_bytes,1);assert_eq!(caller.receipt().copied_items,3);assert_eq!(before-caller.fuel_remaining(),3);close_source_owner(&mut caller,&mut retirement,grant);caller.retire(grant);
}

#[test]
fn original_source_allocation_posts_real_backing_after_installation(){
 let grant=stock();let mut caller=OriginalJobFixtureCaller::admit(OperationId(722),Generation(1),grant,clock,100000,100000);let mut owner=source_owner(&mut caller,grant);let mut output=None;let mut actual=None;let mut progress=None;
 let(result,heap)=observe_heap(||caller.turn(grant,|cx|cx.with_native_encoding(&mut owner,&mut output,|control,output|{
  assert!(control.prepare_work(1).unwrap());let outcome=control.allocate_effect(16,&mut actual,&mut progress,&mut |actual,_|{let mut bytes=Vec::<u8>::new();bytes.try_reserve_exact(16).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original source fixture allocation refused"))?;let capacity=bytes.capacity();*output=Some(bytes);*actual=Some(capacity);Ok(true)}).unwrap().expect("fixture allocation settles in its own turn");
  control.commit_work(1).unwrap();assert!(outcome.primary.is_none(),"{:?}",outcome.primary);assert!(outcome.forwarded_quote.is_none());assert!(outcome.scoped_quote.is_none());assert!(outcome.extent_refusal.is_none());assert!(outcome.forwarded_refusal.is_none());assert!(outcome.scoped_refusal.is_none());Ok(())
 })));
 result.unwrap();assert!(progress.is_none());assert_eq!(actual,Some(output.as_ref().unwrap().capacity()));assert_eq!(heap.0,actual.unwrap());assert_eq!(caller.receipt().retained_capacity_bytes,actual.unwrap());close_original(&mut caller,output.take().unwrap(),grant);close_source_owner(&mut caller,&mut owner,grant);caller.retire(grant);
}

#[test]
fn original_source_post_refusal_keeps_the_real_backing_and_failure(){
 let grant=stock();let mut caller=OriginalJobFixtureCaller::admit(OperationId(723),Generation(1),grant,clock,100000,100000);let cancel=caller.admit_alias(grant);let mut owner=source_owner(&mut caller,grant);let mut output=None;let mut actual=None;let mut progress=None;let mut allocation=None;
 let(result,heap)=observe_heap(||caller.turn(grant,|cx|cx.with_native_encoding(&mut owner,&mut output,|control,output|{
  assert!(control.prepare_work(1).unwrap());allocation=control.allocate_effect(16,&mut actual,&mut progress,&mut |actual,_|{let mut bytes=Vec::<u8>::new();bytes.try_reserve_exact(16).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"original source fixture allocation refused"))?;let capacity=bytes.capacity();*output=Some(bytes);*actual=Some(capacity);cancel.cancel_now();Ok(true)}).unwrap();control.commit_work(1).unwrap();Ok(())
 })));
 result.unwrap();let allocation=allocation.take().unwrap();assert!(allocation.forwarded_quote.is_none());assert!(allocation.scoped_quote.is_none());assert!(allocation.primary.is_none(),"{:?}",allocation.primary);assert!(allocation.extent_refusal.is_none());let semio_framework_value_control_contract::NativeAllocationRefusal::Owned(refusal)=allocation.scoped_refusal.as_ref().unwrap()else{panic!("scoped refusal stays directly owned")};assert_eq!(refusal.kind,semio_framework_value::ValueRefusalKind::Canceled);assert!(progress.is_none());assert_eq!(actual,Some(output.as_ref().unwrap().capacity()));assert_eq!(heap.0,actual.unwrap());assert_eq!(caller.receipt().retained_capacity_bytes,actual.unwrap());close_original(&mut caller,allocation,grant);close_original(&mut caller,output.take().unwrap(),grant);caller.return_alias(cancel,grant);close_source_owner(&mut caller,&mut owner,grant);caller.retire(grant);
}

#[test]
fn original_source_cancels_an_unborn_allocation_reservation(){
 let grant=stock();let mut caller=OriginalJobFixtureCaller::admit(OperationId(724),Generation(1),grant,clock,100000,100000);let mut owner=source_owner(&mut caller,grant);let mut output=None;let mut actual=None;let mut progress=None;let mut allocation=None;
 caller.turn(grant,|cx|cx.with_native_encoding(&mut owner,&mut output,|control,_|{assert!(control.prepare_work(1).unwrap());allocation=control.allocate_effect(16,&mut actual,&mut progress,&mut |_,_|Err(ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed,"fixture left backing unborn"))).unwrap();control.cancel_work().unwrap();Ok(())})).unwrap();
 let allocation=allocation.take().unwrap();assert!(allocation.forwarded_quote.is_none());assert!(allocation.scoped_quote.is_none());assert!(allocation.primary.is_some());assert!(allocation.scoped_refusal.is_none());assert!(actual.is_none());assert!(progress.is_none());assert!(output.is_none());assert_eq!(caller.receipt().retained_capacity_bytes,0);close_original(&mut caller,allocation,grant);close_source_owner(&mut caller,&mut owner,grant);caller.retire(grant);
}

#[test]
fn source_encoding_owner_keeps_its_original_caller_identity(){
 let grant=stock();let mut first=OriginalJobFixtureCaller::admit(OperationId(725),Generation(1),grant,clock,100000,100000);let mut second=OriginalJobFixtureCaller::admit(OperationId(726),Generation(1),grant,clock,100000,100000);let mut owner=source_owner(&mut first,grant);let mut output=None;
 first.turn(grant,|cx|cx.with_native_encoding(&mut owner,&mut output,|_,_|Ok(()))).unwrap();let refusal=second.turn(grant,|cx|cx.with_native_encoding(&mut owner,&mut output,|_,_|panic!("a different caller cannot borrow the source owner"))).unwrap_err();assert_eq!(refusal.primary_value().unwrap().kind,semio_framework_value::ValueRefusalKind::InvariantViolated);assert!(!owner.has_retained_failure());close_original(&mut second,refusal,grant);close_source_owner(&mut first,&mut owner,grant);first.retire(grant);second.retire(grant);
}

#[test]
fn original_source_retains_provenance_refusal_during_unwind(){
 let grant=stock();let mut caller=OriginalJobFixtureCaller::admit(OperationId(727),Generation(1),grant,clock,100000,100000);let mut owner=source_owner(&mut caller,grant);let mut output=None;
 let panic=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||caller.turn(grant,|cx|cx.with_native_encoding(&mut owner,&mut output,|control,output|{allocate_bytes(control,output,16)?;output.as_mut().unwrap().push(42);panic!("fixture unwinds after an unadmitted write")}))));assert!(panic.is_err());assert!(owner.has_retained_failure());let mut failure=None;let denied=RetainedCloneGrant{maximum_items:0,..grant};
 assert!(!caller.turn(denied,|cx|cx.with_original_retirement(|cx|owner.take_retained_failure(cx,&mut failure))).unwrap());assert!(failure.is_none());assert!(caller.turn(grant,|cx|cx.with_original_retirement(|cx|owner.take_retained_failure(cx,&mut failure))).unwrap());let failure=failure.take().unwrap();assert!(failure.receipt.is_none());assert!(failure.provenance.is_some());assert!(!owner.has_retained_failure());close_original(&mut caller,failure,grant);close_original(&mut caller,output.take().unwrap(),grant);close_source_owner(&mut caller,&mut owner,grant);caller.retire(grant);
}

#[test]
fn original_source_keeps_primary_and_provenance_as_distinct_owners(){
 let grant=stock();let mut caller=OriginalJobFixtureCaller::admit(OperationId(728),Generation(1),grant,clock,100000,100000);let mut owner=source_owner(&mut caller,grant);let mut output=None;
 let primary=caller.turn(grant,|cx|cx.with_native_encoding(&mut owner,&mut output,|control,output|{allocate_bytes(control,output,16)?;output.as_mut().unwrap().push(42);Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"fixture primary"))})).unwrap_err();assert_eq!(primary.primary_value().unwrap().message.as_ref(),"fixture primary");let mut retained=None;assert!(caller.turn(grant,|cx|cx.with_original_retirement(|cx|owner.take_retained_failure(cx,&mut retained))).unwrap());let retained=retained.take().unwrap();assert!(retained.receipt.is_none());assert!(retained.provenance.is_some());close_original(&mut caller,primary,grant);close_original(&mut caller,retained,grant);close_original(&mut caller,output.take().unwrap(),grant);close_source_owner(&mut caller,&mut owner,grant);caller.retire(grant);
}

#[test]
fn original_source_quotes_fresh_empty_vector_header(){
 let stock=stock();let header=std::mem::size_of::<Vec<u8>>();
 for allowed in [false,true]{let mut caller=OriginalJobFixtureCaller::admit(OperationId(if allowed{730}else{729}),Generation(1),stock,clock,100000,100000);let mut owner=source_owner(&mut caller,stock);let mut receiver=None;let mut vector=None;let grant=RetainedCloneGrant{maximum_copy_bytes:if allowed{header}else{header-1},..stock};
 let(result,heap)=observe_heap(||caller.turn(grant,|cx|cx.with_native_encoding(&mut owner,&mut receiver,|control,_|{let demand=semio_framework_value::RetirementDemand{copy_bytes:header,depth:1,..Default::default()};let quote=control.construction_grant(demand)?;assert!(control.prepare_work(1).unwrap());vector=Some(Vec::<u8>::new());control.commit_work(1).unwrap();control.submit_constructed_progress(quote,RetainedCloneProgress{copied_items:1,copied_bytes:header,..Default::default()})?;Ok(())})));
 assert_eq!(result.is_ok(),allowed);assert_eq!(vector.is_some(),allowed);assert_eq!(heap.0,0);assert_eq!(caller.receipt().copied_bytes,if allowed{header}else{0});if let Err(error)=result{close_original(&mut caller,error,stock)}if let Some(vector)=vector.take(){close_original(&mut caller,vector,stock)}close_source_owner(&mut caller,&mut owner,stock);caller.retire(stock);}
}

#[test]
fn original_source_issuer_requires_its_actual_fresh_metadata_grant(){
 let stock=stock();let mut caller=OriginalJobFixtureCaller::admit(OperationId(731),Generation(1),stock,clock,100000,100000);let mut owner=OriginalSourceEncodingOwner::new();assert_eq!(owner.next_authority_copy_bytes(),None);
 let mut authorities=0;while !caller.turn(stock,|cx|owner.admit_step(cx)).unwrap(){let Some(width)=owner.next_authority_copy_bytes()else{continue};let denied=RetainedCloneGrant{maximum_copy_bytes:width-1,..stock};assert!(!caller.turn(denied,|cx|owner.admit_step(cx)).unwrap());assert_eq!(caller.receipt().copied_items,0);assert_eq!(owner.next_authority_copy_bytes(),Some(width));let exact=RetainedCloneGrant{maximum_copy_bytes:width,..stock};assert!(!caller.turn(exact,|cx|owner.admit_step(cx)).unwrap());assert_eq!(caller.receipt().copied_items,1);assert_eq!(caller.receipt().copied_bytes,width);assert_ne!(owner.next_authority_copy_bytes(),Some(width));authorities+=1;}
 assert_eq!(authorities,5);assert_eq!(owner.next_authority_copy_bytes(),None);close_source_owner(&mut caller,&mut owner,stock);caller.retire(stock);
}
fn observe_heap<T>(run:impl FnOnce()->T)->(T,(usize,usize)){let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(run);(value,(heap.requested_bytes,heap.released_bytes))}
