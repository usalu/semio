use semio_hub_directory::{AuthSessionDirectory, DocumentDirectory, EventLog, ShareTokenDirectory, SpaceDirectory};

#[cfg(test)]
use protocol::{AckStage, ActorId, ApplyOutcome, ClientFrame, Lane, MutationEnvelope, ServerFrame};
#[cfg(test)]
use directory::os_directory::{AdminIntentStateV1, AdminIntentV1, DOCUMENT_OPEN_PLAN_MAX_TTL_MS, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryEvent, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentDescriptor, DocumentExecutionTargetLeaseFieldsV1, DocumentOpenArtifactV1, DocumentOpenGrantV1, DocumentOpenIntentV1, DocumentOpenParentDialectV1, DocumentOpenPlanErrorCodeV1, DocumentOpenPlanV1, DocumentPlanSocketGrantIntentV1};
#[cfg(test)]
use axum::body::Bytes;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_ledgers::open_plan::{DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVATIONS, DOCUMENT_OPEN_PLAN_DECODE_WIPE_OBSERVING, DocumentOpenPlanDecodeWipeObservationV1};
#[cfg(test)]
use axum::extract::{OriginalUri, Path, State};
#[cfg(test)]
use axum::http::StatusCode;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_route_models::state::{TestDocumentOpenCatalog, TestDocumentOpenPlanIssueGate, TestLiveGate};
#[cfg(any(test, feature = "testkit"))]
use semio_hub_session_routes::access_grants::issue_document_socket_grant_fixture;
#[cfg(test)]
use directory::os_directory;
use semio_hub_directory_contract::model::{AdminEffectCommitV1, AuthSessionIssue, AuthSessionKind, SpaceRole};
use std::sync::Arc;
use semio_hub_artifact_creation::ArtifactCreationActorV1;
use semio_hub_ledgers::open_plan::{DOCUMENT_OPEN_PLAN_BINDING_CAPACITY, DOCUMENT_OPEN_PLAN_LEDGER_CAPACITY, DocumentOpenPlanAuthorityV1, DocumentOpenPlanCapabilityV1, DocumentOpenPlanLedgerV1, DocumentOpenPlanStateV1};
use semio_hub_ledgers::access_grants::{DOCUMENT_OPEN_PLAN_EXCHANGE_REQUEST_MAX_BYTES, DOCUMENT_OPEN_PLAN_REQUEST_MAX_BYTES, document_open_checkpoint, socket_actor_id};
use semio_hub_runtime::support::{DirectoryJson, now_ms};
use semio_hub_route_testkit::{DocumentOpenPlanLedgerFixture, LawHangWatchdogV1, STUDIO, TEST_ARTIFACT_PARENT_DIALECT_KIND, TEST_EXECUTION_TARGET_COMPONENT_BYTES, TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES, announce_document_for_test, artifact_document_id_for_test, authorize_test_admin, bearer_headers, client_binary, create_space_for_test, document_descriptor_for_test, document_open_catalog_for_descriptor, document_open_catalog_for_descriptor_with_generation, document_open_plan_test_authority, document_socket_request, install_document_open_catalog_for_test, issue_and_exchange_document_open_plan_for_test, issue_test_session, loopback_peer, next_close_code, next_close_without_authority, next_directory_message, next_server_frame, next_server_frame_at, post_directory_command_for_test, publish_checkpoint_for_test, publish_genesis_checkpoint_for_test, raw_http_get, raw_http_request, raw_http_request_transport, run_socket_test, sample_envelope, seed_author_token, seed_genesis_for_document_for_test, socket_hello, socket_request, spawn_server, test_state, test_state_with_capacity, upsert_member_for_test};
use semio_hub_route_models::document_socket::{DocumentPlanBootstrapV1, HUB_CATCH_UP_ORIGIN};
use directory::os_directory::{DocumentScope, same_lease_fields_v1};
use semio_hub_runtime::readiness::{HubMode, hub_readiness};
use semio_hub_state::HubState;
use semio_hub_auth::rate_limit::{HubStreamLimiterV1, RateLimitRefusalV1, RateLimitSubjectV1, StreamLimitClassV1};
use semio_hub_ledgers::socket_grants::{SESSION_PROTOCOL_V1, SOCKET_GRANT_BINDING_PENDING_CAPACITY, SocketAudienceV1, SocketBindingKeyV1, SocketBindingValidityV1, SocketGrantKeyV1, SocketGrantLedgerErrorV1, SocketGrantLedgerV1, SocketSubjectV1};
use semio_hub_directory_contract::{SessionCapability, SocketGrantCapability, identity_subject_digest};
use semio_framework_hash::Sha256;
use futures::StreamExt;
use protocol::ArtifactId as WireArtifactId;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use semio_hub_route_models::admin::{admin_effect_receipt_claim, admin_intent_digest};
use semio_hub_document_routes::admin::{admin_intents, execute_admin_intent};
use semio_hub_session_routes::guard::authenticate_admin_principal;
use semio_hub_session_routes::access_grants::{authenticate_document_credential, document_execution_target_selection, issue_directory_socket_grant, issue_document_open_plan, issue_document_open_plan_inner};
use tokio_tungstenite::connect_async;
use semio_hub_document_routes::document_socket::{consume_document_socket_grant, document_plan_bootstrap_current, document_plan_socket_validity};
use semio_hub_runtime::presence::{db_artifact_id, document_scope_key_v1};
use semio_hub_route_models::directory_events::directory_message_matches_scope;
use semio_hub_session_routes::session_agent::post_session_sign_out;

include!("🧰️plan-fixtures/🦀️.rs");

include!("🎟️plan-ledger-and-exchange-laws/🦀️.rs");

include!("🎯️execution-target-laws/🦀️.rs");

include!("🛣️plan-route-laws/🦀️.rs");

include!("🔁️supersession-laws/🦀️.rs");

include!("🔌️socket-grant-laws/🦀️.rs");

include!("🗂️scoped-directory-socket-laws/🦀️.rs");
