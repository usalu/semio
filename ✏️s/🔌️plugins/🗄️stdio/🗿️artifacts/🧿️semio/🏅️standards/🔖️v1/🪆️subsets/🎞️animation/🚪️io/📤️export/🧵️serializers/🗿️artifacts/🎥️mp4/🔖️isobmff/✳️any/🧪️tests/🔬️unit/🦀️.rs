use super::*;
use crate::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTargetProperty, AnimTimeline, AnimValue, STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn real_world_animation() -> SemioAnimationSnapshot {
    SemioAnimationSnapshot {
        schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(),
        timelines: vec![AnimTimeline {
            name: Some("clip".into()),
            channels: vec![AnimChannel {
                target: AnimTarget { node: "n".into(), property: AnimTargetProperty::Translation },
                interpolation: AnimInterpolation::Linear,
                keyframes: vec![AnimKeyframe { t: 0.0, value: AnimValue::Scalar { value: 0.0 } }, AnimKeyframe { t: 0.5, value: AnimValue::Scalar { value: 1.0 } }, AnimKeyframe { t: 1.0, value: AnimValue::Scalar { value: 2.0 } }],
            }],
        }],
    }
}

#[semio_framework_async_macros::async_test]
async fn serialize_builds_one_synthetic_track_with_real_derived_durations() {
    let mp4 = semio_framework_plugin::resolve_ready(SemioAnimationToMp4::serialize(&real_world_animation())).expect("serialize");
    assert_eq!(mp4.tracks.len(), 1);
    assert_eq!(mp4.tracks[0].timescale, SYNTHETIC_TIMESCALE);
    assert_eq!(mp4.tracks[0].samples.len(), 3);
    assert_eq!(mp4.tracks[0].samples[0].duration, 500); // 0.5s * 1000
    assert_eq!(mp4.tracks[0].samples[1].duration, 500);
    assert!(mp4.tracks[0].samples.iter().all(|s| s.data.is_empty()), "no fabricated frame bytes");
}

#[semio_framework_async_macros::async_test]
async fn empty_animation_serializes_to_zero_tracks() {
    let snap = SemioAnimationSnapshot { schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(), timelines: Vec::new() };
    let mp4 = semio_framework_plugin::resolve_ready(SemioAnimationToMp4::serialize(&snap)).expect("serialize");
    assert!(mp4.tracks.is_empty());
}
