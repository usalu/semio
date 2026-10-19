use pack::snapshot::ArtifactSqliteSnapshot as _;
use pack::snapshot::{ArtifactDsl, ArtifactPack};
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
use semio_framework_os_spr::{ArtifactId, Edit, Mutation, MutationComposition, MutationDiff, MutationDiffParticipation, MutationInvertibility, MutationLanguageSurface, MutationLeafDescriptor, MutationOutcomeClass, OpBinary, OpText};
use semio_framework_os_store_codec_factory::lifted_store::artifact_codec;
use semio_framework_os_store_document::create_document_envelope;
use semio_framework_os_pack::{pack_rt, PackDecodeOptions, PackEncodeOptions, PackError};
use semio_framework_os_store_text_io::text_format::{parse_document_pack, parse_document_text, print_document_pack, print_document_text, print_edit_lines, ParsedDocumentText};
use semio_framework_os_store_codec_registry::register_document_codec;
use semio_framework_os_store_text_io::ArtifactCodec;
use semio_framework_os_store_space_schema::ArtifactCommand;
use semio_framework_os_store_space_member::lifted_store::BlobStore;
use semio_framework_os_spr::ColdMutationDiff;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

include!("📮️mailbox-and-backbone-envelope-laws/🦀️.rs");
include!("🧸️demo-snapshot-diff-and-mutation-fixtures/🦀️.rs");
include!("🤝️hub-hello-bootstrap-and-receive-laws/🦀️.rs");
include!("🕸️hostile-binding-heartbeat-and-wire-fixtures/🦀️.rs");
include!("🎭️native-actor-tests-module/🦀️.rs");
include!("💾️folder-storage-round-trips/🦀️.rs");
#[path = "../🔬️backbone-parity/🦀️.rs"]
mod backbone_parity;

include!("♻️rebootstrap-supersession-refusal-and-presence-corpus-laws/🦀️.rs");
