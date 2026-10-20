//! 🧪️ The semio audio per-mutation inverse laws, over the mutation crate's public API.

extern crate semio_framework_os_kernel as protocol;

use semio_s_artifact_stdio_semio_audio_model::schema::snapshot::{SemioAudioChannel, SemioAudioFormat, SemioAudioSnapshot, SemioAudioTag};

fn sample_channel(seed: f32) -> SemioAudioChannel {
    SemioAudioChannel { samples: vec![seed, seed + 1.0, seed + 2.0] }
}

pub(crate) fn base_snapshot() -> SemioAudioSnapshot {
    SemioAudioSnapshot {
        sample_rate: 44_100,
        format: SemioAudioFormat::Pcm16,
        channels: vec![sample_channel(1.0), sample_channel(2.0), sample_channel(3.0)],
        tags: vec![SemioAudioTag { key: "title".into(), value: "t0".into() }],
        ..SemioAudioSnapshot::default()
    }
}

#[path = "../../✂️remove-tag/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_remove_tag;
#[path = "../../🌊set-channel-samples/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_set_channel_samples;
#[path = "../../🎙️insert-channel/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_insert_channel;
#[path = "../../🎚️set-sample-rate/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_set_sample_rate;
#[path = "../../🏷️insert-tag/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_insert_tag;
#[path = "../../💬set-tag-value/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_set_tag_value;
#[path = "../../💽set-format/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_set_format;
#[path = "../../🔇remove-channel/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_remove_channel;
