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
