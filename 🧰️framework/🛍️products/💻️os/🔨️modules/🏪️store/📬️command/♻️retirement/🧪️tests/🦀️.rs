use semio_framework_value::{retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::{RetireOwned,owned_retirement_birth_bytes,admit_owned_retirement}};
use crate::os_store::ArtifactCommand;

#[test]
fn original_command_all_variants_retire_whole_physical_custody(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 assert!(ArtifactCommand::<String>::controlled_retirement_supported());
 let rows=fixture["commands"].as_array().unwrap();assert_eq!(rows.len(),20);
 for row in rows{for body in fixture["copyBodies"].as_array().unwrap(){
  let body=body.as_u64().unwrap() as usize;let text=serde_json::to_string(row).unwrap();
  let (command,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_pack_json::from_json_str::<ArtifactCommand<String>>(&text,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
  let original_bytes=birth.requested_bytes-birth.released_bytes;
  assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(&command)).unwrap(),*row);
  if let ArtifactCommand::IngestRemote{envelope}=&command{assert_eq!(serde_json::from_str::<serde_json::Value>(&semio_framework_pack_json::to_json_string(envelope)).unwrap(),fixture["remoteEnvelope"]);}
  let frame=owned_retirement_birth_bytes::<ArtifactCommand<String>>();
  let admitted=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:frame,maximum_depth:1,..Default::default()};
  let mut command=command;
  for denied in [RetainedCloneGrant{maximum_items:0,..admitted},RetainedCloneGrant{maximum_capacity_bytes:frame-1,..admitted},RetainedCloneGrant{maximum_depth:0,..admitted}]{
   let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||admit_owned_retirement(command,denied));
   command=match result{Err((_,original))=>original,Ok(_)=>panic!("command admitted an unfunded original constructor")};
   assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  }
  let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||admit_owned_retirement(command,admitted));
  let (owner,progress)=result.unwrap_or_else(|_|panic!("command exact constructor refused"));assert!(progress.fits(admitted));
  assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
  let mut owner=Some(owner);let mut born=progress.retained_capacity_bytes;let mut disposed=progress.released_bytes;let mut turns=0;
  while owner.is_some(){
   let demand=semio_framework_value::factory_ticket_demands(owner.as_ref().unwrap(),body).unwrap();
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:body.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
   let original_address=std::ptr::from_ref(owner.as_ref().unwrap().as_ref()) as *const () as usize;
   for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),(demand.copy_bytes>0).then(||RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant}),(demand.capacity_bytes>0).then(||RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant}),(demand.release_bytes>0).then(||RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant})].into_iter().flatten(){
    let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_value::close_factory_ticket(&mut owner,denied));
    assert_eq!(step.unwrap().progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    assert_eq!(std::ptr::from_ref(owner.as_ref().unwrap().as_ref()) as *const () as usize,original_address);
    assert_eq!(semio_framework_value::factory_ticket_demands(owner.as_ref().unwrap(),body).unwrap(),demand);
   }
   if demand.depth>0{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_value::close_factory_ticket(&mut owner,RetainedCloneGrant{maximum_depth:demand.depth-1,..grant}));assert!(matches!(step,Err(error)if error.kind==semio_framework_value::ValueRefusalKind::DepthLimit));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(std::ptr::from_ref(owner.as_ref().unwrap().as_ref()) as *const () as usize,original_address);assert_eq!(semio_framework_value::factory_ticket_demands(owner.as_ref().unwrap(),body).unwrap(),demand);}
   let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_value::close_factory_ticket(&mut owner,grant));
   let progress=step.unwrap().progress();assert!(progress.fits(grant));
   assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
   born+=heap.requested_bytes;disposed+=heap.released_bytes;turns+=1;assert!(turns<fixture["maximumTurns"].as_u64().unwrap() as usize);
  }
  assert_eq!(disposed,original_bytes+born);
  let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,fixture["law"]["terminalDropBytes"].as_u64().unwrap() as usize));
  eprintln!("[DEBUG] original command kind={} copy={body} original={original_bytes} born={born} disposed={disposed} turns={turns} terminalDrop0",row["kind"]);
 }}
}
