//! 💾️ Binary representation codec surface for `stdio.semio.text` (snapshot) — protocol include.
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
pub(crate) fn mark_kind_tag(k: SemioTextMarkKind) -> u8 {
    match k {
        SemioTextMarkKind::Bold => 0,
        SemioTextMarkKind::Italic => 1,
        SemioTextMarkKind::Code => 2,
        SemioTextMarkKind::Link => 3,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn mark_kind_from_tag(tag: u8) -> Result<SemioTextMarkKind, String> {
    match tag {
        0 => Ok(SemioTextMarkKind::Bold),
        1 => Ok(SemioTextMarkKind::Italic),
        2 => Ok(SemioTextMarkKind::Code),
        3 => Ok(SemioTextMarkKind::Link),
        other => Err(format!("unsupported mark kind tag {other}")),
    }
}

semio_s_artifact_stdio_semio_base::semio_snapshot_record_pack!(SemioTextSnapshot);
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SemioTextSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}
}
pub use snapshot_codec::*;

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use framework_schema::ArtifactSchema;
use crate::io::text::snapshot::*;
/// 📦️ Encodes a [`SemioTextSnapshot`] as a semio pack envelope — the binary twin of the DSL text, produced by a
/// SEPARATE codec, which is what makes the two committed encodings of one document able to
/// contradict each other.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_text_pack(snapshot: &SemioTextSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(snapshot)
}
/// 📦️ Decodes a semio pack envelope into a [`SemioTextSnapshot`] — the inverse of
/// [`encode_semio_text_pack`], reading `../../🖼️assets/📃️note/🎒️.pack.semio`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_text_pack(bytes: &[u8]) -> Result<SemioTextSnapshot, String> {
    <SemioTextSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
}
pub use native_snapshot_codec::*;
