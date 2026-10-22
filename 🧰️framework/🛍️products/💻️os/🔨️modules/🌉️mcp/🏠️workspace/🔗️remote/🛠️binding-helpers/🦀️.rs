//! 🛠️ Binding helpers: delegated session exchange, origin and resource URIs, canonical checkpoint text and error mapping.

use semio_framework_async::{HostAsyncRuntime, OperationContext};
use semio_framework_os_kernel::os_directory::DocumentScope;
use semio_framework_os_kernel::os_directory::io::binary::artifact_hash::hex_lower;
use semio_framework_os_kernel::os_directory::client::{DirectoryClientError, DirectoryTransport};
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp_foundation::percent_codec::{percent_decode, percent_encode};
use semio_framework_os_mcp_hub_pair::pair;
use semio_framework_os_mcp_hub_pair::binding_errors::{HubBindingError, HubUnavailableCause};
use semio_framework_os_mcp_hub_pair::pair::{CanonicalPairMountError, CanonicalPairMountIdentity};
use semio_framework_os_mcp_schema::schema::{UntrustedAuthors, UntrustedProvenance, UntrustedRevision, UntrustedSource, untrusted_content, untrusted_content_sha256};
use std::sync::Arc;
use std::sync::atomic::Ordering;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_foundation::agent_credential::{
    CredentialExchangeProtocolV1,
    CredentialExchangeRequestV1,
    DelegatedCredentialV1,
    DelegatedSessionGrantV1,
};
use super::binding_limits::{
    CANONICAL_CHECKPOINT_RESOURCE_MAX_TEXT_BYTES,
    CANONICAL_CHECKPOINT_RESOURCE_METADATA_MAX_BYTES,
    CANONICAL_CHECKPOINT_RESOURCE_SCHEMA,
    HUB_BINDING_DIAGNOSTIC_MAX_BYTES,
    HUB_BINDING_ID_MAX_BYTES,
    HUB_BINDING_OPERATION_TIMEOUT_MS,
    HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS,
};
use super::remote_binding::{HubBindingPhase, HubBindingProgress, HubRemoteBinding, HubRemoteBindingState};

/// 🔑️ Exchanges an explicitly selected credential protocol with the shared bounded native transport.
#[cfg(not(target_arch = "wasm32"))]
pub fn exchange_delegated_session_v1(base_url: &str, credential: &DelegatedCredentialV1, protocol: CredentialExchangeProtocolV1) -> Result<DelegatedSessionGrantV1, GatewayError> {
    use semio_framework_actor::{ActorId, PackageId};
    use semio_framework_async::{ProcessKind, ScopeOwner, TraceId, WorkerPoolConfig};
    use semio_framework_os_kernel::os_directory::client::{native::NativeDirectoryTransport, DirectoryTransport, HttpMethod};
    use semio_framework_os_services::{ComputePool, TokioHostRuntime};

    let origin = base_url.trim_end_matches('/');
    if origin != credential.origin().trim_end_matches('/') {
        return Err(GatewayError::new(GatewayErrorCode::PermissionDenied, "--hub origin does not match the origin this agent credential was issued for"));
    }
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    let pool = semio_framework_async::process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores));
    let runtime = Arc::new(TokioHostRuntime::with_pool(pool.clone()));
    let scope = runtime.open_scope_now(ScopeOwner::Service("mcp-agent-session-exchange"), None);
    let compute = Arc::new(ComputePool::with_pool(1, pool));
    let transport = NativeDirectoryTransport::with_new_http_pool_now(runtime.clone(), scope, compute, 1024 * 1024, 1, PackageId("semio-framework-os-mcp".to_string()), ActorId(0x4d43_5002));
    let cancel = semio_framework_async::CancelToken::root_now();
    let operation_now = runtime.block_on(runtime.now_ms());
    let ctx = OperationContext {
        actor: 0x4d43_5002,
        generation: 0,
        trace: TraceId(operation_now),
        lane: 1,
        deadline_ms: Some(operation_now.saturating_add(HUB_BINDING_OPERATION_TIMEOUT_MS)),
        cancel: cancel.child_now(),
        capability: None,
    };
    let request = (protocol.request)(credential, &agent_instance_id())?;
    let request = CredentialExchangeRequestV1::new(request.path, request.body)?;
    let url = format!("{origin}{}", request.path);
    let mut response = runtime
        .block_on(transport.http(&ctx, HttpMethod::Post, &url, Some(credential.expose_for_exchange()), Some(request.body)))
        .map_err(|error| GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("the hub at {origin} did not answer the agent-session exchange: {error:?}")).retryable())?;
    let outcome = if response.status != 200 { Err((protocol.decode_error)(response.status, &response.body)) } else { (protocol.decode_grant)(&response.body, credential) };
    response.body.fill(0);
    outcome
}

/// 🔖️ A per-process instance id, so two agents sharing one delegation still get separate sessions
/// and therefore separate presence rows.
#[cfg(not(target_arch = "wasm32"))]
fn agent_instance_id() -> String {
    format!("mcp.{}", std::process::id())
}

pub fn validate_hub_origin(base_url: &str, space_id: &str) -> Result<(), GatewayError> {
    pair::normalize_hub_origin(base_url).map_err(|_| GatewayError::new(GatewayErrorCode::InputInvalid, "--hub requires a bounded origin-only http(s) URL"))?;
    validate_identity("space id", space_id).map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, error.to_string()))?;
    Ok(())
}

pub fn descriptor_resource_uri(scope: &DocumentScope) -> String {
    format!("semio://workspace/scopes/{}/{}/descriptor", percent_encode(&scope.space_id), percent_encode(&scope.document_id))
}

pub fn parse_descriptor_resource_uri(uri: &str) -> Option<DocumentScope> {
    let rest = uri.strip_prefix("semio://workspace/scopes/")?;
    let mut parts = rest.split('/');
    let space_id = percent_decode(parts.next()?)?;
    let document_id = percent_decode(parts.next()?)?;
    if parts.next()? != "descriptor" || parts.next().is_some() {
        return None;
    }
    Some(DocumentScope::new(space_id, document_id))
}

pub fn checkpoint_resource_uri(scope: &DocumentScope) -> String {
    format!("semio://workspace/scopes/{}/{}/checkpoint", percent_encode(&scope.space_id), percent_encode(&scope.document_id))
}

pub fn parse_checkpoint_resource_uri(uri: &str) -> Option<DocumentScope> {
    let rest = uri.strip_prefix("semio://workspace/scopes/")?;
    let mut parts = rest.split('/');
    let space_id = percent_decode(parts.next()?)?;
    let document_id = percent_decode(parts.next()?)?;
    if parts.next()? != "checkpoint" || parts.next().is_some() || space_id.is_empty() || document_id.is_empty() {
        return None;
    }
    Some(DocumentScope::new(space_id, document_id))
}

pub(crate) fn checked_base64_length(length: usize) -> Result<usize, CanonicalPairMountError> {
    length.checked_add(2).and_then(|value| value.checked_div(3)).and_then(|value| value.checked_mul(4)).ok_or(CanonicalPairMountError::ResourceLimit)
}

pub(crate) fn base64_encode_exact(bytes: &[u8], output_length: usize) -> Result<String, CanonicalPairMountError> {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    if checked_base64_length(bytes.len())? != output_length {
        return Err(CanonicalPairMountError::ResourceLimit);
    }
    let mut output = String::with_capacity(output_length);
    for chunk in bytes.chunks(3) {
        let first = chunk[0];
        let second = *chunk.get(1).unwrap_or(&0);
        let third = *chunk.get(2).unwrap_or(&0);
        output.push(ALPHABET[(first >> 2) as usize] as char);
        output.push(ALPHABET[(((first & 3) << 4) | (second >> 4)) as usize] as char);
        output.push(if chunk.len() > 1 { ALPHABET[(((second & 15) << 2) | (third >> 6)) as usize] as char } else { '=' });
        output.push(if chunk.len() > 2 { ALPHABET[(third & 63) as usize] as char } else { '=' });
    }
    Ok(output)
}

pub(crate) fn canonical_checkpoint_resource_text(
    identity: &CanonicalPairMountIdentity,
    frontier: &semio_framework_os_kernel::os_directory::ArtifactFrontier,
    pack: &[u8],
    spr: &[u8],
) -> Result<String, CanonicalPairMountError> {
    let raw_length = pack.len().checked_add(spr.len()).ok_or(CanonicalPairMountError::ResourceLimit)?;
    if raw_length > pair::HUB_PAIR_MAX_VERIFIED_BYTES {
        return Err(CanonicalPairMountError::ResourceLimit);
    }
    if frontier.document_id != identity.scope.document_id {
        return Err(CanonicalPairMountError::InvalidResponse("canonical checkpoint projection identity mismatch"));
    }
    let pack_base64_length = checked_base64_length(pack.len())?;
    let spr_base64_length = checked_base64_length(spr.len())?;
    let projected_upper_bound = pack_base64_length
        .checked_add(spr_base64_length)
        .and_then(|value| value.checked_add(CANONICAL_CHECKPOINT_RESOURCE_METADATA_MAX_BYTES))
        .ok_or(CanonicalPairMountError::ResourceLimit)?;
    if projected_upper_bound > CANONICAL_CHECKPOINT_RESOURCE_MAX_TEXT_BYTES {
        return Err(CanonicalPairMountError::ResourceLimit);
    }
    let pack_base64 = base64_encode_exact(pack, pack_base64_length)?;
    let spr_base64 = base64_encode_exact(spr, spr_base64_length)?;
    let provenance = UntrustedProvenance {
        source: UntrustedSource::HubCheckpoint,
        artifact_id: Some(identity.scope.document_id.clone()),
        artifact_kind: None,
        space_id: Some(identity.scope.space_id.clone()),
        revision: UntrustedRevision { content_sha256: untrusted_content_sha256(&[pack, spr]), head_edit_id: Some(frontier.head_edit_id.clone()), commit_seq: Some(frontier.last_commit_seq) },
        authors: UntrustedAuthors::SpaceWriters { space_id: identity.scope.space_id.clone() },
    };
    let value = serde_json::json!({
        "schema": CANONICAL_CHECKPOINT_RESOURCE_SCHEMA,
        "scope": { "spaceId": identity.scope.space_id, "documentId": identity.scope.document_id },
        "descriptorDigestV1": identity.descriptor_digest_v1,
        "activeCheckpointId": identity.active_checkpoint_id,
        "etag": identity.etag,
        "authorityGeneration": identity.authority_generation,
        "catalogGeneration": identity.catalog_generation,
        "frontier": {
            "documentId": frontier.document_id,
            "headEditOrdinal": frontier.head_edit_ordinal,
            "headEditId": frontier.head_edit_id,
            "lastCommitSeq": frontier.last_commit_seq,
            "chainHash": hex_lower(&frontier.chain_hash.0)
        },
        "pack": { "byteLength": pack.len(), "sha256": framework_hash::sha256_hex(pack) },
        "spr": { "byteLength": spr.len(), "sha256": framework_hash::sha256_hex(spr) },
        "untrusted": untrusted_content(&provenance, serde_json::json!({ "packBase64": pack_base64, "sprBase64": spr_base64 }))
    });
    let text = serde_json::to_string(&value).map_err(|_| CanonicalPairMountError::InvalidResponse("canonical checkpoint resource serialization failed"))?;
    if text.len() > CANONICAL_CHECKPOINT_RESOURCE_MAX_TEXT_BYTES {
        return Err(CanonicalPairMountError::ResourceLimit);
    }
    Ok(text)
}

pub(super) fn validate_identity(field: &'static str, value: &str) -> Result<(), HubBindingError> {
    if value.is_empty() || value.len() > HUB_BINDING_ID_MAX_BYTES || value.chars().any(char::is_control) {
        return Err(HubBindingError::InvalidResponse(field));
    }
    Ok(())
}

pub(crate) fn validate_document_count(actual: usize, declared: u32) -> Result<(), HubBindingError> {
    if actual > HUB_DESCRIPTOR_INDEX_MAX_DOCUMENTS {
        return Err(HubBindingError::CapacityExceeded);
    }
    if usize::try_from(declared).ok() != Some(actual) {
        return Err(HubBindingError::InvalidResponse("space document count mismatch"));
    }
    Ok(())
}

pub(crate) fn map_client_error(error: DirectoryClientError) -> HubBindingError {
    match error {
        DirectoryClientError::Unauthorized | DirectoryClientError::Http { status: 403 | 404, .. } => HubBindingError::Unauthorized,
        DirectoryClientError::Cancelled | DirectoryClientError::Transport(semio_framework_os_kernel::os_directory::client::TransportError::Cancelled) => HubBindingError::Cancelled,
        DirectoryClientError::Transport(semio_framework_os_kernel::os_directory::client::TransportError::DeadlineExceeded) => HubBindingError::DeadlineExceeded,
        DirectoryClientError::Decode(_) => HubBindingError::Unavailable(HubUnavailableCause::UndecodableResponse),
        DirectoryClientError::Http { status, body } => HubBindingError::Unavailable(HubUnavailableCause::http(status, &body)),
        DirectoryClientError::Transport(error) => HubBindingError::Unavailable(HubUnavailableCause::transport(&error)),
    }
}

pub(crate) fn map_catalog_client_error(error: DirectoryClientError) -> HubBindingError {
    match error {
        DirectoryClientError::Unauthorized | DirectoryClientError::Http { status: 403, .. } => HubBindingError::Unauthorized,
        DirectoryClientError::Cancelled | DirectoryClientError::Transport(semio_framework_os_kernel::os_directory::client::TransportError::Cancelled) => HubBindingError::Cancelled,
        DirectoryClientError::Transport(semio_framework_os_kernel::os_directory::client::TransportError::DeadlineExceeded) => HubBindingError::DeadlineExceeded,
        DirectoryClientError::Decode(_) => HubBindingError::InvalidResponse("execution-target response failed validation"),
        DirectoryClientError::Http { status: 404 | 409, .. } => HubBindingError::StaleRefresh,
        DirectoryClientError::Http { status, body } => HubBindingError::Unavailable(HubUnavailableCause::http(status, &body)),
        DirectoryClientError::Transport(error) => HubBindingError::Unavailable(HubUnavailableCause::transport(&error)),
    }
}

pub(crate) fn bounded_diagnostic(message: &str) -> String {
    if message.len() <= HUB_BINDING_DIAGNOSTIC_MAX_BYTES {
        return message.to_string();
    }
    let mut end = HUB_BINDING_DIAGNOSTIC_MAX_BYTES;
    while !message.is_char_boundary(end) {
        end -= 1;
    }
    message[..end].to_string()
}

/// 🏷️ The wire label of one refresh phase.
fn phase_label(phase: HubBindingPhase) -> &'static str {
    match phase {
        HubBindingPhase::Idle => "idle",
        HubBindingPhase::Authenticating => "authenticating",
        HubBindingPhase::LoadingSpace => "loading-space",
        HubBindingPhase::ValidatingDocuments => "validating-documents",
        HubBindingPhase::AwaitingNetworkBudget => "awaiting-network-budget",
        HubBindingPhase::Ready => "ready",
        HubBindingPhase::Revoked => "revoked",
    }
}

/// 📈️ What a call parked on an authority refresh hears while it waits, in English and German.
pub(super) fn settle_progress_message(progress: &HubBindingProgress) -> String {
    const MIB: f64 = 1024.0 * 1024.0;
    match progress.phase {
        HubBindingPhase::AwaitingNetworkBudget => format!(
            "waiting for the network byte budget to refill ({:.1} of {:.1} MiB available) — warte auf das Auffüllen des Netzwerkkontingents ({:.1} von {:.1} MiB verfügbar)",
            progress.completed as f64 / MIB,
            progress.total as f64 / MIB,
            progress.completed as f64 / MIB,
            progress.total as f64 / MIB
        ),
        phase => format!(
            "waiting for the hub authority refresh ({} {}/{}) — warte auf die Aktualisierung der Hub-Berechtigungen ({} {}/{})",
            phase_label(phase),
            progress.completed,
            progress.total,
            phase_label(phase),
            progress.completed,
            progress.total
        ),
    }
}

/// 🔁️ Runs one canonical pair `attempt` until it stops failing for a reason a wait cures: an authority refresh in flight
/// (the attempt raced a directory event: waits for it to settle, `wait_ms` in all) or a spent network byte budget (waits
/// through `await_budget`). Every other outcome is the attempt's own. The first `artifact_open` right after a gateway
/// starts used to answer "canonical pair descriptor is unavailable" while the refresh a directory event started was still
/// running (ticket 26/09/23, session 14c).
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn settled_pair_attempt<T>(
    binding: &HubRemoteBinding,
    wait_ms: u64,
    mut await_budget: impl FnMut() -> Result<(), CanonicalPairMountError>,
    mut attempt: impl FnMut() -> Result<T, CanonicalPairMountError>,
) -> Result<T, CanonicalPairMountError> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_millis(wait_ms);
    loop {
        let authority = binding.authority_generation.load(Ordering::SeqCst);
        match attempt() {
            Err(CanonicalPairMountError::BudgetExhausted) => await_budget()?,
            Err(error @ (CanonicalPairMountError::DescriptorUnavailable | CanonicalPairMountError::StaleCompletion)) if binding.settling() || binding.authority_generation.load(Ordering::SeqCst) != authority => {
                let Some(remaining) = deadline.checked_duration_since(std::time::Instant::now()).filter(|remaining| !remaining.is_zero()) else { return Err(error) };
                binding.await_settled(u64::try_from(remaining.as_millis()).unwrap_or(u64::MAX));
            }
            outcome => return outcome,
        }
    }
}

pub(super) fn unavailable_gateway_error(state: HubRemoteBindingState, phase: HubBindingPhase, diagnostic: Option<String>) -> GatewayError {
    let label = match state {
        HubRemoteBindingState::Unbound => "unbound",
        HubRemoteBindingState::Refreshing => "refreshing",
        HubRemoteBindingState::Ready(_) => "expired",
        HubRemoteBindingState::Revoked => "revoked",
    };
    let phase = phase_label(phase);
    GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("authenticated hub descriptor index is {label}; retry after authority refresh"))
        .with_details(serde_json::json!({ "bindingState": label, "phase": phase, "lastFault": diagnostic }))
        .retryable()
}
