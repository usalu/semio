//! 🧪️ Shared image imports publish atomic reversible asset and layer edits.
use super::*;
#[test]
fn imported_image_semantics_are_reversible() {
 use protocol::Mutation as _;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
 let operation=semio_framework_plugin::AppOperationContext {app_instance_id:1,parent_document_id:"images".into(),operation_id:2,generation:1,canonical_base_revision:[0;32],authoring_seed:"images".into()};
 for row in fixture["cases"].as_array().unwrap() {
  let snapshot=DrawingSnapshot::default();let original=snapshot.clone();let asset:DrawingImageAsset=serde_json::from_value(row["expected"].clone()).unwrap();
  let emit=publish(&snapshot,&operation,&ImportImage {payload:row["input"]["payload"].as_str().unwrap().into(),name:Some("Artwork.png".into()),parent_id:None,index:None},asset.clone()).unwrap();assert_eq!(emit.artifact_mutations.len(),2);assert_eq!(emit.interaction_writes.len(),1);
  let mut after=snapshot.clone();let mut inverses=Vec::new();for mutation in emit.artifact_mutations {inverses.push(mutation.inverse(&after).unwrap());crate::mutations::apply_drawing_mutation(&mut after,&mutation).unwrap();}
  let crate::DrawingLayerNode::Image(image)=after.layers.get(0).unwrap() else{unreachable!()};assert_eq!(image.width,asset.width as f64);assert_eq!(image.height,asset.height as f64);assert_eq!(after.assets.get(&image.image_key.to_string_owner()),Some(&asset));
  for inverse in inverses.into_iter().rev(){crate::mutations::apply_drawing_mutation(&mut after,&inverse).unwrap();}assert_eq!(after,original);assert_eq!(snapshot,original);
 }
}
#[test]
fn image_creation_requests_supported_encoded_input() {
 let effect=request().effects.pop().unwrap();let Effect::RequestFileOpen {accept,read_as,import_action,multiple,..}=effect else{unreachable!()};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();assert_eq!(accept,fixture["accept"].as_str().unwrap());assert_eq!(import_action,fixture["action"].as_str().unwrap());assert_eq!(read_as.as_deref(),Some("dataUrl"));assert!(!multiple);
}
