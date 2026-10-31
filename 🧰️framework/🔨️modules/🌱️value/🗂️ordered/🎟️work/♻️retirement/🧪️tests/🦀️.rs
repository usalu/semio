use super::*;
#[test]
fn ordered_retirement_reports_defining_work_and_distinguishes_key_header_clear_from_copy(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 let mut original=OrderedMap::new();let mut oracle=std::collections::BTreeMap::new();
 for row in law["rows"].as_array().unwrap(){let key=row[0].as_str().unwrap().to_owned();let value=row[1].as_u64().unwrap();oracle.insert(key.clone(),value);original.insert(key,value);}
 let mut owner=original.retire();let mut stock:RetainedCloneGrant=serde_json::from_value(law["stock"].clone()).unwrap();let mut cpu=law["maximumCpuWork"].as_u64().unwrap();let mut values=Vec::new();
 for _ in 0..law["maximumTurns"].as_u64().unwrap(){
  if owner.terminal_is_empty(){break}
  let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:owner.next_copy_byte_demand(),maximum_capacity_bytes:0,maximum_release_bytes:owner.next_close_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand()};
  assert_eq!(g.maximum_copy_bytes,law["expected"]["keyHeaderPhysicalCopyBytes"].as_u64().unwrap()as usize);
  assert!(g.maximum_items<=stock.maximum_items&&g.maximum_copy_bytes<=stock.maximum_copy_bytes&&g.maximum_release_bytes<=stock.maximum_release_bytes&&g.maximum_depth<=stock.maximum_depth);
  let denied=owner.advance(RetainedCloneGrant{maximum_items:0,..g});assert_eq!(owner.last_turn_work(),law["expected"]["deniedCpu"].as_u64().unwrap());assert!(matches!(denied,RetirementStep::Blocked));
  let step=owner.advance(g);assert_eq!(owner.last_turn_work(),law["expected"]["paidEventCpu"].as_u64().unwrap());cpu=cpu.checked_sub(owner.last_turn_work()).unwrap();
  match step{
   RetirementStep::Progress{released_items,released_bytes}=>{stock.maximum_items-=released_items;stock.maximum_release_bytes-=released_bytes},
   RetirementStep::ProcessedBytes(bytes)=>{assert!(bytes>0);stock.maximum_items-=1;assert_eq!(g.maximum_copy_bytes,0)},
   RetirementStep::OwnedValue(value)=>{stock.maximum_items-=1;values.push(value)},
   RetirementStep::Failure(error)=>panic!("{error}"),
   RetirementStep::Blocked|RetirementStep::Complete=>panic!("genuine paid ordered close did not advance")
  }
 }
 assert!(owner.terminal_is_empty());values.sort();let expected:Vec<_>=oracle.values().copied().collect();assert_eq!(values,expected);assert_eq!(serde_json::to_value(values).unwrap(),law["expectedValues"]);assert!(cpu>0);
 let _=owner.advance(Default::default());assert_eq!(owner.last_turn_work(),law["expected"]["cachedCpu"].as_u64().unwrap());
}
