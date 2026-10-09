//! 🧪️ Neutral worker policy and actual oversized owner refusal.
use super::*;
#[test]
fn worker_retirement_fixed_policy_preserves_original_owner_on_each_axis_refusal(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 let policy=fixture.get("policy").unwrap();
 assert_eq!(UI_WORKER_RETIREMENT_POLICY.maximum_items,policy["maximum_items"].as_u64().unwrap()as usize);
 assert_eq!(UI_WORKER_RETIREMENT_POLICY.maximum_copy_bytes,policy["maximum_copy_bytes"].as_u64().unwrap()as usize);
 assert_eq!(UI_WORKER_RETIREMENT_POLICY.maximum_capacity_bytes,policy["maximum_capacity_bytes"].as_u64().unwrap()as usize);
 assert_eq!(UI_WORKER_RETIREMENT_POLICY.maximum_release_bytes,policy["maximum_release_bytes"].as_u64().unwrap()as usize);
 assert_eq!(UI_WORKER_RETIREMENT_POLICY.maximum_depth,policy["maximum_depth"].as_u64().unwrap()as usize);
 let mut original=Vec::<u8>::with_capacity(3145728);original.extend_from_slice(b"original");let pointer=original.as_ptr();let capacity=original.capacity();
 for row in fixture["cases"].as_array().unwrap(){
  let demand=&row["demand"];
  let demand=RetirementDemand{copy_bytes:demand["copy_bytes"].as_u64().unwrap()as usize,capacity_bytes:demand["capacity_bytes"].as_u64().unwrap()as usize,release_bytes:demand["release_bytes"].as_u64().unwrap()as usize,depth:demand["depth"].as_u64().unwrap()as usize};
  assert_eq!(ui_worker_retirement_permits(demand),row["admitted"].as_bool().unwrap());
  assert_eq!(original.as_ptr(),pointer);assert_eq!(original.capacity(),capacity);assert_eq!(original.as_slice(),b"original");
 }
 assert!(!ui_worker_retirement_permits(RetirementDemand{release_bytes:capacity,depth:1,..Default::default()}));
 println!("[DEBUG] fixed worker policy retained original pointer/capacity={} across {} independent denial cases; fixed release ceiling={}",capacity,fixture["cases"].as_array().unwrap().len(),UI_WORKER_RETIREMENT_POLICY.maximum_release_bytes);
}

/// 🛑️ Neutral independent refusal identities preserve the original fixed policy.
#[test]
fn worker_retirement_refusal_identity_is_observable_without_retry_or_owner_release(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let d=&row["demand"];let demand=RetirementDemand{copy_bytes:d["copy_bytes"].as_u64().unwrap()as usize,capacity_bytes:d["capacity_bytes"].as_u64().unwrap()as usize,release_bytes:d["release_bytes"].as_u64().unwrap()as usize,depth:d["depth"].as_u64().unwrap()as usize};
  let expected=if demand.depth>UI_WORKER_RETIREMENT_POLICY.maximum_depth{Some(fixture["refusal"]["depth"].as_str().unwrap())}else if demand.capacity_bytes>UI_WORKER_RETIREMENT_POLICY.maximum_capacity_bytes{Some(fixture["refusal"]["capacity"].as_str().unwrap())}else if demand.release_bytes>UI_WORKER_RETIREMENT_POLICY.maximum_release_bytes{Some(fixture["refusal"]["release"].as_str().unwrap())}else if demand.copy_bytes>UI_WORKER_RETIREMENT_POLICY.maximum_copy_bytes{Some(fixture["refusal"]["copy"].as_str().unwrap())}else{None};
  assert_eq!(ui_worker_retirement_admission(demand).err().map(ValueRefusalKind::as_str),expected);
 }
 assert_eq!(fixture["refusal"]["owner"],"retain");assert_eq!(fixture["refusal"]["automaticRetry"],false);
 println!("[DEBUG] original UI worker policy publishes exact independent refusal identities; owner retained and no automatic retry");
}
