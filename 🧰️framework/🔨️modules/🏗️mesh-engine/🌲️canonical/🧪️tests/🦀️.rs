//! 🧪️ Mesh canonical trees equal serde_json's independent rendering of the same payload.
use super::*;
use super::super::{HistoryFoldIndex,MeshAttribute,MeshAttributeDomain,MeshAttributeInterpolation,MeshAttributeSemantic,MeshData,MeshTexture,ComponentReferenceTable};
use semio_framework_pack_json::ArtifactCanonicalJsonTreeCursor;
use pack::value::{DslValue,Number,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneSource}};

fn grant(demand:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes.max(4096),maximum_capacity_bytes:demand.capacity_bytes.max(4096),maximum_release_bytes:demand.release_bytes.max(4096),maximum_depth:demand.depth.max(64)}}

fn settled(poll:pack::value::RetainedWorkPoll<pack::value::retained_clone::RetainedCloneStep,pack::value::OriginalRetirementFailure>)->pack::value::retained_clone::RetainedCloneStep{match poll{pack::value::RetainedWorkPoll::Complete(turn)=>turn.result.unwrap(),pack::value::RetainedWorkPoll::Pending{..}=>panic!("mesh canonical close stayed pending")}}

fn canonical<T:Tree+pack::value::retirement::RetireOwned>(root:T)->Vec<u8>{
 use crate::component_reference_custody_tests::{fx,warm};
 use pack::value::retained_clone::{OriginalEffectReply,RetainedCloneStep,RetainedOwnedProjection,RetainedOwnedProjectionAdmission};
 warm();
 let mut admission=RetainedCloneSource::admission_owned(root,());let mut admitted=None;
 for _ in 0..1_048_576{if admission.terminal_is_empty(){break;}let g=grant(admission.next_demand().unwrap());let step=fx(g,|effect|match admission.phase(){"ready"=>admission.take_output(&mut admitted,effect),"projectionStart"=>admission.project_owned(effect),_=>admission.advance(effect)});step.result.unwrap_or_else(|error|panic!("mesh canonical source birth: {error}"));}
 assert!(admission.terminal_is_empty());let mut source=admitted.unwrap();
 let mut projection_admission=RetainedOwnedProjectionAdmission::<dyn Tree>::default();let mut projection=None;
 let reference=source.borrow().project(0,|value|value as&dyn Tree);
 let mut reply:Option<Result<RetainedCloneStep,pack::value::ValueError>>=None;
 let demand=projection_admission.demand(reference).unwrap();
 fx(grant(demand),|effect|{let slot=OriginalEffectReply::borrow(&mut reply).unwrap();drop(projection_admission.admit(reference,effect,slot).unwrap());});
 assert!(matches!(reply.take().unwrap().unwrap(),RetainedCloneStep::Complete(_)));
 let demand=projection_admission.take_demand().unwrap();
 fx(grant(demand),|effect|{let slot=OriginalEffectReply::borrow(&mut reply).unwrap();drop(projection_admission.take(&mut projection,effect,slot).unwrap());});
 assert!(matches!(reply.take().unwrap().unwrap(),RetainedCloneStep::Complete(_)));
 let projection:RetainedOwnedProjection<dyn Tree>=projection.unwrap();
 let(mut cursor,_)=ArtifactCanonicalJsonTreeCursor::admit(projection,grant(ArtifactCanonicalJsonTreeCursor::constructor_demand())).unwrap_or_else(|(error,_)|panic!("mesh canonical cursor admission: {error}"));
 let mut output=Vec::new();
 for turn in 0..2_000_000{
  if cursor.terminal_is_empty(){break;}
  assert!(turn<1_999_999,"mesh canonical traversal did not settle");
  let demand=cursor.next_demand().unwrap();let mut byte=[0u8];let step=fx(grant(demand),|effect|cursor.advance(&mut byte,effect)).unwrap();output.extend_from_slice(&byte[..step.written_bytes]);
 }
 assert!(cursor.is_complete());drop(cursor);
 for _ in 0..100_000{
  if source.terminal_is_empty(){break;}
  let copy=source.next_close_copy_byte_demand().unwrap();
  let g=grant(RetirementDemand{copy_bytes:copy,capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()});
  settled(crate::component_reference_custody_tests::drive(g,|effect,reply,recover|if recover{source.recover_close_into(copy,effect,reply)}else{source.close_into(copy,effect,reply)}));
 }
 assert!(source.terminal_is_empty());
 output
}

#[test]
fn mesh_data_canonical_tree_matches_serde_json_for_empty_and_fully_populated_payloads(){
 let empty=MeshData::default();
 assert_eq!(serde_json::from_slice::<serde_json::Value>(&canonical(empty.clone())).unwrap(),serde_json::to_value(&empty).unwrap());
 let mut full=MeshData{positions:vec![0.0,1.5,-2.25],normals:vec![0.0,0.0,1.0],colors:vec![1.0,0.5,0.25],indices:vec![0,1,2],uvs:vec![0.0,1.0],face_ids:vec![7],vertex_ids:vec![1,2,3],edge_positions:vec![0.5],edge_ids:vec![4,5],edge_uvs:vec![0.25],edge_is_seam:vec![0,1],paint_texture_base64:Some("AAEC/w==".into()),..Default::default()};
 full.attributes=HistoryFoldIndex::from([("b".to_string(),MeshAttribute{domain:MeshAttributeDomain::Corner,semantic:MeshAttributeSemantic::Custom,interpolation:MeshAttributeInterpolation::Constant,values:vec![DslValue::Number(Number::Float(0.5)),DslValue::String("é".into())],indices:Some(vec![1,0])}),("a".to_string(),MeshAttribute{domain:MeshAttributeDomain::Vertex,semantic:MeshAttributeSemantic::Uv,interpolation:MeshAttributeInterpolation::Linear,values:vec![],indices:None})]);
 full.materials=HistoryFoldIndex::from([("m".to_string(),DslValue::Object(vec![("r".into(),DslValue::Number(Number::UInt(3)))]))]);
 full.textures=HistoryFoldIndex::from([("t".to_string(),MeshTexture{mime:"image/png".into(),bytes:vec![137,80,78,71]})]);
 full.component_references=ComponentReferenceTable::from_entries(vec![("face:2".into(),vec!["x".into(),"y\"".into()]),("edge:1".into(),vec![])]);
 assert_eq!(serde_json::from_slice::<serde_json::Value>(&canonical(full.clone())).unwrap(),serde_json::to_value(&full).unwrap());
 let mut partial=MeshData{positions:vec![1.0],uvs:vec![0.5],..Default::default()};partial.textures=HistoryFoldIndex::from([("t".to_string(),MeshTexture{mime:"m".into(),bytes:vec![]})]);
 assert_eq!(serde_json::from_slice::<serde_json::Value>(&canonical(partial.clone())).unwrap(),serde_json::to_value(&partial).unwrap());
}
