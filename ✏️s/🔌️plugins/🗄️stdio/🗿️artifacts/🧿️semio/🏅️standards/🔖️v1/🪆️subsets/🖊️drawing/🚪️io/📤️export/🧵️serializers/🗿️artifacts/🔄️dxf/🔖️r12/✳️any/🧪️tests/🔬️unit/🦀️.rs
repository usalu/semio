use super::*;
use crate::standards::v1::subsets::base::schema::geometry::SemioTransform;
use crate::standards::v1::subsets::drawing::schema::snapshot::DrawLayer;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn ellipse_path(cx: f64, cy: f64, r: f64) -> Vec<PathSegment> {
    vec![
        PathSegment::MoveTo { to: SemioPoint2 { x: cx + r, y: cy } },
        PathSegment::ArcTo { rx: r, ry: r, x_rotation: 0.0, large_arc: true, sweep: true, to: SemioPoint2 { x: cx - r, y: cy } },
        PathSegment::ArcTo { rx: r, ry: r, x_rotation: 0.0, large_arc: true, sweep: true, to: SemioPoint2 { x: cx + r, y: cy } },
        PathSegment::Close,
    ]
}

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
                    DrawNode::Path { segments: ellipse_path(2.0, 2.0, 1.0), style: None },
                    DrawNode::Path { segments: vec![PathSegment::MoveTo { to: SemioPoint2 { x: 0.0, y: 0.0 } }, PathSegment::LineTo { to: SemioPoint2 { x: 5.0, y: 0.0 } }], style: None },
                    DrawNode::Text { value: "hi".into(), at: SemioPoint2 { x: 1.0, y: 1.0 }, style: None },
                ],
            },
        }],
        ..SemioDrawingSnapshot::default()
    }
}

/// 🧪️ Real round trip through dxf's own real ASCII writer/reader; a genuine circle round
/// trips EXACTLY (not flattened), a straight-line path becomes a real POLYLINE.
#[semio_framework_async_macros::async_test]
async fn real_text_round_trip_through_dxf_codec() {
    let drawing = sample_drawing();
    let dxf = semio_framework_plugin::resolve_ready(SemioDrawingToDxf::serialize(&drawing)).expect("serialize");
    assert_eq!(dxf.entities.len(), 3);
    assert!(matches!(dxf.entities[0], DxfEntity::Circle { .. }));
    assert!(matches!(dxf.entities[1], DxfEntity::Polyline { .. }));
    assert!(matches!(dxf.entities[2], DxfEntity::Text { .. }));

    let text = semio_s_artifact_stdio_dxf::schema::snapshot::print_dxf_document(&dxf);
    let reparsed = semio_s_artifact_stdio_dxf::schema::snapshot::parse_dxf_document(&text).expect("reparse real dxf text");
    match &reparsed.entities[0] {
        DxfEntity::Circle { center, radius, .. } => {
            assert!((center[0] - 2.0).abs() < 1e-6);
            assert!((radius - 1.0).abs() < 1e-6);
        }
        other => panic!("expected exact Circle, got {other:?}"),
    }
}
