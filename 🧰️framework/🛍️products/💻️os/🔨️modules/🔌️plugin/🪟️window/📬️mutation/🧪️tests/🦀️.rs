//! 🧪️ Actual window wrapper backing stays in custody until independently funded retirement.
use semio_framework_value::{RetainedCloneGrant,retirement::{RetireOwned,controlled::ControlledRetirement,OwnedValueRetirementFactory}};
use std::sync::Arc;
pub(crate) fn close_original<T:RetireOwned>(value:T){close_with(value,||{});}
pub(crate) fn close_with<T:RetireOwned>(value:T,mut pump:impl FnMut()){
 let mut owner=ControlledRetirement::new(value).map_err(|(error,_)|error).unwrap();
 for _ in 0..1000000{if owner.terminal_is_empty(){return;}let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();owner.step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.max(4096),maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()}).unwrap();pump();}
 panic!("original test owner did not reach physical terminal emptiness");
}

#[derive(semio_framework_value::RetireOwned)]
struct RefreshSnapshot{window_id:String,payload:String}
impl super::WindowRefreshSnapshot for RefreshSnapshot{fn swap_address(&mut self,other:&mut Self){std::mem::swap(&mut self.window_id,&mut other.window_id);}}
#[test]
fn original_window_refresh_admits_displaced_snapshot_under_actual_currencies(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🔄️refresh/🧫️fixtures/🔣️.json")).unwrap();
 for body in law["copyGrants"].as_array().unwrap(){let body=body.as_u64().unwrap()as usize;
  let((mut current,mut pending,mut retired),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||(RefreshSnapshot{window_id:law["windowId"].as_str().unwrap().into(),payload:law["before"].as_str().unwrap().into()},Some(RefreshSnapshot{window_id:String::new(),payload:law["after"].as_str().unwrap().into()}),semio_framework_value::retirement::queue::RetirementQueue::default()));
  let held=heap.requested_bytes-heap.released_bytes;let address=current.window_id.as_ptr();let payload=pending.as_ref().unwrap().payload.as_ptr();let(mut born,mut released,mut ready)=(0usize,0usize,false);
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){
   let demand=super::refresh_demands(&pending,&retired).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
   for denied in[Some(RetainedCloneGrant{maximum_items:0,..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.depth>0).then_some(RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant})].into_iter().flatten(){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||super::refresh_transfer(&mut current,&mut pending,&mut retired,denied));assert!(matches!(result,Ok(super::WindowAuthorityRefreshStep::Pending(progress))if progress==Default::default()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(current.window_id.as_ptr(),address);assert_eq!(current.payload,law["before"].as_str().unwrap());assert_eq!(pending.as_ref().unwrap().payload.as_ptr(),payload);assert_eq!(retired.len(),0);}
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||super::refresh_transfer(&mut current,&mut pending,&mut retired,grant).unwrap());let progress=match step{super::WindowAuthorityRefreshStep::Pending(progress)=>progress,super::WindowAuthorityRefreshStep::Ready(progress)=>{ready=true;progress}};assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert_eq!(heap.released_bytes,0);born+=heap.requested_bytes;if ready{break;}
  }
  assert!(ready);assert_eq!(current.window_id.as_ptr(),address);assert_eq!(current.payload.as_ptr(),payload);assert!(pending.is_none());assert_eq!(retired.len(),1);
  let(mut owner,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ControlledRetirement::new((current,pending,retired)).map_err(|(error,_)|error).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  for _ in 0..1000000{if owner.terminal_is_empty(){break;}let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body.max(copy),maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;}
  assert!(owner.terminal_is_empty());assert_eq!(released,held+born);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] Original window refresh body={body} original={held} born={born} released={released} finalDrop=0 exact address/new payload pointers retained");
 }
}
fn run<T:RetireOwned+Send+'static>(construct:impl FnOnce()->T,body:usize){
 let(original,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(construct);let held=heap.requested_bytes-heap.released_bytes;let(mut owner,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ControlledRetirement::new(original).map_err(|(error,_)|error).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(mut born,mut released)=(0,0);
 for turn in 0..1000000{if owner.terminal_is_empty(){break;}let demanded_copy=owner.next_copy_byte_demand().unwrap();let copy=demanded_copy.max(body);let release=owner.next_release_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()};
  let denied=[Some(RetainedCloneGrant{maximum_items:0,..grant}),(demanded_copy>0).then_some(RetainedCloneGrant{maximum_copy_bytes:0,..grant}),(grant.maximum_capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(1),..grant}),(grant.maximum_release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(1),..grant}),(grant.maximum_depth>0).then_some(RetainedCloneGrant{maximum_depth:grant.maximum_depth.saturating_sub(1),..grant})];
  for under in denied.into_iter().flatten(){let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(under));if let Ok(step)=step{assert_eq!(step.progress(),Default::default());}assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;assert!(turn<999999);
 }
 assert!(owner.terminal_is_empty());assert_eq!(released,held+born);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] Actual window wrapper original={held} born={born} released={released} terminalDrop=0 copy={body}");
}
#[test]
fn window_original_mutation_wrappers_have_exact_physical_custody(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();for body in law["copyGrants"].as_array().unwrap(){let body=body.as_u64().unwrap()as usize;for kind in ["canvas","layers"]{let text=law["payload"].as_str().unwrap();let capacity=law["capacity"].as_u64().unwrap()as usize;let window=law["windowId"].as_str().unwrap();
 run(||{let mut payload=String::with_capacity(capacity);payload.push_str(text);crate::WindowConfigMutation::from_issued(window,kind,payload,Arc::new(OwnedValueRetirementFactory::<String>::default()))},body);
 run(||{let mut payload=String::with_capacity(capacity);payload.push_str(text);crate::WindowTransientMutation::from_issued(window,kind,payload,Arc::new(OwnedValueRetirementFactory::<String>::default()))},body);
 }}
}
