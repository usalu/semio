//! 💾️ Binary representation codec surface for `stdio.semio.model` (snapshot).

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::schema::geometry::native;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use framework_schema::ArtifactSchema;

semio_s_artifact_stdio_semio_base::semio_snapshot_record_pack!(SemioModelSnapshot);
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SemioModelSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> { Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec()) }
}
}

#[allow(unused_imports)]
mod native_snapshot_codec {
use super::*;
use super::*;
use crate::schema::snapshot::*;
use semio_s_artifact_stdio_semio_base::schema::geometry::native;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint3;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;
use semio_s_artifact_stdio_semio_base::io::text::codec::strip_brackets;
use semio_s_artifact_stdio_semio_base::io::text::codec::split_top_level;
use framework_schema::ArtifactSchema;
use crate::io::text::snapshot::*;
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_semio_model_pack(snapshot: &SemioModelSnapshot) -> Vec<u8> {
    <SemioModelSnapshot as pack::snapshot::ArtifactPack>::encode_pack(snapshot)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_model_pack(bytes: &[u8]) -> Result<SemioModelSnapshot, String> {
    <SemioModelSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes).map_err(|e| e.to_string())
}
}
pub use native_snapshot_codec::*;
