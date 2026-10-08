//! 🧪️ Native error prose and target owners preserve exact physical allocations through admitted retirement.
use semio_framework_value::{retirement::{RetireOwned,controlled::ControlledRetirement},retained_clone::{RetainedCloneGrant,RetainedCloneStep},ValueError,ValueRefusalKind};
use super::super::MutationApplyError;
fn observe_retirement_allocations<T>(operation:impl FnOnce()->T)->(T,(usize,usize)){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(operation);(result,(heap.requested_bytes,heap.released_bytes))}
fn close<T:RetireOwned>(original:T,held:usize,copy:usize,cut:usize){
 let (mut owner,heap)=observe_retirement_allocations(||ControlledRetirement::new(original).unwrap_or_else(|_|panic!("declared native error retirement")));assert_eq!(heap,(0,0));let mut births=0;let mut releases=0;let mut turns=0;
 while !owner.terminal_is_empty(){
  let ((work,capacity,release,depth),heap)=observe_retirement_allocations(||(owner.next_copy_byte_demand().unwrap(),owner.next_capacity_byte_demand(copy).unwrap(),owner.next_release_byte_demand().unwrap(),owner.next_depth_demand().unwrap()));assert_eq!(heap,(0,0));
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.max(work),maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:depth};
  for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(work!=0).then_some(RetainedCloneGrant{maximum_copy_bytes:work.saturating_sub(1),..grant}),(capacity!=0).then_some(RetainedCloneGrant{maximum_capacity_bytes:capacity.saturating_sub(1),..grant}),(release!=0).then_some(RetainedCloneGrant{maximum_release_bytes:release.saturating_sub(1),..grant}),(depth!=0).then_some(RetainedCloneGrant{maximum_depth:depth.saturating_sub(1),..grant})].into_iter().flatten(){
   let (step,heap)=observe_retirement_allocations(||owner.step(denied));match step{Ok(step)=>assert_eq!(step.progress(),Default::default()),Err(error)=>assert_eq!(error.kind,ValueRefusalKind::DepthLimit)}assert_eq!(heap,(0,0));
  }
  let (step,heap)=observe_retirement_allocations(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));births+=heap.0;releases+=heap.1;turns+=1;
  if turns==cut{let (same,heap)=observe_retirement_allocations(||owner);owner=same;assert_eq!(heap,(0,0));}
  assert!(turns<100000,"original error retirement exceeded native law");assert!(step.progress().copied_items!=0||matches!(step,RetainedCloneStep::Complete(_)),"fully admitted original error turn stalled");
 }
 assert_eq!(releases,held+births);let (_,heap)=observe_retirement_allocations(||drop(owner));assert_eq!(heap,(0,0));eprintln!("[DEBUG] Declared error retirement original={held} copy={copy} cancel={cut} turns={turns} births={births} physical={releases} terminalDrop=0");
}
#[test]
fn error_retirement_value_error_preserves_borrowed_and_owned_cow_original_backing(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌱️value/⚠️refusal/♻️retirement/🧫️fixtures/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){for copy in law["copyGrants"].as_array().unwrap(){for cut in law["cancelCuts"].as_array().unwrap(){
  let text=row["text"].as_str().unwrap();let original=if row["ownership"]=="borrowed"{ValueError::literal(ValueRefusalKind::InvalidValue,"immutable refusal")}else{let mut message=String::with_capacity(row["capacity"].as_u64().unwrap()as usize);message.push_str(text);ValueError::new(ValueRefusalKind::InvalidValue,message)};
  assert_eq!(original.message,text);assert_eq!(original.kind.as_str(),"invalidValue");let oracle=serde_json::json!({"kind":original.kind.as_str(),"display":original.to_string()});assert_eq!(oracle["display"],text);
  let pointer=original.message.as_ptr();let held=match &original.message{std::borrow::Cow::Borrowed(_)=>0,std::borrow::Cow::Owned(text)=>text.capacity()};let (_,heap)=observe_retirement_allocations(||{assert!(<ValueError as RetireOwned>::controlled_retirement_supported());assert!(original.retirement_birth_bytes().is_some());assert_eq!(original.message.as_ptr(),pointer);});assert_eq!(heap,(0,0));
  close(original,held,copy.as_u64().unwrap()as usize,cut.as_u64().unwrap()as usize);
 }}}
}
#[test]
fn error_retirement_mutation_apply_error_preserves_original_code_prose_targets_and_empty_capacities(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in law["cases"].as_array().unwrap(){for copy in law["copyGrants"].as_array().unwrap(){for cut in law["cancelCuts"].as_array().unwrap(){
  let caps=row["capacities"].as_array().unwrap();let make=|index:usize,text:&str|{let mut value=String::with_capacity(caps[index].as_u64().unwrap()as usize);value.push_str(text);value};let mut target=Vec::with_capacity(row["targetCapacity"].as_u64().unwrap()as usize);for(index,text)in row["wire"]["target"].as_array().unwrap().iter().enumerate(){target.push(make(index+2,text.as_str().unwrap()));}
  let original=MutationApplyError{code:make(0,row["wire"]["code"].as_str().unwrap()),message:make(1,row["wire"]["message"].as_str().unwrap()),target};let wire=semio_framework_value::ToValue::to_value(&original);assert_eq!(serde_json::to_value(wire).unwrap(),row["wire"]);let held=original.code.capacity()+original.message.capacity()+original.target.capacity()*std::mem::size_of::<String>()+original.target.iter().map(String::capacity).sum::<usize>();let pointer=original.target.as_ptr();let (_,heap)=observe_retirement_allocations(||{assert!(original.retirement_birth_bytes().is_some());assert_eq!(original.target.as_ptr(),pointer);});assert_eq!(heap,(0,0));
  close(original,held,copy.as_u64().unwrap()as usize,cut.as_u64().unwrap()as usize);
 }}}
}
