use super::*;
use crate::standards::v1::subsets::animation::io::gltf_deserializer::SemioAnimationFromGltf;
use crate::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimKeyframe, AnimTarget, AnimTimeline, STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};
use semio_framework_plugin::ArtifactDeserializer;
use semio_s_artifact_stdio_gltf::engine::decode_accessor;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn real_world_animation() -> SemioAnimationSnapshot {
    SemioAnimationSnapshot {
        schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(),
        timelines: vec![AnimTimeline {
            name: Some("walk".into()),
            channels: vec![
                AnimChannel {
                    target: AnimTarget { node: "hip".into(), property: AnimTargetProperty::Translation },
                    interpolation: AnimInterpolation::Linear,
                    keyframes: vec![AnimKeyframe { t: 0.0, value: AnimValue::Vec3 { value: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 } } }, AnimKeyframe { t: 1.0, value: AnimValue::Vec3 { value: SemioPoint3 { x: 1.0, y: 0.5, z: 0.0 } } }],
                },
                AnimChannel {
                    target: AnimTarget { node: "spine".into(), property: AnimTargetProperty::Rotation },
                    interpolation: AnimInterpolation::Step,
                    // 🧭️ x/y/z/w chosen exactly f32-representable (0.5/0.25, not e.g. 0.7071)
                    // -- accessor storage is real IEEE-754 single precision per glTF's own
                    // componentType=FLOAT spec (see this file's module doc comment); an f64
                    // value NOT exactly representable in f32 would not byte-for-byte survive
                    // this round trip, which is glTF's own real numeric boundary, not a defect
                    // in this bridge -- picking exact values here isolates the MAPPING's
                    // correctness from that separately-documented precision fact.
                    keyframes: vec![AnimKeyframe { t: 0.5, value: AnimValue::Quat { value: SemioQuaternion { x: 0.0, y: 0.0, z: 0.5, w: 0.25 } } }],
                },
            ],
        }],
    }
}

/// 🧪️ codec_retention_law-style round trip FROM the semio side: animation -> gltf -> animation
/// is a clean fixpoint for Linear/Step Translation/Rotation channels (everything `animation` can
/// represent for these property kinds survives); CubicSpline/Custom are documented-lossy and
/// deliberately excluded from THIS fixture (see the dedicated tests below for their behavior).
#[semio_framework_async_macros::async_test]
async fn animation_to_gltf_to_animation_round_trips_everything_representable() {
    let original = real_world_animation();
    let gltf = semio_framework_plugin::resolve_ready(SemioAnimationToGltf::serialize(&original)).expect("serialize");
    assert_eq!(gltf.document.animations.len(), 1);
    assert_eq!(gltf.document.animations[0].channels.len(), 2);
    assert_eq!(gltf.document.nodes.len(), 2);
    let back = semio_framework_plugin::resolve_ready(SemioAnimationFromGltf::deserialize(&gltf)).expect("deserialize");
    assert_eq!(back, original);
}

#[semio_framework_async_macros::async_test]
async fn accessors_decode_to_the_real_values_written() {
    let gltf = semio_framework_plugin::resolve_ready(SemioAnimationToGltf::serialize(&real_world_animation())).expect("serialize");
    let sampler = &gltf.document.animations[0].samplers[0];
    let decoded = decode_accessor(&gltf.document, &gltf.buffers, sampler.output).expect("decode");
    assert_eq!(decoded.components, vec![0.0, 0.0, 0.0, 1.0, 0.5, 0.0]);
}

#[semio_framework_async_macros::async_test]
async fn custom_target_property_is_honestly_dropped_not_fabricated() {
    let mut snap = real_world_animation();
    snap.timelines[0].channels.push(AnimChannel {
        target: AnimTarget { node: "rig".into(), property: AnimTargetProperty::Custom { name: "opacity".into() } },
        interpolation: AnimInterpolation::Linear,
        keyframes: vec![AnimKeyframe { t: 0.0, value: AnimValue::Scalar { value: 1.0 } }],
    });
    let gltf = semio_framework_plugin::resolve_ready(SemioAnimationToGltf::serialize(&snap)).expect("serialize");
    assert_eq!(gltf.document.animations[0].channels.len(), 2, "Custom channel must not appear in gltf's own channel list");
}

#[semio_framework_async_macros::async_test]
async fn cubic_spline_downgrades_to_linear_on_export_documented() {
    let mut snap = real_world_animation();
    snap.timelines[0].channels[0].interpolation = AnimInterpolation::CubicSpline;
    let gltf = semio_framework_plugin::resolve_ready(SemioAnimationToGltf::serialize(&snap)).expect("serialize");
    assert_eq!(gltf.document.animations[0].samplers[0].interpolation, GltfInterpolation::Linear);
}
