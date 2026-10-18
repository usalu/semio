//! ⏱️ Evaluation jobs and cancellation of the wasm extension glue: the job registry, dependency identity, cancellation, retirement demands and close step and the progress probe. See [`super`].

use super::*;

// #region ⏱️EvaluationJobs
#[derive(Default)]
pub(super) struct EvaluationJobRegistry {
    pub jobs:protocol::HistoryFoldIndex<(neural_engine::RegistryIdentity,String,u64),RetainedEvaluation>,
    retiring_key:Option<semio_framework_value::retirement::controlled::ControlledRetirement<(neural_engine::RegistryIdentity,String,u64)>>,
    retiring_backing:Option<semio_framework_value::retirement::controlled::ControlledRetirement<protocol::HistoryFoldIndex<(neural_engine::RegistryIdentity,String,u64),RetainedEvaluation>>>,
    pub(super) closing_owner:Option<neural_engine::RegistryIdentity>,pub(super) admission_owner:Option<neural_engine::RegistryIdentity>,pub(super) lookup_epoch:u64,
}

static EVALUATION_JOBS: std::sync::OnceLock<std::sync::Mutex<EvaluationJobRegistry>> = std::sync::OnceLock::new();

pub fn evaluation_jobs() -> &'static std::sync::Mutex<EvaluationJobRegistry> {
    EVALUATION_JOBS.get_or_init(|| std::sync::Mutex::new(EvaluationJobRegistry::default()))
}

pub fn evaluation_retire_cold<T:RetireOwned>(value:T){
    let mut owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("cold evaluation original declaration refused: {error}"));
    while !owner.terminal_is_empty(){
        let copy=owner.next_copy_byte_demand().expect("cold evaluation copy demand");let demand=evaluation_controlled_demands(&owner,copy).expect("cold evaluation physical demand");
        owner.step(RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}).expect("cold evaluation physical closure");
    }
}

/// 🧬️ Canonical dependency identity keeps source, wiring, parameters and operator version together.
pub fn evaluation_dependency_identity(request: &EvaluateRequest) -> String {
    let canonical=|text:&str| {
        let value=match semio_framework_pack_json::parse(text,semio_framework_pack_json::JsonMemberPolicy::Reject) {Ok(value)=>value,Err(_)=>return text.to_string()};
        let mut projection=semio_framework_pack_json::JsonValueProjection::new_ordered(value);
        let mut accepted=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(usize::MAX,&mut accepted);let value=loop {if let Some(value)=projection.step(4096,usize::MAX,&mut decode,COLD_EVALUATION_IDENTITY_POLICY).expect("canonical identity projection"){break value;}};
        let mut writer=semio_framework_pack_json::JsonWriteCursor::new(value);let mut admitted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(usize::MAX,&mut admitted);
        let text=loop {if let Some(text)=writer.step(4096,&mut control,COLD_EVALUATION_IDENTITY_POLICY).expect("canonical identity writer"){break text;}};
        evaluation_retire_cold(projection);evaluation_retire_cold(writer);text
    };
    semio_framework_pack_json::to_json_string(&DslValue::Array(vec![request.operator_id.to_value(),DslValue::String(canonical(&request.input_json)),DslValue::String(canonical(&request.dependency_json)),request.operator_version.to_value()]))
}

/// 🛑️ Marks original inference jobs without allocating or advancing physical retirement.
pub fn cancel_inference_evaluation(owner:&neural_engine::SharedRegistry,cancellation_id:&str)->bool{
    let mut accepted=false;
    if let Ok(mut registry)=evaluation_jobs().lock(){for retained in registry.jobs.slot_values_mut().filter(|retained|retained.owner_identity==owner.owner_identity()&&retained.current_cancellation()==cancellation_id&&!retained.cancelled){retained.cancel();accepted=true;}}
    accepted
}

/// 🛑️ Borrows the original operator key while retaining the same cancelled job.
pub fn cancel_evaluation(owner:&neural_engine::SharedRegistry,operator_id:&str,node_hash:u64)->bool{
    if let Ok(mut registry)=evaluation_jobs().lock(){if let Some((_,retained))=registry.jobs.slot_entries_mut().find(|(key,_)|key.0==owner.owner_identity()&&key.1==operator_id&&key.2==node_hash){if !retained.cancelled{retained.cancel();return true;}}}
    false
}

/// 🛑️ Acknowledges cancellation without performing a hidden cleanup turn.
pub fn cancel_all_evaluations(owner:&neural_engine::SharedRegistry)->usize{
    let mut accepted=0;
    if let Ok(mut registry)=evaluation_jobs().lock(){for retained in registry.jobs.slot_values_mut().filter(|retained|retained.owner_identity==owner.owner_identity()&&!retained.cancelled){retained.cancel();accepted+=1;}}
    accepted
}

fn evaluation_controlled_demands<T:RetireOwned>(owner:&semio_framework_value::retirement::controlled::ControlledRetirement<T>,copy:usize)->Result<RetirementDemand,ValueError>{
    Ok(RetirementDemand {copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})
}

fn evaluation_registry_demands(registry:&EvaluationJobRegistry,owner:neural_engine::RegistryIdentity,copy:usize)->Result<RetirementDemand,ValueError>{
    if registry.closing_owner==Some(owner){if let Some(key)=&registry.retiring_key{return evaluation_controlled_demands(key,copy);}if let Some(backing)=&registry.retiring_backing{return evaluation_controlled_demands(backing,copy);}}
    registry.jobs.values().find(|retained|retained.owner_identity==owner&&retained.cancelled).map_or(Ok(Default::default()),|retained|if retained.close_fields_terminal(false,false){Ok(RetirementDemand {depth:1,..Default::default()})}else{retained.retirement_demands(copy,false,false)})
}

/// 📏️ Borrows the actual cancelled child, original map key, or retained arena quote.
pub fn evaluation_retirement_demands(owner:&neural_engine::SharedRegistry,copy:usize)->Result<RetirementDemand,ValueError>{
    let registry=evaluation_jobs().lock().map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"evaluation registry demand is contended"))?;
    evaluation_registry_demands(&registry,owner.owner_identity(),copy)
}

/// 🧹️ Advances the same map under every independently supplied physical credit.
pub fn retire_cancelled_evaluations_close_step(owner:&neural_engine::SharedRegistry,grant:RetainedCloneGrant)->Result<RetainedCloneStep,EvaluationFailure>{
    let empty=RetainedCloneProgress::default();
    let mut registry=evaluation_jobs().lock().map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"evaluation registry close is contended"))?;
    let identity=owner.owner_identity();let demand=evaluation_registry_demands(&registry,identity,grant.maximum_copy_bytes)?;
    if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
    if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation registry close exceeds supplied depth").into());}
    if registry.closing_owner==Some(identity){
        if let Some(key)=&mut registry.retiring_key{
            let step=key.step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,key.terminal_is_empty(),"original evaluation map key")?;if key.terminal_is_empty(){registry.retiring_key=None;}return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(backing)=&mut registry.retiring_backing{
            let step=backing.step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,backing.terminal_is_empty(),"original evaluation map arena")?;if backing.terminal_is_empty(){registry.retiring_backing=None;registry.closing_owner=None;}return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        registry.closing_owner=None;
    }
    if registry.closing_owner.is_some(){return Ok(RetainedCloneStep::Progress(empty));}
    if let Some(retained)=registry.jobs.slot_values_mut().find(|retained|retained.owner_identity==identity&&retained.cancelled&&!retained.close_fields_terminal(false,false)){
        let step=retained.close_step(grant,false,false)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,retained.close_fields_terminal(false,false),"original cancelled evaluation")?;return Ok(RetainedCloneStep::Progress(step.progress()));
    }
    if registry.admission_owner==Some(identity)&&registry.jobs.is_empty()&&!registry.jobs.terminal_is_empty(){registry.lookup_epoch=registry.lookup_epoch.checked_add(1).expect("original registry backing epoch");registry.admission_owner=None;registry.closing_owner=Some(identity);registry.retiring_backing=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(std::mem::take(&mut registry.jobs)).unwrap_or_else(|(error,_)|panic!("original admitted evaluation backing refused: {error}")));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}
    for slot in 0..registry.jobs.slot_count(){
        if let Some((key,retained))=registry.jobs.extract_slot_if(slot,|_,retained|!(retained.owner_identity==identity&&retained.cancelled&&retained.close_fields_terminal(false,false))){
            registry.lookup_epoch=registry.lookup_epoch.checked_add(1).expect("original registry removed key epoch");drop(retained);registry.retiring_key=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(key).unwrap_or_else(|(error,_)|panic!("original evaluation key declaration refused: {error}")));registry.closing_owner=Some(identity);
            if registry.jobs.is_empty()&&!registry.jobs.terminal_is_empty(){registry.retiring_backing=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(std::mem::take(&mut registry.jobs)).unwrap_or_else(|(error,_)|panic!("original evaluation arena declaration refused: {error}")));}
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty}));
        }
    }
    Ok(RetainedCloneStep::Complete(empty))
}

/// 🧹️ Includes original removed keys and empty-but-allocated arena custody.
pub fn evaluation_retirement_pending(owner:&neural_engine::SharedRegistry)->bool{
    evaluation_jobs().lock().map_or(true,|registry|registry.closing_owner==Some(owner.owner_identity())||registry.admission_owner==Some(owner.owner_identity())||registry.jobs.values().any(|retained|retained.owner_identity==owner.owner_identity()&&retained.cancelled))
}

/// 📈️ Progress of the parked evaluation of `operator_id` at `node_hash`, if one is retained.
pub fn evaluation_progress(owner:&neural_engine::SharedRegistry,operator_id: &str, node_hash: u64) -> Option<neural_engine::OperatorProgress> {
    let registry=evaluation_jobs().lock().ok()?;
    registry.jobs.iter().find(|(key,retained)|key.0==owner.owner_identity()&&key.1==operator_id&&key.2==node_hash&&!retained.close_fields_terminal(false,false)).map(|(_,retained)|retained.progress())
}
// #endregion ⏱️EvaluationJobs
