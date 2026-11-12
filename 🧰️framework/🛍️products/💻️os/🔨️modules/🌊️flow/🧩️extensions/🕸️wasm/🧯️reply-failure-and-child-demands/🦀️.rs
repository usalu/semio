//! 🧯️ Reply, failure and child demands of the wasm extension glue: the evaluation reply, the operator input parse, the child demands and close and the evaluation failure with its conversions. See [`super`].

use super::*;

// #region 🔖️Evaluate
/// 🧵️ The encoded evaluation envelope stays in the paged pages the Pack writer produced until the invocation boundary copies it out.
pub type EvaluationWire=semio_framework_value::paged::PagedBytes<{isize::MAX as usize}>;

/// 🧾️ Publishes the original physical envelope with facts from its same execution owner.
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
pub struct EvaluationReply {pub external_pending:bool,pub(super) pending_source:Option<neural_engine::PendingExtensionEval>,pub wire:Option<EvaluationWire>,pub complete:bool,pub faulted:bool,pub retirement_progress:RetainedCloneProgress,pub(super) retirement_posted:bool,pub returned_request:Option<EvaluateRequest>,pub refusal:Option<EvaluationFailure>}

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

pub fn evaluation_admit_original<T:RetireOwned>(original:&mut Option<T>,active:&mut Option<Box<dyn ErasedSnapshotRetirement>>,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
    let grant=effect.grant();let empty=RetainedCloneProgress::default();
    if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
    let demand=evaluation_owned_birth::<T>();
    if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation source admission exceeds supplied depth").into());}
    if grant.maximum_capacity_bytes<demand.capacity_bytes{return Ok(RetainedCloneStep::Progress(empty));}
    if active.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"evaluation source admission retains a live child").into());}
    if original.is_none(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"evaluation source admission lost its original field").into());}
    Ok(RetainedCloneStep::Progress(match semio_framework_value::retirement::admit_owned_retirement(original,effect).result?{
        Some((owner,progress))=>{*active=Some(owner);progress},
        None=>empty,
    }))
}

pub fn evaluation_child_demands(active:&Option<Box<dyn ErasedSnapshotRetirement>>,copy:usize)->Result<RetirementDemand,ValueError>{
    active.as_ref().map_or(Ok(Default::default()),|owner|semio_framework_value::factory_ticket_demands(owner,copy))
}

pub fn evaluation_child_close(active:&mut Option<Box<dyn ErasedSnapshotRetirement>>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>{
    RetirementPollExit::resume(semio_framework_value::close_factory_ticket(active,body,effect))
}

/// ⏳️ A ladder exit inside a normal turn keeps a suspended child pending: the retained state retries on the next turn and is never refused.
pub(super) fn evaluation_settled(result:Result<RetainedCloneStep,RetirementPollExit>)->Result<RetainedCloneStep,EvaluationFailure>{
    match result{Ok(step)=>Ok(step),Err(RetirementPollExit::Pending(_))=>Ok(RetainedCloneStep::Progress(Default::default())),Err(RetirementPollExit::Failed(failure))=>Err(failure.into())}
}

/// 🪙️ Runs one leaf step inside the turn's own original effect and charges its performed work; `None` means the work quote was not admitted and nothing ran.
pub(super) fn evaluation_effect_cx<R>(cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant,run:impl FnOnce(&mut semio_framework_job::StepContext<'_>,&mut RetainedAllocationEffect<'_>)->R)->Result<Option<R>,ValueError>{
    Ok(cx.with_original_effect(grant,run)?.map(|(output,performed)|{cx.consume_work(performed);output}))
}

/// 🪙️ Runs one leaf step that needs no turn access inside the turn's own original effect.
pub(super) fn evaluation_effect<R>(cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Result<Option<R>,ValueError>{
    evaluation_effect_cx(cx,grant,|_,effect|run(effect))
}

/// 🪙️ A leaf close ladder under the turn's effect: an unadmitted quote or a suspended child is zero progress that retries next turn.
pub(super) fn evaluation_effect_step(cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,RetirementPollExit>)->Result<RetainedCloneStep,EvaluationFailure>{
    match evaluation_effect(cx,grant,run)?{Some(result)=>evaluation_settled(result),None=>Ok(RetainedCloneStep::Progress(Default::default()))}
}

/// 🔁️ Resumes a saved native decoding continuation, handing it back to its slot when the resume is refused.
pub(super) fn evaluation_resume_decoding<'a>(slot:&mut Option<semio_framework_value_native_owned::native_decoding::NativeDecodeContinuation>,callback:&'a mut dyn FnMut(semio_framework_value_native_owned::native_decoding::NativeDecodeProgress)->bool)->Result<semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'a>,ValueError>{
    match slot.take(){
        Some(receipt)=>semio_framework_value_native_owned::native_decoding::NativeDecodeControl::resume(receipt,callback).map_err(|(error,receipt)|{*slot=Some(receipt);error}),
        None=>Ok(semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_retained(callback)),
    }
}

/// 🔁️ Pauses a native decoding control into its slot.
pub(super) fn evaluation_pause_decoding(control:semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,slot:&mut Option<semio_framework_value_native_owned::native_decoding::NativeDecodeContinuation>)->Result<(),ValueError>{
    *slot=Some(control.pause().map_err(|(error,_)|error)?);Ok(())
}

/// 🔁️ Resumes a saved native encoding continuation, handing it back to its slot when the resume is refused.
pub(super) fn evaluation_resume_encoding<'a>(slot:&mut Option<semio_framework_value::NativeEncodeContinuation>,callback:&'a mut dyn FnMut(semio_framework_value::native_encoding::NativeEncodeProgress)->bool)->Result<semio_framework_value::NativeEncodeControl<'a>,ValueError>{
    match slot.take(){
        Some(receipt)=>semio_framework_value::NativeEncodeControl::resume(receipt,callback).map_err(|(error,receipt)|{*slot=Some(receipt);error}),
        None=>Ok(semio_framework_value::NativeEncodeControl::new_retained(callback)),
    }
}

/// 🔁️ Pauses a native encoding control into its slot.
pub(super) fn evaluation_pause_encoding(control:semio_framework_value::NativeEncodeControl<'_>,slot:&mut Option<semio_framework_value::NativeEncodeContinuation>)->Result<(),ValueError>{
    *slot=Some(control.pause().map_err(|(error,_)|error)?);Ok(())
}

/// ⚠️ Keeps each original typed failure without allocating display text during a normal turn.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum EvaluationFailure {Pack(pack::record::BorrowedProjectedPackFailure),Native(ValueError),Json(semio_framework_pack_json::JsonError),Operator(neural_engine::EvalError),Work(neural_engine::OperatorWorkFailure),Original(semio_framework_job::OriginalTurnFailure),Literal(&'static str)}

impl From<pack::record::BorrowedProjectedPackFailure> for EvaluationFailure{fn from(error:pack::record::BorrowedProjectedPackFailure)->Self{Self::Pack(error)}}

impl From<ValueError> for EvaluationFailure {fn from(error:ValueError)->Self{Self::Native(error)}}

impl From<semio_framework_pack_json::JsonError> for EvaluationFailure {fn from(error:semio_framework_pack_json::JsonError)->Self{Self::Json(error)}}

impl From<neural_engine::EvalError> for EvaluationFailure {fn from(error:neural_engine::EvalError)->Self{Self::Operator(error)}}

impl From<semio_framework_job::OriginalTurnFailure> for EvaluationFailure{fn from(original:semio_framework_job::OriginalTurnFailure)->Self{Self::Original(original)}}

impl From<semio_framework_value::OriginalRetirementFailure> for EvaluationFailure{fn from(original:semio_framework_value::OriginalRetirementFailure)->Self{Self::Original(original.into())}}

impl From<neural_engine::OperatorWorkFailure> for EvaluationFailure{fn from(original:neural_engine::OperatorWorkFailure)->Self{Self::Work(original)}}

impl From<&'static str> for EvaluationFailure {fn from(message:&'static str)->Self{Self::Literal(message)}}

impl std::fmt::Display for EvaluationFailure {fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Pack(error)=>formatter.write_str(&error.reason.message),Self::Native(error)=>std::fmt::Display::fmt(error,formatter),Self::Json(error)=>std::fmt::Display::fmt(error,formatter),Self::Operator(error)=>std::fmt::Display::fmt(error,formatter),Self::Original(error)=>formatter.write_str(error.primary.message()),Self::Work(error)=>formatter.write_str(semio_framework_job::OriginalDomainFailure::domain_message(error)),Self::Literal(message)=>formatter.write_str(message)}}}

impl std::error::Error for EvaluationFailure {}
// #endregion ⏱️EvaluationJobs
