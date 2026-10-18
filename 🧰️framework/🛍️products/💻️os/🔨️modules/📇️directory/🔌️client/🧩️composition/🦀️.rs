//! 🔌️ Typed Directory transport and native/browser credential ownership.
pub use semio_framework_os_directory::{schema,io};
pub use semio_framework_os_directory::*;
pub use semio_framework_value::{DslValue,ToValue,FromValue,ValueError};
#[path="../🦀️.rs"]
pub mod client;
pub use client::http_and_transport_contracts::{DOCUMENT_HTTP_PORT_TOPIC, DOCUMENT_HTTP_REQUEST_MAX_BYTES, DOCUMENT_HTTP_RESPONSE_MAX_BYTES, DirectoryTransport, DirectoryWsConnection, DirectoryWsPoll, DocumentHttpOperationV1, DocumentHttpPortCodeV1, DocumentHttpPortDeclarationV1, DocumentHttpRequestV1, HttpMethod, HttpResponse, InstalledServiceContributionV1, InstalledServiceDriverV1, InstalledServiceStatusV1, InstalledServiceTurnV1, TransportError};
#[cfg(not(target_arch = "wasm32"))]
pub use client::http_and_transport_contracts::{DirectoryConnectionPlatform, DirectoryTransportPlatform};
#[cfg(target_arch = "wasm32")]
pub use client::http_and_transport_contracts::{DirectoryConnectionPlatform, DirectoryTransportPlatform};
pub use client::errors_and_event_pages::{CanonicalDirectoryCommandReceiptV1, CanonicalDirectoryEventPageV1, CanonicalDirectorySpaceAdministrationPageV1, DirectoryBootstrapTransition, DirectoryClientError, DirectoryEventPageAckV1, DirectoryEventPageBootstrapV1, SessionMintResponse, SessionView};
pub use client::socket_grants_and_authority::{DocumentSocketAdmissionV1, DocumentSocketAuthorityV1, DocumentSocketExpectationV1, DocumentSocketGrantReceiptV1, HubSocketGrantSource, SocketGrantReceiptV1};
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use client::socket_grants_and_authority::DocumentSocketAdmissionFuture;
pub use client::wire_helpers_and_credentials::{DOCUMENT_ADMISSION_RESPONSE_MAX_BYTES, LocalHubCredential, decode_local_hub_credential, decode_lower_hex_32, encode_url_component, read_local_hub_credential_frame, wipe_dsl_value};
#[cfg(not(all(target_arch = "wasm32", not(target_env = "p2"))))]
pub use client::wire_helpers_and_credentials::wall_now_ms;
pub use client::directory_client::DirectoryClient;
pub use client::directory_stream::{DirectoryStream, DirectoryStreamTurn, HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS, directory_scoped_ws_url, directory_ws_url, next_backoff_ms};
pub use client::{EXECUTION_TARGET_STORE_COMPONENT_CAPACITY_BYTES, EXECUTION_TARGET_STORE_STAGE_ABANDONED_SECS, EXECUTION_TARGET_UNAVAILABLE_ATTEMPTS, ExecutionTargetModuleFiles, ExecutionTargetModuleSource, ExecutionTargetModuleStep, ResolvedExecutionTargetModule, verify_execution_target_bytes};
#[cfg(not(target_arch = "wasm32"))]
pub use client::ExecutionTargetModuleStore;
pub use client::{agent_delegations, document_http, canonical_checkpoint_pair_path, CANONICAL_CHECKPOINT_PAIR_TRANSIENT_ATTEMPTS, CANONICAL_CHECKPOINT_PAIR_TRANSIENT_STATUSES};
#[cfg(all(feature = "ureq", feature = "sync", not(target_arch = "wasm32")))]
pub use client::native;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use client::browser;
#[path="../../🪪️identity/🦀️.rs"]
pub mod identity;
#[cfg(all(target_arch="wasm32",not(target_env="p2")))]
pub use semio_framework_async as wasm_bindgen_futures;
