//! 🥞️ Original flattening preserves exact rendered pixels, placement, spare assets and inverse history.
use semio_s_artifact_raster_raster_model::{RasterLayerNode,SemioImageSnapshot,snapshot::RasterSnapshot};
use semio_s_artifact_raster_raster_editor::{RasterCommand,commands::flatten_layers::FlattenLayers};
use semio_plugin::artifact_app_laws::OriginalWorkFixture;
use protocol::{ColdMutation,Mutation};
#[path="🎟️testkit/🦀️.rs"]mod funded;
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn retire(document:RasterSnapshot){semio_s_artifact_raster_raster_model::snapshot::retire_raster_snapshot(document);}
#[test]
fn flatten_preserves_the_composite_and_restores_every_original_layer(){
    let fixture=fixture();
    for capacity in fixture["assetCapacities"].as_array().unwrap(){
        let mut document=semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot();
        document.layers=semio_framework_pack_json::from_json_str(&fixture["layers"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        for(key,value)in fixture["images"].as_object().unwrap(){let image=semio_s_artifact_raster_raster_model::semio_image_from_rgba8(value["width"].as_u64().unwrap() as u32,value["height"].as_u64().unwrap() as u32,serde_json::from_value(value["pixels"].clone()).unwrap());document.assets.insert(key.clone(),semio_s_artifact_raster_raster_model::mint_raster_image_child(key,&image)).unwrap();}
        let image=document.assets.get("blue").unwrap().local_owner::<SemioImageSnapshot>().unwrap();
        for index in document.assets.len()..capacity.as_u64().unwrap() as usize{let key=format!("spare-{index}");document.assets.insert(key.clone(),semio_s_artifact_raster_raster_model::mint_raster_image_child(&key,&image)).unwrap();}
        let before=document.clone();let command=RasterCommand::FlattenLayers(FlattenLayers{name:"Flattened Image".into()});let emit=funded::bake::<false>(&command,&document,None).unwrap().unwrap();let mut caller=OriginalWorkFixture::new();let mut inverses=Vec::new();
        for mutation in &emit.artifact_mutations{inverses.push(mutation.inverse(&document).unwrap());let(next,messages)=semio_os_kernel::apply_mutation(&document,mutation).unwrap();assert!(messages.is_empty());retire(std::mem::replace(&mut document,next));assert!(document.assets.len()<=64);}
        assert_eq!(document.layers.len(),1);assert_eq!(document.assets.len(),before.assets.len()-fixture["images"].as_object().unwrap().len()+1);
        for(key,asset)in &before.assets{if key.starts_with("spare-"){assert_eq!(document.assets.get(key),Some(asset));}}
        let RasterLayerNode::Pixel{image_key:Some(key),transform,name,..}=&document.layers[0]else{panic!("flattened pixels")};assert_eq!(name,"Flattened Image");assert_eq!((transform.x,transform.y),(-3.0,4.5));assert!(key.starts_with("image-"));
        let image=document.assets.get(key).unwrap().local_owner::<SemioImageSnapshot>().unwrap();assert_eq!(serde_json::to_value(&image.frames[0].rgba8).unwrap(),fixture["expected"]["pixels"]);
        for inverse in inverses.into_iter().rev().flatten(){let(next,messages)=semio_os_kernel::apply_mutation(&document,&inverse).unwrap();assert!(messages.is_empty());retire(std::mem::replace(&mut document,next));caller.retire_owned(inverse,funded::loan());}
        assert_eq!(document,before);caller.retire_owned(emit,funded::loan());caller.finish();retire(document);retire(before);
    }
}
#[test]
fn flatten_preparation_is_cancellable_and_validates_the_name(){
    let fixture=fixture();let mut document=semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot();document.layers=semio_framework_pack_json::from_json_str(&fixture["layers"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for name in ["".to_owned(),"   ".into(),"x".repeat(121)]{let command=RasterCommand::FlattenLayers(FlattenLayers{name});assert!(funded::bake::<false>(&command,&document,None).is_err());}
    let before=document.clone();let command=RasterCommand::FlattenLayers(FlattenLayers{name:"Image".into()});for cut in [0,1,2,8]{assert!(funded::bake::<false>(&command,&document,Some(cut)).unwrap().is_none());assert_eq!(document,before);}retire(document);retire(before);
}
#[test]
fn flatten_refuses_a_protected_descendant_without_changing_the_document(){
    let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🧫️fixtures/🔒️protection/🔣️.json")).unwrap();let mut document=semio_s_artifact_raster_raster_model::io::text::snapshot::snapshot_wire_codec::empty_raster_snapshot();document.layers=semio_framework_pack_json::from_json_str(&fixture["layers"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let before=document.clone();let command=RasterCommand::FlattenLayers(FlattenLayers{name:"Flattened".into()});let error=funded::bake::<false>(&command,&document,None).unwrap_err();assert!(error.contains("locked"));assert_eq!(document,before);retire(document);retire(before);
}
