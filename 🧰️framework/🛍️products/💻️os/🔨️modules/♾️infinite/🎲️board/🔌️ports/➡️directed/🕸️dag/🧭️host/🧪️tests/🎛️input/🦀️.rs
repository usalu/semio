//! 🧪️ dag-host tests (🎛️input): compiled against the `testkit` surface of the board crates.

use semio_os_infinite_board_schema::{DagChannelRef, DagComputingProgress, DagInputFacts, DagNodeStatuses, DagSelectionDomains};
use semio_os_infinite_board_dag_model::DagInputDisplaced;
use semio_os_infinite_board_dag_host::{DagHost, DagHostRetirement, DagInputApplication};
use semio_framework_test_fixtures::fixture_text;
use semio_framework_value::{DslValue, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_framework_value::retained_clone::{RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,RetainedWorkPoll};
use semio_framework_value::retirement::{controlled::ControlledRetirement,RetireOwned};
use semio_framework_value::ValueError;
use semio_framework_value_testing_effect_bank::{EffectBank,Wallet};
use serde_json::Value;
use semio_os_infinite_directed_presentation::*;

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

fn fixture()->Value{serde_json::from_str(fixture_text!("../../../🎛️input/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}

fn grant(fixture:&Value)->RetainedCloneGrant{serde_json::from_value(fixture["retainedGrant"].clone()).unwrap()}

fn host(fixture:&Value)->DagHost{
 let nodes=fixture["nodes"].as_array().unwrap().iter().map(|id|DagNodeSpec::computation(id.as_str().unwrap().into(),"Node","N","emoji:🔷️".into(),fixture["ports"]["inputs"].as_array().unwrap().iter().map(|id|IoPortSpec{id:id.as_str().unwrap().into(),..Default::default()}).collect(),fixture["ports"]["outputs"].as_array().unwrap().iter().map(|id|IoPortSpec{id:id.as_str().unwrap().into(),..Default::default()}).collect(),false,false,0.0,0.0,160.0,56.0)).collect();
 DagHost::from_host_snapshot_without_layout(DagHostSnapshot{schema:"dag.hostDocument".into(),camera:DagCamera{x:0.0,y:0.0,zoom:2.0},nodes,edges:vec![DagHostSnapshotEdge{id:"e1".into(),source:"a@out".into(),target:"b@in".into(),..Default::default()}]})
}

fn facts(family:&str,fixture:&Value)->DagInputFacts{
 let raw=semio_framework_value::DslValue::from(&fixture[family]);let mut observe=|_|true;let mut control=NativeDecodeControl::new(fixture["ownershipBytes"].as_u64().unwrap()as usize,&mut observe);
 match family{"selection"=>DagInputFacts::Selection(DagSelectionDomains::from_value_controlled(&raw,&mut control).unwrap()),"channels"=>DagInputFacts::Channels(Vec::<DagChannelRef>::from_value_controlled(&raw,&mut control).unwrap()),"progress"=>DagInputFacts::Progress(DagComputingProgress::from_value_controlled(&raw,&mut control).unwrap()),_=>DagInputFacts::Statuses(DagNodeStatuses::from_value_controlled(&raw,&mut control).unwrap())}
}

const UNITS:u64=1<<20;

fn bank()->EffectBank{EffectBank::new(Wallet::new(RetainedCloneGrant{maximum_items:1<<40,maximum_copy_bytes:1<<40,maximum_capacity_bytes:1<<40,maximum_release_bytes:1<<40,maximum_depth:1<<20},1<<40))}

fn demand_grant(demand:semio_framework_value::RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}

fn turn<T>(bank:&mut EffectBank,grant:RetainedCloneGrant,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<T,ValueError>)->Result<T,ValueError>{
 let(poll,_)=bank.run(grant,UNITS,|effect|{let before=effect.performed_work();let result=operation(effect);let work=effect.performed_work()-before;RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work})});
 let RetainedWorkPoll::Complete(done)=poll else{panic!("synchronous test turn pended")};assert_ne!(bank.take_admitted(),Some(false));done.result
}

fn advance(bank:&mut EffectBank,app:&mut DagInputApplication,host:&DagHost,control:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant)->Result<bool,ValueError>{turn(bank,grant,|effect|app.step(&host.viewport,&host.graph,1,control,effect))}

fn retire<T:RetireOwned>(bank:&mut EffectBank,value:T,fixture:&Value)->usize{
 let capacity=fixture["ownershipBytes"].as_u64().unwrap()as usize;let mut owner=ControlledRetirement::retain(value);let mut released=0;let mut stalls=0;
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){
  if owner.terminal_is_empty(){return released}
  let demand=owner.next_demand(0).unwrap();assert!(demand.capacity_bytes<=capacity&&demand.release_bytes<=capacity);
  let(poll,_)=bank.run(demand_grant(demand),UNITS,|effect|owner.step(0,effect));assert_ne!(bank.take_admitted(),Some(false));
  let progress=match poll{RetainedWorkPoll::Complete(done)=>done.result.unwrap().progress(),RetainedWorkPoll::Pending{..}=>RetainedCloneProgress::default()};released+=progress.released_bytes;
  if progress==RetainedCloneProgress::default(){stalls+=1;assert!(stalls<16,"controlled owner retirement stalled")}else{stalls=0}
 }
 panic!("controlled owner remained live")
}

fn close(bank:&mut EffectBank,host:DagHost,fixture:&Value){
 let mut owner=DagHostRetirement::new(host);let mut stalls=0;
 for _ in 0..fixture["maximumTurns"].as_u64().unwrap(){
  if owner.terminal_is_empty(){return}
  let demand=owner.next_close_demand(0).unwrap();
  let(poll,_)=bank.run(demand_grant(demand),UNITS,|effect|owner.close_step(0,effect));assert_ne!(bank.take_admitted(),Some(false));
  if let RetainedWorkPoll::Complete(done)=poll{let progress=done.result.unwrap().progress();if progress==RetainedCloneProgress::default(){stalls+=1;assert!(stalls<16,"host retirement stalled")}else{stalls=0}}
 }
 panic!("host remained live")
}

#[test]
fn actual_retained_host_preparation_preserves_original_state_until_atomic_exchange(){
 let fixture=fixture();let mut bank=bank();let mut grants=0;let mut releases=0;
 for family in ["selection","channels","progress","statuses"]{
  let mut host=host(&fixture);host.graph.set_selection(&["a".into()]);let baseline=host.graph.engine.selection.clone();let mut app=DagInputApplication::new(facts(family,&fixture));let mut observe=|_|true;let mut control=NativeDecodeControl::new(fixture["ownershipBytes"].as_u64().unwrap()as usize,&mut observe);
  loop{let before=control.owned_bytes();for axis in 0..3{let mut denied=grant(&fixture);let relevant=match axis{0=>{denied.maximum_items=0;true},1=>{denied.maximum_depth=0;true},_=>{denied.maximum_capacity_bytes=0;app.next_capacity_byte_demand().unwrap()!=0}};if relevant{assert!(!advance(&mut bank,&mut app,&host,&mut control,denied).unwrap());assert_eq!(app.normal_step_progress(),Default::default());assert_eq!(control.owned_bytes(),before);assert_eq!(host.graph.engine.selection,baseline);}}if advance(&mut bank,&mut app,&host,&mut control,grant(&fixture)).unwrap(){break}grants+=1;assert_eq!(host.graph.engine.selection,baseline);assert!(host.paint.computing_stale.is_empty());assert!(host.paint.node_eval_status.is_empty());}assert_eq!(host.graph.engine.selection,baseline);
  let displaced=app.commit(&mut host);let expected=&fixture["expected"];
  match family{
   "selection"=>{assert_eq!(serde_json::to_value(host.graph.engine.selection.node_ids.iter().copied().collect::<std::collections::BTreeSet<_>>()).unwrap(),expected["nodes"]);assert_eq!(serde_json::to_value(host.graph.engine.selection.edge_ids.iter().copied().collect::<std::collections::BTreeSet<_>>()).unwrap(),expected["edges"]);assert_eq!(serde_json::to_value(host.graph.engine.selection.handle_ids.iter().copied().collect::<std::collections::BTreeSet<_>>()).unwrap(),expected["handles"]);},
   "channels"=>assert_eq!(serde_json::to_value(host.graph.engine.selection.handle_ids.iter().copied().collect::<std::collections::BTreeSet<_>>()).unwrap(),expected["channelHandles"]),
   "progress"=>{assert_eq!(host.paint.computing_active,Some(expected["active"].as_u64().unwrap()));assert_eq!(serde_json::to_value(host.paint.computing_stale.iter().copied().collect::<std::collections::BTreeSet<_>>()).unwrap(),expected["stale"]);},
   _=>{assert_eq!(serde_json::to_value(host.paint.computing_stale.iter().copied().collect::<std::collections::BTreeSet<_>>()).unwrap(),expected["queued"]);assert_eq!(serde_json::to_value(host.paint.unresolved_input_ports.iter().map(|(node,index)|[*node,*index as u64]).collect::<Vec<_>>()).unwrap(),expected["blocked"]);},
  }
  releases+=retire(&mut bank,displaced,&fixture);releases+=retire(&mut bank,app,&fixture);close(&mut bank,host,&fixture);
 }
}

#[test]
fn actual_retained_host_preparation_refuses_every_observed_frontier_without_host_mutation(){
 let fixture=fixture();let mut bank=bank();let mut frontiers=0;
 for family in ["selection","channels","progress","statuses"]{
  let mut host=host(&fixture);host.graph.set_selection(&["a".into()]);let baseline=host.graph.engine.selection.clone();let mut app=DagInputApplication::new(facts(family,&fixture));let mut observations=0;let mut observe=|_|{observations+=1;true};let mut control=NativeDecodeControl::new(fixture["ownershipBytes"].as_u64().unwrap()as usize,&mut observe);while !advance(&mut bank,&mut app,&host,&mut control,grant(&fixture)).unwrap(){}retire(&mut bank,app,&fixture);
  for stop in 0..observations{let mut app=DagInputApplication::new(facts(family,&fixture));let mut seen=0;let mut observe=|_|{let proceed=seen!=stop;seen+=1;proceed};let mut control=NativeDecodeControl::new(fixture["ownershipBytes"].as_u64().unwrap()as usize,&mut observe);loop{let before=control.owned_bytes();match advance(&mut bank,&mut app,&host,&mut control,grant(&fixture)){Err(error)=>{assert_eq!(error.kind,semio_framework_value::ValueRefusalKind::Canceled);let receipt=app.normal_step_progress();assert!(receipt.fits(grant(&fixture)));assert_eq!(receipt.retained_capacity_bytes,control.owned_bytes()-before);if receipt.retained_capacity_bytes!=0{assert_eq!(receipt.copied_items,1);}break},Ok(false)=>{},Ok(true)=>panic!("observed frontier did not refuse")}}assert_eq!(host.graph.engine.selection,baseline);assert!(host.paint.node_eval_status.is_empty());assert!(host.paint.computing_stale.is_empty());retire(&mut bank,app,&fixture);frontiers+=1;}
  close(&mut bank,host,&fixture);
 }
}
