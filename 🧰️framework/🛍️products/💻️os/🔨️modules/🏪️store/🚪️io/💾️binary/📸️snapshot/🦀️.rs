//! 🚪️ Store authored representation implementations.
use pack::snapshot::ArtifactPack;
use crate::os_pack::{EncodeOptions as PackEncodeOptions,DecodeOptions as PackDecodeOptions,PackError,pack_rt};
use crate::os_pack::pack_rt::renormalize_whole_number_floats;
use semio_framework_value::{DslValue,ToValue,FromValue};
use crate::os_spr as protocol;
use crate::os_store::space_history_document::SpaceHistorySnapshot;
use crate::os_store::serialization::{ArtifactStoreSnapshot,ArtifactSqliteSnapshotHost,ArtifactSqliteSnapshotCodec};
impl ArtifactStoreSnapshot for protocol::InteractionState {

}

impl ArtifactPack for SpaceHistorySnapshot {
    
    fn encode_pack_with(&self, _options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let value = ToValue::to_value(self);
        Ok(pack_rt::encode_pack_value(&value))
    }
    fn decode_pack_with(bytes: &[u8], _options: &PackDecodeOptions) -> Result<Self, PackError> {
        let value = pack_rt::decode_pack_value(bytes)?;
        FromValue::from_value(renormalize_whole_number_floats(value)).map_err(PackError::from)
    }
}
impl ArtifactStoreSnapshot for SpaceHistorySnapshot {
fn sqlite_snapshot_codec() -> Option<ArtifactSqliteSnapshotCodec> {
        Some(<Self as ArtifactSqliteSnapshotHost>::sqlite_codec())
    }
}

impl crate::os_store::serialization::ArtifactStoreSnapshot for DslValue {}
