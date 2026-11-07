//! 🧱️ binary rep for stdio.binary 📸️snapshot

pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");

#[allow(unused_imports)]
mod snapshot_codec {
use super::*;
use crate::standards::v_raw::subsets::any::schema::snapshot::*;
use crate::STDIO_BINARY_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;

/// 🧬️ CARRIER LAW (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §3):
/// `s.stdio.binary@raw/*`'s native `Binary` `IoPayload` is the raw
/// external file content, byte-for-byte. The previous impl wrapped `self.bytes` in a
/// `SemioEnvelope` (`BINARY_MAGIC` header + token), which made every exported `.bin` file an
/// unopenable `.semio` pack container instead of the honest raw bytes — exactly the
/// `registry_export_media` class of bug the ticket exists to remove. Fixed here (the codec, not
/// the test): `encode_pack_with`/`decode_pack_with` are now the identity function on `bytes`.
/// Proven by `carrier_native_is_raw` in `🚪️io/🦀️.rs`.
impl pack::snapshot::ArtifactPack for BinarySnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    

    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let _ = options;
        Ok(self.bytes.clone())
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let _ = options;
        Ok(Self { schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(), bytes: bytes.to_vec() })
    }
}
impl ::semio_os_kernel::os_store::serialization::ArtifactStoreSnapshot for BinarySnapshot {
    fn reference_snapshot_codec()->Option<::semio_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}
impl store::ArtifactPackReceiving for BinarySnapshot {
    fn receive_pack(bytes:&[u8],owner:&mut store::NativeSnapshotDecodeOwner<'_,'_>)->semio_framework_value::RetainedWorkOutcome<Self,store::NativeSnapshotFailure>{
        owner.receive::<Self,Self,store::NativeSnapshotFailure>(|slot,native,body|store::native_snapshot_turn(|work|{work.performed();(||->Result<Self,semio_framework_value::ValueError>{
            crate::standards::v_raw::subsets::any::io::sqlite::snapshot::bind_raw_pack(bytes,slot,native,body)?;
            slot.take().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"Binary receiving lost original typed output"))
        })().map_err(Into::into)}))
    }
}
}
