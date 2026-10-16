//! 💡️ Artifact inference services: metadata, registry, cancellation and the cold wire entry points.
use semio_framework_plugin_plugin_program::ARTIFACT_INFERENCE_WIRE_VERSION;
use semio_framework_plugin_decl_registry::ArtifactInferenceExecution;
use semio_framework_plugin_decl_registry::ArtifactInferenceExecutionError;
use semio_framework_plugin_decl_registry::ArtifactInferenceExecutionRequest;
use semio_framework_plugin_decl_registry::ArtifactInferenceRegistrationConflict;
use semio_framework_plugin_decl_registry::ArtifactInferenceRegistrationError;
use semio_framework_plugin_decl_registry::ArtifactInferenceStepContext;
use semio_framework_plugin_decl_registry::WireArtifactInferenceBudget;
use semio_framework_plugin_decl_registry::WireArtifactInferenceCacheMode;
use semio_framework_plugin_plugin_program::WireArtifactInferenceRequest;
use semio_framework_plugin_plugin_program::WireArtifactInferenceResult;
use semio_framework_plugin_plugin_program::artifact_inference_service_registry;
use semio_framework_plugin_plugin_program::inference_child_dependency_parts;
use semio_framework_plugin_plugin_program::validate_wire_request_resources;
#[cfg(test)]
use semio_framework_plugin_plugin_program::inference_gateway::wire_artifact_infer_from_cold;
#[cfg(test)]
use semio_framework_plugin_plugin_program::wire_artifact_infer_cold;
#[cfg(test)]
use crate::inference_gateway::cancel_artifact_inference;
use protocol::{OpBinary, OpText};
use semio_framework_value_derive::{FromValue, ToValue};
use pack::snapshot::ArtifactPack;
use semio_framework_value::{RetirementDemand, ValueError};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_plugin_decl_surface::artifact_pair_snapshot;
#[cfg(test)]
use semio_framework_plugin_decl_views::NoConfig;

/// 🪆️ The `dependencies` key under which a composed document's owned child at `slot`/`child_id` travels with an inference
/// request (design §20.15): its head snapshot pack.
pub fn inference_child_dependency(slot: &str, child_id: &str) -> String {
    format!("child:{slot}/{child_id}")
}

/// 🧵️ The owned child at `slot`/`child_id` of the requesting composed document, decoded from the request's dependencies (design
/// §20.15: readers compose on read; the requester injects every owned child under [`inference_child_dependency`]).
pub fn inference_child<S: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot>(request: &ArtifactInferenceExecutionRequest<'_>, slot: &str, child_id: &str) -> Result<S, ArtifactInferenceExecutionError> {
    let key = inference_child_dependency(slot, child_id);
    let (_, pack) = request.dependencies.iter().find(|(name, _)| *name == key).ok_or_else(|| ArtifactInferenceExecutionError::new("artifact-inference.child-missing", format!("the request carries no owned child {slot}/{child_id}")))?;
    S::decode_pack(pack).map_err(|error| ArtifactInferenceExecutionError::new("artifact-inference.child-decode", format!("owned child {slot}/{child_id}: {error:?}")))
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
 pub fn fault_wire_fields(&self)->(&str,&str,RetainedCloneProgress){
  match self{
   Self::Native(error)=>(error.kind.as_str(),&error.message,error.retained_progress()),
   Self::Execution(error)=>(error.code,error.cause.as_ref().map_or(error.message.as_str(),|cause|cause.primary.message()),error.retained_progress),
   Self::Literal(message)=>("inference.gateway.refusal",message,Default::default()),
   Self::Json(error)=>{use semio_framework_pack_json::JsonError;match error{
    JsonError::Native(error)=>(error.kind.as_str(),&error.message,error.retained_progress()),
    JsonError::DuplicateMember{name,..}=>("json.duplicateMember",name,Default::default()),
    JsonError::UnexpectedEof=>("json.unexpectedEof","unexpectedEof",Default::default()),JsonError::UnexpectedByte{..}=>("json.unexpectedByte","unexpectedByte",Default::default()),JsonError::InvalidNumber(_)=>("json.invalidNumber","invalidNumber",Default::default()),JsonError::InvalidEscape(_)=>("json.invalidEscape","invalidEscape",Default::default()),JsonError::InvalidUnicodeEscape(_)=>("json.invalidUnicodeEscape","invalidUnicodeEscape",Default::default()),JsonError::UnpairedSurrogate(_)=>("json.unpairedSurrogate","unpairedSurrogate",Default::default()),JsonError::ControlCharacterInString{..}=>("json.controlCharacterInString","controlCharacterInString",Default::default()),JsonError::InvalidUtf8=>("json.invalidUtf8","invalidUtf8",Default::default()),JsonError::TrailingData(_)=>("json.trailingData","trailingData",Default::default()),JsonError::MaxDepthExceeded(_)=>("json.maxDepthExceeded","maxDepthExceeded",Default::default()),
   }}
  }
 }
 /// 🧭️ Borrows scalar JSON positions from the same retained grammar refusal.
 pub fn fault_source(&self)->Option<semio_framework_diagnostic::FaultSource>{
  use semio_framework_pack_json::JsonError;use semio_framework_diagnostic::FaultSource;let Self::Json(error)=self else{return None};Some(match error{
   JsonError::UnexpectedByte{found,offset}|JsonError::ControlCharacterInString{byte:found,offset}=>FaultSource{offset:Some(*offset as u64),byte:Some(*found),depth:None},
   JsonError::InvalidNumber(offset)|JsonError::InvalidEscape(offset)|JsonError::InvalidUnicodeEscape(offset)|JsonError::UnpairedSurrogate(offset)|JsonError::TrailingData(offset)|JsonError::DuplicateMember{offset,..}=>FaultSource{offset:Some(*offset as u64),..Default::default()},
   JsonError::MaxDepthExceeded(depth)=>FaultSource{depth:Some(u64::from(*depth)),..Default::default()},_=>return None,
  })
 }
 pub fn source_identity(&self)->usize{self.fault_wire_fields().1.as_ptr()as usize}
}

pub use semio_framework_plugin_decl_registry::artifact_inference::*;

pub use semio_framework_plugin_plugin_program::artifact_inference::*;
