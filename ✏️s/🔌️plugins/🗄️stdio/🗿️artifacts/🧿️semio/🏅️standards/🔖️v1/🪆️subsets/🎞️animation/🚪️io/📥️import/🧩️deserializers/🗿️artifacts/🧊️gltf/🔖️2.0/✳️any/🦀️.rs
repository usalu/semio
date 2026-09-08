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

use semio_s_artifact_stdio_gltf::engine::decode_accessor;
use semio_s_artifact_stdio_gltf::schema::snapshot::GltfAnimationPath;
use semio_s_artifact_stdio_gltf::GltfSnapshot;
use crate::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTargetProperty, AnimTimeline, AnimValue, SemioAnimationSnapshot, STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("animation") };

pub struct SemioAnimationFromGltf;

impl ArtifactDeserializer for SemioAnimationFromGltf {
    type From = GltfSnapshot;
    type Into = SemioAnimationSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let document = &from.document;
        let mut timelines = Vec::with_capacity(document.animations.len());
        for anim in &document.animations {
            let mut channels = Vec::with_capacity(anim.channels.len());
            for ch in &anim.channels {
                let sampler = anim.samplers.get(ch.sampler).ok_or_else(|| store::PackError::Schema(format!("animation channel references out-of-range sampler {}", ch.sampler)))?;
                let times = decode_accessor(document, &from.buffers, sampler.input).map_err(store::PackError::Schema)?;
                let values = decode_accessor(document, &from.buffers, sampler.output).map_err(store::PackError::Schema)?;
                let keyframe_count = times.count;
                let is_cubic = matches!(sampler.interpolation, semio_s_artifact_stdio_gltf::schema::snapshot::GltfInterpolation::CubicSpline);
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
                        AnimTargetProperty::Translation | AnimTargetProperty::Scale => {
                            AnimValue::Vec3 { value: SemioPoint3 { x: slice.first().copied().unwrap_or(0.0), y: slice.get(1).copied().unwrap_or(0.0), z: slice.get(2).copied().unwrap_or(0.0) } }
                        }
                        AnimTargetProperty::Rotation => {
                            AnimValue::Quat { value: SemioQuaternion { x: slice.first().copied().unwrap_or(0.0), y: slice.get(1).copied().unwrap_or(0.0), z: slice.get(2).copied().unwrap_or(0.0), w: slice.get(3).copied().unwrap_or(1.0) } }
                        }
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
                    semio_s_artifact_stdio_gltf::schema::snapshot::GltfInterpolation::Linear => AnimInterpolation::Linear,
                    semio_s_artifact_stdio_gltf::schema::snapshot::GltfInterpolation::Step => AnimInterpolation::Step,
                    semio_s_artifact_stdio_gltf::schema::snapshot::GltfInterpolation::CubicSpline => AnimInterpolation::CubicSpline,
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
