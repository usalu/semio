use semio_plugin_guest_contract::original_actor_context::original_core_first_response_reserve;
#[test]
fn authored_mcp_policies_fund_actual_owner_first_response_without_expanding_runtime_grants(){
 let policy:semio_plugin_host::GuestRelayDriverPolicy=semio_framework_pack_json::from_json_str(include_str!("../../🏛️host-driver-policy.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let model:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for length in[144,152,184,200]{
  let(items,demand,work)=original_core_first_response_reserve(length).unwrap();
  assert_eq!(items,model["minimumItems"].as_u64().unwrap()as usize);
  assert_eq!(work,model["minimumWork"].as_u64().unwrap()-if length==model["maximumHeaderBytes"].as_u64().unwrap()as usize{0}else{2});
  for grant in[policy.admission,policy.drive,policy.wake]{
   assert!(grant.maximum_items>=items+model["producerItems"].as_u64().unwrap()as usize);
   assert!(grant.maximum_copy_bytes>=demand.copy_bytes);
   assert!(grant.maximum_capacity_bytes>=demand.capacity_bytes);
   assert!(demand.release_bytes>0);
   assert!(grant.maximum_release_bytes>=demand.release_bytes);
   assert!(grant.maximum_depth>=demand.depth);
  }
  assert!(policy.root.turn_fuel>=work+model["producerWork"].as_u64().unwrap());
 }
}
/// 🪙️ The guest infrastructure grant is cumulative per operation: it must fund the first-response reserves of every close crossing beside at least 16 times the measured five-turn open cost of the note actor and the largest staged actor image.
#[test]
fn authored_operation_policy_infrastructure_funds_the_largest_staged_actor_and_every_close_reserve(){
 let policy:serde_json::Value=serde_json::from_str(include_str!("../../💡️host-operation-policy.json")).unwrap();
 let models:serde_json::Value=serde_json::from_str(include_str!("../../../📡️stdio/📦️component-models/🔣️.json")).unwrap();
 let retained=&policy["infrastructure"]["retained"];
 let axis=|name:&str|retained[name].as_u64().unwrap() as usize;
 let largest_actor=models["definitions"]["Protocol"]["const"]["maximumPreparedActorBytes"].as_u64().unwrap() as usize;
 assert!(axis("maximumCopyBytes")>=largest_actor);
 const FIVE_TURNS:[usize;5]=[244375,38971533,3836911,1828185,9998493];
 assert!(axis("maximumItems")>=FIVE_TURNS[0]*16);
 assert!(axis("maximumCapacityBytes")>=FIVE_TURNS[2]*64);
 assert!(axis("maximumReleaseBytes")>=FIVE_TURNS[3]*64);
 assert!(policy["infrastructure"]["workFuel"].as_u64().unwrap() as usize>=FIVE_TURNS[4]*64);
 let close:[(usize,semio_framework_value::RetirementDemand,u64);5]=[200,144,184,184,184].map(|header|original_core_first_response_reserve(header).unwrap());
 let items:usize=close.iter().map(|row|row.0).sum();
 let copy:usize=close.iter().map(|row|row.1.copy_bytes).sum();
 let work:u64=close.iter().map(|row|row.2).sum();
 assert!(axis("maximumItems")>=items&&axis("maximumCopyBytes")>=copy&&policy["infrastructure"]["workFuel"].as_u64().unwrap()>=work);
}
