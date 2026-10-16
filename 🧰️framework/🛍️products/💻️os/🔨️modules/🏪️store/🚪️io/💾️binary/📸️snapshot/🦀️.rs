//! 🚪️ Store authored representation implementations.
use semio_framework_value::DslValue;
use semio_framework_os_spr as protocol;
use semio_framework_os_store_space_schema::space_history_document::SpaceHistorySnapshot;
use crate::ArtifactStoreSnapshot;
impl ArtifactStoreSnapshot for protocol::InteractionState {

}


impl ArtifactStoreSnapshot for SpaceHistorySnapshot {}

impl crate::ArtifactStoreSnapshot for DslValue {}
