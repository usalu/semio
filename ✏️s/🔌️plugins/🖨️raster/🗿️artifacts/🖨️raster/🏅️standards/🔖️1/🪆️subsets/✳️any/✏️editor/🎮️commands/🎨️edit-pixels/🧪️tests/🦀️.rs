//! 🧪️ Pixel command validation and immutable document ownership.
use super::*;

#[test]
fn edit_pixels_keeps_the_source_asset_when_a_mask_references_it() {
    use crate::standards::v1::subsets::any::schema::{empty_raster_document,snapshot::retire_raster_snapshot};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧬️schema/🧫️fixtures/🎭️mask/🔣️.json")).unwrap();
    let fixture=&fixture["assetRetention"];let key=fixture["imageKey"].as_str().unwrap();
    let mut document=empty_raster_document();
    let image=RasterImage {width:2,height:2,pixels:[255,255,255,255].repeat(4)};
    let asset=RasterImageAsset {mime:"image/png".into(),data:semio_framework_pixels::encode_png(&image).unwrap()};
    document.assets.insert(key.into(),crate::mint_raster_asset_child(key,&asset)).unwrap();
    if let RasterLayerNode::Pixel {image_key,width,height,mask,..}=&mut document.layers[0] {
        *image_key=Some(key.into());*width=Some(2);*height=Some(2);
        *mask=Some(crate::RasterLayerMask {enabled:true,linked:true,invert:false,width:Some(2),height:Some(2),image_key:Some(key.into()),transform:crate::RasterTransform::default()});
    }
    let command=EditPixels {layer_id:layer_node_id(&document.layers[0]).to_owned(),expected_image_key:Some(key.into()),operation:fixture["operation"].to_string(),selection:None};
    let (mut job,layer,parent,index)=finish_preparation(&command,&document);
    while !job.advance(4).unwrap().done {}
    let mut encoder=PngEncodeJob::new(job.into_result().unwrap()).unwrap();
    while !encoder.advance().unwrap().done {}
    let emit=publish(encoder.into_result().unwrap(),layer,parent,index,&document).unwrap();
    retire_raster_snapshot(document);
    let removed:Vec<String>=emit.artifact_mutations.iter().filter_map(|op|if let RasterMutation::RemoveLayerAsset(payload)=op {Some(payload.asset_id.clone())}else{None}).collect();
    assert_eq!(serde_json::to_value(removed).unwrap(),fixture["expectedRemovedAssets"]);
}

#[test]
fn edit_pixels_rejects_stale_image_revision() {
    let mut document = crate::standards::v1::subsets::any::schema::empty_raster_snapshot();
    document.layers.push(crate::standards::v1::subsets::any::schema::create_pixel_layer("Test", 2, 2));
    let id = crate::standards::v1::subsets::any::schema::layer_node_id(&document.layers[0]).to_string();
    let command = EditPixels { layer_id: id, expected_image_key: Some("stale".into()), operation: "{\"kind\":\"invert\"}".into(), selection: None };
    assert!(prepare(&command, &document).is_err());
}

#[test]
fn edit_pixels_refuses_pixels_inside_a_hidden_group() {
    let mut document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();
    let pixel=crate::standards::v1::subsets::any::schema::create_pixel_layer("Pixels",2,2);
    let id=layer_node_id(&pixel).to_owned();
    document.layers.push(RasterLayerNode::Group {id:"hidden".into(),name:"Hidden".into(),visible:false, locked: false,opacity:1.0,blend_mode:"normal".into(),transform:crate::RasterTransform::default(),mask:None,children:vec![pixel]});
    let command=EditPixels {layer_id:id,expected_image_key:None,operation:"{\"kind\":\"invert\"}".into(),selection:None};
    assert!(prepare(&command,&document).is_err());
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
}

#[test]
fn edit_pixels_preserves_layer_identity_and_tree_address() {
    let mut document = crate::standards::v1::subsets::any::schema::empty_raster_snapshot();
    document.layers.push(crate::standards::v1::subsets::any::schema::create_pixel_layer("Test", 2, 2));
    let id = crate::standards::v1::subsets::any::schema::layer_node_id(&document.layers[0]).to_string();
    let command = EditPixels { layer_id: id.clone(), expected_image_key: None, operation: "{\"kind\":\"fill\",\"color\":[255,0,0,255]}".into(), selection: None };
    let (mut job, layer, parent, index) = finish_preparation(&command, &document);
    assert_eq!(index, 0);
    assert_eq!(parent, None);
    assert_eq!(crate::standards::v1::subsets::any::schema::layer_node_id(&layer), id);
    assert!(job.advance(4).unwrap().done);
    assert_eq!(job.result().unwrap().pixels, [255,0,0,255].repeat(4));
    assert!(document.assets.is_empty());
}

#[test]
fn edit_pixels_rejects_malformed_selections_and_parameters() {
    assert!(parse_operation("{\"kind\":\"posterize\",\"value\":2.5}").is_err());
    assert!(parse_operation("{\"kind\":\"resize\",\"width\":-1,\"height\":2,\"sampling\":\"nearest\"}").is_err());
    assert!(parse_selection(Some("[[0,5,255]]"), 4).is_err());
    assert!(parse_selection(Some("[[0,2,255],[1,2,255]]"), 4).is_err());
    assert_eq!(parse_selection(Some("[[1,2,255]]"), 4).unwrap(), Some(vec![0,255,255,0]));
}

#[test]
fn edit_pixels_publication_round_trips_through_history() {
    use protocol::Mutation;
    use crate::standards::v1::subsets::any::schema::{create_pixel_layer, empty_raster_snapshot, snapshot::retire_raster_snapshot};
    let mut document=empty_raster_snapshot();
    document.layers.push(create_pixel_layer("Paint",2,2));
    let id=layer_node_id(&document.layers[0]).to_owned();
    let before=document.clone();
    let command=EditPixels {layer_id:id.clone(),expected_image_key:None,operation:"{\"kind\":\"fill\",\"color\":[255,0,0,255]}".into(),selection:None};
    let (mut job,layer,parent,index)=finish_preparation(&command,&document);
    job.advance(4).unwrap();
    let mut encoder=PngEncodeJob::new(job.into_result().unwrap()).unwrap();
    while !encoder.advance().unwrap().done {}
    let emit=publish(encoder.into_result().unwrap(),layer,parent,index,&document).unwrap();
    let mut inverse=Vec::new();
    for mutation in &emit.artifact_mutations {
        inverse.push(mutation.inverse(&document));
        let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,mutation).unwrap();
        assert!(messages.is_empty());
        retire_raster_snapshot(std::mem::replace(&mut document,next));
    }
    let RasterLayerNode::Pixel {image_key:Some(key),..}=&document.layers[0] else {panic!("published pixel image missing")};
    let asset=crate::raster_asset(&document.assets,key).unwrap();
    assert_eq!(semio_framework_pixels::decode_png(&asset.data).unwrap().pixels,[255,0,0,255].repeat(4));
    assert!(prepare(&command,&document).is_err());
    for mutations in inverse.into_iter().rev() {
        for mutation in mutations {
            let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,&mutation).unwrap();
            assert!(messages.is_empty());
            retire_raster_snapshot(std::mem::replace(&mut document,next));
            mutation.retire_cold();
        }
    }
    assert_eq!(document,before);
    for mutation in emit.artifact_mutations {mutation.retire_cold();}
    retire_raster_snapshot(document);
    retire_raster_snapshot(before);
}

fn finish_preparation(command:&EditPixels,document:&RasterSnapshot)->(PixelEditJob,RasterLayerNode,Option<String>,usize) {
    let mut candidate=prepare(command,document).unwrap();
    while !candidate.advance(document,32768).unwrap() {}
    candidate.into_job().unwrap()
}

#[test]
fn pixel_source_preparation_is_bounded_and_cancellable() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for source in [false,true] {
        let mut document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();
        let width=fixture["width"].as_u64().unwrap() as u32;let height=fixture["height"].as_u64().unwrap() as u32;
        let mut layer=crate::standards::v1::subsets::any::schema::create_pixel_layer("Paint",width,height);
        let id=layer_node_id(&layer).to_owned();
        let key=source.then(||"source".to_owned());
        if source {
            let pixels=fixture["sourcePixel"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect::<Vec<_>>().repeat((width*height) as usize);
            let asset=RasterImageAsset {mime:"image/png".into(),data:semio_framework_pixels::encode_png(&RasterImage {width,height,pixels}).unwrap()};
            document.assets.insert("source".into(),crate::mint_raster_asset_child("source",&asset)).unwrap();
            let RasterLayerNode::Pixel {image_key,..}=&mut layer else {panic!("pixel")};*image_key=key.clone();
        }
        document.layers.push(layer);
        let command=EditPixels {layer_id:id,expected_image_key:key,operation:fixture["operation"].to_string(),selection:Some(fixture["selection"].to_string())};
        let mut candidate=prepare(&command,&document).unwrap();
        assert!(candidate.image.pixels.is_empty());assert!(candidate.selection.as_ref().unwrap().is_empty());
        assert!(!candidate.advance(&document,0).unwrap());assert!(candidate.image.pixels.is_empty());
        assert!(!candidate.advance(&document,fixture["grant"].as_u64().unwrap() as usize).unwrap());
        let prepared=fixture["expectedPreparedPixels"].as_u64().unwrap() as usize;
        assert_eq!(candidate.image.pixels.len(),prepared*4);assert_eq!(candidate.selection.as_ref().unwrap().len(),prepared);
        assert_eq!(candidate.selection.as_ref().unwrap()[32767],128);
        let mut work=PixelEditWork {preparing:Some(candidate),..Default::default()};
        work.begin_close();assert!(!work.terminal_is_empty());
        assert!(matches!(work.close_step(1,262144),semio_framework_job::InteractiveJobCloseStep::Complete));assert!(work.terminal_is_empty());
        let (mut job,..)=finish_preparation(&command,&document);
        while !job.advance(4096).unwrap().done {}
        let pixels=&job.result().unwrap().pixels;
        let expected=fixture[if source {"expectedSelectedPixel"} else {"expectedBlankSelectedPixel"}].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as u8).collect::<Vec<_>>();
        assert_eq!(&pixels[32767*4..32769*4],expected.repeat(2));
        assert_eq!(&pixels[..4],if source {&[10,20,30,255]} else {&[0,0,0,0]});
        crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
    }
}

#[test]
fn protected_pixels_refuse_before_preparation() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧫️fixtures/🔒️protection/🔣️.json")).unwrap();
    let mut document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();document.layers=dsl::json::from_json_str(&fixture["layers"].to_string()).unwrap();
    for id in ["locked-pixel","inherited-pixel"] {
        let command=EditPixels {layer_id:id.into(),expected_image_key:None,operation:r#"{"kind":"invert"}"#.into(),selection:None};
        assert!(prepare(&command,&document).is_err());
    }
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
}

#[test]
fn pixel_replacement_at_capacity_preserves_shared_assets_and_exact_inverse() {
    use protocol::Mutation;
    use crate::standards::v1::subsets::any::schema::{empty_raster_document,snapshot::retire_raster_snapshot};
    let vectors:serde_json::Value=serde_json::from_str(include_str!("../../../🖼️assets/🔄️replacement/🧫️fixtures/🔣️.json")).unwrap();
    let capacity=vectors["cases"][0]["input"]["capacity"].as_u64().unwrap() as usize;
    assert_eq!(capacity,crate::RASTER_OWNED_MAP_CAPACITY);
    for shared in [false,true] {
        let mut document=empty_raster_document();
        let asset=RasterImageAsset {mime:"image/png".into(),data:semio_framework_pixels::encode_png(&RasterImage {width:2,height:2,pixels:[255,255,255,255].repeat(4)}).unwrap()};
        for index in 0..capacity {let key=if index==0 {"previous".to_owned()}else{format!("unused-{index}")};document.assets.insert(key.clone(),crate::mint_raster_asset_child(&key,&asset)).unwrap();}
        if let RasterLayerNode::Pixel {image_key,width,height,mask,..}=&mut document.layers[0] {
            *image_key=Some("previous".into());*width=Some(2);*height=Some(2);
            if shared {*mask=Some(crate::RasterLayerMask {enabled:true,linked:true,invert:false,width:Some(2),height:Some(2),image_key:Some("previous".into()),transform:Default::default()});}
        }
        let before=document.clone();
        let command=EditPixels {layer_id:layer_node_id(&document.layers[0]).into(),expected_image_key:Some("previous".into()),operation:r#"{"kind":"invert"}"#.into(),selection:None};
        let (mut job,layer,parent,index)=finish_preparation(&command,&document);
        while !job.advance(1).unwrap().done {}
        let mut encoder=PngEncodeJob::new(job.into_result().unwrap()).unwrap();while !encoder.advance().unwrap().done {}
        let publication=publish(encoder.into_result().unwrap(),layer,parent,index,&document);
        if shared {assert!(publication.is_err());assert_eq!(document,before);retire_raster_snapshot(document);retire_raster_snapshot(before);continue;}
        let emit=publication.unwrap();let mut inverses=Vec::new();
        for mutation in &emit.artifact_mutations {
            inverses.push(mutation.inverse(&document));
            let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,mutation).unwrap();assert!(messages.is_empty());retire_raster_snapshot(std::mem::replace(&mut document,next));
            assert!(document.assets.len()<=capacity);
        }
        assert_eq!(document.assets.len(),capacity);assert!(!document.assets.contains_key("previous"));
        for inverse in inverses.into_iter().rev().flatten() {
            let (next,messages)=semio_framework_os_kernel::apply_mutation(&document,&inverse).unwrap();assert!(messages.is_empty());retire_raster_snapshot(std::mem::replace(&mut document,next));inverse.retire_cold();assert!(document.assets.len()<=capacity);
        }
        assert_eq!(document,before);
        for mutation in emit.artifact_mutations {mutation.retire_cold();}
        retire_raster_snapshot(document);retire_raster_snapshot(before);
    }
}
