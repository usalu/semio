//! 💾️ Binary representation grammar surface for `stdio.semio.video` (snapshot): real
//! varint-length-prefixed binary pack frame — `format u8` + `schema` (length-prefixed UTF-8) real
//! and fully described, `streams` an opaque trailing payload (`protocol-array-of-records` gap;
//! video wave, replacing the old `serde_json::to_vec`-in-envelope shortcut) — actual encode/decode
//! lives on `SemioVideoSnapshot`'s `pack::snapshot::ArtifactPack` impl in the facet root
//! `🦀️.rs`; this leaf carries the normative protocol description.

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
pub(crate) fn kind_tag(k: SemioVideoStreamKind) -> u8 {
    match k {
        SemioVideoStreamKind::Video => 0,
        SemioVideoStreamKind::Audio => 1,
        SemioVideoStreamKind::Subtitle => 2,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn kind_from_tag(t: u8) -> Result<SemioVideoStreamKind, String> {
    match t {
        0 => Ok(SemioVideoStreamKind::Video),
        1 => Ok(SemioVideoStreamKind::Audio),
        2 => Ok(SemioVideoStreamKind::Subtitle),
        other => Err(format!("stream kind: bad tag {other}")),
    }
}

semio_s_artifact_stdio_semio_base::semio_snapshot_record_pack!(SemioVideoSnapshot);
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SemioVideoSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}
}
pub use snapshot_codec::*;
