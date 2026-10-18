//! 🎬️ The browser document actor: one actor per open document on the page executor, its hub binding, and the browser `ArtifactHost` that owns them.

use semio_framework_os_store_sync::{ARTIFACT_BOOTSTRAP_DEADLINE_MS, ArtifactActorConfig, ArtifactActorMsg, ArtifactChannelsOf, ArtifactDocumentKey, ArtifactEvent, ArtifactHostCore, ArtifactMailboxReceiver, ArtifactSyncStatus, CommandAckOutcome, CommandRejectionCode, CommandRejectionDetail, DOCUMENT_LINK_SHORTAGE_POLICY, DocumentBackboneRetentionV1, DocumentLink, DocumentLinkEvent, DocumentSocket, DocumentSocketBinding, DocumentSocketDialer, DocumentSocketPoll, OpenDocument, PersistenceBinding, PresenceHeartbeatProducer, RemoteState, actor_identity_admission_admits, add_actor_identity_progress, admit_remote_envelopes, artifact_mailbox_pair, canonical_pair_baseline, commands_frames_within, decode_document_backbone_message_exact, document_admission_refuses_access, document_link_terminal_message, document_pack_schema_hash, document_socket_authority_admits, document_socket_expectation, document_socket_hello, document_socket_protocols, frontier_reaches, hub_ws_url, note_authored_envelopes, now_ms, presence_from_bytes, presence_to_bytes, rebootstrap_control, replica_hlc_seed, rollback_envelope, settle_committed_envelopes, spr_events, stamp_session, transition_refusal, validate_artifact_bootstrap_identity, wire_envelopes_with_original_actor};
use semio_framework_os_spr::{AckStage, ApplyOutcome, ArtifactBootstrap, ArtifactBootstrapAssembler, ArtifactBootstrapControl, ArtifactBootstrapLimits, ArtifactBootstrapPair, ArtifactBootstrapProgress, Bootstrap, ClientFrame, Lane, MutationEnvelope, MutationMessage, PresencePeer, PresenceToolRun, RuntimeFrontierSummary, ServerFrame, decode_envelopes, decode_server_frame, encode_client_frame, encode_envelopes};
use semio_framework_os_store_backbone::{BackboneMessage, ChannelBackbone, ChannelBackboneRemote};
use tokio::sync::broadcast;
use std::sync::{Arc, RwLock};
use wasm_bindgen::JsCast;

#[path = "🎛️wasm-actor-state-status-and-connect/🦀️.rs"]
mod wasm_actor_state_status_and_connect;
use wasm_actor_state_status_and_connect::{
    sleep_ms, wall_ms, PendingWasmArtifactBootstrap, SharedDialer, WasmActor, WasmBootstrapControl,
};
pub(super) use wasm_actor_state_status_and_connect::WasmActorHub;

#[path = "🚇️wasm-actor-relay-and-bootstrap/🦀️.rs"]
mod wasm_actor_relay_and_bootstrap;

#[path = "📡️wasm-actor-protocol-and-spawn/🦀️.rs"]
mod wasm_actor_protocol_and_spawn;
pub(super) use wasm_actor_protocol_and_spawn::spawn_actor;

#[path = "🏠️wasm-artifact-host/🦀️.rs"]
mod wasm_artifact_host;
pub use wasm_artifact_host::{ArtifactChannels, ArtifactHost};

#[cfg(test)]
#[path = "../🧪️tests/🔬️echo-suppression-pin/🦀️.rs"]
mod echo_suppression_pin;
