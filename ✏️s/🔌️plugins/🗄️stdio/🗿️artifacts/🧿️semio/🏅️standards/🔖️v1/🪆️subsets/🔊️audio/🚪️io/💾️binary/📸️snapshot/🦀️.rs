//! 💾️ Binary representation codec surface for `s.stdio.semio.audio` (snapshot).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use framework_schema::ArtifactSchema;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn format_tag(f: SemioAudioFormat) -> u8 {
    match f {
        SemioAudioFormat::Pcm8 => 0,
        SemioAudioFormat::Pcm16 => 1,
        SemioAudioFormat::Pcm24 => 2,
        SemioAudioFormat::Pcm32 => 3,
        SemioAudioFormat::Float32 => 4,
        SemioAudioFormat::Float64 => 5,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn format_from_tag(tag: u8) -> Result<SemioAudioFormat, String> {
    match tag {
        0 => Ok(SemioAudioFormat::Pcm8),
        1 => Ok(SemioAudioFormat::Pcm16),
        2 => Ok(SemioAudioFormat::Pcm24),
        3 => Ok(SemioAudioFormat::Pcm32),
        4 => Ok(SemioAudioFormat::Float32),
        5 => Ok(SemioAudioFormat::Float64),
        other => Err(format!("unsupported audio format tag {other}")),
    }
}

semio_s_artifact_stdio_semio_base::semio_snapshot_record_pack!(SemioAudioSnapshot);
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SemioAudioSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}
}
pub use snapshot_codec::*;
