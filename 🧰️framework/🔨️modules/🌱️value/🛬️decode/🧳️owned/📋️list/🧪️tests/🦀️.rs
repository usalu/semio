use semio_framework_test_fixtures::fixture_text;
use super::*;
type Child=[u64;7];
fn fixture()->serde_json::Value{serde_json::from_str(fixture_text!("../../🧪️tests/🧫️fixtures/📐️.json")).unwrap()}
#[test]fn native_persistent_list_receiving_fits_original_interaction_quantum(){
 let f=fixture();
 for row in f["bodyReservations"].as_array().unwrap(){assert_eq!(super::receiving_child_body_budget(row["bodyBytes"].as_u64().unwrap()as usize,row["metadataBytes"].as_u64().unwrap()as usize),row["childBytes"].as_u64().unwrap()as usize);}
 let q=&f["interactionQuantum"];assert!(q["workItems"].as_u64().unwrap()>=1&&q["maximumTurns"].as_u64().unwrap()>=q["workItems"].as_u64().unwrap());
 assert_eq!(f["controlBatch"]["maximumChildPosts"],1);assert_eq!(f["controlBatch"]["independentWork"],true);
}
fn reserve<const N:usize>()->RetirementDemand{NativeOwnedStorage::<[u8;N]>::reserve_demand()}
#[test]fn native_persistent_list_child_custody_prices_stable_birth_and_cancelled_release(){
 let f=fixture();assert_eq!(f["childCustody"]["storage"],"stable-owned-allocation");
 let pointer=std::mem::size_of::<Option<std::ptr::NonNull<u8>>>() as u64;let mut checked=0;
 for row in f["childCustody"]["rows"].as_array().unwrap().iter().filter(|row|row["pointerBytes"].as_u64().unwrap()==pointer){
  let child=row["childBytes"].as_u64().unwrap();assert_eq!(row["capacityBytes"],if row["event"]=="reserve"{child}else{0});assert_eq!(row["releaseBytes"],if row["event"].as_str().unwrap().starts_with("release"){child}else{0});
  if row["event"]=="reserve"{let d=match child{0=>reserve::<0>(),1024=>reserve::<1024>(),other=>panic!("unauthored child size {other}")};assert_eq!(d.copy_bytes as u64,row["copyBytes"].as_u64().unwrap());assert_eq!(d.capacity_bytes as u64,child);assert_eq!(d.depth,1);checked+=1;}
 }
 assert!(checked>=2);
}
