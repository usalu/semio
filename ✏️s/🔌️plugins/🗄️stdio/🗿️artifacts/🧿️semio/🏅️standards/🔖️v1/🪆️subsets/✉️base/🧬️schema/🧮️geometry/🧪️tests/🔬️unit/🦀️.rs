
use super::*;

#[semio_framework_async_macros::async_test]
async fn identity_transform_round_trips_through_json() {
    let t = SemioTransform::identity();
    let json = serde_json::to_string(&t).expect("serialize");
    let back: SemioTransform = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(t, back);
    assert_eq!(back.rotation.w, 1.0);
    assert_eq!(back.scale, SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 });
}

#[semio_framework_async_macros::async_test]
async fn rgba_and_uv_default_to_zero() {
    assert_eq!(SemioRgba::default(), SemioRgba { r: 0.0, g: 0.0, b: 0.0, a: 0.0 });
    assert_eq!(SemioUv::default(), SemioUv { u: 0.0, v: 0.0 });
}

#[semio_framework_async_macros::async_test]
async fn point3_and_point2_are_plain_structs_not_tuples() {
    // 🧪️ Structural proof against the f6 §4.3 bare-tuple `DslField` gap: field ACCESS by
    // name, not `.0`/`.1` positional tuple indexing.
    let p3 = SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 };
    let p2 = SemioPoint2 { x: p3.x, y: p3.y };
    assert_eq!(p2, SemioPoint2 { x: 1.0, y: 2.0 });
}
