//! 📤️ Serialize `s.stdio.semio` (v1/animation) into `s.stdio.gif` (89a/✳️any) — mirror of this
//! pair's deserializer, same "real but approximate" framing the master plan calls for. Uses ONLY
//! the FIRST timeline's FIRST channel's `keyframes` (in order) as the master frame sequence -- the
//! same documented cardinality-reduction choice `animation↔mp4` makes, since GIF (like mp4) is
//! fundamentally a single frame-sequence container, not a multi-timeline/multi-channel model.
//!
//! Each keyframe becomes one `GifFrame` whose `delay_cs` is the REAL derived gap to the next
//! keyframe's `t` (`round((next.t - t) * 100)`, clamped to GIF's minimum meaningful `1` centisecond
//! -- the spec permits `0` but real decoders commonly treat it as "as fast as possible", not
//! "instant", so `1` is the honest floor); the LAST keyframe reuses the prior frame's own delay (or
//! `1` if it is the only frame). `width`/`height` are `0` and `indices` is empty on every produced
//! frame -- `animation` carries no pixel/palette data at all (see the deserializer's own doc
//! comment), so this never fabricates image content, only real, honest frame TIMING.

use crate::artifacts::gif::schema::snapshot::GifFrame;
use crate::artifacts::gif::GifSnapshot;
use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use semio_framework_plugin::{ArtifactSerializer, Dialect, StandardId, SubsetId};

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("animation") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("89a"), subset: SubsetId("*") };

pub struct SemioAnimationToGif;

impl ArtifactSerializer for SemioAnimationToGif {
    type From = SemioAnimationSnapshot;
    type Into = GifSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    fn serialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let keyframes = from.timelines.first().and_then(|t| t.channels.first()).map(|c| c.keyframes.as_slice()).unwrap_or(&[]);
        let mut frames = Vec::with_capacity(keyframes.len());
        let mut last_delay: u16 = 1;
        for (i, k) in keyframes.iter().enumerate() {
            let delay_cs = match keyframes.get(i + 1) {
                Some(next) => {
                    let d = ((next.t - k.t) * 100.0).round().max(1.0) as u16;
                    last_delay = d;
                    d
                }
                None => last_delay,
            };
            frames.push(GifFrame { delay_cs, ..GifFrame::default() });
        }
        Ok(GifSnapshot { schema: "stdio.gif.89a".into(), width: 0, height: 0, gct: None, background_color_index: 0, pixel_aspect_ratio: 0, loop_count: None, frames, comments: Vec::new(), app_extensions: Vec::new() })
    }
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTargetProperty, AnimTimeline, AnimValue, STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA};

    fn real_world_animation() -> SemioAnimationSnapshot {
        SemioAnimationSnapshot {
            schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(),
            timelines: vec![AnimTimeline {
                name: None,
                channels: vec![AnimChannel {
                    target: AnimTarget { node: "gif-frame".into(), property: AnimTargetProperty::Custom { name: "frameIndex".into() } },
                    interpolation: AnimInterpolation::Step,
                    keyframes: vec![AnimKeyframe { t: 0.0, value: AnimValue::Scalar { value: 0.0 } }, AnimKeyframe { t: 0.10, value: AnimValue::Scalar { value: 1.0 } }, AnimKeyframe { t: 0.30, value: AnimValue::Scalar { value: 2.0 } }],
                }],
            }],
        }
    }

    #[test]
    fn serialize_derives_real_delay_from_keyframe_time_deltas() {
        let gif = SemioAnimationToGif::serialize(&real_world_animation()).expect("serialize");
        assert_eq!(gif.frames.len(), 3);
        assert_eq!(gif.frames[0].delay_cs, 10);
        assert_eq!(gif.frames[1].delay_cs, 20);
        assert_eq!(gif.frames[2].delay_cs, 20); // last frame reuses prior delay
        assert!(gif.frames.iter().all(|f| f.indices.is_empty()), "no fabricated pixel data");
    }

    #[test]
    fn single_keyframe_uses_the_minimum_one_centisecond_floor() {
        let snap = SemioAnimationSnapshot {
            schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(),
            timelines: vec![AnimTimeline {
                name: None,
                channels: vec![AnimChannel {
                    target: AnimTarget { node: "gif-frame".into(), property: AnimTargetProperty::Custom { name: "frameIndex".into() } },
                    interpolation: AnimInterpolation::Step,
                    keyframes: vec![AnimKeyframe { t: 0.0, value: AnimValue::Scalar { value: 0.0 } }],
                }],
            }],
        };
        let gif = SemioAnimationToGif::serialize(&snap).expect("serialize");
        assert_eq!(gif.frames.len(), 1);
        assert_eq!(gif.frames[0].delay_cs, 1);
    }

    #[test]
    fn empty_animation_serializes_to_zero_frames() {
        let snap = SemioAnimationSnapshot { schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(), timelines: Vec::new() };
        let gif = SemioAnimationToGif::serialize(&snap).expect("serialize");
        assert!(gif.frames.is_empty());
    }
}
//#endregion 🔖️Tests
