//! 💡️ Artifact inference services: metadata, registry, cancellation and the cold wire entry points.
#[cfg(test)] use crate::inference_gateway::wire_artifact_infer_from_cold;
#[cfg(test)] use crate::inference_gateway::wire_artifact_infer_cold;
#[cfg(test)] use crate::inference_gateway::cancel_artifact_inference;
use protocol::{OpBinary, OpText};
use semio_framework_value_derive::{FromValue, ToValue};
use std::collections::{BTreeMap, BTreeSet};
use pack::snapshot::ArtifactPack;
use semio_framework_value::{RetirementDemand, ValueError, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
use crate::artifact_identity::ArtifactExecutableIdentity;
use crate::artifact_identity::ArtifactIdentity;
use crate::child_view::CHILD_CONTENT_ID_BYTES;
use crate::surface_contract::artifact_pair_snapshot;
#[cfg(test)]
use crate::app_views::NoConfig;

/// 🪪️ Stable native identity and version contract for one executable artifact inference.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArtifactInferenceServiceMetadata {
    pub owner: &'static str,
    pub artifact_kind: &'static str,
    pub artifact_schema: &'static str,
    pub artifact_schema_version: u32,
    pub inference_schema: &'static str,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    /// 📜️ This inference's published payload contract, authored beside the inference itself and
    /// carried all the way into the committed descriptor. `None` is the honest answer for an
    /// inference nobody has written a contract for yet — the gateway then refuses the call by
    /// name rather than dispatching a body the guest cannot decode.
    pub payload: Option<ArtifactInferencePayloadContract>,
}

/// 📜️ The `&'static str` twin of `semio_framework::InferencePayloadContract` — authored in a
/// `const fn` beside the inference, so the contract a plugin publishes and the one its guest
/// enforces are one declaration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArtifactInferencePayloadContract {
    pub payload_schema_id: &'static str,
    pub input_schema: &'static str,
    pub output_schema: &'static str,
    pub progress_unit: &'static str,
    pub artifact_binding: Option<ArtifactInferenceDocumentBinding>,
    pub commit: Option<ArtifactInferenceCommitBinding>,
}

/// 📌️ The `&'static str` twin of `semio_framework::InferenceCommitBinding`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArtifactInferenceCommitBinding {
    pub action: &'static str,
}

/// 🔗️ The `&'static str` twin of `semio_framework::InferenceArtifactBinding`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArtifactInferenceDocumentBinding {
    pub field: &'static str,
    pub encoding: &'static str,
    pub required: bool,
}

impl From<ArtifactInferenceDocumentBinding> for semio_framework::InferenceArtifactBinding {
    fn from(binding: ArtifactInferenceDocumentBinding) -> Self {
        Self { field: binding.field.to_owned(), encoding: binding.encoding.to_owned(), required: binding.required }
    }
}

impl From<ArtifactInferencePayloadContract> for semio_framework::InferencePayloadContract {
    fn from(contract: ArtifactInferencePayloadContract) -> Self {
        Self {
            payload_schema_id: contract.payload_schema_id.to_owned(),
            input_schema: contract.input_schema.to_owned(),
            output_schema: contract.output_schema.to_owned(),
            progress_unit: contract.progress_unit.to_owned(),
            artifact_binding: contract.artifact_binding.map(Into::into),
            commit: contract.commit.map(|commit| semio_framework::InferenceCommitBinding { action: commit.action.to_owned() }),
        }
    }
}

/// ⚠️ Structured failure returned by a type-erased inference execution boundary.
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub struct ArtifactInferenceExecutionError {
    pub code: &'static str,
    pub message: String,
    pub retained_progress:RetainedCloneProgress,
    pub cause:Option<ValueError>,
    pub terminal:bool,
}

// 🚫️async: E1 pure error constructor consumed pervasively by sync closures
// (`.map_err`/`.ok_or_else`) — see R9.
impl ArtifactInferenceExecutionError {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self { code, message: message.into(),retained_progress:Default::default(),cause:None,terminal:true }
    }
}

impl ArtifactInferenceExecutionError {
    pub fn native(error:ValueError,terminal:bool)->Self{let retained_progress=error.retained_progress();Self{code:"artifact-inference.native",message:String::new(),retained_progress,cause:Some(error),terminal}}
}

impl std::fmt::Display for ArtifactInferenceExecutionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ArtifactInferenceExecutionError {}

/// 💡️ Complete, bounded native inference invocation.
pub struct ArtifactInferenceExecutionRequest<'a> {
    pub operation:u64,
    pub generation:u64,
    pub cancelled:bool,
    pub policy: &'a [u8],
    pub budgets: &'a WireArtifactInferenceBudget,
    pub retained: RetainedCloneGrant,
    pub cancellation_id: &'a str,
    pub previous_state: Option<&'a [u8]>,
    pub requested_cache_mode: WireArtifactInferenceCacheMode,
    pub canonical_payload: &'a [u8],
    pub dependencies: &'a [(String, Vec<u8>)],
}

/// 🪆️ The `dependencies` key under which a composed document's owned child at `slot`/`child_id` travels with an inference
/// request (design §20.15): its head snapshot pack.
pub fn inference_child_dependency(slot: &str, child_id: &str) -> String {
    format!("child:{slot}/{child_id}")
}

/// 🪆️ The slot and child id an owned-child dependency key names (`child:<slot>/<childId>`, each part non-empty, at most
/// [`CHILD_CONTENT_ID_BYTES`] bytes, without whitespace or control characters, the slot without `/`), else `None` — the grammar
/// [`validate_wire_request_resources`] admits beside artifact identities.
pub fn inference_child_dependency_parts(key: &str) -> Option<(&str, &str)> {
    let (slot, child_id) = key.strip_prefix("child:")?.split_once('/')?;
    let part = |text: &str| !text.is_empty() && text.len() <= CHILD_CONTENT_ID_BYTES && !text.chars().any(|character| character.is_whitespace() || character.is_control());
    (part(slot) && part(child_id)).then_some((slot, child_id))
}

/// 🧵️ The owned child at `slot`/`child_id` of the requesting composed document, decoded from the request's dependencies (design
/// §20.15: readers compose on read; the requester injects every owned child under [`inference_child_dependency`]).
pub fn inference_child<S: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot>(request: &ArtifactInferenceExecutionRequest<'_>, slot: &str, child_id: &str) -> Result<S, ArtifactInferenceExecutionError> {
    let key = inference_child_dependency(slot, child_id);
    let (_, pack) = request.dependencies.iter().find(|(name, _)| *name == key).ok_or_else(|| ArtifactInferenceExecutionError::new("artifact-inference.child-missing", format!("the request carries no owned child {slot}/{child_id}")))?;
    S::decode_pack(pack).map_err(|error| ArtifactInferenceExecutionError::new("artifact-inference.child-decode", format!("owned child {slot}/{child_id}: {error:?}")))
}

/// 💡️ Native inference facts that the wire boundary must preserve exactly.
#[derive(semio_framework_value::RetireOwned)]
pub struct ArtifactInferenceExecution {
    pub retirement_progress: RetainedCloneProgress,
    pub canonical_payload: Option<Vec<u8>>,
    pub diagnostics: Vec<WireArtifactInferenceDiagnostic>,
    pub validity: String,
    pub quality: String,
    pub complete: bool,
    pub actual_cache_mode: WireArtifactInferenceCacheMode,
}

/// 🧾️ A pending native callback retains its original source and still reports actual effects.
pub struct ArtifactInferenceExecutionStep {pub execution:Option<ArtifactInferenceExecution>,pub retained_progress:RetainedCloneProgress,pub terminal:bool}

impl ArtifactInferenceExecution {pub fn into_step(self,terminal:bool)->ArtifactInferenceExecutionStep {let retained_progress=self.retirement_progress;ArtifactInferenceExecutionStep{execution:Some(self),retained_progress,terminal}}}

/// 💡️ Non-capturing bounded inference implementation.
pub type ArtifactInferenceStepContext<'a>=semio_framework_job::StepContext<'a>;
pub type ArtifactInference = for<'a,'cx> fn(&ArtifactInferenceExecutionRequest<'a>,&mut ArtifactInferenceStepContext<'cx>) -> Result<ArtifactInferenceExecutionStep, ArtifactInferenceExecutionError>;

/// ♻️ Borrows the actual original child retained by the registered executable.
pub type ArtifactInferenceDemands = for<'a> fn(&ArtifactInferenceExecutionRequest<'a>,usize)->Result<RetirementDemand,ValueError>;

#[derive(Clone, Copy)]
pub(crate) enum ArtifactInferenceExecutable {
    Pure(ArtifactInference,ArtifactInferenceDemands),
    Contextual(for<'a,'cx> fn(&ArtifactInferenceExecutionRequest<'a>, &dyn std::any::Any,&mut ArtifactInferenceStepContext<'cx>) -> Result<ArtifactInferenceExecutionStep, ArtifactInferenceExecutionError>,for<'a> fn(&ArtifactInferenceExecutionRequest<'a>,&dyn std::any::Any,usize)->Result<RetirementDemand,ValueError>),
}

/// 🧰️ Native type-erased bounded inference entry.
#[derive(Clone, Copy)]
pub struct ArtifactInferenceService {
    metadata: ArtifactInferenceServiceMetadata,
    pub(crate) infer: ArtifactInferenceExecutable,
}

impl std::fmt::Debug for ArtifactInferenceService {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {formatter.debug_struct("ArtifactInferenceService").field("metadata", &self.metadata).finish_non_exhaustive()}
}

impl ArtifactInferenceService {
    pub const fn new(metadata:ArtifactInferenceServiceMetadata,infer:ArtifactInference,demands:ArtifactInferenceDemands)->Self{Self{metadata,infer:ArtifactInferenceExecutable::Pure(infer,demands)}}
    /// 🔗️ Binds both normal work and child closure to the same original resource context.
    pub const fn new_contextual(metadata:ArtifactInferenceServiceMetadata,infer:for<'a,'cx> fn(&ArtifactInferenceExecutionRequest<'a>,&dyn std::any::Any,&mut ArtifactInferenceStepContext<'cx>)->Result<ArtifactInferenceExecutionStep,ArtifactInferenceExecutionError>,demands:for<'a> fn(&ArtifactInferenceExecutionRequest<'a>,&dyn std::any::Any,usize)->Result<RetirementDemand,ValueError>)->Self{Self{metadata,infer:ArtifactInferenceExecutable::Contextual(infer,demands)}}
    pub const fn metadata(&self)->ArtifactInferenceServiceMetadata{self.metadata}
    /// 🧷️ Returns the capability identity of the actual request-aware executable.
    pub fn executable_identity(&self)->ArtifactExecutableIdentity{ArtifactExecutableIdentity::from_function_pointer(match self.infer{ArtifactInferenceExecutable::Pure(infer,_)=>infer as *const (),ArtifactInferenceExecutable::Contextual(infer,_)=>infer as *const ()})}
    pub fn infer(&self,request:&ArtifactInferenceExecutionRequest<'_>,cx:&mut ArtifactInferenceStepContext<'_>)->Result<ArtifactInferenceExecutionStep,ArtifactInferenceExecutionError>{match self.infer{ArtifactInferenceExecutable::Pure(infer,_)=>infer(request,cx),ArtifactInferenceExecutable::Contextual(_,_)=>Err(ArtifactInferenceExecutionError::new("artifact-inference.context-required","inference requires its owning instance resource context"))}}
    /// 🧩️ Executes with the resource owner already retained by the calling instance.
    pub fn infer_with_context(&self,request:&ArtifactInferenceExecutionRequest<'_>,context:&dyn std::any::Any,cx:&mut ArtifactInferenceStepContext<'_>)->Result<ArtifactInferenceExecutionStep,ArtifactInferenceExecutionError>{match self.infer{ArtifactInferenceExecutable::Pure(infer,_)=>infer(request,cx),ArtifactInferenceExecutable::Contextual(infer,_)=>infer(request,context,cx)}}
    /// ♻️ Quotes only the actual child belonging to this original request and context.
    pub fn retirement_demands(&self,request:&ArtifactInferenceExecutionRequest<'_>,context:Option<&dyn std::any::Any>,copy:usize)->Result<RetirementDemand,ValueError>{match self.infer{ArtifactInferenceExecutable::Pure(_,demands)=>demands(request,copy),ArtifactInferenceExecutable::Contextual(_,demands)=>demands(request,context.ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original inference child requires its owning context"))?,copy)}}
    fn identical_to(&self,other:&Self)->bool{self.metadata==other.metadata&&match(self.infer,other.infer){(ArtifactInferenceExecutable::Pure(a,ad),ArtifactInferenceExecutable::Pure(b,bd))=>std::ptr::fn_addr_eq(a,b)&&std::ptr::fn_addr_eq(ad,bd),(ArtifactInferenceExecutable::Contextual(a,ad),ArtifactInferenceExecutable::Contextual(b,bd))=>std::ptr::fn_addr_eq(a,b)&&std::ptr::fn_addr_eq(ad,bd),_=>false}}
}

/// 🔑️ Lookup key intentionally excludes owner and versions: there can be only one owner
/// and one versioned implementation for an artifact-kind/inference-schema pair in a process.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArtifactInferenceServiceKey {
    pub artifact_kind: &'static str,
    pub inference_schema: &'static str,
}

impl From<ArtifactInferenceServiceMetadata> for ArtifactInferenceServiceKey {
    fn from(metadata: ArtifactInferenceServiceMetadata) -> Self {
        Self { artifact_kind: metadata.artifact_kind, inference_schema: metadata.inference_schema }
    }
}

/// ⚔️ Complete diagnostic for conflicting native inference registrations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactInferenceRegistrationConflict {
    pub key: ArtifactInferenceServiceKey,
    pub existing: ArtifactInferenceServiceMetadata,
    pub incoming: ArtifactInferenceServiceMetadata,
}

/// 🚨️ Typed failure from the native inference-service registry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactInferenceRegistrationError {
    Conflict(Box<ArtifactInferenceRegistrationConflict>),
    Unavailable,
}

impl std::fmt::Display for ArtifactInferenceRegistrationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conflict(conflict) => write!(formatter, "conflicting artifact inference registration for {}/{}: existing {:?}, incoming {:?}", conflict.key.artifact_kind, conflict.key.inference_schema, conflict.existing, conflict.incoming),
            Self::Unavailable => formatter.write_str("artifact inference registry is poisoned"),
        }
    }
}

impl std::error::Error for ArtifactInferenceRegistrationError {}

/// 📚️ Deterministic process-local catalogue for executable native inferences.
#[derive(Default)]
pub struct ArtifactInferenceServiceRegistry {
    by_key: BTreeMap<ArtifactInferenceServiceKey, ArtifactInferenceService>,
}

impl ArtifactInferenceServiceRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, service: ArtifactInferenceService) -> Result<(), ArtifactInferenceRegistrationError> {
        let key = ArtifactInferenceServiceKey::from(service.metadata);
        if let Some(existing) = self.by_key.get(&key) {
            if existing.identical_to(&service) {
                return Ok(());
            }
            return Err(ArtifactInferenceRegistrationError::Conflict(Box::new(ArtifactInferenceRegistrationConflict { key, existing: existing.metadata, incoming: service.metadata })));
        }
        self.by_key.insert(key, service);
        Ok(())
    }

    pub fn get(&self, artifact_kind: &str, inference_schema: &str) -> Option<&ArtifactInferenceService> {
        self.by_key.iter().find_map(|(key, service)| (key.artifact_kind == artifact_kind && key.inference_schema == inference_schema).then_some(service))
    }

    pub fn metadata(&self) -> Vec<ArtifactInferenceServiceMetadata> {
        self.by_key.values().map(ArtifactInferenceService::metadata).collect()
    }

    pub fn infer(&self, artifact_kind: &str, inference_schema: &str, request: &ArtifactInferenceExecutionRequest<'_>,cx:&mut ArtifactInferenceStepContext<'_>) -> Result<ArtifactInferenceExecutionStep, ArtifactInferenceExecutionError> {
        let service = self.get(artifact_kind, inference_schema).ok_or_else(|| ArtifactInferenceExecutionError::new("artifact-inference.not-registered", format!("no native inference service for {artifact_kind}/{inference_schema}")))?;
        service.infer(request,cx)
    }
}

static ARTIFACT_INFERENCE_SERVICE_REGISTRY: std::sync::OnceLock<std::sync::RwLock<ArtifactInferenceServiceRegistry>> = std::sync::OnceLock::new();

pub(crate) fn artifact_inference_service_registry() -> &'static std::sync::RwLock<ArtifactInferenceServiceRegistry> {
    ARTIFACT_INFERENCE_SERVICE_REGISTRY.get_or_init(|| std::sync::RwLock::new(ArtifactInferenceServiceRegistry::new()))
}

pub fn register_artifact_inference_service(service: ArtifactInferenceService) -> Result<(), ArtifactInferenceRegistrationError> {
    artifact_inference_service_registry().write().map_err(|_| ArtifactInferenceRegistrationError::Unavailable)?.register(service)
}

/// 🔬️ Verifies native inference services against the established registry without mutation.
pub fn preflight_artifact_inference_services(services: &[ArtifactInferenceService]) -> Result<(), ArtifactInferenceRegistrationError> {
    let mut proposed = ArtifactInferenceServiceRegistry::new();
    for service in services {
        proposed.register(*service)?;
    }
    let registry = artifact_inference_service_registry().read().map_err(|_| ArtifactInferenceRegistrationError::Unavailable)?;
    for service in services {
        let key = ArtifactInferenceServiceKey::from(service.metadata());
        if let Some(existing) = registry.by_key.get(&key) {
            if !existing.identical_to(service) {
                return Err(ArtifactInferenceRegistrationError::Conflict(Box::new(ArtifactInferenceRegistrationConflict { key, existing: existing.metadata(), incoming: service.metadata() })));
            }
        }
    }
    Ok(())
}

/// 📌️ Registers native inference services only after the whole candidate set is conflict-free.
pub fn register_artifact_inference_services(services: Vec<ArtifactInferenceService>) -> Result<(), ArtifactInferenceRegistrationError> {
    preflight_artifact_inference_services(&services)?;
    let mut registry = artifact_inference_service_registry().write().map_err(|_| ArtifactInferenceRegistrationError::Unavailable)?;
    for service in services {
        registry.register(service)?;
    }
    Ok(())
}

pub fn artifact_inference_service(artifact_kind: &str, inference_schema: &str) -> Result<Option<ArtifactInferenceService>, ValueError> {
    Ok(artifact_inference_service_registry().read().map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"artifact inference registry is poisoned"))?.get(artifact_kind, inference_schema).copied())
}

pub fn list_artifact_inference_services() -> Result<Vec<ArtifactInferenceServiceMetadata>, ArtifactInferenceExecutionError> {
    Ok(artifact_inference_service_registry().read().map_err(|_| ArtifactInferenceExecutionError::new("artifact-inference.unavailable", "artifact inference registry is poisoned"))?.metadata())
}

/// 💡️ Runs one bounded native inference through its immutable request contract.
pub async fn infer_artifact(artifact_kind: &str, inference_schema: &str, request: &ArtifactInferenceExecutionRequest<'_>,cx:&mut ArtifactInferenceStepContext<'_>) -> Result<ArtifactInferenceExecutionStep, ArtifactInferenceExecutionError> {
    let service =
        artifact_inference_service(artifact_kind, inference_schema).map_err(|error|ArtifactInferenceExecutionError::native(error,true))?.ok_or_else(|| ArtifactInferenceExecutionError::new("artifact-inference.not-registered", format!("no native inference service for {artifact_kind}/{inference_schema}")))?;
    service.infer(request,cx)
}

#[cfg(test)]
include!("../🧪️tests/🔬️app-artifact-inference-service/🦀️.rs");

pub const ARTIFACT_INFERENCE_WIRE_VERSION: u32 = 2;

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireArtifactInferenceMetadata {
    pub owner: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub artifact_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    /// 📜️ The published payload contract, carried from the guest's own declaration to
    /// `describe`, which commits it into `contributions.inferenceServices[].payload`. Not part
    /// of this row's IDENTITY — [`validate_wire_request_metadata`] compares the identity
    /// projection, because a REQUEST names an inference and never re-states its contract.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<WireInferencePayloadContract>,
}

/// 📜️ Wire twin of [`ArtifactInferencePayloadContract`] — the shape that crosses the guest
/// boundary on the inference roster.
#[derive(Clone, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireInferencePayloadContract {
    pub payload_schema_id: String,
    pub input_schema: String,
    pub output_schema: String,
    #[value(default, skip_serializing_if = "String::is_empty")]
    pub progress_unit: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub artifact_binding: Option<WireInferenceArtifactBinding>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<WireInferenceCommitBinding>,
}

/// 📌️ Wire twin of [`ArtifactInferenceCommitBinding`].
#[derive(Clone, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireInferenceCommitBinding {
    pub action: String,
}

/// 🔗️ Wire twin of [`ArtifactInferenceDocumentBinding`].
#[derive(Clone, Debug, Default, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireInferenceArtifactBinding {
    pub field: String,
    pub encoding: String,
    pub required: bool,
}

/// 📦️ The artifact document an artifact-bound inference is run against: the artifact's own
/// canonical `pack`/`spr` pair, each standard base64, under the payload field the inference's
/// published contract names (`InferenceArtifactBinding.field`, encoding
/// `semio_framework::INFERENCE_ARTIFACT_PACK_BASE64`). The host fills it from the artifact the
/// caller named; the guest decodes it back into its OWN snapshot type — neither side ever
/// interprets the other's document shape.
#[derive(Clone, Debug, Default, PartialEq, Eq, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArtifactDocumentPayload {
    pub pack: String,
    pub spr: String,
}

impl ArtifactDocumentPayload {
    /// 📦️ Builds the payload from raw pair bytes — the host half of the binding.
    pub fn from_pair(pack: &[u8], spr: &[u8]) -> Self {
        Self { pack: base64_codec::base64_standard_encode(pack), spr: base64_codec::base64_standard_encode(spr) }
    }

    /// 🔡️ The pair bytes back, or a named decode failure — never a partial decode.
    pub fn pair(&self) -> Result<(Vec<u8>, Vec<u8>), String> {
        let pack = base64_codec::base64_standard_decode(&self.pack).map_err(|error| format!("artifact-document-pack-base64:{error}"))?;
        let spr = base64_codec::base64_standard_decode(&self.spr).map_err(|error| format!("artifact-document-spr-base64:{error}"))?;
        Ok((pack, spr))
    }

    /// 🔭️ The document's LIVE snapshot in the guest's own type — one call for every plugin that
    /// binds an inference to an artifact, so the five `🀄️wfc` kinds share one decode path.
    /// 🚫️async: E2 — `artifact_pair_snapshot` is the only awaited callee and it does not suspend;
    /// an inference factory is a synchronous contract, so the settle happens at the call site.
    pub async fn snapshot<P, Mutation>(&self) -> Result<P, String>
    where
        P: Clone + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static,
        Mutation: OpText + OpBinary + store::Mutation<P> + 'static,
    {
        let (pack, spr) = self.pair()?;
        artifact_pair_snapshot::<P, Mutation>(&pack, &spr).await.map_err(|error| format!("artifact-document-decode:{error}"))
    }
}

impl ArtifactDocumentPayload {
    /// 🔭️ [`Self::snapshot`] settled in place. A `ToolJobFactory::create_job_from_wire` is a
    /// SYNCHRONOUS contract — an inference factory has no executor to park on — and the whole
    /// decode chain below this call is `io-async-signatures`-async with no suspension point, so
    /// the settle is total. One call, so every artifact-bound inference in the tree shares one
    /// decode path instead of re-deriving it per kind.
    pub fn settled_snapshot<P, Mutation>(&self) -> Result<P, String>
    where
        P: Clone + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + 'static,
        Mutation: OpText + OpBinary + store::Mutation<P> + 'static,
    {
        ::semio_framework_async::poll::resolve_ready(self.snapshot::<P, Mutation>())
    }
}

impl From<ArtifactInferencePayloadContract> for WireInferencePayloadContract {
    fn from(contract: ArtifactInferencePayloadContract) -> Self {
        Self {
            payload_schema_id: contract.payload_schema_id.to_owned(),
            input_schema: contract.input_schema.to_owned(),
            output_schema: contract.output_schema.to_owned(),
            progress_unit: contract.progress_unit.to_owned(),
            artifact_binding: contract.artifact_binding.map(|binding| WireInferenceArtifactBinding { field: binding.field.to_owned(), encoding: binding.encoding.to_owned(), required: binding.required }),
            commit: contract.commit.map(|commit| WireInferenceCommitBinding { action: commit.action.to_owned() }),
        }
    }
}

impl From<WireInferencePayloadContract> for semio_framework::InferencePayloadContract {
    fn from(contract: WireInferencePayloadContract) -> Self {
        Self {
            payload_schema_id: contract.payload_schema_id,
            input_schema: contract.input_schema,
            output_schema: contract.output_schema,
            progress_unit: contract.progress_unit,
            artifact_binding: contract.artifact_binding.map(|binding| semio_framework::InferenceArtifactBinding { field: binding.field, encoding: binding.encoding, required: binding.required }),
            commit: contract.commit.map(|commit| semio_framework::InferenceCommitBinding { action: commit.action }),
        }
    }
}

impl From<ArtifactInferenceServiceMetadata> for WireArtifactInferenceMetadata {
    fn from(metadata: ArtifactInferenceServiceMetadata) -> Self {
        Self {
            owner: metadata.owner.into(),
            artifact_kind: metadata.artifact_kind.into(),
            artifact_schema: metadata.artifact_schema.into(),
            artifact_schema_version: metadata.artifact_schema_version,
            inference_schema: metadata.inference_schema.into(),
            inference_schema_version: metadata.inference_schema_version,
            algorithm_version: metadata.algorithm_version,
            policy_version: metadata.policy_version,
            payload: metadata.payload.map(Into::into),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireArtifactInferenceRequest {
    pub wire_version: u32,
    pub owner: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub artifact_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    pub revision: u64,
    pub generation: u64,
    pub source_dialect: String,
    pub policy: Vec<u8>,
    pub budgets: WireArtifactInferenceBudget,
    pub retained: RetainedCloneGrant,
    pub cancellation_id: String,
    pub previous_state: Option<Vec<u8>>,
    pub requested_cache_mode: WireArtifactInferenceCacheMode,
    pub canonical_payload: Vec<u8>,
    pub dependencies: Vec<(String, Vec<u8>)>,
}

/// ⏱️ Finite host-enforced inference work limits.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireArtifactInferenceBudget {
    pub allocation_bytes: u64,
    pub work_units: u64,
    pub recursion_depth: u32,
}

/// 🗃️ Requested or actual inference cache behavior.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub enum WireArtifactInferenceCacheMode {
    Cold,
    Incremental,
    Bypass,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireArtifactInferenceDiagnostic {
    pub code: String,
    pub message: String,
    pub severity: String,
    pub parameters: BTreeMap<String, String>,
}

/// 🧾️ Immutable provenance for one inference result.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireArtifactInferenceProvenance {
    pub owner: String,
    pub inference_schema: String,
    pub algorithm_version: u32,
    pub policy_version: u32,
    pub source_dialect: String,
}

#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct WireArtifactInferenceResult {
    pub retirement_progress: RetainedCloneProgress,
    pub wire_version: u32,
    pub owner: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub artifact_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    pub revision: u64,
    pub generation: u64,
    pub source_dialect: String,
    pub policy: Vec<u8>,
    pub budgets: WireArtifactInferenceBudget,
    pub retained: RetainedCloneGrant,
    pub previous_state: Option<Vec<u8>>,
    pub requested_cache_mode: WireArtifactInferenceCacheMode,
    pub canonical_payload: Option<Vec<u8>>,
    pub dependencies: Vec<(String, Vec<u8>)>,
    pub diagnostics: Vec<WireArtifactInferenceDiagnostic>,
    pub provenance: WireArtifactInferenceProvenance,
    pub validity: String,
    pub quality: String,
    pub complete: bool,
    pub actual_cache_mode: WireArtifactInferenceCacheMode,
    pub cancellation_id: String,
}

static ACTIVE_ARTIFACT_INFERENCES: std::sync::OnceLock<std::sync::Mutex<BTreeSet<String>>> = std::sync::OnceLock::new();

static CANCELLED_ARTIFACT_INFERENCES: std::sync::OnceLock<std::sync::Mutex<BTreeSet<String>>> = std::sync::OnceLock::new();

// 🚫️async: E1 pure accessor consumed by `impl Drop for ArtifactInferenceCancellationGuard`
// (Drop::drop is an externally-declared trait method and can never be async) — see R9.
pub(crate) fn active_artifact_inferences() -> &'static std::sync::Mutex<BTreeSet<String>> {
    ACTIVE_ARTIFACT_INFERENCES.get_or_init(|| std::sync::Mutex::new(BTreeSet::new()))
}

// 🚫️async: E1 pure accessor consumed by `impl Drop for ArtifactInferenceCancellationGuard`
// (Drop::drop is an externally-declared trait method and can never be async) — see R9.
pub(crate) fn cancelled_artifact_inferences() -> &'static std::sync::Mutex<BTreeSet<String>> {
    CANCELLED_ARTIFACT_INFERENCES.get_or_init(|| std::sync::Mutex::new(BTreeSet::new()))
}

pub(crate) struct ArtifactInferenceCancellationGuard(String);

impl Drop for ArtifactInferenceCancellationGuard {
    fn drop(&mut self) {
        if let Ok(mut active) = active_artifact_inferences().lock() {
            active.remove(&self.0);
        }
        if let Ok(mut cancelled) = cancelled_artifact_inferences().lock() {
            cancelled.remove(&self.0);
        }
    }
}

pub(crate) fn begin_artifact_inference(cancellation_id: &str) -> Result<ArtifactInferenceCancellationGuard, ArtifactInferenceExecutionError> {
    let mut active = active_artifact_inferences().lock().map_err(|_| ArtifactInferenceExecutionError::new("artifact-inference.unavailable", "active inference registry is poisoned"))?;
    if !active.insert(cancellation_id.to_owned()) {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.in-flight", format!("inference {cancellation_id:?} is already active")));
    }
    Ok(ArtifactInferenceCancellationGuard(cancellation_id.to_owned()))
}

pub fn inference_cancelled(cancellation_id: &str) -> Result<bool, ArtifactInferenceExecutionError> {
    cancelled_artifact_inferences().lock().map_err(|_| ArtifactInferenceExecutionError::new("artifact-inference.unavailable", "cancelled inference registry is poisoned")).map(|cancelled| cancelled.contains(cancellation_id))
}

fn bounded_len(value: u64, field: &str) -> Result<usize, ArtifactInferenceExecutionError> {
    usize::try_from(value).map_err(|_| ArtifactInferenceExecutionError::new("artifact-inference.budget", format!("{field} exceeds this runtime's address space")))
}

pub(crate) fn validate_wire_request_resources(request: &WireArtifactInferenceRequest) -> Result<(), ArtifactInferenceExecutionError> {
    if request.source_dialect.trim().is_empty() {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.source-dialect", "source dialect is required"));
    }
    ArtifactIdentity::parse(&request.source_dialect).map_err(|error| ArtifactInferenceExecutionError::new("artifact-inference.source-dialect", error.message()))?;
    if request.cancellation_id.trim().is_empty() {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.cancellation", "cancellation identity is required"));
    }
    if request.budgets.allocation_bytes == 0 || request.budgets.work_units == 0 || request.budgets.recursion_depth == 0 {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.budget", "allocation, work, and recursion budgets must be finite non-zero values"));
    }
    match request.requested_cache_mode {
        WireArtifactInferenceCacheMode::Incremental if request.previous_state.is_none() => {
            return Err(ArtifactInferenceExecutionError::new("artifact-inference.previous-state", "incremental inference requires previous state"));
        }
        WireArtifactInferenceCacheMode::Cold | WireArtifactInferenceCacheMode::Bypass if request.previous_state.is_some() => {
            return Err(ArtifactInferenceExecutionError::new("artifact-inference.previous-state", "cold or bypass inference must not receive previous state"));
        }
        _ => {}
    }
    let mut identifiers = BTreeSet::new();
    let mut bytes = request.policy.len().saturating_add(request.canonical_payload.len());
    if let Some(previous_state) = &request.previous_state {
        bytes = bytes.saturating_add(previous_state.len());
    }
    for (owner, payload) in &request.dependencies {
        if owner.trim().is_empty() || !identifiers.insert(owner) {
            return Err(ArtifactInferenceExecutionError::new("artifact-inference.dependencies", "dependency owners must be non-empty and unique"));
        }
        if inference_child_dependency_parts(owner).is_none() {
            ArtifactIdentity::parse(owner).map_err(|error| ArtifactInferenceExecutionError::new("artifact-inference.dependencies", error.message()))?;
        }
        bytes = bytes.saturating_add(owner.len()).saturating_add(payload.len());
    }
    let allocation = bounded_len(request.budgets.allocation_bytes, "allocation-bytes")?;
    if bytes > allocation {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.budget", format!("request consumes {bytes} bytes, above allocation limit {allocation}")));
    }
    let work = 1u64.saturating_add(request.dependencies.len() as u64);
    if work > request.budgets.work_units {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.budget", format!("request requires {work} work units, above limit {}", request.budgets.work_units)));
    }
    Ok(())
}

pub(crate) fn validate_inference_execution(request: &WireArtifactInferenceRequest, execution: &ArtifactInferenceExecution) -> Result<(), ArtifactInferenceExecutionError> {
    if execution.validity.trim().is_empty() || execution.quality.trim().is_empty() {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.result", "inference execution must report validity and quality"));
    }
    if execution.actual_cache_mode != request.requested_cache_mode {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.cache-mode", "inference execution did not honor the requested cache mode"));
    }
    let diagnostics = execution.diagnostics.iter().fold(0usize, |total, diagnostic| {
        total
            .saturating_add(diagnostic.code.len())
            .saturating_add(diagnostic.message.len())
            .saturating_add(diagnostic.severity.len())
            .saturating_add(diagnostic.parameters.iter().fold(0usize, |total, (key, value)| total.saturating_add(key.len()).saturating_add(value.len())))
    });
    if execution.complete&&execution.canonical_payload.is_none(){return Err(ArtifactInferenceExecutionError::new("artifact-inference.result-pending","terminal inference requires its original physical payload"));}
    let total = execution.canonical_payload.as_ref().map_or(0,Vec::len).saturating_add(diagnostics);
    let allocation = bounded_len(request.budgets.allocation_bytes, "allocation-bytes")?;
    if total > allocation {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.budget", format!("result consumes {total} bytes, above allocation limit {allocation}")));
    }
    Ok(())
}

pub async fn wire_list_artifact_inference_services() -> Result<Vec<u8>, ArtifactInferenceExecutionError> {
    let registry = artifact_inference_service_registry().read().map_err(|_| ArtifactInferenceExecutionError::new("artifact-inference.unavailable", "artifact inference registry is poisoned"))?;
    Ok(wire_list_artifact_inference_services_with_routes(&registry, &[]))
}

pub(crate) fn wire_list_artifact_inference_services_with_routes(registry: &ArtifactInferenceServiceRegistry, routes: &[ArtifactInferenceServiceMetadata]) -> Vec<u8> {
    let mut catalog: BTreeMap<ArtifactInferenceServiceKey, ArtifactInferenceServiceMetadata> = registry.metadata().into_iter().map(|metadata| (metadata.into(), metadata)).collect();
    for metadata in routes {
        catalog.insert((*metadata).into(), *metadata);
    }
    let metadata: Vec<WireArtifactInferenceMetadata> = catalog.into_values().map(Into::into).collect();
    semio_framework_pack_json::to_json_string(&metadata).into_bytes()
}

/// 📜️ The IDENTITY projection, not the whole row: a request names an inference and never
/// re-states the contract the service publishes, so comparing `payload` here would refuse
/// every real call the moment an owner published one.
pub(crate) fn validate_wire_request_metadata(request: &WireArtifactInferenceRequest, expected: &WireArtifactInferenceMetadata) -> Result<(), ArtifactInferenceExecutionError> {
    let actual = WireArtifactInferenceMetadata {
        owner: request.owner.clone(),
        artifact_kind: request.artifact_kind.clone(),
        artifact_schema: request.artifact_schema.clone(),
        artifact_schema_version: request.artifact_schema_version,
        inference_schema: request.inference_schema.clone(),
        inference_schema_version: request.inference_schema_version,
        algorithm_version: request.algorithm_version,
        policy_version: request.policy_version,
        payload: None,
    };
    let expected_identity = WireArtifactInferenceMetadata { payload: None, ..expected.clone() };
    if actual != expected_identity {
        return Err(ArtifactInferenceExecutionError::new("artifact-inference.metadata-mismatch", format!("request metadata {actual:?} does not match registered service {expected_identity:?}")));
    }
    Ok(())
}

#[cfg(test)]
include!("../🧪️tests/🔬️app-artifact-inference-wire/🦀️.rs");

/// ⚠️ Original typed causes cross the recipient without display allocation.
#[derive(semio_framework_value::RetireOwned)]
pub enum ArtifactInferenceGatewayFailure{Native(ValueError),Json(semio_framework_pack_json::JsonError),Execution(ArtifactInferenceExecutionError),Literal(&'static str)}

/// 🧾️ Holds the original semantic cause and a distinct original receipt refusal.
#[derive(semio_framework_value::RetireOwned)]
pub struct ArtifactInferenceGatewayRefusal{pub primary:ArtifactInferenceGatewayFailure,pub receipt_refusal:Option<ValueError>}
impl From<ArtifactInferenceGatewayFailure> for ArtifactInferenceGatewayRefusal{fn from(primary:ArtifactInferenceGatewayFailure)->Self{Self{primary,receipt_refusal:None}}}
impl From<ValueError> for ArtifactInferenceGatewayRefusal{fn from(primary:ValueError)->Self{ArtifactInferenceGatewayFailure::Native(primary).into()}}

impl From<ValueError> for ArtifactInferenceGatewayFailure{fn from(value:ValueError)->Self{Self::Native(value)}}

impl ArtifactInferenceGatewayFailure {
 pub(crate) fn fault_wire_fields(&self)->(&str,&str,RetainedCloneProgress){
  match self{
   Self::Native(error)=>(error.kind.as_str(),&error.message,error.retained_progress()),
   Self::Execution(error)=>(error.code,error.cause.as_ref().map_or(error.message.as_str(),|cause|cause.message.as_ref()),error.retained_progress),
   Self::Literal(message)=>("inference.gateway.refusal",message,Default::default()),
   Self::Json(error)=>{use semio_framework_pack_json::JsonError;match error{
    JsonError::Native(error)=>(error.kind.as_str(),&error.message,error.retained_progress()),
    JsonError::DuplicateMember{name,..}=>("json.duplicateMember",name,Default::default()),
    JsonError::UnexpectedEof=>("json.unexpectedEof","unexpectedEof",Default::default()),JsonError::UnexpectedByte{..}=>("json.unexpectedByte","unexpectedByte",Default::default()),JsonError::InvalidNumber(_)=>("json.invalidNumber","invalidNumber",Default::default()),JsonError::InvalidEscape(_)=>("json.invalidEscape","invalidEscape",Default::default()),JsonError::InvalidUnicodeEscape(_)=>("json.invalidUnicodeEscape","invalidUnicodeEscape",Default::default()),JsonError::UnpairedSurrogate(_)=>("json.unpairedSurrogate","unpairedSurrogate",Default::default()),JsonError::ControlCharacterInString{..}=>("json.controlCharacterInString","controlCharacterInString",Default::default()),JsonError::InvalidUtf8=>("json.invalidUtf8","invalidUtf8",Default::default()),JsonError::TrailingData(_)=>("json.trailingData","trailingData",Default::default()),JsonError::MaxDepthExceeded(_)=>("json.maxDepthExceeded","maxDepthExceeded",Default::default()),
   }}
  }
 }
 /// 🧭️ Borrows scalar JSON positions from the same retained grammar refusal.
 pub(crate) fn fault_source(&self)->Option<semio_framework_diagnostic::FaultSource>{
  use semio_framework_pack_json::JsonError;use semio_framework_diagnostic::FaultSource;let Self::Json(error)=self else{return None};Some(match error{
   JsonError::UnexpectedByte{found,offset}|JsonError::ControlCharacterInString{byte:found,offset}=>FaultSource{offset:Some(*offset as u64),byte:Some(*found),depth:None},
   JsonError::InvalidNumber(offset)|JsonError::InvalidEscape(offset)|JsonError::InvalidUnicodeEscape(offset)|JsonError::UnpairedSurrogate(offset)|JsonError::TrailingData(offset)|JsonError::DuplicateMember{offset,..}=>FaultSource{offset:Some(*offset as u64),..Default::default()},
   JsonError::MaxDepthExceeded(depth)=>FaultSource{depth:Some(u64::from(*depth)),..Default::default()},_=>return None,
  })
 }
 pub(crate) fn source_identity(&self)->usize{self.fault_wire_fields().1.as_ptr()as usize}
}
