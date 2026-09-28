//! 🥞️ Flattening preserves rendered pixels, world placement and complete inverse history.
use super::*;
use protocol::Mutation;
#[test]
fn flatten_preserves_the_composite_and_restores_every_original_layer() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for capacity in fixture["assetCapacities"].as_array().unwrap() {
    let mut document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();
    document.layers=dsl::json::from_json_str(&fixture["layers"].to_string()).unwrap();
    for (key,value) in fixture["images"].as_object().unwrap() {
        let image=semio_framework_pixels::RasterImage {width:value["width"].as_u64().unwrap() as u32,height:value["height"].as_u64().unwrap() as u32,pixels:value["pixels"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect()};
        let asset=RasterImageAsset {mime:"image/png".into(),data:semio_framework_pixels::encode_png(&image).unwrap()};
        document.assets.insert(key.clone(),crate::mint_raster_asset_child(key,&asset)).unwrap();
    }
    let asset=crate::raster_asset(&document.assets,"blue").unwrap();
    for index in document.assets.len()..capacity.as_u64().unwrap() as usize {let key=format!("spare-{index}");document.assets.insert(key.clone(),crate::mint_raster_asset_child(&key,&asset)).unwrap();}
    let before=document.clone();
    let mut preparation=prepare(&FlattenLayers {name:"Flattened Image".into()},&document).unwrap();
    while !preparation.advance(&document,1).unwrap() {}
    let mut job=preparation.into_job().unwrap();while !job.advance(1).unwrap().done {}
    let composite=job.into_result().unwrap();
    assert_eq!(serde_json::to_value(&composite.image.pixels).unwrap(),fixture["expected"]["pixels"]);
    let origin=composite.origin;
    let mut encoder=PngEncodeJob::new(composite.image).unwrap();while !encoder.advance().unwrap().done {}
    let emit=publish(encoder.into_result().unwrap(),origin,"Flattened Image",&document).unwrap();
    let mut inverses=Vec::new();
    for mutation in &emit.artifact_mutations {
        inverses.push(mutation.inverse(&document));
        let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,mutation).unwrap();assert!(messages.is_empty());
        retire(std::mem::replace(&mut document,next));
    }
    assert_eq!(document.layers.len(),1);assert_eq!(document.assets.len(),before.assets.len()-fixture["images"].as_object().unwrap().len()+1);
    for (key,asset) in &before.assets {if key.starts_with("spare-") {assert_eq!(document.assets.get(key),Some(asset));}}
    let RasterLayerNode::Pixel {image_key:Some(key),transform,name,..}=&document.layers[0] else {panic!("flattened pixels")};
    assert_eq!(name,"Flattened Image");assert_eq!((transform.x,transform.y),(-3.0,4.5));
    let asset=crate::raster_asset(&document.assets,key).unwrap();
    assert_eq!(serde_json::to_value(semio_framework_pixels::decode_png(&asset.data).unwrap().pixels).unwrap(),fixture["expected"]["pixels"]);
    for inverse in inverses.into_iter().rev().flatten() {
        let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,&inverse).unwrap();assert!(messages.is_empty());
        retire(std::mem::replace(&mut document,next));inverse.retire_cold();
    }
    assert_eq!(document,before);
    for mutation in emit.artifact_mutations {mutation.retire_cold();}
    retire(document);retire(before);
    }
}
#[test]
fn flatten_preparation_is_cancellable_and_validates_the_name() {
    let document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();
    for name in ["".to_owned(),"   ".into(),"x".repeat(121)] {assert!(prepare(&FlattenLayers {name},&document).is_err());}
    let mut work=LayerBakeWork::<false> {preparing:Some(prepare(&FlattenLayers {name:"Image".into()},&document).unwrap()),..Default::default()};
    work.begin_close();assert!(!work.terminal_is_empty());
    assert!(matches!(work.close_step(1,262144),semio_framework_job::InteractiveJobCloseStep::Complete));assert!(work.terminal_is_empty());
    assert!(document.layers.is_empty());assert!(document.assets.is_empty());retire(document);
}
fn retire(document:RasterSnapshot) {crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);}

#[test]
fn flatten_refuses_a_protected_descendant_without_changing_the_document() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧬️schema/🧫️fixtures/🔒️protection/🔣️.json")).unwrap();
    let mut document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();document.layers=dsl::json::from_json_str(&fixture["layers"].to_string()).unwrap();
    let before=dsl::json::to_json_string(&document);
    assert!(prepare(&FlattenLayers {name:"Flattened".into()},&document).is_err());assert_eq!(dsl::json::to_json_string(&document),before);
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
}
