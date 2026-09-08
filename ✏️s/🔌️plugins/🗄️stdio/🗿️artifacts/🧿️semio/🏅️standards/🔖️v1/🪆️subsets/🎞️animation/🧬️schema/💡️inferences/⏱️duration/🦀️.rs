//! ⏱ `duration` — one named inference: the semio animation snapshot's real playback length,
//! derived from the LATEST keyframe `t` found anywhere across every `timelines[].channels[]`
//! (gltf-style: a clip's duration is bounded by its slowest-ending channel, matching how a real
//! player would compute clip length). A pure whole-snapshot scalar (one max-`t` fold) — no
//! `InferredField` needed.

use crate::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;

//#region 🔖️Duration
/// ⏱️ Semio animation's keyframe-derived playback duration.
#[derive(Clone, Copy, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioAnimationDuration {
    pub duration_seconds: f64,
    pub timeline_count: u32,
    pub channel_count: u32,
    pub keyframe_count: u32,
}

/// ⏱️ Computes [`SemioAnimationDuration`] — `duration_seconds` is the maximum `t` across every
/// keyframe of every channel of every timeline (`0.0` for no keyframes, an honest degenerate
/// case, not a panic); `channel_count`/`keyframe_count` are real sums across all timelines.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn compute_semio_animation_duration(snapshot: &SemioAnimationSnapshot) -> SemioAnimationDuration {
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
