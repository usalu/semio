//! 📥️ Deserialize `s.stdio.gif` (89a/✳️any) into `s.stdio.semio` (v1/animation) — frame-delay
//! sequences map reasonably onto simple keyframe timelines (master plan: "real but approximate,
//! document the mapping choice"). ONE `AnimTimeline` (unnamed), ONE channel targeting a synthetic
//! node `"gif-frame"` with `property: Custom{name:"frameIndex"}`, `interpolation: Step` (a GIF
//! frame displays until its GCE `delay_cs` elapses, then switches -- exactly `Step` semantics, not
//! interpolated). `t` is the REAL cumulative delay in seconds (`sum(delay_cs[0..i]) / 100.0`); the
//! keyframe `value` is the frame's own index (`Scalar`).
//!
//! Honest, documented lossy fields: `gct`/`frames[i].{lct,indices,disposal,transparent_index,
//! user_input,plain_text}`, `loop_count`, `comments`, `app_extensions` -- ALL pixel/palette/replay
//! metadata stays entirely inside gif's own domain; `animation`'s model has no pixel/palette/replay
//! concept at all (by design, per the master plan's subset recipe), so none of it is fabricated
//! into a fake transform channel here.

use crate::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTargetProperty, AnimTimeline, AnimValue, SemioAnimationSnapshot, STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeserializer, Dialect, StandardId, SubsetId};
use semio_s_artifact_stdio_gif::GifSnapshot;

const FROM_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("89a"), subset: SubsetId("*") };
const INTO_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("animation") };

/// 🏷️ Synthetic node name every gif↔animation channel targets — a GIF has exactly one "thing" that
/// animates (its own frame sequence), never a named node graph, so a single fixed name is the
/// honest choice (matching `video`'s own honest-boundary framing, not a fabricated scene node).
pub const GIF_FRAME_NODE: &str = "gif-frame";

pub struct SemioAnimationFromGif;

impl ArtifactDeserializer for SemioAnimationFromGif {
    type From = GifSnapshot;
    type Into = SemioAnimationSnapshot;
    const FROM: Dialect = FROM_DIALECT;
    const INTO: Dialect = INTO_DIALECT;

    async fn deserialize(from: &Self::From) -> Result<Self::Into, store::PackError> {
        let mut cumulative_cs: u32 = 0;
        let keyframes = from
            .frames
            .iter()
            .enumerate()
            .map(|(i, frame)| {
                let t = cumulative_cs as f64 / 100.0;
                cumulative_cs += frame.delay_cs as u32;
                AnimKeyframe { t, value: AnimValue::Scalar { value: i as f64 } }
            })
            .collect::<Vec<_>>();
        let timelines = if keyframes.is_empty() {
            Vec::new()
        } else {
            vec![AnimTimeline { name: None, channels: vec![AnimChannel { target: AnimTarget { node: GIF_FRAME_NODE.into(), property: AnimTargetProperty::Custom { name: "frameIndex".into() } }, interpolation: AnimInterpolation::Step, keyframes }] }]
        };
        Ok(SemioAnimationSnapshot { schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(), timelines })
    }
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
