//! 🧲️ The native hub binding driver and its authorized directory dial.

use semio_framework_async::{HostAsyncRuntime, OperationContext};
use semio_framework_os_kernel::os_directory::{DocumentExecutionTargetLeaseFieldsV1, DocumentOpenIntentV1, DocumentScope};
use semio_framework_os_kernel::os_directory::client::{DirectoryClient, DirectoryClientError, DirectoryTransport, HubSocketGrantSource, LocalHubCredential};
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp_hub_pair::pair;
use semio_framework_os_mcp_hub_pair::binding_errors::{HubBindingError, HubUnavailableCause};
use semio_framework_os_mcp_hub_pair::pair::{CanonicalPairMountError, CanonicalPairMountIdentity};
use semio_framework_os_mcp_wire::notify::publish_call_progress;
use std::sync::Arc;
use std::sync::atomic::Ordering;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_hub_pair::pair::{CanonicalPairMount, NativeCanonicalPairTransport};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_inference_hub::hub_wire::NativeInferenceHubTransport;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_wire::notify::active_request_cancel_requested;
use super::authorized_views::AuthorizedDescriptorSnapshot;
use super::binding_helpers::{map_client_error, settle_progress_message, validate_hub_origin};
use super::binding_limits::{
    HUB_AUTHORITY_SETTLE_WAIT_MS,
    HUB_BINDING_OPERATION_TIMEOUT_MS,
    HUB_COMPONENT_PROGRESS_KEEPALIVE_MS,
    HUB_EXECUTION_TARGET_LEASE_BUDGET_BYTES,
    HUB_REFRESH_NETWORK_BUDGET_BYTES,
    await_network_budget,
    hub_refresh_retry_ms,
    pause_unless_cancelled,
};
use super::component_assets::HubComponentTransferPhase;
use super::remote_binding::{HubBindingPhase, HubBindingProgress, HubRemoteBinding, HubStreamObservation};
#[cfg(not(target_arch = "wasm32"))]
use super::binding_helpers::settled_pair_attempt;
#[cfg(not(target_arch = "wasm32"))]
use super::component_assets::HubComponentAssets;

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeHubBindingDriver {
    pub cancel: semio_framework_async::CancelToken,
    pub(super) thread: Option<std::thread::JoinHandle<()>>,
    pub runtime: Arc<semio_framework_os_services::TokioHostRuntime>,
    pair_transport: Arc<NativeCanonicalPairTransport<semio_framework_os_services::TokioHostRuntime>>,
    pub(super) inference_transport: Arc<NativeInferenceHubTransport<semio_framework_os_services::TokioHostRuntime>>,
    /// 🪪️ The SAME authenticated client the binding refreshes through, held for the per-document execution-target lease.
    pub client: Arc<DirectoryClient<semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport<semio_framework_os_services::TokioHostRuntime>>>,
    /// 💰️ The same transport, read for its byte budget: a transfer the budget cannot admit waits for the refill turn.
    pub(super) budget: semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport<semio_framework_os_services::TokioHostRuntime>,
    /// 🧩️ The execution-target components, fetched on demand as static assets. The catalog refresh pulls manifest and
    /// descriptor for every selected document; a component is pulled only when an open or an action needs its plugin — it
    /// is tens of megabytes per package and a session that never dispatches an action must not pay for it.
    assets: HubComponentAssets,
}

/// 💰️ Runs one authority refresh step on the binding actor, parking it on the network byte budget whenever the budget ran
/// out: the binding reports `AwaitingNetworkBudget`, and the step runs again (with a fresh operation context of its own)
/// once the refill admits a refresh — a spent budget is never a failed refresh and never clears the authority for good.
#[cfg(not(target_arch = "wasm32"))]
fn budgeted_refresh_step<T>(
    binding: &HubRemoteBinding,
    budget: &semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport<semio_framework_os_services::TokioHostRuntime>,
    cancel: &semio_framework_async::CancelToken,
    mut step: impl FnMut() -> Result<T, HubBindingError>,
) -> Result<T, HubBindingError> {
    loop {
        match step() {
            Err(HubBindingError::Unavailable(HubUnavailableCause::NetworkBudget)) => {
                await_network_budget(|| budget.network_budget(), HUB_REFRESH_NETWORK_BUDGET_BYTES, cancel, |remaining, needed| binding.note_network_budget_wait(remaining, needed))?;
            }
            outcome => return outcome,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeHubBindingDriver {
    /// 💰️ The per-package byte budget must admit ONE authorized execution-target component plus
    /// the directory JSON around it. The previous flat 16 MiB was a directory-page budget: a real
    /// `gis` component is 47 MB, so every component fetch would have aborted mid-body with
    /// `ByteBudgetExhausted` — the budget is therefore derived from the Hub's own fixed component
    /// ceiling rather than from a number chosen when only JSON crossed this pool.
    pub fn connect(credential: Arc<LocalHubCredential>, base_url: &str, space_id: &str) -> Result<(Arc<HubRemoteBinding>, Self, Arc<dyn HubSocketGrantSource>), GatewayError> {
        use semio_framework_actor::{ActorId, PackageId};
        use semio_framework_async::{HostAsyncRuntime, ProcessKind, ScopeOwner, TraceId, WorkerPoolConfig};
        use semio_framework_os_kernel::os_directory::client::{native::NativeDirectoryTransport, DirectoryStreamTurn};
        use semio_framework_os_services::{ComputePool, TokioHostRuntime};

        validate_hub_origin(base_url, space_id)?;
        if base_url.trim_end_matches('/') != credential.hub_origin().trim_end_matches('/') {
            return Err(GatewayError::new(GatewayErrorCode::PermissionDenied, "--hub origin does not match the protected local credential"));
        }
        let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        let pool = semio_framework_async::process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores));
        let runtime = Arc::new(TokioHostRuntime::with_pool(pool.clone()));
        let scope = runtime.open_scope_now(ScopeOwner::Service("mcp-authenticated-hub-descriptor-index"), None);
        let compute = Arc::new(ComputePool::with_pool(2, pool));
        let transport = NativeDirectoryTransport::with_new_http_pool_now(
            runtime.clone(),
            scope,
            compute,
            semio_framework_os_kernel::os_directory::DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES + 16 * 1024 * 1024,
            2,
            PackageId("semio-framework-os-mcp".to_string()),
            ActorId(0x4d43_5001),
        );
        let assets = HubComponentAssets::new(runtime.clone(), credential.clone(), semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStore::for_user());
        let pair_transport = Arc::new(NativeCanonicalPairTransport::new(transport.clone(), credential.clone()));
        let inference_transport = Arc::new(NativeInferenceHubTransport::new(transport.clone(), credential.clone()));
        let budget = transport.clone();
        let client = Arc::new(DirectoryClient::authenticated(transport, credential));
        let grant_source: Arc<dyn HubSocketGrantSource> = client.clone();
        let binding = Arc::new(HubRemoteBinding::new(base_url, space_id).map_err(|error| GatewayError::new(GatewayErrorCode::InputInvalid, error.to_string()))?);
        let cancel = semio_framework_async::CancelToken::root_now();
        let operation_now = runtime.block_on(runtime.now_ms());
        let ctx = OperationContext {
            actor: 0x4d43_5001,
            generation: 0,
            trace: TraceId(operation_now),
            lane: 1,
            deadline_ms: Some(operation_now.saturating_add(HUB_BINDING_OPERATION_TIMEOUT_MS)),
            cancel: cancel.child_now(),
            capability: None,
        };
        let descriptor_snapshot = runtime
            .block_on(binding.refresh(client.as_ref(), &ctx, wall_now_ms(), operation_now))
            .map_err(binding_error_to_gateway)?;
        runtime
            .block_on(binding.refresh_catalog(client.as_ref(), &descriptor_snapshot, &ctx))
            .map_err(binding_error_to_gateway)?;

        let mut stream = client.stream(0);
        let DirectoryStreamTurn::Dial { client: dial_client, since } = stream.turn(&ctx, operation_now) else {
            return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "hub directory stream did not enter its initial dial").retryable());
        };
        let authority_generation = binding.authority_generation.load(Ordering::SeqCst);
        let connection = dial_client.open_stream_ws(&ctx, since, 1_000).map_err(|error| {
            let refusal = directory_dial_refusal(&error);
            binding.invalidate(&refusal.message);
            refusal
        })?;
        match complete_authorized_directory_dial(&mut stream, &binding, &ctx, authority_generation, operation_now, connection) {
            DirectoryStreamTurn::Idle => {}
            DirectoryStreamTurn::Closed if ctx.cancel.is_cancelled_now() => {
                return Err(GatewayError::new(GatewayErrorCode::Cancelled, "hub directory stream activation was cancelled"));
            }
            DirectoryStreamTurn::Closed => {
                return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable, "hub directory stream authority changed before activation").retryable());
            }
            DirectoryStreamTurn::Dial { .. } | DirectoryStreamTurn::DialScoped { .. } | DirectoryStreamTurn::Message(_) | DirectoryStreamTurn::ReconnectAt(_) | DirectoryStreamTurn::Revoked(_) => {
                return Err(GatewayError::new(GatewayErrorCode::Internal, "hub directory stream returned an invalid activation turn"));
            }
        }

        let thread_cancel = cancel.clone();
        let thread_binding = binding.clone();
        let thread_runtime = runtime.clone();
        let thread_client = client.clone();
        let thread_budget = budget.clone();
        let thread = std::thread::Builder::new()
            .name("semio-mcp-hub-binding".to_string())
            .spawn(move || {
                let fresh_context = || {
                    let operation_now = thread_runtime.block_on(thread_runtime.now_ms());
                    let ctx = OperationContext {
                        actor: 0x4d43_5001,
                        generation: 0,
                        trace: TraceId(operation_now),
                        lane: 1,
                        deadline_ms: Some(operation_now.saturating_add(HUB_BINDING_OPERATION_TIMEOUT_MS)),
                        cancel: thread_cancel.child_now(),
                        capability: None,
                    };
                    (ctx, operation_now)
                };
                let refresh_authority = || {
                    budgeted_refresh_step(&thread_binding, &thread_budget, &thread_cancel, || {
                        let (ctx, operation_now) = fresh_context();
                        thread_runtime.block_on(thread_binding.refresh(thread_client.as_ref(), &ctx, wall_now_ms(), operation_now))
                    })
                };
                let refresh_catalog = |snapshot: &Arc<AuthorizedDescriptorSnapshot>| {
                    budgeted_refresh_step(&thread_binding, &thread_budget, &thread_cancel, || {
                        let (ctx, _) = fresh_context();
                        thread_runtime.block_on(thread_binding.refresh_catalog(thread_client.as_ref(), snapshot, &ctx))
                    })
                };
                let mut needs_refresh = false;
                let mut refresh_failures: u32 = 0;
                while !thread_cancel.is_cancelled_now() {
                    let (ctx, operation_now) = fresh_context();
                    match stream.turn(&ctx, operation_now) {
                        DirectoryStreamTurn::Dial { client: dial_client, since } => {
                            if thread_binding.authority_generation.load(Ordering::SeqCst) == 0 {
                                match refresh_authority() {
                                    Err(HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired) => break,
                                    Err(_) => {
                                        match stream.complete_dial(
                                            operation_now,
                                            Err(semio_framework_os_kernel::os_directory::client::TransportError::Io("authenticated authority refresh failed before directory dial".to_string())),
                                        ) {
                                            DirectoryStreamTurn::Closed | DirectoryStreamTurn::Revoked(_) => break,
                                            DirectoryStreamTurn::ReconnectAt(_) | DirectoryStreamTurn::Idle => {}
                                            DirectoryStreamTurn::Dial { .. } | DirectoryStreamTurn::DialScoped { .. } | DirectoryStreamTurn::Message(_) => break,
                                        }
                                        continue;
                                    }
                                    Ok(snapshot) => {
                                        if let Err(error) = refresh_catalog(&snapshot) {
                                            thread_binding.invalidate(&format!("authenticated Hub catalog refresh failed before directory dial: {error}"));
                                            refresh_failures = refresh_failures.saturating_add(1);
                                            pause_unless_cancelled(&thread_cancel, hub_refresh_retry_ms(refresh_failures));
                                            continue;
                                        }
                                        refresh_failures = 0;
                                    }
                                }
                            }
                            let authority_generation = thread_binding.authority_generation.load(Ordering::SeqCst);
                            let (ctx, operation_now) = fresh_context();
                            match dial_client.open_stream_ws(&ctx, since, 1_000) {
                                Ok(connection) => {
                                    match complete_authorized_directory_dial(&mut stream, &thread_binding, &ctx, authority_generation, operation_now, connection) {
                                        DirectoryStreamTurn::Idle => {}
                                        DirectoryStreamTurn::Closed | DirectoryStreamTurn::Revoked(_) => break,
                                        DirectoryStreamTurn::Dial { .. } | DirectoryStreamTurn::DialScoped { .. } | DirectoryStreamTurn::Message(_) | DirectoryStreamTurn::ReconnectAt(_) => break,
                                    }
                                }
                                Err(error) => {
                                    thread_binding.note_stream_fault(&directory_dial_refusal(&error).message);
                                    match stream.complete_dial(operation_now, Err(semio_framework_os_kernel::os_directory::client::TransportError::Io(error.to_string()))) {
                                        DirectoryStreamTurn::Closed | DirectoryStreamTurn::Revoked(_) => break,
                                        DirectoryStreamTurn::ReconnectAt(_) | DirectoryStreamTurn::Idle => {}
                                        DirectoryStreamTurn::Dial { .. } | DirectoryStreamTurn::DialScoped { .. } | DirectoryStreamTurn::Message(_) => break,
                                    }
                                }
                            }
                        }
                        DirectoryStreamTurn::Message(message) => {
                            match thread_binding.observe_stream_message(&message) {
                                HubStreamObservation::Stable => {}
                                HubStreamObservation::RefreshRequired => needs_refresh = true,
                                HubStreamObservation::Revoked => break,
                            }
                        }
                        DirectoryStreamTurn::ReconnectAt(deadline) => {
                            thread_binding.invalidate_stream();
                            let wait = deadline.saturating_sub(operation_now).clamp(1, 25);
                            std::thread::sleep(std::time::Duration::from_millis(wait));
                        }
                        DirectoryStreamTurn::Idle if needs_refresh => {
                            match refresh_authority() {
                                Ok(snapshot) => match refresh_catalog(&snapshot) {
                                    Ok(_) => {
                                        needs_refresh = false;
                                        refresh_failures = 0;
                                    }
                                    Err(HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired) => break,
                                    Err(error) => {
                                        thread_binding.invalidate(&format!("authenticated Hub catalog refresh failed: {error}"));
                                        refresh_failures = refresh_failures.saturating_add(1);
                                        pause_unless_cancelled(&thread_cancel, hub_refresh_retry_ms(refresh_failures));
                                    }
                                },
                                Err(HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired) => break,
                                Err(_) => {
                                    refresh_failures = refresh_failures.saturating_add(1);
                                    pause_unless_cancelled(&thread_cancel, hub_refresh_retry_ms(refresh_failures));
                                }
                            }
                        }
                        DirectoryStreamTurn::Idle => std::thread::sleep(std::time::Duration::from_millis(10)),
                        DirectoryStreamTurn::Closed | DirectoryStreamTurn::Revoked(_) | DirectoryStreamTurn::DialScoped { .. } => break,
                    }
                }
                stream.close();
            })
            .map_err(|_| GatewayError::new(GatewayErrorCode::Internal, "could not start the hub descriptor binding actor"))?;
        Ok((binding, Self { cancel, thread: Some(thread), runtime, pair_transport, inference_transport, client, budget, assets }, grant_source))
    }

    /// 🧩️ The plugin COMPONENT the Hub authorized for `scope`, bit-for-bit the one its lease names — exact byte length and
    /// exact SHA-256, compared before the bytes reach any runtime, because a component the Hub serves is executable code.
    /// This is the leg of the browser shell's own attach chain a native client never had: `open-plan` →
    /// `execution-target/{manifest, component, descriptor}`; it comes from the per-user store when that holds the bytes, else
    /// from the hub as a static asset ([`HubComponentAssets`]), and the calling tool hears the transfer as progress.
    pub fn fetch_execution_target_component(&self, plugin_id: &str, scope: &DocumentScope, expected: &semio_framework_os_kernel::os_directory::DocumentExecutionTargetComponentV1) -> Result<Vec<u8>, GatewayError> {
        let intent = DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: None,
            client_instance_id: format!("mcp-component-{plugin_id}"),
        };
        let cancel = self.cancel.child_now();
        let mut published: Option<(HubComponentTransferPhase, u64, std::time::Instant)> = None;
        self.assets.component(&intent, expected, &cancel, |transfer| {
            if active_request_cancel_requested() {
                cancel.cancel_now();
            }
            let percent = transfer.received_bytes.saturating_mul(100) / transfer.total_bytes.max(1);
            let stale = published.is_none_or(|(phase, at, when)| phase != transfer.phase || at != percent || when.elapsed().as_millis() >= u128::from(HUB_COMPONENT_PROGRESS_KEEPALIVE_MS));
            if stale {
                published = Some((transfer.phase, percent, std::time::Instant::now()));
                publish_call_progress(transfer.fraction(), &transfer.message(plugin_id));
            }
        })
    }

    /// 🪪️ Fetches the execution-target LEASE the Hub minted for one exact document — the small JSON
    /// projection (≈3 KB, measured on hub 7621) that carries what the package-keyed catalog snapshot
    /// deliberately does not: this document's own `surface` (`surfaceId`/`appId`/`windowKindId`),
    /// its `artifact` kind and schema, and the grant behind them.
    ///
    /// 🧭️ `refresh_catalog` deduplicates its selections by PACKAGE identity, so its `scope` names
    /// whichever document first resolved that package — never necessarily the one a caller is
    /// opening. A per-document lease is therefore the only honest source for a per-document surface,
    /// and it is fetched here rather than derived: a surface id assembled host-side from a manifest
    /// would be this gateway's guess at what the hub authorized, which is exactly the class of
    /// fabrication `PROBE_SURFACE_ID` already was.
    pub fn fetch_execution_target_lease(&self, scope: &DocumentScope, client_instance_id: &str) -> Result<DocumentExecutionTargetLeaseFieldsV1, GatewayError> {
        let intent = DocumentOpenIntentV1 {
            schema: "semio.hub.document-open-intent/v1".into(),
            version: 1,
            scope: scope.clone(),
            requested_surface_id: None,
            client_instance_id: client_instance_id.to_string(),
        };
        let lease = loop {
            self.await_transfer_budget(HUB_EXECUTION_TARGET_LEASE_BUDGET_BYTES)?;
            let (ctx, _) = self.operation_context(&self.cancel, HUB_BINDING_OPERATION_TIMEOUT_MS);
            match self.runtime.block_on(self.client.document_execution_target_manifest(&ctx, &intent)) {
                Err(DirectoryClientError::Transport(semio_framework_os_kernel::os_directory::client::TransportError::BudgetExhausted)) => {}
                outcome => break outcome.map_err(|error| binding_error_to_gateway(map_client_error(error)))?,
            }
        };
        if lease.scope != *scope {
            return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, "hub execution-target lease names a different document scope than the one it was requested for"));
        }
        Ok(lease)
    }

    /// 💰️ Waits on the calling tool thread until this transport's byte budget admits `needed` bytes, telling the call what
    /// it waits for as progress; the call's own cancellation or the driver's ends the wait. A transfer the budget cannot
    /// admit now starts after the refill instead of failing half-way.
    fn await_transfer_budget(&self, needed: u64) -> Result<(), GatewayError> {
        let cancel = self.cancel.child_now();
        let started = std::time::Instant::now();
        await_network_budget(|| self.budget.network_budget(), needed, &cancel, |remaining, needed| {
            if active_request_cancel_requested() {
                cancel.cancel_now();
            }
            let progress = HubBindingProgress { phase: HubBindingPhase::AwaitingNetworkBudget, completed: usize::try_from(remaining).unwrap_or(usize::MAX), total: usize::try_from(needed).unwrap_or(usize::MAX) };
            let refill = std::time::Duration::from_millis(semio_framework_os_services::HTTP_BUCKET_REFILL_INTERVAL_MS).as_secs_f64();
            publish_call_progress((started.elapsed().as_secs_f64() / refill).min(0.99), &settle_progress_message(&progress));
        })
        .map_err(binding_error_to_gateway)
    }

    /// 🧊️ One scope's canonical pair, mounted once the authority refresh in flight settled and the byte budget admits it
    /// (see [`settled_pair_attempt`]).
    pub(super) fn mount_settled(&self, binding: &HubRemoteBinding, scope: &DocumentScope, cancel: &semio_framework_async::CancelToken) -> Result<CanonicalPairMount, CanonicalPairMountError> {
        settled_pair_attempt(
            binding,
            HUB_AUTHORITY_SETTLE_WAIT_MS,
            || self.await_transfer_budget(pair::HUB_PAIR_MAX_VERIFIED_BYTES as u64).map_err(|_| CanonicalPairMountError::Cancelled),
            || {
                let (context, operation_now) = self.operation_context(cancel, HUB_BINDING_OPERATION_TIMEOUT_MS);
                self.mount_canonical_pair(binding, scope, None, None, &context, wall_now_ms(), operation_now)
            },
        )
    }

    pub fn mount_canonical_pair(
        &self,
        binding: &HubRemoteBinding,
        scope: &DocumentScope,
        catalog_generation: Option<u64>,
        expected: Option<&CanonicalPairMountIdentity>,
        context: &OperationContext,
        wall_now_ms: i64,
        operation_now_ms: u64,
    ) -> Result<CanonicalPairMount, CanonicalPairMountError> {
        self.runtime.block_on(binding.mount_canonical_pair(self.pair_transport.as_ref(), scope, catalog_generation, expected, context, wall_now_ms, operation_now_ms))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for NativeHubBindingDriver {
    fn drop(&mut self) {
        self.cancel.cancel_now();
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// 🚪️ The refusal a hub binding answers when its directory stream cannot be dialled: retryable `PLUGIN_UNAVAILABLE` naming the
/// dial's own cause (a refused socket grant's status and body, a transport fault), so an agent and its operator see WHY the
/// authenticated snapshot was not activated instead of a bare "unavailable".
pub(crate) fn directory_dial_refusal(cause: &impl std::fmt::Display) -> GatewayError {
    GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("hub directory stream is unavailable ({cause}); authenticated snapshot was not activated")).retryable()
}

pub(crate) fn binding_error_to_gateway(error: HubBindingError) -> GatewayError {
    let code = match error {
        HubBindingError::Cancelled => GatewayErrorCode::Cancelled,
        HubBindingError::CapacityExceeded => GatewayErrorCode::BudgetExceeded,
        HubBindingError::InvalidResponse(_) => GatewayErrorCode::PreconditionFailed,
        HubBindingError::Unauthorized | HubBindingError::SessionExpired | HubBindingError::MembershipRequired => GatewayErrorCode::PermissionDenied,
        HubBindingError::DeadlineExceeded | HubBindingError::StaleRefresh | HubBindingError::Unavailable(_) => GatewayErrorCode::PluginUnavailable,
    };
    let gateway = match &error {
        HubBindingError::Unavailable(cause) => GatewayError::new(code, error.to_string()).with_details(serde_json::json!({ "cause": cause.to_json(), "summary": cause.summary() })),
        _ => GatewayError::new(code, error.to_string()),
    };
    if matches!(code, GatewayErrorCode::PluginUnavailable) { gateway.retryable() } else { gateway }
}

pub(super) fn wall_now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .and_then(|duration| i64::try_from(duration.as_millis()).ok())
        .unwrap_or(i64::MAX)
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn complete_authorized_directory_dial<T: DirectoryTransport + Clone>(
    stream: &mut semio_framework_os_kernel::os_directory::client::DirectoryStream<T>,
    binding: &HubRemoteBinding,
    ctx: &OperationContext,
    expected_authority_generation: u64,
    operation_now_ms: u64,
    connection: T::Ws,
) -> semio_framework_os_kernel::os_directory::client::DirectoryStreamTurn<T> {
    if ctx.cancel.is_cancelled_now()
        || expected_authority_generation == 0
        || binding.authority_generation.load(Ordering::SeqCst) != expected_authority_generation
    {
        stream.close();
    }
    stream.complete_dial(operation_now_ms, Ok(connection))
}
