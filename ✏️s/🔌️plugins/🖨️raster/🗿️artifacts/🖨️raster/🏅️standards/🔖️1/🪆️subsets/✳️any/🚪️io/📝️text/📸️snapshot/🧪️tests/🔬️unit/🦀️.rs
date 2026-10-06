use crate::standards::v1::subsets::any::io::text::snapshot::*;
use crate::RasterOwnedMap;
use crate::{RasterImageAsset, RasterLayerMask, RasterLayerNode, RasterTransform, RASTER_DOCUMENT_SCHEMA};

/// 📄️ Handcrafted document exercising every layer kind/field, shared with the `pack`/`op`
/// taxonomy nodes' own copies (each node keeps its own private copy, per §7 test isolation).
fn representative_raster_document() -> RasterSnapshot {
    let mut assets = RasterOwnedMap::new();
    assets.insert("asset-1".into(), crate::image_asset_child_handle("asset-1", &RasterImageAsset { mime: "image/png".into(), data: b"abc".to_vec() })).expect("bounded fixture operation succeeds");
    let mut params = RasterOwnedMap::new();
    params.insert("brightness".into(), semio_framework_value::DslValue::float(0.06)).expect("bounded fixture operation succeeds");
    params.insert("label".into(), semio_framework_value::DslValue::String("Warm \"Curve\"".to_string())).expect("bounded fixture operation succeeds");
    params.insert("enabled".into(), semio_framework_value::DslValue::Bool(true)).expect("bounded fixture operation succeeds");
    params.insert("fallback".into(), semio_framework_value::DslValue::Null).expect("bounded fixture operation succeeds");
    params
        .insert(
            "curves".into(),
            semio_framework_value::DslValue::Array(vec![
                semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::float(0.0), semio_framework_value::DslValue::float(0.0)]),
                semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::float(0.25), semio_framework_value::DslValue::float(0.2)]),
                semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::float(1.0), semio_framework_value::DslValue::float(1.0)]),
            ]),
        )
        .expect("bounded fixture operation succeeds");
    params.insert("nested".into(), semio_framework_value::DslValue::Object(vec![("inner".to_string(), semio_framework_value::DslValue::float(1.5))])).expect("bounded fixture operation succeeds");
    RasterSnapshot {
        schema: RASTER_DOCUMENT_SCHEMA.into(),
        id: "doc-1".into(),
        title: Some("Representative \"Doc\"".into()),
        assets,
        layers: vec![
            RasterLayerNode::Pixel {
                id: "pixel-1".into(),
                name: "Pixel One".into(),
                visible: true, locked: false,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: Some(RasterLayerMask { enabled: true, linked: false, invert: true, width: Some(64), height: None, image_key:Some("asset-1".into()), transform:RasterTransform {x:-4.0,y:2.0,..RasterTransform::default()} }),
                width: Some(256),
                height: Some(256),
                image_key: Some("asset-1".into()),
            },
            RasterLayerNode::Group {
                id: "group-1".into(),
                name: "Group / Nested".into(),
                visible: false, locked: false,
                opacity: 0.5,
                blend_mode: "screen".into(),
                transform: RasterTransform {x:1.0,y:-2.0,a:1.5,b:0.25,c:-0.5,d:0.75},
                mask: None,
                children: vec![
                    RasterLayerNode::Pixel {
                        id: "pixel-2".into(),
                        name: "Child Pixel".into(),
                        visible: true, locked: false,
                        opacity: 0.75,
                        blend_mode: "multiply".into(),
                        transform: RasterTransform::default(),
                        mask: None,
                        width: None,
                        height: None,
                        image_key: None,
                    },
                    RasterLayerNode::Group { id: "group-2".into(), name: "Nested Group".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() },
                ],
            },
            RasterLayerNode::Adjustment { id: "adjust-1".into(), name: "Curves & Co".into(), visible: true, locked: false, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "curves".into(), params },
        ],
    }
}

#[semio_framework_async_macros::async_test]
async fn semio_example_dsl_round_trips() {
    let fixture = crate::standards::v1::subsets::any::schema::raster_image_test_snapshot();
    // 🧊️ Cold twin: a raster document owns fixed-capacity maps whose `Drop` fails closed, so the
    // helper's own decoded value is handed back to the artifact's retirement seam.
    store::os_store::test_support::assert_dsl_round_trip_cold(&fixture, crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot);
    let printed = print_dsl(&fixture);
    let reparsed = parse_dsl(&printed).expect("parse printed semio fixture");
    assert_eq!(reparsed.id, fixture.id);
    // 🧹️ Both documents own a populated asset pool and a populated `params` map, so they reach the
    // artifact's retirement seam rather than `RasterOwnedMap`'s fail-closed `Drop`.
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(reparsed);
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(fixture);
}

#[semio_framework_async_macros::async_test]
async fn raster_dsl_round_trips_representative_document() {
    let document = representative_raster_document();
    store::os_store::test_support::assert_dsl_round_trip_cold(&document, crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot);
    crate::standards::v1::subsets::any::schema::snapshot::retire_raster_snapshot(document);
}
