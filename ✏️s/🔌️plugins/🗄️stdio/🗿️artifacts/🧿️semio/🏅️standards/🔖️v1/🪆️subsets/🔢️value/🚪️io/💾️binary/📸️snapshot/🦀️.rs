//! 💾️ Binary representation codec surface for `stdio.semio.value` (snapshot).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::schema::snapshot::*;
use framework_schema::ArtifactSchema;
use crate::io::text::snapshot::dec_semio_value_snapshot;
use crate::io::text::snapshot::enc_semio_value_snapshot;
semio_s_artifact_stdio_semio_base::semio_snapshot_record_pack!(SemioValueSnapshot);
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SemioValueSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}
}

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::schema::snapshot::*;
use framework_schema::ArtifactSchema;
use crate::io::text::snapshot::*;
/// 📦️ Encodes a [`SemioValueSnapshot`] as a semio pack envelope — the binary twin of the DSL text.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_value_pack(snapshot: &SemioValueSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(snapshot)
}
/// 📦️ Decodes a semio pack envelope into a [`SemioValueSnapshot`] — the inverse of
/// [`encode_semio_value_pack`], reading
/// `../../../../✉️base/📚️examples/🕸️graph/🖼️assets/🎒️.pack.semio`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_value_pack(bytes: &[u8]) -> Result<SemioValueSnapshot, String> {
    <SemioValueSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|error| error.to_string())
}
}
pub use native_snapshot_codec::*;
