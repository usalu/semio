//! 💾️ XLSX snapshot pack facet over complete literal native fields.

use crate::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot;
use native::{encode_standalone, decode_binary, pack_limits};

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[path = "🧩️native/🦀️.rs"]
pub(in crate::standards::v_ecma_376::subsets::base::io) mod native;

impl pack::snapshot::ArtifactPack for XlsxSnapshot {
    
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
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for XlsxSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}
