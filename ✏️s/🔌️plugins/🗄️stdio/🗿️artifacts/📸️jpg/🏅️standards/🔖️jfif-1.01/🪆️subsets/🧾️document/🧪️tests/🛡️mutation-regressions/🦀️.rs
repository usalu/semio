use crate::{JpgSnapshot,JpgDiff,JpgMutation};
use crate::schema::snapshot::{JfifDensityUnits,JfifThumbnail,JpgSegment};
use crate::schema::mutations::*;
use protocol::{DiffAlgebra,Mutation};
fn fixture()->JpgSnapshot{semio_framework_pack_json::from_json_str(include_str!("../../🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
#[test]
fn semantic_content_intents_apply_inverse_and_compose_without_native_authority(){
 let base=fixture();
 let intents=vec![JpgMutation::ReplacePixels(ReplacePixelsMutation{pixels:vec![0,255,128,255]}),JpgMutation::InsertOtherSegment(InsertOtherSegmentMutation{index:0,segment:JpgSegment{marker:254,data:vec![0,255]}}),JpgMutation::RemoveOtherSegment(RemoveOtherSegmentMutation{index:0})];
 for intent in intents{let forward=intent.diff(&base);let next=protocol::apply_diff(forward.diff(),&base).unwrap();let inverse=forward.diff().inverse(&base);assert_eq!(protocol::apply_diff(&inverse,&next).unwrap(),base);let mut composed=forward.diff().clone();composed.absorb(inverse);assert_eq!(protocol::apply_diff(&composed,&base).unwrap(),base);}
}
#[test]
fn semantic_between_preserves_density_thumbnail_and_duplicate_metadata_positions(){
 let base=fixture();let mut next=base.clone();next.image.width=u32::MAX;next.image.jfif_density_units=JfifDensityUnits::PixelsPerCm;next.image.jfif_thumbnail=Some(JfifThumbnail{width:0,height:0,rgb_data:Vec::new()});next.image.other_segments.insert(0,JpgSegment{marker:254,data:vec![1,2,3]});
 let diff=JpgDiff::between(&base,&next);assert_eq!(protocol::apply_diff(&diff,&base).unwrap(),next);assert_eq!(protocol::apply_diff(&diff.inverse(&base),&next).unwrap(),base);
 let json=semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&diff));let fields:serde_json::Value=serde_json::from_str(&json).unwrap();for field in ["frame","sofMarker","arithmetic","quantTables","huffmanTables","restartInterval"]{assert!(fields.get(field).is_none());}
 println!("[DEBUG] JPEG sparse content diff independently parsed by serde_json");
}
