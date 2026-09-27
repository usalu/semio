//! 🧪️ Shared selection vectors projected into native semantic controls.
use super::*;

#[test]
fn inspector_projects_selected_properties_and_foreground_in_both_languages() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🎛️selection/🔣️.json")).unwrap();
    let mut document: RasterDocument = dsl::json::from_json_str(r#"{"schema":"raster.document","id":"inspection","title":"Inspection","layers":[{"kind":"pixel","id":"paint.foreground","name":"123","mask":null,"width":32,"height":32,"imageKey":null},{"kind":"pixel","id":"paint.background","name":"Background","mask":null,"width":64,"height":64,"imageKey":null}]}"#).unwrap();
    for labels in [&RasterPlayLabels::NATIVE_EN, &RasterPlayLabels::NATIVE_DE] {
        for case in fixture["cases"].as_array().unwrap() {
            let ids: Vec<String> = serde_json::from_value(case["selection"].clone()).unwrap();
            if let crate::RasterLayerNode::Pixel { mask, .. } = &mut document.layers[0] {
                *mask = case.get("mask").map(|value| dsl::json::from_json_str(&value.to_string()).unwrap());
            }
            let node = render(&document, &RasterConfig::default(), &ids, labels).unwrap();
            let text = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(semio_framework_plugin::built_to_component_tree(node)).unwrap();
            let projection: serde_json::Value = serde_json::from_str(&text).unwrap();
            let mut pending = vec![&projection];
            while let Some(node) = pending.pop() {
                if node["component"]["commit"] == "blur" {
                    assert_eq!(node["bindings"][0]["trigger"], fixture["blurTrigger"], "{}", node["key"]);
                }
                pending.extend(node["children"].as_array().unwrap());
            }
            assert!(text.contains("setBrushColor"));
            assert!(text.contains(labels.foreground.as_str()));
            assert_eq!(text.contains("patchLayers"), !ids.is_empty());
            for field in case["fields"].as_array().unwrap() {
                assert!(text.contains(&format!("raster-inspector.{}.input", field.as_str().unwrap())), "{text}");
            }
            if ids.len() == 2 { assert!(text.contains(labels.mixed.as_str())); }
            if !ids.is_empty(){for mode in fixture["blendModes"].as_array().unwrap(){assert!(text.contains(mode.as_str().unwrap()),"missing blend option {mode}: {text}");}}
        }
    }
    crate::standards::v1::subsets::any::schema::mutations::binary::unit_tests::retirement::retire_raster_snapshot(document);
}

#[semio_framework_async_macros::async_test]
async fn inspector_mask_dimensions_resolve_the_attached_image() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎛️selection/🔣️.json")).unwrap();
    let width=fixture["maskImageExtent"]["width"].as_u64().unwrap() as u32;
    let height=fixture["maskImageExtent"]["height"].as_u64().unwrap() as u32;
    let mut encoder=semio_framework_pixels::png_encoding::PngEncodeJob::new(semio_framework_pixels::RasterImage::new(width,height)).unwrap();
    while !encoder.advance().unwrap().done {}
    let bytes=encoder.into_result().unwrap().data;
    let mut document=crate::standards::v1::subsets::any::schema::empty_raster_snapshot();
    document.assets.insert("mask".into(),crate::mint_raster_asset_child("mask",&crate::RasterImageAsset {mime:"image/png".into(),data:bytes})).unwrap();
    let mut layer=crate::standards::v1::subsets::any::schema::create_layer_of_kind("pixel");
    if let RasterLayerNode::Pixel {mask,..}=&mut layer {*mask=Some(crate::RasterLayerMask {enabled:true,linked:true,invert:false,width:None,height:None,image_key:Some("mask".into()),transform:crate::RasterTransform::default()});}
    document.layers.push(layer);
    let resolved=[value(&document.layers[0],"maskWidth",&document),value(&document.layers[0],"maskHeight",&document)];
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
    assert_eq!(resolved,[width.to_string(),height.to_string()]);
}
