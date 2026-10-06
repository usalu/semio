//! binary rep for stdio.ifc.2x3 snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type Ifc2x3SnapshotBinary = Vec<u8>;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v2x3::subsets::base::schema::snapshot::*;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_contract::part21::Part21Document;

impl store::ArtifactPack for Ifc2x3Snapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as semio_framework_os_kernel::ArtifactSqliteSnapshot>::sqlite_codec()) }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> { native::encode_pack(self, options) }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> { native::decode_pack(bytes, options) }
}
}
pub use snapshot_codec::*;
