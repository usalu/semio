// 🎭️ Native actor tests module: the native actor tests with the mock hub, their fixtures and the actor laws.

//#region 🧪️Actor
#[cfg(not(target_arch = "wasm32"))]
mod actor_tests {
    use semio_framework_test_fixtures::fixture_path;

    use super::*;
    use semio_framework_os_store_sync::{ARTIFACT_MAILBOX_BYTES, ARTIFACT_MAILBOX_ITEMS, ArtifactActorConfig, ArtifactActorMsg, ArtifactDocumentKey, ArtifactEvent, ArtifactMailboxSendError, ArtifactSyncStatus, CommandAckOutcome, DocumentBackboneRetentionV1, HISTORY_TRANSITION_REFUSED_CODE, OpenDocument, PersistenceBinding, PresenceHeartbeatProducer, RemoteState, SyncError, SyncSession, artifact_actor_message_bytes, artifact_mailbox_pair, backbone_worker_wire, decode_document_backbone_message_exact, envelopes_from_history_edit, frontier_reaches, hub_ws_url, presence_to_bytes, rollback_envelope, spr_op_ids, stamp_session};
use semio_framework_os_spr::{AckStage, ActorId, ApplyOutcome, ArtifactBootstrap, ArtifactBootstrapPair, Bootstrap, ClientFrame, Lane, MutationEnvelope, MutationId, PresencePeer, PresenceToolRun, RuntimeFrontierSummary, ServerFrame, decode_envelopes, decode_server_frame, encode_client_frame, encode_envelopes};
use semio_framework_os_store_sync::fixtures::{ActorFixture, FixtureInbound, load_fixtures};
use semio_framework_os_store_sync_native::native_actor::ArtifactHost;
use semio_framework_os_store_text_io::ArtifactPackFiles;
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
use semio_framework_os_store_backbone::{BackboneMessage, Backbones, ChannelBackbone, decode_hot_backbone_message_exact};
use semio_framework_os_store_sync_native::lifted_sync::{FOLDER_EVENT_MAGIC, FolderEventLogStorage, FolderTextStorage};
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_value::ValueError;
use tokio::sync::broadcast;
    use semio_framework_os_spr::{decode_client_frame, encode_server_frame};
    use futures::{SinkExt, StreamExt};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::{broadcast as tokio_broadcast, Mutex};
    use tokio_tungstenite::tungstenite::Message as WsMessage;

    include!("🧰️wait-helpers-and-folder-external-edit/🦀️.rs");
    include!("🛰️mock-hub-and-grant-source/🦀️.rs");
    include!("🔁️two-host-convergence-reconnect-and-fixture-replay-laws/🦀️.rs");
}
//#endregion 🧪️Actor
