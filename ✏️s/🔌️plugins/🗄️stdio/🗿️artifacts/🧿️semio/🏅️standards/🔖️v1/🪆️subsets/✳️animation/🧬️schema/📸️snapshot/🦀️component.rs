//! 🧬️ SemioAnimationSnapshot — complete per the master plan's animation cell: `timelines` ->
//! `channels{target{node,property}, interpolation, keyframes{t, value}}` — informed by gltf's
//! `Animation`/`Channel`/`Sampler` triad (`asset/animations[]`). Ticket
//! 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W2b: replaces the
//! W1b `AnimTimeline{channels:Vec<AnimChannel{target:String,keyframes}>}` minimal scaffold with the
//! full spec shape (typed `target`/`interpolation`, the 4-variant `AnimValue` union). Named structs
//! throughout — no bare tuples (f6-final-summary.md §4.3), rotation reuses the shared
//! `engine::geometry::SemioQuaternion{x,y,z,w}` instead of a local 4-field redefinition.

use crate::artifacts::semio::standards::v1::engine::geometry::{SemioPoint3, SemioQuaternion};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Ids
pub const STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA: &str = "s.stdio.semio.animation";
//#endregion 🔖️Ids

//#region 🔖️Target
/// 🎯️ Which property of a node a channel drives — gltf `channel.target.path`, widened with a
/// `Custom` escape hatch for engine/extension-defined paths gltf's own spec leaves open
/// (`KHR_*` animation-pointer style extensions target arbitrary properties by name).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum AnimTargetProperty {
    Translation,
    Rotation,
    Scale,
    Weights,
    Custom { name: String },
}

impl Default for AnimTargetProperty {
    fn default() -> Self { AnimTargetProperty::Translation }
}

/// 🎯️ A channel's animated node + which of its properties is driven.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimTarget {
    pub node: String,
    #[serde(default)]
    pub property: AnimTargetProperty,
}
//#endregion 🔖️Target

//#region 🔖️Interpolation
/// 📈️ gltf `sampler.interpolation` — how `keyframes` are resampled between `t` values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AnimInterpolation {
    Linear,
    Step,
    CubicSpline,
}

impl Default for AnimInterpolation {
    fn default() -> Self { AnimInterpolation::Linear }
}
//#endregion 🔖️Interpolation

//#region 🔖️Value
/// 🎞️ One keyframe's payload — a tagged union over the shapes a channel's `AnimTargetProperty` can
/// take: `Scalar` for a single animated number (e.g. a custom/extension property), `Vec3` for
/// translation/scale, `Quat` for rotation (reuses the shared named quaternion, never a bare
/// `[f64;4]`), `Weights` for morph-target weight vectors (arity = mesh's own primitive count, not
/// fixed — hence `Vec<f64>`, not a fixed array).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum AnimValue {
    Scalar { value: f64 },
    Vec3 { value: SemioPoint3 },
    Quat { value: SemioQuaternion },
    Weights { values: Vec<f64> },
}

impl Default for AnimValue {
    fn default() -> Self { AnimValue::Scalar { value: 0.0 } }
}
//#endregion 🔖️Value

//#region 🔖️Keyframe
/// ⏱️ One sample point on a channel's timeline. Real GIFs/glTF exporters expect `t` non-decreasing
/// across a channel's own `keyframes` (a `SubsetValidator` referential invariant, see the
/// `🎹️composer` module) but this type itself stores whatever was decoded, honestly.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimKeyframe {
    pub t: f64,
    #[serde(default)]
    pub value: AnimValue,
}
//#endregion 🔖️Keyframe

//#region 🔖️Channel
/// 🎚️ One animated property track: gltf `channel` + its `sampler`, flattened into a single owned
/// keyframe list (this snapshot does not separately model gltf's accessor-indirection — the
/// keyframes ARE the resolved sample data).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimChannel {
    pub target: AnimTarget,
    #[serde(default)]
    pub interpolation: AnimInterpolation,
    #[serde(default)]
    pub keyframes: Vec<AnimKeyframe>,
}
//#endregion 🔖️Channel

//#region 🔖️Timeline
/// 🎬️ One gltf `animation` entry — an optional display `name` (gltf's own `animation.name` is
/// optional and not spec-required to be unique, hence `Option<String>` rather than a name key) plus
/// its ordered `channels`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimTimeline {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub channels: Vec<AnimChannel>,
}
//#endregion 🔖️Timeline

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.animation")]
pub struct SemioAnimationSnapshot {
    #[state(persistent)]
    pub schema: String,
    #[state(persistent)]
    #[serde(default)]
    pub timelines: Vec<AnimTimeline>,
}

impl Default for SemioAnimationSnapshot {
    fn default() -> Self {
        Self {
            schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(),
            timelines: Default::default(),
        }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs
/// 📦️ JSON-pack envelope round trip — honest, genuinely working: `SemioAnimationSnapshot` is a
/// NEUTRAL semio type (not itself an on-disk file format like gif/gltf's own bytes), so unlike
/// those artifacts' `ArtifactDsl`/`ArtifactPack` (which call into a real per-format binary codec),
/// this envelope IS the correct, final representation — not a placeholder awaiting a "real" codec.
/// The `🔺️diff`/`🧬️mutations` facets carry the hand-rolled per-field grammars instead (see those
/// modules), matching the recipe's own framing: snapshots serialize whole, diffs/ops are sparse.
impl store::ArtifactDsl for SemioAnimationSnapshot {
    const EXTENSION: &'static str = "semio";
    fn envelope_id() -> &'static str { STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA }

    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let hex: String = body.chars().filter(|c| !c.is_whitespace()).collect();
        if hex.len() % 2 != 0 {
            return Err(store::TextError::new("odd hex length", dsl::TextSpan::at(1, 1)));
        }
        let mut bytes = Vec::with_capacity(hex.len() / 2);
        let mut i = 0usize;
        while i < hex.len() {
            let byte = u8::from_str_radix(&hex[i..i + 2], 16)
                .map_err(|e| store::TextError::new(format!("invalid hex: {e}"), dsl::TextSpan::at(1, 1)))?;
            bytes.push(byte);
            i += 2;
        }
        serde_json::from_slice(&bytes).map_err(|e| store::TextError::new(format!("json decode: {e}"), dsl::TextSpan::at(1, 1)))
    }

    fn print_dsl(&self) -> String {
        let bytes = serde_json::to_vec(self).unwrap_or_default();
        let body: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for SemioAnimationSnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        let raw = serde_json::to_vec(self).map_err(|e| store::PackError::Schema(e.to_string()))?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::ArtifactDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &raw))
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::ArtifactDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::ArtifactDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let _ = options;
        serde_json::from_slice(&inner).map_err(|e| store::PackError::Schema(e.to_string()))
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn full_snapshot() -> SemioAnimationSnapshot {
        SemioAnimationSnapshot {
            schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(),
            timelines: vec![AnimTimeline {
                name: Some("walk".into()),
                channels: vec![
                    AnimChannel {
                        target: AnimTarget { node: "hip".into(), property: AnimTargetProperty::Translation },
                        interpolation: AnimInterpolation::Linear,
                        keyframes: vec![
                            AnimKeyframe { t: 0.0, value: AnimValue::Vec3 { value: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 } } },
                            AnimKeyframe { t: 1.0, value: AnimValue::Vec3 { value: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } } },
                        ],
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

    /// 🧪️ codec_retention_law: decode(encode(x)) == x through both the pack (binary) and dsl
    /// (text) envelopes, on a snapshot exercising every `AnimValue` variant and both `AnimTarget`
    /// property kinds (incl. `Custom`).
    #[test]
    fn codec_retention_law() {
        let snap = full_snapshot();
        let bytes = <SemioAnimationSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);

        let text = <SemioAnimationSnapshot as store::ArtifactDsl>::print_dsl(&snap);
        let back_text = <SemioAnimationSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(snap, back_text);
    }

    #[test]
    fn default_snapshot_round_trips() {
        let snap = SemioAnimationSnapshot::default();
        let bytes = <SemioAnimationSnapshot as store::ArtifactPack>::encode_pack(&snap);
        let back = <SemioAnimationSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(snap, back);
    }
}
//#endregion 🔖️Tests
