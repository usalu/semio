//! 🧬️ typed `Puzzle3dSnapshot`, agreeing byte-for-byte with what `🗣️dsl` prints and parses.
//! 🎒️ Puzzle 3d artifact — the binary document surface and its laws: `encode`/`decode` over the

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::snapshot::Puzzle3dSnapshot;
use store::PackError;

/// 📦️ Encodes a `Puzzle3dSnapshot` to its binary pack form.
pub fn encode(document: &Puzzle3dSnapshot) -> Vec<u8> {
    pack::snapshot::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `Puzzle3dSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<Puzzle3dSnapshot, PackError> {
    <Puzzle3dSnapshot as pack::snapshot::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::snapshot::*;
use ::semio_framework_schema::ArtifactSchema;

impl pack::snapshot::ArtifactPack for Puzzle3dSnapshot {
    
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::from(e.into_value_error()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::from(e.into_value_error()))?;
        if !envelope.matches_identity(<Self as pack::snapshot::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as pack::snapshot::ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::os_store::serialization::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot for Puzzle3dSnapshot {
    fn reference_snapshot_codec()->Option<::semio_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())}
}
}
#[cfg(test)] pub use snapshot_codec::*;

#[allow(unused_imports)]
mod snapshot_wire_codec {
use super::*;
use crate::*;
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.

impl protocol::OpBinary for Puzzle3dEngineCommand {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
}
#[cfg(test)] pub use snapshot_wire_codec::*;
impl pack::snapshot::ArtifactNativeSnapshot for Puzzle3dSnapshot {
 fn decode_native_snapshot(payload:pack::snapshot::NativeSnapshotInput<'_>,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{store::decode_native_snapshot_record(payload,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|Self::__dsl_from_record_controlled(record,native),control)}
 fn encode_native_snapshot(&self,encoding:pack::snapshot::NativeSnapshotEncoding,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<store::io_schema::IoPayload,semio_framework_value::ValueError>{store::encode_native_snapshot_record(encoding,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)}
}
