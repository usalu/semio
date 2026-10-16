//! 📦️ Hub component assets: transfer phases, the stream fault ladder and the asset fetcher.

use semio_framework_async::{HostAsyncRuntime, OperationContext};
use semio_framework_os_kernel::os_directory::DocumentOpenIntentV1;
use semio_framework_os_kernel::os_directory::client::{DirectoryClientError, LocalHubCredential};
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use std::sync::{Arc, Mutex};
use super::binding_helpers::map_client_error;
use super::binding_limits::{
    HUB_COMPONENT_ASSET_BYTES_PER_MINUTE,
    HUB_COMPONENT_STREAMS,
    HUB_COMPONENT_STREAM_ALLOWANCE_MS,
    HUB_COMPONENT_STREAM_ATTEMPTS,
    HUB_COMPONENT_STREAM_FLOOR_BYTES_PER_SECOND,
    HUB_COMPONENT_STREAM_RETRY_MAX_MS,
    HUB_COMPONENT_STREAM_RETRY_MS,
    hub_refresh_retry_ms,
    pause_unless_cancelled,
};
use super::native_driver::binding_error_to_gateway;

//#region 🧩️ComponentAssets

/// 📶️ Where one execution-target component transfer is (schema `HubComponentTransferPhaseV1`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HubComponentTransferPhase {
    AwaitingStream,
    Receiving,
    Retrying,
}

impl HubComponentTransferPhase {
    pub fn label(self) -> &'static str {
        match self {
            Self::AwaitingStream => "awaiting-stream",
            Self::Receiving => "receiving",
            Self::Retrying => "retrying",
        }
    }
}

/// 📶️ One progress reading of a component transfer: its phase and how many of its bytes arrived.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HubComponentTransfer {
    pub phase: HubComponentTransferPhase,
    pub received_bytes: u64,
    pub total_bytes: u64,
}

impl HubComponentTransfer {
    pub fn fraction(&self) -> f64 {
        (self.received_bytes as f64 / self.total_bytes.max(1) as f64).clamp(0.0, 1.0)
    }

    /// 🗣️ What the call transferring `plugin_id`'s component hears, phase first, in English and German.
    pub fn message(&self, plugin_id: &str) -> String {
        const MIB: f64 = 1024.0 * 1024.0;
        let (received, total) = (self.received_bytes as f64 / MIB, self.total_bytes as f64 / MIB);
        let label = self.phase.label();
        match self.phase {
            HubComponentTransferPhase::AwaitingStream => format!("{label}: plugin component `{plugin_id}` ({total:.1} MiB) waits for a hub asset stream — Plugin-Komponente `{plugin_id}` ({total:.1} MiB) wartet auf einen Asset-Stream des Hubs"),
            HubComponentTransferPhase::Receiving => format!("{label}: plugin component `{plugin_id}` {received:.1} of {total:.1} MiB — Plugin-Komponente `{plugin_id}` {received:.1} von {total:.1} MiB"),
            HubComponentTransferPhase::Retrying => format!("{label}: plugin component `{plugin_id}` transfer interrupted, asking again — Übertragung der Plugin-Komponente `{plugin_id}` unterbrochen, neuer Versuch"),
        }
    }
}

/// 🚦️ Why one component stream attempt did not deliver the component.
#[cfg(not(target_arch = "wasm32"))]
enum ComponentStreamFault {
    Busy(u64),
    Transient(GatewayError),
    Final(GatewayError),
}

/// 🧩️ How a gateway gets the execution-target components its opens need, as static assets: from the per-user
/// execution-target store when it holds exactly the lease's bytes (no network at all — a restarted gateway opens every
/// kind it opened before with zero component bytes), else streamed from the hub over a pool of its own that the per-minute
/// API budget never meters, one stream at a time, verified against the lease and admitted to the store. The first open of a
/// plugin is therefore bound by bandwidth alone and reports its bytes as it goes.
#[cfg(not(target_arch = "wasm32"))]
pub struct HubComponentAssets {
    pub runtime: Arc<semio_framework_os_services::TokioHostRuntime>,
    pub(crate) transport: semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport<semio_framework_os_services::TokioHostRuntime>,
    pub credential: Arc<LocalHubCredential>,
    pub store: semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStore,
    serial: Mutex<()>,
}

#[cfg(not(target_arch = "wasm32"))]
impl HubComponentAssets {
    /// 🚰️ Component assets of `credential`'s hub kept in `store`, streamed over a fresh pool on `runtime`.
    pub fn new(runtime: Arc<semio_framework_os_services::TokioHostRuntime>, credential: Arc<LocalHubCredential>, store: semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStore) -> Self {
        let scope = runtime.open_scope_now(semio_framework_async::ScopeOwner::Service("mcp-execution-target-assets"), None);
        let compute = Arc::new(semio_framework_os_services::ComputePool::with_pool(HUB_COMPONENT_STREAMS, runtime.worker_pool().clone()));
        let transport = semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport::with_new_http_pool_now(
            runtime.clone(),
            scope,
            compute,
            HUB_COMPONENT_ASSET_BYTES_PER_MINUTE,
            HUB_COMPONENT_STREAMS,
            semio_framework_actor::PackageId("semio-framework-os-mcp.execution-target-assets".to_string()),
            semio_framework_actor::ActorId(0x4d43_5003),
        );
        Self { runtime, transport, credential, store, serial: Mutex::new(()) }
    }

    /// 🧩️ The component `expected` names for `intent`'s document: the store's verified copy, else the hub's stream. A busy
    /// hub (`429`) is a wait for the time it names, an interrupted stream or a hub shortage is asked again up to
    /// [`HUB_COMPONENT_STREAM_ATTEMPTS`] times, and bytes off the lease are refused and never kept. `report` hears every
    /// phase and every chunk; `cancel` ends the transfer between chunks.
    pub fn component(&self, intent: &DocumentOpenIntentV1, expected: &semio_framework_os_kernel::os_directory::DocumentExecutionTargetComponentV1, cancel: &semio_framework_async::CancelToken, mut report: impl FnMut(&HubComponentTransfer)) -> Result<Vec<u8>, GatewayError> {
        if let Some(bytes) = self.store.component(expected) {
            return Ok(bytes);
        }
        let reading = |phase| HubComponentTransfer { phase, received_bytes: 0, total_bytes: expected.byte_length };
        let _serial = loop {
            match self.serial.try_lock() {
                Ok(guard) => break guard,
                Err(std::sync::TryLockError::Poisoned(poisoned)) => break poisoned.into_inner(),
                Err(std::sync::TryLockError::WouldBlock) if cancel.is_cancelled_now() => return Err(component_cancelled()),
                Err(std::sync::TryLockError::WouldBlock) => {
                    report(&reading(HubComponentTransferPhase::AwaitingStream));
                    pause_unless_cancelled(cancel, 50);
                }
            }
        };
        if let Some(bytes) = self.store.component(expected) {
            return Ok(bytes);
        }
        let mut faults = 0u32;
        loop {
            match self.stream_once(intent, expected, cancel, &mut report) {
                Ok(bytes) => {
                    let _ = self.store.admit_component(&expected.sha256, &bytes);
                    return Ok(bytes);
                }
                Err(ComponentStreamFault::Final(error)) => return Err(error),
                Err(ComponentStreamFault::Busy(wait_ms)) => {
                    let until = std::time::Instant::now() + std::time::Duration::from_millis(wait_ms);
                    while std::time::Instant::now() < until && !cancel.is_cancelled_now() {
                        report(&reading(HubComponentTransferPhase::AwaitingStream));
                        pause_unless_cancelled(cancel, until.saturating_duration_since(std::time::Instant::now()).as_millis().min(250) as u64);
                    }
                }
                Err(ComponentStreamFault::Transient(error)) => {
                    faults += 1;
                    if faults >= HUB_COMPONENT_STREAM_ATTEMPTS {
                        return Err(error);
                    }
                    report(&reading(HubComponentTransferPhase::Retrying));
                    pause_unless_cancelled(cancel, hub_refresh_retry_ms(faults));
                }
            }
            if cancel.is_cancelled_now() {
                return Err(component_cancelled());
            }
        }
    }

    /// 🌊️ One component stream from the hub, read whole and verified against the lease.
    fn stream_once(&self, intent: &DocumentOpenIntentV1, expected: &semio_framework_os_kernel::os_directory::DocumentExecutionTargetComponentV1, cancel: &semio_framework_async::CancelToken, report: &mut impl FnMut(&HubComponentTransfer)) -> Result<Vec<u8>, ComponentStreamFault> {
        use semio_framework_os_kernel::os_directory::client::TransportError;
        let gateway = |error: DirectoryClientError| binding_error_to_gateway(map_client_error(error));
        let operation_now = self.runtime.block_on(self.runtime.now_ms());
        let stream_ms = HUB_COMPONENT_STREAM_ALLOWANCE_MS.saturating_add(expected.byte_length.saturating_mul(1_000) / HUB_COMPONENT_STREAM_FLOOR_BYTES_PER_SECOND);
        let ctx = OperationContext {
            actor: 0x4d43_5003,
            generation: 0,
            trace: semio_framework_async::TraceId(operation_now),
            lane: 1,
            deadline_ms: Some(operation_now.saturating_add(stream_ms)),
            cancel: cancel.child_now(),
            capability: None,
        };
        let (head, mut body) = match self.runtime.block_on(self.transport.execution_target_component_stream(&ctx, &self.credential, intent)) {
            Ok(started) => started,
            Err(TransportError::Cancelled) => return Err(ComponentStreamFault::Final(component_cancelled())),
            Err(error) => return Err(ComponentStreamFault::Transient(gateway(DirectoryClientError::Transport(error)))),
        };
        if head.status != 200 {
            let mut refusal = Vec::new();
            while refusal.len() < 4_096 {
                match self.runtime.block_on(body.next_chunk()) {
                    Ok(Some(chunk)) => refusal.extend_from_slice(&chunk),
                    Ok(None) | Err(_) => break,
                }
            }
            refusal.truncate(4_096);
            return Err(match head.status {
                429 => ComponentStreamFault::Busy(hub_asset_refusal_wait_ms(&refusal)),
                401 => ComponentStreamFault::Final(gateway(DirectoryClientError::Unauthorized)),
                502..=504 => ComponentStreamFault::Transient(gateway(DirectoryClientError::Http { status: head.status, body: String::from_utf8_lossy(&refusal).into_owned() })),
                status => ComponentStreamFault::Final(gateway(DirectoryClientError::Http { status, body: String::from_utf8_lossy(&refusal).into_owned() })),
            });
        }
        let declared = head.headers.iter().find(|(name, _)| name.eq_ignore_ascii_case("content-length")).and_then(|(_, value)| value.trim().parse::<u64>().ok());
        if declared.is_some_and(|declared| declared != expected.byte_length) {
            return Err(ComponentStreamFault::Final(GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("hub announced {} component bytes where its own lease declared {}", declared.unwrap_or_default(), expected.byte_length))));
        }
        let mut bytes = Vec::with_capacity(usize::try_from(expected.byte_length).unwrap_or(0));
        report(&HubComponentTransfer { phase: HubComponentTransferPhase::Receiving, received_bytes: 0, total_bytes: expected.byte_length });
        loop {
            if cancel.is_cancelled_now() {
                return Err(ComponentStreamFault::Final(component_cancelled()));
            }
            match self.runtime.block_on(body.next_chunk()) {
                Ok(Some(chunk)) => {
                    if (bytes.len() + chunk.len()) as u64 > expected.byte_length {
                        return Err(ComponentStreamFault::Final(GatewayError::new(GatewayErrorCode::PreconditionFailed, format!("hub served more than the {} component bytes its own lease declared", expected.byte_length))));
                    }
                    bytes.extend_from_slice(&chunk);
                    report(&HubComponentTransfer { phase: HubComponentTransferPhase::Receiving, received_bytes: bytes.len() as u64, total_bytes: expected.byte_length });
                }
                Ok(None) => break,
                Err(_) if cancel.is_cancelled_now() => return Err(ComponentStreamFault::Final(component_cancelled())),
                Err(error) => return Err(ComponentStreamFault::Transient(gateway(DirectoryClientError::Transport(TransportError::Io(format!("component stream interrupted after {} of {} bytes: {error}", bytes.len(), expected.byte_length)))))),
            }
        }
        if bytes.len() as u64 != expected.byte_length {
            return Err(ComponentStreamFault::Transient(gateway(DirectoryClientError::Transport(TransportError::Io(format!("component stream ended after {} of {} bytes", bytes.len(), expected.byte_length))))));
        }
        semio_framework_os_kernel::os_directory::client::verify_execution_target_bytes("component", &expected.sha256, expected.byte_length, &bytes)
            .map_err(|_| ComponentStreamFault::Final(GatewayError::new(GatewayErrorCode::PreconditionFailed, "hub execution-target component does not hash to the SHA-256 its own lease declared")))?;
        Ok(bytes)
    }
}

/// ⏳️ The wait a busy hub's typed asset refusal (`semio.hub.rate-limit-refusal/v1`) names, held to
/// [`HUB_COMPONENT_STREAM_RETRY_MAX_MS`]; [`HUB_COMPONENT_STREAM_RETRY_MS`] when the body names none.
#[cfg(not(target_arch = "wasm32"))]
fn hub_asset_refusal_wait_ms(body: &[u8]) -> u64 {
    serde_json::from_slice::<serde_json::Value>(body)
        .ok()
        .filter(|refusal| refusal["schema"] == "semio.hub.rate-limit-refusal/v1")
        .and_then(|refusal| refusal["retryAfterMs"].as_u64())
        .unwrap_or(HUB_COMPONENT_STREAM_RETRY_MS)
        .clamp(1, HUB_COMPONENT_STREAM_RETRY_MAX_MS)
}

#[cfg(not(target_arch = "wasm32"))]
fn component_cancelled() -> GatewayError {
    GatewayError::new(GatewayErrorCode::Cancelled, "the execution-target component transfer was cancelled")
}

//#endregion 🧩️ComponentAssets
