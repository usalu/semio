//! 💾️ Binary representation codec surface for `stdio.semio.table` (snapshot) — protocol include.
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use semio_s_artifact_stdio_semio_value_model::io::text::diff::dec_semio_value;
use semio_s_artifact_stdio_semio_value_model::io::text::diff::enc_semio_value;
use semio_s_artifact_stdio_semio_base::io::binary::codec::read_str_lp;
use semio_s_artifact_stdio_semio_base::io::binary::codec::write_str_lp;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_str;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use framework_schema::ArtifactSchema;

semio_s_artifact_stdio_semio_base::semio_snapshot_record_pack!(SemioTableSnapshot);
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SemioTableSnapshot {
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
use semio_s_artifact_stdio_semio_value_model::io::text::diff::dec_semio_value;
use semio_s_artifact_stdio_semio_value_model::io::text::diff::enc_semio_value;
use semio_s_artifact_stdio_semio_base::io::binary::codec::read_str_lp;
use semio_s_artifact_stdio_semio_base::io::binary::codec::write_str_lp;
use semio_s_artifact_stdio_semio_base::io::text::codec::dec_str;
use semio_s_artifact_stdio_semio_base::io::text::codec::enc_str;
use semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValue;
use framework_schema::ArtifactSchema;
use crate::io::text::snapshot::*;
/// 📦️ Encodes a [`SemioTableSnapshot`] as a semio pack envelope — the binary twin of the DSL text, produced by a
/// SEPARATE codec, which is what makes the two committed encodings of one document able to
/// contradict each other.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_table_pack(snapshot: &SemioTableSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(snapshot)
}
/// 📦️ Decodes a semio pack envelope into a [`SemioTableSnapshot`] — the inverse of
/// [`encode_semio_table_pack`], reading `../../🖼️assets/📃️sheet/🎒️.pack.semio`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_table_pack(bytes: &[u8]) -> Result<SemioTableSnapshot, String> {
    <SemioTableSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
}
pub use native_snapshot_codec::*;
