//! 🧪️ Pixel command validation and immutable document ownership.
use super::*;

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
    document.layers.push(RasterLayerNode::Group {id:"hidden".into(),name:"Hidden".into(),visible:false,opacity:1.0,blend_mode:"normal".into(),transform:crate::RasterTransform::default(),mask:None,children:vec![pixel]});
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
    let (mut job, layer, parent, index) = prepare(&command, &document).unwrap();
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
    let (mut job,layer,parent,index)=prepare(&command,&document).unwrap();
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
