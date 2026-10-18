//! 🎫️ Evaluate request, admission and drive of the wasm extension glue: the evaluate request and external result, the continuation identity, the request admission, the step budget constants and the cold envelope drive. See [`super`].

use super::*;

// #region 🔖️Evaluate
/// 📇️ The `evaluate` capability's request, as the wire declares it.
#[derive(FromValue,semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct EvaluateRequest {
    pub neuron_id:String,
    pub operator_id: String,
    pub input_json: String,
    /// 🪪️ The requester's own identity for this evaluation — the key a retained job is resumed by.
    /// Every key, including zero, retains the same original frontier across round trips;
    #[value(default)]
    pub node_hash: u64,
    #[value(default)]
    pub dependency_json: String,
    #[value(default)]
    pub operator_version: String,
    #[value(default)]
    pub cancellation_id: String,
    #[value(default)]
    pub window_id:String,
    #[value(default)]
    pub window_kind_id:String,
    #[value(default)]
    pub extension_id:String,
    #[value(default)]
    pub round_units: u64,
    #[value(default)]
    pub resume: bool,
    #[value(default)]
    pub external_result:Option<EvaluationExternalResult>,
}

/// 📬️ Completion names the exact pending extension owner and its original output source.
#[derive(FromValue,semio_framework_value::RetireOwned)]
#[value(rename_all="camelCase")]
pub struct EvaluationExternalResult {pub neuron_id:String,pub extension_id:String,pub operator_id:String,pub node_hash:u64,pub output:DslValue}

/// 🪪️ Retains only inline comparison metadata while the whole original request stays owned.
pub struct EvaluationContinuationIdentity{external:bool,pub(super) field:usize,pub(super) position:usize}

impl EvaluationContinuationIdentity{
 pub(super) fn new(external:bool)->Self{Self{external,field:0,position:0}}
 pub(super) fn step(&mut self,request:&EvaluateRequest,pending:Option<&neural_engine::PendingExtensionEval>,neuron:&str,version:&str,cancellation:&str,cancelled:bool,grant:RetainedCloneGrant)->Result<(Option<bool>,RetainedCloneProgress),ValueError>{
  let empty=RetainedCloneProgress::default();if grant.maximum_items==0{return Ok((None,empty))}if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original continuation identity requires admitted depth"))}
  let item=RetainedCloneProgress{copied_items:1,..empty};
  let pair:(&str,&str)=if self.external{
   let Some(result)=request.external_result.as_ref()else{return Ok((Some(false),item))};let Some(pending)=pending else{return Ok((Some(false),item))};
   if cancelled{return Ok((Some(false),item))}
   match self.field{0=>(&pending.neuron_id[..],&result.neuron_id[..]),1=>(&pending.extension_id[..],&result.extension_id[..]),2=>(&pending.operator_id[..],&result.operator_id[..]),3=>(neuron,&request.neuron_id),4=>(version,&request.operator_version),5=>(cancellation,&request.cancellation_id),6=>{if pending.node_hash!=result.node_hash{return Ok((Some(false),item))}self.field=7;return Ok((None,item))},_=>return Ok((Some(true),empty))}
  }else{
   if !request.input_json.is_empty()||!request.dependency_json.is_empty(){return Ok((Some(false),item))}if cancelled{return Ok((Some(true),item))}
   match self.field{0=>(neuron,&request.neuron_id),1=>(version,&request.operator_version),2=>(cancellation,&request.cancellation_id),_=>return Ok((Some(true),item))}
  };
  let(expected,actual)=(pair.0.as_bytes(),pair.1.as_bytes());if expected.len()!=actual.len(){return Ok((Some(false),item))}if self.position==expected.len(){self.field+=1;self.position=0;return Ok((None,item))}
  let count=grant.maximum_copy_bytes.min(64).min(expected.len()-self.position);if count==0{return Ok((None,empty))}let mut copied=[0u8;64];copied[..count].copy_from_slice(&expected[self.position..self.position+count]);let equal=copied[..count]==actual[self.position..self.position+count];self.position+=count;if equal&&self.position==expected.len(){self.field+=1;self.position=0;}Ok((if equal{None}else{Some(false)},RetainedCloneProgress{copied_bytes:count,..item}))
 }
}

/// 📥️ Keeps the original request while independently admitting its required metadata copies.
#[derive(semio_framework_value::RetireOwned)]
pub struct EvaluationRequestAdmission {pub(super) request:Option<EvaluateRequest>,copies:[Option<Vec<u8>>;2],pub(super) field:usize,pub(super) position:usize}

impl EvaluationRequestAdmission {
    pub(super) fn new(request:EvaluateRequest)->Self{Self{request:Some(request),copies:[None,None],field:0,position:0}}
    pub fn original_request_source_ptr(&self)->Option<*const u8>{self.request.as_ref().map(|request|request.input_json.as_ptr())}
    pub fn ready(&self)->bool{self.field==2}
    pub(super) fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{self.step_original(None,grant)}
    pub(super) fn step_with_operator(&mut self,operator:&str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{self.step_original(Some(operator),grant)}
    pub(super) fn step_original(&mut self,operator:Option<&str>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        let empty=RetainedCloneProgress::default();
        if self.ready()||grant.maximum_items==0{return Ok(empty);}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original request metadata requires admitted depth"));}
        let request=self.request.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original request admission lost its source"))?;
        let source=if self.field==0{if request.operator_id.is_empty(){operator.unwrap_or(&request.operator_id).as_bytes()}else{request.operator_id.as_bytes()}}else{request.operator_version.as_bytes()};
        if self.copies[self.field].is_none(){
            if grant.maximum_capacity_bytes<source.len(){return Ok(empty);}
            let mut bytes=Vec::new();bytes.try_reserve_exact(source.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original request metadata allocation failed"))?;
            let retained_capacity_bytes=bytes.capacity();self.copies[self.field]=Some(bytes);
            if source.is_empty(){self.field+=1;self.position=0;}
            return Ok(RetainedCloneProgress{copied_items:1,retained_capacity_bytes,..empty});
        }
        let bytes=grant.maximum_copy_bytes.min(source.len().saturating_sub(self.position));
        if bytes==0{return Ok(empty);}
        self.copies[self.field].as_mut().unwrap().extend_from_slice(&source[self.position..self.position+bytes]);self.position+=bytes;
        if self.position==source.len(){self.field+=1;self.position=0;}
        Ok(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..empty})
    }
    /// 🔒️ Transfers the complete bytes copied from original immutable UTF8 fields without rescanning.
    pub fn take_ready(&mut self)->Option<(EvaluateRequest,String,String)>{
        if !self.ready(){return None;}
        let request=self.request.take()?;let key=unsafe{String::from_utf8_unchecked(self.copies[0].take().unwrap())};let version=unsafe{String::from_utf8_unchecked(self.copies[1].take().unwrap())};Some((request,key,version))
    }
}

/// ⏱️ Units one `evaluate` STEP spends before the round trip re-checks its wall deadline — the
/// granularity at which a cancel can land, never the round trip's own bound. Mirrors
/// `TESSELLATE_STEP_BUDGET`'s reasoning: units are not time.
pub const EVALUATE_STEP_BUDGET: usize = 8;

/// ⌛️ Wall-clock allowance for one `evaluate` round trip when the requester names none. Chosen
/// well under the host shard watchdog's 16 s silence threshold AND under the 5 s ordinary
/// heartbeat window, so a worker driving a budgeted evaluation is never mistaken for a dead one
/// (`🎭️actor/📮️shard-client/🟦️.ts`'s `SHARD_LIVENESS_POLICY`).
pub const EVALUATE_STEP_WALL_MICROS: u64 = 2_000_000;

/// 🧊️ Explicit cold dependency identity policy; normal requests carry their own full grant.
pub(super) const COLD_EVALUATION_IDENTITY_POLICY:RetainedCloneGrant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:1048576,maximum_release_bytes:256*1024*1024,maximum_depth:1024};

/// 🧾️ The shape every `evaluate` answer takes. Declared here as one string so the Rust law, the
/// TypeScript twin and the fixture all name the same fields.
pub const EVALUATE_ENVELOPE_SCHEMA: &str = "{done,cancellable,phase,unitsDone,unitsTotal,output}";

/// ⏱️ One budgeted `evaluate` ROUND TRIP as the extension-boundary JSON envelope.
///
/// `budget` bounds ONE step in operator-defined units; `wall_micros` bounds the whole round trip:
/// the call keeps stepping while the job is still working AND the deadline has not passed. Both
/// are needed because units are not time.
pub fn evaluate_step_envelope_cold(registry:&neural_engine::SharedRegistry,request:EvaluateRequest,cx:&mut semio_framework_job::StepContext<'_>)->EvaluationReply {
    let identity=registry.owner_identity();let mut jobs=evaluation_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let existing=(0..jobs.jobs.slot_count()).find(|slot|jobs.jobs.slot_entry(*slot).is_some_and(|(key,_)|key.0==identity&&key.1==request.operator_id&&key.2==request.node_hash));
    evaluation_admit_original_request(registry,request,&mut jobs,existing,Default::default(),&mut None,cx)
}

pub(super) fn evaluation_admit_original_request(registry:&neural_engine::SharedRegistry,mut request:EvaluateRequest,jobs:&mut EvaluationJobRegistry,existing:Option<usize>,mut physical:RetainedCloneProgress,admitted_slot:&mut Option<usize>,cx:&mut semio_framework_job::StepContext<'_>)->EvaluationReply {
    let memory_grant=cx.retained_grant();let round_units=request.round_units;let identity=registry.owner_identity();
    let mut returned_request=None;
    let slot=if let Some(slot)=existing{
        let remaining=evaluation_remaining_grant(memory_grant,physical);if remaining.maximum_items==0||remaining.maximum_depth==0{return EvaluationReply::pending_request(request,physical)}physical.copied_items+=1;
        let (_,retained)=jobs.jobs.slot_entry_mut(slot).unwrap();
        if retained.response.is_some()||retained.compact_request.is_some(){returned_request=Some(request);}
        else if retained.close_fields_terminal(false,false){
            if request.resume||request.external_result.is_some(){return EvaluationReply::refused(request,physical,"evaluation continuation has no retained owner");}
            *retained=RetainedEvaluation::new(registry.clone(),request);
        }else if request.external_result.is_some()||request.resume{
            if memory_grant.maximum_items==0||memory_grant.maximum_depth==0{return EvaluationReply::pending_request(request,physical)}
            if request.external_result.is_some()&&retained.external_completion.is_some(){return EvaluationReply::refused(request,physical,"external completion remains owned by its previous frontier")}
            retained.continuation_identity=Some(EvaluationContinuationIdentity::new(request.external_result.is_some()));retained.compact_request=Some(request);retained.compact_cleanup=false;
        }else{
            if retained.discarded_request.is_some()||retained.discarded_operator.is_some()||retained.discarded_neuron.is_some()||retained.discarded_version.is_some()||retained.discarded_cancellation.is_some()||retained.discarded_candidate.is_some()||retained.discarded_preparation.is_some(){return EvaluationReply::refused(request,physical,"supersession remains owned by its previous physical frontier");}
            let replacement=EvaluationRequestAdmission::new(request);
            let active=retained.request_admission.is_some()||retained.admission.is_some()||retained.candidate.is_some()||retained.external_pending.is_some()||retained.external_completion.is_some()||retained.pending_wave.is_some()||retained.finished.is_some()||retained.output.is_some()||retained.cancelled;
            retained.discarded_request=retained.request_admission.replace(replacement);retained.request_initial=active;
            if active{retained.discarded_candidate=retained.candidate.take();retained.discarded_preparation=retained.admission.take();retained.cancel();retained.restarting=true;}
        }
        slot
    }else{
        if request.resume||request.external_result.is_some(){return EvaluationReply::refused(request,physical,"evaluation continuation has no retained owner");}
        if jobs.closing_owner.is_some()||jobs.jobs.len()>=EVALUATION_JOB_CAPACITY{return EvaluationReply::refused(request,physical,"evaluation capacity remains owned by pending work");}
        let key=(identity,std::mem::take(&mut request.operator_id),request.node_hash);let key_pointer=key.1.as_ptr();
        loop{
            let grant=evaluation_remaining_grant(memory_grant,physical);
            let capacity=match jobs.jobs.next_insert_capacity_byte_demand(&key,grant.maximum_copy_bytes){Ok(capacity)=>capacity,Err(error)=>{request.operator_id=key.1;return EvaluationReply::refused_original(request,physical,error.into());}};
            if capacity==0{break;}
            if grant.maximum_items==0||grant.maximum_capacity_bytes<capacity{request.operator_id=key.1;return EvaluationReply::pending_request(request,physical);}
            if jobs.jobs.is_empty(){jobs.admission_owner=Some(identity);}
            match jobs.jobs.reserve_insert_step(&key,grant){Ok(receipt)=>physical=physical.checked_add(receipt).expect("original SDK indexed backing receipt"),Err((error,receipt))=>{physical=physical.checked_add(receipt).expect("original SDK failed indexed backing receipt");request.operator_id=key.1;return EvaluationReply::refused_original(request,physical,error.with_retained_progress(receipt).into());}}
        }
        let grant=evaluation_remaining_grant(memory_grant,physical);
        let depth=match jobs.jobs.next_insert_depth_demand(&key){Ok(depth)=>depth,Err(error)=>{request.operator_id=key.1;return EvaluationReply::refused_original(request,physical,error.into());}};
        if grant.maximum_items==0||grant.maximum_depth<depth{request.operator_id=key.1;return EvaluationReply::pending_request(request,physical);}
        let retained=RetainedEvaluation::new(registry.clone(),request);
        match jobs.jobs.insert_reserved(key,retained,grant){Ok((None,receipt))=>{physical=physical.checked_add(receipt).expect("original SDK indexed row receipt");jobs.admission_owner=None;jobs.lookup_epoch=jobs.lookup_epoch.checked_add(1).expect("original registry key epoch");},Ok((Some(_),_))=>unreachable!("original vacant evaluation slot remains unique"),Err((error,key,mut retained))=>{let mut request=retained.request_admission.take().unwrap().request.take().unwrap();request.operator_id=key.1;let registry=retained.registry.take().unwrap();let _original_registry=registry;return EvaluationReply::refused_original(request,physical,error.into());}}
        (0..jobs.jobs.slot_count()).find(|slot|jobs.jobs.slot_entry(*slot).is_some_and(|(key,_)|key.0==identity&&key.1.as_ptr()==key_pointer)).unwrap()
    };
    *admitted_slot=Some(slot);evaluation_drive_original(registry,jobs,slot,memory_grant,round_units,physical,returned_request,cx)
}

pub(super) fn evaluation_drive_original(registry:&neural_engine::SharedRegistry,jobs:&mut EvaluationJobRegistry,slot:usize,memory_grant:RetainedCloneGrant,round_units:u64,mut physical:RetainedCloneProgress,returned_request:Option<EvaluateRequest>,cx:&mut semio_framework_job::StepContext<'_>)->EvaluationReply {
    let (key,retained)=jobs.jobs.slot_entry_mut(slot).unwrap();
    let mut spent=0u64;
    loop{
        if cx.should_yield(){return EvaluationReply{retirement_posted:false,external_pending:false,pending_source:None,wire:None,complete:false,faulted:false,retirement_progress:physical,returned_request,refusal:None}}
        let remaining=evaluation_remaining_grant(memory_grant,physical);
        if let Some(response)=&mut retained.response{
            if retained.response_failed{let result=response.close_step(remaining);let(receipt,refusal)=match result{Ok(step)=>(step.progress(),None),Err(error)=>(error.retained_progress(),Some(error))};physical=physical.checked_add(receipt).expect("failed original response cleanup receipt");if response.terminal_is_empty(){retained.response=None;retained.response_failed=false;}return EvaluationReply{retirement_posted:false,external_pending:false,pending_source:None,wire:None,complete:false,faulted:true,retirement_progress:physical,returned_request,refusal};}
            let result=response.step(1,retained.units,remaining);let receipt=response.normal_step_progress();physical=physical.checked_add(receipt).expect("original SDK response receipt");
            match result{Ok((Some(mut reply),_))=>{assert!(response.terminal_is_empty());retained.response=None;retained.compact_cleanup=true;reply.retirement_progress=physical;reply.returned_request=returned_request;return reply;},Ok((None,_))=>return EvaluationReply{retirement_posted:false,external_pending:false,pending_source:None,wire:None,complete:false,faulted:false,retirement_progress:physical,returned_request,refusal:None},Err(error)=>{retained.response_failed=true;retained.fault=Some(error);retained.cancel();return EvaluationReply{retirement_posted:false,external_pending:false,pending_source:None,wire:None,complete:false,faulted:true,retirement_progress:physical,returned_request,refusal:None};}}
        }
        if let Some(mut identity)=retained.continuation_identity.take(){
            let request=retained.compact_request.as_ref().expect("original continuation source");
            let result=identity.step(request,retained.external_pending.as_ref(),retained.current_neuron(),retained.current_version(),retained.current_cancellation(),retained.cancelled,remaining);
            let(status,receipt)=match result{Ok(step)=>step,Err(error)=>{retained.continuation_identity=Some(identity);return EvaluationReply{retirement_posted:false,external_pending:false,pending_source:None,wire:None,complete:false,faulted:true,retirement_progress:physical.checked_add(error.retained_progress()).expect("original failed identity receipt"),returned_request,refusal:Some(error.into())}}};
            physical=physical.checked_add(receipt).expect("original continuation identity receipt");
            match status{None=>retained.continuation_identity=Some(identity),Some(false)=>return EvaluationReply::refused(retained.compact_request.take().unwrap(),physical,"continuation does not match its original retained owner"),Some(true)=>{if identity.external{retained.continuation_identity=Some(identity);if remaining.maximum_items<=receipt.copied_items{return EvaluationReply{retirement_posted:false,external_pending:false,pending_source:None,wire:None,complete:false,faulted:false,retirement_progress:physical,returned_request,refusal:None}}retained.external_completion=Some(EvaluationExternalCompletion::new(retained.compact_request.as_mut().unwrap().external_result.take().unwrap()));retained.compact_cleanup=true;retained.continuation_identity=None;physical.copied_items+=1;}}}
            return EvaluationReply{retirement_posted:false,external_pending:false,pending_source:None,wire:None,complete:false,faulted:false,retirement_progress:physical,returned_request,refusal:None}
        }
        if retained.compact_cleanup&&(retained.compact_request.is_some()||retained.compact_active.is_some()){
            let result=if retained.compact_active.is_some(){evaluation_child_close(&mut retained.compact_active,remaining)}else{evaluation_admit_original(&mut retained.compact_request,&mut retained.compact_active,remaining)};
            match result{Ok(step)=>physical=physical.checked_add(step.progress()).expect("original SDK compact source receipt"),Err(error)=>{physical=physical.checked_add(error.retained_progress()).expect("original SDK failed compact source receipt");retained.fault=Some(error.into());retained.cancel();}}
            if retained.compact_request.is_none()&&retained.compact_active.is_none(){retained.compact_cleanup=false;}return EvaluationReply{retirement_posted:false,external_pending:false,pending_source:None,wire:None,complete:false,faulted:false,retirement_progress:physical,returned_request,refusal:None};
        }
        let budget=usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX);let units=if round_units==0{budget}else{budget.min(round_units.saturating_sub(spent)as usize)};spent=spent.saturating_add(units as u64);
        if physical!=Default::default(){return EvaluationReply{retirement_posted:false,external_pending:false,pending_source:None,wire:None,complete:false,faulted:false,retirement_progress:physical,returned_request,refusal:None}}
        let before_units=retained.units;let restarting=retained.restarting;let (step,receipt,failed)=match retained.advance(registry,&key.1,cx){Ok((step,receipt))=>(step,receipt,false),Err(error)=>{let receipt=retained.normal_step_progress();retained.fault=Some(error);retained.cancel();(EvaluationStep::Working(retained.progress()),receipt,true)}};
        if !failed&&!retained.normal_context_posted&&receipt==Default::default()&&retained.units==before_units{return EvaluationReply{retirement_posted:false,external_pending:false,pending_source:None,wire:None,complete:false,faulted:false,retirement_progress:physical,returned_request,refusal:None}}
        let work=if restarting&&!retained.restarting&&retained.units==0{receipt.copied_items.max(1)}else{retained.units.checked_sub(before_units).expect("original evaluation unit progress changes only at explicit restart").max(receipt.copied_items).max(1)};if !retained.normal_context_posted{cx.consume_fuel(work as u64);}
        physical=physical.checked_add(receipt).expect("original evaluation cumulative physical receipt");
        let funded=receipt.fits(remaining)&&physical.fits(memory_grant);if !funded{if retained.fault.is_none(){retained.fault=Some("original evaluation exceeded its caller memory grant".into());}retained.cancel();}
        match step{
            EvaluationStep::Fault(error)=>return EvaluationReply{retirement_posted:retained.normal_context_posted,external_pending:false,pending_source:None,wire:None,complete:false,faulted:true,retirement_progress:physical,returned_request,refusal:Some(error)},
            EvaluationStep::Done(mut reply)=>{reply.retirement_posted=retained.normal_context_posted;reply.retirement_progress=physical;reply.returned_request=returned_request;if reply.complete{retained.cancelled=true;}return reply;},
            EvaluationStep::Cancelled(progress)=>retained.response=Some(EvaluationOutput::envelope(EvaluationEnvelopeSource{output:None,done:true,cancellable:false,phase:"cancelled",units_done:progress.units_done,units_total:progress.units_total,pending:None},false)),
            EvaluationStep::Working(progress)=>{
                let expired=cx.should_yield();
                let _=(funded,expired,spent);
                retained.response=Some(EvaluationOutput::envelope(EvaluationEnvelopeSource{output:None,done:false,cancellable:true,phase:progress.phase,units_done:progress.units_done,units_total:progress.units_total,pending:None},false));
            },
        }
        return EvaluationReply{retirement_posted:retained.normal_context_posted,external_pending:false,pending_source:None,wire:None,complete:false,faulted:false,retirement_progress:physical,returned_request,refusal:None};
    }
}

/// 🎟️ Keeps each original round-trip currency after its actual physical effects.
pub fn evaluation_remaining_grant(memory_grant:RetainedCloneGrant,physical:RetainedCloneProgress)->RetainedCloneGrant {
    RetainedCloneGrant{maximum_items:memory_grant.maximum_items.saturating_sub(physical.copied_items),maximum_copy_bytes:memory_grant.maximum_copy_bytes.saturating_sub(physical.copied_bytes),maximum_capacity_bytes:memory_grant.maximum_capacity_bytes.saturating_sub(physical.retained_capacity_bytes),maximum_release_bytes:memory_grant.maximum_release_bytes.saturating_sub(physical.released_bytes),..memory_grant}
}
// #endregion 🔖️Evaluate
