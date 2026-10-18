//! 🔌️ Directory hub client (ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS,
//! contract §C2/§C6) — the Rust twin of lane 1-C's TypeScript `DirectoryClient`, talking the SAME
//! frozen hub HTTP/WS surface: `POST /directory/commands`, `GET /directory/spaces[/{id}]`,
//! `GET /directory/events`, `GET /directory/socket/v1`, and `GET /auth/sessions/me`.
//! No concrete HTTP/WS client type ever appears in a public signature here (CLAUDE.md: external
//! libraries sit behind our own interface) — `DirectoryTransport`/`DirectoryWsConnection` are the
//! seam, mirroring `🎒️pack/🌐️http`'s `RangeTransport` and this crate's own `🏪️store/🔄️sync`
//! (native `tokio-tungstenite`, browser `web_sys`) pattern.
//! `🪪️identity/🦀️.rs` (sibling module) layers the mint-or-restore session helper on top.
//!
//! 🌀️ ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet terra-directory-and-run:
//! every request-issuing method now takes an `OperationContext` (`semio-framework-async` —
//! cancellation/deadline/trace/capability), checked up front so an already-cancelled caller never
//! reaches the transport. The native transport (`🔖️Native` below) is rebuilt on
//! `semio-framework-os-services`'s `HttpPool`/`ComputePool` — the per-call blocking-`ureq`-on-a-
//! dedicated-thread pattern and the ad hoc private tokio runtime this crate's own native `open_ws`
//! used to need are RETIRED, not wrapped: tokio stays confined to `semio-framework-os-services`
//! (this crate names it nowhere), and every native HTTP call is now admitted onto that crate's
//! bounded `ComputePool` semaphore instead of an unbounded `std::thread::spawn` per call.

use super::schema::{
    lease_fields_from_plan_v1, same_lease_fields_v1, DirectoryCommand, DirectoryCommandErrorCodeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryEvent, DirectoryEventPageV1, DirectorySessionAuthorityV1,
    DirectorySpaceAdministrationCapabilitiesV1, DirectorySpaceAdministrationDocumentWindowV1, DirectorySpaceAdministrationInviteWindowV1, DirectorySpaceAdministrationMemberWindowV1, DirectorySpaceAdministrationPageV1, DirectorySpaceListEntryV1,
    DirectorySpaceRole, DirectoryStreamMessage, DocumentBrowserActorSourceV1, DocumentExecutionTargetComponentV1, DocumentExecutionTargetDescriptorV1, DocumentExecutionTargetLeaseFieldsV1, DocumentOpenArtifactV1, DocumentOpenBrowserActorV1,
    DocumentOpenCatalogV1, DocumentOpenCheckpointV1, DocumentOpenGrantV1, DocumentOpenIntentV1, DocumentOpenPackageV1, DocumentOpenParentDialectV1, DocumentOpenPlanErrorCodeV1, DocumentOpenPlanV1, DocumentOpenRendererTargetV1,
    DocumentOpenRevalidationV1, DocumentOpenSurfaceRoleV1, DocumentOpenSurfaceV1, DocumentPlanSocketGrantIntentV1, DocumentScope, DocumentView, MemberSpaceViewV1, DIRECTORY_COMMAND_RECEIPT_MAX_BYTES, DIRECTORY_EVENT_PAGE_MAX_BYTES, DIRECTORY_SESSION_AUTHORITY_MAX_BYTES,
    DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES, DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES, DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES, DOCUMENT_OPEN_MAX_SAFE_INTEGER,
};
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_async::OperationContext;
use std::sync::Arc;

#[path = "📜️http-and-transport-contracts/🦀️.rs"]
pub(crate) mod http_and_transport_contracts;
use http_and_transport_contracts::{DOCUMENT_HTTP_PORT_TOPIC, DOCUMENT_HTTP_REQUEST_MAX_BYTES, DOCUMENT_HTTP_RESPONSE_MAX_BYTES, DirectoryTransport, DirectoryWsConnection, DirectoryWsPoll, DocumentHttpOperationV1, DocumentHttpPortCodeV1, DocumentHttpPortDeclarationV1, DocumentHttpRequestV1, HttpMethod, HttpResponse, TransportError};

#[path = "⚠️errors-and-event-pages/🦀️.rs"]
pub(crate) mod errors_and_event_pages;
use errors_and_event_pages::{CanonicalDirectoryCommandReceiptV1, CanonicalDirectoryEventPageV1, CanonicalDirectorySpaceAdministrationPageV1, DirectoryClientError, SessionView};

#[path = "🔑️socket-grants-and-authority/🦀️.rs"]
pub(crate) mod socket_grants_and_authority;
use socket_grants_and_authority::{DocumentSocketAdmissionV1, DocumentSocketAuthorityV1, DocumentSocketExpectationV1, DocumentSocketGrantReceiptV1, HubSocketGrantSource, SocketGrantReceiptV1};
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use socket_grants_and_authority::DocumentSocketAdmissionFuture;

#[path = "🧰️wire-helpers-and-credentials/🦀️.rs"]
pub(crate) mod wire_helpers_and_credentials;
use wire_helpers_and_credentials::{DIRECTORY_SCOPE_ID_MAX_BYTES, DOCUMENT_ADMISSION_RESPONSE_MAX_BYTES, LocalHubCredential, WipeBytes, WipeDocumentPlanSocketGrantIntent, decode_json_bytes, decode_lower_hex_32, directory_socket_hello_v1, encode_url_component, valid_socket_actor, valid_socket_grant, wipe_string};
#[cfg(all(feature = "ureq", feature = "sync", not(target_arch = "wasm32")))]
use wire_helpers_and_credentials::WipeSecretText;
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use wire_helpers_and_credentials::wall_now_ms;
#[cfg(not(all(target_arch = "wasm32", not(target_env = "p2"))))]
use wire_helpers_and_credentials::wall_now_ms;
#[cfg(not(target_arch = "wasm32"))]
use wire_helpers_and_credentials::{emit_document_socket_grant_probe, emit_socket_grant_probe};
#[cfg(target_arch = "wasm32")]
use wire_helpers_and_credentials::{emit_document_socket_grant_probe, emit_socket_grant_probe};

#[path = "📡️directory-client/🦀️.rs"]
pub(crate) mod directory_client;
use directory_client::{DirectoryClient, execution_target_refusal};

#[path = "🌊️directory-stream/🦀️.rs"]
pub(crate) mod directory_stream;
use directory_stream::{DirectoryStream, directory_scoped_ws_url, directory_ws_url};

//#region 🔖️Native
/// 🐎️ Native transport: HTTP routes through `semio-framework-os-services`'s `HttpPool`/
/// `ComputePool` (this crate's EXISTING optional `ureq` dep supplies the ONE blocking call
/// `HttpPool` admits onto its bounded pool — no per-call `std::thread::spawn` and no private
/// tokio runtime built by this crate), `tokio-tungstenite` for WS (this crate's EXISTING `sync`
/// feature, which already wires it up for `🏪️store/🔄️sync`'s native actor). Both features are
/// required together because a real client needs both halves; either alone would leave the other
/// method type unusable.
#[cfg(all(feature = "ureq", feature = "sync", not(target_arch = "wasm32")))]
#[path = "🖥️native-transport/🦀️.rs"]
pub mod native;

/// 🌉️ Browser wgpu build seam: `web_sys::WebSocket`/`fetch` transport, kept coherent with
/// `🏪️store/🔄️sync`'s own `wasm_actor` precedent (event-callback bridged into an `async`-shaped
/// interface via an unbounded channel) so a future in-wasm host can link this client. NOT the
/// production browser path today — the React shell uses lane 1-C's TypeScript `DirectoryClient`
/// (`🏪️store/🔄️sync`'s own module doc: "The production browser shell instead uses a TS twin").
// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too, and this bridge is browser-only,
// so it is narrowed to exclude the WASI component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🌉️browser-transport/🦀️.rs"]
pub mod browser;
//#endregion 🔖️Native

//#region 🌱️SpaceArtifactCreation
#[path = "🌱️space-artifact-creation/🦀️.rs"]
mod space_artifact_creation;

#[path = "📌️document-check-in/🦀️.rs"]
mod document_check_in;

#[path = "🪢️canonical-checkpoint-pair/🦀️.rs"]
mod canonical_checkpoint_pair;
pub use canonical_checkpoint_pair::{canonical_checkpoint_pair_path, CANONICAL_CHECKPOINT_PAIR_TRANSIENT_ATTEMPTS, CANONICAL_CHECKPOINT_PAIR_TRANSIENT_STATUSES};
//#endregion 🌱️SpaceArtifactCreation

//#region 🤖️AgentDelegations
#[path = "🤖️agent-delegations/🦀️.rs"]
pub mod agent_delegations;
//#endregion 🤖️AgentDelegations

//#region 🧩️ExecutionTargetModule
#[path = "🧩️execution-target-module/🦀️.rs"]
mod execution_target_module;
pub use execution_target_module::*;
//#endregion 🧩️ExecutionTargetModule

#[path = "🌐️document-http/🦀️.rs"]
pub mod document_http;
