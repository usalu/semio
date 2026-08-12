//! 📥️ Deserialize `s.stdio.gltf` (2.0/✳️any) into `s.stdio.semio` (v1/animation) — the most
//! direct pairing per the master plan (gltf 2.0 has real animation data). Reuses gltf's OWN
//! `engine::decode_accessor` to resolve `sampler.input`/`sampler.output` accessor indices into
//! real `f64` component arrays (zero re-parsing of buffer bytes here -- that decode already
//! handles bufferView byteStride/sparse substitution).
//!
//! `channel.target.node` (an accessor INDEX) becomes `AnimTarget.node` (a display STRING) via the
//! node's own `name` when present, else a synthesized `"node#<index>"` -- `animation` has no
//! index-keyed node table of its own, only names, a real and documented representational
//! difference (see the reverse serializer's own doc comment for how this is undone).
//!
//! `GLTFInterpolation::CubicSpline` samplers store 3 values per keyframe per spec (in-tangent,
//! value, out-tangent); `AnimKeyframe`/`AnimValue` have no tangent slot, so only the MIDDLE third
//! (the actual value) is kept -- tangents are honestly dropped, never fabricated, and
//! `AnimInterpolation::CubicSpline` is still recorded (informational -- see the reverse direction's
//! own doc comment on why it downgrades on export).

use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::gltf::engine::decode_accessor;
use crate::artifacts::gltf::schema::snapshot::GltfAnimationPath;
use crate::artifacts::semio::standards::v1::engine::geometry::{SemioPoint3, SemioQuaternion};
use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::{
    AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTargetProperty, AnimTimeline, AnimValue,
    SemioAnimationSnapshot, STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA,
};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("animation") };

pub struct SemioAnimationFromGltf;

impl ArtifactDeserializer for SemioAnimationFromGltf {
    type From = GltfSnapshot;
    type Into = SemioAnimationSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let document = &from.document;
        let mut timelines = Vec::with_capacity(document.animations.len());
        for anim in &document.animations {
            let mut channels = Vec::with_capacity(anim.channels.len());
            for ch in &anim.channels {
                let sampler = anim.samplers.get(ch.sampler).ok_or_else(|| store::PackError::Schema(format!("animation channel references out-of-range sampler {}", ch.sampler)))?;
                let times = decode_accessor(document, &from.buffers, sampler.input).map_err(store::PackError::Schema)?;
                let values = decode_accessor(document, &from.buffers, sampler.output).map_err(store::PackError::Schema)?;
                let keyframe_count = times.count;
                let is_cubic = matches!(sampler.interpolation, crate::artifacts::gltf::schema::snapshot::GltfInterpolation::CubicSpline);
                let multiplier = if is_cubic { 3 } else { 1 };
                let property = match ch.target.path {
                    GltfAnimationPath::Translation => AnimTargetProperty::Translation,
                    GltfAnimationPath::Rotation => AnimTargetProperty::Rotation,
                    GltfAnimationPath::Scale => AnimTargetProperty::Scale,
                    GltfAnimationPath::Weights => AnimTargetProperty::Weights,
                };
                let arity = match property {
                    AnimTargetProperty::Translation | AnimTargetProperty::Scale => 3,
                    AnimTargetProperty::Rotation => 4,
                    AnimTargetProperty::Weights => {
                        let denom = (keyframe_count * multiplier).max(1);
                        values.components.len() / denom
                    }
                    AnimTargetProperty::Custom { .. } => unreachable!("gltf's own path enum never produces Custom"),
                };
                let mut keyframes = Vec::with_capacity(keyframe_count);
                for i in 0..keyframe_count {
                    let base = (i * multiplier + if is_cubic { 1 } else { 0 }) * arity;
                    let slice = values.components.get(base..base + arity).unwrap_or(&[]);
                    let value = match property {
                        AnimTargetProperty::Translation | AnimTargetProperty::Scale => AnimValue::Vec3 {
                            value: SemioPoint3 { x: slice.first().copied().unwrap_or(0.0), y: slice.get(1).copied().unwrap_or(0.0), z: slice.get(2).copied().unwrap_or(0.0) },
                        },
                        AnimTargetProperty::Rotation => AnimValue::Quat {
                            value: SemioQuaternion { x: slice.first().copied().unwrap_or(0.0), y: slice.get(1).copied().unwrap_or(0.0), z: slice.get(2).copied().unwrap_or(0.0), w: slice.get(3).copied().unwrap_or(1.0) },
                        },
                        AnimTargetProperty::Weights => AnimValue::Weights { values: slice.to_vec() },
                        AnimTargetProperty::Custom { .. } => unreachable!(),
                    };
                    keyframes.push(AnimKeyframe { t: times.components.get(i).copied().unwrap_or(0.0), value });
                }
                let node_name = match ch.target.node {
                    Some(idx) => document.nodes.get(idx).and_then(|n| n.name.clone()).unwrap_or_else(|| format!("node#{idx}")),
                    None => "unassigned".to_string(),
                };
                let interpolation = match sampler.interpolation {
                    crate::artifacts::gltf::schema::snapshot::GltfInterpolation::Linear => AnimInterpolation::Linear,
                    crate::artifacts::gltf::schema::snapshot::GltfInterpolation::Step => AnimInterpolation::Step,
                    crate::artifacts::gltf::schema::snapshot::GltfInterpolation::CubicSpline => AnimInterpolation::CubicSpline,
                };
                channels.push(AnimChannel { target: AnimTarget { node: node_name, property }, interpolation, keyframes });
            }
            timelines.push(AnimTimeline { name: anim.name.clone(), channels });
        }
        Ok(SemioAnimationSnapshot { schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(), timelines })
    }
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gltf::schema::snapshot::{
        GltfAnimation, GltfAnimationChannel, GltfAnimationChannelTarget, GltfAnimationSampler, GltfDocument, GltfInterpolation, GltfNode, GltfSourceForm,
    };
    use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
    use crate::artifacts::gltf::standards::v2_0::subsets::any::schema::{GltfAccessorSpec, GltfBuilderConstruction as GltfDocBuilder};
    use semio_framework_plugin::ArtifactBuilder;

    /// 🏗️ Builds a real, decodable glTF document: two nodes, one animation with a Linear
    /// Translation channel (2 keyframes) and a CubicSpline Weights channel (2 keyframes, arity 2)
    /// -- exercises tangent-stripping and node-name resolution in one real-world-shaped fixture.
    fn real_world_gltf() -> GltfSnapshot {
        let mut b = GltfDocBuilder::empty();
        b.set_asset_version("2.0");
        let n0 = b.add_node(None);
        let n1 = b.add_node(None);

        let mut time_bytes = Vec::new();
        for t in [0.0f32, 1.0f32] { time_bytes.extend_from_slice(&t.to_le_bytes()); }
        let time_buf = b.add_buffer(time_bytes);
        let time_bv = b.add_buffer_view(time_buf, 0, 8, None, None);
        let time_acc = b.add_accessor(GltfAccessorSpec::new(GltfComponentType::Float, GltfAccessorType::Scalar, 2).with_buffer_view(time_bv, 0));

        let mut translation_bytes = Vec::new();
        for v in [[0.0f32, 0.0, 0.0], [1.0, 2.0, 3.0]] { for c in v { translation_bytes.extend_from_slice(&c.to_le_bytes()); } }
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
        for f in weight_floats { weight_bytes.extend_from_slice(&f.to_le_bytes()); }
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

    #[test]
    fn deserialize_maps_linear_translation_channel_with_named_node() {
        let anim = SemioAnimationFromGltf::deserialize(&real_world_gltf()).expect("deserialize");
        assert_eq!(anim.timelines.len(), 1);
        assert_eq!(anim.timelines[0].name.as_deref(), Some("clip"));
        let ch = &anim.timelines[0].channels[0];
        assert_eq!(ch.target.node, "hip");
        assert_eq!(ch.target.property, AnimTargetProperty::Translation);
        assert_eq!(ch.interpolation, AnimInterpolation::Linear);
        assert_eq!(ch.keyframes.len(), 2);
        assert_eq!(ch.keyframes[1].value, AnimValue::Vec3 { value: SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 } });
    }

    #[test]
    fn deserialize_strips_cubic_spline_tangents_keeping_only_the_real_value_third() {
        let anim = SemioAnimationFromGltf::deserialize(&real_world_gltf()).expect("deserialize");
        let ch = &anim.timelines[0].channels[1];
        assert_eq!(ch.target.node, "node#1"); // unnamed node -> synthesized name
        assert_eq!(ch.interpolation, AnimInterpolation::CubicSpline);
        assert_eq!(ch.keyframes.len(), 2);
        assert_eq!(ch.keyframes[0].value, AnimValue::Weights { values: vec![0.25, 0.5] });
        assert_eq!(ch.keyframes[1].value, AnimValue::Weights { values: vec![0.75, 0.125] });
    }
}
//#endregion 🔖️Tests
