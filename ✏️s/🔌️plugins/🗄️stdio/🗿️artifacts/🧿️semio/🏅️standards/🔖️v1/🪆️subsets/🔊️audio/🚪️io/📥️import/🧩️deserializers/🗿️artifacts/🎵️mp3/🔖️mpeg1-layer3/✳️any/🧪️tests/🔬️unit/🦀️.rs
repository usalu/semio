use super::*;
use semio_s_artifact_stdio_mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::{Id3Frame, Id3v1Tag, Id3v2Tag, Mp3Frame, Mp3FrameHeader};

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn real_world_mp3() -> Mp3Snapshot {
    Mp3Snapshot {
        schema: "stdio.mp3".into(),
        id3v2: Some(Id3v2Tag {
            major_version: 3,
            minor_version: 0,
            flags: 0,
            frames: vec![Id3Frame {
                id: "TIT2".into(),
                flags: 0,
                data: {
                    let mut d = vec![0u8]; // ISO-8859-1
                    d.extend_from_slice(b"Test Tone");
                    d
                },
            }],
        }),
        frames: vec![Mp3Frame {
            header: Mp3FrameHeader { mpeg_version_id: 3, layer: 1, protection_bit: true, bitrate_index: 9, sample_rate_index: 0, padding: false, private_bit: false, channel_mode: 0, mode_extension: 0, copyright: false, original: true, emphasis: 0 },
            payload: vec![0u8; 100],
        }],
        id3v1: Some(Id3v1Tag { raw: vec![0u8; 128] }),
    }
}

#[semio_framework_async_macros::async_test]
async fn deserialize_derives_real_sample_rate_and_channel_count_leaves_samples_empty() {
    let audio = semio_framework_plugin::resolve_ready(SemioAudioFromMp3::deserialize(&real_world_mp3())).expect("deserialize");
    assert_eq!(audio.sample_rate, 44_100); // MPEG1, index 0
    assert_eq!(audio.channels.len(), 2); // channel_mode 0 = stereo
    for ch in &audio.channels {
        assert!(ch.samples.is_empty(), "mp3 payload is opaque -- no fabricated samples");
    }
}

#[semio_framework_async_macros::async_test]
async fn deserialize_carries_real_id3v2_title_and_id3v1_presence_as_tags() {
    let audio = semio_framework_plugin::resolve_ready(SemioAudioFromMp3::deserialize(&real_world_mp3())).expect("deserialize");
    assert!(audio.tags.iter().any(|t| t.key == "TIT2" && t.value == "Test Tone"));
    assert!(audio.tags.iter().any(|t| t.key == "id3v1.raw"));
}

#[semio_framework_async_macros::async_test]
async fn mono_channel_mode_maps_to_a_single_channel() {
    let mut mp3 = real_world_mp3();
    mp3.frames[0].header.channel_mode = 3;
    let audio = semio_framework_plugin::resolve_ready(SemioAudioFromMp3::deserialize(&mp3)).expect("deserialize");
    assert_eq!(audio.channels.len(), 1);
}

#[semio_framework_async_macros::async_test]
async fn no_frames_honestly_yields_zero_sample_rate_and_zero_channels() {
    let mp3 = Mp3Snapshot::default();
    let audio = semio_framework_plugin::resolve_ready(SemioAudioFromMp3::deserialize(&mp3)).expect("deserialize");
    assert_eq!(audio.sample_rate, 0);
    assert!(audio.channels.is_empty());
}
