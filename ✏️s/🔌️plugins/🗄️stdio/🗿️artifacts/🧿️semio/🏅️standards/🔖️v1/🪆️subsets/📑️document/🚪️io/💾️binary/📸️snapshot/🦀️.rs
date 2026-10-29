//! 💾️ Binary representation codec surface for `s.stdio.semio.document` (snapshot) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use crate::io::text::snapshot::dec_image;
use crate::io::text::snapshot::enc_image;
use crate::io::text::snapshot::dec_style;
use crate::io::text::snapshot::enc_style;
use crate::io::text::snapshot::dec_block;
use crate::io::text::snapshot::enc_block;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_str;
use framework_schema::ArtifactSchema;

semio_s_artifact_stdio_semio_base::semio_snapshot_record_pack!(SemioDocumentSnapshot);
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SemioDocumentSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> { Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec()) }
}
}

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use crate::io::text::snapshot::dec_image;
use crate::io::text::snapshot::enc_image;
use crate::io::text::snapshot::dec_style;
use crate::io::text::snapshot::enc_style;
use crate::io::text::snapshot::dec_block;
use crate::io::text::snapshot::enc_block;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_str;
use framework_schema::ArtifactSchema;
use crate::io::text::snapshot::*;
/// 📥️ Decodes this subset's own committed `.pack.semio` bytes into a real [`SemioDocumentSnapshot`] — the
/// binary half of the same bridge, so a caller outside this crate can check the two codecs against
/// each other on the two real committed artifacts instead of against itself.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_document_pack(bytes: &[u8]) -> Result<SemioDocumentSnapshot, String> {
    <SemioDocumentSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
/// 📤️ The `pack::snapshot::ArtifactPack::encode_pack` inverse of [`decode_semio_document_pack`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_document_pack(snapshot: &SemioDocumentSnapshot) -> Vec<u8> {
    <SemioDocumentSnapshot as pack::snapshot::ArtifactPack>::encode_pack(snapshot)
}
}
pub use native_snapshot_codec::*;
