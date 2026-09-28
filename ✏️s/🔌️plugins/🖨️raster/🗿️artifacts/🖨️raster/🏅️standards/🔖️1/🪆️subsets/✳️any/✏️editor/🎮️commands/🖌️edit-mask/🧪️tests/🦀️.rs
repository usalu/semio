//! 🖌️ Shared coverage vectors verify mask-only publication and exact undo.
use super::*;
use protocol::Mutation;
fn retire(document:RasterSnapshot) {crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);}
#[test]
fn mask_paint_preserves_source_and_settings_and_restores_history() {
    let fixtures:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for fixture in std::iter::once(&fixtures).chain(fixtures["coverageCases"].as_array().unwrap().iter()) {
    for (kind,shared,full) in [("pixel",false,false),("pixel",true,false),("group",false,false),("pixel",false,true),("pixel",true,true),("group",false,true)] {
        let mut document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();
        let mut layer=crate::standards::v1::subsets::any::schema::create_layer_of_kind(kind);
        let id=layer_node_id(&layer).to_owned();
        let image=RasterImage {width:fixture["width"].as_u64().unwrap() as u32,height:fixture["height"].as_u64().unwrap() as u32,pixels:fixture["beforeRgba"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect()};
        let asset=RasterImageAsset {mime:"image/png".into(),data:semio_framework_pixels::encode_png(&image).unwrap()};
        document.assets.insert("coverage".into(),crate::mint_raster_asset_child("coverage",&asset)).unwrap();
        let (RasterLayerNode::Pixel {mask,..}|RasterLayerNode::Group {mask,..})=&mut layer else {panic!("mask owner")};
        *mask=Some(RasterLayerMask {enabled:false,linked:false,invert:true,width:Some(6),height:Some(2),image_key:Some("coverage".into()),transform:crate::RasterTransform {x:2.0,y:3.0,a:0.8660254037844386,b:0.5,c:-0.5,d:0.8660254037844386}});
        if shared {if let RasterLayerNode::Pixel {image_key,..}=&mut layer {*image_key=Some("coverage".into());}}
        document.layers.push(layer);
        if full {for index in 1..crate::RASTER_OWNED_MAP_CAPACITY {let key=format!("unused-{index}");document.assets.insert(key.clone(),crate::mint_raster_asset_child(&key,&asset)).unwrap();}}

        let command=EditMask {layer_id:id,expected_mask:dsl::json::to_json_string(layer_mask(&document.layers[0]).unwrap()),operation:fixture["operation"].to_string(),selection:Some(fixture["selection"].to_string())};
        for invalid in fixture["invalidOperations"].as_array().unwrap() {assert!(prepare(&EditMask {operation:invalid.to_string(),..command.clone()},&document).is_err());}
        assert!(prepare(&EditMask {expected_mask:"{}".into(),..command.clone()},&document).is_err());
        let mut stale=layer_mask(&document.layers[0]).unwrap().clone();stale.transform.x+=1.0;
        assert!(prepare(&EditMask {expected_mask:dsl::json::to_json_string(&stale),..command.clone()},&document).is_err());
        let before=document.clone();
        let mut candidate=prepare(&command,&document).unwrap();
        assert!(!candidate.advance(&document,1).unwrap());
        while !candidate.advance(&document,1).unwrap() {}
        let mut job=candidate.take_job().unwrap();
        while !job.advance(1).unwrap().done {}
        assert_eq!(serde_json::to_value(&job.result().unwrap().pixels).unwrap(),fixture["expectedRgba"]);
        let mut encoder=PngEncodeJob::new(job.into_result().unwrap()).unwrap();
        while !encoder.advance().unwrap().done {}
        let publication=publish(encoder.into_result().unwrap(),&candidate,&document);
        if full&&shared {assert!(publication.is_err());assert_eq!(document,before);retire(document);retire(before);continue;}
        let emit=publication.unwrap();
        let mut inverses=Vec::new();
        for mutation in &emit.artifact_mutations {
            inverses.push(mutation.inverse(&document));
            let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,mutation).unwrap();assert!(messages.is_empty());retire(std::mem::replace(&mut document,next));
        }
        let mask=layer_mask(&document.layers[0]).unwrap();
        let prior=layer_mask(&before.layers[0]).unwrap();
        assert_eq!((mask.enabled,mask.linked,mask.invert,mask.width,mask.height,&mask.transform),(prior.enabled,prior.linked,prior.invert,prior.width,prior.height,&prior.transform));
        if let (RasterLayerNode::Pixel {image_key:after,..},RasterLayerNode::Pixel {image_key:before,..})=(&document.layers[0],&before.layers[0]) {assert_eq!(after,before);}
        assert_eq!(document.assets.contains_key("coverage"),shared);
        assert!(publish(EncodedPngImage {width:3,height:1,data:vec![],content_hash:0},&candidate,&document).is_err());
        for inverse in inverses.into_iter().rev().flatten() {
            let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,&inverse).unwrap();assert!(messages.is_empty());retire(std::mem::replace(&mut document,next));inverse.retire_cold();
        }
        assert_eq!(document,before);
        for mutation in emit.artifact_mutations {mutation.retire_cold();}
        retire(document);retire(before);
    }
    }
}

#[test]
fn blank_mask_preparation_is_bounded_and_cancels_without_publication() {
    let mut document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();
    let mut layer=crate::standards::v1::subsets::any::schema::create_pixel_layer("Paint",256,256);
    let id=layer_node_id(&layer).to_owned();
    let RasterLayerNode::Pixel {mask,..}=&mut layer else {panic!("pixel")};
    *mask=Some(RasterLayerMask {enabled:true,linked:true,invert:false,width:None,height:None,image_key:None,transform:Default::default()});
    document.layers.push(layer);
    let command=EditMask {layer_id:id,expected_mask:dsl::json::to_json_string(layer_mask(&document.layers[0]).unwrap()),operation:r#"{"kind":"alphaStroke","points":[[0.5,0.5]],"size":1,"opacity":1,"hardness":1,"alpha":0}"#.into(),selection:Some("[[0,65536,128]]".into())};
    let mut candidate=prepare(&command,&document).unwrap();
    assert!(candidate.image.pixels.is_empty());
    assert!(!candidate.advance(&document,32768).unwrap());
    assert_eq!(candidate.image.pixels.len(),32768*4);
    assert!(candidate.image.pixels.iter().all(|value|*value==255));
    assert_eq!(candidate.selection.as_ref().unwrap(),&vec![128;32768]);
    let mut work=EditMaskWork {candidate:Some(candidate),..Default::default()};
    work.begin_close();assert!(!work.terminal_is_empty());
    assert!(matches!(work.close_step(0,0),semio_framework_job::InteractiveJobCloseStep::Blocked));
    assert!(matches!(work.close_step(1,262144),semio_framework_job::InteractiveJobCloseStep::Complete));
    assert!(work.terminal_is_empty());assert!(document.assets.is_empty());
    assert!(layer_mask(&document.layers[0]).unwrap().image_key.is_none());retire(document);
}

#[test]
fn mask_preparation_refuses_own_and_inherited_protection() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧬️schema/🧫️fixtures/🔒️protection/🔣️.json")).unwrap();
    let mut document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();document.layers=dsl::json::from_json_str(&fixture["layers"].to_string()).unwrap();
    for id in ["locked-pixel","inherited-pixel","locked-group"] {
        let command=EditMask {layer_id:id.into(),expected_mask:"null".into(),operation:r#"{"kind":"alphaFill","alpha":0,"opacity":1}"#.into(),selection:None};
        let Err(error)=prepare(&command,&document) else {panic!("protected mask admitted")};assert!(error.message.contains("raster-layer-locked"));
    }
    retire(document);
}
