
use super::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::drawing::io::import::deserializers::artifacts::dwg::v_ac1024::any::SemioDrawingFromDwg;
use crate::standards::v1::subsets::drawing::schema::snapshot::DrawLayer;
use semio_framework_plugin::ArtifactDeserializer;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn sample_drawing() -> SemioDrawingSnapshot {
    SemioDrawingSnapshot {
        layers: vec![DrawLayer {
            id: "0".into(),
            name: "0".into(),
            visible: true,
            root: DrawNode::Group {
                transform: SemioTransform::identity(),
                children: vec![
                    DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } }, PathSegment::LineTo { to: SemioPoint2 { x: 5.0, y: 0.0 } }], style: None },
                    DrawNode::Text { value: "hi".into(), at: SemioPoint2 { x: 1.0, y: 1.0 }, style: None },
                ],
            },
        }],
        ..SemioDrawingSnapshot::default()
    }
}

#[semio_framework_async_macros::async_test]
async fn real_round_trip_through_relocated_dwg_codec() {
    let drawing = sample_drawing();
    let dwg = semio_framework_plugin::resolve_ready(SemioDrawingToDwg::serialize(&drawing)).expect("serialize");
    assert_eq!(dwg.version, DWG_CODEC_VERSION);
    let round_tripped = semio_framework_plugin::resolve_ready(SemioDrawingFromDwg::deserialize(&dwg)).expect("deserialize");
    assert_eq!(round_tripped.layers.len(), 1);
    match &round_tripped.layers[0].root {
        DrawNode::Group { children, .. } => {
            assert_eq!(children.len(), 2);
            assert!(matches!(children[0], DrawNode::Path { .. }));
            assert!(matches!(children[1], DrawNode::Text { .. }));
        }
        other => panic!("expected Group, got {other:?}"),
    }
}
