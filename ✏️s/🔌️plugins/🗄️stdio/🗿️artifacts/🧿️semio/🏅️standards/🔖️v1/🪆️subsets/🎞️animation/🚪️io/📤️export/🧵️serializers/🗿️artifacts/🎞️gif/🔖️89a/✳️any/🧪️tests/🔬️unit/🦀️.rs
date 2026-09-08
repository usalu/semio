
use super::*;
use crate::standards::v1::subsets::animation::schema::snapshot::{AnimChannel, AnimInterpolation, AnimKeyframe, AnimTarget, AnimTargetProperty, AnimTimeline, AnimValue, STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
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

#[semio_framework_async_macros::async_test]
async fn serialize_derives_real_delay_from_keyframe_time_deltas() {
    let gif = semio_framework_plugin::resolve_ready(SemioAnimationToGif::serialize(&real_world_animation())).expect("serialize");
    assert_eq!(gif.frames.len(), 3);
    assert_eq!(gif.frames[0].delay_cs, 10);
    assert_eq!(gif.frames[1].delay_cs, 20);
    assert_eq!(gif.frames[2].delay_cs, 20); // last frame reuses prior delay
    assert!(gif.frames.iter().all(|f| f.indices.is_empty()), "no fabricated pixel data");
}

#[semio_framework_async_macros::async_test]
async fn single_keyframe_uses_the_minimum_one_centisecond_floor() {
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
    let gif = semio_framework_plugin::resolve_ready(SemioAnimationToGif::serialize(&snap)).expect("serialize");
    assert_eq!(gif.frames.len(), 1);
    assert_eq!(gif.frames[0].delay_cs, 1);
}

#[semio_framework_async_macros::async_test]
async fn empty_animation_serializes_to_zero_frames() {
    let snap = SemioAnimationSnapshot { schema: STDIO_SEMIOANIMATION_DOCUMENT_SCHEMA.into(), timelines: Vec::new() };
    let gif = semio_framework_plugin::resolve_ready(SemioAnimationToGif::serialize(&snap)).expect("serialize");
    assert!(gif.frames.is_empty());
}
