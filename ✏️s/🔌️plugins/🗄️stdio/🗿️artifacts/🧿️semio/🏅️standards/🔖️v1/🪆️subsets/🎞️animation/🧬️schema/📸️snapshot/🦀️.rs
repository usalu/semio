//! 🧬️ SemioAnimationSnapshot — complete per the master plan's animation cell: `timelines` ->
//! `channels{target{node,property}, interpolation, keyframes{t, value}}` — informed by gltf's
//! `Animation`/`Channel`/`Sampler` triad (`asset/animations[]`). Ticket
//! 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W2b: replaces the
//! W1b `AnimTimeline{channels:Vec<AnimChannel{target:String,keyframes}>}` minimal scaffold with the
//! full spec shape (typed `target`/`interpolation`, the 4-variant `AnimValue` union). Named structs
//! throughout — no bare tuples (f6-final-summary.md §4.3), rotation reuses the shared
//! `engine::geometry::SemioQuaternion{x,y,z,w}` instead of a local 4-field redefinition.
use crate::standards::v1::subsets::base::schema::geometry::native;
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};
use framework_schema::ArtifactSchema;
//#region 🔖️Ids
pub const STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA: &str = "s.stdio.semio.animation";
//#endregion 🔖️Ids
//#region 🔖️Target
/// 🎯️ Which property of a node a channel drives — gltf `channel.target.path`, widened with a
/// `Custom` escape hatch for engine/extension-defined paths gltf's own spec leaves open
/// (`KHR_*` animation-pointer style extensions target arbitrary properties by name).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned)]
#[value(tag = "kind", rename_all = "camelCase")]
#[derive(Default, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum AnimTargetProperty {
    #[default]
    Translation,
    Rotation,
    Scale,
    Weights,
    Custom {
        name: String,
    },
}
/// 🎯️ A channel's animated node + which of its properties is driven.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct AnimTarget {
    pub node: String,
    #[value(default)]
    pub property: AnimTargetProperty,
}
//#endregion 🔖️Target
//#region 🔖️Interpolation
/// 📈️ gltf `sampler.interpolation` — how `keyframes` are resampled between `t` values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
#[derive(Default, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum AnimInterpolation {
    #[default]
    Linear,
    Step,
    CubicSpline,
}
//#endregion 🔖️Interpolation
//#region 🔖️Value
/// 🎞️ One keyframe's payload — a tagged union over the shapes a channel's `AnimTargetProperty` can
/// take: `Scalar` for a single animated number (e.g. a custom/extension property), `Vec3` for
/// translation/scale, `Quat` for rotation (reuses the shared named quaternion, never a bare
/// `[f64;4]`), `Weights` for morph-target weight vectors (arity = mesh's own primitive count, not
/// fixed — hence `Vec<f64>`, not a fixed array).
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum AnimValue {
    Scalar { value: f64 },
    Vec3 { value: SemioPoint3 },
    Quat { value: SemioQuaternion },
    Weights { values: Vec<f64> },
}
impl Default for AnimValue {
    fn default() -> Self {
        AnimValue::Scalar { value: 0.0 }
    }
}
//#endregion 🔖️Value
//#region 🔖️Keyframe
/// ⏱️ One sample point on a channel's timeline. Real GIFs/glTF exporters expect `t` non-decreasing
/// across a channel's own `keyframes` (a `SubsetValidator` referential invariant, see the
/// `🎹️composer` module) but this type itself stores whatever was decoded, honestly.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct AnimKeyframe {
    pub t: f64,
    #[value(default)]
    pub value: AnimValue,
}
//#endregion 🔖️Keyframe
//#region 🔖️Channel
/// 🎚️ One animated property track: gltf `channel` + its `sampler`, flattened into a single owned
/// keyframe list (this snapshot does not separately model gltf's accessor-indirection — the
/// keyframes ARE the resolved sample data).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct AnimChannel {
    pub target: AnimTarget,
    #[value(default)]
    pub interpolation: AnimInterpolation,
    #[value(default)]
    pub keyframes: Vec<AnimKeyframe>,
}
//#endregion 🔖️Channel
//#region 🔖️Timeline
/// 🎬️ One gltf `animation` entry — an optional display `name` (gltf's own `animation.name` is
/// optional and not spec-required to be unique, hence `Option<String>` rather than a name key) plus
/// its ordered `channels`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct AnimTimeline {
    #[value(default)]
    pub name: Option<String>,
    #[value(default)]
    pub channels: Vec<AnimChannel>,
}
//#endregion 🔖️Timeline
//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.animation")]
pub struct SemioAnimationSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub timelines: Vec<AnimTimeline>,
}
impl Default for SemioAnimationSnapshot {
    fn default() -> Self {
        Self { schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(), timelines: Default::default() }
    }
}
//#endregion 🔖️Snapshot
//#region 🔖️TextPrimitives
//#endregion 🔖️TextPrimitives
//#region 🔖️BinaryPrimitives
//#endregion 🔖️BinaryPrimitives
//#region 🔖️HandcraftedArtifactCodecs
//#region 🔖️DslFreeFunctions
//#endregion 🔖️DslFreeFunctions
//#endregion 🔖️HandcraftedArtifactCodecs
//#region 🔖️Demo
/// 🌱 The demo `s.stdio.semio.animation` document — one timeline exercising every `AnimValue`
/// variant (`Scalar`/`Vec3`/`Quat`/`Weights`) and every `AnimTargetProperty` kind (incl. `Custom`).
/// Single source of truth for `📚️examples/🚶️walk/🖼️assets/🗣️.dsl.semio`/
/// `🎒️.pack.semio` and for the conformance-law tests in `🎹️composer/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_animation_snapshot() -> SemioAnimationSnapshot {
    SemioAnimationSnapshot {
        schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(),
        timelines: vec![AnimTimeline {
            name: Some("walk".into()),
            channels: vec![
                AnimChannel {
                    target: AnimTarget { node: "hip".into(), property: AnimTargetProperty::Translation },
                    interpolation: AnimInterpolation::Linear,
                    keyframes: vec![AnimKeyframe { t: 0.0, value: AnimValue::Vec3 { value: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 } } }, AnimKeyframe { t: 1.0, value: AnimValue::Vec3 { value: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } } }],
                },
                AnimChannel {
                    target: AnimTarget { node: "spine".into(), property: AnimTargetProperty::Rotation },
                    interpolation: AnimInterpolation::CubicSpline,
                    keyframes: vec![AnimKeyframe { t: 0.5, value: AnimValue::Quat { value: SemioQuaternion::default() } }],
                },
                AnimChannel {
                    target: AnimTarget { node: "face".into(), property: AnimTargetProperty::Weights },
                    interpolation: AnimInterpolation::Step,
                    keyframes: vec![AnimKeyframe { t: 0.0, value: AnimValue::Weights { values: vec![0.0, 1.0, 0.5] } }],
                },
                AnimChannel {
                    target: AnimTarget { node: "rig".into(), property: AnimTargetProperty::Custom { name: "opacity".into() } },
                    interpolation: AnimInterpolation::Linear,
                    keyframes: vec![AnimKeyframe { t: 0.0, value: AnimValue::Scalar { value: 1.0 } }],
                },
            ],
        }],
    }
}
//#endregion 🔖️Demo
//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
                                       