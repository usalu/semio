//! 🧪️ The semio video per-mutation inverse laws, over the mutation crate's public API.

extern crate semio_framework_os_kernel as protocol;

use semio_s_artifact_stdio_semio_video_model::schema::snapshot::{SemioRational, SemioVideoSample, SemioVideoSnapshot, SemioVideoStream, SemioVideoStreamKind, STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA};

pub(crate) fn fixture() -> SemioVideoSnapshot {
    SemioVideoSnapshot {
        schema: STDIO_SEMIOVIDEO_DOCUMENT_SCHEMA.into(),
        streams: vec![
            SemioVideoStream {
                kind: SemioVideoStreamKind::Video,
                codec: "h264".into(),
                width: 1920,
                height: 1080,
                rate: SemioRational { num: 30, den: 1 },
                samples: vec![SemioVideoSample { pts: 0, key: true, data: vec![1, 2, 3] }, SemioVideoSample { pts: 33, key: false, data: vec![4, 5, 6] }],
            },
            SemioVideoStream { kind: SemioVideoStreamKind::Audio, codec: "aac".into(), width: 0, height: 0, rate: SemioRational { num: 48_000, den: 1_000 }, samples: Vec::new() },
        ],
    }
}

#[path = "../../➕️insert-sample/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_insert_sample;
#[path = "../../🎥insert-stream/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_insert_stream;
#[path = "../../📀set-sample-data/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_set_sample_data;
#[path = "../../📋set-stream-meta/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_set_stream_meta;
#[path = "../../🗑️remove-stream/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_remove_stream;
#[path = "../../🚩set-sample-flags/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_set_sample_flags;
#[path = "../../🚮remove-sample/🧪️tests/↩️inverts/🦀️.rs"]
mod inverts_remove_sample;
