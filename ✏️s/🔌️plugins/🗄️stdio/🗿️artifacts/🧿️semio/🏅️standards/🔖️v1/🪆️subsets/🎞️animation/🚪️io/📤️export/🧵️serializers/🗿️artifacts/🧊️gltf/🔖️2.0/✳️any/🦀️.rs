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

use semio_s_artifact_stdio_gltf::engine::{GltfAccessorType, GltfComponentType};
use semio_s_artifact_stdio_gltf::schema::snapshot::{
    GltfAccessor, GltfAnimation, GltfAnimationChannel, GltfAnimationChannelTarget, GltfAnimationPath, GltfAnimationSampler, GltfAsset, GltfBuffer, GltfBufferView, GltfDocument, GltfInterpolation, GltfNode, GltfSourceForm,
};
use semio_s_artifact_stdio_gltf::GltfSnapshot;
use crate::standards::v1::subsets::animation::schema::snapshot::{AnimInterpolation, AnimTargetProperty, AnimValue, SemioAnimationSnapshot};
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
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn push_accessor(buffers: &mut Vec<Vec<u8>>, buffer_views: &mut Vec<GltfBufferView>, accessors: &mut Vec<GltfAccessor>, component_type: GltfComponentType, accessor_type: GltfAccessorType, values: &[f32], count: usize) -> usize {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
