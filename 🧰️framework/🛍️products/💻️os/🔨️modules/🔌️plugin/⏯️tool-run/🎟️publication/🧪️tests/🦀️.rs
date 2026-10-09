//! 🎟️ Original inline publication ingress survives every funded partial cancellation.
use super::*;
#[test]
fn original_tool_run_publication_ingress_preserves_partial_custody_and_fixed_grants(){
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:262144,maximum_depth:4096};
 for actor in["alice","actor🧩\0ü"]{for stop in[0,1,2]{
  let mut ingress=ToolRunPublicationIngress::default();let mut born=0;
  if stop>0{let(receipt,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ingress.admit_actor(actor,grant).unwrap().unwrap());assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,0));born+=heap.requested_bytes;}
  if stop>1{let clock=protocol::HybridLogicalTimestamp{actor:17,physical_ms:1728000000123,logical:129};let(receipt,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ingress.admit_transaction(actor,&clock,"draw","select",grant).unwrap().unwrap());assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,0));born+=heap.requested_bytes;}
  let actor_pointer=ingress.actor.as_ref().and_then(ControlledRetirement::original).map(|actor|actor.as_str().as_ptr());let transaction_pointer=ingress.transaction.as_ref().map(|transaction|transaction.id.as_ptr());
  let mut closed=0;let mut turns=0;
  while !ingress.terminal_is_empty(){let(demand,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ingress.close_demands(grant.maximum_copy_bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   let mut denied=vec![RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}];if demand.copy_bytes>0{denied.push(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant});}if demand.capacity_bytes>0{denied.push(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant});}if demand.release_bytes>0{denied.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant});}
   for denied in denied{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ingress.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   if turns==0{assert_eq!(ingress.actor.as_ref().and_then(ControlledRetirement::original).map(|actor|actor.as_str().as_ptr()),actor_pointer);assert_eq!(ingress.transaction.as_ref().map(|transaction|transaction.id.as_ptr()),transaction_pointer);}
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ingress.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;closed+=heap.released_bytes;turns+=1;assert!(turns<100000);
  }
  assert_eq!(born,closed);let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(ingress));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));println!("[DEBUG] original ToolRun ingress cancel={stop} turns={turns} physical={closed} fixed1/4096/4096/262144/4096 all-axis denials pure, original actor/transaction pointers, terminalDrop0");
 }}
}
