use super::*;
struct ObservedHeap {requested_bytes:usize,released_bytes:usize}
fn observe<T>(operation:impl FnOnce()->T)->(T,ObservedHeap){let (value,requested_bytes,released_bytes)=crate::registry::tests::observe_ownership(operation);(value,ObservedHeap{requested_bytes,released_bytes})}

#[test]
fn original_budgeted_topology_preserves_tree_and_matches_graphlib_order_with_physical_receipts(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){for copy in [1,3,64]{
  let (source,heap)=observe(||Arc::new(Tree{neurons:row["nodes"].as_array().unwrap().iter().map(|id|super::super::Neuron::with_kind(id.as_str().unwrap(),"actual-kind",super::super::Dictionary::new())).collect(),synapses:row["edges"].as_array().unwrap().iter().map(|edge|super::super::Synapse{id:String::new(),from:edge[0].as_str().unwrap().into(),to:edge[1].as_str().unwrap().into(),from_port:String::new(),to_port:String::new()}).collect()}));
  let original=heap.requested_bytes-heap.released_bytes;let pointer=Arc::as_ptr(&source);let (mut owner,heap)=observe(||BudgetedEvalTopology::new(source));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(Arc::as_ptr(owner.source().unwrap()),pointer);
  let mut born=0;let mut released=0;
  for _ in 0..100000{if owner.is_complete(){break;}let demand=owner.next_demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
   let (denied,heap)=observe(||owner.step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(denied.progress(),owner.step_progress());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(Arc::as_ptr(owner.source().unwrap()),pointer);
   if demand.capacity_bytes>0{let (_,heap)=observe(||owner.step(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant}).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   let (step,heap)=observe(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(step.progress(),owner.step_progress());assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;
  }
  assert!(owner.is_complete());let actual:Vec<_>=owner.order().map(|index|owner.source().unwrap().neurons[*index].id.as_str()).collect();let expected:Vec<_>=row["order"].as_array().unwrap().iter().map(|id|id.as_str().unwrap()).collect();assert_eq!(actual,expected);
  let (_,heap)=observe(||owner.begin_close());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  let mut idle=0;
  for turn in 0..1000000{if owner.terminal_is_empty(){break;}let demand=owner.next_close_demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
   let (step,heap)=observe(||owner.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(step.progress(),owner.step_progress());assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;if step.progress()==RetainedCloneProgress::default(){idle+=1;}else{idle=0;}assert!(idle<16,"original topology close stalled id={} copy={copy} turn={turn} demand={demand:?} grant={grant:?} born={born} released={released}",row["id"]);
  }
  assert!(owner.terminal_is_empty());assert_eq!(released,original+born);let (_,heap)=observe(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] Original topology id={} copy={copy} original={original} born={born} released={released} terminalDrop=0",row["id"]);
 }}
}
