//! 🧰️ Shared fixtures of the hub route laws: test states, servers, sockets, sessions and document-open plan authorities.

use semio_hub_route_models::admin::{ADMIN_OPERATION_SHUTDOWN_DEADLINE, AdminOperationTaskOwner};
use protocol::{ActorId, ClientFrame, Lane, MutationEnvelope, ServerFrame, ArtifactId as WireArtifactId};
use semio_hub_ledgers::guard::{AdminSubject, admin_provider_digest};
use std::sync::Arc;
use semio_hub_artifact_authority::{ArtifactBlobIntegrity, ArtifactPair, AuthorityLimits, CheckpointPublicationOrchestrator, OperationContext, checkpoint_id_encoding_v1};
use semio_hub_runtime::startup::{ArtifactCasMaintenanceSupervisor, DocumentOpenCatalogAuthorityV1, StartupCatalogControl, instance_state};
use semio_hub_artifact_authority::chunk_cas::{ArtifactChunkBlobStore, ArtifactChunkCasStores, FsArtifactChunkCasStorage, MemoryArtifactChunkCasStorage, artifact_cas_manifest_locator_v1, prepare_artifact_cas_manifest_v1, prepare_artifact_cas_ownership_v1};
use semio_hub_artifact_creation::ArtifactCreationActorV1;
use semio_hub_route_models::artifact_creation::{ArtifactCreationHttpTaskOwnerV1, HubArtifactCreationCommitAuthorityV1};
use semio_hub_directory_contract::model::{AuthSessionIssue, AuthSessionKind, SpaceRole};
use axum::body::Bytes;
use semio_framework_trace::record::{CapturingSink, TraceLevel, Tracer};
use semio_hub_auth::CredentialSignInPolicyV1;
use semio_hub_runtime::transport::{CrossOriginPolicyV1, ForwardedTlsTrustV1};
use directory::os_directory::{DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryCommandRequestV1, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentDescriptor, DocumentOpenArtifactV1, DocumentOpenGrantV1, DocumentOpenIntentV1, DocumentOpenPackageV1, DocumentOpenPlanV1, DocumentOpenSurfaceV1, DocumentPlanSocketGrantIntentV1, DocumentScope};
use semio_hub_runtime::support::{DirectoryJson, now_ms};
use semio_hub_directory_service::{DirectoryService, HubVerifiedCheckpointPublisher};
use semio_hub_artifact_checkin::DocumentCheckInJobs;
use semio_hub_ledgers::open_plan::{DocumentOpenPlanAuthorityV1, DocumentOpenPlanLedgerV1};
use semio_hub_ledgers::access_grants::DocumentSocketGrantReceiptV1;
use directory::{FromValue, os_directory};
use axum::http::HeaderMap;
use semio_hub_directory_dispatch::HubDirectories;
use semio_hub_stores::HubInstance;
use semio_hub_runtime::readiness::{HubMode, hub_readiness};
use semio_hub_auth::rate_limit::HubRateLimiterV1;
use semio_hub_runtime::lifecycle::{HubSocketDrainV1, hub_worker_pool};
use semio_hub_state::HubState;
use axum::Json;
use semio_hub_runtime::presence::PresenceDirectoryProjections;
use semio_hub_ledgers::socket_grants::{SESSION_PROTOCOL_V1, SOCKET_PROTOCOL_V1, SocketBindingGatesV1, SocketGrantLedgerV1, SocketSubjectV1};
use server::gateway::ServerState;
use semio_hub_directory_contract::{SessionCapability, identity_subject_digest};
use semio_framework_hash::Sha256;
use semio_framework_async::ShardedMap;
use std::net::SocketAddr;
use directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreateV1;
use semio_hub_directory_sqlite::SqliteDirectory;
use futures::StreamExt;
use semio_hub_route_models::state::{TestDocumentOpenCatalog, last_document_plan_refusal};
use semio_hub_trusted_catalog_descriptor::VerifiedDocumentOpenSelectionV1;
use semio_hub_lag_rebootstrap::VerifiedRebootstrapSource;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use semio_hub_artifact_creation::io::artifact_creation_command_digest_v1;
use semio_hub_session_routes::access_grants::{issue_document_open_plan_inner, issue_document_plan_socket_grant_inner};
use semio_hub_routes::router;
use semio_hub_directory::{ArtifactCasDirectory, ArtifactCreationDirectory, AuthSessionDirectory, DocumentDirectory, UserDirectory};

#[path = "🌱️document-seeding/🦀️.rs"]
mod document_seeding;
pub use document_seeding::*;

#[path = "🎁️stdio-bundle/🦀️.rs"]
mod stdio_bundle;
pub use stdio_bundle::*;

#[path = "⏱️law-guards/🦀️.rs"]
mod law_guards;
pub use law_guards::*;

#[path = "🏗️test-states/🦀️.rs"]
mod test_states;
pub use test_states::*;

#[path = "🌐️servers-and-sockets/🦀️.rs"]
mod servers_and_sockets;
pub use servers_and_sockets::*;

#[path = "🪪️sessions-and-open-plans/🦀️.rs"]
mod sessions_and_open_plans;
pub use sessions_and_open_plans::*;
