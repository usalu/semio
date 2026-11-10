//! 🧪️ Catalogue derivation preserves exact genuine registry metadata, generation and controlled custody.
use super::*;
use semio_framework_value::FromValue;
fn grant(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}}
fn pump<R>(grant:RetainedCloneGrant,run:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->R)->Option<R>{semio_framework_job::OriginalRetirementPump::new().turn(1<<16,u64::MAX,||Some(0),grant).expect("catalogue law turn").with_effect(run).expect("catalogue law effect").map(|(output,_)|output)}
fn close(work:&mut FlowOperatorCatalogueWork){work.begin_close();for _ in 0..1000000{if work.terminal_is_empty(){return}let demand=work.next_close_demands(4096).unwrap();if let Some(semio_framework_value::RetainedWorkPoll::Complete(turn))=pump(grant(demand),|effect|work.close_step(4096,effect)){turn.result.unwrap();}}panic!("original catalogue did not close")}
fn close_fields<T:RetireOwned>(source:T){let mut owner=ControlledRetirement::new(source).unwrap_or_else(|_|panic!("catalogue fixture fields support controlled retirement"));for _ in 0..1000000{if owner.terminal_is_empty(){return}let demand=owner.selected_demand(4096).unwrap();if let Some(semio_framework_value::RetainedWorkPoll::Complete(turn))=pump(grant(demand),|effect|owner.step(4096,effect)){turn.result.unwrap();}}panic!("catalogue fixture did not close")}
fn close_registry(owner:&mut neural_engine::RegistryRetirement){for _ in 0..1000000{if owner.terminal_is_empty(){return}let copy=owner.next_copy_byte_demand().unwrap().max(4096);let demand=RetirementDemand{copy_bytes:copy,capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),release_bytes:owner.next_release_byte_demand().unwrap(),depth:owner.next_depth_demand().unwrap()};if let Some(semio_framework_value::RetainedWorkPoll::Complete(turn))=pump(grant(demand),|effect|owner.close_step(copy,effect)){turn.result.unwrap();}}panic!("original registry did not close")}
fn fixture(row:&semio_framework_pack_json::Value)->(SharedRegistry,neural_engine::RegistryRetirement){let mut registry=neural_engine::Registry::new();for info in row.get("operators").unwrap().as_array().unwrap(){let info=OperatorInfo::from_value(semio_framework_pack_json::to_dsl_value(info)).unwrap();registry.register_operator(info,Vec::new(),&[])}SharedRegistry::new(registry)}
#[test]
fn genuine_catalogue_work_matches_original_registry_and_neutral_sorted_ids(){
 let cases=semio_framework_pack_json::parse(include_str!("📁️cases.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 for row in cases.as_array().unwrap(){
  let(registry,mut retirement)=fixture(row);let identity=registry.owner_identity();let expected:Infos=registry.operator_infos().map(|info|(info.id.clone(),info.clone())).collect();let generation=row.get("generation").unwrap().as_u64().unwrap();
  let original_default=registry.operator_info("zeta").unwrap().inputs[0].default.as_ref().unwrap().as_dictionary().unwrap().original_root_identity();
  let mut work=FlowOperatorCatalogueWork::new(registry,generation);let mut output=None;
  for _ in 0..1000000{let demand=work.next_demands(4096,u64::MAX).unwrap();let original=grant(demand);assert!(pump(RetainedCloneGrant{maximum_items:0,..original},|effect|work.advance(4096,effect)).unwrap().unwrap().is_none());assert_eq!(work.step_progress(),Default::default());let result=pump(original,|effect|work.advance(4096,effect)).unwrap().unwrap();assert!(work.step_progress().fits(original));if let Some(catalogue)=result{output=Some(catalogue);break}}
  let catalogue=output.expect("original catalogue transfers once");assert_eq!(catalogue.generation,generation);assert_eq!(catalogue.registry.owner_identity(),identity);assert_eq!(&*catalogue.infos,&expected);assert_eq!(catalogue.infos.get("zeta").unwrap().inputs[0].default.as_ref().unwrap().as_dictionary().unwrap().original_root_identity(),original_default);
  let ids:Vec<_>=catalogue.infos.keys().map(String::as_str).collect();let expected_ids:Vec<_>=row.get("expectedIds").unwrap().as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect();assert_eq!(ids,expected_ids);
  work.restore_output(catalogue);close(&mut work);close_fields(expected);close_registry(&mut retirement);
 }
}
#[test]
fn genuine_catalogue_cancellation_preserves_same_registry_retirement(){
 let cases=semio_framework_pack_json::parse(include_str!("📁️cases.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let row=&cases.as_array().unwrap()[0];
 for stop in 0..50{let(registry,mut retirement)=fixture(row);let mut work=FlowOperatorCatalogueWork::new(registry,7);for _ in 0..stop{let demand=work.next_demands(4096,u64::MAX).unwrap();assert!(pump(grant(demand),|effect|work.advance(4096,effect)).unwrap().unwrap().is_none())}close(&mut work);close_registry(&mut retirement)}
}
