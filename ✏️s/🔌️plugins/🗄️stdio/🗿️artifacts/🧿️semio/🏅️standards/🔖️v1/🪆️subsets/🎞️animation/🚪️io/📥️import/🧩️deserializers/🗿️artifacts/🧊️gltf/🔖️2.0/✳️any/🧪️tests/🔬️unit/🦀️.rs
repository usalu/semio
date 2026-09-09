use super::*;
use semio_framework_plugin::ArtifactBuilder;
use semio_s_artifact_stdio_gltf::engine::{GltfAccessorType, GltfComponentType};
use semio_s_artifact_stdio_gltf::schema::snapshot::{GltfAnimation, GltfAnimationChannel, GltfAnimationChannelTarget, GltfAnimationSampler, GltfDocument, GltfInterpolation, GltfNode, GltfSourceForm};
use semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::schema::{GltfAccessorSpec, GltfBuilderConstruction as GltfDocBuilder};

/// 🏗️ Builds a real, decodable glTF document: two nodes, one animation with a Linear
/// Translation channel (2 keyframes) and a CubicSpline Weights channel (2 keyframes, arity 2)
/// -- exercises tangent-stripping and node-name resolution in one real-world-shaped fixture.
async fn real_world_gltf() -> GltfSnapshot {
    let mut b = GltfDocBuilder::empty();
    b.set_asset_version("2.0");
    let n0 = b.add_node(None);
    let n1 = b.add_node(None);

    let mut time_bytes = Vec::new();
    for t in [0.0f32, 1.0f32] {
        time_bytes.extend_from_slice(&t.to_le_bytes());
    }
    let time_buf = b.add_buffer(time_bytes);
    let time_bv = b.add_buffer_view(time_buf, 0, 8, None, None);
    let time_acc = b.add_accessor(GltfAccessorSpec::new(GltfComponentType::Float, GltfAccessorType::Scalar, 2).with_buffer_view(time_bv, 0));

    let mut translation_bytes = Vec::new();
    for v in [[0.0f32, 0.0, 0.0], [1.0, 2.0, 3.0]] {
        for c in v {
            translation_bytes.extend_from_slice(&c.to_le_bytes());
        }
    }
    let trans_buf = b.add_buffer(translation_bytes);
    let trans_bv = b.add_buffer_view(trans_buf, 0, 24, None, None);
    let trans_acc = b.add_accessor(GltfAccessorSpec::new(GltfComponentType::Float, GltfAccessorType::Vec3, 2).with_buffer_view(trans_bv, 0));

    // CubicSpline weights: 2 keyframes * 2 morph targets * 3 (in/value/out) = 12 floats.
    // Values chosen exactly f32-representable (0.25/0.5/0.75/0.125) so the f32-widened-to-f64
    // decode compares bit-exact against plain f64 literals below -- glTF's own accessor
    // component type IS real IEEE-754 single precision, a genuine boundary this test isolates
    // from (see the reverse serializer's own doc comment on this same precision fact).
    let mut weight_bytes = Vec::new();
    let weight_floats: [f32; 12] = [0.0, 0.0, 0.25, 0.5, 0.0, 0.0, 0.0, 0.0, 0.75, 0.125, 0.0, 0.0];
    for f in weight_floats {
        weight_bytes.extend_from_slice(&f.to_le_bytes());
    }
    let weight_buf = b.add_buffer(weight_bytes);
    let weight_bv = b.add_buffer_view(weight_buf, 0, 48, None, None);
    let weight_acc = b.add_accessor(GltfAccessorSpec::new(GltfComponentType::Float, GltfAccessorType::Scalar, 12).with_buffer_view(weight_bv, 0));

    let anim = GltfAnimation {
        name: Some("clip".into()),
        samplers: vec![
            GltfAnimationSampler { input: time_acc, interpolation: GltfInterpolation::Linear, output: trans_acc, extensions: None, extras: None },
            GltfAnimationSampler { input: time_acc, interpolation: GltfInterpolation::CubicSpline, output: weight_acc, extensions: None, extras: None },
        ],
        channels: vec![
            GltfAnimationChannel { sampler: 0, target: GltfAnimationChannelTarget { node: Some(n0), path: GltfAnimationPath::Translation, extensions: None, extras: None }, extensions: None, extras: None },
            GltfAnimationChannel { sampler: 1, target: GltfAnimationChannelTarget { node: Some(n1), path: GltfAnimationPath::Weights, extensions: None, extras: None }, extensions: None, extras: None },
        ],
        extensions: None,
        extras: None,
    };
    let mut document: GltfDocument = b.document().clone();
    document.nodes[n0] = GltfNode { name: Some("hip".into()), ..GltfNode::default() };
    document.animations.push(anim);
    GltfSnapshot { schema: "s.stdio.gltf".into(), document, buffers: b.buffers().to_vec(), source_form: GltfSourceForm::Json }
}

#[semio_framework_async_macros::async_test]
async fn deserialize_maps_linear_translation_channel_with_named_node() {
    let anim = semio_framework_plugin::resolve_ready(SemioAnimationFromGltf::deserialize(&real_world_gltf().await)).expect("deserialize");
    assert_eq!(anim.timelines.len(), 1);
    assert_eq!(anim.timelines[0].name.as_deref(), Some("clip"));
    let ch = &anim.timelines[0].channels[0];
    assert_eq!(ch.target.node, "hip");
    assert_eq!(ch.target.property, AnimTargetProperty::Translation);
    assert_eq!(ch.interpolation, AnimInterpolation::Linear);
    assert_eq!(ch.keyframes.len(), 2);
    assert_eq!(ch.keyframes[1].value, AnimValue::Vec3 { value: SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 } });
}

#[semio_framework_async_macros::async_test]
async fn deserialize_strips_cubic_spline_tangents_keeping_only_the_real_value_third() {
    let anim = semio_framework_plugin::resolve_ready(SemioAnimationFromGltf::deserialize(&real_world_gltf().await)).expect("deserialize");
    let ch = &anim.timelines[0].channels[1];
    assert_eq!(ch.target.node, "node#1"); // unnamed node -> synthesized name
    assert_eq!(ch.interpolation, AnimInterpolation::CubicSpline);
    assert_eq!(ch.keyframes.len(), 2);
    assert_eq!(ch.keyframes[0].value, AnimValue::Weights { values: vec![0.25, 0.5] });
    assert_eq!(ch.keyframes[1].value, AnimValue::Weights { values: vec![0.75, 0.125] });
}
