//! 🎭️ The native document actor: one finite-turn actor per open document on the process WorkerPool, its runner, and the native `ArtifactHost` that owns them.

use semio_framework_os_store_sync::{acknowledged_edited_head, ARTIFACT_BOOTSTRAP_DEADLINE_MS, ArtifactActorConfig, ArtifactActorMsg, ArtifactChannelsOf, ArtifactDocumentKey, ArtifactEvent, ArtifactHostCore, ArtifactHostState, ArtifactMailboxClose, ArtifactMailboxReceiver, ArtifactSyncStatus, CommandAckOutcome, CommandRejectionCode, CommandRejectionDetail, DOCUMENT_LINK_SHORTAGE_POLICY, DocumentBackboneRetentionV1, DocumentLink, DocumentLinkEvent, OpenDocument, PersistenceBinding, PresenceHeartbeatProducer, RemoteState, actor_identity_admission_admits, actor_seed, add_actor_identity_progress, admit_remote_envelopes, artifact_mailbox_pair, canonical_pair_baseline, decode_document_backbone_message_exact, document_admission_refuses_access, document_archive_hash, document_link_terminal_message, document_pack_schema_hash, frontier_reaches, history_edit_from_envelope, hub_ws_url, next_timestamp, note_authored_envelopes, now_ms, presence_from_bytes, presence_to_bytes, rebootstrap_control, rollback_envelope, settle_committed_envelopes, spr_events, spr_op_ids, stamp_session, transition_refusal, validate_artifact_bootstrap_identity, wire_envelopes_with_original_actor};
use semio_framework_os_spr::{AckStage, ApplyOutcome, ArtifactBootstrap, ArtifactBootstrapAssembler, ArtifactBootstrapControl, ArtifactBootstrapLimits, ArtifactBootstrapPair, ArtifactBootstrapProgress, Bootstrap, ClientFrame, Lane, MutationEnvelope, MutationMessage, OpBinary, PresencePeer, PresenceToolRun, RuntimeFrontierSummary, ServerFrame, decode_envelopes, decode_server_frame, encode_client_frame, encode_envelopes};
use semio_framework_os_store_text_io::ArtifactPackFiles;
use semio_framework_os_store_backbone::{BackboneMessage, ChannelBackbone, ChannelBackboneRemote};
use crate::lifted_sync::{FolderEventLogStorage, FolderTextStorage};
use tokio::sync::broadcast;
use futures::{SinkExt, StreamExt};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::Instant;
use tokio_tungstenite::tungstenite::{client::IntoClientRequest, Message};

#[path = "🔌️actor-socket-types-bootstrap-control-and-folder-endpoint/🦀️.rs"]
mod actor_socket_types_bootstrap_control_and_folder_endpoint;
use actor_socket_types_bootstrap_control_and_folder_endpoint::{
    instant_at, wall_ms, ArtifactDrive, ArtifactDrivePhase, ArtifactReadinessWake,
    ConnectedDocumentSocket, DocumentConnectFailure, FolderEndpoint, HubConn,
    NativeBootstrapControl, PendingArtifactBootstrap, ARTIFACT_CLOSE_OUTBOX_DRAIN,
};
pub use actor_socket_types_bootstrap_control_and_folder_endpoint::ArtifactActor;

#[path = "🎬️actor-construction-drive-and-commands/🦀️.rs"]
mod actor_construction_drive_and_commands;

#[path = "📁️actor-folder-persistence-and-watcher/🦀️.rs"]
mod actor_folder_persistence_and_watcher;
use actor_folder_persistence_and_watcher::{
    build_folder_endpoint, folder_watch_path_for, install_watcher,
};

#[path = "🔗️actor-hub-link-and-bootstrap/🦀️.rs"]
mod actor_hub_link_and_bootstrap;

#[path = "📨️actor-hub-frames-acks-and-delivery/🦀️.rs"]
mod actor_hub_frames_acks_and_delivery;

#[path = "🏃️actor-runner-types-and-handles/🦀️.rs"]
mod actor_runner_types_and_handles;
use actor_runner_types_and_handles::{
    ActorRunner, ActorTerminalJob, ActorTurnFuture, ActorTurnOwner, ActorTurnWake,
    ACTOR_RUNNER_RETRY_LIMIT, ACTOR_RUNNER_RETRY_MS,
};
pub use actor_runner_types_and_handles::{
    ArtifactActorRunnerHandle, ArtifactActorRunnerTicket, ArtifactActorTerminalJob,
    ArtifactActorTerminalReason, ArtifactActorTerminalState,
};

#[path = "⏲️actor-runner-scheduling-terminal-and-spawn/🦀️.rs"]
mod actor_runner_scheduling_terminal_and_spawn;
#[cfg(any(test, feature = "testing"))]
use actor_runner_scheduling_terminal_and_spawn::document_socket_io_reactor;
pub(super) use actor_runner_scheduling_terminal_and_spawn::spawn_actor;

#[path = "🏠️native-artifact-host/🦀️.rs"]
mod native_artifact_host;
pub use native_artifact_host::{ArtifactChannels, ArtifactHost};

#[cfg(any(test, feature = "testing"))]
include!("../🧪️tests/🔬️retained-turn-fixtures/🦀️.rs");

#[cfg(test)]
#[path = "../🧪️tests/🔬️echo-suppression-pin/🦀️.rs"]
mod echo_suppression_pin;
