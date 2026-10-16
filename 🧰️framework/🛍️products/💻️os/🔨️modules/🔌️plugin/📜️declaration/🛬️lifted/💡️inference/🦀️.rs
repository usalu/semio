//! 💡️ Lift file: 36 item(s), about 259 lines, of `💡️inference/🦀️.rs` that belong to crate `decl-registry` (WP2).
//!
//! WP3 owns the source file and cuts the items here (ArtifactInferenceServiceMetadata, ArtifactInferencePayloadContract, ArtifactInferenceCommitBinding, ArtifactInferenceDocumentBinding, impl From for InferenceArtifactBinding, impl From for InferencePayloadContract, ...); WP2 wires this file with one `mod` line in the crate root. Until `decl-registry` exists, the source module re-exports the items with `pub use`.
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use std::collections::BTreeMap;
use semio_framework_value::RetirementDemand;
use semio_framework_value::ValueError;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value::retained_clone::RetainedCloneProgress;
use semio_framework_plugin_identity::ArtifactExecutableIdentity;
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
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct ArtifactInferenceExecutionError {
    pub code: &'static str,
    pub message: String,
    pub retained_progress:RetainedCloneProgress,
    pub cause:Option<semio_framework_job::OriginalTurnFailure>,
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
    /// 🪢️ Moves the complete original envelope after producer admission, preserving every receipt and context cause.
    pub fn original(failure:semio_framework_job::OriginalTurnFailure,terminal:bool)->Self{let retained_progress=failure.primary.retained_progress();Self{code:"artifact-inference.original",message:String::new(),retained_progress,cause:Some(failure),terminal}}
    pub fn native(error:ValueError,terminal:bool)->Self{let retained_progress=error.retained_progress();Self{code:"artifact-inference.native",message:String::new(),retained_progress,cause:Some(error.into()),terminal}}
}

impl semio_framework_job::OriginalDomainProjection for ArtifactInferenceExecutionError{
 fn projection_view(&self,path:&[usize])->Result<semio_framework_job::OriginalDomainView<'_>,ValueError>{
  use semio_framework_job::OriginalDomainView as V;match path{
   []=>Ok(V::IntrinsicObject(5)),[0]=>Ok(V::Text(self.code)),[1]=>Ok(V::Text(&self.message)),[2]=>Ok(V::IntrinsicObject(4)),
   [2,0]=>Ok(V::UInt(self.retained_progress.copied_items as u64)),[2,1]=>Ok(V::UInt(self.retained_progress.copied_bytes as u64)),[2,2]=>Ok(V::UInt(self.retained_progress.retained_capacity_bytes as u64)),[2,3]=>Ok(V::UInt(self.retained_progress.released_bytes as u64)),
   [3,rest @ ..]=>match &self.cause{Some(original)=>semio_framework_job::project_original_turn_failure(original,rest),None if rest.is_empty()=>Ok(V::IntrinsicNull),_=>Err(inference_execution_projection_refusal())},[4]=>Ok(V::Bool(self.terminal)),_=>Err(inference_execution_projection_refusal()),
  }
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{match path{
  []=>["code","message","retainedProgress","cause","terminal"].get(index).copied().ok_or_else(inference_execution_projection_refusal),
  [2]=>["items","copyBytes","capacityBytes","releaseBytes"].get(index).copied().ok_or_else(inference_execution_projection_refusal),
  [3,rest @ ..]=>self.cause.as_ref().ok_or_else(inference_execution_projection_refusal).and_then(|original|semio_framework_job::key_original_turn_failure(original,rest,index)),_=>Err(inference_execution_projection_refusal()),
 }}
}
fn inference_execution_projection_refusal()->ValueError{ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original inference execution failure projection ordinal changed")}

impl semio_framework_job::OriginalDomainFailure for ArtifactInferenceExecutionError{
 fn domain_code(&self)->&str{self.code}
 fn domain_message(&self)->&str{self.cause.as_ref().map_or(self.message.as_str(),|original|original.primary.message())}
 fn domain_retained_progress(&self)->RetainedCloneProgress{self.retained_progress}
}

impl std::fmt::Display for ArtifactInferenceExecutionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.cause.as_ref().map_or(self.message.as_str(),|original|original.primary.message()))
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
pub enum ArtifactInferenceExecutable {
    Pure(ArtifactInference,ArtifactInferenceDemands),
    Contextual(for<'a,'cx> fn(&ArtifactInferenceExecutionRequest<'a>, &dyn std::any::Any,&mut ArtifactInferenceStepContext<'cx>) -> Result<ArtifactInferenceExecutionStep, ArtifactInferenceExecutionError>,for<'a> fn(&ArtifactInferenceExecutionRequest<'a>,&dyn std::any::Any,usize)->Result<RetirementDemand,ValueError>),
}

/// 🧰️ Native type-erased bounded inference entry.
#[derive(Clone, Copy)]
pub struct ArtifactInferenceService {
    pub metadata: ArtifactInferenceServiceMetadata,
    pub infer: ArtifactInferenceExecutable,
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
    pub fn identical_to(&self,other:&Self)->bool{self.metadata==other.metadata&&match(self.infer,other.infer){(ArtifactInferenceExecutable::Pure(a,ad),ArtifactInferenceExecutable::Pure(b,bd))=>std::ptr::fn_addr_eq(a,b)&&std::ptr::fn_addr_eq(ad,bd),(ArtifactInferenceExecutable::Contextual(a,ad),ArtifactInferenceExecutable::Contextual(b,bd))=>std::ptr::fn_addr_eq(a,b)&&std::ptr::fn_addr_eq(ad,bd),_=>false}}
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
    pub by_key: BTreeMap<ArtifactInferenceServiceKey, ArtifactInferenceService>,
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
