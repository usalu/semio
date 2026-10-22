//! 💡️ Hub inference on the remote binding and the native driver.

use semio_framework_async::{HostAsyncRuntime, OperationContext};
use semio_framework_os_kernel::os_directory::DocumentScope;
use semio_framework_os_kernel::os_directory::io::binary::artifact_hash::hex_lower;
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp_hub_pair::pair::CanonicalPairMountError;
use semio_framework_os_mcp_inference_hub::hub_wire::HubInferenceSubjectV1;
use std::sync::atomic::Ordering;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_inference_hub::hub_wire::{
    HubInferenceApprovalReceiptV1,
    HubInferenceApprovalRequestV1,
    HubInferenceBaseBindingV1,
    HubInferenceEventPageV1,
    HubInferenceJobReceiptV1,
    HubInferenceServiceV1,
    HubInferenceSubmitRequestV1,
    InferenceRouteErrorV1,
    approve_hub_inference_job,
    cancel_hub_inference_job,
    read_hub_inference_job_events,
    read_hub_inference_services,
    submit_hub_inference_job,
    undo_hub_inference_approval,
};
use super::authorized_views::AuthorizedDocumentView;
use super::binding_helpers::canonical_checkpoint_resource_text;
use super::binding_limits::HUB_BINDING_OPERATION_TIMEOUT_MS;
use super::native_driver::wall_now_ms;
use super::remote_binding::{HubRemoteBinding, HubRemoteBindingState};
#[cfg(not(target_arch = "wasm32"))]
use super::native_driver::NativeHubBindingDriver;

//#region 💡️Inference

/// ⏱️ The bounded deadline every authenticated inference route call runs under. It is the hub's own
/// fixed job lifetime plus one binding-operation timeout of slack, because `POST …/jobs` runs the
/// bounded deterministic service inline before it answers.
pub const HUB_INFERENCE_OPERATION_TIMEOUT_MS: u64 = 120_000 + HUB_BINDING_OPERATION_TIMEOUT_MS;

impl HubRemoteBinding {
    /// 🌐️ The normalized origin every protected inference request is pinned to.
    pub fn hub_origin(&self) -> &str {
        &self.hub_origin
    }

    /// 🏷️ The one space this binding was constructed for; a caller never supplies another.
    pub fn space_id(&self) -> &str {
        &self.space_id
    }

    /// 👤️ The live authenticated subject and coarse authority fence one inference job is owned by.
    pub fn inference_subject(&self, wall_now_ms: i64) -> Result<HubInferenceSubjectV1, GatewayError> {
        let snapshot = self.ready_snapshot(wall_now_ms)?;
        let authority_generation = self.authority_generation.load(Ordering::SeqCst);
        if authority_generation == 0 {
            return Err(self.unavailable(HubRemoteBindingState::Refreshing));
        }
        Ok(HubInferenceSubjectV1 { hub_origin: self.hub_origin.clone(), space_id: self.space_id.clone(), user_id: snapshot.authenticated_user_id.clone(), authority_generation })
    }

    /// 📄️ Resolves one document id inside this binding's own space against the authenticated
    /// descriptor index — never a caller-supplied scope, never a document from another space.
    pub fn inference_document(&self, document_id: &str, wall_now_ms: i64) -> Result<(DocumentScope, AuthorizedDocumentView), GatewayError> {
        let snapshot = self.ready_snapshot(wall_now_ms)?;
        let scope = DocumentScope::new(self.space_id.clone(), document_id.to_string());
        let document = snapshot
            .documents
            .get(&scope)
            .ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("document `{document_id}` is not in this authenticated hub space")))?;
        Ok((scope, document.clone()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeHubBindingDriver {
    pub(super) fn operation_context(&self, cancel: &semio_framework_async::CancelToken, timeout_ms: u64) -> (OperationContext, u64) {
        use semio_framework_async::TraceId;
        let operation_now = self.runtime.block_on(self.runtime.now_ms());
        let context = OperationContext {
            actor: 0x4d43_5002,
            generation: 0,
            trace: TraceId(operation_now),
            lane: 1,
            deadline_ms: Some(operation_now.saturating_add(timeout_ms)),
            cancel: cancel.child_now(),
            capability: None,
        };
        (context, operation_now)
    }

    /// 🌎️ The inference services the hub executes itself, read from its own readiness body — the one
    /// place a hub declares them, so a client never keeps a table of its own.
    pub fn read_hub_inference_services(&self, hub_origin: &str, cancel: &semio_framework_async::CancelToken) -> Result<Vec<HubInferenceServiceV1>, InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_BINDING_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(read_hub_inference_services(self.inference_transport.as_ref(), &context, hub_origin))
    }

    /// 📥️ Submits one closed client intent through the protected inference transport and blocks the
    /// synchronous MCP tool call until the hub answers or `cancel`/the deadline interrupts it.
    pub fn submit_hub_inference_job(
        &self, scope: &DocumentScope, hub_origin: &str, route: &str, request: &HubInferenceSubmitRequestV1, cancel: &semio_framework_async::CancelToken,
    ) -> Result<HubInferenceJobReceiptV1, InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_INFERENCE_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(submit_hub_inference_job(self.inference_transport.as_ref(), &context, hub_origin, scope, route, request))
    }

    /// 📤️ Reads one owner-private bounded event page.
    pub fn read_hub_inference_job_events(
        &self, scope: &DocumentScope, hub_origin: &str, route: &str, job_id: &str, after: u64, cancel: &semio_framework_async::CancelToken,
    ) -> Result<HubInferenceEventPageV1, InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_BINDING_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(read_hub_inference_job_events(self.inference_transport.as_ref(), &context, hub_origin, scope, route, job_id, after))
    }

    /// 🛑️ Records the owner's durable cancel request.
    pub fn cancel_hub_inference_job(
        &self, scope: &DocumentScope, hub_origin: &str, route: &str, job_id: &str, cancel: &semio_framework_async::CancelToken,
    ) -> Result<HubInferenceEventPageV1, InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_BINDING_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(cancel_hub_inference_job(self.inference_transport.as_ref(), &context, hub_origin, scope, route, job_id))
    }

    /// ✅️ Sends one explicit approval of an exact proposal hash.
    pub fn approve_hub_inference_job(
        &self, scope: &DocumentScope, hub_origin: &str, route: &str, request: &HubInferenceApprovalRequestV1, cancel: &semio_framework_async::CancelToken,
    ) -> Result<HubInferenceApprovalReceiptV1, InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_INFERENCE_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(approve_hub_inference_job(self.inference_transport.as_ref(), &context, hub_origin, scope, route, request))
    }

    /// ↩️ Sends one Hub-minted durable GIS approval undo through the authenticated transport.
    pub fn undo_hub_inference_approval(
        &self,
        scope: &DocumentScope,
        hub_origin: &str,
        route: &str,
        request:&semio_framework_value::DslValue,
        cancel: &semio_framework_async::CancelToken,
    ) -> Result<semio_framework_value::DslValue,InferenceRouteErrorV1> {
        let (context, _) = self.operation_context(cancel, HUB_INFERENCE_OPERATION_TIMEOUT_MS);
        self.runtime.block_on(undo_hub_inference_approval(self.inference_transport.as_ref(), &context, hub_origin, scope, route, request))
    }

    /// 🧊️ Mounts the P4-C canonical checkpoint pair for one scope and projects exactly the frozen
    /// base identity an inference job is compared against: descriptor digest, active checkpoint,
    /// catalog generation, etag and the verified baseline frontier.
    pub fn hub_inference_base(&self, binding: &HubRemoteBinding, scope: &DocumentScope, cancel: &semio_framework_async::CancelToken) -> Result<HubInferenceBaseBindingV1, CanonicalPairMountError> {
        let mount = self.mount_settled(binding, scope, cancel)?;
        let identity = mount.identity();
        let baseline = mount.baseline();
        Ok(HubInferenceBaseBindingV1 {
            hub_origin: identity.hub_origin.clone(),
            space_id: identity.scope.space_id.clone(),
            document_id: identity.scope.document_id.clone(),
            authority_generation: identity.authority_generation,
            descriptor_digest_v1: identity.descriptor_digest_v1.clone(),
            active_checkpoint_id: identity.active_checkpoint_id.clone(),
            etag: identity.etag.clone(),
            catalog_generation: identity.catalog_generation,
            head_edit_ordinal: baseline.head_edit_ordinal,
            head_edit_id: baseline.head_edit_id.clone(),
            last_commit_seq: baseline.last_commit_seq,
            chain_hash: hex_lower(&baseline.chain_hash.0),
        })
    }

    /// 🧪 Reads one authenticated frozen checkpoint through the protected pair transport and
    /// projects it to the bounded MCP resource codec without exposing the retained cache bytes.
    pub fn read_canonical_checkpoint(
        &self,
        binding: &HubRemoteBinding,
        scope: &DocumentScope,
        cancel: &semio_framework_async::CancelToken,
    ) -> Result<String, CanonicalPairMountError> {
        let mount = self.mount_settled(binding, scope, cancel)?;
        if mount.identity().scope != *scope {
            return Err(CanonicalPairMountError::InvalidResponse("canonical checkpoint scope does not match its mount"));
        }
        binding.project_mounted_canonical_pair(&mount, wall_now_ms(), canonical_checkpoint_resource_text)
    }

    /// 📖️ One hub document's own canonical `pack`/`spr` bytes — the SAME verified mount
    /// [`Self::read_canonical_checkpoint`] takes, projected to the bytes instead of to the
    /// checkpoint's descriptive JSON.
    ///
    /// 🧊️ This is what `artifact_open`/`artifact_snapshot` of a hub document needs. Before ticket
    /// 26/09/18 slice M8 the workspace's hub arm answered a flat `canonical artifact bodies remain
    /// unavailable until P4-B`, so an agent bound to a real hub space could list the hub's documents
    /// and read their descriptors but never read one — measured live on hub 7631
    /// (`📓️m8-mcp-agent-third-participant.md` §2.3). Nothing new is fetched or trusted here: the
    /// mount is already digest-verified against the descriptor the authenticated binding published.
    pub fn read_canonical_pair_bytes(
        &self,
        binding: &HubRemoteBinding,
        scope: &DocumentScope,
        cancel: &semio_framework_async::CancelToken,
    ) -> Result<(Vec<u8>, Vec<u8>), CanonicalPairMountError> {
        let mount = self.mount_settled(binding, scope, cancel)?;
        if mount.identity().scope != *scope {
            return Err(CanonicalPairMountError::InvalidResponse("canonical pair scope does not match its mount"));
        }
        binding.project_mounted_canonical_pair(&mount, wall_now_ms(), |_identity, _baseline, pack, spr| Ok((pack.to_vec(), spr.to_vec())))
    }
}

/// ⚠️ The typed gateway error one canonical pair mount failure answers with — the same closed
/// mapping `binding_error_to_gateway` establishes for descriptor refresh failures.
pub fn pair_mount_error_to_gateway(error: CanonicalPairMountError) -> GatewayError {
    let code = match error {
        CanonicalPairMountError::Cancelled => GatewayErrorCode::Cancelled,
        CanonicalPairMountError::ResourceLimit => GatewayErrorCode::BudgetExceeded,
        CanonicalPairMountError::InvalidResponse(_) => GatewayErrorCode::PreconditionFailed,
        CanonicalPairMountError::Unauthorized => GatewayErrorCode::PermissionDenied,
        CanonicalPairMountError::DeadlineExceeded | CanonicalPairMountError::DescriptorUnavailable | CanonicalPairMountError::StaleCompletion | CanonicalPairMountError::Unavailable | CanonicalPairMountError::BudgetExhausted => GatewayErrorCode::PluginUnavailable,
    };
    let gateway = GatewayError::new(code, error.to_string());
    if matches!(code, GatewayErrorCode::PluginUnavailable) {
        gateway.retryable()
    } else {
        gateway
    }
}

//#endregion 💡️Inference
