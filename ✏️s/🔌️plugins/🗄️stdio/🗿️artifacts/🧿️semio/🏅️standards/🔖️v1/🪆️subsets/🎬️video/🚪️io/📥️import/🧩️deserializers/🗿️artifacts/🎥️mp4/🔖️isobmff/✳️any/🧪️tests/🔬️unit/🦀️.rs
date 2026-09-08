
use super::*;
use semio_s_artifact_stdio_mp4::standards::isobmff::subsets::any::schema::snapshot::{Mp4Codec, Mp4Ftyp, Mp4Sample, Mp4Track};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn real_world_mp4() -> Mp4Snapshot {
    Mp4Snapshot {
        schema: "stdio.mp4".into(),
        ftyp: Mp4Ftyp { major_brand: "isom".into(), minor_version: 0, compatible_brands: vec!["isom".into(), "mp41".into()] },
        movie: Default::default(),
        tracks: vec![Mp4Track {
            track_id: 1,
            timescale: 30,
            codec: Mp4Codec::default(),
            width: 1920,
            height: 1080,
            metadata: Default::default(),
            chunk_sample_counts: vec![3],
            samples: vec![
                Mp4Sample { data: vec![0xAA, 0xBB], duration: 1, cts_offset: 0, sync: true },
                Mp4Sample { data: vec![0xCC], duration: 1, cts_offset: 1, sync: false },
                Mp4Sample { data: vec![0xDD, 0xEE, 0xFF], duration: 1, cts_offset: 0, sync: false },
            ],
        }],
    }
}

#[semio_framework_async_macros::async_test]
async fn deserialize_maps_real_track_metadata_and_derives_pts_from_duration_plus_cts_offset() {
    let video = semio_framework_plugin::resolve_ready(SemioVideoFromMp4::deserialize(&real_world_mp4())).expect("deserialize");
    assert_eq!(video.streams.len(), 1);
    let stream = &video.streams[0];
    assert_eq!(stream.kind, SemioVideoStreamKind::Video);
    assert_eq!(stream.codec, "avc1");
    assert_eq!(stream.width, 1920);
    assert_eq!(stream.height, 1080);
    assert_eq!(stream.rate, SemioRational { num: 30, den: 1 });
    assert_eq!(stream.samples.len(), 3);
    // dts: 0, 1, 2 -- pts = dts + cts_offset
    assert_eq!(stream.samples[0].pts, 0);
    assert_eq!(stream.samples[1].pts, 2); // dts=1, cts_offset=1
    assert_eq!(stream.samples[2].pts, 2); // dts=2, cts_offset=0
    assert!(stream.samples[0].key);
    assert!(!stream.samples[1].key);
    assert_eq!(stream.samples[2].data, vec![0xDD, 0xEE, 0xFF]);
}

#[semio_framework_async_macros::async_test]
async fn deserialize_of_track_with_no_samples_falls_back_to_unit_rate_denominator() {
    let mut mp4 = real_world_mp4();
    mp4.tracks[0].samples.clear();
    let video = semio_framework_plugin::resolve_ready(SemioVideoFromMp4::deserialize(&mp4)).expect("deserialize");
    assert_eq!(video.streams[0].rate, SemioRational { num: 30, den: 1 });
    assert!(video.streams[0].samples.is_empty());
}
