use super::*;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioTransform};
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
use protocol::Inference;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn fixture() -> SemioDrawingSnapshot {
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: 10.0, height: 10.0, background: None },
        styles: Vec::new(),
        layers: vec![DrawLayer {
            id: "l0".into(),
            name: "base".into(),
            visible: true,
            root: DrawNode::Group { transform: SemioTransform::identity(), children: vec![DrawNode::Text { value: "hi".into(), at: SemioPoint2 { x: 1.0, y: 1.0 }, style: None }] },
        }],
    }
}

#[semio_framework_async_macros::async_test]
async fn inference_determinism_law() {
    let snapshot = fixture();
    assert_eq!(SemioDrawingInference::infer(&snapshot), SemioDrawingInference::infer(&snapshot));
}

#[semio_framework_async_macros::async_test]
async fn inference_default_law() {
    assert_eq!(SemioDrawingInference::infer(&SemioDrawingSnapshot::default()), SemioDrawingInference::default());
}

#[semio_framework_async_macros::async_test]
async fn inference_matches_direct_infer_field_call() {
    let snapshot = fixture();
    let inferred = SemioDrawingInference::infer(&snapshot);
    let direct = store::infer_field::<SemioDrawingSnapshot, DrawFlattenedScene>(&snapshot, None);
    for (key, value) in &direct {
        assert_eq!(inferred.flattened_scene.get(key), Some(value), "inference must match infer_field exactly for {key}");
    }
}
