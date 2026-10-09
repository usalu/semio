use super::*;
use crate::value::observe_retirement_allocations as observe;
#[test]
fn original_vector_native_ordered_handoffs_cancel_every_original_frontier_and_pay_final_backing(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for stop in law["cancelAfter"].as_array().unwrap(){
  let(source,birth)=observe(||{let mut values=Vec::with_capacity(37);for value in law["values"].as_array().unwrap(){values.push(value.as_str().unwrap().repeat(8192));}values});
  let pointers:Vec<_>=source.iter().map(|value|value.as_ptr()).collect();let backing=source.as_ptr();let backing_bytes=source.capacity()*size_of::<String>();
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:262144,maximum_depth:4096};
  let mut source=source;
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(result,heap)=observe(||OriginalVectorCursor::admit(source,denied));source=result.err().unwrap().1;assert_eq!(heap,(0,0));assert_eq!(source.as_ptr(),backing);assert_eq!(source[0].as_ptr(),pointers[0]);}
  let(result,heap)=observe(||OriginalVectorCursor::admit(source,grant));let(mut owner,receipt)=result.map_err(|(error,_)|error).unwrap();assert_eq!(heap,(0,0));assert!(receipt.fits(grant));assert_eq!(owner.remaining().as_ptr(),backing);
  let mut returned=Vec::new();let mut births=0;let mut releases=0;
  for index in 0..stop.as_u64().unwrap()as usize{let(demand,heap)=observe(||owner.next_take_demand());assert_eq!(heap,(0,0));let(result,heap)=observe(||owner.take_front(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant}).unwrap());assert!(result.is_none());assert_eq!(heap,(0,0));assert_eq!(owner.remaining()[0].as_ptr(),pointers[index]);let(result,heap)=observe(||owner.take_front(grant).unwrap());let(value,receipt)=result.unwrap();assert_eq!(heap,(0,0));assert_eq!(value.as_ptr(),pointers[index]);assert!(receipt.fits(grant));returned.push(value);}
  owner.begin_close();if owner.remaining().is_empty(){let(step,heap)=observe(||owner.step(RetainedCloneGrant{maximum_release_bytes:backing_bytes-1,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));assert!(!owner.terminal_is_empty());}
  for _ in 0..100000{if owner.terminal_is_empty(){break;}let(demand,heap)=observe(||owner.next_demand().unwrap());assert_eq!(heap,(0,0));let denied=RetainedCloneGrant{maximum_release_bytes:0,maximum_capacity_bytes:0,maximum_copy_bytes:0,maximum_depth:0,..grant};let(step,heap)=observe(||owner.step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));let(step,heap)=observe(||owner.step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));births+=heap.0;releases+=heap.1;assert!(demand.depth<=grant.maximum_depth);}
  assert!(owner.terminal_is_empty());let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));
  for value in returned{let mut cursor=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("native string custody"));for _ in 0..1000{if cursor.terminal_is_empty(){break;}let(step,heap)=observe(||cursor.step(grant).unwrap());let receipt=step.progress();assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));assert!(receipt.fits(grant));births+=heap.0;releases+=heap.1;}assert!(cursor.terminal_is_empty());let(_,heap)=observe(||drop(cursor));assert_eq!(heap,(0,0));}
  assert_eq!(releases,birth.0+births);assert!(releases>=backing_bytes);
 }
 eprintln!("[DEBUG] original vector exact payload pointers and ordered return retained through4cancellation frontiers; every heap receipt paid and finalDrop0");
}
