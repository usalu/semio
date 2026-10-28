//! 🧪️ Native original edit hashing agrees with independent Python hashlib and the existing whole-value reference.
use super::*;
#[test]
fn native_edit_digest_preserves_single_and_chained_contracts(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for case in cases.as_array().unwrap(){let wire=serde_json::to_string(&case["edit"]).unwrap();let value=semio_framework_pack_json::from_json_str(&wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let edit=Edit::<bool>::from_value(value).unwrap();let mut cursor=NativeEditDigest::default();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:64};
 for _ in 0..100000{cursor.advance(&edit,grant).unwrap();if cursor.digest().is_some(){break}}
 let expected:Vec<u8>=case["digest"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();assert_eq!(cursor.digest().unwrap().as_slice(),expected);assert_eq!(cursor.digest().unwrap(),CursorRevisionAccumulator::edit_digest(&edit));
 }
}

#[derive(Default,PartialEq)]
struct Delta;
impl ToValue for Delta{fn to_value(&self)->semio_framework_value::DslValue{semio_framework_value::DslValue::Null}}
impl semio_framework_os_spr::DiffAlgebra<()> for Delta{fn is_empty(&self)->bool{true}}
impl semio_framework_os_spr::MutationDiff<()> for Delta{}
struct Operation(bool);
impl ToValue for Operation{fn to_value(&self)->semio_framework_value::DslValue{self.0.to_value()}}
impl FromValue for Operation{fn from_value(value:semio_framework_value::DslValue)->Result<Self,ValueError>{Ok(Self(bool::from_value(value)?))}}
impl Mutation<()> for Operation{type Diff=Delta;const DESCRIPTORS:&'static[semio_framework_os_spr::MutationLeafDescriptor]=&[];fn descriptor(&self)->&'static semio_framework_os_spr::MutationLeafDescriptor{panic!("hash fixture never asks for semantic execution")}}
#[test]
fn native_effective_digest_preserves_borrowed_replacement_contract(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let wire=serde_json::to_string(&cases[2]["edit"]).unwrap();let value=semio_framework_pack_json::from_json_str(&wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let edit=Edit::<Operation>::from_value(value).unwrap();let original=CursorRevisionAccumulator::edit_digest(&edit);
 let mut map=crate::lifted_replay::EffectiveSupersessions::new();
 for row in [0,2]{map.insert(MutationId(format!("{}#{row}",edit.id)),semio_framework_os_spr::EffectiveSupersession{transition_id:format!("energy:replacement-{row}"),actor:"energy:test".into(),timestamp:semio_framework_os_spr::HybridLogicalTimestamp::new(1,2),scope:None,replacement:if row==0{semio_framework_os_spr::InputReplacement::Withdrawn}else{semio_framework_os_spr::InputReplacement::Input{schema:"energy/model".into(),payload:vec![0,255,3,16,32]}}});}
 let expected=CursorRevisionAccumulator::effective_edit_digest::<(),Operation>(&edit,original,&map);let mut cursor=NativeEffectiveDigest::default();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1};
 for _ in 0..10000{let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance::<(),Operation>(&edit,&map,original,grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if cursor.digest().is_some(){break}}
 assert_eq!(cursor.digest(),Some(expected));let oracle:Vec<u8>=cases[2]["effectiveDigest"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();assert_eq!(expected.as_slice(),oracle);
}

#[test]
fn native_staged_inverse_digest_preserves_live_original_and_independent_bytes(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for case in cases.as_array().unwrap(){let wire=serde_json::to_string(&case["edit"]).unwrap();let value=semio_framework_pack_json::from_json_str(&wire,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let mut edit=Edit::<bool>::from_value(value).unwrap();let candidate=std::mem::take(&mut *edit.inverse);edit.inverse.push(true);edit.inverse.push(true);edit.inverse.push(false);let live=edit.inverse.iter().copied().collect::<Vec<_>>();let mut cursor=NativeEditDigest::default();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:64};for _ in 0..100000{let(p,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance_with_inverse(&edit,&candidate,grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));assert_eq!(edit.inverse.iter().copied().collect::<Vec<_>>(),live);if cursor.digest().is_some(){break}}let expected=case["digest"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect::<Vec<_>>();assert_eq!(cursor.digest().unwrap().as_slice(),expected);}
}
