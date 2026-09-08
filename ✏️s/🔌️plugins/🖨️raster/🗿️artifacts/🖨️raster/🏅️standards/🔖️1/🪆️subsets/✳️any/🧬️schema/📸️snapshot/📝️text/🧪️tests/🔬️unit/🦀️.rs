
use super::*;
use crate::RasterOwnedMap;
use crate::{RASTER_DOCUMENT_SCHEMA, RasterImageAsset, RasterLayerMask, RasterLayerNode, RasterTransform};

/// 📄️ Handcrafted document exercising every layer kind/field, shared with the `pack`/`op`
/// taxonomy nodes' own copies (each node keeps its own private copy, per §7 test isolation).
fn representative_raster_document() -> RasterSnapshot {
    let mut assets = RasterOwnedMap::new();
    assets.insert("asset-1".into(), crate::image_asset_child_handle("asset-1", &RasterImageAsset { mime: "image/png".into(), data: b"abc".to_vec() })).expect("bounded fixture operation succeeds");
    let mut params = RasterOwnedMap::new();
    params.insert("brightness".into(), dsl::DslValue::float(0.06)).expect("bounded fixture operation succeeds");
    params.insert("label".into(), dsl::DslValue::String("Warm \"Curve\"".to_string())).expect("bounded fixture operation succeeds");
    params.insert("enabled".into(), dsl::DslValue::Bool(true)).expect("bounded fixture operation succeeds");
    params.insert("fallback".into(), dsl::DslValue::Null).expect("bounded fixture operation succeeds");
    params
        .insert(
            "curves".into(),
            dsl::DslValue::Array(vec![
                dsl::DslValue::Array(vec![dsl::DslValue::float(0.0), dsl::DslValue::float(0.0)]),
                dsl::DslValue::Array(vec![dsl::DslValue::float(0.25), dsl::DslValue::float(0.2)]),
                dsl::DslValue::Array(vec![dsl::DslValue::float(1.0), dsl::DslValue::float(1.0)]),
            ]),
        )
        .expect("bounded fixture operation succeeds");
    params.insert("nested".into(), dsl::DslValue::Object(vec![("inner".to_string(), dsl::DslValue::float(1.5))])).expect("bounded fixture operation succeeds");
    RasterSnapshot {
        schema: RASTER_DOCUMENT_SCHEMA.into(),
        id: "doc-1".into(),
        title: Some("Representative \"Doc\"".into()),
        assets,
        layers: vec![
            RasterLayerNode::Pixel {
                id: "pixel-1".into(),
                name: "Pixel One".into(),
                visible: true,
                opacity: 1.0,
                blend_mode: "normal".into(),
                transform: RasterTransform::default(),
                mask: Some(RasterLayerMask { enabled: true, linked: false, invert: true, width: Some(64), height: None }),
                width: Some(256),
                height: Some(256),
                image_key: Some("asset-1".into()),
            },
            RasterLayerNode::Group {
                id: "group-1".into(),
                name: "Group / Nested".into(),
                visible: false,
                opacity: 0.5,
                blend_mode: "screen".into(),
                transform: RasterTransform { x: 1.0, y: -2.0, scale_x: 1.5, scale_y: 0.5, rotation: 12.0 },
                mask: None,
                children: vec![
                    RasterLayerNode::Pixel {
                        id: "pixel-2".into(),
                        name: "Child Pixel".into(),
                        visible: true,
                        opacity: 0.75,
                        blend_mode: "multiply".into(),
                        transform: RasterTransform::default(),
                        mask: None,
                        width: None,
                        height: None,
                        image_key: None,
                    },
                    RasterLayerNode::Group { id: "group-2".into(), name: "Nested Group".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), mask: None, children: Vec::new() },
                ],
            },
            RasterLayerNode::Adjustment { id: "adjust-1".into(), name: "Curves & Co".into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), transform: RasterTransform::default(), adjustment_kind: "curves".into(), params },
        ],
    }
}

#[semio_framework_async_macros::async_test]
async fn semio_example_dsl_round_trips() {
    let fixture = crate::schema::semio_fixture_snapshot();
    store::os_store::test_support::assert_dsl_round_trip(&fixture);
    let printed = print_dsl(&fixture);
    let reparsed = parse_dsl(&printed).expect("parse printed semio fixture");
    assert_eq!(reparsed.id, fixture.id);
}

#[semio_framework_async_macros::async_test]
async fn raster_dsl_round_trips_representative_document() {
    store::os_store::test_support::assert_dsl_round_trip(&representative_raster_document());
}
