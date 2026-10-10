use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
use std::sync::Arc;
#[global_allocator]
static CANCEL_HEAP_WITNESS:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
#[test]
fn original_cancel_graph_returns_exact_nodes_backing_and_retains_registered_waker(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let nodes=row["nodes"].as_u64().unwrap()as usize;let mut token=CancelToken::root_now();
  for _ in 1..nodes{let child=token.child_now();drop(token);token=child;}
  let alias=(row["aliases"].as_u64().unwrap()>1).then(||token.clone());
  let requested=row["emptyCapacity"].as_u64().unwrap()as usize;let mut capacity=0;
  if requested!=0{let mut waiters=token.0.waiters.lock().unwrap();waiters.reserve_exact(requested);capacity=waiters.capacity();}
  struct Wake;impl std::task::Wake for Wake{fn wake(self:Arc<Self>) {}}
  if row["waiters"].as_u64().unwrap()!=0{token.0.waiters.lock().unwrap().push((1,std::task::Waker::from(Arc::new(Wake))));}
  let mut cursor=CancelTokenRetirement{source:Some(token),body:None};let mut released=0;let mut retained=false;
  for _ in 0..64{
   if cursor.terminal_is_empty(){break;}
   let Some(bytes)=cursor.release_demand()else{assert!(matches!(cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:512,maximum_release_bytes:512,maximum_depth:1,..Default::default()}),RetirementStep::Failure(_)));retained=true;break;};
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:cursor.copy_demand(),maximum_release_bytes:bytes,maximum_depth:1,..Default::default()};
   for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let(step,heap)=observe(||cursor.close_step(denied));assert!(matches!(step,RetirementStep::BudgetExhausted));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   if bytes!=0{let(step,heap)=observe(||cursor.close_step(RetainedCloneGrant{maximum_release_bytes:bytes-1,..grant}));assert!(matches!(step,RetirementStep::BudgetExhausted));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   if grant.maximum_copy_bytes!=0{let(step,heap)=observe(||cursor.close_step(RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-1,..grant}));assert!(matches!(step,RetirementStep::BudgetExhausted));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   let(step,heap)=observe(||cursor.close_step(grant));let RetirementStep::Progress(progress)=step else{panic!("original cancellation owner turn");};assert_eq!(heap.requested_bytes,0);assert_eq!(heap.released_bytes,progress.released_bytes);released+=progress.released_bytes;
  }
  if row["outcome"]=="retained"{assert!(retained);let original=cursor.body.as_mut().unwrap().waiters.get_mut().unwrap().pop();drop(original);while !cursor.terminal_is_empty(){cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:512,maximum_release_bytes:512,maximum_depth:1,..Default::default()});}}
  else if row["outcome"]=="alias-returned"{assert_eq!(released,0);assert!(alias.is_some());}
  else{assert_eq!(released,nodes*CancelHandle::frame_bytes()+capacity*std::mem::size_of::<(u64,std::task::Waker)>());assert!(cursor.terminal_is_empty());}
  drop(alias);eprintln!("[DEBUG] original cancellation graph {} released={released} retained={retained}",row["name"]);
 }
}
