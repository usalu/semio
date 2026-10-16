//! 💡️ The authenticated hub inference facade and history undo port of the headless workspace.

use semio_framework_os_kernel::os_directory::DocumentScope;
use std::sync::Arc;
use semio_framework_os_mcp_hub_remote::remote;
use crate::headless_workspace_state::HeadlessWorkspace;
use semio_framework_os_mcp_foundation::clock::now_ms;
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp_hub_remote::remote::HubRemoteBinding;
use semio_framework_os_mcp_inference_hub::hub_wire::{HubInferenceApprovalReceiptV1, HubInferenceApprovalRequestV1, HubInferenceBaseBindingV1, HubInferenceEventPageV1, HubInferenceJobReceiptV1, HubInferenceServiceV1, HubInferenceSubjectV1, HubInferenceSubmitRequestV1, hub_inference_binding_required, inference_operation_label, release_inference_operation, retain_inference_operation};
use semio_framework_os_mcp_inference_hub::service::remote_inference_protocol_v1;
use semio_framework_os_mcp_plugin_runtime::plugin_binding_and_activation::WorkspaceOrigin;
use semio_framework_os_mcp_port::channel_port::{HistoryUndoPort, HubInferenceApprovalUndoMemberV1};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_hub_remote::remote::NativeHubBindingDriver;

//#region 💡️Inference

/// 💡️ The authenticated facade to the inference services a bound hub executes itself. Every method
/// here is a thin, typed pass to the route family the hub publishes for a service: this process holds
/// no authority over a hub-executed job, and never applies anything to a document on its own. A
/// `--folder` workspace publishes no hub-executed service at all.
impl HeadlessWorkspace {
    pub(crate) fn hub_inference_binding(&self) -> Result<&Arc<HubRemoteBinding>, GatewayError> {
        if !matches!(self.origin, WorkspaceOrigin::Hub { .. }) {
            return Err(hub_inference_binding_required("this workspace"));
        }
        self.settled_hub_binding()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn hub_inference_driver(&self) -> Result<&NativeHubBindingDriver, GatewayError> {
        self.hub_driver.as_deref().ok_or_else(|| GatewayError::new(GatewayErrorCode::PluginUnavailable, "hub binding actor is not running").retryable())
    }

    /// 👤️ The live authenticated subject every inference job in this process is owned by.
    pub fn hub_inference_subject(&self) -> Result<HubInferenceSubjectV1, GatewayError> {
        self.hub_inference_binding()?.inference_subject(i64::try_from(now_ms()).unwrap_or(i64::MAX))
    }

    /// 🌎️ The inference services the bound hub executes itself, as its readiness publishes them
    /// (`features.inferenceServices`). A folder workspace executes every service in its guest.
    pub fn hub_inference_services(&self) -> Result<Vec<HubInferenceServiceV1>, GatewayError> {
        if !matches!(self.origin, WorkspaceOrigin::Hub { .. }) {
            return Ok(Vec::new());
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let binding = self.hub_inference_binding()?;
            let driver = self.hub_inference_driver()?;
            driver.read_hub_inference_services(binding.hub_origin(), &semio_framework_async::CancelToken::root_now()).map_err(|error| error.to_gateway_error("hub readiness"))
        }
        #[cfg(target_arch = "wasm32")]
        {
            Err(hub_inference_binding_required("hub_inference_services"))
        }
    }

    /// 📄️ Resolves one document id inside the bound space.
    pub(crate) fn admit_remote_inference_protocol(&self,route:&str)->Result<(),GatewayError> {
        let protocol=remote_inference_protocol_v1(route).map_err(|error|error.to_gateway_error("installed owner service"))?;
        let expected=(protocol.declaration)();
        let mut count=0;
        for descriptor in self.discovery_descriptors()? {
            if descriptor.manifest.plugin_id!=protocol.owner {continue;}
            for topic in descriptor.manifest.topic_contributions {
                if topic.topic!=semio_framework_os_kernel::os_directory::client::DOCUMENT_HTTP_PORT_TOPIC {continue;}
                if let Ok(declaration)=<semio_framework_os_kernel::os_directory::client::DocumentHttpPortDeclarationV1 as semio_framework_value::FromValue>::from_value(topic.payload) {
                    if declaration.service_id==protocol.service_id && declaration==expected {count+=1;}
                }
            }
        }
        if count!=1 {return Err(GatewayError::new(GatewayErrorCode::PluginUnavailable,"remote service owner declaration is absent or ambiguous"));}
        Ok(())
    }

    pub(crate) fn hub_inference_scope(&self, document_id: &str) -> Result<DocumentScope, GatewayError> {
        let (scope, _) = self.hub_inference_binding()?.inference_document(document_id, i64::try_from(now_ms()).unwrap_or(i64::MAX))?;
        Ok(scope)
    }

    /// 🧊️ The P4-C canonical pair mount projected to exactly the frozen base an inference job is
    /// compared against. It is a LOCAL record: the hub re-derives its own base from server objects.
    pub fn hub_inference_base(&self, document_id: &str) -> Result<HubInferenceBaseBindingV1, GatewayError> {
        let scope = self.hub_inference_scope(document_id)?;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let binding = self.hub_inference_binding()?;
            let driver = self.hub_inference_driver()?;
            let cancel = semio_framework_async::CancelToken::root_now();
            let label = inference_operation_label(&scope.space_id, &scope.document_id, None);
            retain_inference_operation(&label, cancel.clone());
            let mounted = driver.hub_inference_base(binding.as_ref(), &scope, &cancel);
            release_inference_operation(&label);
            mounted.map_err(remote::pair_mount_error_to_gateway)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = scope;
            Err(hub_inference_binding_required("hub_inference_base"))
        }
    }

    /// 📥️ Submits one closed client intent. The hub runs the bounded deterministic service inline,
    /// so this call blocks until an offer, a refusal or the bounded deadline — the local wait is
    /// retained under this document's operation label so `inference_cancel` can interrupt it.
    pub fn submit_hub_inference_job(&self, document_id: &str, route: &str, request: &HubInferenceSubmitRequestV1) -> Result<HubInferenceJobReceiptV1, GatewayError> {
        self.admit_remote_inference_protocol(route)?;
        let scope = self.hub_inference_scope(document_id)?;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let binding = self.hub_inference_binding()?;
            let driver = self.hub_inference_driver()?;
            let cancel = semio_framework_async::CancelToken::root_now();
            let label = inference_operation_label(&scope.space_id, &scope.document_id, None);
            retain_inference_operation(&label, cancel.clone());
            let submitted = driver.submit_hub_inference_job(&scope, binding.hub_origin(), route, request, &cancel);
            release_inference_operation(&label);
            submitted.map_err(|error| error.to_gateway_error("inference_submit"))
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (scope, route, request);
            Err(hub_inference_binding_required("submit_hub_inference_job"))
        }
    }

    /// 📤️ Reads the next owner-private bounded page of lifecycle events and progress rows.
    pub fn read_hub_inference_job_events(&self, document_id: &str, route: &str, job_id: &str, after: u64) -> Result<HubInferenceEventPageV1, GatewayError> {
        self.admit_remote_inference_protocol(route)?;
        let scope = self.hub_inference_scope(document_id)?;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let binding = self.hub_inference_binding()?;
            let driver = self.hub_inference_driver()?;
            let cancel = semio_framework_async::CancelToken::root_now();
            driver.read_hub_inference_job_events(&scope, binding.hub_origin(), route, job_id, after, &cancel).map_err(|error| error.to_gateway_error("inference_events"))
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (scope, route, job_id, after);
            Err(hub_inference_binding_required("read_hub_inference_job_events"))
        }
    }

    /// 🛑️ Records the owner's durable cancel request on the hub.
    pub fn cancel_hub_inference_job(&self, document_id: &str, route: &str, job_id: &str) -> Result<HubInferenceEventPageV1, GatewayError> {
        self.admit_remote_inference_protocol(route)?;
        let scope = self.hub_inference_scope(document_id)?;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let binding = self.hub_inference_binding()?;
            let driver = self.hub_inference_driver()?;
            let cancel = semio_framework_async::CancelToken::root_now();
            driver.cancel_hub_inference_job(&scope, binding.hub_origin(), route, job_id, &cancel).map_err(|error| error.to_gateway_error("inference_cancel"))
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (scope, route, job_id);
            Err(hub_inference_binding_required("cancel_hub_inference_job"))
        }
    }

    /// ✅️ Sends one explicit approval of an exact proposal hash; the hub rebuilds the typed effect.
    pub fn approve_hub_inference_job(&self, document_id: &str, route: &str, request: &HubInferenceApprovalRequestV1) -> Result<HubInferenceApprovalReceiptV1, GatewayError> {
        self.admit_remote_inference_protocol(route)?;
        let scope = self.hub_inference_scope(document_id)?;
        #[cfg(not(target_arch = "wasm32"))]
        {
            let binding = self.hub_inference_binding()?;
            let driver = self.hub_inference_driver()?;
            let cancel = semio_framework_async::CancelToken::root_now();
            let label = inference_operation_label(&scope.space_id, &scope.document_id, Some(&request.job_id));
            retain_inference_operation(&label, cancel.clone());
            let approved = driver.approve_hub_inference_job(&scope, binding.hub_origin(), route, request, &cancel);
            release_inference_operation(&label);
            approved.map_err(|error| error.to_gateway_error("inference_approve"))
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (scope, route, request);
            Err(hub_inference_binding_required("approve_hub_inference_job"))
        }
    }

    /// ↩️ Resolves one session-private history member through the normal authenticated Hub undo route.
    pub fn undo_hub_inference_approval(&self, member: &HubInferenceApprovalUndoMemberV1) -> Result<semio_framework_value::DslValue,GatewayError> {
        self.admit_remote_inference_protocol(&member.route)?;
        let scope = self.hub_inference_scope(&member.document_id)?;
        let binding = self.hub_inference_binding()?;
        if scope.space_id != member.space_id || binding.hub_origin() != member.hub_origin {
            return Err(GatewayError::new(GatewayErrorCode::PermissionDenied, "durable undo authority does not belong to this Hub workspace"));
        }
        let mut request=member.payload.clone();
        let semio_framework_value::DslValue::Object(fields)=&mut request else {return Err(GatewayError::new(GatewayErrorCode::InputInvalid,"invalid owner history payload"));};
        fields.retain(|(key,_)|key!="idempotencyKey");fields.push(("idempotencyKey".into(),semio_framework_value::DslValue::String(member.idempotency_key.clone())));
        #[cfg(not(target_arch = "wasm32"))]
        {
            let driver = self.hub_inference_driver()?;
            let cancel = semio_framework_async::CancelToken::root_now();
            let label = inference_operation_label(&scope.space_id, &scope.document_id, None);
            retain_inference_operation(&label, cancel.clone());
            let receipt = driver.undo_hub_inference_approval(&scope, binding.hub_origin(), &member.route, &request, &cancel);
            release_inference_operation(&label);
            receipt.map_err(|error| error.to_gateway_error("history_undo"))
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (scope, binding, request);
            Err(hub_inference_binding_required("history_undo"))
        }
    }
}

//#endregion 💡️Inference

impl HistoryUndoPort for HeadlessWorkspace {
    fn undo_hub_inference_approval(&self, member: &HubInferenceApprovalUndoMemberV1) -> Result<(), GatewayError> {
        self.undo_hub_inference_approval(member).map(|_| ())
    }
}
