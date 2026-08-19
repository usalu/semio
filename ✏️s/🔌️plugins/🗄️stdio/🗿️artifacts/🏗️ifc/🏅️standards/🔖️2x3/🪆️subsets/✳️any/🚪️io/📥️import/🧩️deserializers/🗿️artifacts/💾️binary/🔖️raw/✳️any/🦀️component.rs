//! deser ifc.2x3 via binary

use crate::artifacts::binary::BinarySnapshot;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;

pub async fn register() {}

pub async fn deserialize(from: &BinarySnapshot) -> Result<Ifc2x3Snapshot, store::PackError> {
    crate::artifacts::ifc::standards::v2x3::engine::decode_ifc2x3(&from.bytes).map_err(store::PackError::Schema)
}

pub async fn deserialize_bytes(bytes: &[u8]) -> Result<Ifc2x3Snapshot, store::PackError> {
    deserialize(&<BinarySnapshot as store::ArtifactPack>::decode_pack(bytes)?)
}
