//! 🔁️ Local-first sync actor layer: a schema-agnostic per-document backbone actor that runs all IO
//! (persist, semio_hub WebSocket sync, file watching) off the UI thread, plus the causal {@link SyncSession}
//! that feeds remote {@link MutationEnvelope}s into a document's vcs edit timeline.
//!
//! # Threading model
//! - **Native** (wgpu native host, tests): {@link ArtifactHost::open} schedules bounded actor turns
//!   on the injected process `WorkerPool`; WebSocket readiness remains on the ambient Tokio I/O
//!   reactor and all actor deadlines use the pool's `TimerWheel`.
//! - **Browser wgpu build** (`wasm32-unknown-unknown`): the actor runs on the owned browser-local
//!   executor and dials its hub through the same `semio.session.v1` admission as the native actor, over
//!   the socket the host's injected `DocumentSocketDialer` opens (no threads, no filesystem). The React
//!   shell's browser host keeps its TS twin (`🏪️store/👷️worker/🟦️.ts`, WS-E).
//! - **WASI-P2 plugins never link this crate** — inside the sandbox a store attaches vcs's pure
//!   `PortBackbone` (an in-memory queue relayed to the host). This actor is a host-side concern only.

use semio_framework_process_routing_workspace as workspace_identity;

#[path = "🚨️sync-errors-actor-identity-and-envelope-serde/🦀️.rs"]
mod sync_errors_actor_identity_and_envelope_serde;
pub use sync_errors_actor_identity_and_envelope_serde::{
    actor_identity_admission_admits, add_actor_identity_progress, is_repository_root,
    wire_envelopes_with_original_actor, SyncError,
};
use sync_errors_actor_identity_and_envelope_serde::{actor_identity_batch_admits, envelope_serde};

#[path = "🗄️persistence-binding-actor-config-and-messages/🦀️.rs"]
mod persistence_binding_actor_config_and_messages;
pub use persistence_binding_actor_config_and_messages::{
    bindings_data_class, wire_lane_data_class, ArtifactActorConfig, ArtifactActorMsg,
    ArtifactDocumentKey, PersistenceBinding, PersistenceDataClass,
};

#[path = "📬️artifact-mailbox/🦀️.rs"]
mod artifact_mailbox;
pub use artifact_mailbox::{
    artifact_mailbox_pair, ArtifactMailboxClose, ArtifactMailboxReceiver, ArtifactMailboxSendError,
    ArtifactMailboxSender, ARTIFACT_MAILBOX_BYTES, ARTIFACT_MAILBOX_ITEMS,
};

#[path = "📡️actor-message-bytes-status-and-events/🦀️.rs"]
mod actor_message_bytes_status_and_events;
pub use actor_message_bytes_status_and_events::{
    artifact_actor_message_bytes, ArtifactEvent, ArtifactSyncStatus, RemoteState,
};

#[path = "🧭️backbone-retention-bootstrap-and-command-acks/🦀️.rs"]
mod backbone_retention_bootstrap_and_command_acks;
pub use backbone_retention_bootstrap_and_command_acks::{
    acknowledged_edited_head, decode_document_backbone_message_exact, frontier_reaches, validate_artifact_bootstrap_identity,
    CommandAckOutcome, CommandRejectionCode, CommandRejectionDetail, DocumentBackboneRetentionV1,
    ARTIFACT_BOOTSTRAP_DEADLINE_MS,
};

#[path = "🧵️backbone-worker-wire-and-history-envelopes/🦀️.rs"]
mod backbone_worker_wire_and_history_envelopes;
pub use backbone_worker_wire_and_history_envelopes::{
    backbone_worker_wire, envelopes_from_history_edit, hub_ws_url, spr_events,
};
#[cfg(not(target_arch="wasm32"))]
pub use backbone_worker_wire_and_history_envelopes::{
    document_archive_hash, history_edit_from_envelope, spr_op_ids,
};

#[path = "🔌️document-socket-link-and-echo-suppression/🦀️.rs"]
mod document_socket_link_and_echo_suppression;
pub use document_socket_link_and_echo_suppression::{
    admit_remote_envelopes, canonical_pair_baseline, commands_frames_within,
    document_admission_refuses_access, document_link_terminal_message, document_pack_schema_hash,
    document_socket_authority_admits, document_socket_expectation, document_socket_hello,
    document_socket_protocols, note_authored_envelopes, rebootstrap_control, replica_hlc_seed,
    settle_committed_envelopes, CommandsFrame, CommandsFramePlan, DocumentLink, DocumentLinkEvent,
    DocumentLinkShortagePolicy, DocumentLinkStatus, DocumentSocketBinding,
    DOCUMENT_LINK_ACCESS_REFUSED_STATUSES, DOCUMENT_LINK_SHORTAGE_POLICY,
};
#[cfg(all(target_arch="wasm32",not(target_env="p2")))]
pub use document_socket_link_and_echo_suppression::{
    DocumentSocket, DocumentSocketDialer, DocumentSocketPoll,
};

#[path = "🌉️wire-bridge-sync-session-and-presence-heartbeat/🦀️.rs"]
mod wire_bridge_sync_session_and_presence_heartbeat;
pub use wire_bridge_sync_session_and_presence_heartbeat::{
    actor_seed, presence_from_bytes, presence_to_bytes, presence_tool_run_clamped,
    rollback_envelope, stamp_session, transition_refusal, PresenceHeartbeatProducer, SyncSession,
    HISTORY_TRANSITION_REFUSED_CODE, PRESENCE_HEARTBEAT_INTERVAL_MS,
};
#[cfg(not(target_arch="wasm32"))]
pub use wire_bridge_sync_session_and_presence_heartbeat::{next_timestamp, now_ms};

#[path = "🏠️artifact-host-state-and-core/🦀️.rs"]
mod artifact_host_state_and_core;
pub use artifact_host_state_and_core::{
    ArtifactChannelsOf, ArtifactHostCore, ArtifactHostState, OpenDocument,
};

use semio_framework_value::DslValue;
use semio_framework_value::FromValue as FromValueTrait;
use semio_framework_value::ToValue as ToValueTrait;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{PresencePeer, PresenceToolRun};
use semio_framework_os_spr::{decode_document_backbone_envelopes_exact, decode_envelopes, decode_server_frame, encode_client_frame, encode_envelopes, AckStage, ApplyOutcome, ArtifactBootstrap, ArtifactBootstrapAssembler, ArtifactBootstrapControl, ArtifactBootstrapLimits, ArtifactBootstrapPair, ArtifactBootstrapProgress, Bootstrap, ClientFrame, Lane, MutationEnvelope, MutationMessage, OpBinary, RuntimeFrontierSummary, ServerFrame};
use semio_framework_os_spr::{ActorId, MutationId};
use semio_framework_os_store_backbone::{decode_hot_backbone_message_exact, BackboneMessage, Backbones, ChannelBackbone, ChannelBackboneRemote, BACKBONE_CHANNEL_MAXIMUM_BYTES, BACKBONE_CHANNEL_MAXIMUM_MESSAGES, BACKBONE_HOT_MESSAGE_MAXIMUM_BYTES};
use semio_framework_os_store_text_io::ArtifactPackFiles;
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
use semio_framework_os_store_text_io::text_format::ArtifactTextFiles;
use semio_framework_value_derive::{FromValue, ToValue};
use tokio::sync::broadcast;

//#region 🔖️Endpoints
// 🧱️ `ArtifactId` names the document of every envelope `envelopes_from_history_edit` rebuilds off an spr
// log, which both actors (native and browser) read.
use semio_framework_os_spr::ArtifactId;

//#endregion 🔖️Endpoints

//#region 🔖️NativeActor

//#endregion 🔖️NativeActor
//#region 🔖️WasmActor

//#endregion 🔖️WasmActor

#[cfg(any(test, feature = "testing"))]
pub mod fixtures {
use super::*;
//#region 🔖️Fixtures
/// 🎬️ A scripted actor test vector shared by cargo test (here) and vitest (WS-E's TS twin).
/// Each fixture drives inbound events at a document actor and asserts the resulting `ArtifactEvent`
/// sequence and the final persisted envelope edit ids. See `framework/sync/fixtures/README.md`.
#[derive(Clone, Debug)]
pub struct ActorFixture {
    pub name: String,
    pub schema: String,
    pub document_id: String,
    /// 📥️ Inbound stimulus applied to the actor, in order.
    pub inbound: Vec<FixtureInbound>,
    /// 📤️ The `ArtifactEvent` variant tags expected on the subscriber channel, in order.
    pub expected_events: Vec<String>,
    /// 📇️ Edit ids expected in the document's timeline after replay.
    pub expected_edit_ids: Vec<String>,
}

/// 📥️ One scripted inbound stimulus: either a semio_hub server frame or an external folder edit.
/// Document/op CONTENT lives in sibling text files (never JSON) — this is the LOADED shape
/// (content already read off disk); see `RawFixtureInbound` for the on-disk manifest shape that
/// only references filenames.
#[derive(Clone, Debug)]
pub enum FixtureInbound {
    /// 📬️ A raw `crate::os_spr::wire::ServerFrame`'s encoded bytes (`crate::os_spr::encode_server_frame`
    /// output, `lane` byte included), delivered as if received over the semio_hub WebSocket — already
    /// real binary, not document/op content, so it stays inline in the manifest as a JSON number
    /// array. Driven by `🏪️store/👷️worker/🟦️.ts`'s TS fallback vitest harness (which decodes these
    /// bytes with its own binary decoder); the folder-only Rust harness skips these.
    HubFrame { frame_bytes: Vec<u8> },
    /// 📁️ An external folder edit: `.ops`-grammar text (one or more `edit ...` blocks) to
    /// append to the spr log out-of-band.
    ExternalEdits { ops_text: String },
    /// ♻️ An external whole-document rewrite (divergent history): dsl + ops text compiled
    /// via `codec.compile_dsl` and written in place of the stored document.
    ReplaceDocument { dsl_text: String, ops_text: String },
}

/// 📄️ The on-disk manifest shape: `kind`-tagged like `FixtureInbound`, but content-bearing
/// variants reference a sibling filename (relative to the fixture's own directory) instead of
/// carrying the text inline — `load_fixtures` resolves these into real `FixtureInbound`s.
#[derive(Clone, Debug, ToValue, FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
enum RawFixtureInbound {
    HubFrame { frame_bytes: Vec<u8> },
    ExternalEdits { ops_file: String },
    ReplaceDocument { dsl_file: String, ops_file: String },
}

#[derive(Clone, Debug, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
struct TestEvidence {
    name: String,
    schema: String,
    document_id: String,
    inbound: Vec<RawFixtureInbound>,
    expected_events: Vec<String>,
    expected_edit_ids: Vec<String>,
}

#[cfg(not(target_arch = "wasm32"))]
async fn parse_fixture_dsl_manifest(text: &str) -> Option<TestEvidence> {
    use std::collections::BTreeMap;

    let mut name = None;
    let mut schema = None;
    let mut document_id = None;
    let mut expected_events = Vec::new();
    let mut expected_edit_ids = Vec::new();
    let mut inbound_fields: BTreeMap<(usize, String), String> = BTreeMap::new();

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, value) = line.split_once('=')?;
        let value = value.trim().to_string();
        if let Some(rest) = key.strip_prefix("inbound.") {
            let (index, field) = rest.split_once('.')?;
            let index: usize = index.parse().ok()?;
            inbound_fields.insert((index, field.to_string()), value);
            continue;
        }
        match key {
            "name" => name = Some(value),
            "schema" => schema = Some(value),
            "documentId" => document_id = Some(value),
            "expectedEvent" => expected_events.push(value),
            "expectedEditId" => expected_edit_ids.push(value),
            _ => {}
        }
    }

    let mut inbound_indexes: Vec<usize> = inbound_fields.keys().map(|(index, _)| *index).collect();
    inbound_indexes.sort_unstable();
    inbound_indexes.dedup();

    let mut inbound = Vec::new();
    for index in inbound_indexes {
        let kind = inbound_fields.get(&(index, "kind".into()))?.clone();
        let raw = match kind.as_str() {
            "externalEdits" => {
                let ops_file = inbound_fields.get(&(index, "opsFile".into()))?.clone();
                RawFixtureInbound::ExternalEdits { ops_file }
            }
            "replaceDocument" => {
                let dsl_file = inbound_fields.get(&(index, "dslFile".into()))?.clone();
                let ops_file = inbound_fields.get(&(index, "opsFile".into()))?.clone();
                RawFixtureInbound::ReplaceDocument { dsl_file, ops_file }
            }
            "hubFrame" => {
                let frame_bytes = inbound_fields.get(&(index, "frameBytes".into()))?.split(',').filter_map(|part| part.trim().parse::<u8>().ok()).collect();
                RawFixtureInbound::HubFrame { frame_bytes }
            }
            _ => return None,
        };
        inbound.push(raw);
    }

    Some(TestEvidence { name: name?, schema: schema?, document_id: document_id?, inbound, expected_events, expected_edit_ids })
}

/// 📂️ Loads every `<name>/🔣️fixture.dsl` manifest directory under `dir`, resolving each
/// content-bearing inbound entry against its sibling text file. A fixture whose manifest or
/// any referenced file is missing/unreadable is skipped (never a partial/silently-wrong fixture).
#[cfg(not(target_arch = "wasm32"))]
pub async fn load_fixtures(dir: &std::path::Path) -> Vec<ActorFixture> {
    let mut fixtures = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return fixtures };
    let mut fixture_dirs: Vec<std::path::PathBuf> = entries.filter_map(|entry| entry.ok().map(|entry| entry.path())).filter(|path| path.is_dir()).collect();
    fixture_dirs.sort();
    for fixture_dir in fixture_dirs {
        let Ok(manifest_text) = std::fs::read_to_string(fixture_dir.join("🔣️fixture.dsl")) else { continue };
        let Some(manifest) = parse_fixture_dsl_manifest(&manifest_text).await else { continue };
        let mut inbound = Vec::with_capacity(manifest.inbound.len());
        let mut all_resolved = true;
        for raw in manifest.inbound {
            let resolved = match raw {
                RawFixtureInbound::HubFrame { frame_bytes } => Some(FixtureInbound::HubFrame { frame_bytes }),
                RawFixtureInbound::ExternalEdits { ops_file } => std::fs::read_to_string(fixture_dir.join(&ops_file)).ok().map(|ops_text| FixtureInbound::ExternalEdits { ops_text }),
                RawFixtureInbound::ReplaceDocument { dsl_file, ops_file } => {
                    let dsl_text = std::fs::read_to_string(fixture_dir.join(&dsl_file)).ok();
                    let ops_text = std::fs::read_to_string(fixture_dir.join(&ops_file)).ok();
                    dsl_text.zip(ops_text).map(|(dsl_text, ops_text)| FixtureInbound::ReplaceDocument { dsl_text, ops_text })
                }
            };
            match resolved {
                Some(inbound_item) => inbound.push(inbound_item),
                None => {
                    all_resolved = false;
                    break;
                }
            }
        }
        if all_resolved {
            fixtures.push(ActorFixture { name: manifest.name, schema: manifest.schema, document_id: manifest.document_id, inbound, expected_events: manifest.expected_events, expected_edit_ids: manifest.expected_edit_ids });
        }
    }
    fixtures
}

}
#[cfg(any(test, feature = "testing"))]
use fixtures::*;
//#endregion 🔖️Fixtures
//#region 🔖️FolderStorage

//#endregion 🔖️FolderStorage
//#region 🔖️BlobStoreImpl

//#endregion 🔖️BlobStoreImpl

#[cfg(test)]
#[path = "🧪️tests/🔬️persistence-data-class/🦀️.rs"]
mod persistence_data_class_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️document-socket-connect/🦀️.rs"]
mod document_socket_connect_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️document-link-shortage/🦀️.rs"]
mod document_link_shortage_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️document-echo-suppression/🦀️.rs"]
mod document_echo_suppression_tests;
#[cfg(test)]
#[path = "🧪️tests/🧪️command-rejection/🦀️.rs"]
mod command_rejection_tests;
