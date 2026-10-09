use super::*;
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
#[test]
fn original_returned_read_registry_alias_has_exact_physical_custody(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let source=Arc::new(17u32);
  let((witness,mut other,backing),allocation)=observe(||{
   let registry=crate::os_store::SnapshotReadRegistryHandle::new();
   let backing=registry.state.try_lock().unwrap().slots.as_ptr();
   let lease=registry.try_issue(source.clone()).unwrap_or_else(|_|panic!("real source admission"));
   let read=super::super::SnapshotRead::new(source.clone(),lease);
   let witness=read.return_to_registry_witness().expect("real source returned once");
   assert!(witness.terminal_is_empty());
   let other=(row["otherAliases"].as_u64().unwrap()!=0).then_some(registry);
   (witness,other,backing)
  });
  assert!(!allocation.overflowed);assert_eq!(allocation.released_bytes,0);let original_bytes=allocation.requested_bytes;
  let(mut cursor,allocation)=observe(||SnapshotReadReturnRetirement::new(witness));assert_eq!((allocation.requested_bytes,allocation.released_bytes),(0,0));
  let demand=cursor.demands().unwrap();assert_eq!(demand.release_bytes,snapshot_registry_frame_bytes().unwrap());
  if row["dropOtherBeforeRelease"].as_bool().unwrap(){let(_,allocation)=observe(||drop(other.take()));assert_eq!(allocation.released_bytes,0);}
  for grant in [RetainedCloneGrant::default(),RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes-1,maximum_depth:1,..Default::default()}]{
   let(step,allocation)=observe(||cursor.step(grant).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((allocation.requested_bytes,allocation.released_bytes),(0,0));assert!(cursor.registry.is_some());
  }
  let mut released=0;
  for _ in 0..8{
   if cursor.terminal_is_empty(){break;}
   let demand=cursor.demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()};
   let(step,allocation)=observe(||cursor.step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(allocation.requested_bytes,0);assert_eq!(allocation.released_bytes,progress.released_bytes);released+=progress.released_bytes;
   if let Some(owner)=cursor.last_registry.as_ref(){let state=owner.state.try_lock().unwrap();if !state.slots.is_empty(){assert_eq!(state.slots.as_ptr(),backing);}}
  }
  assert!(cursor.terminal_is_empty());assert_eq!(released,if row["expectLastCustody"].as_bool().unwrap(){original_bytes}else{0});
  let(_,allocation)=observe(||drop(cursor));assert_eq!(allocation.released_bytes,0);
  if let Some(registry)=other.take(){
   let mut remaining=SnapshotReadReturnRetirement::new(SnapshotReadReturn{registry,index:0,generation:0});
   for _ in 0..8{if remaining.terminal_is_empty(){break;}let demand=remaining.demands().unwrap();remaining.step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()}).unwrap();}
   assert!(remaining.terminal_is_empty());
  }
  assert_eq!(*source,17);eprintln!("[DEBUG] Returned source witness {} original={original_bytes} released={released} terminalDrop=0",row["id"]);
 }
}

#[test]
fn original_returned_read_registry_parallel_aliases_credit_exactly_one_frame(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let shares=fixture["parallelShares"].as_u64().unwrap()as usize;assert_eq!(shares,2);
 let source=Arc::new(31u32);
 let((left,right,identity),allocation)=observe(||{
  let registry=crate::os_store::SnapshotReadRegistryHandle::new();let identity=registry.identity();
  let read=||{let lease=registry.try_issue(source.clone()).unwrap_or_else(|_|panic!("real parallel read admission"));super::super::SnapshotRead::new(source.clone(),lease).return_to_registry_witness().unwrap()};
  let left=read();let right=read();drop(registry);(left,right,identity)
 });
 assert!(!allocation.overflowed);assert_eq!(allocation.released_bytes,0);let original=allocation.requested_bytes;
 let barrier=std::sync::Barrier::new(shares);
 let drain=|witness|{
  let mut cursor=SnapshotReadReturnRetirement::new(witness);assert_eq!(cursor.registry.as_ref().unwrap().identity(),identity);barrier.wait();let mut released=0;let mut frame=false;
  for _ in 0..8{
   if cursor.terminal_is_empty(){break;}let demand=cursor.demands().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth,..Default::default()};
   let(step,allocation)=observe(||cursor.step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!(allocation.requested_bytes,0);assert_eq!(allocation.released_bytes,progress.released_bytes);released+=progress.released_bytes;frame|=cursor.last_registry.is_some();
  }
  assert!(cursor.terminal_is_empty());let(_,allocation)=observe(||drop(cursor));assert_eq!(allocation.released_bytes,0);(released,frame)
 };
 let results=std::thread::scope(|scope|{let left=scope.spawn(||drain(left));let right=scope.spawn(||drain(right));[left.join().unwrap(),right.join().unwrap()]});
 assert_eq!(results.iter().map(|row|row.0).sum::<usize>(),original);assert_eq!(results.iter().filter(|row|row.1).count(),1);assert_eq!(*source,31);
 eprintln!("[DEBUG] Parallel original returned-read aliases frame-winners=1 original={original} physical={}",results.iter().map(|row|row.0).sum::<usize>());
}
