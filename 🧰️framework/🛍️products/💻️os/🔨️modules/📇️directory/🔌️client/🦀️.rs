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
    DirectoryCommand, DirectoryCommandErrorCodeV1, DirectoryCommandReceiptV1, DirectoryCommandRequestV1, DirectoryEvent, DirectoryEventPageV1, DirectorySpaceAdministrationCapabilitiesV1, DirectorySpaceAdministrationDocumentWindowV1, DirectorySpaceAdministrationInviteWindowV1, DirectorySpaceAdministrationMemberWindowV1, DirectorySpaceAdministrationPageV1, DirectorySpaceListEntryV1, DirectorySpaceRole, DocumentView, MemberSpaceViewV1, DirectoryStreamMessage, DocumentExecutionTargetComponentV1, DocumentExecutionTargetDescriptorV1, DocumentExecutionTargetLeaseFieldsV1, DocumentOpenArtifactV1, DocumentOpenCatalogV1, DocumentOpenCheckpointV1, DocumentOpenGrantV1, DocumentOpenIntentV1,
    DocumentOpenPackageV1, DocumentOpenParentDialectV1, DocumentOpenPlanV1, DocumentOpenRendererTargetV1, DocumentOpenRevalidationV1, DocumentOpenSurfaceRoleV1, DocumentOpenSurfaceV1,
    DocumentPlanSocketGrantIntentV1, DocumentScope, GisMapInferenceApprovalReceiptV1, GisMapInferenceApprovalRequestV1, GisMapInferenceEventPageV1, GisMapInferenceJobReceiptV1, GisMapInferenceJobRequestV1, GisMapInferencePortCodeV1, GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR, GIS_MAP_INFERENCE_REQUEST_MAX_BYTES, GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES, lease_fields_from_plan_v1, same_lease_fields_v1, DIRECTORY_COMMAND_RECEIPT_MAX_BYTES, DIRECTORY_EVENT_PAGE_MAX_BYTES, DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES, DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES, DOCUMENT_OPEN_MAX_SAFE_INTEGER,
};
use crate::os_dsl::{DslValue, FromValue, ToValue, ValueError};
use semio_framework_async::OperationContext;
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::Arc;

//#region 🔖️Transport
/// 📨️ One HTTP verb `DirectoryClient` issues against the hub REST surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Delete,
}

/// 📬️ A transport-agnostic HTTP response: status code plus raw body bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

/// ⚠️ A transport-layer failure (connection refused, DNS, closed socket, …) — distinct from an
/// HTTP error status (that decodes to `DirectoryClientError::Http`/`Unauthorized`) or a JSON
/// decode failure (`DirectoryClientError::Decode`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransportError {
    Io(String),
    /// 🛑️ The `OperationContext` handed to this call was already cancelled, or was cancelled
    /// before the call finished — a transport checks this itself (rather than relying solely on
    /// the caller's own pre-check) so an in-flight call started just before cancellation still
    /// surfaces it instead of returning a stale success.
    Cancelled,
    /// ⏰️ `ctx.deadline_ms` elapsed before this call could complete.
    DeadlineExceeded,
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(detail) => write!(formatter, "transport io: {detail}"),
            Self::Cancelled => formatter.write_str("cancelled"),
            Self::DeadlineExceeded => formatter.write_str("deadline exceeded"),
        }
    }
}

impl std::error::Error for TransportError {}

/// 🧵️ Native transports cross worker turns; browser transports remain local to the wasm event loop.
#[cfg(not(target_arch = "wasm32"))]
pub trait DirectoryTransportPlatform: Send + Sync {}

#[cfg(not(target_arch = "wasm32"))]
impl<T: Send + Sync> DirectoryTransportPlatform for T {}

#[cfg(target_arch = "wasm32")]
pub trait DirectoryTransportPlatform {}

#[cfg(target_arch = "wasm32")]
impl<T> DirectoryTransportPlatform for T {}

/// 🧵️ Native connections move between finite worker turns; browser sockets remain event-loop local.
#[cfg(not(target_arch = "wasm32"))]
pub trait DirectoryConnectionPlatform: Send {}

#[cfg(not(target_arch = "wasm32"))]
impl<T: Send> DirectoryConnectionPlatform for T {}

#[cfg(target_arch = "wasm32")]
pub trait DirectoryConnectionPlatform {}

#[cfg(target_arch = "wasm32")]
impl<T> DirectoryConnectionPlatform for T {}

/// 🔌️ The injection seam: no concrete HTTP client type may appear in any public signature
/// outside an implementor of this trait. Native `ureq`, browser `web_sys::fetch`, or a test
/// double all implement this identically. Native transports are `Send + Sync` and native HTTP
/// futures are `Send`; wasm uses a platform-local marker because `JsValue` handles cannot move.
/// `ctx` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME) carries cancellation/deadline/
/// trace/capability through to whichever concrete transport is wired — a native call routes it
/// into `semio-framework-os-services`'s `HttpPool`/`ComputePool` (see `🔖️Native` below), so the
/// SAME `OperationContext` that gates `SpaceRunner::compute_node`'s `exchange` calls
/// (`🏃️run/🦀️.rs`) also gates a directory request, rather than each path inventing its
/// own bolted-on cancellation.
// 🔀️ dedyn-fw-os-misc, O1/R11(b): `DirectoryTransport` is already generic (never `dyn`, see
// `DirectoryClient<T: DirectoryTransport>` below) — `open_ws` is a trait method that RETURNS a
// runtime-chosen `DirectoryWsConnection` implementation, so the associated type `Ws` pushes that
// choice to the implementor exactly per the `ResourceResolver` precedent, replacing
// `Box<dyn DirectoryWsConnection>`.
pub trait DirectoryTransport: DirectoryTransportPlatform {
    type Ws: DirectoryWsConnection;
    async fn http(&self, ctx: &OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError>;
    fn issue_socket_grant(&self, ctx: &OperationContext, url: &str, bearer: &str, body: &[u8], timeout_ms: u64) -> Result<HttpResponse, TransportError>;
    fn open_ws(&self, ctx: &OperationContext, url: &str, protocols: &[String], timeout_ms: u64) -> Result<Self::Ws, TransportError>;
}

/// 🔌️ One open `/directory/socket/v1` connection. Sequential by construction (`DirectoryStream` never
/// calls `send_text`/`try_recv_text` concurrently), so a single stream/sink object suffices — no
/// split halves needed the way `🏪️store/🔄️sync`'s bidirectional relay requires.
pub trait DirectoryWsConnection: DirectoryConnectionPlatform {
    fn send_text(&mut self, text: String) -> Result<(), TransportError>;
    fn send_binary(&mut self, bytes: Vec<u8>) -> Result<(), TransportError>;
    /// 📭️ Nonblocking receive: `Pending` yields the worker immediately; `Closed` reconnects.
    fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError>;
    fn close(&mut self);
}

/// 📬️ One nonblocking WebSocket receive observation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DirectoryWsPoll {
    Text(String),
    Pending,
    Closed(Option<u16>),
}
//#endregion 🔖️Transport

//#region 🔖️Errors
#[derive(Debug)]
pub enum DirectoryClientError {
    Transport(TransportError),
    Decode(String),
    Unauthorized,
    Http {
        status: u16,
        body: String,
    },
    /// 🛑️ `ctx.cancel` was already cancelled BEFORE this call ever reached the transport — checked
    /// in `DirectoryClient::request_json`/`DirectoryStream::recv` up front, so a cancelled caller
    /// never even builds a request. `TransportError::Cancelled` (via the `Transport` variant above)
    /// is the other half: cancelled WHILE the transport call was in flight.
    Cancelled,
}

impl std::fmt::Display for DirectoryClientError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Transport(error) => std::fmt::Display::fmt(error, formatter),
            Self::Decode(error) => write!(formatter, "decode: {error}"),
            Self::Unauthorized => formatter.write_str("unauthorized"),
            Self::Http { status, body } => write!(formatter, "http {status}: {body}"),
            Self::Cancelled => formatter.write_str("cancelled"),
        }
    }
}

impl std::error::Error for DirectoryClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Transport(error) => Some(error),
            _ => None,
        }
    }
}

impl From<TransportError> for DirectoryClientError {
    fn from(error: TransportError) -> Self {
        Self::Transport(error)
    }
}

impl From<ValueError> for DirectoryClientError {
    fn from(error: ValueError) -> Self {
        Self::Decode(error.to_string())
    }
}
//#endregion 🔖️Errors

//#region 🔖️Wire
/// 📮️ One canonical `POST /directory/commands` completion, preserving the exact response bytes the
/// receipt digest covers. The public command boundary is closed: no `DslValue` and no open
/// `unknown` crosses it.
#[derive(Clone, Debug, PartialEq)]
pub struct CanonicalDirectoryCommandReceiptV1 {
    pub canonical_json: String,
    pub receipt: DirectoryCommandReceiptV1,
}

/// 🪪️ `GET /auth/sessions/me`'s body (contract §C2, camelCase — this route is NEW this wave).
#[derive(Clone, Debug, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct SessionView {
    pub user_id: String,
    pub email: String,
    pub display_name: String,
    #[value(rename = "expiresAt")]
    pub expires_at_ms: i64,
}

/// 🎫️ `POST /auth/sessions`'s body. Wire is snake_case (`token`, `user_id`), NOT this contract's
/// general camelCase convention: this route predates the wave (`🌎️hub/📦️bin.rs`'s
/// `CreateAuthSessionResponse` has no `rename_all`) and §C2 marks it "unchanged" — the client
/// matches the ACTUAL wire, not the convention.
#[derive(Clone, Debug, ToValue, FromValue)]
pub struct SessionMintResponse {
    pub token: String,
    pub user_id: String,
}

/// 🏛️ One administration page plus the exact response bytes its receipt covers.
#[derive(Clone, Debug, PartialEq)]
pub struct CanonicalDirectorySpaceAdministrationPageV1 {
    canonical_json: String,
    page: DirectorySpaceAdministrationPageV1,
}

impl CanonicalDirectorySpaceAdministrationPageV1 {
    pub fn canonical_json(&self) -> &str { &self.canonical_json }
    pub fn page(&self) -> &DirectorySpaceAdministrationPageV1 { &self.page }
    pub fn receipt_sha256(&self) -> &str { self.page.receipt_sha256() }
}

/// 📄️ Immutable original page bytes plus only the authenticated header needed for ACK ordering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalDirectoryEventPageV1 {
    canonical_json: String,
    session_binding_sha256: String,
    authorization_generation: u64,
    after_seq_exclusive: u64,
    through_seq_inclusive: u64,
    has_more: bool,
    receipt_sha256: String,
}

impl CanonicalDirectoryEventPageV1 {
    pub fn canonical_json(&self) -> &str { &self.canonical_json }
    pub fn session_binding_sha256(&self) -> &str { &self.session_binding_sha256 }
    pub fn authorization_generation(&self) -> u64 { self.authorization_generation }
    pub fn after_seq_exclusive(&self) -> u64 { self.after_seq_exclusive }
    pub fn through_seq_inclusive(&self) -> u64 { self.through_seq_inclusive }
    pub fn has_more(&self) -> bool { self.has_more }
    pub fn receipt_sha256(&self) -> &str { &self.receipt_sha256 }

    /// ✅️ Captures the exact non-secret header a retained Home action must acknowledge.
    pub fn acknowledgement(&self, bootstrap_epoch: u64) -> DirectoryEventPageAckV1 {
        DirectoryEventPageAckV1 {
            bootstrap_epoch,
            session_binding_sha256: self.session_binding_sha256.clone(),
            authorization_generation: self.authorization_generation,
            through_seq_inclusive: self.through_seq_inclusive,
            receipt_sha256: self.receipt_sha256.clone(),
        }
    }
}

/// 🧾️ Exact retained-Home acknowledgement for one still-owned directory page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DirectoryEventPageAckV1 {
    pub bootstrap_epoch: u64,
    pub session_binding_sha256: String,
    pub authorization_generation: u64,
    pub through_seq_inclusive: u64,
    pub receipt_sha256: String,
}

/// 🚦️ Next I/O boundary after one accepted page acknowledgement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectoryBootstrapTransition {
    Fetch { after: u64 },
    Live { since: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DirectoryBootstrapPhase {
    Fetching,
    AwaitingAck,
    Live,
    Closed,
}

/// 🧭️ Sole native-shell owner of the fetch → retained Home ACK → live cursor.
pub struct DirectoryEventPageBootstrapV1 {
    bootstrap_epoch: u64,
    acknowledged_through: u64,
    pending: Option<CanonicalDirectoryEventPageV1>,
    phase: DirectoryBootstrapPhase,
}

impl DirectoryEventPageBootstrapV1 {
    pub fn new(bootstrap_epoch: u64, after: u64) -> Result<Self, DirectoryClientError> {
        if bootstrap_epoch > DOCUMENT_OPEN_MAX_SAFE_INTEGER || after > DOCUMENT_OPEN_MAX_SAFE_INTEGER {
            return Err(DirectoryClientError::Decode("directory bootstrap owner is not wire-safe".into()));
        }
        Ok(Self { bootstrap_epoch, acknowledged_through: after, pending: None, phase: DirectoryBootstrapPhase::Fetching })
    }

    pub fn bootstrap_epoch(&self) -> u64 {
        self.bootstrap_epoch
    }

    pub fn after(&self) -> u64 {
        self.acknowledged_through
    }

    pub fn present(&mut self, page: CanonicalDirectoryEventPageV1) -> Result<DirectoryEventPageAckV1, DirectoryClientError> {
        if self.phase != DirectoryBootstrapPhase::Fetching || page.after_seq_exclusive != self.acknowledged_through || page.through_seq_inclusive < self.acknowledged_through {
            return Err(DirectoryClientError::Decode("directory bootstrap page ordering mismatch".into()));
        }
        let acknowledgement = page.acknowledgement(self.bootstrap_epoch);
        self.pending = Some(page);
        self.phase = DirectoryBootstrapPhase::AwaitingAck;
        Ok(acknowledgement)
    }

    pub fn acknowledge(&mut self, acknowledgement: &DirectoryEventPageAckV1) -> Result<DirectoryBootstrapTransition, DirectoryClientError> {
        let page = self.pending.as_ref().ok_or_else(|| DirectoryClientError::Decode("directory bootstrap has no pending page".into()))?;
        if self.phase != DirectoryBootstrapPhase::AwaitingAck
            || acknowledgement.bootstrap_epoch != self.bootstrap_epoch
            || acknowledgement.session_binding_sha256 != page.session_binding_sha256
            || acknowledgement.authorization_generation != page.authorization_generation
            || acknowledgement.through_seq_inclusive != page.through_seq_inclusive
            || acknowledgement.receipt_sha256 != page.receipt_sha256
        {
            return Err(DirectoryClientError::Decode("directory bootstrap acknowledgement mismatch".into()));
        }
        self.acknowledged_through = page.through_seq_inclusive;
        let has_more = page.has_more;
        self.pending = None;
        self.phase = if has_more { DirectoryBootstrapPhase::Fetching } else { DirectoryBootstrapPhase::Live };
        Ok(if has_more { DirectoryBootstrapTransition::Fetch { after: self.acknowledged_through } } else { DirectoryBootstrapTransition::Live { since: self.acknowledged_through } })
    }

    pub fn reject(&mut self, bootstrap_epoch: u64, receipt_sha256: &str) -> Result<u64, DirectoryClientError> {
        let page = self.pending.as_ref().ok_or_else(|| DirectoryClientError::Decode("directory bootstrap has no pending page".into()))?;
        if self.phase != DirectoryBootstrapPhase::AwaitingAck || bootstrap_epoch != self.bootstrap_epoch || receipt_sha256 != page.receipt_sha256 {
            return Err(DirectoryClientError::Decode("directory bootstrap rejection mismatch".into()));
        }
        self.pending = None;
        self.phase = DirectoryBootstrapPhase::Fetching;
        Ok(self.acknowledged_through)
    }

    pub fn wake(&mut self, rebootstrap: bool) -> Option<u64> {
        if self.phase != DirectoryBootstrapPhase::Live {
            return None;
        }
        if rebootstrap {
            self.acknowledged_through = 0;
        }
        self.phase = DirectoryBootstrapPhase::Fetching;
        Some(self.acknowledged_through)
    }

    pub fn close(&mut self) {
        self.pending = None;
        self.phase = DirectoryBootstrapPhase::Closed;
    }
}

#[derive(FromValue)]
#[value(rename_all = "camelCase")]
pub struct SocketGrantReceiptV1 {
    pub schema: String,
    pub protocol: String,
    pub grant: String,
    pub actor_id: String,
    pub expires_at_ms: i64,
}

impl Drop for SocketGrantReceiptV1 {
    fn drop(&mut self) {
        wipe_string(&mut self.grant);
    }
}

/// 🔐 Receipt-free server-selected authority retained by a native document connection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentSocketAuthorityV1 {
    pub hub_origin: String,
    pub expires_at_unix_ms: u64,
    pub scope: DocumentScope,
    pub descriptor_digest_v1: String,
    pub catalog: DocumentOpenCatalogV1,
    pub package: DocumentOpenPackageV1,
    pub artifact: DocumentOpenArtifactV1,
    pub parent_dialect: DocumentOpenParentDialectV1,
    pub pack_schema_hash: [u8; 32],
    pub surface: DocumentOpenSurfaceV1,
    pub grant: DocumentOpenGrantV1,
    pub checkpoint: Option<DocumentOpenCheckpointV1>,
    pub revalidation: DocumentOpenRevalidationV1,
}

impl DocumentSocketAuthorityV1 {
    /// 🧾 Receipt-free lease projection of this retained authority at the byte lengths under
    /// comparison. The plan constrains every identity but no byte length.
    pub fn lease_fields(&self, component_byte_length: u64, descriptor_byte_length: u64) -> DocumentExecutionTargetLeaseFieldsV1 {
        DocumentExecutionTargetLeaseFieldsV1 {
            schema: "semio.os.document-execution-target-lease/v1".to_string(),
            version: 1,
            scope: self.scope.clone(),
            descriptor_digest_v1: self.descriptor_digest_v1.clone(),
            catalog: self.catalog.clone(),
            package: self.package.clone(),
            component: DocumentExecutionTargetComponentV1 { sha256: self.package.component_sha256.clone(), blake3: self.package.component_blake3.clone(), byte_length: component_byte_length },
            descriptor: DocumentExecutionTargetDescriptorV1 { sha256: self.package.descriptor_byte_sha256.clone(), byte_length: descriptor_byte_length },
            artifact: self.artifact.clone(),
            parent_dialect: self.parent_dialect.clone(),
            surface: self.surface.clone(),
            grant: self.grant,
            checkpoint: self.checkpoint.clone(),
            revalidation: self.revalidation,
        }
    }

    /// ⚖️ The one shared full-field lease relation, replacing every partial surface predicate: it
    /// compares both scope ids, the descriptor digest, catalog generation, all three package
    /// digests, both byte lengths, artifact, parent dialect, every surface field, every grant bit,
    /// the checkpoint and every revalidation generation.
    pub fn matches_lease_fields(&self, expected: &DocumentExecutionTargetLeaseFieldsV1) -> bool {
        same_lease_fields_v1(&self.lease_fields(expected.component.byte_length, expected.descriptor.byte_length), expected)
    }
}

/// 🎫 One exchanged socket grant and its receipt-free open authority.
pub struct DocumentSocketAdmissionV1 {
    pub socket: SocketGrantReceiptV1,
    pub authority: DocumentSocketAuthorityV1,
}

/// 🧬 Local codec expectation plus the complete verified execution-target lease a server-authoritative
/// open plan must satisfy. There is no partial surface expectation: a locally installed target is
/// compared through the one shared full-field relation or not at all.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct DocumentSocketExpectationV1 {
    pub artifact_schema: String,
    pub pack_schema_hash: [u8; 32],
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub requested_surface_id: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub lease: Option<DocumentExecutionTargetLeaseFieldsV1>,
}

pub trait HubSocketGrantSource: Send + Sync {
    fn admit_document_socket(
        &self,
        ctx: &OperationContext,
        space_id: &str,
        document_id: &str,
        expectation: &DocumentSocketExpectationV1,
        client_instance_id: &str,
        timeout_ms: u64,
    ) -> Result<DocumentSocketAdmissionV1, DirectoryClientError>;
}
//#endregion 🔖️Wire

/// 🔤️ Wire bytes → `FromValue` type, via `pack::json` rather than `serde_json` — the one decode
/// choke point every `request_json` response body passes through.
fn decode_json_bytes<R: FromValue>(bytes: &[u8]) -> Result<R, DirectoryClientError> {
    let text = std::str::from_utf8(bytes).map_err(|error| DirectoryClientError::Decode(error.to_string()))?;
    crate::os_pack::json::from_json_str(text).map_err(DirectoryClientError::from)
}

fn wipe_string(value: &mut String) -> usize {
    let len = value.len();
    unsafe { value.as_mut_vec().fill(0) };
    len
}

struct WipeDocumentPlanSocketGrantIntent(DocumentPlanSocketGrantIntentV1);

impl Drop for WipeDocumentPlanSocketGrantIntent {
    fn drop(&mut self) {
        wipe_string(&mut self.0.plan_receipt);
    }
}

fn lower_hex(bytes: &[u8]) -> bool {
    bytes.iter().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}

fn valid_socket_grant(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 107 && bytes.get(..10) == Some(b"socket.v1.".as_slice()) && bytes.get(10..42).is_some_and(lower_hex) && bytes.get(42) == Some(&b'.') && bytes.get(43..107).is_some_and(lower_hex)
}

fn valid_socket_actor(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 71 && bytes.get(..7) == Some(b"hub.v1.".as_slice()) && bytes.get(7..71).is_some_and(lower_hex)
}

fn decode_lower_hex_32(value: &str) -> Option<[u8; 32]> {
    if value.len() != 64 || !lower_hex(value.as_bytes()) {
        return None;
    }
    let digit = |byte| match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    };
    let mut decoded = [0u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        decoded[index] = digit(pair[0])? << 4 | digit(pair[1])?;
    }
    Some(decoded)
}

/// 🔗 UTF-8 byte percent-encoding shared by protected HTTP scope and document WebSocket URLs.
pub fn encode_url_component(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(char::from(byte));
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}

const DOCUMENT_ADMISSION_RESPONSE_MAX_BYTES: usize = 64 * 1024;
const DIRECTORY_SCOPE_ID_MAX_BYTES: usize = 4 * 1024;

#[cfg(not(target_arch = "wasm32"))]
fn emit_socket_grant_probe(path: &str, grant: &str) {
    if std::env::var("SEMIO_DIRECT_CHILD_PROBE").ok().as_deref() != Some("1") {
        return;
    }
    let digest = semio_framework_hash::Sha256::digest(&grant.as_bytes()[10..42]);
    eprintln!("[semio-directory-client] socket-grant-selector-digest {path} {}", digest.iter().map(|byte| format!("{byte:02x}")).collect::<String>());
}

#[cfg(target_arch = "wasm32")]
fn emit_socket_grant_probe(_path: &str, _grant: &str) {}

fn wall_now_ms() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| i64::try_from(duration.as_millis()).unwrap_or(i64::MAX))
}

fn directory_socket_hello_v1() -> Vec<u8> {
    let schema = b"semio.directory.events/v1";
    let mut bytes = Vec::with_capacity(5 + schema.len() + 34);
    bytes.extend_from_slice(&[0, 7, 1, 1, schema.len() as u8]);
    bytes.extend_from_slice(schema);
    bytes.extend_from_slice(&[0; 32]);
    bytes.extend_from_slice(&[0, 0]);
    bytes
}

//#region 🔖️Client
/// 📇️ Talks the hub's directory REST/WS surface over an injected `DirectoryTransport`. Holds the
/// protected local credential and socket-grant authority without serializing either into bindings.
pub struct LocalHubCredential {
    hub_origin: String,
    capability: Box<[u8]>,
}

struct WipeBytes<'a> {
    bytes: Vec<u8>,
    observer: Option<&'a std::sync::atomic::AtomicUsize>,
}

struct WipeSecretText(String);

impl Drop for WipeSecretText {
    fn drop(&mut self) {
        wipe_string(&mut self.0);
    }
}

impl Drop for WipeBytes<'_> {
    fn drop(&mut self) {
        let len = self.bytes.len();
        self.bytes.fill(0);
        if let Some(observer) = self.observer {
            observer.fetch_add(self.bytes.iter().filter(|byte| **byte == 0).count().min(len), std::sync::atomic::Ordering::SeqCst);
        }
    }
}

fn read_local_hub_credential_frame<'a, R: std::io::Read>(pipe: &mut R, observer: Option<&'a std::sync::atomic::AtomicUsize>) -> Result<WipeBytes<'a>, DirectoryClientError> {
    let mut length = [0u8; 4];
    pipe.read_exact(&mut length).map_err(|_| DirectoryClientError::Unauthorized)?;
    let length = u32::from_be_bytes(length) as usize;
    if length == 0 || length > 16 * 1024 {
        return Err(DirectoryClientError::Unauthorized);
    }
    let mut bytes = WipeBytes { bytes: vec![0u8; length], observer };
    pipe.read_exact(&mut bytes.bytes).map_err(|_| DirectoryClientError::Unauthorized)?;
    let mut trailing = WipeBytes { bytes: vec![0u8; 1], observer };
    if pipe.read(&mut trailing.bytes).map_err(|_| DirectoryClientError::Unauthorized)? != 0 {
        return Err(DirectoryClientError::Unauthorized);
    }
    Ok(bytes)
}

struct WipeDslValue<'a> {
    value: DslValue,
    observer: Option<&'a std::sync::atomic::AtomicUsize>,
}

impl Drop for WipeDslValue<'_> {
    fn drop(&mut self) {
        let wiped = wipe_dsl_value(&mut self.value);
        if let Some(observer) = self.observer {
            observer.fetch_add(wiped, std::sync::atomic::Ordering::SeqCst);
        }
    }
}

fn wipe_dsl_value(value: &mut DslValue) -> usize {
    match value {
        DslValue::String(value) => wipe_string(value),
        DslValue::Array(values) => values.iter_mut().map(wipe_dsl_value).sum(),
        DslValue::Object(entries) => entries.iter_mut().map(|(key, value)| wipe_string(key) + wipe_dsl_value(value)).sum(),
        DslValue::Null | DslValue::Bool(_) | DslValue::Number(_) => 0,
    }
}

fn decode_local_hub_credential(bytes: &[u8], expected_class: &str, observer: Option<&std::sync::atomic::AtomicUsize>) -> Result<LocalHubCredential, DirectoryClientError> {
    let mut decoded = WipeDslValue { value: decode_json_bytes::<DslValue>(bytes)?, observer };
    let field = |name: &str| decoded.value.get(name).and_then(|value| value.as_str()).ok_or(DirectoryClientError::Unauthorized);
    let schema = field("schema")?;
    let client_class = field("clientClass")?;
    let hub_origin = field("hubOrigin")?;
    let capability = field("capability")?;
    let expires_at_ms = decoded.value.get("expiresAtMs").and_then(|value| value.as_i64()).ok_or(DirectoryClientError::Unauthorized)?;
    if schema != "semio.local.consumer-credential/v1"
        || client_class != expected_class
        || !hub_origin.strip_prefix("http://127.0.0.1:").is_some_and(|port| port.parse::<u16>().is_ok())
        || expires_at_ms <= wall_now_ms()
        || !valid_session_capability(capability)
    {
        return Err(DirectoryClientError::Unauthorized);
    }
    let hub_origin = hub_origin.to_string();
    let DslValue::Object(entries) = &mut decoded.value else { return Err(DirectoryClientError::Unauthorized) };
    let capability = entries
        .iter_mut()
        .find_map(|(key, value)| (key == "capability").then_some(value))
        .and_then(|value| match value {
            DslValue::String(value) => Some(std::mem::take(value).into_bytes().into_boxed_slice()),
            _ => None,
        })
        .ok_or(DirectoryClientError::Unauthorized)?;
    Ok(LocalHubCredential { hub_origin, capability })
}

impl std::fmt::Debug for LocalHubCredential {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("LocalHubCredential").field("hub_origin", &self.hub_origin).field("capability", &"<redacted>").finish()
    }
}

impl Drop for LocalHubCredential {
    fn drop(&mut self) {
        self.capability.fill(0);
    }
}

impl LocalHubCredential {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_inherited(expected_class: &str) -> Result<Self, DirectoryClientError> {
        #[cfg(unix)]
        let mut pipe = {
            use std::os::fd::FromRawFd;
            unsafe extern "C" {
                fn fcntl(fd: i32, command: i32, ...) -> i32;
            }
            const F_GETFD: i32 = 1;
            const F_SETFD: i32 = 2;
            const FD_CLOEXEC: i32 = 1;
            let flags = unsafe { fcntl(3, F_GETFD) };
            if flags < 0 || unsafe { fcntl(3, F_SETFD, flags | FD_CLOEXEC) } < 0 {
                return Err(DirectoryClientError::Unauthorized);
            }
            unsafe { std::fs::File::from_raw_fd(3) }
        };
        #[cfg(windows)]
        let mut pipe = inherited_windows_file()?;
        let bytes = read_local_hub_credential_frame(&mut pipe, None)?;
        decode_local_hub_credential(&bytes.bytes, expected_class, None)
    }

    #[cfg(test)]
    pub(crate) fn test(hub_origin: &str, capability: &str) -> Self {
        Self { hub_origin: hub_origin.to_string(), capability: capability.as_bytes().into() }
    }

    pub(crate) fn capability(&self) -> Result<&str, DirectoryClientError> {
        std::str::from_utf8(&self.capability).map_err(|_| DirectoryClientError::Unauthorized)
    }

    pub fn hub_origin(&self) -> &str {
        &self.hub_origin
    }

    pub fn authorizes_capability(&self, candidate: &str) -> bool {
        if candidate.len() != self.capability.len() {
            return false;
        }
        candidate.as_bytes().iter().zip(self.capability.iter()).fold(0u8, |difference, (left, right)| difference | (left ^ right)) == 0
    }

    pub fn authorizes_bearer_header(&self, header: &str) -> bool {
        header.strip_prefix("Bearer ").is_some_and(|candidate| self.authorizes_capability(candidate))
    }
}

fn valid_session_capability(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 108 && bytes.get(..11) == Some(b"session.v1.".as_slice()) && bytes.get(11..43).is_some_and(lower_hex) && bytes.get(43) == Some(&b'.') && bytes.get(44..108).is_some_and(lower_hex)
}

#[cfg(windows)]
fn inherited_windows_file() -> Result<std::fs::File, DirectoryClientError> {
    use std::ffi::c_void;
    use std::os::windows::io::FromRawHandle;
    type Handle = *mut c_void;
    unsafe extern "C" {
        fn _get_osfhandle(fd: i32) -> isize;
        fn _close(fd: i32) -> i32;
        fn GetCurrentProcess() -> Handle;
        fn DuplicateHandle(source_process: Handle, source: Handle, target_process: Handle, target: *mut Handle, access: u32, inherit: i32, options: u32) -> i32;
        fn CloseHandle(handle: Handle) -> i32;
    }
    let source = unsafe { _get_osfhandle(3) };
    if source == -1 {
        return Err(DirectoryClientError::Unauthorized);
    }
    let process = unsafe { GetCurrentProcess() };
    let mut duplicate: Handle = std::ptr::null_mut();
    let duplicated = unsafe { DuplicateHandle(process, source as Handle, process, &mut duplicate, 0, 0, 2) } != 0 && !duplicate.is_null();
    let closed = unsafe { _close(3) } == 0;
    if !duplicated || !closed {
        if duplicated {
            unsafe { CloseHandle(duplicate) };
        }
        return Err(DirectoryClientError::Unauthorized);
    }
    Ok(unsafe { std::fs::File::from_raw_handle(duplicate) })
}

pub struct DirectoryClient<T: DirectoryTransport> {
    transport: T,
    base_url: String,
    credential: Option<Arc<LocalHubCredential>>,
}

impl<T: DirectoryTransport> DirectoryClient<T> {
    pub fn new(transport: T, base_url: impl Into<String>) -> Self {
        Self { transport, base_url: base_url.into(), credential: None }
    }

    pub fn authenticated(transport: T, credential: Arc<LocalHubCredential>) -> Self {
        Self { transport, base_url: credential.hub_origin.clone(), credential: Some(credential) }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url.trim_end_matches('/'), path)
    }

    /// 🛑️ Checked BEFORE every call reaches `self.transport` — an already-cancelled `ctx` never
    /// builds a request at all (`TransportError::Cancelled`, via the SAME transport call, is the
    /// other half: cancelled WHILE the call was in flight — see that variant's own doc).
    async fn request_json<R: FromValue>(&self, ctx: &OperationContext, method: HttpMethod, path: &str, body: Option<Vec<u8>>) -> Result<R, DirectoryClientError> {
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
        let response = self.transport.http(ctx, method, &self.url(path), bearer, body).await?;
        match response.status {
            200..=299 => Ok(decode_json_bytes(&response.body)?),
            401 => Err(DirectoryClientError::Unauthorized),
            status => Err(DirectoryClientError::Http { status, body: String::from_utf8_lossy(&response.body).into_owned() }),
        }
    }

    /// 🛡️ Bounded request/response variant used by the closed command boundary: it caps raw bytes
    /// BEFORE decoding and maps every failure into a closed code, never a raw server body.
    async fn request_bytes_limited(&self, ctx: &OperationContext, method: HttpMethod, path: &str, body: Option<Vec<u8>>, response_max_bytes: usize) -> Result<Vec<u8>, DirectoryCommandErrorCodeV1> {
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryCommandErrorCodeV1::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose().map_err(|_| DirectoryCommandErrorCodeV1::Unauthorized)?;
        let response = match self.transport.http(ctx, method, &self.url(path), bearer, body).await {
            Ok(response) => response,
            Err(TransportError::Cancelled) => return Err(DirectoryCommandErrorCodeV1::Cancelled),
            Err(_) => return Err(DirectoryCommandErrorCodeV1::Transport),
        };
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryCommandErrorCodeV1::Cancelled);
        }
        if !(200..=299).contains(&response.status) {
            return Err(DirectoryCommandErrorCodeV1::from_status(response.status));
        }
        if response.body.len() > response_max_bytes {
            return Err(DirectoryCommandErrorCodeV1::TooLarge);
        }
        Ok(response.body)
    }

    pub async fn spaces(&self, ctx: &OperationContext) -> Result<Vec<DirectorySpaceListEntryV1>, DirectoryClientError> {
        let spaces: Vec<DirectorySpaceListEntryV1> = self.request_json(ctx, HttpMethod::Get, "/directory/spaces", None).await?;
        if spaces.iter().all(DirectorySpaceListEntryV1::validate) {
            Ok(spaces)
        } else {
            Err(DirectoryClientError::Decode("directory space list access discriminator mismatch".into()))
        }
    }

    /// 🏛️ Fetches one bounded canonical administration page for exactly one space. `cursor`
    /// advances precisely the window it was issued for; every other window restarts at its head.
    pub async fn space_administration_page(&self, ctx: &OperationContext, id: &str, cursor: Option<&str>) -> Result<CanonicalDirectorySpaceAdministrationPageV1, DirectoryClientError> {
        if let Some(cursor) = cursor {
            if cursor.is_empty() || cursor.len() > DIRECTORY_SPACE_ADMINISTRATION_CURSOR_MAX_BYTES || !cursor.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_')) {
                return Err(DirectoryClientError::Decode("directory space administration cursor is not opaque-safe".into()));
            }
        }
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
        let path = match cursor {
            Some(cursor) => format!("/directory/spaces/{id}?cursor={cursor}"),
            None => format!("/directory/spaces/{id}"),
        };
        let response = self.transport.http(ctx, HttpMethod::Get, &self.url(&path), bearer, None).await?;
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        match response.status {
            401 => return Err(DirectoryClientError::Unauthorized),
            200..=299 => {}
            status => return Err(DirectoryClientError::Http { status, body: String::from_utf8_lossy(&response.body).into_owned() }),
        }
        if response.body.len() > DIRECTORY_SPACE_ADMINISTRATION_PAGE_MAX_BYTES {
            return Err(DirectoryClientError::Decode("directory space administration page response exceeded 48 KiB".into()));
        }
        let canonical_json = String::from_utf8(response.body).map_err(|error| DirectoryClientError::Decode(error.to_string()))?;
        let page = DirectorySpaceAdministrationPageV1::parse_canonical_json(&canonical_json).map_err(|_| DirectoryClientError::Decode("directory space administration page response is not canonical".into()))?;
        if page.space_id() != id {
            return Err(DirectoryClientError::Decode("directory space administration page space mismatch".into()));
        }
        Ok(CanonicalDirectorySpaceAdministrationPageV1 { canonical_json, page })
    }

    pub async fn events(&self, ctx: &OperationContext, since: u64) -> Result<Vec<DirectoryEvent>, DirectoryClientError> {
        self.request_json(ctx, HttpMethod::Get, &format!("/directory/events?since={since}"), None).await
    }

    /// 📄️ Fetches one bounded canonical page while preserving the exact response bytes for Home.
    pub async fn event_page(&self, ctx: &OperationContext, after: u64) -> Result<CanonicalDirectoryEventPageV1, DirectoryClientError> {
        if after > DOCUMENT_OPEN_MAX_SAFE_INTEGER {
            return Err(DirectoryClientError::Decode("directory event page after frontier is not wire-safe".into()));
        }
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose()?;
        let path = format!("/directory/event-page/v1?after={after}");
        let response = self.transport.http(ctx, HttpMethod::Get, &self.url(&path), bearer, None).await?;
        if ctx.cancel.is_cancelled().await {
            return Err(DirectoryClientError::Cancelled);
        }
        match response.status {
            401 => return Err(DirectoryClientError::Unauthorized),
            200..=299 => {}
            status => return Err(DirectoryClientError::Http { status, body: String::from_utf8_lossy(&response.body).into_owned() }),
        }
        if response.body.len() > DIRECTORY_EVENT_PAGE_MAX_BYTES {
            return Err(DirectoryClientError::Decode("directory event page response exceeded 64 KiB".into()));
        }
        let canonical_json = String::from_utf8(response.body).map_err(|error| DirectoryClientError::Decode(error.to_string()))?;
        let page = DirectoryEventPageV1::parse_canonical_json(&canonical_json).map_err(|_| DirectoryClientError::Decode("directory event page response is not canonical".into()))?;
        if page.after_seq_exclusive != after {
            return Err(DirectoryClientError::Decode("directory event page response frontier mismatch".into()));
        }
        Ok(CanonicalDirectoryEventPageV1 {
            canonical_json,
            session_binding_sha256: page.session_binding_sha256,
            authorization_generation: page.authorization_generation,
            after_seq_exclusive: page.after_seq_exclusive,
            through_seq_inclusive: page.through_seq_inclusive,
            has_more: page.has_more,
            receipt_sha256: page.receipt_sha256,
        })
    }

    pub async fn me(&self, ctx: &OperationContext) -> Result<SessionView, DirectoryClientError> {
        self.request_json(ctx, HttpMethod::Get, "/auth/sessions/me", None).await
    }

    /// 🧾️ Posts one sealed V1 request and parses only a raw-byte-capped canonical receipt bound to
    /// it. Every non-2xx becomes a closed {@link DirectoryCommandErrorCodeV1}; the response body is
    /// never decoded into a UI-facing error. `ctx.cancel` cancels the HTTP wait only — a command
    /// already past its server linearization point stays committed and the operation is indeterminate.
    pub async fn command(&self, ctx: &OperationContext, request: &DirectoryCommandRequestV1) -> Result<CanonicalDirectoryCommandReceiptV1, DirectoryCommandErrorCodeV1> {
        request.validate()?;
        let body = request.canonical_json().into_bytes();
        let response = self.request_bytes_limited(ctx, HttpMethod::Post, "/directory/commands", Some(body), DIRECTORY_COMMAND_RECEIPT_MAX_BYTES).await?;
        let canonical_json = String::from_utf8(response).map_err(|_| DirectoryCommandErrorCodeV1::Invalid)?;
        let receipt = DirectoryCommandReceiptV1::parse_canonical_json(&canonical_json, request)?;
        Ok(CanonicalDirectoryCommandReceiptV1 { canonical_json, receipt })
    }

    //#region 💡️InferencePort
    /// 🚪 The only four inference calls this client may make, each rebuilt from the caller's own
    /// document scope. No package, digest, generation, path or receipt selector is accepted, so a
    /// caller can never widen the request beyond the document it already holds.
    fn gis_map_inference_path(scope: &DocumentScope, suffix: &str) -> String {
        format!("/spaces/{}/documents/{}/inference/gis-map{}", encode_url_component(&scope.space_id), encode_url_component(&scope.document_id), suffix)
    }

    async fn gis_map_inference_call<R: FromValue>(&self, ctx: &OperationContext, method: HttpMethod, scope: &DocumentScope, suffix: &str, body: Option<Vec<u8>>) -> Result<R, GisMapInferencePortCodeV1> {
        if body.as_ref().is_some_and(|bytes| bytes.len() > GIS_MAP_INFERENCE_REQUEST_MAX_BYTES) {
            return Err(GisMapInferencePortCodeV1::Bounds);
        }
        if ctx.cancel.is_cancelled().await {
            return Err(GisMapInferencePortCodeV1::Cancelled);
        }
        let bearer = self.credential.as_ref().map(|credential| credential.capability()).transpose().map_err(|_| GisMapInferencePortCodeV1::Denied)?;
        let path = Self::gis_map_inference_path(scope, suffix);
        let response = match self.transport.http(ctx, method, &self.url(&path), bearer, body).await {
            Ok(response) => response,
            Err(TransportError::Cancelled) => return Err(GisMapInferencePortCodeV1::Cancelled),
            Err(_) => return Err(GisMapInferencePortCodeV1::Transport),
        };
        if ctx.cancel.is_cancelled().await {
            return Err(GisMapInferencePortCodeV1::Cancelled);
        }
        if !(200..=299).contains(&response.status) {
            return Err(GisMapInferencePortCodeV1::from_status(response.status));
        }
        if response.body.len() > GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES {
            return Err(GisMapInferencePortCodeV1::Bounds);
        }
        decode_json_bytes(&response.body).map_err(|_: DirectoryClientError| GisMapInferencePortCodeV1::Invalid)
    }

    /// 📮 Submits exactly one job. It is never retried: an indeterminate transport is terminal, so a
    /// replay can never mint a second job behind the operator's back.
    pub async fn submit_gis_map_inference_job(&self, ctx: &OperationContext, scope: &DocumentScope, request: &GisMapInferenceJobRequestV1) -> Result<GisMapInferenceJobReceiptV1, GisMapInferencePortCodeV1> {
        let body = crate::os_pack::json::to_json_string(request).into_bytes();
        self.gis_map_inference_call(ctx, HttpMethod::Post, scope, "/jobs", Some(body)).await
    }

    /// 📃 Reads one bounded owner-private page after an exact progress cursor.
    pub async fn read_gis_map_inference_events(&self, ctx: &OperationContext, scope: &DocumentScope, job_id: &str, after: u64) -> Result<GisMapInferenceEventPageV1, GisMapInferencePortCodeV1> {
        if after > GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR {
            return Err(GisMapInferencePortCodeV1::Bounds);
        }
        self.gis_map_inference_call(ctx, HttpMethod::Get, scope, &format!("/jobs/{}/events?after={after}", encode_url_component(job_id)), None).await
    }

    /// 🛑 Requests cancellation and returns the server's own next page; the caller never assumes it.
    pub async fn cancel_gis_map_inference_job(&self, ctx: &OperationContext, scope: &DocumentScope, job_id: &str) -> Result<GisMapInferenceEventPageV1, GisMapInferencePortCodeV1> {
        self.gis_map_inference_call(ctx, HttpMethod::Post, scope, &format!("/jobs/{}/cancel", encode_url_component(job_id)), None).await
    }

    /// ✅ Approves exactly the offered proposal, echoing back the hash the server itself published.
    pub async fn approve_gis_map_inference_job(&self, ctx: &OperationContext, scope: &DocumentScope, request: &GisMapInferenceApprovalRequestV1) -> Result<GisMapInferenceApprovalReceiptV1, GisMapInferencePortCodeV1> {
        let body = crate::os_pack::json::to_json_string(request).into_bytes();
        self.gis_map_inference_call(ctx, HttpMethod::Post, scope, &format!("/jobs/{}/approval", encode_url_component(&request.job_id)), Some(body)).await
    }
    //#endregion 💡️InferencePort

    pub fn stream(self: &Arc<Self>, since: u64) -> DirectoryStream<T>
    where
        T: Clone,
    {
        DirectoryStream::new(self.clone(), None, since, true)
    }

    /// 📌️ Creates a global wakeup stream whose reconnect cursor advances only by explicit page ACK.
    pub fn stream_acknowledged(self: &Arc<Self>, since: u64) -> Result<DirectoryStream<T>, DirectoryClientError>
    where
        T: Clone,
    {
        if since > DOCUMENT_OPEN_MAX_SAFE_INTEGER {
            return Err(DirectoryClientError::Decode("directory stream acknowledged frontier is not wire-safe".into()));
        }
        Ok(DirectoryStream::new(self.clone(), None, since, false))
    }

    /// 🎯️ Creates one exact scope-bound stream whose membership revocation is terminal.
    pub fn stream_scoped(self: &Arc<Self>, scope: DocumentScope, since: u64) -> DirectoryStream<T>
    where
        T: Clone,
    {
        DirectoryStream::new(self.clone(), Some(scope), since, true)
    }

    pub fn open_stream_ws(&self, ctx: &OperationContext, since: u64, timeout_ms: u64) -> Result<T::Ws, DirectoryClientError> {
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        let mut receipt = self.issue_socket_grant(ctx, "/directory/socket-grants", b"{}", timeout_ms)?;
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        let url = directory_ws_url(&self.base_url, since);
        let mut protocols = vec![std::mem::take(&mut receipt.protocol), std::mem::take(&mut receipt.grant)];
        let connection = self.transport.open_ws(ctx, &url, &protocols, timeout_ms);
        wipe_string(&mut protocols[1]);
        let mut connection = connection?;
        if ctx.cancel.is_cancelled_now() {
            connection.close();
            return Err(DirectoryClientError::Cancelled);
        }
        if let Err(error) = connection.send_binary(directory_socket_hello_v1()) {
            connection.close();
            return Err(error.into());
        }
        if ctx.cancel.is_cancelled_now() {
            connection.close();
            return Err(DirectoryClientError::Cancelled);
        }
        Ok(connection)
    }

    /// 🎯️ Issues and consumes one membership-bound grant for an exact document scope.
    pub fn open_scoped_stream_ws(&self, ctx: &OperationContext, scope: &DocumentScope, since: u64, timeout_ms: u64) -> Result<T::Ws, DirectoryClientError> {
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        if scope.space_id.is_empty() || scope.document_id.is_empty() || scope.space_id.len() > DIRECTORY_SCOPE_ID_MAX_BYTES || scope.document_id.len() > DIRECTORY_SCOPE_ID_MAX_BYTES {
            return Err(DirectoryClientError::Decode("directory stream scope invalid".into()));
        }
        let prefix = format!(
            "/directory/spaces/{}/documents/{}",
            encode_url_component(&scope.space_id),
            encode_url_component(&scope.document_id),
        );
        let mut receipt = self.issue_socket_grant(ctx, &format!("{prefix}/socket-grants"), b"", timeout_ms)?;
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        let url = directory_scoped_ws_url(&self.base_url, scope, since);
        let mut protocols = vec![std::mem::take(&mut receipt.protocol), std::mem::take(&mut receipt.grant)];
        let connection = self.transport.open_ws(ctx, &url, &protocols, timeout_ms);
        wipe_string(&mut protocols[1]);
        let mut connection = connection?;
        if ctx.cancel.is_cancelled_now() {
            connection.close();
            return Err(DirectoryClientError::Cancelled);
        }
        if let Err(error) = connection.send_binary(directory_socket_hello_v1()) {
            connection.close();
            return Err(error.into());
        }
        Ok(connection)
    }

    fn protected_post(&self, ctx: &OperationContext, path: &str, body: &[u8], timeout_ms: u64) -> Result<WipeBytes<'static>, DirectoryClientError> {
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        let credential = self.credential.as_ref().ok_or(DirectoryClientError::Unauthorized)?;
        let origin = credential.hub_origin().trim_end_matches('/');
        if self.base_url.trim_end_matches('/') != origin || !path.starts_with('/') || path.starts_with("//") {
            return Err(DirectoryClientError::Unauthorized);
        }
        let url = format!("{origin}{path}");
        let response = self.transport.issue_socket_grant(ctx, &url, credential.capability()?, body, timeout_ms.clamp(1, 5_000))?;
        let response_body = WipeBytes { bytes: response.body, observer: None };
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        if response_body.bytes.len() > DOCUMENT_ADMISSION_RESPONSE_MAX_BYTES {
            return Err(DirectoryClientError::Decode("protected document admission response exceeded 64 KiB".into()));
        }
        if response.status == 401 {
            return Err(DirectoryClientError::Unauthorized);
        }
        if !(200..=299).contains(&response.status) {
            return Err(DirectoryClientError::Http { status: response.status, body: "protected document admission rejected".into() });
        }
        Ok(response_body)
    }

    fn issue_socket_grant(&self, ctx: &OperationContext, path: &str, body: &[u8], timeout_ms: u64) -> Result<SocketGrantReceiptV1, DirectoryClientError> {
        let response_body = self.protected_post(ctx, path, body, timeout_ms)?;
        let receipt: SocketGrantReceiptV1 = decode_json_bytes(&response_body.bytes).map_err(|_| DirectoryClientError::Decode("socket grant receipt invalid".into()))?;
        if receipt.schema != "semio.hub.socket-grant/v1" || receipt.protocol != "semio.socket.v1" || !valid_socket_grant(&receipt.grant) || !valid_socket_actor(&receipt.actor_id) || receipt.expires_at_ms <= wall_now_ms() {
            return Err(DirectoryClientError::Decode("socket grant receipt binding invalid".into()));
        }
        emit_socket_grant_probe(path, &receipt.grant);
        Ok(receipt)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<T: DirectoryTransport + Send + Sync> HubSocketGrantSource for DirectoryClient<T> {
    fn admit_document_socket(
        &self,
        ctx: &OperationContext,
        space_id: &str,
        document_id: &str,
        expectation: &DocumentSocketExpectationV1,
        client_instance_id: &str,
        timeout_ms: u64,
    ) -> Result<DocumentSocketAdmissionV1, DirectoryClientError> {
        let scope = DocumentScope::new(space_id, document_id);
        let intent = DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: expectation.requested_surface_id.clone(),
            client_instance_id: client_instance_id.to_string(),
        };
        if intent.validate().is_err() || expectation.artifact_schema.is_empty() {
            return Err(DirectoryClientError::Decode("document open intent invalid".into()));
        }
        let prefix = format!("/spaces/{}/documents/{}", encode_url_component(space_id), encode_url_component(document_id));
        let intent_body = crate::os_pack::json::to_json_string(&intent).into_bytes();
        let plan_body = self.protected_post(ctx, &format!("{prefix}/open-plan"), &intent_body, timeout_ms)?;
        let mut plan: DocumentOpenPlanV1 = decode_json_bytes(&plan_body.bytes).map_err(|_| DirectoryClientError::Decode("document open plan response invalid".into()))?;
        let now_ms = u64::try_from(wall_now_ms()).map_err(|_| DirectoryClientError::Decode("document open clock invalid".into()))?;
        let pack_schema_hash = decode_lower_hex_32(&plan.artifact.pack_schema_hash);
        let surface_matches = expectation
            .lease
            .as_ref()
            .is_none_or(|expected| same_lease_fields_v1(&lease_fields_from_plan_v1(&plan, expected.component.byte_length, expected.descriptor.byte_length), expected))
            && expectation.requested_surface_id.as_ref().is_none_or(|surface| plan.surface.surface_id == *surface);
        if plan.validate(now_ms).is_err()
            || plan.scope != scope
            || plan.artifact.schema != expectation.artifact_schema
            || pack_schema_hash != Some(expectation.pack_schema_hash)
            || !surface_matches
        {
            wipe_string(&mut plan.receipt);
            return Err(DirectoryClientError::Decode("document open plan binding invalid".into()));
        }
        if ctx.cancel.is_cancelled_now() {
            wipe_string(&mut plan.receipt);
            return Err(DirectoryClientError::Cancelled);
        }
        let exchange = WipeDocumentPlanSocketGrantIntent(DocumentPlanSocketGrantIntentV1 {
            schema: "semio.hub.document-plan-socket-grant-intent/v1".into(),
            version: 1,
            plan_receipt: std::mem::take(&mut plan.receipt),
        });
        let exchange_body = WipeBytes { bytes: crate::os_pack::json::to_json_string(&exchange.0).into_bytes(), observer: None };
        let socket = self.issue_socket_grant(ctx, &format!("{prefix}/socket-grants"), &exchange_body.bytes, timeout_ms)?;
        if ctx.cancel.is_cancelled_now() {
            return Err(DirectoryClientError::Cancelled);
        }
        let pack_schema_hash = pack_schema_hash.ok_or_else(|| DirectoryClientError::Decode("document open plan schema hash invalid".into()))?;
        let credential_origin = self.credential.as_ref().ok_or(DirectoryClientError::Unauthorized)?.hub_origin().trim_end_matches('/');
        let authority = DocumentSocketAuthorityV1 {
            hub_origin: credential_origin.to_string(),
            expires_at_unix_ms: plan.expires_at_unix_ms,
            scope: plan.scope,
            descriptor_digest_v1: plan.descriptor_digest_v1,
            catalog: plan.catalog,
            package: plan.package,
            artifact: plan.artifact,
            parent_dialect: plan.parent_dialect,
            pack_schema_hash,
            surface: plan.surface,
            grant: plan.grant,
            checkpoint: plan.checkpoint,
            revalidation: plan.revalidation,
        };
        Ok(DocumentSocketAdmissionV1 { socket, authority })
    }
}
//#endregion 🔖️Client

//#region 🔖️Stream
/// ⏱️ Reconnect backoff floor/ceiling — same constants `🧵️backbone-worker.ts`'s
/// `HUB_RECONNECT_MIN_MS`/`HUB_RECONNECT_MAX_MS` already use for the document WS.
pub const HUB_RECONNECT_MIN_MS: u64 = 500;
pub const HUB_RECONNECT_MAX_MS: u64 = 30_000;

/// 🔗️ `remote://host:port` / `http(s)://…` → `ws(s)://host:port/directory/socket/v1?since=`
/// (contract §C2). Pure and independently testable, mirroring `🏪️store/🔄️sync`'s `hub_ws_url`.
pub fn directory_ws_url(base_url: &str, since: u64) -> String {
    let secure = base_url.starts_with("https://") || base_url.starts_with("wss://");
    let authority = base_url.split_once("://").map_or(base_url, |(_, rest)| rest).split('/').next().unwrap_or(base_url);
    let scheme = if secure { "wss" } else { "ws" };
    format!("{scheme}://{authority}/directory/socket/v1?since={since}")
}

/// 🎯️ Builds the exact URL paired with one scope-bound directory grant.
pub fn directory_scoped_ws_url(base_url: &str, scope: &DocumentScope, since: u64) -> String {
    let secure = base_url.starts_with("https://") || base_url.starts_with("wss://");
    let authority = base_url.split_once("://").map_or(base_url, |(_, rest)| rest).split('/').next().unwrap_or(base_url);
    let scheme = if secure { "wss" } else { "ws" };
    format!(
        "{scheme}://{authority}/directory/spaces/{}/documents/{}/socket/v1?since={since}",
        encode_url_component(&scope.space_id),
        encode_url_component(&scope.document_id),
    )
}

/// ⏱️ Doubling backoff capped at `HUB_RECONNECT_MAX_MS`, floored at `HUB_RECONNECT_MIN_MS`.
pub fn next_backoff_ms(current_ms: u64) -> u64 {
    current_ms.saturating_mul(2).clamp(HUB_RECONNECT_MIN_MS, HUB_RECONNECT_MAX_MS)
}

/// 📡️ One finite stream turn. `Dial` is executed separately on `Lane::Io`; `ReconnectAt` is armed
/// on the shared `TimerWheel`; `Idle` yields immediately instead of parking a worker on a socket.
pub enum DirectoryStreamTurn<T: DirectoryTransport> {
    Dial { client: Arc<DirectoryClient<T>>, since: u64 },
    DialScoped { client: Arc<DirectoryClient<T>>, scope: DocumentScope, since: u64 },
    Message(DirectoryStreamMessage),
    ReconnectAt(u64),
    Idle,
    Revoked(DocumentScope),
    Closed,
}

/// 🔁️ `GET /directory/socket/v1?since=` with auto-reconnect that resumes from the last `seq`/`headSeq`
/// this stream has observed — never re-subscribes from the caller's original `since` after a
/// drop, so the hub replays only the gap.
pub struct DirectoryStream<T: DirectoryTransport> {
    client: Arc<DirectoryClient<T>>,
    scope: Option<DocumentScope>,
    since: u64,
    track_observed_frontier: bool,
    connection: Option<T::Ws>,
    backoff_ms: u64,
    reconnect_at_ms: Option<u64>,
    dialing: bool,
    closed: bool,
}

impl<T: DirectoryTransport + Clone> DirectoryStream<T> {
    fn new(client: Arc<DirectoryClient<T>>, scope: Option<DocumentScope>, since: u64, track_observed_frontier: bool) -> Self {
        Self { client, scope, since, track_observed_frontier, connection: None, backoff_ms: HUB_RECONNECT_MIN_MS, reconnect_at_ms: None, dialing: false, closed: false }
    }

    pub fn since(&self) -> u64 {
        self.since
    }

    /// 📌️ Advances the durable reconnect frontier after an exact Home page acknowledgement.
    pub fn acknowledge(&mut self, through: u64) -> Result<(), DirectoryClientError> {
        if self.track_observed_frontier || through < self.since || through > DOCUMENT_OPEN_MAX_SAFE_INTEGER {
            return Err(DirectoryClientError::Decode("directory stream acknowledgement frontier invalid".into()));
        }
        self.since = through;
        Ok(())
    }

    pub fn close(&mut self) {
        self.closed = true;
        if let Some(mut connection) = self.connection.take() {
            connection.close();
        }
    }

    /// 🔄️ Applies a completed I/O-lane dial without reading from the socket in the same turn.
    pub fn complete_dial(&mut self, now_ms: u64, result: Result<T::Ws, TransportError>) -> DirectoryStreamTurn<T> {
        self.dialing = false;
        if self.closed {
            if let Ok(mut connection) = result {
                connection.close();
            }
            return DirectoryStreamTurn::Closed;
        }
        match result {
            Ok(connection) => {
                self.connection = Some(connection);
                self.backoff_ms = HUB_RECONNECT_MIN_MS;
                self.reconnect_at_ms = None;
                DirectoryStreamTurn::Idle
            }
            Err(_) => self.reconnecting(now_ms),
        }
    }

    /// 🔁️ Performs bounded, nonblocking protocol work only; at most eight invalid/control frames
    /// are skipped before yielding so malformed traffic cannot monopolize a worker turn.
    pub fn turn(&mut self, ctx: &OperationContext, now_ms: u64) -> DirectoryStreamTurn<T> {
        if self.closed || ctx.cancel.is_cancelled_now() {
            self.close();
            return DirectoryStreamTurn::Closed;
        }
        if self.dialing {
            return DirectoryStreamTurn::Idle;
        }
        if let Some(reconnect_at_ms) = self.reconnect_at_ms {
            if now_ms < reconnect_at_ms {
                return DirectoryStreamTurn::ReconnectAt(reconnect_at_ms);
            }
            self.reconnect_at_ms = None;
        }
        if self.connection.is_none() {
            self.dialing = true;
            return match &self.scope {
                Some(scope) => DirectoryStreamTurn::DialScoped { client: self.client.clone(), scope: scope.clone(), since: self.since },
                None => DirectoryStreamTurn::Dial { client: self.client.clone(), since: self.since },
            };
        }
        for _ in 0..8 {
            let Some(connection) = self.connection.as_mut() else { return self.reconnecting(now_ms) };
            match connection.try_recv_text() {
                Ok(DirectoryWsPoll::Text(text)) => {
                    if let Ok(message) = crate::os_pack::json::from_json_str::<DirectoryStreamMessage>(&text) {
                        self.track(&message);
                        return DirectoryStreamTurn::Message(message);
                    }
                }
                Ok(DirectoryWsPoll::Pending) => return DirectoryStreamTurn::Idle,
                Ok(DirectoryWsPoll::Closed(Some(4401))) => {
                    self.connection = None;
                    self.closed = true;
                    return self.scope.clone().map_or(DirectoryStreamTurn::Closed, DirectoryStreamTurn::Revoked);
                }
                Ok(DirectoryWsPoll::Closed(_)) | Err(_) => {
                    self.connection = None;
                    return self.reconnecting(now_ms);
                }
            }
        }
        DirectoryStreamTurn::Idle
    }

    fn reconnecting(&mut self, now_ms: u64) -> DirectoryStreamTurn<T> {
        let reconnect_at_ms = now_ms.saturating_add(self.backoff_ms);
        self.reconnect_at_ms = Some(reconnect_at_ms);
        self.backoff_ms = next_backoff_ms(self.backoff_ms);
        DirectoryStreamTurn::ReconnectAt(reconnect_at_ms)
    }

    fn track(&mut self, message: &DirectoryStreamMessage) {
        if !self.track_observed_frontier {
            return;
        }
        match message {
            DirectoryStreamMessage::Event { event } => self.since = self.since.max(event.seq),
            DirectoryStreamMessage::Heartbeat { head_seq } => self.since = self.since.max(*head_seq),
            DirectoryStreamMessage::Connection { .. } | DirectoryStreamMessage::Presence { .. } | DirectoryStreamMessage::RebootstrapRequired { .. } => {}
        }
    }
}

//#endregion 🔖️Stream

//#region 🔖️Native
/// 🐎️ Native transport: HTTP routes through `semio-framework-os-services`'s `HttpPool`/
/// `ComputePool` (this crate's EXISTING optional `ureq` dep supplies the ONE blocking call
/// `HttpPool` admits onto its bounded pool — no per-call `std::thread::spawn` and no private
/// tokio runtime built by this crate), `tokio-tungstenite` for WS (this crate's EXISTING `sync`
/// feature, which already wires it up for `🏪️store/🔄️sync`'s native actor). Both features are
/// required together because a real client needs both halves; either alone would leave the other
/// method type unusable.
#[cfg(all(feature = "ureq", feature = "sync", not(target_arch = "wasm32")))]
pub mod native {
    use super::{DirectoryTransport, DirectoryWsConnection, DirectoryWsPoll, HttpMethod, HttpResponse, LocalHubCredential, TransportError};
    use semio_framework_actor::{ActorId, PackageId};
    use semio_framework_async::{HostAsyncRuntime, HostFuture, OperationContext, ScopeHandle};
    use semio_framework_os_services::{AsyncHttpTransport, ComputeError, ComputePool, HttpBody, HttpPool, HttpPoolError, HttpRequest as PoolHttpRequest, HttpResponseHead, TokioHostRuntime};
    use std::io::Read;
    use std::net::{TcpStream, ToSocketAddrs};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use tokio_tungstenite::tungstenite::{self, client::IntoClientRequest, stream::MaybeTlsStream, Message};

    const UREQ_HTTP_URL_BYTES: usize = 2_048;
    const UREQ_HTTP_HEADER_ITEMS: usize = 64;
    const UREQ_HTTP_HEADER_BYTES: usize = 16 * 1024;
    const UREQ_HTTP_REQUEST_BODY_BYTES: usize = 16 * 1024 * 1024;
    const UREQ_HTTP_BODY_PAGE_BYTES: usize = 16 * 1024;

    type UreqBodyReader = Box<dyn Read + Send + Sync + 'static>;

    #[cfg(test)]
    mod runtime_identity_tests {
        include!("🪪️runtime/🧪️tests/🦀️.rs");
    }

    pub struct UreqStreamingHttpTransport {
        agent: ureq::Agent,
        compute: Arc<ComputePool>,
        runtime: Arc<TokioHostRuntime>,
        scope: ScopeHandle,
    }

    impl UreqStreamingHttpTransport {
        pub fn new(compute: Arc<ComputePool>, runtime: Arc<TokioHostRuntime>, scope: ScopeHandle) -> Self {
            Self { agent: ureq::Agent::new(), compute, runtime, scope }
        }
    }

    struct UreqStreamingHttpBody {
        reader: Arc<Mutex<Option<UreqBodyReader>>>,
        compute: Arc<ComputePool>,
        runtime: Arc<TokioHostRuntime>,
        scope: ScopeHandle,
        ctx: OperationContext,
    }

    impl HttpBody for UreqStreamingHttpBody {
        fn next_chunk(&mut self) -> HostFuture<Result<Option<Vec<u8>>, HttpPoolError>> {
            if self.ctx.cancel.is_cancelled_now() {
                return Box::pin(async { Err(HttpPoolError::Transport("ureq HTTP body cancelled".into())) });
            }
            let reader = self.reader.clone();
            let compute = self.compute.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            let ctx = self.ctx.clone();
            Box::pin(async move { compute.run_io(runtime.as_ref(), &scope, ctx, move || ureq_stream_read_page(&reader)).await.map_err(HttpPoolError::Compute)? })
        }
    }

    impl AsyncHttpTransport for UreqStreamingHttpTransport {
        fn start(&self, ctx: &OperationContext, request: PoolHttpRequest) -> HostFuture<Result<(HttpResponseHead, Box<dyn HttpBody>), HttpPoolError>> {
            if ctx.cancel.is_cancelled_now() {
                return Box::pin(async { Err(HttpPoolError::Transport("ureq HTTP request cancelled".into())) });
            }
            let agent = self.agent.clone();
            let compute = self.compute.clone();
            let runtime = self.runtime.clone();
            let scope = self.scope.clone();
            let connect_compute = compute.clone();
            let connect_runtime = runtime.clone();
            let connect_scope = scope.clone();
            let connect_ctx = ctx.clone();
            let body_ctx = ctx.clone();
            Box::pin(async move {
                let (head, reader) = connect_compute.run_io(connect_runtime.as_ref(), &connect_scope, connect_ctx, move || ureq_stream_start(&agent, request)).await.map_err(HttpPoolError::Compute)??;
                let body: Box<dyn HttpBody> = Box::new(UreqStreamingHttpBody { reader: Arc::new(Mutex::new(Some(reader))), compute, runtime, scope, ctx: body_ctx });
                Ok((head, body))
            })
        }
    }

    fn ureq_stream_start(agent: &ureq::Agent, request: PoolHttpRequest) -> Result<(HttpResponseHead, UreqBodyReader), HttpPoolError> {
        if request.url.len() > UREQ_HTTP_URL_BYTES || request.headers.len() > UREQ_HTTP_HEADER_ITEMS || request.body.len() > UREQ_HTTP_REQUEST_BODY_BYTES {
            return Err(HttpPoolError::Transport("ureq HTTP request exceeded fixed credits".into()));
        }
        let mut header_bytes = 0usize;
        for (name, value) in &request.headers {
            if name.bytes().any(|byte| byte <= b' ' || byte == b':') || value.bytes().any(|byte| byte == b'\r' || byte == b'\n') {
                return Err(HttpPoolError::Transport("ureq HTTP request contained invalid header bytes".into()));
            }
            header_bytes = header_bytes
                .checked_add(name.len())
                .and_then(|total| total.checked_add(value.len()))
                .and_then(|total| total.checked_add(4))
                .filter(|total| *total <= UREQ_HTTP_HEADER_BYTES)
                .ok_or_else(|| HttpPoolError::Transport("ureq HTTP request headers exceeded fixed credits".into()))?;
        }
        let _ = header_bytes;
        let response = {
            let mut builder = match request.method.as_str() {
                "GET" => agent.get(&request.url),
                "POST" => agent.post(&request.url),
                "DELETE" => agent.delete(&request.url),
                other => return Err(HttpPoolError::Transport(format!("ureq HTTP transport does not admit method {other}"))),
            };
            for (name, value) in &request.headers {
                builder = builder.set(name, value);
            }
            let outcome = if request.body.is_empty() { builder.call() } else { builder.set("Content-Type", "application/json").send_bytes(&request.body) };
            match outcome {
                Ok(response) => response,
                Err(ureq::Error::Status(_, response)) => response,
                Err(error) => return Err(HttpPoolError::Transport(error.to_string())),
            }
        };
        let status = response.status();
        let mut headers = Vec::with_capacity(4);
        let mut response_header_bytes = 0usize;
        for name in ["Content-Length", "Content-Type", "ETag", "Last-Modified"] {
            if let Some(value) = response.header(name) {
                response_header_bytes = response_header_bytes
                    .checked_add(name.len())
                    .and_then(|total| total.checked_add(value.len()))
                    .filter(|total| *total <= UREQ_HTTP_HEADER_BYTES)
                    .ok_or_else(|| HttpPoolError::Transport("ureq HTTP response headers exceeded fixed credits".into()))?;
                headers.push((name.to_string(), value.to_string()));
            }
        }
        Ok((HttpResponseHead { status, headers }, response.into_reader()))
    }

    fn ureq_stream_read_page(reader: &Arc<Mutex<Option<UreqBodyReader>>>) -> Result<Option<Vec<u8>>, HttpPoolError> {
        let mut slot = reader.lock().map_err(|_| HttpPoolError::Transport("ureq HTTP body lock poisoned".into()))?;
        let body = slot.as_mut().ok_or_else(|| HttpPoolError::Transport("ureq HTTP body reached terminal ownership".into()))?;
        let mut page = vec![0u8; UREQ_HTTP_BODY_PAGE_BYTES];
        let count = body.read(&mut page).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
        if count == 0 {
            slot.take();
            return Ok(None);
        }
        page.truncate(count);
        Ok(Some(page))
    }

    async fn http_method_str(method: HttpMethod) -> &'static str {
        match method {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Delete => "DELETE",
        }
    }

    type TungsteniteStream = tungstenite::WebSocket<MaybeTlsStream<TcpStream>>;

    pub struct TungsteniteConnection(TungsteniteStream);

    impl DirectoryWsConnection for TungsteniteConnection {
        fn send_text(&mut self, text: String) -> Result<(), TransportError> {
            self.0.send(Message::Text(text.into())).map_err(|error| TransportError::Io(error.to_string()))
        }

        fn send_binary(&mut self, bytes: Vec<u8>) -> Result<(), TransportError> {
            self.0.send(Message::Binary(bytes.into())).map_err(|error| TransportError::Io(error.to_string()))
        }

        fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError> {
            for _ in 0..8 {
                match self.0.read() {
                    Ok(Message::Text(text)) => return Ok(DirectoryWsPoll::Text(text.to_string())),
                    Ok(Message::Close(frame)) => return Ok(DirectoryWsPoll::Closed(frame.map(|frame| u16::from(frame.code)))),
                    Ok(Message::Ping(_) | Message::Pong(_) | Message::Binary(_) | Message::Frame(_)) => {}
                    Err(tungstenite::Error::Io(error)) if error.kind() == std::io::ErrorKind::WouldBlock => return Ok(DirectoryWsPoll::Pending),
                    Err(tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed) => return Ok(DirectoryWsPoll::Closed(None)),
                    Err(error) => return Err(TransportError::Io(error.to_string())),
                }
            }
            Ok(DirectoryWsPoll::Pending)
        }

        fn close(&mut self) {
            let _ = self.0.close(None);
        }
    }

    /// 🚀️ A `DirectoryTransport` for the wgpu native host and native tests. `runtime`/`scope`/
    /// `http_pool` are INJECTED, never constructed by this crate — `TokioHostRuntime` stays
    /// confined to `semio-framework-os-services` (this crate names no tokio type anywhere; see the
    /// packet report's `## tokio-containment evidence`), and one `HttpPool` belongs to the WHOLE
    /// host process (its byte budget/outstanding cap are per-package/per-actor accounting,
    /// meaningless re-created fresh per client). The ideal caller is whoever already owns the host
    /// process's single `TokioHostRuntime`/`HttpPool` — seeded via `with_new_http_pool` below if a
    /// caller has a runtime/scope/`ComputePool` but wants its own pool. `Clone` (every field is
    /// itself cheap to clone — `Arc`/`ScopeHandle`/`PackageId`/`Copy` `ActorId`) so a host that
    /// wants several `DirectoryClient`s sharing the SAME pool/quota bucket can build one transport
    /// once and `.clone()` it per client, rather than re-deriving the pool per client.
    pub struct NativeDirectoryTransport<R: HostAsyncRuntime> {
        runtime: Arc<R>,
        scope: ScopeHandle,
        http_pool: Arc<HttpPool>,
        package: PackageId,
        actor: ActorId,
    }

    impl<R: HostAsyncRuntime> Clone for NativeDirectoryTransport<R> {
        fn clone(&self) -> Self {
            Self { runtime: Arc::clone(&self.runtime), scope: self.scope.clone(), http_pool: Arc::clone(&self.http_pool), package: self.package.clone(), actor: self.actor }
        }
    }

    impl<R: HostAsyncRuntime> NativeDirectoryTransport<R> {
        pub async fn new(runtime: Arc<R>, scope: ScopeHandle, http_pool: Arc<HttpPool>, package: PackageId, actor: ActorId) -> Self {
            Self::new_now(runtime, scope, http_pool, package, actor)
        }

        pub fn new_now(runtime: Arc<R>, scope: ScopeHandle, http_pool: Arc<HttpPool>, package: PackageId, actor: ActorId) -> Self {
            Self { runtime, scope, http_pool, package, actor }
        }

        /// 🛡️ Starts one protected bounded binary GET without exposing bearer text to the caller.
        pub async fn fetch_protected_stream(&self, ctx: &OperationContext, credential: &LocalHubCredential, url: &str, accept: &str) -> Result<(HttpResponseHead, semio_framework_os_services::HttpPoolBody), TransportError>
        where
            R: 'static,
        {
            if ctx.cancel.is_cancelled().await {
                return Err(TransportError::Cancelled);
            }
            let origin = credential.hub_origin().trim_end_matches('/');
            if !url.strip_prefix(origin).is_some_and(|path| path.starts_with('/') && !path.starts_with("//")) || !matches!(accept, "application/vnd.semio.canonical-checkpoint-pair.v1") {
                return Err(TransportError::Io("protected binary request authority mismatch".into()));
            }
            let bearer = credential.capability().map_err(|_| TransportError::Io("protected binary credential was invalid".into()))?;
            let request = PoolHttpRequest { method: "GET".to_string(), url: url.to_string(), headers: vec![("Authorization".to_string(), format!("Bearer {bearer}")), ("Accept".to_string(), accept.to_string())], body: Vec::new() };
            self.http_pool.fetch(self.runtime.as_ref(), &self.scope, ctx.clone(), self.package.clone(), self.actor, request).await.map_err(|error| {
                if matches!(error, HttpPoolError::Compute(ComputeError::DeadlineExceeded)) {
                    TransportError::DeadlineExceeded
                } else if ctx.cancel.is_cancelled_now() {
                    TransportError::Cancelled
                } else {
                    TransportError::Io(error.to_string())
                }
            })
        }
    }

    //#region 💡️Inference
    impl<R: HostAsyncRuntime> NativeDirectoryTransport<R> {
        /// 🛡️ Issues one protected bounded JSON request without ever exposing bearer text to the
        /// caller — the small-body twin of [`NativeDirectoryTransport::fetch_protected_stream`], for
        /// hub routes whose request and response are closed JSON documents rather than a stream.
        /// The URL must be an exact path under the credential's own origin, so a caller can never
        /// redirect a protected credential at a foreign host.
        pub async fn request_protected_json(&self, ctx: &OperationContext, credential: &LocalHubCredential, method: HttpMethod, url: &str, body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError>
        where
            R: 'static,
        {
            if ctx.cancel.is_cancelled().await {
                return Err(TransportError::Cancelled);
            }
            let origin = credential.hub_origin().trim_end_matches('/');
            if !url.strip_prefix(origin).is_some_and(|path| path.starts_with('/') && !path.starts_with("//")) {
                return Err(TransportError::Io("protected json request authority mismatch".into()));
            }
            let bearer = credential.capability().map_err(|_| TransportError::Io("protected json credential was invalid".into()))?;
            self.http(ctx, method, url, Some(bearer), body).await
        }
    }
    //#endregion 💡️Inference

    impl NativeDirectoryTransport<TokioHostRuntime> {
        pub async fn with_new_http_pool(runtime: Arc<TokioHostRuntime>, scope: ScopeHandle, compute: Arc<ComputePool>, bytes_per_minute_cap: u64, outstanding_cap: u32, package: PackageId, actor: ActorId) -> Self {
            Self::with_new_http_pool_now(runtime, scope, compute, bytes_per_minute_cap, outstanding_cap, package, actor)
        }

        pub fn with_new_http_pool_now(runtime: Arc<TokioHostRuntime>, scope: ScopeHandle, compute: Arc<ComputePool>, bytes_per_minute_cap: u64, outstanding_cap: u32, package: PackageId, actor: ActorId) -> Self {
            let transport: Arc<dyn AsyncHttpTransport> = Arc::new(UreqStreamingHttpTransport::new(compute, runtime.clone(), scope.clone()));
            Self::new_now(runtime, scope, Arc::new(HttpPool::new_with_async_transport_now(transport, bytes_per_minute_cap, outstanding_cap)), package, actor)
        }
    }

    impl<R: HostAsyncRuntime + 'static> DirectoryTransport for NativeDirectoryTransport<R> {
        type Ws = TungsteniteConnection;
        async fn http(&self, ctx: &OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
            if ctx.cancel.is_cancelled().await {
                return Err(TransportError::Cancelled);
            }
            let mut headers = Vec::new();
            if let Some(token) = bearer {
                headers.push(("Authorization".to_string(), format!("Bearer {token}")));
            }
            let request = PoolHttpRequest { method: http_method_str(method).await.to_string(), url: url.to_string(), headers, body: body.unwrap_or_default() };
            match self.http_pool.request(self.runtime.as_ref(), &self.scope, ctx.clone(), self.package.clone(), self.actor, request).await {
                Ok(response) => Ok(HttpResponse { status: response.status, body: response.body }),
                Err(error) => {
                    let cancelled = matches!(error, HttpPoolError::Compute(ComputeError::WorkerLost)) && ctx.cancel.is_cancelled().await;
                    Err(match error {
                        HttpPoolError::Compute(ComputeError::DeadlineExceeded) => TransportError::DeadlineExceeded,
                        _ if cancelled => TransportError::Cancelled,
                        other => TransportError::Io(other.to_string()),
                    })
                }
            }
        }

        fn issue_socket_grant(&self, ctx: &OperationContext, url: &str, bearer: &str, body: &[u8], timeout_ms: u64) -> Result<HttpResponse, TransportError> {
            if ctx.cancel.is_cancelled_now() {
                return Err(TransportError::Cancelled);
            }
            let timeout = Duration::from_millis(timeout_ms.max(1));
            let agent = ureq::AgentBuilder::new().timeout_connect(timeout).timeout_read(timeout).timeout_write(timeout).build();
            let authorization = super::WipeSecretText(format!("Bearer {bearer}"));
            let response = match agent.post(url).set("Authorization", &authorization.0).set("Content-Type", "application/json").send_bytes(body) {
                Ok(response) => response,
                Err(ureq::Error::Status(_, response)) => response,
                Err(error) => return Err(TransportError::Io(error.to_string())),
            };
            let status = response.status();
            let mut body = Vec::new();
            response
                .into_reader()
                .take((super::DOCUMENT_ADMISSION_RESPONSE_MAX_BYTES + 1) as u64)
                .read_to_end(&mut body)
                .map_err(|error| TransportError::Io(error.to_string()))?;
            Ok(HttpResponse { status, body })
        }

        fn open_ws(&self, ctx: &OperationContext, url: &str, protocols: &[String], timeout_ms: u64) -> Result<Self::Ws, TransportError> {
            if ctx.cancel.is_cancelled_now() {
                return Err(TransportError::Cancelled);
            }
            if protocols.len() != 2 || protocols[0] != "semio.socket.v1" || !super::valid_socket_grant(&protocols[1]) {
                return Err(TransportError::Io("directory websocket protocol offer invalid".into()));
            }
            let mut request = url.into_client_request().map_err(|error| TransportError::Io(error.to_string()))?;
            request.headers_mut().insert("Sec-WebSocket-Protocol", format!("{}, {}", protocols[0], protocols[1]).parse().map_err(|_| TransportError::Io("directory websocket protocol header invalid".into()))?);
            let host = request.uri().host().ok_or_else(|| TransportError::Io("directory websocket URL has no host".to_string()))?.to_string();
            let port = request.uri().port_u16().unwrap_or_else(|| if request.uri().scheme_str() == Some("wss") { 443 } else { 80 });
            let timeout = Duration::from_millis(timeout_ms.max(1));
            let addresses = (host.as_str(), port).to_socket_addrs().map_err(|error| TransportError::Io(error.to_string()))?;
            let mut tcp = None;
            for address in addresses {
                if ctx.cancel.is_cancelled_now() {
                    return Err(TransportError::Cancelled);
                }
                if let Ok(stream) = TcpStream::connect_timeout(&address, timeout) {
                    tcp = Some(stream);
                    break;
                }
            }
            let tcp = tcp.ok_or_else(|| TransportError::Io(format!("unable to connect to {host}:{port}")))?;
            tcp.set_read_timeout(Some(timeout)).map_err(|error| TransportError::Io(error.to_string()))?;
            tcp.set_write_timeout(Some(timeout)).map_err(|error| TransportError::Io(error.to_string()))?;
            let (mut socket, response) = tungstenite::client_tls(request, tcp).map_err(|error| TransportError::Io(error.to_string()))?;
            if response.headers().get("Sec-WebSocket-Protocol").and_then(|value| value.to_str().ok()) != Some("semio.socket.v1") {
                return Err(TransportError::Io("directory websocket protocol negotiation invalid".into()));
            }
            match socket.get_mut() {
                MaybeTlsStream::Plain(stream) => stream.set_nonblocking(true).map_err(|error| TransportError::Io(error.to_string()))?,
                MaybeTlsStream::Rustls(stream) => stream.sock.set_nonblocking(true).map_err(|error| TransportError::Io(error.to_string()))?,
                _ => return Err(TransportError::Io("nonblocking TLS directory sockets are not enabled".to_string())),
            }
            Ok(TungsteniteConnection(socket))
        }
    }

    #[cfg(test)]
    mod streaming_tests {
        use super::*;
        use semio_framework_async::{CancelToken, ScopeOwner, TraceId};
        use std::sync::atomic::{AtomicUsize, Ordering};

        struct MockReader {
            bytes: Vec<u8>,
            cursor: usize,
            page: usize,
            reads: Arc<AtomicUsize>,
            fail_at: Option<usize>,
        }

        impl Read for MockReader {
            fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
                let turn = self.reads.fetch_add(1, Ordering::SeqCst);
                if self.fail_at == Some(turn) {
                    return Err(std::io::Error::other("scripted reader failure"));
                }
                if self.cursor == self.bytes.len() {
                    return Ok(0);
                }
                let count = self.page.min(output.len()).min(self.bytes.len() - self.cursor);
                output[..count].copy_from_slice(&self.bytes[self.cursor..self.cursor + count]);
                self.cursor += count;
                Ok(count)
            }
        }

        fn mock_reader(bytes: Vec<u8>, page: usize, reads: Arc<AtomicUsize>, fail_at: Option<usize>) -> Arc<Mutex<Option<UreqBodyReader>>> {
            Arc::new(Mutex::new(Some(Box::new(MockReader { bytes, cursor: 0, page, reads, fail_at }))))
        }

        #[test]
        fn https_owned_reader_yields_one_bounded_page_per_pull_and_reaches_terminal() {
            let reads = Arc::new(AtomicUsize::new(0));
            let reader = mock_reader(vec![1, 2, 3, 4, 5, 6, 7], 3, reads.clone(), None);
            assert_eq!(reads.load(Ordering::SeqCst), 0);
            assert_eq!(ureq_stream_read_page(&reader).expect("first page"), Some(vec![1, 2, 3]));
            assert_eq!(reads.load(Ordering::SeqCst), 1);
            assert_eq!(ureq_stream_read_page(&reader).expect("second page"), Some(vec![4, 5, 6]));
            assert_eq!(ureq_stream_read_page(&reader).expect("third page"), Some(vec![7]));
            assert_eq!(ureq_stream_read_page(&reader).expect("eof"), None);
            assert!(reader.lock().expect("reader lock").is_none());
        }

        #[test]
        fn https_owned_reader_reports_partial_failure_without_an_implicit_retry() {
            let reads = Arc::new(AtomicUsize::new(0));
            let reader = mock_reader(vec![1, 2, 3, 4], 2, reads.clone(), Some(1));
            assert_eq!(ureq_stream_read_page(&reader).expect("first page"), Some(vec![1, 2]));
            assert!(matches!(ureq_stream_read_page(&reader), Err(HttpPoolError::Transport(message)) if message == "scripted reader failure"));
            assert_eq!(reads.load(Ordering::SeqCst), 2);
        }

        #[test]
        fn https_owned_reader_honours_cancel_before_pulling_the_next_page() {
            let runtime = Arc::new(TokioHostRuntime::new());
            let scope = runtime.open_scope_now(ScopeOwner::Service("ureq_stream_test"), None);
            let compute = Arc::new(runtime.block_on(ComputePool::new(1)));
            let cancel = CancelToken::root_now();
            cancel.cancel_now();
            let reads = Arc::new(AtomicUsize::new(0));
            let reader = mock_reader(vec![1, 2, 3], 1, reads.clone(), None);
            let mut body = UreqStreamingHttpBody { reader, compute, runtime: runtime.clone(), scope, ctx: OperationContext { actor: 0, generation: 0, trace: TraceId(0), lane: 0, deadline_ms: None, cancel, capability: None } };
            let result = runtime.block_on(body.next_chunk());
            assert!(matches!(result, Err(HttpPoolError::Transport(message)) if message == "ureq HTTP body cancelled"));
            assert_eq!(reads.load(Ordering::SeqCst), 0);
        }
    }
}

/// 🌉️ Browser wgpu build seam: `web_sys::WebSocket`/`fetch` transport, kept coherent with
/// `🏪️store/🔄️sync`'s own `wasm_actor` precedent (event-callback bridged into an `async`-shaped
/// interface via an unbounded channel) so a future in-wasm host can link this client. NOT the
/// production browser path today — the React shell uses lane 1-C's TypeScript `DirectoryClient`
/// (`🏪️store/🔄️sync`'s own module doc: "The production browser shell instead uses a TS twin").
// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too, and this bridge is browser-only,
// so it is narrowed to exclude the WASI component target.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub mod browser {
    use super::{DirectoryTransport, DirectoryWsConnection, DirectoryWsPoll, HttpMethod, HttpResponse, TransportError};
    use semio_framework_async::browser::JsFuture;
    use semio_framework_async::OperationContext;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::{BinaryType, CloseEvent, MessageEvent, RequestInit, Response, WebSocket};

    enum BrowserDirectoryFrame {
        Text(String),
        Closed(Option<u16>),
    }

    #[derive(Clone)]
    pub struct BrowserDirectoryTransport;

    impl Default for BrowserDirectoryTransport {
        fn default() -> Self {
            Self
        }
    }

    impl DirectoryTransport for BrowserDirectoryTransport {
        type Ws = BrowserWsConnection;
        async fn http(&self, ctx: &OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
            if ctx.cancel.is_cancelled().await {
                return Err(TransportError::Cancelled);
            }
            let window = web_sys::window().ok_or_else(|| TransportError::Io("no window".to_string()))?;
            let init = RequestInit::new();
            init.set_method(match method {
                HttpMethod::Get => "GET",
                HttpMethod::Post => "POST",
                HttpMethod::Delete => "DELETE",
            });
            if let Some(bytes) = &body {
                let array = js_sys::Uint8Array::from(bytes.as_slice());
                init.set_body(&array);
            }
            let request = web_sys::Request::new_with_str_and_init(url, &init).map_err(|error| TransportError::Io(format!("{error:?}")))?;
            if let Some(token) = bearer {
                request.headers().set("Authorization", &format!("Bearer {token}")).map_err(|error| TransportError::Io(format!("{error:?}")))?;
            }
            let response_value = JsFuture::from(window.fetch_with_request(&request)).await.map_err(|error| TransportError::Io(format!("{error:?}")))?;
            let response: Response = response_value.dyn_into().map_err(|error| TransportError::Io(format!("{error:?}")))?;
            let status = response.status();
            let array_buffer = JsFuture::from(response.array_buffer().map_err(|error| TransportError::Io(format!("{error:?}")))?).await.map_err(|error| TransportError::Io(format!("{error:?}")))?;
            let bytes = js_sys::Uint8Array::new(&array_buffer).to_vec();
            Ok(HttpResponse { status, body: bytes })
        }

        fn issue_socket_grant(&self, _ctx: &OperationContext, _url: &str, _bearer: &str, _body: &[u8], _timeout_ms: u64) -> Result<HttpResponse, TransportError> {
            Err(TransportError::Io("browser directory socket issuance requires the TypeScript credential relay".into()))
        }

        fn open_ws(&self, ctx: &OperationContext, url: &str, protocols: &[String], _timeout_ms: u64) -> Result<Self::Ws, TransportError> {
            if ctx.cancel.is_cancelled_now() {
                return Err(TransportError::Cancelled);
            }
            let protocol_values = js_sys::Array::new();
            for protocol in protocols {
                protocol_values.push(&JsValue::from_str(protocol));
            }
            let socket = WebSocket::new_with_str_sequence(url, &protocol_values).map_err(|error| TransportError::Io(format!("{error:?}")))?;
            socket.set_binary_type(BinaryType::Blob);
            let (incoming_tx, incoming_rx) = tokio::sync::mpsc::unbounded_channel::<BrowserDirectoryFrame>();
            let message_tx = incoming_tx.clone();
            let onmessage = Closure::wrap(Box::new(move |event: MessageEvent| {
                if let Some(text) = event.data().as_string() {
                    let _ = message_tx.send(BrowserDirectoryFrame::Text(text));
                }
            }) as Box<dyn FnMut(MessageEvent)>);
            let onclose = Closure::wrap(Box::new(move |event: CloseEvent| {
                let _ = incoming_tx.send(BrowserDirectoryFrame::Closed(Some(event.code())));
            }) as Box<dyn FnMut(CloseEvent)>);
            socket.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            socket.set_onclose(Some(onclose.as_ref().unchecked_ref()));
            Ok(BrowserWsConnection { socket, _onmessage: onmessage, _onclose: onclose, incoming_rx })
        }
    }

    pub struct BrowserWsConnection {
        socket: WebSocket,
        _onmessage: Closure<dyn FnMut(MessageEvent)>,
        _onclose: Closure<dyn FnMut(CloseEvent)>,
        incoming_rx: tokio::sync::mpsc::UnboundedReceiver<BrowserDirectoryFrame>,
    }

    impl DirectoryWsConnection for BrowserWsConnection {
        fn send_text(&mut self, text: String) -> Result<(), TransportError> {
            self.socket.send_with_str(&text).map_err(|error| TransportError::Io(format!("{error:?}")))
        }

        fn send_binary(&mut self, bytes: Vec<u8>) -> Result<(), TransportError> {
            self.socket.send_with_u8_array(&bytes).map_err(|error| TransportError::Io(format!("{error:?}")))
        }

        fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError> {
            match self.incoming_rx.try_recv() {
                Ok(BrowserDirectoryFrame::Text(text)) => Ok(DirectoryWsPoll::Text(text)),
                Ok(BrowserDirectoryFrame::Closed(code)) => Ok(DirectoryWsPoll::Closed(code)),
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => Ok(DirectoryWsPoll::Pending),
                Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => Ok(DirectoryWsPoll::Closed(None)),
            }
        }

        fn close(&mut self) {
            let _ = self.socket.close();
        }
    }
}
//#endregion 🔖️Native

//#region 🧪️Tests
/// 🧪️ Reused by `🪪️identity`'s own tests (`super::client::test_support::FakeTransport`) so the
/// mint-vs-restore decision is exercised against the SAME double this module's stream/HTTP tests
/// use, rather than a second hand-rolled copy.
#[cfg(test)]
pub mod test_support {
    use super::{DirectoryWsConnection, DirectoryWsPoll, HttpMethod, HttpResponse, TransportError};
    use semio_framework_async::OperationContext;
    use std::collections::VecDeque;
    use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Debug, PartialEq)]
    pub struct RecordedRequest {
        pub method: HttpMethod,
        pub url: String,
        pub bearer: Option<String>,
        pub body: Vec<u8>,
    }

    #[derive(Clone, Default)]
    pub struct FakeTransport {
        pub responses: Arc<Mutex<VecDeque<Result<HttpResponse, TransportError>>>>,
        pub requests: Arc<Mutex<Vec<RecordedRequest>>>,
        pub ws_outcomes: Arc<Mutex<VecDeque<Result<VecDeque<Result<Option<String>, TransportError>>, TransportError>>>>,
        pub ws_urls: Arc<Mutex<Vec<String>>>,
        pub ws_closes: Arc<AtomicUsize>,
        pub ws_sends: Arc<AtomicUsize>,
        pub cancel_after_grant: Arc<AtomicBool>,
        pub cancel_after_grant_number: Arc<AtomicUsize>,
        pub cancel_after_open: Arc<AtomicBool>,
        /// 🧪️ Cooperative yield points `http()` passes through (checking `ctx.cancel` at each one)
        /// BEFORE touching `responses`/`requests` — 0 (the default) keeps every existing test's
        /// synchronous-looking behavior unchanged; a cancellation test sets this > 0 so an
        /// interleaved caller has a real window to flip the token between yields (see
        /// `an_in_flight_request_is_cancelled_when_its_context_is_cancelled` below).
        pub yields_before_response: Arc<AtomicU32>,
    }

    impl FakeTransport {
        pub async fn push_response(&self, response: Result<HttpResponse, TransportError>) {
            self.responses.lock().unwrap().push_back(response);
        }

        pub async fn push_ws(&self, outcome: Result<VecDeque<Result<Option<String>, TransportError>>, TransportError>) {
            self.ws_outcomes.lock().unwrap().push_back(outcome);
        }

        pub async fn json_response(status: u16, body: &serde_json::Value) -> Result<HttpResponse, TransportError> {
            Ok(HttpResponse { status, body: serde_json::to_vec(body).unwrap() })
        }
    }

    // 🔀️ `pub` (was private): now named in `impl DirectoryTransport for FakeTransport`'s public
    // `type Ws = FakeWs;` associated type.
    pub struct FakeWs {
        frames: VecDeque<Result<Option<String>, TransportError>>,
        close_frame: Option<Option<u16>>,
        closes: Option<Arc<AtomicUsize>>,
        sends: Option<Arc<AtomicUsize>>,
    }

    impl FakeWs {
        /// 🧪️ Creates a late-dial socket whose close is observable by the cancellation law.
        pub fn with_close_observer(closes: Arc<AtomicUsize>) -> Self {
            Self { frames: VecDeque::new(), close_frame: None, closes: Some(closes), sends: None }
        }

        /// 🛑️ Creates a socket whose next receive preserves one close code.
        pub fn with_close_code(code: u16) -> Self {
            Self { frames: VecDeque::new(), close_frame: Some(Some(code)), closes: None, sends: None }
        }
    }

    impl DirectoryWsConnection for FakeWs {
        fn send_text(&mut self, _text: String) -> Result<(), TransportError> {
            Ok(())
        }

        fn send_binary(&mut self, _bytes: Vec<u8>) -> Result<(), TransportError> {
            if let Some(sends) = &self.sends {
                sends.fetch_add(1, Ordering::SeqCst);
            }
            Ok(())
        }

        fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError> {
            if let Some(code) = self.close_frame.take() {
                return Ok(DirectoryWsPoll::Closed(code));
            }
            match self.frames.pop_front() {
                Some(Ok(Some(text))) => Ok(DirectoryWsPoll::Text(text)),
                Some(Ok(None)) => Ok(DirectoryWsPoll::Closed(None)),
                Some(Err(error)) => Err(error),
                None => Ok(DirectoryWsPoll::Pending),
            }
        }

        fn close(&mut self) {
            if let Some(closes) = &self.closes {
                closes.fetch_add(1, Ordering::SeqCst);
            }
        }
    }

    impl super::DirectoryTransport for FakeTransport {
        type Ws = FakeWs;
        async fn http(&self, ctx: &OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
            for _ in 0..self.yields_before_response.load(Ordering::SeqCst) {
                if ctx.cancel.is_cancelled().await {
                    return Err(TransportError::Cancelled);
                }
                semio_framework_async::yield_once().await;
            }
            if ctx.cancel.is_cancelled().await {
                return Err(TransportError::Cancelled);
            }
            self.requests.lock().unwrap().push(RecordedRequest { method, url: url.to_string(), bearer: bearer.map(str::to_string), body: body.unwrap_or_default() });
            self.responses.lock().unwrap().pop_front().unwrap_or_else(|| Err(TransportError::Io("no scripted response".to_string())))
        }

        fn issue_socket_grant(&self, ctx: &OperationContext, url: &str, bearer: &str, body: &[u8], _timeout_ms: u64) -> Result<HttpResponse, TransportError> {
            if ctx.cancel.is_cancelled_now() {
                return Err(TransportError::Cancelled);
            }
            let request_number = {
                let mut requests = self.requests.lock().unwrap();
                requests.push(RecordedRequest { method: HttpMethod::Post, url: url.to_string(), bearer: Some(bearer.to_string()), body: body.to_vec() });
                requests.len()
            };
            let response = self.responses.lock().unwrap().pop_front().unwrap_or_else(|| Err(TransportError::Io("no scripted response".to_string())));
            if self.cancel_after_grant.load(Ordering::SeqCst) || self.cancel_after_grant_number.load(Ordering::SeqCst) == request_number {
                ctx.cancel.cancel_now();
            }
            response
        }

        fn open_ws(&self, ctx: &OperationContext, url: &str, _protocols: &[String], _timeout_ms: u64) -> Result<Self::Ws, TransportError> {
            if ctx.cancel.is_cancelled_now() {
                return Err(TransportError::Cancelled);
            }
            self.ws_urls.lock().unwrap().push(url.to_string());
            let frames = self.ws_outcomes.lock().unwrap().pop_front().unwrap_or_else(|| Err(TransportError::Io("no scripted ws".to_string())))?;
            if self.cancel_after_open.load(Ordering::SeqCst) {
                ctx.cancel.cancel_now();
            }
            Ok(FakeWs { frames, close_frame: None, closes: Some(self.ws_closes.clone()), sends: Some(self.ws_sends.clone()) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::{FakeTransport, FakeWs};
    use super::*;
    use crate::os_directory::{directory_command_sha256, DirectoryCommandOutcomeV1, DirectoryCommandResultV1};
    use semio_framework_async::{CancelToken, TraceId};
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn root_ctx() -> OperationContext {
        OperationContext { actor: 0, generation: 0, trace: TraceId(0), lane: 0, deadline_ms: None, cancel: CancelToken::root_now(), capability: None }
    }

    fn authenticated_client(transport: FakeTransport, capability: &str) -> Arc<DirectoryClient<FakeTransport>> {
        Arc::new(DirectoryClient::authenticated(transport, Arc::new(LocalHubCredential::test("http://hub.local", capability))))
    }

    #[semio_framework_async_macros::async_test]
    async fn directory_event_page_preserves_canonical_bytes_bounds_and_cancels_before_io() {
        let capability = format!("session.v1.{}.{}", "a".repeat(32), "b".repeat(64));
        let mut page = DirectoryEventPageV1 {
            schema: "semio.directory.event-page.v1".into(),
            session_binding_sha256: "c".repeat(64),
            authorization_generation: 9,
            after_seq_exclusive: 3,
            through_seq_inclusive: 5,
            has_more: true,
            events: Vec::new(),
            receipt_sha256: String::new(),
        };
        page.receipt_sha256 = semio_framework_hash::sha256_hex(page.canonical_unsigned_json().as_bytes());
        let canonical = crate::os_pack::json::to_json_string(&page);
        let transport = FakeTransport::default();
        transport.push_response(Ok(HttpResponse { status: 200, body: canonical.as_bytes().to_vec() })).await;
        let client = authenticated_client(transport.clone(), &capability);
        let retained = client.event_page(&root_ctx(), 3).await.expect("canonical page");
        assert_eq!(retained.canonical_json(), canonical);
        assert_eq!(retained.session_binding_sha256(), page.session_binding_sha256);
        assert_eq!(retained.authorization_generation(), 9);
        assert_eq!(retained.after_seq_exclusive(), 3);
        assert_eq!(retained.through_seq_inclusive(), 5);
        assert!(retained.has_more());
        assert_eq!(retained.receipt_sha256(), page.receipt_sha256);
        let mut bootstrap = DirectoryEventPageBootstrapV1::new(7, 3).expect("bootstrap owner");
        let first_ack = bootstrap.present(retained.clone()).expect("first pending page");
        let mut forged_ack = first_ack.clone();
        forged_ack.receipt_sha256 = "d".repeat(64);
        assert!(bootstrap.acknowledge(&forged_ack).is_err());
        assert_eq!(bootstrap.after(), 3);
        assert_eq!(bootstrap.acknowledge(&first_ack).expect("first Home ACK"), DirectoryBootstrapTransition::Fetch { after: 5 });
        let second = CanonicalDirectoryEventPageV1 {
            canonical_json: "{}".into(),
            session_binding_sha256: page.session_binding_sha256.clone(),
            authorization_generation: page.authorization_generation,
            after_seq_exclusive: 5,
            through_seq_inclusive: 8,
            has_more: false,
            receipt_sha256: "e".repeat(64),
        };
        let second_ack = bootstrap.present(second).expect("second pending page");
        assert_eq!(bootstrap.acknowledge(&second_ack).expect("final Home ACK"), DirectoryBootstrapTransition::Live { since: 8 });
        assert_eq!(bootstrap.wake(false), Some(8));
        assert_eq!(bootstrap.wake(false), None, "a dirty burst cannot duplicate the page fetch");

        let mut rejected = DirectoryEventPageBootstrapV1::new(7, 3).expect("retry owner");
        let rejected_ack = rejected.present(retained.clone()).expect("pending retry page");
        assert_eq!(rejected.reject(7, &rejected_ack.receipt_sha256).expect("exact rejection"), 3);
        rejected.close();
        assert!(rejected.present(retained.clone()).is_err());
        let requests = transport.requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].method, HttpMethod::Get);
        assert_eq!(requests[0].url, "http://hub.local/directory/event-page/v1?after=3");
        assert_eq!(requests[0].bearer.as_deref(), Some(capability.as_str()));
        drop(requests);

        let cancelled_transport = FakeTransport::default();
        cancelled_transport.push_response(Ok(HttpResponse { status: 200, body: canonical.as_bytes().to_vec() })).await;
        let cancelled_client = authenticated_client(cancelled_transport.clone(), &capability);
        let cancelled_ctx = root_ctx();
        cancelled_ctx.cancel.cancel_now();
        assert!(matches!(cancelled_client.event_page(&cancelled_ctx, 3).await, Err(DirectoryClientError::Cancelled)));
        assert!(cancelled_transport.requests.lock().unwrap().is_empty());

        let oversized_transport = FakeTransport::default();
        oversized_transport.push_response(Ok(HttpResponse { status: 200, body: vec![b'x'; DIRECTORY_EVENT_PAGE_MAX_BYTES + 1] })).await;
        assert!(matches!(authenticated_client(oversized_transport, &capability).event_page(&root_ctx(), 0).await, Err(DirectoryClientError::Decode(_))));

        let noncanonical_transport = FakeTransport::default();
        noncanonical_transport.push_response(Ok(HttpResponse { status: 200, body: format!("{canonical} ").into_bytes() })).await;
        assert!(matches!(authenticated_client(noncanonical_transport, &capability).event_page(&root_ctx(), 3).await, Err(DirectoryClientError::Decode(_))));

        let unsafe_transport = FakeTransport::default();
        assert!(matches!(authenticated_client(unsafe_transport.clone(), &capability).event_page(&root_ctx(), DOCUMENT_OPEN_MAX_SAFE_INTEGER + 1).await, Err(DirectoryClientError::Decode(_))));
        assert!(unsafe_transport.requests.lock().unwrap().is_empty());

        let wake: DirectoryStreamMessage = crate::os_pack::json::from_json_str(
            &serde_json::json!({ "kind": "event", "event": { "seq": 99, "id": "wake", "hlc": { "physicalMs": 1, "logical": 0 }, "actor": { "kind": "system", "id": "sys" }, "body": { "kind": "space.archived", "spaceId": "sp-1" }, "recordedAtMs": 1 } }).to_string(),
        )
        .expect("wakeup event");
        let mut acknowledged = client.stream_acknowledged(3).expect("acknowledged stream");
        acknowledged.track(&wake);
        acknowledged.track(&DirectoryStreamMessage::Heartbeat { head_seq: 101 });
        assert_eq!(acknowledged.since(), 3, "observed wakeups never advance the committed cursor");
        acknowledged.acknowledge(5).expect("exact Home ACK");
        assert_eq!(acknowledged.since(), 5);
        assert!(acknowledged.acknowledge(4).is_err());
        assert!(acknowledged.acknowledge(DOCUMENT_OPEN_MAX_SAFE_INTEGER + 1).is_err());

        let mut observed = client.stream(3);
        observed.track(&wake);
        assert_eq!(observed.since(), 99, "legacy observed-frontier stream remains explicit");
    }

    #[test]
    fn decoded_consumer_credential_moves_success_secret_and_wipes_every_invalid_field() {
        let capability = format!("session.v1.{}.{}", "a".repeat(32), "b".repeat(64));
        let schema = "semio.local.consumer-credential/v1";
        let client_class = "native";
        let hub_origin = "http://127.0.0.1:8787";
        let decoded_keys_len = "schema".len() + "clientClass".len() + "hubOrigin".len() + "capability".len() + "expiresAtMs".len();
        let success_wipe_len = decoded_keys_len + schema.len() + client_class.len() + hub_origin.len();
        let envelope = serde_json::to_vec(&serde_json::json!({
            "schema": schema,
            "clientClass": client_class,
            "hubOrigin": hub_origin,
            "capability": capability,
            "expiresAtMs": wall_now_ms() + 30_000
        }))
        .expect("external JSON credential oracle");
        let success_wipes = AtomicUsize::new(0);
        let credential = decode_local_hub_credential(&envelope, "native", Some(&success_wipes)).expect("valid credential");
        assert_eq!(credential.capability().expect("credential capability"), capability);
        assert_eq!(success_wipes.load(Ordering::SeqCst), success_wipe_len, "success wipes every decoded key and non-secret string while moving the sole capability allocation");

        let invalid_wipes = AtomicUsize::new(0);
        assert!(matches!(decode_local_hub_credential(&envelope, "mcp", Some(&invalid_wipes)), Err(DirectoryClientError::Unauthorized)));
        assert_eq!(invalid_wipes.load(Ordering::SeqCst), success_wipe_len + capability.len(), "invalid-after-decode wipes every decoded string byte including the capability");
    }

    #[test]
    fn non_ascii_capability_crossing_a_fixed_boundary_is_denied_without_panicking_and_fully_wiped() {
        use std::io::Cursor;

        let capability = format!("session.v1.{}é.{}", "a".repeat(31), "b".repeat(63));
        assert_eq!(capability.len(), 108);
        let schema = "semio.local.consumer-credential/v1";
        let client_class = "native";
        let hub_origin = "http://127.0.0.1:8787";
        let decoded_keys_len = "schema".len() + "clientClass".len() + "hubOrigin".len() + "capability".len() + "expiresAtMs".len();
        let expected_wipe_len = decoded_keys_len + schema.len() + client_class.len() + hub_origin.len() + capability.len();
        let envelope = serde_json::to_vec(&serde_json::json!({
            "schema": schema,
            "clientClass": client_class,
            "hubOrigin": hub_origin,
            "capability": capability,
            "expiresAtMs": wall_now_ms() + 30_000
        }))
        .expect("external JSON credential oracle");
        let mut framed = Vec::with_capacity(4 + envelope.len());
        framed.extend_from_slice(&u32::try_from(envelope.len()).expect("bounded envelope").to_be_bytes());
        framed.extend_from_slice(&envelope);
        let raw_wipes = AtomicUsize::new(0);
        let mut cursor = Cursor::new(framed);
        let raw = read_local_hub_credential_frame(&mut cursor, Some(&raw_wipes)).expect("complete frame");
        let decoded_wipes = AtomicUsize::new(0);
        assert!(matches!(decode_local_hub_credential(&raw.bytes, client_class, Some(&decoded_wipes)), Err(DirectoryClientError::Unauthorized)));
        drop(raw);
        assert_eq!(decoded_wipes.load(Ordering::SeqCst), expected_wipe_len);
        assert_eq!(raw_wipes.load(Ordering::SeqCst), envelope.len() + 1);
    }

    #[test]
    fn inherited_frame_reader_wipes_exactly_every_allocated_body_and_trailing_probe_byte() {
        use std::io::Cursor;

        let partial_wipes = AtomicUsize::new(0);
        let mut partial = Cursor::new([8u32.to_be_bytes().as_slice(), b"abc"].concat());
        assert!(matches!(read_local_hub_credential_frame(&mut partial, Some(&partial_wipes)), Err(DirectoryClientError::Unauthorized)));
        assert_eq!(partial_wipes.load(Ordering::SeqCst), 8);

        let complete_wipes = AtomicUsize::new(0);
        let mut complete = Cursor::new([4u32.to_be_bytes().as_slice(), b"body"].concat());
        drop(read_local_hub_credential_frame(&mut complete, Some(&complete_wipes)).expect("complete frame"));
        assert_eq!(complete_wipes.load(Ordering::SeqCst), 5);

        let trailing_wipes = AtomicUsize::new(0);
        let mut trailing = Cursor::new([4u32.to_be_bytes().as_slice(), b"body", b"x"].concat());
        assert!(matches!(read_local_hub_credential_frame(&mut trailing, Some(&trailing_wipes)), Err(DirectoryClientError::Unauthorized)));
        assert_eq!(trailing_wipes.load(Ordering::SeqCst), 5);
    }

    async fn push_grant(transport: &FakeTransport) {
        transport
            .push_response(
                FakeTransport::json_response(
                    200,
                    &serde_json::json!({
                        "schema": "semio.hub.socket-grant/v1",
                        "protocol": "semio.socket.v1",
                        "grant": format!("socket.v1.{}.{}", "1".repeat(32), "2".repeat(64)),
                        "actorId": format!("hub.v1.{}", "3".repeat(64)),
                        "expiresAtMs": wall_now_ms() + 30_000
                    }),
                )
                .await,
            )
            .await;
    }

    async fn push_document_plan(transport: &FakeTransport, space_id: &str, document_id: &str, schema: &str, surface_id: &str) -> String {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("neutral plan fixture");
        let mut plan = fixture["validPlan"].clone();
        plan["expiresAtUnixMs"] = serde_json::json!(u64::try_from(wall_now_ms()).expect("wall clock") + 20_000);
        plan["scope"] = serde_json::json!({ "spaceId": space_id, "documentId": document_id });
        plan["artifact"]["schema"] = serde_json::json!(schema);
        plan["surface"]["surfaceId"] = serde_json::json!(surface_id);
        plan["checkpoint"]["baselineFrontier"]["documentId"] = serde_json::json!(document_id);
        let receipt = plan["receipt"].as_str().expect("fixture receipt").to_string();
        transport.push_response(FakeTransport::json_response(200, &plan).await).await;
        receipt
    }

    /// 🪪️ The exact receipt-free lease projection of the same neutral plan {@link push_document_plan}
    /// serves, at pinned byte lengths. It is built through the one shared relation, so a test can only
    /// make admission fail by changing a real field.
    fn document_lease_fields(space_id: &str, document_id: &str, schema: &str, surface_id: &str) -> DocumentExecutionTargetLeaseFieldsV1 {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json")).expect("neutral plan fixture");
        let mut plan = fixture["validPlan"].clone();
        plan["expiresAtUnixMs"] = serde_json::json!(u64::try_from(wall_now_ms()).expect("wall clock") + 20_000);
        plan["scope"] = serde_json::json!({ "spaceId": space_id, "documentId": document_id });
        plan["artifact"]["schema"] = serde_json::json!(schema);
        plan["surface"]["surfaceId"] = serde_json::json!(surface_id);
        plan["checkpoint"]["baselineFrontier"]["documentId"] = serde_json::json!(document_id);
        let decoded: DocumentOpenPlanV1 = crate::os_pack::json::from_json_str(&serde_json::to_string(&plan).expect("plan json")).expect("neutral plan");
        lease_fields_from_plan_v1(&decoded, 1_024, 512)
    }

    fn document_expectation(schema: &str, surface_id: Option<&str>) -> DocumentSocketExpectationV1 {
        DocumentSocketExpectationV1 {
            artifact_schema: schema.to_string(),
            pack_schema_hash: [0x11; 32],
            requested_surface_id: surface_id.map(str::to_string),
            lease: None,
        }
    }

    /// 🪪️ The one shared full-field lease relation, driven by the language-neutral
    /// `document-execution-target-lease-v1` corpus: the positive GIS Map viewer vector's plan
    /// projection equals its manifest, its exact component and descriptor bytes hash to the declared
    /// SHA-256/BLAKE3 digests, and every single-field substitution in the corpus is denied by the
    /// same relation the retained `DocumentSocketAuthorityV1` uses.
    #[test]
    fn execution_target_lease_compares_every_plan_and_verified_byte_field() {
        let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🌎️hub/🧪️fixtures/📇️directory/🔏️document-execution-target-lease-v1/🔣️.json")).expect("execution target lease corpus");
        let decode_fields = |value: &serde_json::Value| -> Result<DocumentExecutionTargetLeaseFieldsV1, ()> {
            crate::os_pack::json::from_json_str::<DocumentExecutionTargetLeaseFieldsV1>(&serde_json::to_string(value).expect("fields json")).map_err(|_| ())
        };
        let manifest = decode_fields(&corpus["manifest"]).expect("corpus manifest");
        manifest.validate().expect("corpus manifest is a valid lease projection");
        let hex_bytes = |text: &str| -> Vec<u8> { (0..text.len() / 2).map(|index| u8::from_str_radix(&text[index * 2..index * 2 + 2], 16).expect("hex")).collect() };
        let component = hex_bytes(corpus["componentHex"].as_str().expect("component hex"));
        let descriptor_bytes = hex_bytes(corpus["descriptorHex"].as_str().expect("descriptor hex"));
        assert_eq!(semio_framework_hash::sha256_hex(&component), manifest.component.sha256);
        assert_eq!(semio_framework_hash::hash_bytes(&component), manifest.component.blake3);
        assert_eq!(semio_framework_hash::sha256_hex(&descriptor_bytes), manifest.descriptor.sha256);
        assert_eq!(component.len() as u64, manifest.component.byte_length);
        assert_eq!(descriptor_bytes.len() as u64, manifest.descriptor.byte_length);

        let mut plan_json = corpus["plan"].clone();
        plan_json["expiresAtUnixMs"] = serde_json::json!(u64::try_from(wall_now_ms()).expect("wall clock") + 20_000);
        let plan: DocumentOpenPlanV1 = crate::os_pack::json::from_json_str(&serde_json::to_string(&plan_json).expect("plan json")).expect("corpus plan");
        plan.validate(u64::try_from(wall_now_ms()).expect("wall clock")).expect("corpus plan validates");
        let projected = lease_fields_from_plan_v1(&plan, manifest.component.byte_length, manifest.descriptor.byte_length);
        assert!(same_lease_fields_v1(&projected, &manifest));

        let authority = DocumentSocketAuthorityV1 {
            hub_origin: corpus["hubOrigin"].as_str().expect("hub origin").to_string(),
            expires_at_unix_ms: plan.expires_at_unix_ms,
            scope: plan.scope.clone(),
            descriptor_digest_v1: plan.descriptor_digest_v1.clone(),
            catalog: plan.catalog.clone(),
            package: plan.package.clone(),
            artifact: plan.artifact.clone(),
            parent_dialect: plan.parent_dialect.clone(),
            pack_schema_hash: decode_lower_hex_32(&plan.artifact.pack_schema_hash).expect("pack schema hash"),
            surface: plan.surface.clone(),
            grant: plan.grant,
            checkpoint: plan.checkpoint.clone(),
            revalidation: plan.revalidation,
        };
        assert!(authority.matches_lease_fields(&manifest));

        let mut substitutions = 0usize;
        for vector in corpus["hostile"].as_array().expect("hostile rows") {
            if vector["kind"] != "manifest-field" {
                continue;
            }
            assert_eq!(vector["expected"], "unpublished");
            let mut candidate = corpus["manifest"].clone();
            let mut cursor = &mut candidate;
            let path: Vec<&str> = vector["path"].as_str().expect("hostile path").split('.').collect();
            for segment in &path[..path.len() - 1] {
                cursor = cursor.get_mut(*segment).expect("hostile path segment");
            }
            cursor[path[path.len() - 1]] = vector["value"].clone();
            let denied = match decode_fields(&candidate) {
                Err(()) => true,
                Ok(mutated) => mutated.validate().is_err() || (!same_lease_fields_v1(&projected, &mutated) && !authority.matches_lease_fields(&mutated)),
            };
            assert!(denied, "single-field substitution {} was admitted", vector["name"]);
            substitutions += 1;
        }
        assert!(substitutions >= 30, "corpus lost single-field substitutions: {substitutions}");
    }

    #[test]
    fn url_components_percent_encode_utf8_punctuation_without_aliases() {
        assert_eq!(encode_url_component("space /東京?#"), "space%20%2F%E6%9D%B1%E4%BA%AC%3F%23");
        assert_eq!(encode_url_component("a-z_A.9~"), "a-z_A.9~");
    }

    #[semio_framework_async_macros::async_test]
    async fn native_document_admission_issues_validates_and_exchanges_exactly_once() {
        let transport = FakeTransport::default();
        let space_id = "space /東京?";
        let document_id = "document#ä";
        let surface_id = "surface /editor?#";
        let receipt = push_document_plan(&transport, space_id, document_id, "demo/v1", surface_id).await;
        push_grant(&transport).await;
        let client = authenticated_client(transport.clone(), "protected-session");
        let mut expectation = document_expectation("demo/v1", Some(surface_id));
        expectation.lease = Some(document_lease_fields(space_id, document_id, "demo/v1", surface_id));

        let admission = client
            .admit_document_socket(&root_ctx(), space_id, document_id, &expectation, "native-instance", 30_000)
            .expect("document admission");
        assert_eq!(admission.authority.scope, DocumentScope::new(space_id, document_id));
        assert_eq!(admission.authority.artifact.schema, "demo/v1");
        assert_eq!(admission.authority.surface.surface_id, surface_id);
        assert_eq!(admission.authority.pack_schema_hash, [0x11; 32]);
        assert_eq!(admission.authority.package.plugin_id, "s.gis:地図");
        assert_eq!(admission.authority.package.package_id, "s.gis.gismap:codec");
        assert_eq!(admission.authority.package.version, "1.0.0:β");
        assert_eq!(
            admission.authority.parent_dialect,
            DocumentOpenParentDialectV1 { artifact_kind: "s.gis:gismap".into(), standard: "1".into(), subset: "*".into() }
        );
        assert_eq!(admission.authority.surface.app_id, "app.gis");
        assert_eq!(admission.authority.surface.window_kind_id, "window.document");
        assert_eq!(admission.authority.surface.renderer_target, DocumentOpenRendererTargetV1::React);

        let requests = transport.requests.lock().unwrap();
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].url, "http://hub.local/spaces/space%20%2F%E6%9D%B1%E4%BA%AC%3F/documents/document%23%C3%A4/open-plan");
        assert_eq!(requests[1].url, "http://hub.local/spaces/space%20%2F%E6%9D%B1%E4%BA%AC%3F/documents/document%23%C3%A4/socket-grants");
        let intent: serde_json::Value = serde_json::from_slice(&requests[0].body).expect("independent intent decode");
        assert_eq!(intent, serde_json::json!({
            "schema": "semio.hub.document-open-intent/v1",
            "version": 1,
            "scope": { "spaceId": space_id, "documentId": document_id },
            "requestedSurfaceId": surface_id,
            "clientInstanceId": "native-instance"
        }));
        let exchange: serde_json::Value = serde_json::from_slice(&requests[1].body).expect("independent exchange decode");
        assert_eq!(exchange, serde_json::json!({
            "schema": "semio.hub.document-plan-socket-grant-intent/v1",
            "version": 1,
            "planReceipt": receipt
        }));
        assert!(!requests[0].body.windows(receipt.len()).any(|window| window == receipt.as_bytes()));
    }

    #[semio_framework_async_macros::async_test]
    async fn hostile_or_cancelled_plan_never_reaches_receipt_exchange() {
        let transport = FakeTransport::default();
        let receipt = push_document_plan(&transport, "space", "document", "foreign/v1", "surface").await;
        let client = authenticated_client(transport.clone(), "protected-session");
        let expectation = document_expectation("expected/v1", Some("surface"));
        let error = match client.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", 5_000) {
            Err(error) => error,
            Ok(_) => panic!("schema substitution accepted"),
        };
        assert!(!error.to_string().contains(&receipt));
        assert_eq!(transport.requests.lock().unwrap().len(), 1, "invalid plan is never exchanged");

        let cancelled_transport = FakeTransport::default();
        push_document_plan(&cancelled_transport, "space", "document", "expected/v1", "surface").await;
        cancelled_transport.cancel_after_grant.store(true, Ordering::SeqCst);
        let cancelled = authenticated_client(cancelled_transport.clone(), "protected-session");
        assert!(matches!(cancelled.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", 5_000), Err(DirectoryClientError::Cancelled)));
        assert_eq!(cancelled_transport.requests.lock().unwrap().len(), 1, "cancel after plan prevents exchange");
    }

    #[semio_framework_async_macros::async_test]
    async fn cancellation_after_receipt_exchange_never_reaches_a_document_socket() {
        let transport = FakeTransport::default();
        push_document_plan(&transport, "space", "document", "expected/v1", "surface").await;
        push_grant(&transport).await;
        transport.cancel_after_grant_number.store(2, Ordering::SeqCst);
        let client = authenticated_client(transport.clone(), "protected-session");
        let expectation = document_expectation("expected/v1", Some("surface"));

        assert!(matches!(client.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", 5_000), Err(DirectoryClientError::Cancelled)));
        assert_eq!(transport.requests.lock().unwrap().len(), 2, "plan and receipt exchange completed once");
        assert!(transport.ws_urls.lock().unwrap().is_empty(), "cancelled exchanged grant never reaches a socket URL or protocol header");
    }

    #[semio_framework_async_macros::async_test]
    async fn protected_document_admission_is_bounded_and_redacts_hostile_responses() {
        let oversized = FakeTransport::default();
        oversized.push_response(Ok(HttpResponse { status: 200, body: vec![b'x'; DOCUMENT_ADMISSION_RESPONSE_MAX_BYTES + 1] })).await;
        let client = authenticated_client(oversized.clone(), "protected-session");
        let expectation = document_expectation("expected/v1", None);
        let error = match client.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", u64::MAX) {
            Err(error) => error,
            Ok(_) => panic!("oversized plan accepted"),
        };
        assert_eq!(error.to_string(), "decode: protected document admission response exceeded 64 KiB");
        assert_eq!(oversized.requests.lock().unwrap().len(), 1);

        let hostile = FakeTransport::default();
        let secret = format!("plan.v1.{}.{}", "a".repeat(32), "b".repeat(43));
        hostile.push_response(Ok(HttpResponse { status: 500, body: secret.as_bytes().to_vec() })).await;
        let client = authenticated_client(hostile.clone(), "protected-session");
        let error = match client.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", 5_000) {
            Err(error) => error,
            Ok(_) => panic!("hostile response accepted"),
        };
        assert!(!error.to_string().contains(&secret));
        assert_eq!(error.to_string(), "http 500: protected document admission rejected");
    }

    #[semio_framework_async_macros::async_test]
    async fn mismatched_local_plugin_selection_never_exchanges_a_plan_receipt() {
        let transport = FakeTransport::default();
        let receipt = push_document_plan(&transport, "space", "document", "expected/v1", "surface").await;
        let client = authenticated_client(transport.clone(), "protected-session");
        let mut expectation = document_expectation("expected/v1", Some("surface"));
        let mut lease = document_lease_fields("space", "document", "expected/v1", "surface");
        lease.package.package_id = "foreign.package".into();
        expectation.lease = Some(lease);
        let error = match client.admit_document_socket(&root_ctx(), "space", "document", &expectation, "native-instance", 5_000) {
            Err(error) => error,
            Ok(_) => panic!("foreign plugin selection accepted"),
        };
        assert!(!error.to_string().contains(&receipt));
        assert_eq!(transport.requests.lock().unwrap().len(), 1);
    }

    #[semio_framework_async_macros::async_test]
    async fn ws_url_switches_scheme_and_encodes_query() {
        assert_eq!(directory_ws_url("http://127.0.0.1:8787", 0), "ws://127.0.0.1:8787/directory/socket/v1?since=0");
        assert_eq!(directory_ws_url("https://hub.example", 42), "wss://hub.example/directory/socket/v1?since=42");
        assert_eq!(
            directory_scoped_ws_url("https://hub.example", &DocumentScope::new("space /a", "document#b"), 7),
            "wss://hub.example/directory/spaces/space%20%2Fa/documents/document%23b/socket/v1?since=7"
        );
    }

    #[semio_framework_async_macros::async_test]
    async fn scoped_stream_close_4401_is_terminal_and_never_redials() {
        let client = Arc::new(authenticated_client(FakeTransport::default(), "tok"));
        let scope = DocumentScope::new("space-a", "document-a");
        let mut stream = client.stream_scoped(scope.clone(), 9);
        let DirectoryStreamTurn::DialScoped { scope: dial_scope, since, .. } = stream.turn(&root_ctx(), 0) else { panic!("scoped stream must dial exact scope") };
        assert_eq!(dial_scope, scope);
        assert_eq!(since, 9);
        assert!(matches!(stream.complete_dial(0, Ok(FakeWs::with_close_code(4401))), DirectoryStreamTurn::Idle));
        assert!(matches!(stream.turn(&root_ctx(), 1), DirectoryStreamTurn::Revoked(revoked) if revoked == scope));
        assert!(matches!(stream.turn(&root_ctx(), u64::MAX), DirectoryStreamTurn::Closed));
    }

    #[semio_framework_async_macros::async_test]
    async fn scoped_stream_issues_and_dials_the_same_encoded_scope() {
        let transport = FakeTransport::default();
        push_grant(&transport).await;
        transport.push_ws(Ok(std::collections::VecDeque::new())).await;
        let client = authenticated_client(transport.clone(), "tok");
        let scope = DocumentScope::new("space /a", "document#b");
        let _connection = client.open_scoped_stream_ws(&root_ctx(), &scope, 7, 100).expect("scoped socket opens");
        let requests = transport.requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].url, "http://hub.local/directory/spaces/space%20%2Fa/documents/document%23b/socket-grants");
        assert!(requests[0].body.is_empty());
        assert_eq!(transport.ws_urls.lock().unwrap().as_slice(), ["ws://hub.local/directory/spaces/space%20%2Fa/documents/document%23b/socket/v1?since=7"]);
    }

    #[semio_framework_async_macros::async_test]
    async fn backoff_doubles_and_caps() {
        assert_eq!(next_backoff_ms(500), 1000);
        assert_eq!(next_backoff_ms(20_000), 30_000);
        assert_eq!(next_backoff_ms(30_000), 30_000);
        assert_eq!(next_backoff_ms(0), HUB_RECONNECT_MIN_MS);
    }

    #[semio_framework_async_macros::async_test]
    async fn spaces_decodes_and_sends_bearer() {
        let transport = FakeTransport::default();
        transport.push_response(FakeTransport::json_response(200, &serde_json::json!([])).await).await;
        let client = authenticated_client(transport.clone(), "tok");

        let spaces = client.spaces(&root_ctx()).await.expect("decodes");
        assert!(spaces.is_empty());
        let requests = transport.requests.lock().unwrap();
        assert_eq!(requests[0].url, "http://hub.local/directory/spaces");
        assert_eq!(requests[0].bearer.as_deref(), Some("tok"));
    }

    #[semio_framework_async_macros::async_test]
    async fn space_exposes_the_durable_document_descriptor() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧫️fixtures/📇️directory/🪪️document-descriptor.json")).expect("descriptor fixture");
        let descriptor = fixture.get("valid").expect("valid descriptor").clone();
        let document: DocumentView = crate::os_pack::json::from_json_str(&serde_json::json!({ "descriptor": descriptor, "headSeq": 7, "commitSeq": 6, "epoch": 2 }).to_string()).expect("document view fixture");
        let space = MemberSpaceViewV1 {
            id: "space-a".into(),
            name: "Fixture".into(),
            kind: crate::os_directory::DirectorySpaceKind::Studio,
            visibility: crate::os_directory::DirectorySpaceVisibility::Private,
            owner_user_id: "user-owner".into(),
            role: DirectorySpaceRole::Author,
            member_count: 1,
            document_count: 1,
            active_connections: 0,
            created_at_ms: 1,
            updated_at_ms: 2,
        };
        let mut page = DirectorySpaceAdministrationPageV1::Author {
            schema: crate::os_directory::DIRECTORY_SPACE_ADMINISTRATION_PAGE_SCHEMA.into(),
            session_binding_sha256: "a".repeat(64),
            authorization_generation: 7,
            space_id: "space-a".into(),
            space,
            members: DirectorySpaceAdministrationMemberWindowV1 { rows: Vec::new(), next_cursor: None },
            documents: DirectorySpaceAdministrationDocumentWindowV1 { rows: vec![document], next_cursor: None },
            invites: DirectorySpaceAdministrationInviteWindowV1 { rows: Vec::new(), next_cursor: None },
            capabilities: DirectorySpaceAdministrationCapabilitiesV1 { rename_space: true, set_visibility: true, delete_space: true, upsert_member: true, remove_member: true, create_invite: true, revoke_invite: true },
            receipt_sha256: String::new(),
        };
        let receipt = semio_framework_hash::sha256_hex(page.canonical_unsigned_json().as_bytes());
        if let DirectorySpaceAdministrationPageV1::Author { receipt_sha256, .. } = &mut page {
            *receipt_sha256 = receipt;
        }
        let canonical = crate::os_pack::json::to_json_string(&page);
        assert!(!canonical.contains("selector") && !canonical.contains("secretDigest") && !canonical.contains("passwordHash"));
        let transport = FakeTransport::default();
        transport.push_response(Ok(HttpResponse { status: 200, body: canonical.clone().into_bytes() })).await;
        let client = authenticated_client(transport.clone(), "member-token");

        let fetched = client.space_administration_page(&root_ctx(), "space-a", None).await.expect("administration page decodes");
        assert_eq!(fetched.canonical_json(), canonical);
        let DirectorySpaceAdministrationPageV1::Author { documents, .. } = fetched.page().clone() else { panic!("author projection") };
        assert_eq!(documents.rows[0].descriptor.document_id, "shared-document");
        assert_eq!(documents.rows[0].descriptor.owner.plugin_id, "s.gis");
        assert_eq!(documents.rows[0].descriptor.bootstrap_frontier.head_seq, 7);
        let requests = transport.requests.lock().unwrap();
        assert_eq!(requests[0].bearer.as_deref(), Some("member-token"));
    }

    #[semio_framework_async_macros::async_test]
    async fn unauthorized_status_maps_to_unauthorized_error() {
        let transport = FakeTransport::default();
        transport.push_response(Ok(HttpResponse { status: 401, body: Vec::new() })).await;
        let client = DirectoryClient::new(transport, "http://hub.local");

        let error = client.me(&root_ctx()).await.expect_err("401 is unauthorized");
        assert!(matches!(error, DirectoryClientError::Unauthorized));
    }

    #[semio_framework_async_macros::async_test]
    async fn stream_reconnects_and_resumes_from_last_seq() {
        let transport = FakeTransport::default();
        transport.push_ws(Err(TransportError::Io("refused".to_string()))).await;
        let event =
            serde_json::json!({ "kind": "event", "event": { "seq": 7, "id": "e1", "hlc": { "physicalMs": 1, "logical": 0 }, "actor": { "kind": "system", "id": "sys" }, "body": { "kind": "space.archived", "spaceId": "sp-1" }, "recordedAtMs": 1 } })
                .to_string();
        transport.push_ws(Ok(std::collections::VecDeque::from([Ok(Some(event)), Ok(None)]))).await;
        transport.push_ws(Err(TransportError::Io("refused again".to_string()))).await;
        push_grant(&transport).await;
        push_grant(&transport).await;
        push_grant(&transport).await;

        let client = authenticated_client(transport.clone(), "tok");
        let mut stream = client.stream(0);
        let ctx = root_ctx();

        let DirectoryStreamTurn::Dial { client: dial, since } = stream.turn(&ctx, 0) else { panic!("first turn must dial") };
        assert!(matches!(stream.complete_dial(0, dial.open_stream_ws(&ctx, since, 100).map_err(|error| TransportError::Io(error.to_string()))), DirectoryStreamTurn::ReconnectAt(HUB_RECONNECT_MIN_MS)));
        assert!(matches!(stream.turn(&ctx, HUB_RECONNECT_MIN_MS), DirectoryStreamTurn::Dial { .. }));
        let result = client.open_stream_ws(&ctx, 0, 100).map_err(|error| TransportError::Io(error.to_string()));
        assert!(matches!(stream.complete_dial(HUB_RECONNECT_MIN_MS, result), DirectoryStreamTurn::Idle));
        match stream.turn(&ctx, HUB_RECONNECT_MIN_MS) {
            DirectoryStreamTurn::Message(DirectoryStreamMessage::Event { event }) => assert_eq!(event.seq, 7),
            _ => panic!("second connection must deliver the event"),
        }
        assert_eq!(stream.since(), 7);
        assert!(matches!(stream.turn(&ctx, HUB_RECONNECT_MIN_MS), DirectoryStreamTurn::ReconnectAt(1_000)));
        let DirectoryStreamTurn::Dial { client: dial, since } = stream.turn(&ctx, 1_000) else { panic!("reconnect deadline must dial") };
        assert!(matches!(stream.complete_dial(1_000, dial.open_stream_ws(&ctx, since, 100).map_err(|error| TransportError::Io(error.to_string()))), DirectoryStreamTurn::ReconnectAt(2_000)));

        let ws_urls = transport.ws_urls.lock().unwrap();
        assert_eq!(ws_urls.len(), 3);
        assert!(ws_urls[0].ends_with("since=0"));
        assert!(ws_urls[2].ends_with("since=7"), "the resumed dial carries the last-seen seq, not the original since");
    }

    #[semio_framework_async_macros::async_test]
    async fn stream_turn_is_bounded_and_preserves_event_order() {
        let transport = FakeTransport::default();
        let frames = (1..=2)
            .map(|seq| {
                Ok(Some(
                    serde_json::json!({ "kind": "event", "event": { "seq": seq, "id": format!("e{seq}"), "hlc": { "physicalMs": seq, "logical": 0 }, "actor": { "kind": "system", "id": "sys" }, "body": { "kind": "space.archived", "spaceId": format!("sp-{seq}") }, "recordedAtMs": seq } })
                        .to_string(),
                ))
            })
            .collect();
        transport.push_ws(Ok(frames)).await;
        push_grant(&transport).await;
        let client = authenticated_client(transport, "tok");
        let ctx = root_ctx();
        let mut stream = client.stream(0);
        let DirectoryStreamTurn::Dial { client, since } = stream.turn(&ctx, 0) else { panic!("first turn must dial") };
        assert!(matches!(stream.complete_dial(0, client.open_stream_ws(&ctx, since, 100).map_err(|error| TransportError::Io(error.to_string()))), DirectoryStreamTurn::Idle));

        let started = std::time::Instant::now();
        let mut seqs = Vec::new();
        for _ in 0..2 {
            match stream.turn(&ctx, 0) {
                DirectoryStreamTurn::Message(DirectoryStreamMessage::Event { event }) => seqs.push(event.seq),
                _ => panic!("scripted event must be delivered"),
            }
        }
        assert_eq!(seqs, vec![1, 2]);
        assert!(started.elapsed() < std::time::Duration::from_millis(8));
    }

    /// 🧪️ The closed command boundary: a canonical sealed request in, a raw-byte-capped canonical
    /// receipt out, closed codes for every failure, and no raw server text in any UI-facing error.
    #[semio_framework_async_macros::async_test]
    async fn directory_command_parses_only_a_bounded_canonical_receipt_and_never_echoes_server_text() {
        let capability = format!("session.v1.{}.{}", "a".repeat(32), "b".repeat(64));
        let command = DirectoryCommand::CreateInvite { space_id: "space-a".into(), role: DirectorySpaceRole::Spectator, ttl_secs: 3_600 };
        let request = DirectoryCommandRequestV1::new("1f2e3d4c5b6a7988a1b2c3d4e5f60718", command.clone());
        let receipt = DirectoryCommandReceiptV1::seal(
            request.request_id.clone(),
            directory_command_sha256(&command),
            DirectoryCommandOutcomeV1::Accepted,
            Vec::new(),
            DirectoryCommandResultV1::Invite { invite_token: "invite.v1.one-shot".into() },
        );
        let canonical = crate::os_pack::json::to_json_string(&receipt);

        let transport = FakeTransport::default();
        transport.push_response(Ok(HttpResponse { status: 200, body: canonical.as_bytes().to_vec() })).await;
        let client = authenticated_client(transport.clone(), &capability);
        let delivered = client.command(&root_ctx(), &request).await.expect("canonical receipt");
        assert_eq!(delivered.canonical_json, canonical);
        assert_eq!(delivered.receipt, receipt);
        assert_eq!(transport.requests.lock().unwrap().first().map(|entry| entry.body.clone()).expect("sealed request bytes"), request.canonical_json().into_bytes());

        for (status, code) in [(401u16, DirectoryCommandErrorCodeV1::Unauthorized), (403, DirectoryCommandErrorCodeV1::Forbidden), (409, DirectoryCommandErrorCodeV1::RequestConflict), (413, DirectoryCommandErrorCodeV1::TooLarge), (503, DirectoryCommandErrorCodeV1::Overloaded), (500, DirectoryCommandErrorCodeV1::Invalid)] {
            let transport = FakeTransport::default();
            transport.push_response(Ok(HttpResponse { status, body: b"hub text a UI-facing error must never carry".to_vec() })).await;
            let client = authenticated_client(transport, &capability);
            let error = client.command(&root_ctx(), &request).await.expect_err("closed denial");
            assert_eq!(error, code, "status {status}");
            assert!(!format!("{error:?}").contains("hub text"));
        }

        let oversized = FakeTransport::default();
        oversized.push_response(Ok(HttpResponse { status: 200, body: vec![b'x'; DIRECTORY_COMMAND_RECEIPT_MAX_BYTES + 1] })).await;
        assert_eq!(authenticated_client(oversized, &capability).command(&root_ctx(), &request).await.expect_err("byte ceiling"), DirectoryCommandErrorCodeV1::TooLarge);

        let forged = FakeTransport::default();
        let mut substituted = receipt.clone();
        substituted.command_sha256 = directory_command_sha256(&DirectoryCommand::RenameSpace { space_id: "space-a".into(), name: "Substituted".into() });
        forged.push_response(Ok(HttpResponse { status: 200, body: crate::os_pack::json::to_json_string(&substituted).into_bytes() })).await;
        assert_eq!(authenticated_client(forged, &capability).command(&root_ctx(), &request).await.expect_err("digest substitution"), DirectoryCommandErrorCodeV1::Invalid);

        let redacted = FakeTransport::default();
        let leaking = DirectoryCommandReceiptV1::seal(request.request_id.clone(), directory_command_sha256(&command), DirectoryCommandOutcomeV1::SecretUndeliverable, Vec::new(), DirectoryCommandResultV1::Invite { invite_token: "invite.v1.replayed".into() });
        redacted.push_response(Ok(HttpResponse { status: 200, body: crate::os_pack::json::to_json_string(&leaking).into_bytes() })).await;
        assert_eq!(authenticated_client(redacted, &capability).command(&root_ctx(), &request).await.expect_err("redaction violation"), DirectoryCommandErrorCodeV1::Invalid);

        let cancelled = FakeTransport::default();
        cancelled.push_response(Ok(HttpResponse { status: 200, body: canonical.as_bytes().to_vec() })).await;
        let client = authenticated_client(cancelled.clone(), &capability);
        let ctx = root_ctx();
        ctx.cancel.cancel_now();
        assert_eq!(client.command(&ctx, &request).await.expect_err("pre-cancelled"), DirectoryCommandErrorCodeV1::Cancelled);
        assert!(cancelled.requests.lock().unwrap().is_empty(), "an already-cancelled operation never builds a command request");

        let mut malformed = request.clone();
        malformed.request_id = "not-hex".into();
        assert_eq!(authenticated_client(FakeTransport::default(), &capability).command(&root_ctx(), &malformed).await.expect_err("malformed correlation"), DirectoryCommandErrorCodeV1::Invalid);
    }

    //#region 🔖️CancellationTests
    /// 🧪️ A `ctx` that is ALREADY cancelled before the call starts must never reach the transport
    /// at all — `request_json`'s own up-front check (see `DirectoryClientError::Cancelled`'s doc).
    #[semio_framework_async_macros::async_test]
    async fn a_request_with_an_already_cancelled_context_never_reaches_the_transport() {
        let transport = FakeTransport::default();
        transport.push_response(FakeTransport::json_response(200, &serde_json::json!([])).await).await;
        let client = DirectoryClient::new(transport.clone(), "http://hub.local");
        let ctx = root_ctx();
        ctx.cancel.cancel_now();

        let result = client.spaces(&ctx).await;
        assert!(matches!(result, Err(DirectoryClientError::Cancelled)), "got {result:?}");
        assert!(transport.requests.lock().unwrap().is_empty(), "an already-cancelled context must never even build a request");
    }

    /// 🧪️ The property this ticket asks for: an IN-FLIGHT request — already past
    /// `request_json`'s up-front check, genuinely inside the transport call — is cancelled once its
    /// `OperationContext` is cancelled. `FakeTransport::yields_before_response` gives `http()` two
    /// cooperative yield points (checking `ctx.cancel` at each); `semio_framework_async::join2` drives
    /// the request future and a "canceller" future that cancels after ONE yield in lockstep on the
    /// SAME thread — no real time, no real thread, fully deterministic.
    #[semio_framework_async_macros::async_test]
    async fn an_in_flight_request_is_cancelled_when_its_context_is_cancelled() {
        let transport = FakeTransport::default();
        transport.yields_before_response.store(2, std::sync::atomic::Ordering::SeqCst);
        transport.push_response(FakeTransport::json_response(200, &serde_json::json!([])).await).await;
        let client = DirectoryClient::new(transport.clone(), "http://hub.local");
        let ctx = root_ctx();

        let request_fut = client.spaces(&ctx);
        let canceller_fut = async {
            semio_framework_async::yield_once().await;
            ctx.cancel.cancel().await;
        };
        let (result, ()) = semio_framework_async::join2(request_fut, canceller_fut).await;
        assert!(matches!(result, Err(DirectoryClientError::Transport(TransportError::Cancelled))), "an in-flight request must observe cancellation, got {result:?}");
        assert!(transport.requests.lock().unwrap().is_empty(), "the cancelled call must never reach the scripted response — the response stays queued, unconsumed");
    }

    /// 🧪️ A cancelled context closes the finite stream state machine rather than reconnecting.
    #[semio_framework_async_macros::async_test]
    async fn cancelling_the_context_closes_an_open_stream() {
        let transport = FakeTransport::default();
        let client = Arc::new(DirectoryClient::new(transport, "http://hub.local"));
        let mut stream = client.stream(0);
        let ctx = root_ctx();
        ctx.cancel.cancel_now();

        assert!(matches!(stream.turn(&ctx, 0), DirectoryStreamTurn::Closed));
        assert!(matches!(stream.turn(&root_ctx(), 0), DirectoryStreamTurn::Closed));
    }

    /// 🧪️ Cancellation while a grant/dial is outstanding closes a late socket exactly once.
    #[semio_framework_async_macros::async_test]
    async fn cancelling_a_pending_grant_dial_closes_its_late_socket() {
        use super::test_support::FakeWs;
        use std::sync::atomic::{AtomicUsize, Ordering};

        let client = authenticated_client(FakeTransport::default(), "tok");
        let mut stream = client.stream(0);
        let ctx = root_ctx();
        assert!(matches!(stream.turn(&ctx, 0), DirectoryStreamTurn::Dial { .. }));

        ctx.cancel.cancel_now();
        assert!(matches!(stream.turn(&ctx, 1), DirectoryStreamTurn::Closed));
        let closes = Arc::new(AtomicUsize::new(0));
        assert!(matches!(stream.complete_dial(2, Ok(FakeWs::with_close_observer(closes.clone()))), DirectoryStreamTurn::Closed));
        assert_eq!(closes.load(Ordering::SeqCst), 1);
        assert!(matches!(stream.turn(&root_ctx(), 3), DirectoryStreamTurn::Closed));
    }

    /// 🧪️ A cancellation that races the fresh grant prevents any socket dial from starting.
    #[semio_framework_async_macros::async_test]
    async fn cancellation_after_grant_refresh_never_opens_or_greets_a_socket() {
        let transport = FakeTransport::default();
        push_grant(&transport).await;
        transport.cancel_after_grant.store(true, Ordering::SeqCst);
        let client = authenticated_client(transport.clone(), "tok");

        assert!(matches!(client.open_stream_ws(&root_ctx(), 0, 100), Err(DirectoryClientError::Cancelled)));
        assert!(transport.ws_urls.lock().unwrap().is_empty());
        assert_eq!(transport.ws_sends.load(Ordering::SeqCst), 0);
        assert_eq!(transport.ws_closes.load(Ordering::SeqCst), 0);
    }

    /// 🧪️ A cancellation after transport open but before tag-7 greeting closes the socket exactly
    /// once and sends no frame.
    #[semio_framework_async_macros::async_test]
    async fn cancellation_after_socket_open_closes_before_socket_hello() {
        let transport = FakeTransport::default();
        push_grant(&transport).await;
        transport.push_ws(Ok(std::collections::VecDeque::new())).await;
        transport.cancel_after_open.store(true, Ordering::SeqCst);
        let client = authenticated_client(transport.clone(), "tok");

        assert!(matches!(client.open_stream_ws(&root_ctx(), 0, 100), Err(DirectoryClientError::Cancelled)));
        assert_eq!(transport.ws_urls.lock().unwrap().len(), 1);
        assert_eq!(transport.ws_sends.load(Ordering::SeqCst), 0);
        assert_eq!(transport.ws_closes.load(Ordering::SeqCst), 1);
    }
    //#endregion 🔖️CancellationTests
}
//#endregion 🧪️Tests
