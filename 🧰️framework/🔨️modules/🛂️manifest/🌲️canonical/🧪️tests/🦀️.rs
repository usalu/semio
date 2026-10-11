//! 🧪️ Host contribution canonical trees equal serde_json's parse of their `ToValue` wire.
use super::*;
use semio_framework_pack_json::ArtifactCanonicalJsonTreeCursor;
use semio_framework_value::{DslValue,Number,RetirementDemand,ToValue,retained_clone::{RetainedCloneGrant,RetainedCloneSource}};

fn grant(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(4096),maximum_capacity_bytes:demand.capacity_bytes.max(4096),maximum_release_bytes:demand.release_bytes.max(4096),maximum_depth:demand.depth.max(64)}}

fn canonical<T:Tree+semio_framework_value::retirement::RetireOwned>(root:T)->Vec<u8>{
 let birth=RetainedCloneSource::<T>::owned_constructor_demand::<()>();
 let(mut source,_)=RetainedCloneSource::admit_owned(root,(),grant(RetirementDemand{copy_bytes:RetainedCloneSource::<T>::constructor_copy_bytes(),capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()})).unwrap_or_else(|(error,_,_)|panic!("contribution canonical source birth: {error}"));
 let projection=source.project_owned(0,|value|value as&dyn Tree,grant(RetirementDemand{copy_bytes:source.borrow().binding_copy_bytes(),depth:1,..Default::default()})).unwrap().0;
 let(mut cursor,_)=ArtifactCanonicalJsonTreeCursor::admit(projection,grant(ArtifactCanonicalJsonTreeCursor::constructor_demand())).unwrap_or_else(|(error,_)|panic!("contribution canonical cursor admission: {error}"));
 let mut output=Vec::new();
 for turn in 0..1_000_000{
  if cursor.terminal_is_empty(){break;}
  assert!(turn<999_999,"contribution canonical traversal did not settle");
  let demand=cursor.next_demand().unwrap();let mut byte=[0u8];let step=cursor.advance(&mut byte,grant(demand)).unwrap();output.extend_from_slice(&byte[..step.written_bytes]);
 }
 assert!(cursor.is_complete());drop(cursor);
 for _ in 0..100_000{
  if source.terminal_is_empty(){break;}
  let copy=source.next_close_copy_byte_demand().unwrap();
  source.close_step(grant(RetirementDemand{copy_bytes:copy,capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()})).unwrap();
 }
 assert!(source.terminal_is_empty());
 output
}

#[test]
fn program_contribution_canonical_tree_matches_its_to_value_wire_through_serde_json(){
 let payload=DslValue::Object(vec![("k".into(),DslValue::Array(vec![DslValue::Null,DslValue::Number(Number::Int(-2)),DslValue::String("é\"".into())]))]);
 for entry in [ProgramContributionEntry{plugin_id:"a.b".into(),topic_contribution:Some(TopicContribution::new("flow.extension",payload))},ProgramContributionEntry{plugin_id:"c".into(),topic_contribution:None}]{
  let wire=serde_json::Value::from(entry.to_value());
  let actual:serde_json::Value=serde_json::from_slice(&canonical(entry)).unwrap();
  assert_eq!(actual,wire);
 }
}
