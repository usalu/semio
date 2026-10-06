//! 🧪️ Merge-down publication preserves surrounding artwork, asset ownership and inverse history.
use super::*;
use protocol::Mutation;
use semio_framework_pixels::{RasterImage,png_encoding::PngEncodeJob};
fn fixture()->serde_json::Value {serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
fn document(layers:&serde_json::Value,fixture:&serde_json::Value)->RasterSnapshot {
    let mut document=crate::standards::v1::subsets::any::io::text::snapshot::empty_raster_snapshot();
    document.layers=semio_framework_pack_json::from_json_str(&layers.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for (key,value) in fixture["images"].as_object().unwrap() {
        let image=RasterImage {width:value["width"].as_u64().unwrap() as u32,height:value["height"].as_u64().unwrap() as u32,pixels:value["pixels"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect()};
        let asset=crate::RasterImageAsset {mime:"image/png".into(),data:semio_framework_pixels::encode_png(&image).unwrap()};
        document.assets.insert(key.clone(),crate::mint_raster_asset_child(key,&asset)).unwrap();
    }
    document
}
fn composite(document:&RasterSnapshot)->semio_framework_pixels::compositing::layers::RasterStackResult {
    let mut job=crate::standards::v1::subsets::any::io::raster_composite_job(document).unwrap();while !job.advance(1).unwrap().done {}job.into_result().unwrap()
}
fn retire(document:RasterSnapshot) {crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);}
#[test]
fn merge_preserves_pixels_parent_siblings_shared_assets_and_exact_inverse() {
    let fixture=fixture();
    for row in fixture["cases"].as_array().unwrap() {
        let mut document=document(&row["layers"],&fixture);
        if let Some(capacity)=row["assetCapacity"].as_u64() {fill_assets(&mut document,capacity as usize);}
        let before=document.clone();let rendered=composite(&before);
        let command=MergeDown {layer_id:row["layerId"].as_str().unwrap().into()};
        let (parent,index,layers)=plan(&document,&command.layer_id).unwrap();
        assert_eq!(serde_json::to_value(parent).unwrap(),row["parentId"]);assert_eq!(index,row["index"].as_u64().unwrap() as usize);assert_eq!(layers.len(),2);
        let mut preparation=prepare(&command,&document).unwrap();while !preparation.advance(&document,1).unwrap() {}
        let mut job=preparation.into_job().unwrap();while !job.advance(1).unwrap().done {}let result=job.into_result().unwrap();
        assert_eq!(serde_json::to_value(&result.image.pixels).unwrap(),row.get("expectedPixels").unwrap_or(&fixture["expected"]["pixels"]).clone());assert_eq!(result.origin,[fixture["expected"]["origin"][0].as_f64().unwrap(),fixture["expected"]["origin"][1].as_f64().unwrap()]);
        let mut encoder=PngEncodeJob::new(result.image).unwrap();while !encoder.advance().unwrap().done {}
        let emit=publish(encoder.into_result().unwrap(),result.origin,&command,&document).unwrap();let mut inverses=Vec::new();
        for mutation in &emit.artifact_mutations {
            inverses.push(mutation.inverse(&document).expect("valid retained mutation inverse fixture"));let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,mutation).unwrap();assert!(messages.is_empty());retire(std::mem::replace(&mut document,next));
        }
        for key in row.get("retainedAssets").unwrap_or(&fixture["expected"]["retainedAssets"]).as_array().unwrap() {assert!(document.assets.contains_key(key.as_str().unwrap()));}
        for key in row.get("removedAssets").unwrap_or(&fixture["expected"]["removedAssets"]).as_array().unwrap() {assert!(!document.assets.contains_key(key.as_str().unwrap()));}
        assert_eq!(document.assets.len(),before.assets.len()-row.get("removedAssets").unwrap_or(&fixture["expected"]["removedAssets"]).as_array().unwrap().len()+1);let after=composite(&document);assert_eq!(after.origin,rendered.origin);assert_eq!(after.image,rendered.image);
        assert!(find_layer(&document.layers,"lower").is_none());assert!(matches!(find_layer(&document.layers,"upper"),Some(RasterLayerNode::Pixel {..})));assert!(find_layer(&document.layers,"outside").is_some());
        for inverse in inverses.into_iter().rev().flatten() {let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,&inverse).unwrap();assert!(messages.is_empty());retire(std::mem::replace(&mut document,next));inverse.retire_cold();}
        assert_eq!(document,before);for mutation in emit.artifact_mutations {mutation.retire_cold();}retire(document);retire(before);
    }
}
#[test]
fn unsupported_merge_and_cancel_leave_the_document_intact() {
    let fixture=fixture();
    for invalid in fixture["invalid"].as_array().unwrap() {
        let mut layers=fixture["cases"][0]["layers"].clone();
        if let Some(patch)=invalid["patch"].as_object() {
            let node=layers.as_array_mut().unwrap().iter_mut().find(|node|node["id"].as_str()==Some(invalid["target"].as_str().unwrap_or("upper"))).unwrap().as_object_mut().unwrap();
            node.extend(patch.clone());if node["kind"]=="adjustment" {for field in ["width","height","imageKey","mask"] {node.remove(field);}}
        }
        let document=document(&layers,&fixture);assert!(plan(&document,invalid["id"].as_str().unwrap()).is_err());retire(document);
    }
    let document=document(&fixture["cases"][0]["layers"],&fixture);let before=document.clone();let mut preparation=prepare(&MergeDown {layer_id:"upper".into()},&document).unwrap();preparation.cancel();assert!(preparation.advance(&document,1).is_err());assert_eq!(document,before);retire(document);retire(before);
}

fn fill_assets(document:&mut RasterSnapshot,capacity:usize) {
    let asset=crate::raster_asset(&document.assets,"blue").unwrap();
    for index in document.assets.len()..capacity {let key=format!("spare-{index}");document.assets.insert(key.clone(),crate::mint_raster_asset_child(&key,&asset)).unwrap();}
}
#[semio_framework_async_macros::async_test]
async fn layer_baking_at_asset_capacity_publishes_and_restores_retained_history() {
    use crate::editor::raster::{RasterCommand,unit_tests::context};
    use semio_framework_plugin::PluginApp;
    let fixture=fixture();
    for merge in [true,false] {
        let mut app=context::app().await;
        let mut source=document(&fixture["cases"][0]["layers"],&fixture);fill_assets(&mut source,fixture["cases"][0]["assetCapacity"].as_u64().unwrap() as usize);
        let envelope=store::create_document_envelope::<RasterSnapshot,RasterMutation>(crate::RASTER_DOCUMENT_SCHEMA,"bake-capacity",source,None);
        let files=store::print_document_pack(&envelope).await.unwrap();context::retire_raster_envelope(envelope);
        semio_framework_plugin::artifact_app_laws::load_document(&mut app, &files).await.unwrap();
        let before=app.snapshot().unwrap();assert_eq!(before.assets.len(),64);
        let command=if merge {RasterCommand::MergeDown(MergeDown {layer_id:"upper".into()})} else {RasterCommand::FlattenLayers(super::super::flatten_layers::FlattenLayers {name:"Image".into()})};
        context::dispatch(&mut app,command).await;
        let after=app.snapshot().unwrap();assert_eq!(after.layers.len(),if merge {2} else {1});assert_eq!(composite(&after).image,composite(&before).image);
        context::history(&mut app,"undo").await;let undone=app.snapshot().unwrap();assert_eq!(undone,before);retire(undone);
        context::history(&mut app,"redo").await;let redone=app.snapshot().unwrap();assert_eq!(redone,after);retire(redone);
        retire(before);retire(after);
    }
}

#[test]
fn merge_refuses_ancestor_and_descendant_protection() {
    let fixture=fixture();
    for row in fixture["protected"].as_array().unwrap() {
        let document=document(&row["layers"],&fixture);
        let Err(error)=plan(&document,row["layerId"].as_str().unwrap()) else {panic!("protected merge admitted")};
        assert!(error.message.contains("raster-layer-locked"));retire(document);
    }
}
