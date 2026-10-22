
use semio_hub_route_models::admin::{ADMIN_EFFECT_ADMITTED, ADMIN_EFFECT_PRE_EFFECT, ADMIN_OPERATION_DEADLINE, AdminIntentMetadata, AdminIntentSecretResult, AdminOperationCleanup, AdminOperationRuntime, AdminOperationTaskOwner, AdminPageQuery, admin_audit_receipt, admin_audit_visibility, admin_create_space_id, admin_cursor_decode, admin_cursor_decode_scoped, admin_cursor_encode_scoped, admin_intent_bindings, admin_page_limit, new_admin_audit_fact};
use semio_hub_directory_contract::{ADMIN_PAGE_MAX, InviteCapability, SessionCapability, SocketGrantCapability};
use semio_hub_directory_contract::model::{AdminEffectCommitV1, AuthSessionKind, DirectoryCommandClaimV1, DirectoryCommandDispositionV1, DirectoryCommandReceiptCompletion, DirectoryCommandReceiptRecord, DirectoryCommandResultKindV1, NewAdminOperationEffectReceiptV1, NewDirectoryCommandReceipt, SpaceRole};
use semio_hub_ledgers::guard::AdminPrincipalV1;
use std::sync::Arc;
use directory::os_directory::{DirectoryCommandOutcomeV1, DirectoryCommandResultV1, DocumentScope};
use semio_hub_directory_contract::error::DirectoryError;
use semio_hub_runtime::support::{DirectoryJson, now_ms};
use semio_hub_route_testkit::{DocumentOpenPlanLedgerFixture, LawHangWatchdogV1, RawHttpResponse, SOCKET_RENDEZVOUS_HANG_GUARD, STUDIO, TEST_EXECUTION_TARGET_COMPONENT_BYTES, TEST_EXECUTION_TARGET_DESCRIPTOR_BYTES, announce_document_for_test, authorize_test_admin, bearer_headers, bounded_law_step, client_binary, create_space_for_test, document_open_catalog_for_descriptor, document_open_plan_test_authority, document_socket_request, issue_and_exchange_document_open_plan_for_test, issue_test_session, next_close_code, next_close_without_authority, next_directory_message, next_server_frame_at, post_directory_command_for_test, presence_hex_bytes, presence_normalization_fixture, raw_http_get, raw_http_request, run_socket_test, seed_author_token, seed_genesis_for_document_for_test, socket_hello, socket_request, spawn_restartable_server, spawn_server, stop_recovery_server, tempdir, test_state, test_state_with_directory, upsert_member_for_test};
use semio_hub_directory_dispatch::HubDirectories;
use semio_hub_runtime::readiness::{HubMode, hub_readiness};
use semio_hub_state::HubState;
use semio_hub_ledgers::socket_grants::{SOCKET_GRANT_BINDING_PENDING_CAPACITY, SOCKET_GRANT_LEDGER_CAPACITY, SocketAudienceV1, SocketBindingKeyV1, SocketBindingValidityV1, SocketGrantKeyV1, SocketGrantLedgerV1, SocketGrantRecordV1, SocketGrantStateV1, SocketSubjectV1};
use semio_framework_async_slots::ShardedMap;
use semio_hub_directory_sqlite::SqliteDirectory;
use tokio_tungstenite::tungstenite::Message as WsMessage;
use semio_hub_session_routes::guard::authenticate_admin_principal;
use tokio_tungstenite::connect_async;
use semio_hub_route_models::directory_events::{directory_access_change_for_reader, directory_message_bindings, directory_stream_message_space};
use semio_hub_directory_service::{directory_command_result_kind, replay_directory_command_receipt};
use {directory as directory_command_sha256, directory as validate_directory_event_page_event};
use semio_hub_runtime::presence::document_scope_key_v1;
use semio_hub_session_routes::session_agent::{get_session_me, post_session_sign_out};
use semio_hub_session_routes::access_grants::issue_directory_socket_grant;
use semio_hub_document_routes::admin::reconcile_stale_admin_acceptance;
use semio_hub_session_routes::directory_commands::resolve_bearer_user;
use semio_hub_document_routes::directory_events::socket_directory_membership_visibility;
use semio_hub_directory::{AdminOperationDirectory, AuthSessionDirectory, DocumentDirectory, EventLog, InviteDirectory, ReceiptDirectory, ShareTokenDirectory, SpaceDirectory};
#[cfg(any(test, feature = "testkit"))]
use semio_hub_route_models::admin::ADMIN_SECRET_WIPE_BYTES;
#[cfg(test)]
use directory::os_directory::{AdminIntentReceiptV1, AdminIntentStateV1, AdminIntentV1, ConnectionView, DIRECTORY_COMMAND_REQUEST_MAX_BYTES, DIRECTORY_EVENT_PAGE_MAX_BYTES, DIRECTORY_EVENT_PAGE_MAX_RAW_ROWS, DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES, DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA, DirectoryActor, DirectoryActorKind, DirectoryCommand, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryConnectionPhase, DirectoryEvent, DirectoryEventPageErrorV1, DirectoryEventPageV1, DirectorySpaceAdministrationPageV1, DirectorySpaceRole, DirectorySpaceVisibility, DirectoryStreamMessage, DocumentExecutionTargetLeaseFieldsV1, DocumentOpenIntentV1, DocumentOpenPlanErrorCodeV1, DocumentOpenPlanErrorV1, DocumentOpenPlanV1, DocumentPlanSocketGrantIntentV1};
#[cfg(test)]
use axum::body::Bytes;
#[cfg(test)]
use protocol::{ClientFrame, Lane, ServerFrame};
#[cfg(test)]
use axum::http::{HeaderMap, StatusCode};
#[cfg(test)]
use axum::extract::State;
#[cfg(any(test, feature = "testkit"))]
use semio_hub_route_models::state::{TestLiveGate, last_document_plan_refusal};
#[cfg(test)]
use directory::os_directory;

include!("🛑️admin-removal-binding-wire-and-short-effects/🦀️.rs");
include!("👥️principal-held-commands-and-reader-access-events/🦀️.rs");
include!("🔌️socket-delivery-visibility-and-public-projection/🦀️.rs");
include!("🌐️space-administration-page-and-public-boundary/🦀️.rs");
include!("🧯️admin-document-cursor-shutdown-and-rebuild-slots/🦀️.rs");
include!("🔑️directory-command-authority-and-invite-redemption/🦀️.rs");
include!("🧾️directory-command-receipt-routes/🦀️.rs");
include!("📄️directory-event-page-routes-and-session-roundtrip/🦀️.rs");
