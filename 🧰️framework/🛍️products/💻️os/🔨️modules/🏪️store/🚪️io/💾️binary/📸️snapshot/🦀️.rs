//! 🚪️ Store authored representation implementations.
use semio_framework_value::DslValue;
use semio_framework_os_spr as protocol;
use semio_framework_os_store_space_schema::space_history_document::SpaceHistorySnapshot;
use crate::ArtifactStoreSnapshot;
impl ArtifactStoreSnapshot for protocol::InteractionState {
    fn reference_snapshot_codec()->Option<crate::ArtifactReferenceSnapshotCodec<Self>>{Some(crate::binary_reference_snapshot_codec::<Self>())}


}


impl ArtifactStoreSnapshot for SpaceHistorySnapshot {
    fn reference_snapshot_codec()->Option<crate::ArtifactReferenceSnapshotCodec<Self>>{Some(crate::reference_snapshot_codec::<Self>())}
}

impl crate::ArtifactStoreSnapshot for DslValue {
    fn reference_snapshot_codec()->Option<crate::ArtifactReferenceSnapshotCodec<Self>>{Some(crate::binary_reference_snapshot_codec::<Self>())}
}
