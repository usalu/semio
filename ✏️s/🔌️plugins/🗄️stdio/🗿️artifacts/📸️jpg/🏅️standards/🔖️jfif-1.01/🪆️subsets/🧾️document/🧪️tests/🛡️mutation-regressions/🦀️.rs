use crate::{JpgSnapshot,JpgDiff,JpgMutation};
use crate::schema::snapshot::{JfifDensityUnits,JfifThumbnail,JpgSegment};
use crate::schema::mutations::*;
use protocol::{DiffAlgebra,Mutation,MutationDiff};
fn fixture()->JpgSnapshot{semio_framework_pack_json::from_json_str(include_str!("../../🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json"),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()}
#[test]
fn semantic_content_intents_apply_inverse_and_compose_without_native_authority(){
 let base=fixture();
 let intents=vec![JpgMutation::ReplacePixels(ReplacePixelsMutation{pixels:vec![0,255,128,255]}),JpgMutation::InsertOtherSegment(InsertOtherSegmentMutation{index:0,segment:JpgSegment{marker:254,data:vec![0,255]}}),JpgMutation::RemoveOtherSegment(RemoveOtherSegmentMutation{index:0})];
 for intent in intents{let forward=intent.diff(&base);let next=protocol::apply_diff(forward.diff(),&base).unwrap();let inverse=forward.diff().inverse(&base);assert_eq!(protocol::apply_diff(&inverse,&next).unwrap(),base);let mut composed=forward.diff().clone();composed.absorb(inverse);assert_eq!(protocol::apply_diff(&composed,&base).unwrap(),base);}
}
