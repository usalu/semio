use crate::{DiffAlgebra,MutationDiff};
use semio_framework_value::{DslValue,ToValue};
use semio_framework_value::retirement::{controlled::ControlledRetirement,RetireOwned,RetirementCursor};
use semio_framework_value::retained_clone::RetainedCloneGrant;
#[derive(Default,PartialEq,semio_framework_value::RetireOwned)]
struct NativeOnlyDiff{value:Option<i64>}
impl ToValue for NativeOnlyDiff{fn to_value(&self)->DslValue{DslValue::object(self.value.into_iter().map(|value|("value".into(),value.to_value())))}}
impl DiffAlgebra<()> for NativeOnlyDiff{fn is_empty(&self)->bool{self.value.is_none()}}
impl MutationDiff<()> for NativeOnlyDiff{}
fn require_native_diff<T:MutationDiff<()>>(_: &T){}
/// 🧪️ Native stored deltas implement neither synchronous receiving nor unpriced coalescing.
#[test]
fn native_delta_vocabulary_does_not_require_cold_coalescing_or_receiving(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for item in corpus["cases"].as_array().unwrap(){
  let diff=NativeOnlyDiff{value:item["value"].as_i64()};require_native_diff(&diff);
  let expected:DslValue=semio_framework_pack_json::from_json_str(&item["expected"]["wire"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
  assert_eq!(diff.is_empty(),item["expected"]["empty"].as_bool().unwrap());assert_eq!(diff.to_value(),expected);
  let mut close=ControlledRetirement::new(diff).map_err(|(error,_)|error).unwrap();
  while !close.terminal_is_empty(){let demand=close.next_demand(4096).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};assert!(close.step(grant).unwrap().progress().fits(grant));}
  eprintln!("[DEBUG] native-only delta {:?}: complete neutral output, explicit native retirement, no cold decoder/coalescer",item["name"]);
 }
}
