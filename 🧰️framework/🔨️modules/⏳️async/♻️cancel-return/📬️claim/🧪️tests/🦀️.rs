use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
fn quote(cursor:&ClaimRetirement)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:cursor.next_work_byte_demand().unwrap(),maximum_release_bytes:cursor.next_close_byte_demand().unwrap(),maximum_depth:1,..Default::default()}}
fn close(mut cursor:ClaimRetirement)->usize{
 let mut released=0;
 while !cursor.terminal_is_empty(){let grant=quote(&cursor);if grant.maximum_release_bytes!=0{let(step,heap)=observe(||cursor.close_step(RetainedCloneGrant{maximum_release_bytes:grant.maximum_release_bytes-1,..grant}));assert!(matches!(step,RetirementStep::BudgetExhausted));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}let(step,heap)=observe(||cursor.close_step(grant));let RetirementStep::Progress(progress)=step else{panic!("genuine original claim turn");};assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,progress.released_bytes);released+=progress.released_bytes;}
 released
}
#[test]
fn original_publication_permit_return_slot_has_exact_allocator_and_parallel_custody(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let source=PublicationClaim::new();let aliases=row["aliases"].as_u64().unwrap()as usize;let other=(aliases>1).then(||source.clone());
  for _ in 0..row["repetitions"].as_u64().unwrap(){
   let(permit,heap)=observe(||source.try_claim().expect("original sole permit"));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   assert!(source.is_claimed());let mut cursor=ClaimRetirement{source:Some(source.clone())};let(step,heap)=observe(||cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:512,maximum_release_bytes:512,maximum_depth:1,..Default::default()}));assert!(matches!(step,RetirementStep::Failure(_)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   if row["cancelled"].as_bool().unwrap(){source.cancel();}
   let(_,heap)=if row["name"]=="abandoned-publication-future"{observe(||{let future=async move{std::future::pending::<()>().await;drop(permit);};let mut future=std::pin::pin!(future);let mut context=std::task::Context::from_waker(std::task::Waker::noop());assert!(std::future::Future::poll(future.as_mut(),&mut context).is_pending());})}else{observe(||drop(permit))};assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!source.is_claimed());assert!(source.has_returned_alias());
   let grant=quote(&cursor);assert_eq!(grant.maximum_release_bytes,0);
   for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(step,heap)=observe(||cursor.close_step(denied));assert!(matches!(step,RetirementStep::BudgetExhausted));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(source.has_returned_alias());}
   let(step,heap)=observe(||cursor.close_step(grant));assert!(matches!(step,RetirementStep::Progress(_)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!source.has_returned_alias());assert_eq!(close(cursor),0);
  }
  let cursor=ClaimRetirement{source:Some(source)};let bytes=PublicationClaimHandle::frame_bytes();let released=if row["parallel"].as_bool().unwrap(){let other=ClaimRetirement{source:other};let first=std::thread::spawn(move||close(cursor));let second=std::thread::spawn(move||close(other));first.join().unwrap()+second.join().unwrap()}else{let first=close(cursor);first+other.map_or(0,|source|close(ClaimRetirement{source:Some(source)}))};assert_eq!(released,bytes);eprintln!("[DEBUG] original publication claim {} released={released}",row["name"]);
 }
}
