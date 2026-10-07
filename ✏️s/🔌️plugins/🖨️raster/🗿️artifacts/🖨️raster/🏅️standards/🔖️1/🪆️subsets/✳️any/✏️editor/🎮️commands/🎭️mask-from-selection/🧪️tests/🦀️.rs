//! 🎭️ Selection mask pixels and undo history share a neutral coverage fixture.
use super::*;
use protocol::Mutation;

#[test]
fn selection_mask_preserves_soft_coverage_and_round_trips_history() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut document = crate::standards::v1::subsets::any::io::text::snapshot::empty_raster_snapshot();
    document.layers.push(crate::standards::v1::subsets::any::schema::create_pixel_layer("Paint",3,2));
    let id = crate::standards::v1::subsets::any::schema::layer_node_id(&document.layers[0]).to_owned();
    let command = MaskFromSelection { layer_id: id, expected_image_key: None, selection: fixture["selection"].to_string() };
    for invalid in fixture["invalid"].as_array().unwrap() {
        assert!(prepare(&MaskFromSelection { selection: invalid.to_string(), ..command.clone() }, &document).is_err());
    }
    assert!(prepare(&MaskFromSelection { expected_image_key: Some("stale".into()), ..command.clone() }, &document).is_err());
    let before = document.clone();
    let mut candidate = prepare(&command, &document).unwrap();
    assert!(!candidate.advance(1));
    while !candidate.advance(1) {}
    assert_eq!(serde_json::to_value(&candidate.image.pixels).unwrap(), fixture["expectedRgba"]);
    let mut encoder = PngEncodeJob::new(candidate.image.clone()).unwrap();
    while !encoder.advance().unwrap().done {}
    let emit = publish(encoder.into_result().unwrap(), &candidate, &document).unwrap();
    let mut inverses = Vec::new();
    for mutation in &emit.artifact_mutations {
        inverses.push(mutation.inverse(&document).expect("valid retained mutation inverse fixture"));
        let (next,messages) = semio_framework_os_kernel::apply_mutation(&document,mutation).unwrap();
        assert!(messages.is_empty());
        retire(std::mem::replace(&mut document,next));
    }
    let RasterLayerNode::Pixel { mask:Some(mask), image_key, .. } = &document.layers[0] else { panic!("mask must be attached"); };
    assert!(image_key.is_none());
    let asset = crate::raster_asset(&document.assets,mask.image_key.as_ref().unwrap()).unwrap();
    assert_eq!(serde_json::to_value(semio_framework_pixels::decode_png(&asset.data).unwrap().pixels).unwrap(),fixture["expectedRgba"]);
    assert!(publish(semio_framework_pixels::png_encoding::EncodedPngImage { width:3,height:2,data:vec![],content_hash:0 }, &candidate, &document).is_err());
    for inverse in inverses.into_iter().rev().flatten() {
        let (next,messages) = semio_framework_os_kernel::apply_mutation(&document,&inverse).unwrap();
        assert!(messages.is_empty());retire(std::mem::replace(&mut document,next));inverse.retire_cold();
    }
    assert_eq!(document,before);
    for mutation in emit.artifact_mutations { mutation.retire_cold(); }
    retire(document);retire(before);
}

fn retire(document: RasterSnapshot) { crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document); }

#[test]
fn selection_mask_cancellation_releases_private_work_without_publication() {
    let mut document=crate::standards::v1::subsets::any::io::text::snapshot::empty_raster_snapshot();
    document.layers.push(crate::standards::v1::subsets::any::schema::create_pixel_layer("Paint",256,256));
    let id=crate::standards::v1::subsets::any::schema::layer_node_id(&document.layers[0]).to_owned();
    let command=MaskFromSelection {layer_id:id,expected_image_key:None,selection:"[[0,65536,255]]".into()};
    let mut work=MaskFromSelectionWork {candidate:Some(prepare(&command,&document).unwrap()),..Default::default()};
    assert!(!work.candidate.as_mut().unwrap().advance(32768));
    work.begin_close();
    assert!(!work.terminal_is_empty());
    assert!(matches!(work.close_step(0,0),semio_framework_job::InteractiveJobCloseStep::Blocked));
    assert!(matches!(work.close_step(1,262144),semio_framework_job::InteractiveJobCloseStep::Complete));
    assert!(work.terminal_is_empty());
    assert!(document.assets.is_empty());
    assert!(matches!(&document.layers[0],RasterLayerNode::Pixel {mask:None,image_key:None,..}));
    retire(document);
}

#[test]
fn protected_pixels_refuse_before_preparation() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧫️fixtures/🔒️protection/🔣️.json")).unwrap();
    let mut document=crate::standards::v1::subsets::any::io::text::snapshot::empty_raster_snapshot();document.layers=semio_framework_pack_json::from_json_str(&fixture["layers"].to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    for id in ["locked-pixel","inherited-pixel"] {
        let command=MaskFromSelection {layer_id:id.into(),expected_image_key:None,selection:"[[0,1,255]]".into()};
        assert!(prepare(&command,&document).is_err());
    }
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
}
