//! 💾️ XLSX snapshot pack facet over complete literal native fields.

use crate::XlsxSnapshot;
use native::{encode_standalone, decode_binary, pack_limits};

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[path = "🧩️native/🦀️.rs"]
pub(in crate::standards::v_ecma_376::subsets::base::io) mod native;

impl store::ArtifactPack for XlsxSnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        match encode_standalone(self, semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding::Binary, &mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_| true, pack_limits(&options.limits)))
            .map_err(store::PackError::from)?
        {
            semio_framework_os_kernel::io_schema::IoPayload::Binary(bytes) => Ok(bytes),
            semio_framework_os_kernel::io_schema::IoPayload::Text(_) => unreachable!(),
        }
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        decode_binary(bytes, &mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_| true, pack_limits(&options.limits))).map_err(store::PackError::from)
    }
}
