//! 🧶️ Companion tests of os-directory-client: client unit tests, native transports, document HTTP and identity.
//!
//! 🧭️ The tests are mounted unchanged from the production tree and reach the production crate through its public API only.
#![cfg(test)]

pub use semio_framework_os_directory_client::*;

#[path = "."]
pub mod client {
    pub use semio_framework_os_directory_client::{DOCUMENT_HTTP_PORT_TOPIC, DOCUMENT_HTTP_REQUEST_MAX_BYTES, DOCUMENT_HTTP_RESPONSE_MAX_BYTES, DirectoryTransport, DirectoryWsConnection, DirectoryWsPoll, DocumentHttpOperationV1, DocumentHttpPortCodeV1, DocumentHttpPortDeclarationV1, DocumentHttpRequestV1, HttpMethod, HttpResponse, InstalledServiceContributionV1, InstalledServiceDriverV1, InstalledServiceStatusV1, InstalledServiceTurnV1, TransportError};
    #[cfg(not(target_arch = "wasm32"))]
    pub use semio_framework_os_directory_client::{DirectoryConnectionPlatform, DirectoryTransportPlatform};
    #[cfg(target_arch = "wasm32")]
    pub use semio_framework_os_directory_client::{DirectoryConnectionPlatform, DirectoryTransportPlatform};
    pub use semio_framework_os_directory_client::{CanonicalDirectoryCommandReceiptV1, CanonicalDirectoryEventPageV1, CanonicalDirectorySpaceAdministrationPageV1, DirectoryBootstrapTransition, DirectoryClientError, DirectoryEventPageAckV1, DirectoryEventPageBootstrapV1, SessionMintResponse, SessionView};
    pub use semio_framework_os_directory_client::{DocumentSocketAdmissionV1, DocumentSocketAuthorityV1, DocumentSocketExpectationV1, DocumentSocketGrantReceiptV1, HubSocketGrantSource, SocketGrantReceiptV1};
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    pub use semio_framework_os_directory_client::DocumentSocketAdmissionFuture;
    pub use semio_framework_os_directory_client::{DOCUMENT_ADMISSION_RESPONSE_MAX_BYTES, LocalHubCredential, decode_local_hub_credential, decode_lower_hex_32, encode_url_component, read_local_hub_credential_frame, wipe_dsl_value};
    #[cfg(not(all(target_arch = "wasm32", not(target_env = "p2"))))]
    pub use semio_framework_os_directory_client::wall_now_ms;
    pub use semio_framework_os_directory_client::DirectoryClient;
    pub use semio_framework_os_directory_client::{DirectoryStream, DirectoryStreamTurn, HUB_RECONNECT_MAX_MS, HUB_RECONNECT_MIN_MS, directory_scoped_ws_url, directory_ws_url, next_backoff_ms};
    pub use semio_framework_os_directory_client::{EXECUTION_TARGET_STORE_COMPONENT_CAPACITY_BYTES, EXECUTION_TARGET_STORE_STAGE_ABANDONED_SECS, EXECUTION_TARGET_UNAVAILABLE_ATTEMPTS, ExecutionTargetModuleFiles, ExecutionTargetModuleSource, ExecutionTargetModuleStep, ResolvedExecutionTargetModule, verify_execution_target_bytes};
    #[cfg(not(target_arch = "wasm32"))]
    pub use semio_framework_os_directory_client::ExecutionTargetModuleStore;
    pub use semio_framework_os_directory_client::{agent_delegations, canonical_checkpoint_pair_path, CANONICAL_CHECKPOINT_PAIR_TRANSIENT_ATTEMPTS, CANONICAL_CHECKPOINT_PAIR_TRANSIENT_STATUSES};
    use semio_framework_os_directory_client::schema::{lease_fields_from_plan_v1, same_lease_fields_v1, DirectoryCommand, DirectoryCommandErrorCodeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryEventPageV1, DirectorySpaceAdministrationCapabilitiesV1, DirectorySpaceAdministrationDocumentWindowV1, DirectorySpaceAdministrationInviteWindowV1, DirectorySpaceAdministrationMemberWindowV1, DirectorySpaceAdministrationPageV1, DirectorySpaceRole, DirectoryStreamMessage, DocumentExecutionTargetComponentV1, DocumentExecutionTargetLeaseFieldsV1, DocumentOpenCheckpointV1, DocumentOpenIntentV1, DocumentOpenParentDialectV1, DocumentOpenPlanV1, DocumentOpenRendererTargetV1, DocumentScope, DocumentView, MemberSpaceViewV1, DIRECTORY_COMMAND_RECEIPT_MAX_BYTES, DIRECTORY_EVENT_PAGE_MAX_BYTES, DOCUMENT_OPEN_MAX_SAFE_INTEGER};
    use semio_framework_value::{DslValue, FromValue, ToValue, ValueError};
    use semio_framework_async::OperationContext;
    use std::sync::Arc;

    #[path = "../🔬️unit/🦀️.rs"]
    mod tests;

    #[path = "../../🌐️document-http/🧪️tests"]
    pub mod document_http {
        pub use semio_framework_os_directory_client::client::document_http::*;
        use semio_framework_schema_validator::OwnedJsonSchemaValidator;
        use semio_framework_os_directory_client::{DirectoryClient, DirectoryTransport, DocumentHttpPortCodeV1, DocumentHttpPortDeclarationV1, HttpMethod};
        use semio_framework_os_directory_client::DocumentScope;
        use semio_framework_async::OperationContext;

        #[path = "🦀️.rs"]
        mod tests;
    }

    #[path = "."]
    pub mod native {
        pub use semio_framework_os_directory_client::client::native::*;
        use semio_framework_os_directory_client::{DirectoryTransport, DirectoryWsConnection, HttpMethod, TransportError};
        use semio_framework_actor::{ActorId, PackageId};
        use semio_framework_async::{HostAsyncRuntime, HostFuture, OperationContext, ScopeHandle};
        use semio_framework_os_services::{AsyncHttpTransport, ComputeError, ComputePool, HttpBody, HttpBodyCancellationHandle, HttpPool, HttpPoolError, HttpRequest as PoolHttpRequest, HttpResponseHead, HttpTransportStart, HttpTransportTerminalGuard, TokioHostRuntime, HTTP_BUCKET_REFILL_INTERVAL_MS};
        use std::io::Read;
        use std::net::TcpStream;
        use std::sync::{Arc, Mutex};
        use std::time::Duration;
        use tokio_tungstenite::tungstenite;
        use tokio_tungstenite::tungstenite::client::IntoClientRequest;
        use tokio_tungstenite::tungstenite::stream::MaybeTlsStream;
        use tokio_tungstenite::tungstenite::Message;

        include!("../🔬️native-runtime-identity/🦀️.rs");

        include!("../🔬️native-streaming/🦀️.rs");
    }
}

#[path = "../../../🪪️identity/🧪️tests/🔬️unit"]
pub mod identity {
    pub use semio_framework_os_directory_client::identity::*;
    use semio_framework_os_directory_client::{DirectoryTransport, LocalHubCredential};
    use semio_framework_async::OperationContext;
    use semio_framework_value_derive::{FromValue, ToValue};
    use std::sync::{Arc, OnceLock};

    #[path = "🦀️.rs"]
    mod tests;
}
