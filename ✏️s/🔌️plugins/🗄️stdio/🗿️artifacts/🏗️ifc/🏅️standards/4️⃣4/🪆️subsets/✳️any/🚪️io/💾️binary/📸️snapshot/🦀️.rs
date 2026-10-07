//! binary rep for stdio.ifc 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use crate::standards::v4::subsets::any::io::sqlite::snapshot::native;
use super::*;
use crate::standards::v4::subsets::any::schema::snapshot::*;
use crate::STDIO_IFC_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_contract::part21::{parse_part21, write_part21, Part21Document, Part21Header, Part21Instance, Part21Value};

impl store::ArtifactPack for IfcSnapshot {
    fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as semio_framework_os_kernel::ArtifactSqliteSnapshot>::sqlite_codec())}
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> { native::encode_pack(self,options) }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> { native::decode_pack(bytes,options) }
}
}
pub use snapshot_codec::*;
