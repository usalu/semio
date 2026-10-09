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
pub fn evaluate_invoke_json(registry: &neural_engine::SharedRegistry, request: &[u8]) -> Result<Vec<u8>, String> {
    let text = std::str::from_utf8(request).map_err(|error| error.to_string())?;
    let request: EvaluateRequest = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    Ok(evaluate_step_envelope(registry,request).wire.into_bytes())
}

/// 📇️ The `evaluate` capability's request, as the wire declares it.
#[derive(FromValue)]
#[value(rename_all = "camelCase")]
pub struct EvaluateRequest {
    pub retained:RetainedCloneGrant,
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
pub fn evaluate_step_envelope(registry:&neural_engine::SharedRegistry,request:EvaluateRequest)->EvaluationReply {
    let memory_grant=request.retained;
    use semio_framework_pack_json::{object,Value};
    let budget=if request.budget==0 {EVALUATE_STEP_BUDGET}else{request.budget as usize};
    let wall=if request.wall_micros==0 {EVALUATE_STEP_WALL_MICROS}else{request.wall_micros};
    let mut key=(registry.owner_identity(),request.operator_id.clone(),request.node_hash);
    let complete=|output:String,units:usize|EvaluationReply {wire:semio_framework_pack_json::to_string(&object([("done".into(),Value::Bool(true)),("cancellable".into(),Value::Bool(false)),("phase".into(),Value::String("complete".into())),("unitsDone".into(),Value::from(units as u64)),("unitsTotal".into(),Value::from(units as u64)),("outputJson".into(),Value::String(output))])),complete:true,faulted:true,retirement_progress:Default::default()};
    let existing=evaluation_jobs().lock().ok().and_then(|mut jobs|jobs.jobs.remove_entry(&key));
    let mut retained=match existing {
        Some((original_key,mut retained))=>{
            key=original_key;
            if retained.close_fields_terminal(false,false){
                if request.resume{retain_evaluation_job(key,retained);return complete(error_output_json("compact evaluation resume has no retained owner"),0);}
                RetainedEvaluation::new(registry.clone(),request.input_json,request.dependency_json,request.operator_id.clone(),request.operator_version,request.cancellation_id)
            }else{
            if request.resume {
                if !request.input_json.is_empty()||!request.dependency_json.is_empty()||(!retained.cancelled&&(retained.version!=request.operator_version||retained.cancellation_id!=request.cancellation_id)) {
                    retain_evaluation_job(key,retained);
                    return complete(error_output_json("compact evaluation resume does not match the retained request"),0);
                }
            }else{
                if retained.discarded_version.is_some()||retained.discarded_cancellation.is_some()||retained.discarded_candidate.is_some()||retained.discarded_preparation.is_some(){retain_evaluation_job(key,retained);return complete(error_output_json("supersession remains owned by its previous physical frontier"),0);}
                retained.discarded_version=retained.next_version.replace(request.operator_version.clone());retained.discarded_cancellation=retained.next_cancellation.replace(request.cancellation_id.clone());
                let admission=EvaluationInputPreparation::new(request.input_json,request.dependency_json,request.operator_id.clone(),request.operator_version);
                if retained.admission.is_some()||retained.candidate.is_some()||retained.finished.is_some()||retained.output.is_some()||retained.cancelled {
                    retained.discarded_candidate=retained.candidate.take();
                    retained.discarded_preparation=retained.replacement.replace(admission);
                    retained.cancel();retained.restarting=true;
                }else{retained.admission=Some(admission);}
            }
            retained
            }
        },
        None=>{
            if request.resume {return complete(error_output_json("compact evaluation resume has no retained owner"),0);}
            if evaluation_jobs().lock().map_or(true,|jobs|jobs.jobs.len()>=EVALUATION_JOB_CAPACITY) {return complete(error_output_json("evaluation capacity remains owned by pending work"),0);}
            RetainedEvaluation::new(registry.clone(),request.input_json,request.dependency_json,request.operator_id.clone(),request.operator_version,request.cancellation_id)
        },
    };
    let deadline=semio_framework_job::default_now_us().map(|now|now.saturating_add(wall));let mut spent=0u64;let mut physical=RetainedCloneProgress::default();
    loop{
        let units=if request.round_units==0{budget}else{budget.min(request.round_units.saturating_sub(spent)as usize)};spent=spent.saturating_add(units as u64);
        let remaining=evaluation_remaining_grant(memory_grant,physical);
        let (step,receipt)=match retained.advance(registry,&request.operator_id,units,4096,remaining){Ok(step)=>step,Err(error)=>{let receipt=retained.normal_step_progress();retained.fault=Some(error);retained.cancel();(EvaluationStep::Working(retained.progress()),receipt)}};
        physical=physical.checked_add(receipt).expect("original evaluation cumulative physical receipt");
        let funded=receipt.fits(remaining)&&physical.fits(memory_grant);if !funded{if retained.fault.is_none(){retained.fault=Some("original evaluation exceeded its caller memory grant".into());}retained.cancel();}
        match step{
            EvaluationStep::Fault(error)=>{let mut reply=complete(error_output_json(&error.to_string()),retained.units);reply.retirement_progress=physical;retained.fault=Some(error);retain_evaluation_job(key,retained);return reply;},
            EvaluationStep::Done(mut output)=>{output.retirement_progress=physical;retained.cancelled=true;retain_evaluation_job(key,retained);return output;},
            EvaluationStep::Cancelled(progress)=>{retain_evaluation_job(key,retained);return EvaluationReply {wire:semio_framework_pack_json::to_string(&object([("done".into(),Value::Bool(true)),("cancellable".into(),Value::Bool(false)),("phase".into(),Value::String("cancelled".into())),("unitsDone".into(),Value::from(progress.units_done as u64)),("unitsTotal".into(),Value::from(progress.units_total as u64)),("outputJson".into(),Value::String(String::new()))])),complete:true,faulted:false,retirement_progress:physical};},
            EvaluationStep::Working(progress)=>{
                let expired=deadline.is_none_or(|deadline|semio_framework_job::default_now_us().is_none_or(|now|now>=deadline));
                if funded&&receipt!=RetainedCloneProgress::default()&&physical.copied_items<memory_grant.maximum_items&&!expired&&(request.round_units==0||spent<request.round_units){continue;}
                let envelope=object([("done".into(),Value::Bool(false)),("cancellable".into(),Value::Bool(true)),("phase".into(),Value::String(progress.phase.into())),("unitsDone".into(),Value::from(progress.units_done as u64)),("unitsTotal".into(),Value::from(progress.units_total as u64)),("outputJson".into(),Value::String(String::new()))]);
                retain_evaluation_job(key,retained);return EvaluationReply {wire:semio_framework_pack_json::to_string(&envelope),complete:false,faulted:false,retirement_progress:physical};
            },
        }
    }
}

/// 🎟️ Keeps each original round-trip currency after its actual physical effects.
fn evaluation_remaining_grant(memory_grant:RetainedCloneGrant,physical:RetainedCloneProgress)->RetainedCloneGrant {
    RetainedCloneGrant{maximum_items:memory_grant.maximum_items.saturating_sub(physical.copied_items),maximum_copy_bytes:memory_grant.maximum_copy_bytes.saturating_sub(physical.copied_bytes),maximum_capacity_bytes:memory_grant.maximum_capacity_bytes.saturating_sub(physical.retained_capacity_bytes),maximum_release_bytes:memory_grant.maximum_release_bytes.saturating_sub(physical.released_bytes),..memory_grant}
}

/// 🧾️ Publishes the original physical envelope with facts from its same execution owner.
pub struct EvaluationReply {pub wire:String,pub complete:bool,pub faulted:bool,pub retirement_progress:RetainedCloneProgress}

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
                let bytes=if field==32{std::mem::size_of::<semio_framework_value::native_encoding::NativeEncodeContinuation>()}else{std::mem::size_of::<semio_framework_value::native_decoding::NativeDecodeContinuation>()};
                if grant.maximum_copy_bytes<bytes{return Ok(Default::default());}
                if field==32{self.encoding=None;}else{self.decoding=None;}RetainedCloneProgress {copied_items:1,copied_bytes:bytes,..Default::default()}
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
        let bytes=if self.encoding.is_some(){std::mem::size_of::<semio_framework_value::native_encoding::NativeEncodeContinuation>()}else if self.decoding.is_some(){std::mem::size_of::<semio_framework_value::native_decoding::NativeDecodeContinuation>()}else{0};
        Ok(RetirementDemand {copy_bytes:bytes,depth:usize::from(!self.terminal_is_empty()),..Default::default()})
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
            if grant.maximum_copy_bytes<demand.copy_bytes{return Ok(RetainedCloneStep::Progress(empty));}
            if self.encoding.is_some(){self.encoding=None;}else{self.decoding=None;}
            return self.admit_close(grant,RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..empty}));
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

struct EvaluationOutput {
    writer:Option<semio_framework_pack_json::JsonWriteCursor<Dictionary>>,envelope:Option<semio_framework_pack_json::JsonWriteCursor<DslValue>>,
    encoding:Option<semio_framework_value::native_encoding::NativeEncodeContinuation>,retirement:neural_engine::ValueRetirement,
    text:Option<String>,wire:Option<String>,stage:u8,units:usize,faulted:bool,closing_fields:u8,normal_progress:RetainedCloneProgress,
    active:std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}
impl EvaluationOutput {
    fn new(output:Dictionary)->Self {let faulted=output.get("error").is_some();Self {writer:Some(semio_framework_pack_json::JsonWriteCursor::new(output)),envelope:None,encoding:None,retirement:Default::default(),text:None,wire:None,stage:0,units:0,faulted,closing_fields:0,normal_progress:Default::default(),active:std::mem::ManuallyDrop::new(None)}}
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
            4=>{let bytes=std::mem::size_of::<semio_framework_value::native_encoding::NativeEncodeContinuation>();if grant.maximum_copy_bytes<bytes{return Ok(Default::default());}self.encoding=None;RetainedCloneProgress {copied_items:1,copied_bytes:bytes,..Default::default()}},
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
                if let Some(text)=result.map_err(EvaluationFailure::from)?{if self.stage==0{self.text=Some(text);self.closing_fields=1|4;self.stage=1;}else{self.wire=Some(text);self.closing_fields=2|4;self.stage=3;}}
                self.units=self.units.saturating_add(progress.copied_items);return Ok((None,progress));
            },
            1=>{
                let names=["cancellable","done","outputJson","phase","unitsDone","unitsTotal"];
                let bytes=names.len()*std::mem::size_of::<(String,DslValue)>()+names.iter().map(|name|name.len()).sum::<usize>()+"complete".len();
                if grant.maximum_items==0||grant.maximum_capacity_bytes<bytes{return Ok((None,empty));}
                if grant.maximum_depth==0{return Err("evaluation envelope birth exceeds admitted depth".into());}
                let mut entries=Vec::new();entries.try_reserve_exact(names.len()).map_err(|_|EvaluationFailure::Native(ValueError::literal(ValueRefusalKind::AllocationFailed,"evaluation envelope allocation failed")))?;
                let units=base_units.saturating_add(self.units);
                entries.extend(names.into_iter().zip([DslValue::Bool(false),DslValue::Bool(true),DslValue::String(self.text.take().unwrap()),DslValue::String("complete".into()),DslValue::Number(semio_framework_value::Number::UInt(units as u64)),DslValue::Number(semio_framework_value::Number::UInt(units as u64))]).map(|(key,value)|(key.into(),value)));
                self.envelope=Some(semio_framework_pack_json::JsonWriteCursor::new(DslValue::Object(entries)));self.stage=2;
                return Ok((None,RetainedCloneProgress {copied_items:1,retained_capacity_bytes:bytes,..empty}));
            },
            3=>{self.stage=4;return Ok((Some(EvaluationReply {wire:self.wire.take().unwrap(),complete:true,faulted:self.faulted,retirement_progress:Default::default()}),empty));},
            _=>return Ok((None,empty)),
        }
        self.units=self.units.saturating_add(1);Ok((None,empty))
    }
    fn retirement_demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if !self.retirement.terminal_is_empty(){return evaluation_value_demands(&self.retirement,copy);}
        if self.active.is_some(){return evaluation_child_demands(&self.active,copy);}
        if self.writer.is_some(){return Ok(evaluation_owned_birth::<semio_framework_pack_json::JsonWriteCursor<Dictionary>>());}
        if self.envelope.is_some(){return Ok(evaluation_owned_birth::<semio_framework_pack_json::JsonWriteCursor<DslValue>>());}
        Ok(RetirementDemand {copy_bytes:if self.encoding.is_some(){std::mem::size_of::<semio_framework_value::native_encoding::NativeEncodeContinuation>()}else{0},depth:usize::from(!self.terminal_is_empty()),..Default::default()})
    }
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation output close exceeds supplied depth"));}
        let step=if !self.retirement.terminal_is_empty(){self.retirement.close_step(grant)?}
        else if self.active.is_some(){evaluation_child_close(&mut self.active,grant)?}
        else if self.writer.is_some(){evaluation_admit_original(&mut self.writer,&mut self.active,grant)?}
        else if self.envelope.is_some(){evaluation_admit_original(&mut self.envelope,&mut self.active,grant)?}
        else if self.encoding.is_some(){
            if grant.maximum_copy_bytes<demand.copy_bytes{return Ok(RetainedCloneStep::Progress(empty));}
            self.encoding=None;RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..empty})
        }else{
            let source=if self.text.is_some(){&mut self.text}else{&mut self.wire};
            let value=source.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"evaluation output close lost its original text"))?;
            match self.retirement.text(value,grant){Ok(progress)=>RetainedCloneStep::Progress(progress),Err((error,value))=>{*source=Some(value);return Err(error);}}
        };
        let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,RetainedCloneStep::Progress(step.progress()),false,"evaluation output parent")?;
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }
    fn terminal_is_empty(&self)->bool {self.writer.is_none()&&self.envelope.is_none()&&self.text.is_none()&&self.wire.is_none()&&self.encoding.is_none()&&self.active.is_none()&&self.retirement.terminal_is_empty()}
}
impl Drop for EvaluationOutput {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"evaluation output dropped before terminal-empty");}}

struct RetainedEvaluation {
    registry:Option<neural_engine::SharedRegistry>,lease:Option<neural_engine::RegistryLeaseRetirement>,owner_identity:neural_engine::RegistryIdentity,
    job:Option<Box<dyn neural_engine::OperatorJob>>,admission:Option<EvaluationInputPreparation>,replacement:Option<EvaluationInputPreparation>,candidate:Option<(Dictionary,String)>,
    dependency_identity:String,version:String,cancellation_id:String,next_version:Option<String>,next_cancellation:Option<String>,
    retirement:neural_engine::ValueRetirement,finished:Option<Dictionary>,output:Option<EvaluationOutput>,fault:Option<EvaluationFailure>,
    units:usize,identity_position:Option<usize>,cancelled:bool,restarting:bool,normal_progress:RetainedCloneProgress,
    plan:neural_engine::OperatorPlanCursor,planning:bool,finish:neural_engine::OperatorFinishCursor,
    active:std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    completed_reply:Option<EvaluationReply>,pending_input:Option<Dictionary>,discarded_candidate:Option<(Dictionary,String)>,discarded_preparation:Option<EvaluationInputPreparation>,discarded_version:Option<String>,discarded_cancellation:Option<String>,
}
impl RetainedEvaluation {
    fn new(registry:neural_engine::SharedRegistry,input:String,dependency:String,operator:String,version:String,cancellation_id:String)->Self {
        Self {finish:neural_engine::OperatorFinishCursor::new(),plan:neural_engine::OperatorPlanCursor::new(),planning:false,owner_identity:registry.owner_identity(),registry:Some(registry),lease:None,active:std::mem::ManuallyDrop::new(None),completed_reply:None,pending_input:None,discarded_candidate:None,discarded_preparation:None,discarded_version:None,discarded_cancellation:None,job:None,admission:Some(EvaluationInputPreparation::new(input,dependency,operator,version.clone())),replacement:None,candidate:None,dependency_identity:String::new(),version,cancellation_id,next_version:None,next_cancellation:None,retirement:Default::default(),finished:None,output:None,fault:None,units:0,identity_position:None,cancelled:false,restarting:false,normal_progress:Default::default()}
    }
    fn progress(&self)->neural_engine::OperatorProgress {
        let (done,total,phase)=if self.cancelled {(self.units,self.units.saturating_add(1),if self.restarting {"retire-invalidated"}else{"input-retire"})}
        else if self.completed_reply.is_some() {(self.units,self.units.saturating_add(1),"output-retire")}
        else if !self.retirement.terminal_is_empty() {(self.units,self.units.saturating_add(1),"input-retire")}
        else if let Some(admission)=&self.admission {let (done,total,phase)=admission.progress();(self.units.saturating_add(done),self.units.saturating_add(total),phase)}
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
        if !self.retirement.terminal_is_empty(){return evaluation_value_demands(&self.retirement,copy);}
        if self.active.is_some(){return evaluation_child_demands(&self.active,copy);}
        if let Some(previous)=&self.discarded_preparation{return previous.retirement_demands(copy);}
        if self.discarded_candidate.is_some(){return Ok(evaluation_owned_birth::<(Dictionary,String)>());}
        if self.pending_input.is_some()||self.discarded_version.is_some()||self.discarded_cancellation.is_some(){return Ok(RetirementDemand {depth:1,..Default::default()});}
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
        (preserve_fault||self.fault.is_none())&&(preserve_fault||self.completed_reply.is_none())&&self.pending_input.is_none()&&self.discarded_candidate.is_none()&&self.discarded_preparation.is_none()&&self.discarded_version.is_none()&&self.discarded_cancellation.is_none()&&self.active.is_none()&&self.retirement.terminal_is_empty()&&self.admission.is_none()&&self.job.is_none()&&self.finished.is_none()&&self.output.is_none()&&self.dependency_identity.capacity()==0&&self.version.capacity()==0&&self.cancellation_id.capacity()==0&&(preserve_candidate||(self.replacement.is_none()&&self.candidate.is_none()&&self.next_version.is_none()&&self.next_cancellation.is_none()&&self.registry.is_none()&&self.lease.is_none()))
    }
    fn close_step(&mut self,grant:RetainedCloneGrant,preserve_candidate:bool,preserve_fault:bool)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();
        if self.close_fields_terminal(preserve_candidate,preserve_fault){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.retirement_demands(grant.maximum_copy_bytes,preserve_candidate,preserve_fault)?;
        if grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation close exceeds supplied depth"));}
        let step=if !self.retirement.terminal_is_empty(){self.retirement.close_step(grant)?}
        else if self.active.is_some(){evaluation_child_close(&mut self.active,grant)?}
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
            let mut reply=self.completed_reply.take().unwrap();let wire=std::mem::take(&mut reply.wire);match self.retirement.text(wire,grant){Ok(progress)=>RetainedCloneStep::Progress(progress),Err((error,wire))=>{reply.wire=wire;self.completed_reply=Some(reply);return Err(error);}}
        }else if !preserve_fault&&self.fault.is_some(){
            evaluation_admit_original(&mut self.fault,&mut self.active,grant)?
        }else if let Some(lease)=&mut self.lease{
            let step=lease.close_step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,lease.terminal_is_empty(),"evaluation registry source lease")?;if lease.terminal_is_empty(){self.lease=None;}step
        }else if !preserve_candidate&&self.registry.is_some(){self.lease=Some(self.registry.take().unwrap().into_retirement());RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty})}
        else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"evaluation close lost its retained frontier"));};
        let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,RetainedCloneStep::Progress(step.progress()),false,"original evaluation parent")?;
        Ok(if self.close_fields_terminal(preserve_candidate,preserve_fault){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    }
    fn intermediate_pending(&self)->bool{!self.retirement.terminal_is_empty()||self.active.is_some()||(self.pending_input.is_some()&&!self.planning)||self.discarded_candidate.is_some()||self.discarded_preparation.is_some()||self.discarded_version.is_some()||self.discarded_cancellation.is_some()}
    fn close_intermediate(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        if !self.retirement.terminal_is_empty(){return self.retirement.close_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if self.active.is_some(){return evaluation_child_close(&mut self.active,grant);}
        if let Some(previous)=&mut self.discarded_preparation{
            let step=previous.close_step(grant)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,previous.terminal_is_empty(),"discarded original input")?;if previous.terminal_is_empty(){self.discarded_preparation=None;}return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.discarded_candidate.is_some(){return evaluation_admit_original(&mut self.discarded_candidate,&mut self.active,grant);}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"evaluation intermediate handoff exceeds admitted depth"));}
        if let Some(value)=self.pending_input.take(){return match self.retirement.push_dictionary(value,grant){Ok(progress)=>Ok(RetainedCloneStep::Progress(progress)),Err((error,value))=>{self.pending_input=Some(value);Err(error)}};}
        let original=if self.discarded_version.is_some(){&mut self.discarded_version}else{&mut self.discarded_cancellation};
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
        if !self.cancelled&&self.completed_reply.is_some(){
            let step=self.close_step(grant,false,true).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;let receipt=step.progress();
            if matches!(step,RetainedCloneStep::Complete(_)){return Ok((EvaluationStep::Done(self.completed_reply.take().unwrap()),receipt));}
            return Ok((EvaluationStep::Working(self.progress()),receipt));
        }
        if self.cancelled{
            let step=self.close_step(grant,self.restarting,true).map_err(|error|{self.normal_progress=error.retained_progress();EvaluationFailure::from(error)})?;let receipt=step.progress();
            if matches!(step,RetainedCloneStep::Complete(_)){
                if let Some(fault)=self.fault.take(){return Ok((EvaluationStep::Fault(fault),receipt));}
                if self.restarting{if self.replacement.is_some(){self.admission=self.replacement.take();}self.version=self.next_version.take().unwrap();self.cancellation_id=self.next_cancellation.take().unwrap();self.cancelled=false;self.restarting=false;self.units=0;self.identity_position=None;return Ok((EvaluationStep::Working(neural_engine::OperatorProgress {units_done:0,units_total:0,phase:"restart-admission"}),receipt));}
                return Ok((EvaluationStep::Cancelled(neural_engine::OperatorProgress {units_done:self.units,units_total:self.units,phase:"cancelled"}),receipt));
            }
            return Ok((EvaluationStep::Working(self.progress()),receipt));
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
                Ok(neural_engine::OperatorPlanStep::Admitted(neural_engine::OperatorPlanAdmission::Immediate(original)))=>{self.pending_input=Some(original);self.planning=false;self.fault=Some("original operator has no retained evaluation plan".into());self.cancelled=true;},
                Err(error)=>{self.fault=Some(error.into());self.planning=false;self.cancelled=true;},
            }
            self.units=self.units.saturating_add(1);return Ok((EvaluationStep::Working(self.progress()),receipt));
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
    closing_owner:Option<neural_engine::RegistryIdentity>,
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

/// 🅿️ Parks this same admitted source, job or retirement frontier for the next round trip.
fn retain_evaluation_job(key:(neural_engine::RegistryIdentity,String,u64),retained:RetainedEvaluation) {
    let mut registry=evaluation_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert!(registry.jobs.len()<EVALUATION_JOB_CAPACITY||registry.jobs.contains_key(&key),"retained evaluations require admitted capacity");registry.jobs.insert(key,retained);
}

/// 🛑️ Marks original inference jobs without allocating or advancing physical retirement.
pub fn cancel_inference_evaluation(owner:&neural_engine::SharedRegistry,cancellation_id:&str)->bool{
    let mut accepted=false;
    if let Ok(mut registry)=evaluation_jobs().lock(){for retained in registry.jobs.slot_values_mut().filter(|retained|retained.owner_identity==owner.owner_identity()&&retained.cancellation_id==cancellation_id&&!retained.cancelled){retained.cancel();accepted=true;}}
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
    for slot in 0..registry.jobs.slot_count(){
        if let Some((key,retained))=registry.jobs.extract_slot_if(slot,|_,retained|retained.owner_identity==identity&&retained.cancelled&&retained.close_fields_terminal(false,false)){
            drop(retained);registry.retiring_key=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(key).unwrap_or_else(|(error,_)|panic!("original evaluation key declaration refused: {error}")));registry.closing_owner=Some(identity);
            if registry.jobs.is_empty()&&!registry.jobs.terminal_is_empty(){registry.retiring_backing=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(std::mem::take(&mut registry.jobs)).unwrap_or_else(|(error,_)|panic!("original evaluation arena declaration refused: {error}")));}
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..empty}));
        }
    }
    Ok(RetainedCloneStep::Complete(empty))
}
/// 🧹️ Includes original removed keys and empty-but-allocated arena custody.
pub fn evaluation_retirement_pending(owner:&neural_engine::SharedRegistry)->bool{
    evaluation_jobs().lock().map_or(true,|registry|registry.closing_owner==Some(owner.owner_identity())||registry.jobs.values().any(|retained|retained.owner_identity==owner.owner_identity()&&retained.cancelled))
}

/// 📈️ Progress of the parked evaluation of `operator_id` at `node_hash`, if one is retained.
pub fn evaluation_progress(owner:&neural_engine::SharedRegistry,operator_id: &str, node_hash: u64) -> Option<neural_engine::OperatorProgress> {
    let registry=evaluation_jobs().lock().ok()?;
    registry.jobs.iter().find(|(key,retained)|key.0==owner.owner_identity()&&key.1==operator_id&&key.2==node_hash&&!retained.close_fields_terminal(false,false)).map(|(_,retained)|retained.progress())
}

/// 🔌️ Retains the original extension registry and its existing exact retirement authority.
pub struct ExtensionEvaluationResources {registry:Option<neural_engine::SharedRegistry>,lease:Option<neural_engine::RegistryLeaseRetirement>,retirement:neural_engine::RegistryRetirement}
impl ExtensionEvaluationResources {
    pub fn new(registry:Registry)->Self {let (registry,retirement)=neural_engine::SharedRegistry::new(registry);Self {registry:Some(registry),lease:None,retirement}}
    pub fn registry(&self)->&neural_engine::SharedRegistry {self.registry.as_ref().expect("open extension evaluation owner")}
    fn source_close(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let empty=RetainedCloneProgress::default();
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
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
        if let Some(registry)=&self.registry{if evaluation_retirement_pending(registry){return evaluation_retirement_demands(registry,copy);}return Ok(RetirementDemand {depth:1,..Default::default()});}
        if let Some(lease)=&self.lease{return Ok(RetirementDemand {copy_bytes:lease.next_copy_byte_demand()?,capacity_bytes:lease.next_capacity_byte_demand(copy)?,release_bytes:lease.next_release_byte_demand()?,depth:lease.next_depth_demand()?});}
        Ok(RetirementDemand {copy_bytes:self.retirement.next_copy_byte_demand()?,capacity_bytes:self.retirement.next_capacity_byte_demand(copy)?,release_bytes:self.retirement.next_release_byte_demand()?,depth:self.retirement.next_depth_demand()?})
    }
    fn invoke(&self,capability:&str,request:&[u8])->Result<Vec<u8>,semio_framework::Fault>{
        match capability {"evaluate"=>evaluate_invoke_json(self.registry(),request).map_err(|message|semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin,semio_framework::FaultCode::new("extension.evaluate.bad-request"),message)),_=>Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin,semio_framework::FaultCode::new("extension.unknown-capability"),"unknown flow evaluation capability"))}
    }
    fn cancel_inference(&self,cancellation_id:&str)->bool{self.registry.as_ref().is_some_and(|registry|cancel_inference_evaluation(registry,cancellation_id))}
    fn begin_close(&mut self){if let Some(registry)=&self.registry{cancel_all_evaluations(registry);}}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<semio_framework_plugin::PluginLifecycleStep,semio_framework::Fault>{
        let result=self.source_close(grant).and_then(|step|semio_framework_value::retained_clone::admit_retained_clone_close(grant,step,self.terminal_is_empty(),"original extension evaluation resource"));
        result.map(|step|semio_framework_plugin::PluginLifecycleStep::retained(step,self.terminal_is_empty())).map_err(|error|semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin,semio_framework::FaultCode::new("extension.registry-close"),error.into_message()))
    }
    fn terminal_is_empty(&self)->bool{self.registry.is_none()&&self.lease.is_none()&&self.retirement.terminal_is_empty()}
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
