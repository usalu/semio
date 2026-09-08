
use super::*;
use crate::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimKeyframe, AnimTarget, AnimTimeline, STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn keyframe(t: f64) -> AnimKeyframe {
    AnimKeyframe { t, value: Default::default() }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn channel(keyframes: Vec<AnimKeyframe>) -> AnimChannel {
    AnimChannel { target: AnimTarget { node: "n".into(), property: Default::default() }, interpolation: Default::default(), keyframes }
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn snapshot(timelines: Vec<AnimTimeline>) -> SemioAnimationSnapshot {
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
