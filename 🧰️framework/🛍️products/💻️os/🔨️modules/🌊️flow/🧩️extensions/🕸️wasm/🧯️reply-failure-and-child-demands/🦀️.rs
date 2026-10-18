//! 🧯️ Reply, failure and child demands of the wasm extension glue: the evaluation reply, the operator input parse, the child demands and close and the evaluation failure with its conversions. See [`super`].

use super::*;

// #region 🔖️Evaluate
/// 🧾️ Publishes the original physical envelope with facts from its same execution owner.
#[derive(semio_framework_value::RetireOwned)]
pub struct EvaluationReply {pub external_pending:bool,pub(super) pending_source:Option<neural_engine::PendingExtensionEval>,pub wire:Option<Vec<u8>>,pub complete:bool,pub faulted:bool,pub retirement_progress:RetainedCloneProgress,pub(super) retirement_posted:bool,pub returned_request:Option<EvaluateRequest>,pub refusal:Option<EvaluationFailure>}

impl EvaluationReply{
    pub(super) fn pending_request(request:EvaluateRequest,retirement_progress:RetainedCloneProgress)->Self{Self{retirement_posted:false,external_pending:false,pending_source:None,wire:None,complete:false,faulted:false,retirement_progress,returned_request:Some(request),refusal:None}}
    pub(super) fn refused(request:EvaluateRequest,retirement_progress:RetainedCloneProgress,message:&'static str)->Self{Self::refused_original(request,retirement_progress,EvaluationFailure::Literal(message))}
    pub(super) fn refused_original(request:EvaluateRequest,retirement_progress:RetainedCloneProgress,refusal:EvaluationFailure)->Self{Self{retirement_posted:false,external_pending:false,pending_source:None,wire:None,complete:false,faulted:true,retirement_progress,returned_request:Some(request),refusal:Some(refusal)}}
}

/// 🧮️ The operator input `dispatch`/`dispatch_job` both see: the request's `inputJson` parsed and
/// given this operator's declared channel defaults. Factored out so the budgeted path and the
/// one-shot path cannot disagree about what the operator was asked.
fn parsed_operator_input(registry: &Registry, operator_id: &str, input_json: &str) -> Result<ColdOwner<Dictionary>, String> {
    let input: Dictionary = semio_framework_pack_json::from_json_str(input_json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    Ok(ColdOwner::new(match registry.operator_info(operator_id) {
        Some(info) => inject_channel_defaults(input, info),
        None => input,
    }))
}
// #endregion 🔖️Evaluate

// #region ⏱️EvaluationJobs
pub(super) fn evaluation_value_demands(owner:&neural_engine::ValueRetirement,copy:usize)->Result<RetirementDemand,ValueError>{
    Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})
}

pub fn evaluation_owned_birth<T:RetireOwned>()->RetirementDemand{
    RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(),depth:1,..Default::default()}
}

pub fn evaluation_admit_original<T:RetireOwned>(original:&mut Option<T>,active:&mut Option<Box<dyn ErasedSnapshotRetirement>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
    let empty=RetainedCloneProgress::default();
    if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
    let demand=evaluation_owned_birth::<T>();
    if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation source admission exceeds supplied depth"));}
    if grant.maximum_capacity_bytes<demand.capacity_bytes{return Ok(RetainedCloneStep::Progress(empty));}
    if active.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"evaluation source admission retains a live child"));}
    let source=original.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"evaluation source admission lost its original field"))?;
    match semio_framework_value::retirement::admit_owned_retirement(source,grant){
        Ok((owner,progress))=>{*active=Some(owner);Ok(RetainedCloneStep::Progress(progress))},
        Err((error,source))=>{*original=Some(source);Err(error)},
    }
}

pub fn evaluation_child_demands(active:&Option<Box<dyn ErasedSnapshotRetirement>>,copy:usize)->Result<RetirementDemand,ValueError>{
    active.as_ref().map_or(Ok(Default::default()),|owner|semio_framework_value::factory_ticket_demands(owner,copy))
}

pub fn evaluation_child_close(active:&mut Option<Box<dyn ErasedSnapshotRetirement>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
    semio_framework_value::close_factory_ticket(active,grant)
}

/// ⚠️ Keeps each original typed failure without allocating display text during a normal turn.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum EvaluationFailure {Pack(pack::record::BorrowedProjectedPackFailure),Native(ValueError),Json(semio_framework_pack_json::JsonError),Operator(neural_engine::EvalError),Work(neural_engine::OperatorWorkFailure),Original(semio_framework_job::OriginalTurnFailure),Literal(&'static str)}

impl From<pack::record::BorrowedProjectedPackFailure> for EvaluationFailure{fn from(error:pack::record::BorrowedProjectedPackFailure)->Self{Self::Pack(error)}}

impl From<ValueError> for EvaluationFailure {fn from(error:ValueError)->Self{Self::Native(error)}}

impl From<semio_framework_pack_json::JsonError> for EvaluationFailure {fn from(error:semio_framework_pack_json::JsonError)->Self{Self::Json(error)}}

impl From<neural_engine::EvalError> for EvaluationFailure {fn from(error:neural_engine::EvalError)->Self{Self::Operator(error)}}

impl From<semio_framework_job::OriginalTurnFailure> for EvaluationFailure{fn from(original:semio_framework_job::OriginalTurnFailure)->Self{Self::Original(original)}}

impl From<neural_engine::OperatorWorkFailure> for EvaluationFailure{fn from(original:neural_engine::OperatorWorkFailure)->Self{Self::Work(original)}}

impl From<&'static str> for EvaluationFailure {fn from(message:&'static str)->Self{Self::Literal(message)}}

impl std::fmt::Display for EvaluationFailure {fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Pack(error)=>formatter.write_str(&error.reason.message),Self::Native(error)=>std::fmt::Display::fmt(error,formatter),Self::Json(error)=>std::fmt::Display::fmt(error,formatter),Self::Operator(error)=>std::fmt::Display::fmt(error,formatter),Self::Original(error)=>formatter.write_str(error.primary.message()),Self::Work(error)=>formatter.write_str(semio_framework_job::OriginalDomainFailure::domain_message(error)),Self::Literal(message)=>formatter.write_str(message)}}}

impl std::error::Error for EvaluationFailure {}
// #endregion ⏱️EvaluationJobs
