//! 📤️ Serialize `s.stdio.semio` (v1/animation) into `s.stdio.gltf` (2.0/✳️any) — mirror image of
//! this pair's deserializer. Builds ONE synthetic `GltfNode` per distinct `AnimTarget.node` name
//! referenced by any channel (in first-seen order) -- `animation` carries no node hierarchy/mesh
//! graph of its own (only name strings, see the deserializer's own doc comment), so this is
//! honestly the minimal document that can address every channel's target, not a full scene.
//!
//! Honest, documented lossy/simplifying choices (real, never fabricated):
//! - `AnimTargetProperty::Custom` channels are DROPPED (gltf 2.0's core `GltfAnimationPath` enum
//!   has no equivalent -- that's a `KHR_animation_pointer`-style extension concept this artifact's
//!   typed model doesn't represent -- fabricating a channel gltf can't actually express would be
//!   dishonest; dropping with this documented reason is the chosen strategy).
//! - `AnimInterpolation::CubicSpline` DOWNGRADES to `GltfInterpolation::Linear` on export --
//!   `AnimKeyframe` never stored in/out tangents in the first place (see the deserializer's own doc
//!   comment on why they're stripped on import), so this bridge cannot fabricate the tangent triple
//!   spec-required data a real `CUBICSPLINE` sampler output needs; picking a tangent-free
//!   interpolation mode instead of inventing zero tangents is the honest choice.
//! - Every keyframe's time/value array becomes its OWN buffer (one accessor's worth) -- simple and
//!   correct, not byte-packing-optimal; gltf's own encoder is free to re-pack on a later true
//!   binary write, this bridge only produces the typed `Snapshot`.

use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::schema::snapshot::{
    GltfAccessor, GltfAnimation, GltfAnimationChannel, GltfAnimationChannelTarget, GltfAnimationPath, GltfAnimationSampler, GltfAsset, GltfBuffer, GltfBufferView, GltfDocument, GltfInterpolation, GltfNode, GltfSourceForm,
};
use crate::artifacts::gltf::GltfSnapshot;
use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::{AnimInterpolation, AnimTargetProperty, AnimValue, SemioAnimationSnapshot};
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("animation") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };

pub struct SemioAnimationToGltf;

impl ArtifactSerializer for SemioAnimationToGltf {
    type From = SemioAnimationSnapshot;
    type Into = GltfSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let mut buffers: Vec<Vec<u8>> = Vec::new();
        let mut buffer_views: Vec<GltfBufferView> = Vec::new();
        let mut accessors: Vec<GltfAccessor> = Vec::new();
        let mut node_names: Vec<String> = Vec::new();
        let mut animations = Vec::with_capacity(from.timelines.len());

        for timeline in &from.timelines {
            let mut channels = Vec::new();
            let mut samplers = Vec::new();
            for ch in &timeline.channels {
                let path = match ch.target.property {
                    AnimTargetProperty::Translation => GltfAnimationPath::Translation,
                    AnimTargetProperty::Rotation => GltfAnimationPath::Rotation,
                    AnimTargetProperty::Scale => GltfAnimationPath::Scale,
                    AnimTargetProperty::Weights => GltfAnimationPath::Weights,
                    AnimTargetProperty::Custom { .. } => continue, // 📦️ dropped, documented above
                };
                let node_index = node_names.iter().position(|n| n == &ch.target.node).unwrap_or_else(|| {
                    node_names.push(ch.target.node.clone());
                    node_names.len() - 1
                });

                let times: Vec<f32> = ch.keyframes.iter().map(|k| k.t as f32).collect();
                let input_acc = push_accessor(&mut buffers, &mut buffer_views, &mut accessors, GltfComponentType::Float, GltfAccessorType::Scalar, &times, times.len());

                let (accessor_type, output_values): (GltfAccessorType, Vec<f32>) = match path {
                    GltfAnimationPath::Translation | GltfAnimationPath::Scale => {
                        let mut v = Vec::with_capacity(ch.keyframes.len() * 3);
                        for k in &ch.keyframes {
                            let p = if let AnimValue::Vec3 { value } = &k.value { *value } else { Default::default() };
                            v.extend_from_slice(&[p.x as f32, p.y as f32, p.z as f32]);
                        }
                        (GltfAccessorType::Vec3, v)
                    }
                    GltfAnimationPath::Rotation => {
                        let mut v = Vec::with_capacity(ch.keyframes.len() * 4);
                        for k in &ch.keyframes {
                            let q = if let AnimValue::Quat { value } = &k.value { *value } else { Default::default() };
                            v.extend_from_slice(&[q.x as f32, q.y as f32, q.z as f32, q.w as f32]);
                        }
                        (GltfAccessorType::Vec4, v)
                    }
                    GltfAnimationPath::Weights => {
                        let mut v = Vec::new();
                        for k in &ch.keyframes {
                            if let AnimValue::Weights { values } = &k.value {
                                v.extend(values.iter().map(|x| *x as f32));
                            }
                        }
                        (GltfAccessorType::Scalar, v)
                    }
                };
                let output_count = ch.keyframes.len();
                let output_acc = push_accessor(&mut buffers, &mut buffer_views, &mut accessors, GltfComponentType::Float, accessor_type, &output_values, output_count);

                let interpolation = match ch.interpolation {
                    AnimInterpolation::Linear => GltfInterpolation::Linear,
                    AnimInterpolation::Step => GltfInterpolation::Step,
                    AnimInterpolation::CubicSpline => GltfInterpolation::Linear, // 📦️ downgraded, documented above
                };
                let sampler_index = samplers.len();
                samplers.push(GltfAnimationSampler { input: input_acc, interpolation, output: output_acc, extensions: None, extras: None });
                channels.push(GltfAnimationChannel { sampler: sampler_index, target: GltfAnimationChannelTarget { node: Some(node_index), path, extensions: None, extras: None }, extensions: None, extras: None });
            }
            animations.push(GltfAnimation { channels, samplers, name: timeline.name.clone(), extensions: None, extras: None });
        }

        let nodes: Vec<GltfNode> = node_names.into_iter().map(|name| GltfNode { name: Some(name), ..GltfNode::default() }).collect();
        let buffers_meta: Vec<GltfBuffer> = buffers.iter().map(|b| GltfBuffer { byte_length: b.len(), uri: None, name: None, extensions: None, extras: None }).collect();

        let document = GltfDocument {
            asset: GltfAsset::default(),
            scene: None,
            scenes: Vec::new(),
            nodes,
            meshes: Vec::new(),
            accessors,
            buffer_views,
            buffers: buffers_meta,
            materials: Vec::new(),
            textures: Vec::new(),
            images: Vec::new(),
            samplers: Vec::new(),
            skins: Vec::new(),
            animations,
            cameras: Vec::new(),
            extensions_used: Vec::new(),
            extensions_required: Vec::new(),
            extensions: None,
            extras: None,
        };
        Ok(GltfSnapshot { schema: "s.stdio.gltf".into(), document, buffers, source_form: GltfSourceForm::Json })
    }
}

/// 📦️ Writes `values` (already the flat, row-major component list) into its own new buffer, spans
/// it with one bufferView, and registers one accessor over it (`count` elements of `accessor_type`
/// each). Returns the new accessor index.
async fn push_accessor(buffers: &mut Vec<Vec<u8>>, buffer_views: &mut Vec<GltfBufferView>, accessors: &mut Vec<GltfAccessor>, component_type: GltfComponentType, accessor_type: GltfAccessorType, values: &[f32], count: usize) -> usize {
    let mut bytes = Vec::with_capacity(values.len() * 4);
    for v in values {
        bytes.extend_from_slice(&v.to_le_bytes());
    }
    let byte_length = bytes.len();
    let buffer_index = buffers.len();
    buffers.push(bytes);
    let bv_index = buffer_views.len();
    buffer_views.push(GltfBufferView { buffer: buffer_index, byte_offset: 0, byte_length, byte_stride: None, target: None, name: None, extensions: None, extras: None });
    let accessor_index = accessors.len();
    accessors.push(GltfAccessor { buffer_view: Some(bv_index), byte_offset: 0, component_type, normalized: false, count, kind: accessor_type, max: None, min: None, sparse: None, name: None, extensions: None, extras: None });
    accessor_index
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::gltf::engine::decode_accessor;
    use crate::artifacts::semio::standards::v1::subsets::animation::io::gltf_deserializer::SemioAnimationFromGltf;
    use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimKeyframe, AnimTarget, AnimTimeline, STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA};
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::{SemioPoint3, SemioQuaternion};
    use semio_framework_plugin::ArtifactDeserializer;

    async fn real_world_animation() -> SemioAnimationSnapshot {
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
    #[test]
    async fn animation_to_gltf_to_animation_round_trips_everything_representable() {
        let original = real_world_animation();
        let gltf = semio_framework_plugin::resolve_ready(SemioAnimationToGltf::serialize(&original)).expect("serialize");
        assert_eq!(gltf.document.animations.len(), 1);
        assert_eq!(gltf.document.animations[0].channels.len(), 2);
        assert_eq!(gltf.document.nodes.len(), 2);
        let back = semio_framework_plugin::resolve_ready(SemioAnimationFromGltf::deserialize(&gltf)).expect("deserialize");
        assert_eq!(back, original);
    }

    #[test]
    async fn accessors_decode_to_the_real_values_written() {
        let gltf = semio_framework_plugin::resolve_ready(SemioAnimationToGltf::serialize(&real_world_animation())).expect("serialize");
        let sampler = &gltf.document.animations[0].samplers[0];
        let decoded = decode_accessor(&gltf.document, &gltf.buffers, sampler.output).expect("decode");
        assert_eq!(decoded.components, vec![0.0, 0.0, 0.0, 1.0, 0.5, 0.0]);
    }

    #[test]
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

    #[test]
    async fn cubic_spline_downgrades_to_linear_on_export_documented() {
        let mut snap = real_world_animation();
        snap.timelines[0].channels[0].interpolation = AnimInterpolation::CubicSpline;
        let gltf = semio_framework_plugin::resolve_ready(SemioAnimationToGltf::serialize(&snap)).expect("serialize");
        assert_eq!(gltf.document.animations[0].samplers[0].interpolation, GltfInterpolation::Linear);
    }
}
//#endregion 🔖️Tests
