//! 📦️ Raster artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::snapshot::RasterSnapshot;
use store::PackError;

/// 📦️ Encodes a `RasterSnapshot` to its binary pack form.
pub fn encode(document: &RasterSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `RasterSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<RasterSnapshot, PackError> {
    <RasterSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::snapshot::*;

use schema::ArtifactSchema;
use crate::io::text::snapshot::record::RasterNativeDocument;

impl pack::snapshot::ArtifactPack for RasterSnapshot {
    
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&RasterNativeDocument::__dsl_spec(), &RasterNativeDocument::ordinary(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &RasterNativeDocument::__dsl_spec(), options)?;
        RasterNativeDocument::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)?.ordinary_snapshot().map_err(store::PackError::from)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(RasterNativeDocument::__dsl_spec())
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for RasterSnapshot {
fn sqlite_snapshot_codec()->Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec>{Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())}
}
}
#[cfg(test)] pub use snapshot_codec::*;