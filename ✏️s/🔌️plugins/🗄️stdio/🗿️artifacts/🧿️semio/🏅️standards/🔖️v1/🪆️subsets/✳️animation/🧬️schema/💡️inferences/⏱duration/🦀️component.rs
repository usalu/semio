//! ⏱ `duration` — one named inference: the semio animation snapshot's real playback length,
//! derived from the LATEST keyframe `t` found anywhere across every `timelines[].channels[]`
//! (gltf-style: a clip's duration is bounded by its slowest-ending channel, matching how a real
//! player would compute clip length). A pure whole-snapshot scalar (one max-`t` fold) — no
//! `InferredField` needed.

use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️Duration
/// ⏱️ Semio animation's keyframe-derived playback duration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioAnimationDuration {
    pub duration_seconds: f64,
    pub timeline_count: u32,
    pub channel_count: u32,
    pub keyframe_count: u32,
}

/// ⏱️ Computes [`SemioAnimationDuration`] — `duration_seconds` is the maximum `t` across every
/// keyframe of every channel of every timeline (`0.0` for no keyframes, an honest degenerate
/// case, not a panic); `channel_count`/`keyframe_count` are real sums across all timelines.
pub async fn compute_semio_animation_duration(snapshot: &SemioAnimationSnapshot) -> SemioAnimationDuration {
    let mut duration_seconds = 0.0f64;
    let mut channel_count = 0u32;
    let mut keyframe_count = 0u32;
    for timeline in &snapshot.timelines {
        channel_count += timeline.channels.len() as u32;
        for channel in &timeline.channels {
            keyframe_count += channel.keyframes.len() as u32;
            for keyframe in &channel.keyframes {
                if keyframe.t > duration_seconds {
                    duration_seconds = keyframe.t;
                }
            }
        }
    }
    SemioAnimationDuration { duration_seconds, timeline_count: snapshot.timelines.len() as u32, channel_count, keyframe_count }
}
//#endregion 🔖️Duration

#[cfg(test)]
//#region 🧪️Tests
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimKeyframe, AnimTarget, AnimTimeline, STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA};

    async fn keyframe(t: f64) -> AnimKeyframe {
        AnimKeyframe { t, value: Default::default() }
    }

    async fn channel(keyframes: Vec<AnimKeyframe>) -> AnimChannel {
        AnimChannel { target: AnimTarget { node: "n".into(), property: Default::default() }, interpolation: Default::default(), keyframes }
    }

    async fn snapshot(timelines: Vec<AnimTimeline>) -> SemioAnimationSnapshot {
        SemioAnimationSnapshot { schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(), timelines }
    }

    #[semio_framework_async_macros::async_test]
    async fn duration_is_the_latest_keyframe_across_every_channel() {
        let snap = snapshot(vec![AnimTimeline { name: None, channels: vec![channel(vec![keyframe(0.0), keyframe(1.5)])] }, AnimTimeline { name: None, channels: vec![channel(vec![keyframe(0.0), keyframe(3.25)]), channel(vec![keyframe(2.0)])] }]);
        let duration = compute_semio_animation_duration(&snap);
        assert_eq!(duration, SemioAnimationDuration { duration_seconds: 3.25, timeline_count: 2, channel_count: 3, keyframe_count: 5 });
    }

    #[semio_framework_async_macros::async_test]
    async fn no_keyframes_yields_zero_duration() {
        let snap = snapshot(vec![AnimTimeline { name: None, channels: vec![channel(Vec::new())] }]);
        assert_eq!(compute_semio_animation_duration(&snap).duration_seconds, 0.0);
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        let snap = snapshot(vec![AnimTimeline { name: None, channels: vec![channel(vec![keyframe(1.0)])] }]);
        assert_eq!(compute_semio_animation_duration(&snap), compute_semio_animation_duration(&snap));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        assert_eq!(compute_semio_animation_duration(&SemioAnimationSnapshot::default()), SemioAnimationDuration::default());
    }
}
//#endregion 🧪️Tests
