//! 🔌️ Shared wasm extension glue for flow modules.

use neural_engine::{inject_channel_defaults, ColdOwner, Dictionary, Registry};
use semio_framework_replication as protocol;
use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_framework_value::{ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retirement::RetireOwned,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

pub use semio_framework_artifact_flow_flow::extensions::*;

/// 🧬️ Canonical source and wiring ancestry of one geometry inference, excluding presentation data.
pub fn flow_inference_dependency_json(snapshot: &semio_framework_artifact_flow_flow::FlowHostSnapshot, neuron_id: &str) -> String {
    use semio_framework_artifact_flow_flow::Widget;
    use std::collections::BTreeMap;
    let mut ids = std::collections::BTreeSet::new();
    let mut pending = vec![neuron_id];
    while let Some(id) = pending.pop() {
        if !ids.insert(id) { continue; }
        pending.extend(snapshot.synapses.iter().filter(|edge| edge.to == id).map(|edge| edge.from.as_str()));
    }
    let widgets: BTreeMap<_, _> = snapshot.widgets.iter().filter(|widget| ids.contains(semio_framework_artifact_flow_flow::widget_id_for(widget))).map(|widget| {
        let value = match widget {
            Widget::Neuron { neuron_kind, params, .. } => DslValue::object([("operatorId".into(), neuron_kind.to_value()), ("params".into(), params.to_value())]),
            Widget::InputSlider { value, .. } => DslValue::object([("number".into(), value.to_value())]),
            Widget::InputNote { text, .. } => DslValue::object([("text".into(), text.to_value())]),
            Widget::InputImage { src, .. } => DslValue::object([("source".into(), src.to_value())]),
            Widget::Variable { name, schema, .. } => DslValue::object([("name".into(), name.to_value()), ("schema".into(), schema.to_value())]),
            Widget::Cluster { tree, .. } => tree.to_value(),
            _ => DslValue::Null,
        };
        (semio_framework_artifact_flow_flow::widget_id_for(widget).to_string(), value)
    }).collect();
    let synapses: BTreeMap<_, _> = snapshot.synapses.iter().filter(|edge| ids.contains(edge.to.as_str())).map(|edge| (edge.id.clone(), edge.to_value())).collect();
    semio_framework_pack_json::to_json_string(&DslValue::object([
        ("widgets".into(), DslValue::object(widgets)), ("synapses".into(), DslValue::object(synapses)),
    ]))
}

// #region 🔖️Evaluate
/// 🧮️ Evaluates an operator and returns JSON dictionary or `{ "error": ... }`.
pub fn evaluate_json(registry: &Registry, kind_id: &str, input_json: &str) -> String {
    let input: Dictionary = match semio_framework_pack_json::from_json_str(input_json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        Ok(d) => d,
        Err(err) => return semio_framework_pack_json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    };
    let input = ColdOwner::new(match registry.operator_info(kind_id) {
        Some(info) => inject_channel_defaults(input, info),
        None => input,
    });
    match registry.dispatch(kind_id, &input) {
        Ok(out) => semio_framework_pack_json::to_json_string(&*ColdOwner::new(out)),
        Err(err) => semio_framework_pack_json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    }
}

/// 🧮️ Evaluates a neural tree as a function and returns the out dictionary JSON or `{ "error": ... }`.
pub fn evaluate_function_json(registry: &Registry, tree_json: &str, in_dict_json: &str) -> String {
    let tree: neural_engine::Tree = match semio_framework_pack_json::from_json_str(tree_json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        Ok(tree) => tree,
        Err(err) => return semio_framework_pack_json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    };
    let tree = ColdOwner::new(tree);
    let in_dict: Dictionary = match semio_framework_pack_json::from_json_str(in_dict_json, semio_framework_pack_json::JsonMemberPolicy::Reject) {
        Ok(dict) => dict,
        Err(err) => return semio_framework_pack_json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    };
    let evaluator = neural_engine::Evaluator::new(registry);
    let in_dict = ColdOwner::new(in_dict);
    match evaluator.evaluate_function(&tree, &in_dict) {
        Ok(out) => semio_framework_pack_json::to_json_string(&*ColdOwner::new(out)),
        Err(err) => semio_framework_pack_json::to_json_string(&DslValue::object([("error".to_string(), err.to_string().to_value())])),
    }
}
/// 🔀️ Decodes one original extension evaluation request with its mandatory retained policy.
/// ⏱️ Every request key retains the same original Registry frontier across round trips.
/// 🧾️ Each round trip debits actual physical effects once before reporting progress or completion.
#[path="📥️request/🦀️.rs"]
mod request;
pub use request::{EvaluationInvokeCursor,EvaluationRequestContextInput,evaluate_invoke_json};

/// 📇️ The `evaluate` capability's request, as the wire declares it.
#[derive(FromValue,semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct EvaluateRequest {
    pub retained:RetainedCloneGrant,
    pub neuron_id:String,
    pub operator_id: String,
    pub input_json: String,
    /// 🪪️ The requester's own identity for this evaluation — the key a retained job is resumed by.
    /// Every key, including zero, retains the same original frontier across round trips;
    #[value(default)]
    pub node_hash: u64,
    /// ⏱️ Units ONE step may spend before the round trip re-checks its wall deadline.
    #[value(default)]
    pub budget: u64,
    /// ⌛️ Wall-clock allowance for the WHOLE round trip, in microseconds.
    #[value(default)]
    pub wall_micros: u64,
    #[value(default)]
    pub dependency_json: String,
    #[value(default)]
    pub operator_version: String,
    #[value(default)]
    pub cancellation_id: String,
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
pub struct EvaluationExternalResult {pub neuron_id:String,pub extension_id:String,pub operator_id:String,pub node_hash:u64,pub output_json:String}
#[path="📨️result/🦀️.rs"] mod external_completion;
use external_completion::EvaluationExternalCompletion;

/// 🪪️ Retains only inline comparison metadata while the whole original request stays owned.
struct EvaluationContinuationIdentity{external:bool,field:usize,position:usize}
impl EvaluationContinuationIdentity{
 fn new(external:bool)->Self{Self{external,field:0,position:0}}
 fn step(&mut self,request:&EvaluateRequest,pending:Option<&neural_engine::PendingExtensionEval>,neuron:&str,version:&str,cancellation:&str,cancelled:bool,grant:RetainedCloneGrant)->Result<(Option<bool>,RetainedCloneProgress),ValueError>{
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
struct EvaluationRequestAdmission {request:Option<EvaluateRequest>,copies:[Option<Vec<u8>>;2],field:usize,position:usize}
impl EvaluationRequestAdmission {
    fn new(request:EvaluateRequest)->Self{Self{request:Some(request),copies:[None,None],field:0,position:0}}
    fn original_request_source_ptr(&self)->Option<*const u8>{self.request.as_ref().map(|request|request.input_json.as_ptr())}
    fn ready(&self)->bool{self.field==2}
    fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{self.step_original(None,grant)}
    fn step_with_operator(&mut self,operator:&str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{self.step_original(Some(operator),grant)}
    fn step_original(&mut self,operator:Option<&str>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
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
    fn take_ready(&mut self)->Option<(EvaluateRequest,String,String)>{
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
const COLD_EVALUATION_IDENTITY_POLICY:RetainedCloneGrant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:1048576,maximum_release_bytes:256*1024*1024,maximum_depth:1024};

/// 🧾️ The shape every `evaluate` answer takes. Declared here as one string so the Rust law, the
/// TypeScript twin and the fixture all name the same fields.
pub const EVALUATE_ENVELOPE_SCHEMA: &str = "{done,cancellable,phase,unitsDone,unitsTotal,outputJson}";

/// ⏱️ One budgeted `evaluate` ROUND TRIP as the extension-boundary JSON envelope.
///
/// `budget` bounds ONE step in operator-defined units; `wall_micros` bounds the whole round trip:
/// the call keeps stepping while the job is still working AND the deadline has not passed. Both
/// are needed because units are not time.
pub fn evaluate_step_envelope_cold(registry:&neural_engine::SharedRegistry,request:EvaluateRequest)->EvaluationReply {
    let identity=registry.owner_identity();let mut jobs=evaluation_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let existing=(0..jobs.jobs.slot_count()).find(|slot|jobs.jobs.slot_entry(*slot).is_some_and(|(key,_)|key.0==identity&&key.1==request.operator_id&&key.2==request.node_hash));
    evaluation_admit_original_request(registry,request,&mut jobs,existing,Default::default(),&mut None)
}

fn evaluation_admit_original_request(registry:&neural_engine::SharedRegistry,mut request:EvaluateRequest,jobs:&mut EvaluationJobRegistry,existing:Option<usize>,mut physical:RetainedCloneProgress,admitted_slot:&mut Option<usize>)->EvaluationReply {
    let memory_grant=request.retained;let round_units=request.round_units;
    let budget=if request.budget==0{EVALUATE_STEP_BUDGET}else{request.budget as usize};let wall=if request.wall_micros==0{EVALUATE_STEP_WALL_MICROS}else{request.wall_micros};let identity=registry.owner_identity();
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
    *admitted_slot=Some(slot);evaluation_drive_original(registry,jobs,slot,memory_grant,budget,wall,round_units,physical,returned_request)
}

fn evaluation_drive_original(registry:&neural_engine::SharedRegistry,jobs:&mut EvaluationJobRegistry,slot:usize,memory_grant:RetainedCloneGrant,budget:usize,wall:u64,round_units:u64,mut physical:RetainedCloneProgress,returned_request:Option<EvaluateRequest>)->EvaluationReply {
    let (key,retained)=jobs.jobs.slot_entry_mut(slot).unwrap();
    let deadline=semio_framework_job::default_now_us().map(|now|now.saturating_add(wall));let mut spent=0u64;
    loop{
        let remaining=evaluation_remaining_grant(memory_grant,physical);
        if let Some(response)=&mut retained.response{
            if retained.response_failed{let result=response.close_step(remaining);let(receipt,refusal)=match result{Ok(step)=>(step.progress(),None),Err(error)=>(error.retained_progress(),Some(EvaluationFailure::Native(error)))};physical=physical.checked_add(receipt).expect("failed original response cleanup receipt");if response.terminal_is_empty(){retained.response=None;retained.response_failed=false;}return EvaluationReply{external_pending:false,pending_source:None,wire:None,complete:false,faulted:true,retirement_progress:physical,returned_request,refusal};}
            let result=response.step(1,retained.units,remaining);let receipt=response.normal_step_progress();physical=physical.checked_add(receipt).expect("original SDK response receipt");
            match result{Ok((Some(mut reply),_))=>{assert!(response.terminal_is_empty());retained.response=None;retained.compact_cleanup=true;reply.retirement_progress=physical;reply.returned_request=returned_request;return reply;},Ok((None,_))=>return EvaluationReply{external_pending:false,pending_source:None,wire:None,complete:false,faulted:false,retirement_progress:physical,returned_request,refusal:None},Err(error)=>{retained.response_failed=true;retained.fault=Some(error);retained.cancel();return EvaluationReply{external_pending:false,pending_source:None,wire:None,complete:false,faulted:true,retirement_progress:physical,returned_request,refusal:None};}}
        }
        if let Some(mut identity)=retained.continuation_identity.take(){
            let request=retained.compact_request.as_ref().expect("original continuation source");
            let result=identity.step(request,retained.external_pending.as_ref(),retained.current_neuron(),retained.current_version(),retained.current_cancellation(),retained.cancelled,remaining);
            let(status,receipt)=match result{Ok(step)=>step,Err(error)=>{retained.continuation_identity=Some(identity);return EvaluationReply{external_pending:false,pending_source:None,wire:None,complete:false,faulted:true,retirement_progress:physical.checked_add(error.retained_progress()).expect("original failed identity receipt"),returned_request,refusal:Some(error.into())}}};
            physical=physical.checked_add(receipt).expect("original continuation identity receipt");
            match status{None=>retained.continuation_identity=Some(identity),Some(false)=>return EvaluationReply::refused(retained.compact_request.take().unwrap(),physical,"continuation does not match its original retained owner"),Some(true)=>{if identity.external{retained.continuation_identity=Some(identity);if remaining.maximum_items<=receipt.copied_items{return EvaluationReply{external_pending:false,pending_source:None,wire:None,complete:false,faulted:false,retirement_progress:physical,returned_request,refusal:None}}retained.external_completion=Some(EvaluationExternalCompletion::new(retained.compact_request.as_mut().unwrap().external_result.take().unwrap()));retained.compact_cleanup=true;retained.continuation_identity=None;physical.copied_items+=1;}}}
            return EvaluationReply{external_pending:false,pending_source:None,wire:None,complete:false,faulted:false,retirement_progress:physical,returned_request,refusal:None}
        }
        if retained.compact_cleanup&&(retained.compact_request.is_some()||retained.compact_active.is_some()){
            let result=if retained.compact_active.is_some(){evaluation_child_close(&mut retained.compact_active,remaining)}else{evaluation_admit_original(&mut retained.compact_request,&mut retained.compact_active,remaining)};
            match result{Ok(step)=>physical=physical.checked_add(step.progress()).expect("original SDK compact source receipt"),Err(error)=>{physical=physical.checked_add(error.retained_progress()).expect("original SDK failed compact source receipt");retained.fault=Some(error.into());retained.cancel();}}
            if retained.compact_request.is_none()&&retained.compact_active.is_none(){retained.compact_cleanup=false;}return EvaluationReply{external_pending:false,pending_source:None,wire:None,complete:false,faulted:false,retirement_progress:physical,returned_request,refusal:None};
        }
        let units=if round_units==0{budget}else{budget.min(round_units.saturating_sub(spent)as usize)};spent=spent.saturating_add(units as u64);
        let (step,receipt)=match retained.advance(registry,&key.1,units,4096,remaining){Ok(step)=>step,Err(error)=>{let receipt=retained.normal_step_progress();retained.fault=Some(error);retained.cancel();(EvaluationStep::Working(retained.progress()),receipt)}};
        physical=physical.checked_add(receipt).expect("original evaluation cumulative physical receipt");
        let funded=receipt.fits(remaining)&&physical.fits(memory_grant);if !funded{if retained.fault.is_none(){retained.fault=Some("original evaluation exceeded its caller memory grant".into());}retained.cancel();}
        match step{
            EvaluationStep::Fault(error)=>retained.response=Some(EvaluationOutput::fault(error,retained.units)),
            EvaluationStep::Done(mut reply)=>{reply.retirement_progress=physical;reply.returned_request=returned_request;if reply.complete{retained.cancelled=true;}return reply;},
            EvaluationStep::Cancelled(progress)=>retained.response=Some(EvaluationOutput::envelope(EvaluationEnvelopeSource{output_json:String::new(),done:true,cancellable:false,phase:"cancelled",units_done:progress.units_done,units_total:progress.units_total,pending:None,fault:None},false)),
            EvaluationStep::Working(progress)=>{
                let expired=deadline.is_none_or(|deadline|semio_framework_job::default_now_us().is_none_or(|now|now>=deadline));
                if funded&&receipt!=RetainedCloneProgress::default()&&physical.copied_items<memory_grant.maximum_items&&!expired&&(round_units==0||spent<round_units){continue;}
                retained.response=Some(EvaluationOutput::envelope(EvaluationEnvelopeSource{output_json:String::new(),done:false,cancellable:true,phase:progress.phase,units_done:progress.units_done,units_total:progress.units_total,pending:None,fault:None},false));
            },
        }
    }
}
/// 🎟️ Keeps each original round-trip currency after its actual physical effects.
fn evaluation_remaining_grant(memory_grant:RetainedCloneGrant,physical:RetainedCloneProgress)->RetainedCloneGrant {
    RetainedCloneGrant{maximum_items:memory_grant.maximum_items.saturating_sub(physical.copied_items),maximum_copy_bytes:memory_grant.maximum_copy_bytes.saturating_sub(physical.copied_bytes),maximum_capacity_bytes:memory_grant.maximum_capacity_bytes.saturating_sub(physical.retained_capacity_bytes),maximum_release_bytes:memory_grant.maximum_release_bytes.saturating_sub(physical.released_bytes),..memory_grant}
}

/// 🧾️ Publishes the original physical envelope with facts from its same execution owner.
pub struct EvaluationReply {pub external_pending:bool,pending_source:Option<neural_engine::PendingExtensionEval>,pub wire:Option<String>,pub complete:bool,pub faulted:bool,pub retirement_progress:RetainedCloneProgress,pub returned_request:Option<EvaluateRequest>,pub refusal:Option<EvaluationFailure>}
impl EvaluationReply{
    fn pending_request(request:EvaluateRequest,retirement_progress:RetainedCloneProgress)->Self{Self{external_pending:false,pending_source:None,wire:None,complete:false,faulted:false,retirement_progress,returned_request:Some(request),refusal:None}}
    fn refused(request:EvaluateRequest,retirement_progress:RetainedCloneProgress,message:&'static str)->Self{Self::refused_original(request,retirement_progress,EvaluationFailure::Literal(message))}
    fn refused_original(request:EvaluateRequest,retirement_progress:RetainedCloneProgress,refusal:EvaluationFailure)->Self{Self{external_pending:false,pending_source:None,wire:None,complete:false,faulted:true,retirement_progress,returned_request:Some(request),refusal:Some(refusal)}}
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

/// 💥️ The `{"error": …}` output body a refused evaluation carries — the SAME shape
/// [`evaluate_json`] has always produced, so a budgeted failure and a one-shot failure are
/// indistinguishable to the requester's node cache.
fn error_output_json(message: &str) -> String {
    semio_framework_pack_json::to_json_string(&DslValue::object([("error".to_string(), message.to_value())]))
}
// #endregion 🔖️Evaluate

// #region ⏱️EvaluationJobs

fn evaluation_value_demands(owner:&neural_engine::ValueRetirement,copy:usize)->Result<RetirementDemand,ValueError>{
    Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})
}

fn evaluation_owned_birth<T:RetireOwned>()->RetirementDemand{
    RetirementDemand{capacity_bytes:semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(),depth:1,..Default::default()}
}

fn evaluation_admit_original<T:RetireOwned>(original:&mut Option<T>,active:&mut Option<Box<dyn ErasedSnapshotRetirement>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
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

fn evaluation_child_demands(active:&Option<Box<dyn ErasedSnapshotRetirement>>,copy:usize)->Result<RetirementDemand,ValueError>{
    active.as_ref().map_or(Ok(Default::default()),|owner|semio_framework_value::retirement::factory::factory_ticket_demands(owner,copy))
}

fn evaluation_child_close(active:&mut Option<Box<dyn ErasedSnapshotRetirement>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
    semio_framework_value::retirement::factory::close_factory_ticket(active,grant)
}

/// ⚠️ Keeps each original typed failure without allocating display text during a normal turn.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum EvaluationFailure {Native(ValueError),Json(semio_framework_pack_json::JsonError),Operator(neural_engine::EvalError),Literal(&'static str)}
impl From<ValueError> for EvaluationFailure {fn from(error:ValueError)->Self{Self::Native(error)}}
impl From<semio_framework_pack_json::JsonError> for EvaluationFailure {fn from(error:semio_framework_pack_json::JsonError)->Self{Self::Json(error)}}
impl From<neural_engine::EvalError> for EvaluationFailure {fn from(error:neural_engine::EvalError)->Self{Self::Operator(error)}}
impl From<&'static str> for EvaluationFailure {fn from(message:&'static str)->Self{Self::Literal(message)}}
impl EvaluationFailure{
    fn wire_fields(&self)->usize{match self{Self::Literal(_)|Self::Native(_)=>3,Self::Operator(neural_engine::EvalError::CycleDetected)=>2,Self::Operator(neural_engine::EvalError::PendingExtension{..})=>5,Self::Operator(neural_engine::EvalError::Retained(_))=>4,Self::Operator(_)=>3,Self::Json(error)=>match error{semio_framework_pack_json::JsonError::Native(_)|semio_framework_pack_json::JsonError::UnexpectedByte{..}|semio_framework_pack_json::JsonError::ControlCharacterInString{..}|semio_framework_pack_json::JsonError::DuplicateMember{..}=>4,semio_framework_pack_json::JsonError::UnexpectedEof|semio_framework_pack_json::JsonError::InvalidUtf8=>2,_=>3}}}
    fn wire_entry(&self,index:usize)->Result<(&str,semio_framework_pack_json::JsonWriteNode<'_>),ValueError>{
        use semio_framework_pack_json::{JsonError,JsonWriteNode as Node};use neural_engine::EvalError;
        let invalid=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original typed evaluation fault path is absent");
        if index==0{return Ok(("source",Node::String(match self{Self::Native(_)=>"native",Self::Json(_)=>"json",Self::Operator(_)=>"operator",Self::Literal(_)=>"literal"})));}
        match self{
            Self::Literal(message)=>match index{1=>Ok(("kind",Node::String("literal"))),2=>Ok(("message",Node::String(message))),_=>Err(invalid())},
            Self::Native(error)=>match index{1=>Ok(("kind",Node::String(error.kind.as_str()))),2=>Ok(("message",Node::String(&error.message))),_=>Err(invalid())},
            Self::Operator(error)=>{let kind=match error{EvalError::Retained(_)=>"retained",EvalError::UnknownKind(_)=>"unknownKind",EvalError::MissingInput(_)=>"missingInput",EvalError::InvalidInput(_)=>"invalidInput",EvalError::CardinalityViolation(_)=>"cardinalityViolation",EvalError::HeterogeneousList(_)=>"heterogeneousList",EvalError::CycleDetected=>"cycleDetected",EvalError::PendingExtension{..}=>"pendingExtension"};if index==1{return Ok(("kind",Node::String(kind)));}match error{EvalError::Retained(error)=>match index{2=>Ok(("refusalKind",Node::String(error.kind.as_str()))),3=>Ok(("message",Node::String(&error.message))),_=>Err(invalid())},EvalError::UnknownKind(message)|EvalError::MissingInput(message)|EvalError::InvalidInput(message)|EvalError::CardinalityViolation(message)|EvalError::HeterogeneousList(message)if index==2=>Ok(("message",Node::String(message))),EvalError::PendingExtension{extension_id,operator_id,node_hash}=>match index{2=>Ok(("extensionId",Node::String(extension_id))),3=>Ok(("operatorId",Node::String(operator_id))),4=>Ok(("nodeHash",Node::Number(semio_framework_value::Number::UInt(*node_hash)))),_=>Err(invalid())},_=>Err(invalid())}},
            Self::Json(error)=>{let kind=match error{JsonError::Native(_)=>"native",JsonError::UnexpectedEof=>"unexpectedEof",JsonError::UnexpectedByte{..}=>"unexpectedByte",JsonError::InvalidNumber(_)=>"invalidNumber",JsonError::InvalidEscape(_)=>"invalidEscape",JsonError::InvalidUnicodeEscape(_)=>"invalidUnicodeEscape",JsonError::UnpairedSurrogate(_)=>"unpairedSurrogate",JsonError::ControlCharacterInString{..}=>"controlCharacterInString",JsonError::InvalidUtf8=>"invalidUtf8",JsonError::TrailingData(_)=>"trailingData",JsonError::MaxDepthExceeded(_)=>"maxDepthExceeded",JsonError::DuplicateMember{..}=>"duplicateMember"};if index==1{return Ok(("kind",Node::String(kind)));}let number=|key,value|Ok((key,Node::Number(semio_framework_value::Number::UInt(value))));match error{JsonError::Native(error)=>match index{2=>Ok(("refusalKind",Node::String(error.kind.as_str()))),3=>Ok(("message",Node::String(&error.message))),_=>Err(invalid())},JsonError::UnexpectedByte{found,offset}|JsonError::ControlCharacterInString{byte:found,offset}=>match index{2=>number("offset",*offset as u64),3=>number("byte",u64::from(*found)),_=>Err(invalid())},JsonError::DuplicateMember{name,offset}=>match index{2=>number("offset",*offset as u64),3=>Ok(("member",Node::String(name))),_=>Err(invalid())},JsonError::InvalidNumber(offset)|JsonError::InvalidEscape(offset)|JsonError::InvalidUnicodeEscape(offset)|JsonError::UnpairedSurrogate(offset)|JsonError::TrailingData(offset)if index==2=>number("offset",*offset as u64),JsonError::MaxDepthExceeded(depth)if index==2=>number("depth",u64::from(*depth)),_=>Err(invalid())}}
        }
    }
    fn wire_node(&self,index:usize)->Result<semio_framework_pack_json::JsonWriteNode<'_>,ValueError>{self.wire_entry(index).map(|(_,node)|node)}
    fn wire_key(&self,index:usize)->Result<&str,ValueError>{self.wire_entry(index).map(|(key,_)|key)}
}
impl std::fmt::Display for EvaluationFailure {fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Native(error)=>std::fmt::Display::fmt(error,formatter),Self::Json(error)=>std::fmt::Display::fmt(error,formatter),Self::Operator(error)=>std::fmt::Display::fmt(error,formatter),Self::Literal(message)=>formatter.write_str(message)}}}
impl std::error::Error for EvaluationFailure {}

/// 🎒️ Moves the existing canonical JSON and typed input owners through retained admission.
pub struct EvaluationInputPreparation {
    input:Option<String>,dependency:Option<String>,operator:Option<String>,version:Option<String>,
    parser:Option<semio_framework_pack_json::JsonParseCursor>,projection:Option<semio_framework_pack_json::JsonValueProjection>,
    writer:Option<semio_framework_pack_json::JsonWriteCursor<DslValue>>,encoding:Option<semio_framework_value::native_encoding::NativeEncodeContinuation>,decoding:Option<semio_framework_value::native_decoding::NativeDecodeContinuation>,
    binding:Option<neural_engine::retirement::RetainedDictionaryInput>,dictionary:Option<Dictionary>,
    canonical_input:Option<String>,canonical_dependency:Option<String>,identity:Option<String>,
    retirement:neural_engine::ValueRetirement,stage:u8,units:usize,cancelled:bool,closing_fields:u16,fault_member:Option<String>,normal_progress:RetainedCloneProgress,
    active:std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}


impl EvaluationInputPreparation {
    /// 🌱️ Takes one initial request's source allocation without scanning or copying its payload.
    pub fn new(input:String,dependency:String,operator:String,version:String)->Self {
        Self {input:Some(input),dependency:Some(dependency),operator:Some(operator),version:Some(version),parser:Some(semio_framework_pack_json::JsonParseCursor::new(semio_framework_pack_json::JsonMemberPolicy::Reject)),projection:None,writer:None,encoding:None,decoding:None,binding:None,dictionary:None,canonical_input:None,canonical_dependency:None,identity:None,active:std::mem::ManuallyDrop::new(None),retirement:Default::default(),stage:0,units:0,cancelled:false,closing_fields:0,fault_member:None,normal_progress:Default::default()}
    }
    /// 🪪️ Exposes the still-owned source allocation for ownership laws.
    pub fn source_ptr(&self)->Option<*const u8> {self.input.as_ref().map(|source|source.as_ptr())}
    /// 📍️ Reports this same candidate's retained input and identity frontier.
    pub fn progress(&self)->(usize,usize,&'static str) {
        let phase=if !self.retirement.terminal_is_empty()||self.active.is_some()||self.closing_fields!=0 {"input-retire"} else {match self.stage {0=>"input-parse",1=>"input-order",2=>"input-canonical",3=>"input-bind",4=>"dependency-parse",5=>"dependency-order",6=>"dependency-canonical",7=>"identity-canonical",_=>"input-retire"}};
        (self.units,self.units.saturating_add(usize::from(self.stage<9)),phase)
    }
    fn write_step(&mut self,grant:RetainedCloneGrant)->Result<(Option<String>,RetainedCloneProgress),EvaluationFailure> {
        let mut admitted=|_|true;
        let mut control=match self.encoding.take() {Some(receipt)=>semio_framework_value::NativeEncodeControl::resume(receipt,&mut admitted),None=>Ok(semio_framework_value::NativeEncodeControl::new_retained(&mut admitted))}.map_err(EvaluationFailure::from)?;
        let result=self.writer.as_mut().unwrap().step(1,&mut control,grant).map_err(EvaluationFailure::from);let progress=self.writer.as_ref().unwrap().normal_step_progress();self.normal_progress=progress;
        self.encoding=Some(control.pause().map_err(EvaluationFailure::from)?);result.map(|value|(value,progress))
    }
    fn close_intermediate(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        if grant.maximum_items==0{return Ok(Default::default());}
        if !self.retirement.terminal_is_empty(){return self.retirement.close_step(grant).map(|step|step.progress());}
        if self.active.is_some(){return evaluation_child_close(&mut self.active,grant).map(|step|step.progress());}
        if self.closing_fields==0{return Ok(Default::default());}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation intermediate handoff requires admitted depth"));}
        let field=self.closing_fields&self.closing_fields.wrapping_neg();
        let progress=match field{
            1=>evaluation_admit_original(&mut self.parser,&mut self.active,grant)?.progress(),
            2=>evaluation_admit_original(&mut self.projection,&mut self.active,grant)?.progress(),
            4=>evaluation_admit_original(&mut self.writer,&mut self.active,grant)?.progress(),
            8|16|128=>{
                let source=match field{8=>&mut self.input,16=>&mut self.dependency,_=>&mut self.fault_member};let value=source.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"evaluation intermediate source lost its field"))?;
                match self.retirement.text(value,grant){Ok(progress)=>progress,Err((error,value))=>{*source=Some(value);return Err(error);}}
            },
            32|64=>{
                if field==32{self.encoding=None;}else{self.decoding=None;}RetainedCloneProgress {copied_items:1,..Default::default()}
            },
            _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"evaluation input has an unknown original close field")),
        };
        if progress.copied_items!=0{self.closing_fields&=!field;}
        Ok(progress)
    }
    /// ⏱️ Advances the original codec authority or one separately granted physical cleanup turn.
    pub fn normal_step_progress(&self)->RetainedCloneProgress {self.normal_progress}
    pub fn step(&mut self,maximum_units:usize,maximum_bytes:usize,grant:RetainedCloneGrant)->Result<(Option<(Dictionary,String)>,RetainedCloneProgress),EvaluationFailure>{
        self.normal_progress=Default::default();let result=self.step_original(maximum_units,maximum_bytes,grant);if let Ok((_,progress))=&result {self.normal_progress=*progress;}result
    }
    fn step_original(&mut self,maximum_units:usize,maximum_bytes:usize,grant:RetainedCloneGrant)->Result<(Option<(Dictionary,String)>,RetainedCloneProgress),EvaluationFailure>{
        let empty=RetainedCloneProgress::default();
        if maximum_units==0||maximum_bytes==0||grant.maximum_items==0{return Ok((None,empty));}
        if self.cancelled{return Err("evaluation input admission canceled".into());}
        if !self.retirement.terminal_is_empty()||self.active.is_some()||self.closing_fields!=0{
            let receipt=self.close_intermediate(grant).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;self.units=self.units.saturating_add(receipt.copied_items);return Ok((None,receipt));
        }
        match self.stage{
            0|4=>{
                let source=if self.stage==0{self.input.as_deref().unwrap()}else{self.dependency.as_deref().unwrap()};
                if source.len()>16*1024*1024{return Err("evaluation source exceeds admission limit".into());}
                let mut accepted=|_|true;let mut control=match self.decoding.take(){Some(receipt)=>semio_framework_value::NativeDecodeControl::resume(receipt,&mut accepted),None=>Ok(semio_framework_value::NativeDecodeControl::new_retained(&mut accepted))}.map_err(EvaluationFailure::from)?;
                let parsed=self.parser.as_mut().unwrap().step(source,1,&mut control,grant);let progress=self.parser.as_ref().unwrap().normal_step_progress();self.normal_progress=progress;self.decoding=Some(control.pause().map_err(EvaluationFailure::from)?);
                match parsed{
                    Ok(Some(value))=>{self.projection=Some(semio_framework_pack_json::JsonValueProjection::new_ordered(value));self.closing_fields=1|if self.stage==0{8}else{16};self.stage+=1;},
                    Ok(None)=>{},
                    Err(error) if self.stage==4&&error.kind()==ValueRefusalKind::InvalidValue=>{self.canonical_dependency=self.dependency.take();self.closing_fields=1|64;self.stage=7;if let semio_framework_pack_json::JsonError::DuplicateMember{name,..}=error{self.fault_member=Some(name);self.closing_fields|=128;}},
                    Err(error)=>return Err(error.into()),
                }
                self.units=self.units.saturating_add(progress.copied_items);return Ok((None,progress));
            },
            1|5=>{
                let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::resume(self.decoding.take().unwrap(),&mut accepted).map_err(EvaluationFailure::from)?;
                let projected=self.projection.as_mut().unwrap().step(1,maximum_bytes,&mut control,grant);let progress=self.projection.as_ref().unwrap().normal_step_progress();self.normal_progress=progress;self.decoding=Some(control.pause().map_err(EvaluationFailure::from)?);
                if let Some(value)=projected.map_err(EvaluationFailure::from)?{self.writer=Some(semio_framework_pack_json::JsonWriteCursor::new(value));self.closing_fields=2|64;self.stage+=1;}
                self.units=self.units.saturating_add(progress.copied_items);return Ok((None,progress));
            },
            2|6=>{let (text,progress)=self.write_step(grant)?;if let Some(text)=text{
                if self.stage==2{self.canonical_input=Some(text);self.binding=Some(neural_engine::retirement::RetainedDictionaryInput::new(self.writer.as_mut().unwrap().take_source().unwrap()));self.stage=3;}
                else{self.canonical_dependency=Some(text);self.stage=7;}
                self.closing_fields=4|32;
            }self.units=self.units.saturating_add(progress.copied_items);return Ok((None,progress));},
            3=>{
                let binding=self.binding.as_mut().unwrap();let step=binding.step(grant).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;
                if !step.progress.fits(grant){return Err("evaluation input binding exceeded caller grant".into());}
                if let Some(dictionary)=step.dictionary{assert!(binding.terminal_is_empty());self.binding=None;self.dictionary=Some(dictionary);self.parser=Some(semio_framework_pack_json::JsonParseCursor::new(semio_framework_pack_json::JsonMemberPolicy::Reject));self.stage=4;}
                self.units=self.units.saturating_add(step.progress.copied_items);return Ok((None,step.progress));
            },
            7=>{
                if self.writer.is_none(){
                    let bytes=4*std::mem::size_of::<DslValue>();
                    if grant.maximum_items==0||grant.maximum_capacity_bytes<bytes{return Ok((None,empty));}
                    if grant.maximum_depth==0{return Err("evaluation identity birth exceeds admitted depth".into());}
                    let mut values=Vec::new();values.try_reserve_exact(4).map_err(|_|EvaluationFailure::Native(ValueError::literal(ValueRefusalKind::AllocationFailed,"evaluation identity allocation failed")))?;
                    values.extend([self.operator.take().unwrap(),self.canonical_input.take().unwrap(),self.canonical_dependency.take().unwrap(),self.version.take().unwrap()].map(DslValue::String));
                    let retained_capacity_bytes=values.capacity()*std::mem::size_of::<DslValue>();self.writer=Some(semio_framework_pack_json::JsonWriteCursor::new(DslValue::Array(values)));
                    return Ok((None,RetainedCloneProgress {copied_items:1,retained_capacity_bytes,..empty}));
                }
                let (text,progress)=self.write_step(grant)?;if let Some(text)=text{self.identity=Some(text);self.closing_fields=4|32;self.stage=8;}self.units=self.units.saturating_add(progress.copied_items);return Ok((None,progress));
            },
            8=>{self.stage=9;self.units=self.units.saturating_add(1);return Ok((Some((self.dictionary.take().unwrap(),self.identity.take().unwrap())),empty));},
            _=>return Ok((None,empty)),
        }
        self.units=self.units.saturating_add(1);Ok((None,empty))
    }
    /// 🛑️ Marks the current candidate for close without dropping a partial frontier.
    pub fn cancel(&mut self) {self.cancelled=true;if let Some(binding)=&mut self.binding {binding.cancel();}}
    /// 📏️ Borrows the next original field constructor or admitted child without moving it.
    pub fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError> {
        if !self.retirement.terminal_is_empty(){return evaluation_value_demands(&self.retirement,copy);}
        if self.active.is_some(){return evaluation_child_demands(&self.active,copy);}
        if let Some(binding)=&self.binding{return Ok(RetirementDemand {copy_bytes:binding.next_close_copy_byte_demand()?,capacity_bytes:binding.next_close_capacity_byte_demand(copy)?,release_bytes:binding.next_close_release_byte_demand()?,depth:binding.next_close_depth_demand()?});}
        if self.parser.is_some(){return Ok(evaluation_owned_birth::<semio_framework_pack_json::JsonParseCursor>());}
        if self.projection.is_some(){return Ok(evaluation_owned_birth::<semio_framework_pack_json::JsonValueProjection>());}
        if self.writer.is_some(){return Ok(evaluation_owned_birth::<semio_framework_pack_json::JsonWriteCursor<DslValue>>());}
        Ok(RetirementDemand {depth:usize::from(!self.terminal_is_empty()),..Default::default()})
    }
    /// ♻️ Preserves independent credits through original fields and separately admitted child shells.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        self.cancel();
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){self.stage=9;return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation input close exceeds supplied depth"));}
        if !self.retirement.terminal_is_empty(){let step=self.retirement.close_step(grant)?;return self.admit_close(grant,step);}
        if self.active.is_some(){let step=evaluation_child_close(&mut self.active,grant)?;return self.admit_close(grant,step);}
        if let Some(binding)=&mut self.binding {
            let step=binding.close_step(grant)?;
            let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,binding.terminal_is_empty(),"evaluation input binding")?;
            if binding.terminal_is_empty(){self.binding=None;}
            return self.admit_close(grant,step);
        }
        if self.parser.is_some(){return evaluation_admit_original(&mut self.parser,&mut self.active,grant);}
        if self.projection.is_some(){return evaluation_admit_original(&mut self.projection,&mut self.active,grant);}
        if self.writer.is_some(){return evaluation_admit_original(&mut self.writer,&mut self.active,grant);}
        if self.encoding.is_some()||self.decoding.is_some(){
            if self.encoding.is_some(){self.encoding=None;}else{self.decoding=None;}
            return self.admit_close(grant,RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty}));
        }
        let progress=if let Some(value)=self.dictionary.take(){
            match self.retirement.push_dictionary(value,grant){Ok(progress)=>progress,Err((error,value))=>{self.dictionary=Some(value);return Err(error);}}
        }else{
            let source=if self.input.is_some(){&mut self.input}else if self.dependency.is_some(){&mut self.dependency}else if self.operator.is_some(){&mut self.operator}else if self.version.is_some(){&mut self.version}else if self.canonical_input.is_some(){&mut self.canonical_input}else if self.canonical_dependency.is_some(){&mut self.canonical_dependency}else if self.identity.is_some(){&mut self.identity}else{&mut self.fault_member};
            let value=source.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"evaluation input close has no original field"))?;
            match self.retirement.text(value,grant){Ok(progress)=>progress,Err((error,value))=>{*source=Some(value);return Err(error);}}
        };
        self.admit_close(grant,RetainedCloneStep::Progress(progress))
    }
    fn admit_close(&self,grant:RetainedCloneGrant,step:RetainedCloneStep)->Result<RetainedCloneStep,ValueError>{
        let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,RetainedCloneStep::Progress(step.progress()),false,"evaluation input parent")?;
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }
    /// 🔒️ Includes both opaque scalar continuations and the admitted original child shell.
    pub fn terminal_is_empty(&self)->bool {self.input.is_none()&&self.dependency.is_none()&&self.operator.is_none()&&self.version.is_none()&&self.parser.is_none()&&self.projection.is_none()&&self.writer.is_none()&&self.binding.is_none()&&self.dictionary.is_none()&&self.canonical_input.is_none()&&self.canonical_dependency.is_none()&&self.identity.is_none()&&self.fault_member.is_none()&&self.encoding.is_none()&&self.decoding.is_none()&&self.active.is_none()&&self.retirement.terminal_is_empty()}
}
impl Drop for EvaluationInputPreparation {fn drop(&mut self) {assert!(std::thread::panicking()||self.terminal_is_empty(),"evaluation input candidate dropped before terminal-empty");}}
struct EvaluationInputRetirement(EvaluationInputPreparation);
impl semio_framework_value::retirement::RetirementCursor for EvaluationInputRetirement {
    fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep {
        use semio_framework_value::retirement::RetirementStep;
        match self.0.close_step(grant){Err(error)=>RetirementStep::Failure(error),Ok(RetainedCloneStep::Complete(progress)) if progress==RetainedCloneProgress::default()=>RetirementStep::Complete,Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>RetirementStep::Progress(progress)}
    }
    fn terminal_is_empty(&self)->bool {self.0.terminal_is_empty()}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.retirement_demands(0)?.copy_bytes)}
    fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.0.retirement_demands(copy).ok().map(|demand|demand.capacity_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.0.retirement_demands(0).ok().map(|demand|demand.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.retirement_demands(0)?.depth)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl RetireOwned for EvaluationInputPreparation {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {Box::new(EvaluationInputRetirement(self))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<EvaluationInputRetirement>())}
    fn controlled_retirement_supported()->bool{true}
}


/// ⏱️ Retained resumable evaluations keyed by `(operatorId, nodeHash)`. Bounded: a new job past the
/// ceiling refuses admission until an existing owner finishes retirement.
const EVALUATION_JOB_CAPACITY: usize = 16;

enum EvaluationStep {Fault(EvaluationFailure),Working(neural_engine::OperatorProgress),Done(EvaluationReply),Cancelled(neural_engine::OperatorProgress)}

/// 📤️ Borrows inline envelope fields while retaining the same original output string.
#[derive(semio_framework_value::RetireOwned)]
struct EvaluationEnvelopeSource {output_json:String,done:bool,cancellable:bool,phase:&'static str,units_done:usize,units_total:usize,pending:Option<neural_engine::PendingExtensionEval>,fault:Option<EvaluationFailure>}
impl semio_framework_pack_json::JsonWriteSource for EvaluationEnvelopeSource {
    fn node_at_path(&self,path:&[usize])->Result<semio_framework_pack_json::JsonWriteNode<'_>,ValueError>{
        use semio_framework_pack_json::JsonWriteNode as Node;
        match path{[]=>Ok(Node::Object(if self.pending.is_some()||self.fault.is_some(){7}else{6})),[0]=>Ok(Node::Bool(self.cancellable)),[1]=>Ok(Node::Bool(self.done)),[2]=>Ok(Node::String(&self.output_json)),[3]=>Ok(Node::String(self.phase)),[4]=>Ok(Node::Number(semio_framework_value::Number::UInt(self.units_done as u64))),[5]=>Ok(Node::Number(semio_framework_value::Number::UInt(self.units_total as u64))),[6]=>Ok(Node::Object(if let Some(fault)=&self.fault{fault.wire_fields()}else{5})),[6,index]=>{if let Some(fault)=&self.fault{return fault.wire_node(*index);}let original=self.pending.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original pending envelope is absent"))?;match index{0=>Ok(Node::String(&original.extension_id)),1=>Ok(Node::String(&original.input_json)),2=>Ok(Node::String(&original.neuron_id)),3=>Ok(Node::Number(semio_framework_value::Number::UInt(original.node_hash))),4=>Ok(Node::String(&original.operator_id)),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original pending envelope field is absent"))}},_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original evaluation envelope path is absent"))}
    }
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
        if path==[6]{if let Some(fault)=&self.fault{return fault.wire_key(index);}return ["extensionId","inputJson","neuronId","nodeHash","operatorId"].get(index).copied().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original pending envelope key is absent"));}
        if !path.is_empty(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original evaluation envelope key path is absent"));}
        ["cancellable","done","outputJson","phase","unitsDone","unitsTotal",if self.fault.is_some(){"fault"}else{"pending"}].get(index).copied().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original evaluation envelope key is absent"))
    }
}

struct EvaluationOutput {
    writer:Option<semio_framework_pack_json::JsonWriteCursor<Dictionary>>,envelope:Option<semio_framework_pack_json::JsonWriteCursor<EvaluationEnvelopeSource>>,
    encoding:Option<semio_framework_value::native_encoding::NativeEncodeContinuation>,retirement:neural_engine::ValueRetirement,
    pending_source:Option<neural_engine::PendingExtensionEval>,text:Option<String>,wire:Option<String>,stage:u8,units:usize,faulted:bool,complete:bool,external_pending:bool,closing_fields:u8,normal_progress:RetainedCloneProgress,
    active:std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}


impl EvaluationOutput {
    fn new(output:Dictionary)->Self {let faulted=output.get("error").is_some();Self {writer:Some(semio_framework_pack_json::JsonWriteCursor::new(output)),envelope:None,encoding:None,retirement:Default::default(),pending_source:None,text:None,wire:None,stage:0,units:0,faulted,complete:true,external_pending:false,closing_fields:0,normal_progress:Default::default(),active:std::mem::ManuallyDrop::new(None)}}
    fn envelope(source:EvaluationEnvelopeSource,faulted:bool)->Self{let complete=source.done;let external_pending=source.pending.is_some();Self{writer:None,envelope:Some(semio_framework_pack_json::JsonWriteCursor::new(source)),encoding:None,retirement:Default::default(),pending_source:None,text:None,wire:None,stage:2,units:0,faulted,complete,external_pending,closing_fields:0,normal_progress:Default::default(),active:std::mem::ManuallyDrop::new(None)}}
    fn pending(original:neural_engine::PendingExtensionEval)->Self{Self::envelope(EvaluationEnvelopeSource{output_json:String::new(),done:false,cancellable:true,phase:"external-invoke",units_done:0,units_total:0,pending:Some(original),fault:None},false)}
    fn fault(original:EvaluationFailure,units:usize)->Self{Self::envelope(EvaluationEnvelopeSource{output_json:String::new(),done:true,cancellable:false,phase:"fault",units_done:units,units_total:units,pending:None,fault:Some(original)},true)}
    fn phase(&self)->&'static str {if self.active.is_some()||self.closing_fields!=0{return "output-retire";}match self.stage {0=>if self.writer.as_ref().unwrap().progress().1 {"output-write"}else{"output-measure"},1=>"output-retire",2=>if self.envelope.as_ref().unwrap().progress().1 {"output-envelope-write"}else{"output-envelope-measure"},3=>"output-envelope-retire",_=>"complete"}}
    fn close_intermediate(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        if !self.retirement.terminal_is_empty(){return self.retirement.close_step(grant).map(|step|step.progress());}
        if self.active.is_some(){return evaluation_child_close(&mut self.active,grant).map(|step|step.progress());}
        if grant.maximum_items==0{return Ok(Default::default());}
        if self.closing_fields==0{return Ok(Default::default());}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation output handoff requires admitted depth"));}
        let field=self.closing_fields&self.closing_fields.wrapping_neg();
        let progress=match field{
            1=>evaluation_admit_original(&mut self.writer,&mut self.active,grant)?.progress(),
            2=>evaluation_admit_original(&mut self.envelope,&mut self.active,grant)?.progress(),
            4=>{self.encoding=None;RetainedCloneProgress {copied_items:1,..Default::default()}},
            _=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"evaluation output has an unknown original close field")),
        };
        if progress.copied_items!=0{self.closing_fields&=!field;}Ok(progress)
    }
    fn normal_step_progress(&self)->RetainedCloneProgress {self.normal_progress}
    fn step(&mut self,maximum_units:usize,base_units:usize,grant:RetainedCloneGrant)->Result<(Option<EvaluationReply>,RetainedCloneProgress),EvaluationFailure>{
        self.normal_progress=Default::default();let result=self.step_original(maximum_units,base_units,grant);if let Ok((_,progress))=&result {self.normal_progress=*progress;}result
    }
    fn step_original(&mut self,maximum_units:usize,base_units:usize,grant:RetainedCloneGrant)->Result<(Option<EvaluationReply>,RetainedCloneProgress),EvaluationFailure>{
        let empty=RetainedCloneProgress::default();
        if maximum_units==0||grant.maximum_items==0{return Ok((None,empty));}
        if self.active.is_some()||self.closing_fields!=0||!self.retirement.terminal_is_empty(){
            let receipt=self.close_intermediate(grant).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;self.units=self.units.saturating_add(receipt.copied_items);return Ok((None,receipt));
        }
        match self.stage{
            0|2=>{
                let mut admitted=|_|true;let mut control=match self.encoding.take(){Some(receipt)=>semio_framework_value::NativeEncodeControl::resume(receipt,&mut admitted),None=>Ok(semio_framework_value::NativeEncodeControl::new_retained(&mut admitted))}.map_err(EvaluationFailure::from)?;
                let result=if self.stage==0{self.writer.as_mut().unwrap().step(1,&mut control,grant)}else{self.envelope.as_mut().unwrap().step(1,&mut control,grant)};let progress=if self.stage==0{self.writer.as_ref().unwrap().normal_step_progress()}else{self.envelope.as_ref().unwrap().normal_step_progress()};self.normal_progress=progress;self.encoding=Some(control.pause().map_err(EvaluationFailure::from)?);
                if let Some(text)=result.map_err(EvaluationFailure::from)?{if self.stage==0{self.text=Some(text);self.closing_fields=1|4;self.stage=1;}else{self.wire=Some(text);if self.external_pending{let mut source=self.envelope.as_mut().unwrap().take_source().unwrap();self.pending_source=source.pending.take();assert!(source.output_json.capacity()==0&&source.fault.is_none());}self.closing_fields=2|4;self.stage=3;}}
                self.units=self.units.saturating_add(progress.copied_items);return Ok((None,progress));
            },
            1=>{
                if grant.maximum_depth==0{return Err("evaluation envelope handoff exceeds admitted depth".into());}
                let units=base_units.saturating_add(self.units);
                self.envelope=Some(semio_framework_pack_json::JsonWriteCursor::new(EvaluationEnvelopeSource{output_json:self.text.take().unwrap(),done:true,cancellable:false,phase:"complete",units_done:units,units_total:units,pending:None,fault:None}));self.stage=2;
                return Ok((None,RetainedCloneProgress {copied_items:1,..empty}));
            },
            3=>{self.stage=4;return Ok((Some(EvaluationReply {external_pending:self.pending_source.is_some(),pending_source:self.pending_source.take(),wire:Some(self.wire.take().unwrap()),returned_request:None,refusal:None,complete:self.complete,faulted:self.faulted,retirement_progress:Default::default()}),empty));},
            _=>return Ok((None,empty)),
        }
        self.units=self.units.saturating_add(1);Ok((None,empty))
    }
    fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if !self.retirement.terminal_is_empty(){return evaluation_value_demands(&self.retirement,copy);}
        if self.active.is_some(){return evaluation_child_demands(&self.active,copy);}
        if self.pending_source.is_some(){return Ok(evaluation_owned_birth::<neural_engine::PendingExtensionEval>());}
        if self.writer.is_some(){return Ok(evaluation_owned_birth::<semio_framework_pack_json::JsonWriteCursor<Dictionary>>());}
        if self.envelope.is_some(){return Ok(evaluation_owned_birth::<semio_framework_pack_json::JsonWriteCursor<EvaluationEnvelopeSource>>());}
        Ok(RetirementDemand {depth:usize::from(!self.terminal_is_empty()),..Default::default()})
    }
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation output close exceeds supplied depth"));}
        let step=if !self.retirement.terminal_is_empty(){self.retirement.close_step(grant)?}
        else if self.active.is_some(){evaluation_child_close(&mut self.active,grant)?}
        else if self.pending_source.is_some(){evaluation_admit_original(&mut self.pending_source,&mut self.active,grant)?}
        else if self.writer.is_some(){evaluation_admit_original(&mut self.writer,&mut self.active,grant)?}
        else if self.envelope.is_some(){evaluation_admit_original(&mut self.envelope,&mut self.active,grant)?}
        else if self.encoding.is_some(){
            self.encoding=None;RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty})
        }else{
            let source=if self.text.is_some(){&mut self.text}else{&mut self.wire};
            let value=source.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"evaluation output close lost its original text"))?;
            match self.retirement.text(value,grant){Ok(progress)=>RetainedCloneStep::Progress(progress),Err((error,value))=>{*source=Some(value);return Err(error);}}
        };
        let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,RetainedCloneStep::Progress(step.progress()),false,"evaluation output parent")?;
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }
    fn terminal_is_empty(&self)->bool {self.pending_source.is_none()&&self.writer.is_none()&&self.envelope.is_none()&&self.text.is_none()&&self.wire.is_none()&&self.encoding.is_none()&&self.active.is_none()&&self.retirement.terminal_is_empty()}
}
impl Drop for EvaluationOutput {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"evaluation output dropped before terminal-empty");}}

struct RetainedEvaluation {
    registry:Option<neural_engine::SharedRegistry>,lease:Option<neural_engine::RegistryLeaseRetirement>,owner_identity:neural_engine::RegistryIdentity,
    request_admission:Option<EvaluationRequestAdmission>,discarded_request:Option<EvaluationRequestAdmission>,request_initial:bool,discarded_operator:Option<String>,neuron_id:String,pending_wave:Option<neural_engine::evaluation::wave::PendingWaveCursor>,pending_wave_closing:bool,external_pending:Option<neural_engine::PendingExtensionEval>,external_completion:Option<EvaluationExternalCompletion>,discarded_neuron:Option<String>,
    job:Option<Box<dyn neural_engine::OperatorJob>>,admission:Option<EvaluationInputPreparation>,replacement:Option<EvaluationInputPreparation>,candidate:Option<(Dictionary,String)>,
    dependency_identity:String,version:String,cancellation_id:String,next_version:Option<String>,next_cancellation:Option<String>,
    retirement:neural_engine::ValueRetirement,finished:Option<Dictionary>,output:Option<EvaluationOutput>,fault:Option<EvaluationFailure>,
    units:usize,identity_position:Option<usize>,cancelled:bool,restarting:bool,normal_progress:RetainedCloneProgress,
    plan:neural_engine::OperatorPlanCursor,planning:bool,finish:neural_engine::OperatorFinishCursor,
    active:std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    response:Option<EvaluationOutput>,response_failed:bool,continuation_identity:Option<EvaluationContinuationIdentity>,compact_request:Option<EvaluateRequest>,compact_cleanup:bool,compact_active:std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,completed_reply:Option<EvaluationReply>,pending_input:Option<Dictionary>,discarded_candidate:Option<(Dictionary,String)>,discarded_preparation:Option<EvaluationInputPreparation>,discarded_version:Option<String>,discarded_cancellation:Option<String>,
}


impl RetainedEvaluation {
    fn new(registry:neural_engine::SharedRegistry,request:EvaluateRequest)->Self {
        Self {finish:neural_engine::OperatorFinishCursor::new(),plan:neural_engine::OperatorPlanCursor::new(),planning:false,owner_identity:registry.owner_identity(),registry:Some(registry),lease:None,active:std::mem::ManuallyDrop::new(None),response:None,response_failed:false,continuation_identity:None,compact_request:None,compact_cleanup:false,compact_active:std::mem::ManuallyDrop::new(None),completed_reply:None,pending_input:None,discarded_candidate:None,discarded_preparation:None,discarded_version:None,discarded_cancellation:None,request_admission:Some(EvaluationRequestAdmission::new(request)),discarded_request:None,request_initial:true,discarded_operator:None,neuron_id:String::new(),pending_wave:None,pending_wave_closing:false,external_pending:None,external_completion:None,discarded_neuron:None,job:None,admission:None,replacement:None,candidate:None,dependency_identity:String::new(),version:String::new(),cancellation_id:String::new(),next_version:None,next_cancellation:None,retirement:Default::default(),finished:None,output:None,fault:None,units:0,identity_position:None,cancelled:false,restarting:false,normal_progress:Default::default()}
    }
    fn current_version(&self)->&str{self.request_admission.as_ref().and_then(|owner|owner.request.as_ref()).map_or_else(||self.next_version.as_deref().unwrap_or(&self.version),|request|request.operator_version.as_str())}
    fn current_neuron(&self)->&str{self.request_admission.as_ref().and_then(|owner|owner.request.as_ref()).map_or(&self.neuron_id,|request|&request.neuron_id)}
    fn current_cancellation(&self)->&str{self.request_admission.as_ref().and_then(|owner|owner.request.as_ref()).map_or_else(||self.next_cancellation.as_deref().unwrap_or(&self.cancellation_id),|request|request.cancellation_id.as_str())}
    fn progress(&self)->neural_engine::OperatorProgress {
        let (done,total,phase)=if self.cancelled {(self.units,self.units.saturating_add(1),if self.restarting {"retire-invalidated"}else{"input-retire"})}
        else if self.request_admission.is_some(){(self.units,self.units.saturating_add(1),"request-admit")}
        else if self.completed_reply.is_some() {(self.units,self.units.saturating_add(1),"output-retire")}
        else if !self.retirement.terminal_is_empty() {(self.units,self.units.saturating_add(1),"input-retire")}
        else if let Some(admission)=&self.admission {let (done,total,phase)=admission.progress();(self.units.saturating_add(done),self.units.saturating_add(total),phase)}
        else if self.pending_wave.is_some(){(self.units,self.units.saturating_add(1),if self.pending_wave_closing{"external-source-retire"}else{"external-wave"})}
        else if let Some(job)=&self.job {let progress=job.progress();(self.units.saturating_add(progress.units_done),self.units.saturating_add(progress.units_total),progress.phase)}
        else if let Some(output)=&self.output {(self.units.saturating_add(output.units),self.units.saturating_add(output.units).saturating_add(1),output.phase())}
        else {(self.units,self.units.saturating_add(1),if self.identity_position.is_some(){"identity-compare"}else{"input-admit"})};
        neural_engine::OperatorProgress {units_done:done,units_total:total,phase}
    }
    fn cancel(&mut self) {
        if !self.cancelled {if let Some(output)=&mut self.output {self.units=self.units.saturating_add(output.units);output.units=0;}}
        self.cancelled=true;self.planning=false;self.restarting=false;if let Some(admission)=&mut self.admission {admission.cancel();}if let Some(job)=&mut self.job {job.cancel();}
    }
    fn retirement_demands(&self,copy:usize,preserve_candidate:bool,preserve_fault:bool)->Result<RetirementDemand,ValueError>{
        if self.continuation_identity.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()})}
        if !self.retirement.terminal_is_empty(){return evaluation_value_demands(&self.retirement,copy);}
        if self.active.is_some(){return evaluation_child_demands(&self.active,copy);}
        if self.compact_active.is_some(){return evaluation_child_demands(&self.compact_active,copy);}
        if self.compact_request.is_some(){return Ok(evaluation_owned_birth::<EvaluateRequest>());}
        if let Some(response)=&self.response{return response.retirement_demands(copy);}
        if let Some(completion)=&self.external_completion{return completion.retirement_demands(copy);}
        if self.external_pending.is_some(){return Ok(evaluation_owned_birth::<neural_engine::PendingExtensionEval>());}
        if let Some(wave)=&self.pending_wave{return wave.next_close_demands(copy);}
        if self.discarded_neuron.is_some(){return Ok(RetirementDemand {depth:1,..Default::default()});}
        if self.discarded_request.is_some()||(!preserve_candidate&&self.request_admission.is_some()){return Ok(evaluation_owned_birth::<EvaluationRequestAdmission>());}
        if let Some(previous)=&self.discarded_preparation{return previous.retirement_demands(copy);}
        if self.discarded_candidate.is_some(){return Ok(evaluation_owned_birth::<(Dictionary,String)>());}
        if self.pending_input.is_some()||self.discarded_operator.is_some()||self.discarded_version.is_some()||self.discarded_cancellation.is_some(){return Ok(RetirementDemand {depth:1,..Default::default()});}
        if let Some(admission)=&self.admission{return admission.retirement_demands(copy);}
        if !preserve_candidate {if let Some(replacement)=&self.replacement{return replacement.retirement_demands(copy);}}
        if let Some(job)=&self.job{
            if job.terminal_is_empty(){return Ok(RetirementDemand {release_bytes:std::mem::size_of_val(job.as_ref()),depth:1,..Default::default()});}
            return Ok(RetirementDemand {copy_bytes:job.next_close_copy_byte_demand()?,capacity_bytes:job.next_close_capacity_byte_demand(copy)?,release_bytes:job.next_close_release_byte_demand()?,depth:job.next_close_depth_demand()?});
        }
        if let Some(output)=&self.output{return output.retirement_demands(copy);}
        if !preserve_candidate&&self.candidate.is_some(){return Ok(evaluation_owned_birth::<(Dictionary,String)>());}
        if !preserve_fault&&self.fault.is_some(){return Ok(evaluation_owned_birth::<EvaluationFailure>());}
        if let Some(lease)=&self.lease{return Ok(RetirementDemand {copy_bytes:lease.next_copy_byte_demand()?,capacity_bytes:lease.next_capacity_byte_demand(copy)?,release_bytes:lease.next_release_byte_demand()?,depth:lease.next_depth_demand()?});}
        Ok(RetirementDemand {depth:usize::from(!self.close_fields_terminal(preserve_candidate,preserve_fault)),..Default::default()})
    }
    fn close_fields_terminal(&self,preserve_candidate:bool,preserve_fault:bool)->bool{
        self.continuation_identity.is_none()&&self.external_pending.is_none()&&self.external_completion.is_none()&&self.response.is_none()&&self.compact_request.is_none()&&self.compact_active.is_none()&&self.pending_wave.is_none()&&self.discarded_neuron.is_none()&&self.neuron_id.capacity()==0&&(preserve_candidate||self.request_admission.is_none())&&self.discarded_request.is_none()&&self.discarded_operator.is_none()&&(preserve_fault||self.fault.is_none())&&(preserve_fault||self.completed_reply.is_none())&&self.pending_input.is_none()&&self.discarded_candidate.is_none()&&self.discarded_preparation.is_none()&&self.discarded_version.is_none()&&self.discarded_cancellation.is_none()&&self.active.is_none()&&self.retirement.terminal_is_empty()&&self.admission.is_none()&&self.job.is_none()&&self.finished.is_none()&&self.output.is_none()&&self.dependency_identity.capacity()==0&&self.version.capacity()==0&&self.cancellation_id.capacity()==0&&(preserve_candidate||(self.replacement.is_none()&&self.candidate.is_none()&&self.next_version.is_none()&&self.next_cancellation.is_none()&&self.registry.is_none()&&self.lease.is_none()))
    }
    fn close_step(&mut self,grant:RetainedCloneGrant,preserve_candidate:bool,preserve_fault:bool)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();
        if self.close_fields_terminal(preserve_candidate,preserve_fault){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.retirement_demands(grant.maximum_copy_bytes,preserve_candidate,preserve_fault)?;
        if self.continuation_identity.is_some(){if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original comparison metadata close requires depth"))}self.continuation_identity=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}))}
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation close exceeds supplied depth"));}
        let step=if !self.retirement.terminal_is_empty(){self.retirement.close_step(grant)?}
        else if self.active.is_some(){evaluation_child_close(&mut self.active,grant)?}
        else if self.compact_active.is_some(){evaluation_child_close(&mut self.compact_active,grant)?}
        else if self.compact_request.is_some(){evaluation_admit_original(&mut self.compact_request,&mut self.compact_active,grant)?}
        else if let Some(response)=&mut self.response{let step=response.close_step(grant)?;if response.terminal_is_empty(){self.response=None;}step}
        else if let Some(completion)=&mut self.external_completion{let step=completion.close_step(grant)?;if completion.terminal_is_empty(){self.external_completion=None;}step}
        else if self.external_pending.is_some(){evaluation_admit_original(&mut self.external_pending,&mut self.active,grant)?}
        else if let Some(wave)=&mut self.pending_wave{wave.begin_close();let step=wave.close_step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,wave.terminal_is_empty(),"original pending evaluation wave")?;if wave.terminal_is_empty(){self.pending_wave=None;self.pending_wave_closing=false;}step}
        else if self.discarded_neuron.is_some(){let original=self.discarded_neuron.take().unwrap();match self.retirement.text(original,grant){Ok(progress)=>RetainedCloneStep::Progress(progress),Err((error,original))=>{self.discarded_neuron=Some(original);return Err(error);}}}
        else if self.neuron_id.capacity()!=0{let original=std::mem::take(&mut self.neuron_id);match self.retirement.text(original,grant){Ok(progress)=>RetainedCloneStep::Progress(progress),Err((error,original))=>{self.neuron_id=original;return Err(error);}}}
        else if self.discarded_request.is_some(){evaluation_admit_original(&mut self.discarded_request,&mut self.active,grant)?}
        else if !preserve_candidate&&self.request_admission.is_some(){evaluation_admit_original(&mut self.request_admission,&mut self.active,grant)?}
        else if self.intermediate_pending(){self.close_intermediate(grant)?}
        else if let Some(admission)=&mut self.admission{
            let step=admission.close_step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,admission.terminal_is_empty(),"evaluation admission")?;
            if admission.terminal_is_empty(){self.admission=None;}step
        }else if !preserve_candidate&&self.replacement.is_some(){
            let replacement=self.replacement.as_mut().unwrap();let step=replacement.close_step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,replacement.terminal_is_empty(),"evaluation replacement")?;
            if replacement.terminal_is_empty(){self.replacement=None;}step
        }else if let Some(job)=&mut self.job{
            if job.terminal_is_empty(){
                let bytes=std::mem::size_of_val(job.as_ref());
                if grant.maximum_release_bytes<bytes{return Ok(RetainedCloneStep::Progress(empty));}
                drop(self.job.take());RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..empty})
            }else{let step=job.close_step(grant)?;semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,job.terminal_is_empty(),"evaluation operator job")?}
        }else if let Some(value)=self.finished.take(){
            match self.retirement.push_dictionary(value,grant){Ok(progress)=>RetainedCloneStep::Progress(progress),Err((error,value))=>{self.finished=Some(value);self.finish=neural_engine::OperatorFinishCursor::new();return Err(error);}}
        }else if let Some(output)=&mut self.output{
            let step=output.close_step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,output.terminal_is_empty(),"evaluation output")?;
            if output.terminal_is_empty(){self.output=None;}step
        }else if self.dependency_identity.capacity()!=0||self.version.capacity()!=0||self.cancellation_id.capacity()!=0{
            let original=if self.dependency_identity.capacity()!=0{&mut self.dependency_identity}else if self.version.capacity()!=0{&mut self.version}else{&mut self.cancellation_id};
            let value=std::mem::take(original);match self.retirement.text(value,grant){Ok(progress)=>RetainedCloneStep::Progress(progress),Err((error,value))=>{*original=value;return Err(error);}}
        }else if !preserve_candidate&&self.candidate.is_some(){evaluation_admit_original(&mut self.candidate,&mut self.active,grant)?}
        else if !preserve_candidate&&(self.next_version.is_some()||self.next_cancellation.is_some()){
            let original=if self.next_version.is_some(){&mut self.next_version}else{&mut self.next_cancellation};let value=original.take().unwrap();match self.retirement.text(value,grant){Ok(progress)=>RetainedCloneStep::Progress(progress),Err((error,value))=>{*original=Some(value);return Err(error);}}
        }else if !preserve_fault&&self.completed_reply.is_some(){
            let mut reply=self.completed_reply.take().unwrap();let wire=reply.wire.take().unwrap();match self.retirement.text(wire,grant){Ok(progress)=>RetainedCloneStep::Progress(progress),Err((error,wire))=>{reply.wire=Some(wire);self.completed_reply=Some(reply);return Err(error);}}
        }else if !preserve_fault&&self.fault.is_some(){
            evaluation_admit_original(&mut self.fault,&mut self.active,grant)?
        }else if let Some(lease)=&mut self.lease{
            let step=lease.close_step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,lease.terminal_is_empty(),"evaluation registry source lease")?;if lease.terminal_is_empty(){self.lease=None;}step
        }else if !preserve_candidate&&self.registry.is_some(){self.lease=Some(self.registry.take().unwrap().into_retirement());RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty})}
        else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"evaluation close lost its retained frontier"));};
        let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,RetainedCloneStep::Progress(step.progress()),false,"original evaluation parent")?;
        Ok(if self.close_fields_terminal(preserve_candidate,preserve_fault){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }
    fn intermediate_pending(&self)->bool{!self.retirement.terminal_is_empty()||self.active.is_some()||(self.pending_input.is_some()&&!self.planning)||self.discarded_request.is_some()||self.discarded_candidate.is_some()||self.discarded_preparation.is_some()||self.discarded_operator.is_some()||self.discarded_version.is_some()||self.discarded_cancellation.is_some()}
    fn close_intermediate(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if !self.retirement.terminal_is_empty(){return self.retirement.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if self.active.is_some(){return evaluation_child_close(&mut self.active,grant);}
        if self.discarded_request.is_some(){return evaluation_admit_original(&mut self.discarded_request,&mut self.active,grant);}
        if let Some(previous)=&mut self.discarded_preparation{
            let step=previous.close_step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,previous.terminal_is_empty(),"discarded original input")?;if previous.terminal_is_empty(){self.discarded_preparation=None;}return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.discarded_candidate.is_some(){return evaluation_admit_original(&mut self.discarded_candidate,&mut self.active,grant);}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation intermediate handoff exceeds admitted depth"));}
        if let Some(value)=self.pending_input.take(){return match self.retirement.push_dictionary(value,grant){Ok(progress)=>Ok(RetainedCloneStep::Progress(progress)),Err((error,value))=>{self.pending_input=Some(value);Err(error)}};}
        let original=if self.discarded_operator.is_some(){&mut self.discarded_operator}else if self.discarded_version.is_some(){&mut self.discarded_version}else{&mut self.discarded_cancellation};
        if let Some(value)=original.take(){return match self.retirement.text(value,grant){Ok(progress)=>Ok(RetainedCloneStep::Progress(progress)),Err((error,value))=>{*original=Some(value);Err(error)}};}
        Ok(RetainedCloneStep::Complete(empty))
    }
    fn close_finished_job(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
        let job=self.job.as_mut().unwrap();let empty=RetainedCloneProgress::default();
        if grant.maximum_items==0{return Ok(empty);}
        if job.terminal_is_empty(){
            if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"terminal evaluation job shell requires admitted depth"));}
            let bytes=std::mem::size_of_val(job.as_ref());if grant.maximum_release_bytes<bytes{return Ok(empty);}drop(self.job.take());return Ok(RetainedCloneProgress {copied_items:1,released_bytes:bytes,..empty});
        }
        let step=job.close_step(grant)?;semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,job.terminal_is_empty(),"finished original operator job").map(|step|step.progress())
    }
    fn normal_step_progress(&self)->RetainedCloneProgress {self.normal_progress}
    fn advance(&mut self,registry:&Registry,operator:&str,maximum_units:usize,maximum_bytes:usize,grant:RetainedCloneGrant)->Result<(EvaluationStep,RetainedCloneProgress),EvaluationFailure>{
        self.normal_progress=Default::default();let result=self.advance_original(registry,operator,maximum_units,maximum_bytes,grant);if let Ok((_,progress))=&result {self.normal_progress=*progress;}result
    }
    fn advance_original(&mut self,registry:&Registry,operator:&str,maximum_units:usize,maximum_bytes:usize,grant:RetainedCloneGrant)->Result<(EvaluationStep,RetainedCloneProgress),EvaluationFailure>{
        use neural_engine::OperatorJobStep;
        let empty=RetainedCloneProgress::default();
        if maximum_units==0||maximum_bytes==0||grant.maximum_items==0{return Ok((EvaluationStep::Working(self.progress()),empty));}
        if !self.cancelled&&self.completed_reply.as_ref().is_some_and(|reply|reply.external_pending){let mut reply=self.completed_reply.take().unwrap();self.external_pending=reply.pending_source.take();return Ok((EvaluationStep::Done(reply),RetainedCloneProgress{copied_items:1,..empty}));}
        if !self.cancelled&&self.completed_reply.is_some(){
            let step=self.close_step(grant,false,true).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;let receipt=step.progress();
            if matches!(step,RetainedCloneStep::Complete(_)){return Ok((EvaluationStep::Done(self.completed_reply.take().unwrap()),receipt));}
            return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if self.cancelled{
            let step=self.close_step(grant,self.restarting,true).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;let receipt=step.progress();
            if matches!(step,RetainedCloneStep::Complete(_)){
                if let Some(fault)=self.fault.take(){return Ok((EvaluationStep::Fault(fault),receipt));}
                if self.restarting{if self.replacement.is_some(){self.admission=self.replacement.take();}if self.request_admission.is_none(){self.version=self.next_version.take().unwrap();self.cancellation_id=self.next_cancellation.take().unwrap();}else{self.request_initial=true;}self.cancelled=false;self.restarting=false;self.units=0;self.identity_position=None;return Ok((EvaluationStep::Working(neural_engine::OperatorProgress {units_done:0,units_total:0,phase:"restart-admission"}),receipt));}
                return Ok((EvaluationStep::Cancelled(neural_engine::OperatorProgress {units_done:self.units,units_total:self.units,phase:"cancelled"}),receipt));
            }
            return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if let Some(completion)=&mut self.external_completion{let result=completion.step(grant);let receipt=completion.normal_step_progress();self.normal_progress=receipt;match result{Ok(Some(dictionary))=>{assert!(completion.terminal_is_empty());self.external_completion=None;self.finished=Some(dictionary);self.finish=neural_engine::OperatorFinishCursor::new();},Ok(None)=>{},Err(error)=>{self.fault=Some(error);self.cancel();}}return Ok((EvaluationStep::Working(self.progress()),receipt));}
        if self.external_pending.is_some()&&self.finished.is_none(){return Ok((EvaluationStep::Working(neural_engine::OperatorProgress{units_done:self.units,units_total:self.units.saturating_add(1),phase:"external-await"}),empty));}
        if let Some(owner)=&mut self.request_admission{
            let receipt=owner.step_with_operator(operator,grant).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;
            if let Some((request,operator,version))=owner.take_ready(){if self.neuron_id.capacity()!=0{self.discarded_neuron=Some(std::mem::take(&mut self.neuron_id));}self.neuron_id=request.neuron_id;if self.request_initial{self.version=request.operator_version;self.cancellation_id=request.cancellation_id;}else{self.discarded_version=self.next_version.replace(request.operator_version);self.discarded_cancellation=self.next_cancellation.replace(request.cancellation_id);}if request.operator_id.capacity()!=0{self.discarded_operator=Some(request.operator_id);}self.admission=Some(EvaluationInputPreparation::new(request.input_json,request.dependency_json,operator,version));self.request_admission=None;}
            self.units=self.units.saturating_add(receipt.copied_items);return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if self.intermediate_pending(){let step=self.close_intermediate(grant).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;return Ok((EvaluationStep::Working(self.progress()),step.progress()));}
        if self.finished.is_some()&&self.job.is_some(){let receipt=self.close_finished_job(grant).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;return Ok((EvaluationStep::Working(self.progress()),receipt));}
        if let Some(admission)=&mut self.admission{
            let result=admission.step(maximum_units,maximum_bytes,grant);
            let receipt=match result{
                Ok((Some(value),receipt))=>{self.units=self.units.saturating_add(admission.progress().0);assert!(admission.terminal_is_empty());self.admission=None;self.candidate=Some(value);if self.job.is_some(){self.identity_position=Some(0);}receipt},
                Ok((None,receipt))=>receipt,
                Err(error)=>{let receipt=admission.normal_step_progress();self.units=self.units.saturating_add(admission.progress().0);self.fault=Some(error);self.cancelled=true;if let Some(job)=&mut self.job{job.cancel();}receipt},
            };return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if let Some(position)=self.identity_position{
            let identity=&self.candidate.as_ref().unwrap().1;
            let changed=identity.len()!=self.dependency_identity.len()||(position<identity.len()&&identity.as_bytes()[position]!=self.dependency_identity.as_bytes()[position]);
            if changed{self.job.as_mut().unwrap().cancel();self.cancelled=true;self.restarting=true;self.identity_position=None;}
            else if position==identity.len(){
                self.discarded_candidate=self.candidate.take();
                if let Some(version)=self.next_version.take(){self.discarded_version=Some(std::mem::replace(&mut self.version,version));}
                if let Some(id)=self.next_cancellation.take(){self.discarded_cancellation=Some(std::mem::replace(&mut self.cancellation_id,id));}self.identity_position=None;
            }else{self.identity_position=Some(position+1);}
            self.units=self.units.saturating_add(1);return Ok((EvaluationStep::Working(self.progress()),empty));
        }
        if let Some((input,identity))=self.candidate.take(){
            self.dependency_identity=identity;self.pending_input=Some(input);self.plan=neural_engine::OperatorPlanCursor::new();self.planning=true;
            self.units=self.units.saturating_add(1);return Ok((EvaluationStep::Working(self.progress()),empty));
        }
        if self.planning {
            let result=registry.dispatch_job(operator,&mut self.pending_input,&mut self.plan,grant);self.normal_progress=self.plan.step_progress();let receipt=self.normal_progress;
            match result {
                Ok(neural_engine::OperatorPlanStep::Working)=>{},
                Ok(neural_engine::OperatorPlanStep::Admitted(neural_engine::OperatorPlanAdmission::Job(job)))=>{self.job=Some(job);self.planning=false;},
                Ok(neural_engine::OperatorPlanStep::Admitted(neural_engine::OperatorPlanAdmission::Pending(original)))=>{self.pending_wave=Some(neural_engine::evaluation::wave::PendingWaveCursor::new(original));self.pending_wave_closing=false;self.planning=false;},
                Ok(neural_engine::OperatorPlanStep::Admitted(neural_engine::OperatorPlanAdmission::Immediate(original)))=>{self.pending_input=Some(original);self.planning=false;self.fault=Some("original operator has no retained evaluation plan".into());self.cancelled=true;},
                Err(error)=>{self.fault=Some(error.into());self.planning=false;self.cancelled=true;},
            }
            self.units=self.units.saturating_add(1);return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if let Some(wave)=&mut self.pending_wave{
            if self.pending_wave_closing{let step=wave.close_step(grant).map_err(|error|{self.normal_progress=wave.progress();EvaluationFailure::from(error)})?;let receipt=step.progress();if wave.terminal_is_empty(){self.pending_wave=None;self.pending_wave_closing=false;}return Ok((EvaluationStep::Working(self.progress()),receipt));}
            let result=wave.step(&self.neuron_id,grant);let receipt=wave.progress();self.normal_progress=receipt;
            match result{Ok(Some(original))=>{self.output=Some(EvaluationOutput::pending(original));wave.begin_close();self.pending_wave_closing=true;},Ok(None)=>{},Err(error)=>{self.fault=Some(error.into());self.cancelled=true;}}
            return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if let Some(job)=&mut self.job{
            let demand=RetirementDemand {copy_bytes:job.next_step_copy_byte_demand().map_err(EvaluationFailure::from)?,capacity_bytes:job.next_step_capacity_byte_demand(grant.maximum_copy_bytes).map_err(EvaluationFailure::from)?,release_bytes:job.next_step_release_byte_demand().map_err(EvaluationFailure::from)?,depth:job.next_step_depth_demand().map_err(EvaluationFailure::from)?};
            if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok((EvaluationStep::Working(self.progress()),empty));}
            let receipt=match job.step(maximum_units,grant){
                Ok((step,receipt))=>{
                    self.normal_progress=job.normal_step_progress();
                    if self.normal_progress!=receipt{return Err("original operator reply differs from its actual normal step receipt".into());}
                    if !receipt.fits(grant){return Err("original operator step exceeded the caller full grant".into());}
                    match step{OperatorJobStep::Working(progress)=>return Ok((EvaluationStep::Working(neural_engine::OperatorProgress {units_done:self.units.saturating_add(progress.units_done),units_total:self.units.saturating_add(progress.units_total),phase:progress.phase}),receipt)),OperatorJobStep::Done(value)=>{self.units=self.units.saturating_add(job.progress().units_total);self.finished=Some(value);self.finish=neural_engine::OperatorFinishCursor::new();},OperatorJobStep::Cancelled(_)=>{self.cancelled=true;}}receipt
                },
                Err(error)=>{let receipt=job.normal_step_progress();self.normal_progress=receipt;self.fault=Some(error.into());self.cancelled=true;job.cancel();receipt},
            };return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if self.finished.is_some()&&self.external_pending.is_some(){let step=evaluation_admit_original(&mut self.external_pending,&mut self.active,grant).map_err(EvaluationFailure::from)?;return Ok((EvaluationStep::Working(self.progress()),step.progress()));}
        if self.finished.is_some(){
            let result=registry.finish_job(operator,&mut self.finished,&mut self.finish,grant);self.normal_progress=self.finish.step_progress();let receipt=self.normal_progress;
            match result {Ok(Some(output))=>self.output=Some(EvaluationOutput::new(output)),Ok(None)=>{},Err(error)=>{self.fault=Some(error.into());self.cancelled=true;}}
            return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if let Some(output)=&mut self.output{
            match output.step(maximum_units,self.units,grant){Ok((Some(wire),receipt))=>{assert!(output.terminal_is_empty());self.output=None;self.completed_reply=Some(wire);return Ok((EvaluationStep::Working(self.progress()),receipt));},Ok((None,receipt))=>return Ok((EvaluationStep::Working(self.progress()),receipt)),Err(error)=>{let receipt=output.normal_step_progress();self.fault=Some(error);self.cancelled=true;return Ok((EvaluationStep::Working(self.progress()),receipt));}}
        }
        Ok((EvaluationStep::Working(self.progress()),empty))
    }
}

struct EvaluationRetirement(RetainedEvaluation);
impl semio_framework_value::retirement::RetirementCursor for EvaluationRetirement {
    fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep{
        use semio_framework_value::retirement::RetirementStep as Step;
        match self.0.close_step(grant,false,false){Err(error)=>Step::Failure(error),Ok(RetainedCloneStep::Complete(progress)) if progress==RetainedCloneProgress::default()=>Step::Complete,Ok(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=>Step::Progress(progress)}
    }
    fn terminal_is_empty(&self)->bool{self.0.close_fields_terminal(false,false)}
    fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.0.retirement_demands(0,false,false)?.copy_bytes)}
    fn next_birth_bytes(&self,copy:usize)->Option<usize>{self.0.retirement_demands(copy,false,false).ok().map(|demand|demand.capacity_bytes)}
    fn next_close_byte_demand(&self)->Option<usize>{self.0.retirement_demands(0,false,false).ok().map(|demand|demand.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.0.retirement_demands(0,false,false)?.depth)}
    fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(std::mem::size_of::<Self>())}
}
impl RetireOwned for RetainedEvaluation {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(EvaluationRetirement(self))}
    fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<EvaluationRetirement>())}
    fn controlled_retirement_supported()->bool{true}
}

#[derive(Default)]
struct EvaluationJobRegistry {
    jobs:protocol::HistoryFoldIndex<(neural_engine::RegistryIdentity,String,u64),RetainedEvaluation>,
    retiring_key:Option<semio_framework_value::retirement::controlled::ControlledRetirement<(neural_engine::RegistryIdentity,String,u64)>>,
    retiring_backing:Option<semio_framework_value::retirement::controlled::ControlledRetirement<protocol::HistoryFoldIndex<(neural_engine::RegistryIdentity,String,u64),RetainedEvaluation>>>,
    closing_owner:Option<neural_engine::RegistryIdentity>,admission_owner:Option<neural_engine::RegistryIdentity>,lookup_epoch:u64,
}

static EVALUATION_JOBS: std::sync::OnceLock<std::sync::Mutex<EvaluationJobRegistry>> = std::sync::OnceLock::new();

fn evaluation_jobs() -> &'static std::sync::Mutex<EvaluationJobRegistry> {
    EVALUATION_JOBS.get_or_init(|| std::sync::Mutex::new(EvaluationJobRegistry::default()))
}

fn evaluation_retire_cold<T:RetireOwned>(value:T){
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
pub fn retire_cancelled_evaluations_close_step(owner:&neural_engine::SharedRegistry,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
    let empty=RetainedCloneProgress::default();
    let mut registry=evaluation_jobs().lock().map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"evaluation registry close is contended"))?;
    let identity=owner.owner_identity();let demand=evaluation_registry_demands(&registry,identity,grant.maximum_copy_bytes)?;
    if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
    if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation registry close exceeds supplied depth"));}
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

/// 🔌️ Retains the original extension registry and its existing exact retirement authority.
pub struct ExtensionEvaluationResources {invoke:std::sync::Mutex<Option<EvaluationInvokeCursor>>,registry:Option<neural_engine::SharedRegistry>,lease:Option<neural_engine::RegistryLeaseRetirement>,retirement:neural_engine::RegistryRetirement}
impl ExtensionEvaluationResources {
    pub fn new(registry:Registry)->Self {let (registry,retirement)=neural_engine::SharedRegistry::new(registry);Self {invoke:std::sync::Mutex::new(None),registry:Some(registry),lease:None,retirement}}
    pub fn registry(&self)->&neural_engine::SharedRegistry {self.registry.as_ref().expect("open extension evaluation owner")}
    fn source_close(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        let pending=self.invoke.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(cursor)=pending {let step=cursor.close_step_with_registry(self.registry.as_ref().expect("original invocation retains its registry"),grant)?;if cursor.terminal_is_empty(){*pending=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if let Some(registry)=&self.registry{
            if evaluation_retirement_pending(registry){return retire_cancelled_evaluations_close_step(registry,grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
            if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation resource lease handoff requires admitted depth"));}
            self.lease=Some(self.registry.take().unwrap().into_retirement());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty}));
        }
        if let Some(lease)=&mut self.lease{
            let step=lease.close_step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,lease.terminal_is_empty(),"extension evaluation source lease")?;
            if lease.terminal_is_empty(){self.lease=None;}return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        self.retirement.close_step(grant)
    }
}
impl semio_framework_plugin::ExtensionResourceOwner for ExtensionEvaluationResources {
    fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        let pending=self.invoke.lock().unwrap_or_else(std::sync::PoisonError::into_inner);if let Some(cursor)=pending.as_ref(){return cursor.retirement_demands_with_registry(self.registry.as_ref().expect("original invocation retains its registry"),copy);}
        if let Some(registry)=&self.registry{if evaluation_retirement_pending(registry){return evaluation_retirement_demands(registry,copy);}return Ok(RetirementDemand {depth:1,..Default::default()});}
        if let Some(lease)=&self.lease{return Ok(RetirementDemand {copy_bytes:lease.next_copy_byte_demand()?,capacity_bytes:lease.next_capacity_byte_demand(copy)?,release_bytes:lease.next_release_byte_demand()?,depth:lease.next_depth_demand()?});}
        Ok(RetirementDemand {copy_bytes:self.retirement.next_copy_byte_demand()?,capacity_bytes:self.retirement.next_capacity_byte_demand(copy)?,release_bytes:self.retirement.next_release_byte_demand()?,depth:self.retirement.next_depth_demand()?})
    }
    fn invoke(&self,capability:&str,request:&[u8],cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_plugin::ExtensionInvokeStep,semio_framework::Fault>{
        match capability {"evaluate"=>evaluate_invoke_json(self,request,cx),_=>Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin,semio_framework::FaultCode::new("extension.unknown-capability"),"unknown flow evaluation capability"))}
    }

    fn cancel_inference(&self,cancellation_id:&str)->bool{self.registry.as_ref().is_some_and(|registry|cancel_inference_evaluation(registry,cancellation_id))}
    fn begin_close(&mut self){if let Some(cursor)=self.invoke.get_mut().unwrap_or_else(std::sync::PoisonError::into_inner){cursor.begin_close();}if let Some(registry)=&self.registry{cancel_all_evaluations(registry);}}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_plugin::PluginLifecycleStep,semio_framework::Fault>{
        let result=self.source_close(grant).and_then(|step|semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,self.terminal_is_empty(),"original extension evaluation resource"));
        result.map(|step|semio_framework_plugin::PluginLifecycleStep::retained(step,self.terminal_is_empty())).map_err(|error|semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin,semio_framework::FaultCode::new("extension.registry-close"),error.into_message()))
    }
    fn terminal_is_empty(&self)->bool{self.invoke.lock().unwrap_or_else(std::sync::PoisonError::into_inner).is_none()&&self.registry.is_none()&&self.lease.is_none()&&self.retirement.terminal_is_empty()}
}

// #endregion ⏱️EvaluationJobs

// #region 🔖️TopicContribution
/// 🗺️ Builds the `"flow.extension"` topic contribution one host app (`flow-play`,
/// `procedural3d-play`) consumes — factored out of every flow extension's own guest bundle so those
/// crates never need `serde_json` themselves. See
/// `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs::TopicContribution`.
pub fn flow_extension_topic_contribution(app_id: &str, extension_id: &str, label: &str, icon_id: &str, manifest_json: &str) -> semio_framework::TopicContribution {
    semio_framework::TopicContribution::new(
        "flow.extension",
        DslValue::object([
            ("appId".to_string(), DslValue::String(app_id.to_string())),
            ("extensionId".to_string(), DslValue::String(extension_id.to_string())),
            ("label".to_string(), DslValue::String(label.to_string())),
            ("iconId".to_string(), DslValue::String(icon_id.to_string())),
            ("manifestJson".to_string(), DslValue::String(manifest_json.to_string())),
        ]),
    )
}
// #endregion 🔖️TopicContribution

// #region 🔖️Command
/// ⌘️ Stub command handler returning acknowledgement JSON.
pub fn command_json(command_id: &str, args_json: &str) -> String {
    semio_framework_pack_json::to_json_string(&DslValue::object([
        ("ok".to_string(), true.to_value()),
        ("commandId".to_string(), command_id.to_value()),
        ("args".to_string(), args_json.to_value()),
    ]))
}
// #endregion 🔖️Command

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
