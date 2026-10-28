use super::*;
#[test]
fn complete_original_grants_match_neutral_contract(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧪️fixtures/🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap(){
  if row.get("demand").is_none(){continue}
  let d=&row["demand"];let demand=RetirementDemand{copy_bytes:d["copyBytes"].as_u64().unwrap() as usize,capacity_bytes:d["capacityBytes"].as_u64().unwrap() as usize,release_bytes:d["releaseBytes"].as_u64().unwrap() as usize,depth:d["depth"].as_u64().unwrap() as usize};
  let g=&row["grant"];let supplied=RetainedCloneGrant{maximum_items:g["maximumItems"].as_u64().unwrap() as usize,maximum_copy_bytes:g["maximumCopyBytes"].as_u64().unwrap() as usize,maximum_capacity_bytes:g["maximumCapacityBytes"].as_u64().unwrap() as usize,maximum_release_bytes:g["maximumReleaseBytes"].as_u64().unwrap() as usize,maximum_depth:g["maximumDepth"].as_u64().unwrap() as usize};
  assert_eq!(semio_framework_value::retained_clone::retained_clone_grant_admits(supplied,demand),row["admit"].as_bool().unwrap());
  let exact=grant(demand);assert_eq!(exact.maximum_copy_bytes,demand.copy_bytes);assert_eq!(exact.maximum_capacity_bytes,demand.capacity_bytes);assert_eq!(exact.maximum_release_bytes,demand.release_bytes);assert_eq!(exact.maximum_depth,demand.depth);
 }
}
