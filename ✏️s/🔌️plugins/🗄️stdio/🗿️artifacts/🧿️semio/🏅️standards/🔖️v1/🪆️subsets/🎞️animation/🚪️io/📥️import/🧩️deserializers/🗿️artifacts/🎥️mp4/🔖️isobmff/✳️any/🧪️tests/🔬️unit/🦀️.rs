use super::*;
use semio_s_artifact_stdio_mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Sample, Mp4Track};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn real_world_mp4() -> Mp4Snapshot {
    Mp4Snapshot {
        schema: "stdio.mp4".into(),
        ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: vec![] },
        movie: Default::default(),
        tracks: vec![Mp4Track {
            track_id: 1,
            timescale: 30,
            codec: Mp4Codec::default(),
            width: 640,
            height: 480,
            metadata: Default::default(),
            chunk_sample_counts: vec![2],
            samples: vec![Mp4Sample { data: vec![1], duration: 30, cts_offset: 0, sync: true }, Mp4Sample { data: vec![2], duration: 30, cts_offset: 0, sync: false }],
        }],
    }
}

#[semio_framework_async_macros::async_test]
async fn deserialize_exposes_real_sample_timing_as_a_step_scalar_channel() {
    let anim = semio_framework_plugin::resolve_ready(SemioAnimationFromMp4::deserialize(&real_world_mp4())).expect("deserialize");
    assert_eq!(anim.timelines.len(), 1);
    assert_eq!(anim.timelines[0].name.as_deref(), Some("track-1"));
    let ch = &anim.timelines[0].channels[0];
    assert_eq!(ch.target.node, "track-1");
    assert_eq!(ch.interpolation, AnimInterpolation::Step);
    assert_eq!(ch.keyframes.len(), 2);
    assert_eq!(ch.keyframes[0].t, 0.0); // dts=0 / timescale 30
    assert_eq!(ch.keyframes[1].t, 1.0); // dts=30 / timescale 30 = 1s
    assert_eq!(ch.keyframes[0].value, AnimValue::Scalar { value: 0.0 });
    assert_eq!(ch.keyframes[1].value, AnimValue::Scalar { value: 1.0 });
}
