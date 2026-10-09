use super::*;
use serde_json::Value;
use semio_framework_value::{retained_clone::{RetainedCloneGrant,RetainedCloneStep},retirement::{admit_owned_retirement,owned_retirement_birth_bytes},FromValue};
fn fixture()->Value{serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn grant(fixture:&Value)->RetainedCloneGrant{serde_json::from_value(fixture["retainedGrant"].clone()).unwrap()}
fn host(fixture:&Value)->DagHost{
 let nodes=fixture["nodes"].as_array().unwrap().iter().map(|id|DagNodeSpec::computation(id.as_str().unwrap().into(),"Node","N","emoji:🔷️".into(),fixture["ports"]["inputs"].as_array().unwrap().iter().map(|id|IoPortSpec{id:id.as_str().unwrap().into(),..Default::default()}).collect(),fixture["ports"]["outputs"].as_array().unwrap().iter().map(|id|IoPortSpec{id:id.as_str().unwrap().into(),..Default::default()}).collect(),false,false,0.0,0.0,160.0,56.0)).collect();
 DagHost::from_host_snapshot_without_layout(DagHostSnapshot{schema:"dag.hostDocument".into(),camera:DagCamera{x:0.0,y:0.0,zoom:2.0},nodes,edges:vec![DagHostSnapshotEdge{id:"e1".into(),source:"a@out".into(),target:"b@in".into(),..Default::default()}]})
}
fn facts(family:&str,fixture:&Value)->DagInputFacts{
 let raw=semio_framework_value::DslValue::from(&fixture[family]);let mut observe=|_|true;let mut control=NativeDecodeControl::new(fixture["ownershipBytes"].as_u64().unwrap()as usize,&mut observe);
 match family{"selection"=>DagInputFacts::Selection(DagSelectionDomains::from_value_controlled(&raw,&mut control).unwrap()),"channels"=>DagInputFacts::Channels(Vec::<DagChannelRef>::from_value_controlled(&raw,&mut control).unwrap()),"progress"=>DagInputFacts::Progress(crate::infinite::board::schema::dag_input::DagComputingProgress::from_value_controlled(&raw,&mut control).unwrap()),_=>DagInputFacts::Statuses(DagNodeStatuses::from_value_controlled(&raw,&mut control).unwrap())}
}
fn retire<T:RetireOwned>(value:T,fixture:&Value)->usize{
 let capacity=fixture["ownershipBytes"].as_u64().unwrap()as usize;let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:owned_retirement_birth_bytes::<T>(),maximum_release_bytes:0,maximum_depth:1};let(mut owner,_)=admit_owned_retirement(value,grant).unwrap_or_else(|_|panic!("controlled original owner admission"));let mut released=0;
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){released+=std::mem::size_of_val(owner.as_ref());drop(owner);return released}let copy=owner.next_copy_byte_demand().unwrap();let birth=owner.next_capacity_byte_demand(copy).unwrap();let release=owner.next_release_byte_demand().unwrap();assert!(birth<=capacity&&release<=capacity);let depth=owner.next_depth_demand().unwrap();let step=owner.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:birth,maximum_release_bytes:release,maximum_depth:depth}).unwrap();let receipt=match step{RetainedCloneStep::Progress(receipt)|RetainedCloneStep::Complete(receipt)=>receipt};released+=receipt.released_bytes;}
 panic!("controlled owner remained live")
}
fn close(host:DagHost,fixture:&Value){let mut owner=DagHostRetirement::new(host);for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){if owner.terminal_is_empty(){return}let copy=owner.next_close_copy_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_close_release_byte_demand().unwrap(),maximum_depth:owner.next_close_depth_demand().unwrap()};owner.close_step(grant).unwrap();}panic!("host remained live")}

#[test]
fn actual_retained_host_preparation_preserves_original_state_until_atomic_exchange(){
 let fixture=fixture();let mut grants=0;let mut releases=0;
 for family in ["selection","channels","progress","statuses"]{
  let mut host=host(&fixture);host.set_selection(&["a".into()]);let baseline=host.engine.selection.clone();let mut app=DagInputApplication::new(facts(family,&fixture));let mut observe=|_|true;let mut control=NativeDecodeControl::new(fixture["ownershipBytes"].as_u64().unwrap()as usize,&mut observe);
  loop{let before=control.owned_bytes();for axis in 0..3{let mut denied=grant(&fixture);let relevant=match axis{0=>{denied.maximum_items=0;true},1=>{denied.maximum_depth=0;true},_=>{denied.maximum_capacity_bytes=0;app.next_capacity_byte_demand().unwrap()!=0}};if relevant{assert!(!app.step(&host,1,&mut control,denied).unwrap());assert_eq!(app.normal_step_progress(),Default::default());assert_eq!(control.owned_bytes(),before);assert_eq!(host.engine.selection,baseline);}}if app.step(&host,1,&mut control,grant(&fixture)).unwrap(){break}grants+=1;assert_eq!(host.engine.selection,baseline);assert!(host.computing_stale.is_empty());assert!(host.node_eval_status.is_empty());}assert_eq!(host.engine.selection,baseline);
  let bytes=owned_retirement_birth_bytes::<DagInputDisplaced>();let zero=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:bytes-1,maximum_release_bytes:0,maximum_depth:1};assert!(admit_owned_retirement(DagInputDisplaced::empty(),zero).is_err());assert_eq!(host.engine.selection,baseline);
  let displaced=app.commit(&mut host);let expected=&fixture["expected"];
  match family{
   "selection"=>{assert_eq!(serde_json::to_value(host.engine.selection.node_ids.iter().copied().collect::<std::collections::BTreeSet<_>>()).unwrap(),expected["nodes"]);assert_eq!(serde_json::to_value(host.engine.selection.edge_ids.iter().copied().collect::<std::collections::BTreeSet<_>>()).unwrap(),expected["edges"]);assert_eq!(serde_json::to_value(host.engine.selection.handle_ids.iter().copied().collect::<std::collections::BTreeSet<_>>()).unwrap(),expected["handles"]);},
   "channels"=>assert_eq!(serde_json::to_value(host.engine.selection.handle_ids.iter().copied().collect::<std::collections::BTreeSet<_>>()).unwrap(),expected["channelHandles"]),
   "progress"=>{assert_eq!(host.computing_active,Some(expected["active"].as_u64().unwrap()));assert_eq!(serde_json::to_value(host.computing_stale.iter().copied().collect::<std::collections::BTreeSet<_>>()).unwrap(),expected["stale"]);},
   _=>{assert_eq!(serde_json::to_value(host.computing_stale.iter().copied().collect::<std::collections::BTreeSet<_>>()).unwrap(),expected["queued"]);assert_eq!(serde_json::to_value(host.unresolved_input_ports.iter().map(|(node,index)|[*node,*index as u64]).collect::<Vec<_>>()).unwrap(),expected["blocked"]);},
  }
  releases+=retire(displaced,&fixture);releases+=retire(app,&fixture);close(host,&fixture);
 }
 println!("[DEBUG] original DagHost preparation families=4 singleUnitGrants={grants} independentBTreeSetSerde=true exactDisplacedReleased={releases} preExchangeNoMutation=true duplicateInputPortOrdinalsPreserved=true");
}
#[test]
fn actual_retained_host_preparation_refuses_every_observed_frontier_without_host_mutation(){
 let fixture=fixture();let mut frontiers=0;
 for family in ["selection","channels","progress","statuses"]{
  let mut host=host(&fixture);host.set_selection(&["a".into()]);let baseline=host.engine.selection.clone();let mut app=DagInputApplication::new(facts(family,&fixture));let mut observations=0;let mut observe=|_|{observations+=1;true};let mut control=NativeDecodeControl::new(fixture["ownershipBytes"].as_u64().unwrap()as usize,&mut observe);while !app.step(&host,1,&mut control,grant(&fixture)).unwrap(){}retire(app,&fixture);
  for stop in 0..observations{let mut app=DagInputApplication::new(facts(family,&fixture));let mut seen=0;let mut observe=|_|{let proceed=seen!=stop;seen+=1;proceed};let mut control=NativeDecodeControl::new(fixture["ownershipBytes"].as_u64().unwrap()as usize,&mut observe);loop{let before=control.owned_bytes();match app.step(&host,1,&mut control,grant(&fixture)){Err(error)=>{assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::Canceled);let receipt=app.normal_step_progress();assert!(receipt.fits(grant(&fixture)));assert_eq!(receipt.retained_capacity_bytes,control.owned_bytes()-before);if receipt.retained_capacity_bytes!=0{assert_eq!(receipt.copied_items,1);}break},Ok(false)=>{},Ok(true)=>panic!("observed frontier did not refuse")}}assert_eq!(host.engine.selection,baseline);assert!(host.node_eval_status.is_empty());assert!(host.computing_stale.is_empty());retire(app,&fixture);frontiers+=1;}
  close(host,&fixture);
 }
 println!("[DEBUG] original DagHost preparation refusedFrontiers={frontiers} fourFamilies=true hostOwnerPreserved=true independentColdFacts=true controlledRetirement=true");
}
