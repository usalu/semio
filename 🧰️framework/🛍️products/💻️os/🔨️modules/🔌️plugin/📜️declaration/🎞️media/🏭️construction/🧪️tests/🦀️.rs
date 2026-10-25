//! 📏️ The neutral queue corpus uses native framework layouts for every supported platform.
use super::*;
#[test]
fn original_output_birth_quote_matches_neutral_slot_bounds(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let demand=ArtifactOutputChunks::original_birth_demands(row["maximum"].as_u64().unwrap()as usize);
  assert_eq!(demand.is_ok(),row["accepted"].as_bool().unwrap());
  if let Ok(demand)=demand{
   let slots=row["slots"].as_u64().unwrap()as usize;
   assert_eq!(demand,RetirementDemand{copy_bytes:size_of::<ArtifactOutputChunks>(),capacity_bytes:slots*size_of::<Vec<u8>>()+semio_framework_value::shared_retirement_allocation_bytes::<ArtifactOutputChunksInner>(),depth:1,..Default::default()});
  }
 }
}
