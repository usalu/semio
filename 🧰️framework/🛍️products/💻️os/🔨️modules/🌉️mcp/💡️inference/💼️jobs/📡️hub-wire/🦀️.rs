//! 📡️ Hub wire of the inference-hub crate: Inference job wire, hub inference client and bindings.

use semio_framework_async::OperationContext;
use semio_framework_os_kernel::os_directory::DocumentScope;
use std::sync::Arc;
use crate::service::{call_remote_inference_v1, remote_inference_protocol_v1};
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp_foundation::opaque_json::{INFERENCE_RESPONSE_MAX_BYTES, dsl_json, dsl_json_optional};

//#region 🔖️Wire

pub const INFERENCE_JOB_PAGE_SCHEMA: &str = "semio.mcp.inference-job/v1";

pub const INFERENCE_PROPOSAL_DIGEST_DOMAIN: &str = "semio.mcp.inference-proposal/v1";

pub const INFERENCE_JOB_EVENT_PAGE_MAX_ITEMS: usize = 64;

/// 🧭️ Where one declared inference service executes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InferenceExecutionSiteV1 {
    Guest,
    Hub,
}

/// 🌎️ One service a hub executes itself, exactly as its readiness publishes it.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubInferenceServiceV1 {
    pub service_id: String,
    pub route: String,
}

/// 🗓️ One lifecycle fact of one job.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceJobEventV1 {
    pub ordinal: u64,
    pub kind: String,
    pub at_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// 📈️ One progress row, as the fraction of the job's own work done.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceJobProgressV1 {
    pub cursor: u64,
    pub fraction: f64,
    pub at_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// 🧾️ What an approval would commit: the declared commit action, its exact input, and the digest an
/// approval must name. `preview` is the service's own result for a reader to inspect first.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceProposalV1 {
    pub hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview: Option<serde_json::Value>,
}

/// 📃️ The one page every quartet tool answers with, for either execution site.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceJobPageV1 {
    pub schema: String,
    pub job_id: String,
    pub service_id: String,
    pub artifact_kind: String,
    pub plugin_id: String,
    pub document_id: String,
    pub site: InferenceExecutionSiteV1,
    pub state: InferenceJobStateV1,
    pub proposal_state: InferenceProposalStateV1,
    pub cancel_requested: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proposal: Option<InferenceProposalV1>,
    pub events: Vec<InferenceJobEventV1>,
    pub progress: Vec<InferenceJobProgressV1>,
    pub next_cursor: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
}

//#endregion 🔖️Wire

//#region 💡️InferenceJobWire

/// 💡️ The exact closed wire vocabulary the installed service transport outcomes
/// publish (`🌎️hub/💡️inference/🏃️runtime/🦀️.rs`, `🌎️hub/🏗️bootstrap/🦀️.rs`'s own `//#region 💡️Inference`).
/// Mirrored here as typed Rust rather than reached for as free-form JSON: nothing on this
/// boundary is a `serde_json::Value`, and a hub field this client does not know about is a loud
/// decode failure rather than a silently-dropped one.
pub const INFERENCE_REQUEST_MAX_BYTES:usize=64*1024;

pub const INFERENCE_JOB_MAX_LIFETIME_MS:u64=120_000;

pub const INFERENCE_EVENT_PAGE_MAX_ITEMS:usize=64;

pub const INFERENCE_REQUEST_ID_HEX_LENGTH:usize=32;

pub const INFERENCE_INFLIGHT_CAPACITY: usize = 32;

/// 🚦️ The one stable failure vocabulary those routes publish, code and HTTP status verbatim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InferenceRouteErrorV1 {
    Unavailable,
    Denied,
    NotFound,
    Invalid,
    Bounds,
    Conflict,
    Capacity,
    Expired,
    Cancelled,
    CommitUnavailable,
    Storage,
}

pub const INFERENCE_ROUTE_ERRORS: [InferenceRouteErrorV1; 11] = [
    InferenceRouteErrorV1::Unavailable,
    InferenceRouteErrorV1::Denied,
    InferenceRouteErrorV1::NotFound,
    InferenceRouteErrorV1::Invalid,
    InferenceRouteErrorV1::Bounds,
    InferenceRouteErrorV1::Conflict,
    InferenceRouteErrorV1::Capacity,
    InferenceRouteErrorV1::Expired,
    InferenceRouteErrorV1::Cancelled,
    InferenceRouteErrorV1::CommitUnavailable,
    InferenceRouteErrorV1::Storage,
];

impl InferenceRouteErrorV1 {
    /// 🏷️ The exact wire code; a caller never learns which private hub object was missing.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Unavailable => "inference.unavailable",
            Self::Denied => "inference.denied",
            Self::NotFound => "inference.not-found",
            Self::Invalid => "inference.invalid",
            Self::Bounds => "inference.bounds",
            Self::Conflict => "inference.conflict",
            Self::Capacity => "inference.capacity",
            Self::Expired => "inference.expired",
            Self::Cancelled => "inference.cancelled",
            Self::CommitUnavailable => "approval.commit-unavailable",
            Self::Storage => "inference.storage",
        }
    }

    /// 🔢️ The exact HTTP status the hub publishes for this code.
    pub const fn status(self) -> u16 {
        match self {
            Self::Unavailable | Self::CommitUnavailable | Self::Storage => 503,
            Self::Denied => 403,
            Self::NotFound => 404,
            Self::Invalid => 400,
            Self::Bounds => 413,
            Self::Conflict | Self::Cancelled => 409,
            Self::Capacity => 429,
            Self::Expired => 410,
        }
    }

    /// 🔎️ Resolves a published code back to its variant; an unknown code is never guessed at.
    pub fn from_code(code: &str) -> Option<Self> {
        INFERENCE_ROUTE_ERRORS.into_iter().find(|candidate| candidate.code() == code)
    }

    /// 🔢️ The conservative fallback when a hub reply carries no decodable closed error body: the
    /// status alone is ambiguous for 503 and 409, so it resolves to the widest honest member.
    pub fn from_status(status: u16) -> Self {
        match status {
            403 => Self::Denied,
            404 => Self::NotFound,
            400 => Self::Invalid,
            413 => Self::Bounds,
            409 => Self::Conflict,
            429 => Self::Capacity,
            410 => Self::Expired,
            _ => Self::Unavailable,
        }
    }

    /// ♻️ Whether a caller may retry this exact call unchanged without changing anything else.
    pub const fn retryable(self) -> bool {
        matches!(self, Self::Unavailable | Self::CommitUnavailable | Self::Storage | Self::Capacity)
    }

    /// 🧭️ The gateway code this maps to, plus the one sentence that names the missing binding.
    pub fn gateway_code(self) -> GatewayErrorCode {
        match self {
            Self::Unavailable | Self::CommitUnavailable | Self::Storage | Self::Capacity => GatewayErrorCode::PluginUnavailable,
            Self::Denied => GatewayErrorCode::PermissionDenied,
            Self::NotFound => GatewayErrorCode::NotFound,
            Self::Invalid => GatewayErrorCode::InputInvalid,
            Self::Bounds => GatewayErrorCode::BudgetExceeded,
            Self::Conflict | Self::Expired => GatewayErrorCode::PreconditionFailed,
            Self::Cancelled => GatewayErrorCode::Cancelled,
        }
    }

    fn explanation(self) -> &'static str {
        match self {
            Self::Unavailable => "this hub executes no such inference service — bind a hub whose readiness lists it in `features.inferenceServices`",
            Self::CommitUnavailable => "the hub has no registered atomic parent+existing-child composition transaction, so an approval cannot be published; the prepared proposal is retained and nothing was applied",
            Self::Storage => "the hub's private inference ledger is temporarily unavailable",
            Self::Capacity => "the hub's fixed inference operation capacity is exhausted",
            Self::Denied => "the hub re-checked the live author, session, authorization generation and scope and refused; only the document's current `Author` may run, read, cancel or approve a job, and only its original owner",
            Self::NotFound => "the hub has no such job or document for this authenticated subject",
            Self::Invalid => "the hub rejected this closed client intent",
            Self::Bounds => "the request or reply exceeded the installed service’s declared bounds",
            Self::Conflict => "the frozen binding, document frontier, base pack or proposal hash drifted from the accepted job",
            Self::Expired => "this job outlived the hub's fixed job lifetime",
            Self::Cancelled => "this job carries a durable cancel request",
        }
    }

    /// ⚠️ The typed gateway error one tool call answers with; retryable members stay retryable.
    pub fn to_gateway_error(self, what: &str) -> GatewayError {
        let error = GatewayError::new(self.gateway_code(), format!("`{what}` was refused by the hub with `{}`: {}", self.code(), self.explanation()))
            .with_details(serde_json::json!({ "inferenceCode": self.code(), "httpStatus": self.status(), "retryable": self.retryable() }));
        if self.retryable() {
            error.retryable()
        } else {
            error
        }
    }
}

/// ⚠️ Why a protected inference request never reached a decodable hub reply at all.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InferenceHubTransportErrorV1 {
    Cancelled,
    DeadlineExceeded,
    Unauthorized,
    Unavailable,
    ResourceLimit,
    InvalidRequest(&'static str),
}

impl From<InferenceHubTransportErrorV1> for InferenceRouteErrorV1 {
    fn from(error: InferenceHubTransportErrorV1) -> Self {
        match error {
            InferenceHubTransportErrorV1::Cancelled => Self::Cancelled,
            InferenceHubTransportErrorV1::DeadlineExceeded | InferenceHubTransportErrorV1::Unavailable => Self::Unavailable,
            InferenceHubTransportErrorV1::Unauthorized => Self::Denied,
            InferenceHubTransportErrorV1::ResourceLimit => Self::Bounds,
            InferenceHubTransportErrorV1::InvalidRequest(_) => Self::Invalid,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InferenceJobStateV1 {
    Accepted,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum InferenceProposalStateV1 {
    None,
    Offered,
    Approved,
    Stale,
    Cancelled,
}

/// 📨 Neutral submission arguments projected by the installed owner codec.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct HubInferenceSubmitRequestV1 {pub request_id:String,pub service_id:String,pub lifetime_ms:u64}

impl HubInferenceSubmitRequestV1 {
    pub fn new(service_id:impl Into<String>,request_id:impl Into<String>,lifetime_ms:u64)->Self {Self {service_id:service_id.into(),request_id:request_id.into(),lifetime_ms}}
    pub fn validate(&self)->Result<(),InferenceRouteErrorV1> {if !is_lower_hex(&self.request_id,32) || !is_service_id(&self.service_id) || self.lifetime_ms==0 || self.lifetime_ms>INFERENCE_JOB_MAX_LIFETIME_MS {Err(InferenceRouteErrorV1::Invalid)}else{Ok(())}}
}

/// ✅ Neutral proposal admission arguments; the installed owner binds its receipt.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct HubInferenceApprovalRequestV1 {pub job_id:String,pub proposal_hash:String}

impl HubInferenceApprovalRequestV1 {
    pub fn new(job_id:impl Into<String>,proposal_hash:impl Into<String>)->Self {Self {job_id:job_id.into(),proposal_hash:proposal_hash.into()}}
    pub fn validate(&self)->Result<(),InferenceRouteErrorV1> {if !is_lower_hex(&self.job_id,32) || !is_lower_hex(&self.proposal_hash,64) {Err(InferenceRouteErrorV1::Invalid)}else{Ok(())}}
}

/// 📣 Owner-validated job admission projection.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct HubInferenceJobReceiptV1 {pub job_id:String,pub state:InferenceJobStateV1,pub proposal_state:InferenceProposalStateV1,pub proposal_hash:Option<String>,pub cursor:u64,pub expires_at_ms:u64}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct HubInferenceEventV1 {pub ordinal:u64,pub kind:String,pub at_ms:u64}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct HubInferenceProgressV1 {pub cursor:u64,pub run_epoch:u64,pub completed:u64,pub total:u64,pub at_ms:u64}

/// 📃 Owner-validated lifecycle projection with an opaque presentation payload.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct HubInferenceEventPageV1 {pub job_id:String,pub state:InferenceJobStateV1,pub proposal_state:InferenceProposalStateV1,pub cancel_requested:bool,pub stale:bool,pub proposal_hash:Option<String>,#[serde(default,with="dsl_json_optional")]pub preview:Option<semio_framework_value::DslValue>,pub events:Vec<HubInferenceEventV1>,pub progress:Vec<HubInferenceProgressV1>,pub next_cursor:u64}

/// 🧾 Generic remote commit witness and owner-authored history locator.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct HubInferenceApprovalReceiptV1 {pub job_id:String,pub mutation_id:String,pub command_hash:String,pub proposal_hash:String,pub applied:bool,#[serde(with="dsl_json")]pub undo:semio_framework_value::DslValue}

/// 🪪️ A declared inference service id: the `InferenceIdV1` shape the discovery schema pins.
fn is_service_id(value: &str) -> bool {
    !value.is_empty() && value.len() <= 256 && value.bytes().next().is_some_and(|byte| byte.is_ascii_alphanumeric()) && value.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-'))
}

fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

//#endregion 💡️InferenceJobWire

//#region 💡️InferenceHubClient

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InferenceHubMethodV1 {
    Get,
    Post,
}

/// 📨️ One protected inference request: an origin, an exact path, and a bounded closed body.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceHubRequestV1 {
    pub hub_origin: String,
    pub method: InferenceHubMethodV1,
    pub path: String,
    pub body: Vec<u8>,
    pub maximum_response_bytes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceHubResponseV1 {
    pub status: u16,
    pub body: Vec<u8>,
}

/// 🔌️ The injection seam for the four inference routes — the JSON twin of `CanonicalPairTransport`.
/// No concrete HTTP type ever appears above it, and the bearer never crosses it.
pub trait InferenceHubTransport: Send + Sync {
    async fn request(&self, context: &OperationContext, request: &InferenceHubRequestV1) -> Result<InferenceHubResponseV1, InferenceHubTransportErrorV1>;
}

#[cfg(not(target_arch = "wasm32"))]
pub struct NativeInferenceHubTransport<R: semio_framework_async::HostAsyncRuntime> {
    pub transport: semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport<R>,
    pub credential: Arc<semio_framework_os_kernel::os_directory::client::LocalHubCredential>,
}

#[cfg(not(target_arch = "wasm32"))]
impl<R: semio_framework_async::HostAsyncRuntime> NativeInferenceHubTransport<R> {
    pub fn new(transport: semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport<R>, credential: Arc<semio_framework_os_kernel::os_directory::client::LocalHubCredential>) -> Self {
        Self { transport, credential }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<R: semio_framework_async::HostAsyncRuntime + 'static> InferenceHubTransport for NativeInferenceHubTransport<R> {
    async fn request(&self, context: &OperationContext, request: &InferenceHubRequestV1) -> Result<InferenceHubResponseV1, InferenceHubTransportErrorV1> {
        use semio_framework_os_kernel::os_directory::client::{HttpMethod, TransportError};
        if self.credential.hub_origin().trim_end_matches('/') != request.hub_origin.trim_end_matches('/') {
            return Err(InferenceHubTransportErrorV1::InvalidRequest("inference request authority mismatch"));
        }
        if !request.path.starts_with('/') || request.path.starts_with("//") || request.path.contains('#') || request.body.len() > INFERENCE_REQUEST_MAX_BYTES {
            return Err(InferenceHubTransportErrorV1::InvalidRequest("inference request path or body is out of bounds"));
        }
        let method = match request.method {
            InferenceHubMethodV1::Get => HttpMethod::Get,
            InferenceHubMethodV1::Post => HttpMethod::Post,
        };
        let url = format!("{}{}", request.hub_origin.trim_end_matches('/'), request.path);
        let body = match request.method {
            InferenceHubMethodV1::Get => None,
            InferenceHubMethodV1::Post => Some(request.body.clone()),
        };
        let response = self.transport.request_protected_json(context, self.credential.as_ref(), method, &url, body).await.map_err(|error| match error {
            TransportError::Cancelled => InferenceHubTransportErrorV1::Cancelled,
            TransportError::DeadlineExceeded => InferenceHubTransportErrorV1::DeadlineExceeded,
            TransportError::Io(_) | TransportError::BudgetExhausted => InferenceHubTransportErrorV1::Unavailable,
        })?;
        if response.body.len() > request.maximum_response_bytes {
            return Err(InferenceHubTransportErrorV1::ResourceLimit);
        }
        Ok(InferenceHubResponseV1 { status: response.status, body: response.body })
    }
}

/// 🛡 Accepts bounded relative route labels; concrete paths belong to declared operations.
pub fn is_hub_inference_route(route:&str)->bool { !route.is_empty() && route.len()<=256 && route.split('/').all(|segment|!segment.is_empty() && segment.bytes().all(|byte|byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte==b'-')) }

/// 🧾️ The part of a hub's readiness body that declares the inference services it executes.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct HubReadinessInferenceV1 {
    pub features: HubReadinessInferenceFeaturesV1,
    #[serde(default)]
    startup: Option<serde::de::IgnoredAny>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct HubReadinessInferenceFeaturesV1 {
    pub inference_services: Vec<HubInferenceServiceV1>,
}

/// 🌎️ `GET /readyz` → `features.inferenceServices`. A body that declares none is an empty roster;
/// one whose routes are not relative lower-case route families is refused whole. A hub still booting
/// (`startup` present) has not loaded the catalog its roster comes from, so it is `Unavailable` — a
/// retryable answer — never an empty roster the caller would act on.
pub async fn read_hub_inference_services<T: InferenceHubTransport>(transport: &T, context: &OperationContext, hub_origin: &str) -> Result<Vec<HubInferenceServiceV1>, InferenceRouteErrorV1> {
    let wire = InferenceHubRequestV1 { hub_origin: hub_origin.to_string(), method: InferenceHubMethodV1::Get, path: "/readyz".to_string(), body: Vec::new(), maximum_response_bytes: INFERENCE_RESPONSE_MAX_BYTES };
    let response = transport.request(context, &wire).await?;
    if !(200..=299).contains(&response.status) && response.status != 503 {
        return Err(InferenceRouteErrorV1::from_status(response.status));
    }
    let readiness: HubReadinessInferenceV1 = serde_json::from_slice(&response.body).map_err(|_| InferenceRouteErrorV1::Invalid)?;
    if readiness.startup.is_some() {
        return Err(InferenceRouteErrorV1::Unavailable);
    }
    let services = readiness.features.inference_services;
    if services.iter().any(|service| !is_service_id(&service.service_id) || !is_hub_inference_route(&service.route)) {
        return Err(InferenceRouteErrorV1::Invalid);
    }
    Ok(services)
}

fn remote_payload(value:serde_json::Value)->Result<semio_framework_value::DslValue,InferenceRouteErrorV1> {
    semio_framework_pack_json::from_json_str(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_|InferenceRouteErrorV1::Invalid)
}

fn remote_projection<T:serde::de::DeserializeOwned>(value:semio_framework_value::DslValue)->Result<T,InferenceRouteErrorV1> {
    semio_framework_os_kernel::os_directory::client::document_http::CompiledDocumentHttpPortV1::validate_payload(&value,INFERENCE_RESPONSE_MAX_BYTES).map_err(|code|if code==semio_framework_os_kernel::os_directory::client::DocumentHttpPortCodeV1::Bounds {InferenceRouteErrorV1::Bounds}else{InferenceRouteErrorV1::Invalid})?;
    serde_json::from_str(&semio_framework_pack_json::to_json_string(&value)).map_err(|_|InferenceRouteErrorV1::Invalid)
}

/// 📡 Runs one neutral admission through its installed owner declaration and codecs.
pub async fn submit_hub_inference_job<T:InferenceHubTransport>(transport:&T,context:&OperationContext,origin:&str,scope:&DocumentScope,route:&str,request:&HubInferenceSubmitRequestV1)->Result<HubInferenceJobReceiptV1,InferenceRouteErrorV1> {
    request.validate()?;
    let protocol=remote_inference_protocol_v1(route)?;
    if protocol.service_id!=request.service_id {return Err(InferenceRouteErrorV1::Invalid);}
    let payload=remote_payload(serde_json::to_value(request).map_err(|_|InferenceRouteErrorV1::Invalid)?)?;
    remote_projection((protocol.receipt)(call_remote_inference_v1(transport,context,origin,scope,route,"submit",&payload).await?)?)
}

pub async fn read_hub_inference_job_events<T:InferenceHubTransport>(transport:&T,context:&OperationContext,origin:&str,scope:&DocumentScope,route:&str,job_id:&str,after:u64)->Result<HubInferenceEventPageV1,InferenceRouteErrorV1> {
    let protocol=remote_inference_protocol_v1(route)?;
    let payload=remote_payload(serde_json::json!({"jobId":job_id,"after":after}))?;
    remote_projection((protocol.page)(call_remote_inference_v1(transport,context,origin,scope,route,"events",&payload).await?,job_id)?)
}

pub async fn cancel_hub_inference_job<T:InferenceHubTransport>(transport:&T,context:&OperationContext,origin:&str,scope:&DocumentScope,route:&str,job_id:&str)->Result<HubInferenceEventPageV1,InferenceRouteErrorV1> {
    let protocol=remote_inference_protocol_v1(route)?;
    let payload=remote_payload(serde_json::json!({"jobId":job_id}))?;
    remote_projection((protocol.page)(call_remote_inference_v1(transport,context,origin,scope,route,"cancel",&payload).await?,job_id)?)
}

pub async fn approve_hub_inference_job<T:InferenceHubTransport>(transport:&T,context:&OperationContext,origin:&str,scope:&DocumentScope,route:&str,request:&HubInferenceApprovalRequestV1)->Result<HubInferenceApprovalReceiptV1,InferenceRouteErrorV1> {
    request.validate()?;
    let protocol=remote_inference_protocol_v1(route)?;
    let payload=remote_payload(serde_json::to_value(request).map_err(|_|InferenceRouteErrorV1::Invalid)?)?;
    remote_projection((protocol.approval)(call_remote_inference_v1(transport,context,origin,scope,route,"approve",&payload).await?,scope,&request.job_id,&request.proposal_hash)?)
}

pub async fn undo_hub_inference_approval<T:InferenceHubTransport>(transport:&T,context:&OperationContext,origin:&str,scope:&DocumentScope,route:&str,request:&semio_framework_value::DslValue)->Result<semio_framework_value::DslValue,InferenceRouteErrorV1> {
    let protocol=remote_inference_protocol_v1(route)?;
    (protocol.undo)(call_remote_inference_v1(transport,context,origin,scope,route,"undo",request).await?,request)
}

//#endregion 💡️InferenceHubClient

//#region 💡️InferenceJobBinding

/// 🧊️ The client-side frozen base one job was submitted against — the P4-C canonical pair mount's
/// own identity (`descriptor_digest_v1`, `active_checkpoint_id`, `etag`, `catalog_generation`) plus
/// its verified baseline `ArtifactFrontier`. It is a LOCAL staleness and display record: the hub
/// re-derives its own binding from server objects at admission and never trusts a client field.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubInferenceBaseBindingV1 {
    pub hub_origin: String,
    pub space_id: String,
    pub document_id: String,
    pub authority_generation: u64,
    pub descriptor_digest_v1: String,
    pub active_checkpoint_id: String,
    pub etag: String,
    pub catalog_generation: Option<u64>,
    pub head_edit_ordinal: u64,
    pub head_edit_id: String,
    pub last_commit_seq: u64,
    pub chain_hash: String,
}

/// 🎫️ The payload one `job_` handle carries, for either execution site. The handle is owned by the
/// connection's own `SessionHandle`, so a job minted by one MCP connection is unreadable by another;
/// a hub job additionally carries the subject that minted it, and the hub applies its own
/// owner-private check on top.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InferenceJobHandlePayloadV1 {
    pub site: InferenceExecutionSiteV1,
    pub service_id: String,
    pub artifact_kind: String,
    pub plugin_id: String,
    pub document_id: String,
    pub job_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hub: Option<HubInferenceJobBindingV1>,
}

/// 🌎️ What a hub-executed job's handle binds beyond the job itself: the published route family, the
/// minting subject and authority generation, the client idempotency key, and the frozen base.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubInferenceJobBindingV1 {
    pub route: String,
    pub space_id: String,
    pub subject_user_id: String,
    pub authority_generation: u64,
    pub request_id: String,
    pub base: Option<HubInferenceBaseBindingV1>,
}

/// 👤️ The live authenticated subject one hub-bound workspace is speaking as.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HubInferenceSubjectV1 {
    pub hub_origin: String,
    pub space_id: String,
    pub user_id: String,
    pub authority_generation: u64,
}

//#endregion 💡️InferenceJobBinding

//#region 💡️InferenceJobTools

/// 🛑️ Every in-flight inference call this process started, keyed by its bounded operation label, so
/// `inference_cancel` interrupts the local wait as well as recording the hub's durable cancel.
/// MCP has no progress notification and no out-of-band request cancellation wired to jobs
/// (`notifications/cancelled` is a recognized no-op for JSON-RPC REQUESTS), so this registry is the
/// only local half of cancellation; the durable half is always the hub's `/cancel` route.
static INFERENCE_INFLIGHT: std::sync::OnceLock<std::sync::Mutex<Vec<(String, semio_framework_async::CancelToken)>>> = std::sync::OnceLock::new();

pub fn inflight_registry() -> &'static std::sync::Mutex<Vec<(String, semio_framework_async::CancelToken)>> {
    INFERENCE_INFLIGHT.get_or_init(|| std::sync::Mutex::new(Vec::new()))
}

/// 🎛️ Retains one bounded cancellation token for the lifetime of one local inference call.
pub fn retain_inference_operation(label: &str, cancel: semio_framework_async::CancelToken) {
    let mut registry = inflight_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    registry.retain(|(existing, token)| existing.as_str() != label && !token.is_cancelled_now());
    if registry.len() >= INFERENCE_INFLIGHT_CAPACITY {
        registry.remove(0);
    }
    registry.push((label.to_string(), cancel));
}

/// 🧹️ Releases one retained token once its bounded local wait has terminated.
pub fn release_inference_operation(label: &str) {
    inflight_registry().lock().unwrap_or_else(std::sync::PoisonError::into_inner).retain(|(existing, _)| existing.as_str() != label);
}

/// 🏷️ The bounded operation label one local wait is retained under. `None` is the document-wide
/// label a submit or approve waits under before its job id is known — the hub serializes every
/// checked phase on one per-`DocumentScope` gate, so at most one such wait per document is live.
pub fn inference_operation_label(space_id: &str, document_id: &str, job_id: Option<&str>) -> String {
    match job_id {
        Some(job_id) => format!("inference:{space_id}/{document_id}/{job_id}"),
        None => format!("inference:{space_id}/{document_id}/*"),
    }
}

//#region 🔖️InferenceServiceResolution

pub fn hub_inference_binding_required(what: &str) -> GatewayError {
    GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("`{what}` needs an authenticated hub binding — start this gateway with `--hub <url> --space <id>`"))
        .with_details(serde_json::json!({ "bindWith": ["--hub", "--space"] }))
        .retryable()
}

//#endregion 🔖️InferenceServiceResolution

//#endregion 💡️InferenceJobTools
