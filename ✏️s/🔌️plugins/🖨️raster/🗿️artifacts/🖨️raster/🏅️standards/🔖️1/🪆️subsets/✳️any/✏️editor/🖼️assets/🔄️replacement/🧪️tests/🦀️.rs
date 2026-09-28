//! 🧪️ Neutral asset-capacity vectors match the native publication plan.
use super::*;
#[test]
fn asset_replacement_uses_the_neutral_capacity_plan(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let input=&row["input"];
        let plan=replacement_steps(input["count"].as_u64().unwrap() as usize,input["capacity"].as_u64().unwrap() as usize,input["removable"].as_bool().unwrap(),input["nextExists"].as_bool().unwrap());
        if row["steps"].is_null(){assert!(plan.is_err());continue;}
        let names:Vec<_>=plan.unwrap().iter().map(|step|match step {Step::Detach=>"detach",Step::Remove=>"remove",Step::Add=>"add",Step::Replace=>"replace"}).collect();
        assert_eq!(serde_json::to_value(names).unwrap(),row["steps"]);
    }
}

#[semio_framework_async_macros::async_test]
async fn pixel_and_mask_replacement_at_capacity_publish_one_reversible_edit(){
    use crate::{RasterLayerNode,RasterSnapshot,RasterMutation,RasterImageAsset,RasterLayerMask};
    use crate::editor::raster::{RasterCommand,commands::{edit_pixels::EditPixels,edit_mask::EditMask},unit_tests::context};
    use crate::standards::v1::subsets::any::schema::{empty_raster_document,layer_node_id,snapshot::retire_raster_snapshot};
    use semio_framework_plugin::PluginApp;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let capacity=fixture["cases"][0]["input"]["capacity"].as_u64().unwrap() as usize;
    for mask_target in [false,true]{
        let mut source=empty_raster_document();
        let asset=RasterImageAsset {mime:"image/png".into(),data:semio_framework_pixels::encode_png(&semio_framework_pixels::RasterImage {width:2,height:2,pixels:[255,255,255,255].repeat(4)}).unwrap()};
        for index in 0..capacity {let key=if index==0 {"previous".to_owned()}else{format!("unused-{index}")};source.assets.insert(key.clone(),crate::mint_raster_asset_child(&key,&asset)).unwrap();}
        let id=layer_node_id(&source.layers[0]).to_owned();
        let mask=RasterLayerMask {enabled:true,linked:true,invert:false,width:Some(2),height:Some(2),image_key:Some("previous".into()),transform:Default::default()};
        if let RasterLayerNode::Pixel {image_key,width,height,mask:slot,..}=&mut source.layers[0] {
            *width=Some(2);*height=Some(2);
            if mask_target {*slot=Some(mask.clone());*image_key=None;}else{*image_key=Some("previous".into());}
        }
        let envelope=store::create_document_envelope::<RasterSnapshot,RasterMutation>(crate::RASTER_DOCUMENT_SCHEMA,"replacement-capacity",source,None);
        let files=store::print_document_pack(&envelope).await.unwrap();context::retire_raster_envelope(envelope);
        let mut app=context::app().await;app.load_document_pack(&files).await.unwrap();let before=app.snapshot().unwrap();
        let command=if mask_target {RasterCommand::EditMask(EditMask {layer_id:id,expected_mask:dsl::json::to_json_string(&mask),operation:r#"{"kind":"alphaFill","alpha":0,"opacity":1}"#.into(),selection:None})}else{RasterCommand::EditPixels(EditPixels {layer_id:id,expected_image_key:Some("previous".into()),operation:r#"{"kind":"invert"}"#.into(),selection:None})};
        context::dispatch(&mut app,command).await;
        let after=app.snapshot().unwrap();assert_eq!(after.assets.len(),capacity);assert!(!after.assets.contains_key("previous"));assert_ne!(after,before);
        context::history(&mut app,"undo").await;let undone=app.snapshot().unwrap();assert_eq!(undone,before);retire_raster_snapshot(undone);
        context::history(&mut app,"redo").await;let redone=app.snapshot().unwrap();assert_eq!(redone,after);retire_raster_snapshot(redone);
        retire_raster_snapshot(before);retire_raster_snapshot(after);
    }
}
