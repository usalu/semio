//! 💾️ Binary representation codec surface for `stdio.semio.presentation` (snapshot).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use semio_s_artifact_stdio_semio_document::schema::snapshot::DocBlock;
/// 🧱️ REUSE, don't reinvent — the sibling `🔺️diff` facet re-exports document's own real, already-
/// tested `DocBlock` codec (`enc_block`/`dec_block`) plus the entity value-codecs it owns
/// (`enc_master`/`enc_layout`/`enc_slide`, `enc_str`, `enc_list`) — this facet imports them rather
/// than duplicating a third independent copy (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-
/// EVOLUTION presentation wave, following `document`'s own snapshot-imports-from-diff convention).
use crate::io::text::diff::dec_slide;
use crate::io::text::diff::enc_slide;
use crate::io::text::diff::dec_layout;
use crate::io::text::diff::enc_layout;
use crate::io::text::diff::dec_master;
use crate::io::text::diff::enc_master;
use semio_s_artifact_stdio_semio_document::io::text::snapshot::dec_block;
use semio_s_artifact_stdio_semio_document::io::text::snapshot::enc_block;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_str;
use framework_schema::ArtifactSchema;

semio_s_artifact_stdio_semio_base::semio_snapshot_record_pack!(SemioPresentationSnapshot);
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SemioPresentationSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> { Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec()) }
}
}

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_document::schema::snapshot::DocBlock;
/// 🧱️ REUSE, don't reinvent — the sibling `🔺️diff` facet re-exports document's own real, already-
/// tested `DocBlock` codec (`enc_block`/`dec_block`) plus the entity value-codecs it owns
/// (`enc_master`/`enc_layout`/`enc_slide`, `enc_str`, `enc_list`) — this facet imports them rather
/// than duplicating a third independent copy (ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-
/// EVOLUTION presentation wave, following `document`'s own snapshot-imports-from-diff convention).
use crate::io::text::diff::dec_slide;
use crate::io::text::diff::enc_slide;
use crate::io::text::diff::dec_layout;
use crate::io::text::diff::enc_layout;
use crate::io::text::diff::dec_master;
use crate::io::text::diff::enc_master;
use semio_s_artifact_stdio_semio_document::io::text::snapshot::dec_block;
use semio_s_artifact_stdio_semio_document::io::text::snapshot::enc_block;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_str;
use framework_schema::ArtifactSchema;
use crate::io::text::snapshot::*;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_presentation_pack(snapshot: &SemioPresentationSnapshot) -> Vec<u8> {
    <SemioPresentationSnapshot as pack::snapshot::ArtifactPack>::encode_pack(snapshot)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_presentation_pack(bytes: &[u8]) -> Result<SemioPresentationSnapshot, String> {
    <SemioPresentationSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|e| e.to_string())
}
}
pub use native_snapshot_codec::*;
