//! 🔌️ Shared wasm extension glue for flow modules.

use neural_engine::{inject_channel_defaults, ColdOwner, Dictionary, Registry};
use semio_framework_value::{DslValue, FromValue, ToValue};

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
/// 🔀️ Parses a WIT `extension::invoke` "evaluate" capability request and answers ONE budgeted
/// round trip of it — factored out of every flow extension's own guest bundle so those crates
/// never need `serde_json`/`serde::Deserialize` themselves
/// (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`),
/// and so the budget is declared in ONE place for all ten of them.
///
/// ⏱️ Request: `{operatorId, inputJson, nodeHash?, budget?, wallMicros?}`. Answer: the
/// [`EVALUATE_ENVELOPE_SCHEMA`] envelope. An operator with no sub-structure
/// ([`neural_engine::Operator::step_plan`] answering `None` — which is every operator but the three
/// brep set operations) completes inside the first round trip and is indistinguishable from the
/// unbudgeted call it replaces. An operator that DOES offer a job is retained by
/// `(operatorId, nodeHash)` and resumed by the next identical request, exactly the way a budgeted
/// `tessellate` is resumed by the next `flowEvalTick`
/// (ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, `📓️extension-evaluate-budget-2026-09-12.md`).
pub fn evaluate_invoke_json(registry: &neural_engine::SharedRegistry, request: &[u8]) -> Result<Vec<u8>, String> {
    let text = std::str::from_utf8(request).map_err(|error| error.to_string())?;
    let request: EvaluateRequest = semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
    Ok(evaluate_step_envelope(registry, request).wire.into_bytes())
}

/// 📇️ The `evaluate` capability's request, as the wire declares it.
#[derive(FromValue)]
#[value(rename_all = "camelCase")]
pub struct EvaluateRequest {
    pub operator_id: String,
    pub input_json: String,
    /// 🪪️ The requester's own identity for this evaluation — the key a retained job is resumed by.
    /// Zero (the default) means "do not retain": every round trip starts a fresh job, which is what
    /// an unaddressed caller (a test, an export bridge) wants.
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

/// 🧾️ The shape every `evaluate` answer takes. Declared here as one string so the Rust law, the
/// TypeScript twin and the fixture all name the same fields.
pub const EVALUATE_ENVELOPE_SCHEMA: &str = "{done,cancellable,phase,unitsDone,unitsTotal,outputJson}";

/// ⏱️ One budgeted `evaluate` ROUND TRIP as the extension-boundary JSON envelope.
///
/// `budget` bounds ONE step in operator-defined units; `wall_micros` bounds the whole round trip:
/// the call keeps stepping while the job is still working AND the deadline has not passed. Both
/// are needed because units are not time.
pub fn evaluate_step_envelope(registry:&neural_engine::SharedRegistry,request:EvaluateRequest)->EvaluationReply {
    use semio_framework_pack_json::{object,Value};
    let budget=if request.budget==0 {EVALUATE_STEP_BUDGET}else{request.budget as usize};
    let wall=if request.wall_micros==0 {EVALUATE_STEP_WALL_MICROS}else{request.wall_micros};
    let key=(registry.owner_identity(),request.operator_id.clone(),request.node_hash);
    let complete=|output:String,units:usize|EvaluationReply {wire:semio_framework_pack_json::to_string(&object([("done".into(),Value::Bool(true)),("cancellable".into(),Value::Bool(false)),("phase".into(),Value::String("complete".into())),("unitsDone".into(),Value::from(units as u64)),("unitsTotal".into(),Value::from(units as u64)),("outputJson".into(),Value::String(output))])),complete:true,faulted:true};
    let existing=if request.node_hash==0 {None}else{evaluation_jobs().lock().ok().and_then(|mut jobs|jobs.jobs.remove(&key))};
    let mut retained=match existing {
        Some(mut retained)=>{
            if request.resume {
                if !request.input_json.is_empty()||!request.dependency_json.is_empty()||(!retained.cancelled&&(retained.version!=request.operator_version||retained.cancellation_id!=request.cancellation_id)) {
                    retain_evaluation_job(key,retained);
                    return complete(error_output_json("compact evaluation resume does not match the retained request"),0);
                }
            }else{
                if let Some(version)=retained.next_version.replace(request.operator_version.clone()){retained.retirement.text(version);}if let Some(id)=retained.next_cancellation.replace(request.cancellation_id.clone()){retained.retirement.text(id);}
                let admission=EvaluationInputPreparation::new(request.input_json,request.dependency_json,request.operator_id.clone(),request.operator_version);
                if retained.admission.is_some()||retained.candidate.is_some()||retained.finished.is_some()||retained.output.is_some()||retained.cancelled {
                    if let Some((input,identity))=retained.candidate.take(){retained.retirement.push_dictionary(input);retained.retirement.text(identity);}
                    if let Some(previous)=retained.replacement.replace(admission){retained.retirement.push_owned(previous);}
                    retained.cancel();retained.restarting=true;
                }else{retained.admission=Some(admission);}
            }
            retained
        },
        None=>{
            if request.resume {return complete(error_output_json("compact evaluation resume has no retained owner"),0);}
            if evaluation_jobs().lock().map_or(true,|jobs|jobs.jobs.len()>=EVALUATION_JOB_CAPACITY) {return complete(error_output_json("evaluation capacity remains owned by pending work"),0);}
            RetainedEvaluation::new(registry.clone(),request.input_json,request.dependency_json,request.operator_id.clone(),request.operator_version,request.cancellation_id)
        },
    };
    let deadline=semio_framework_job::default_now_us().map(|now|now.saturating_add(wall));let mut spent=0u64;
    loop {
        let grant=if request.node_hash==0||request.round_units==0 {budget}else{budget.min(request.round_units.saturating_sub(spent) as usize)};spent=spent.saturating_add(grant as u64);
        match retained.advance(registry,&request.operator_id,grant,4096) {
            Err(error)=>return complete(error_output_json(&error),retained.units),
            Ok(EvaluationStep::Done(output))=>return output,
            Ok(EvaluationStep::Cancelled(progress))=>return EvaluationReply {wire:semio_framework_pack_json::to_string(&object([("done".into(),Value::Bool(true)),("cancellable".into(),Value::Bool(false)),("phase".into(),Value::String("cancelled".into())),("unitsDone".into(),Value::from(progress.units_done as u64)),("unitsTotal".into(),Value::from(progress.units_total as u64)),("outputJson".into(),Value::String(String::new()))])),complete:true,faulted:false},
            Ok(EvaluationStep::Working(progress))=>{
                let expired=deadline.is_none_or(|deadline|semio_framework_job::default_now_us().is_none_or(|now|now>=deadline));
                if request.node_hash==0||(!expired&&(request.round_units==0||spent<request.round_units)) {continue;}
                let envelope=object([("done".into(),Value::Bool(false)),("cancellable".into(),Value::Bool(true)),("phase".into(),Value::String(progress.phase.into())),("unitsDone".into(),Value::from(progress.units_done as u64)),("unitsTotal".into(),Value::from(progress.units_total as u64)),("outputJson".into(),Value::String(String::new()))]);
                retain_evaluation_job(key,retained);return EvaluationReply {wire:semio_framework_pack_json::to_string(&envelope),complete:false,faulted:false};
            },
        }
    }
}

/// 🧾️ Publishes the original physical envelope with facts from its same execution owner.
pub struct EvaluationReply {pub wire:String,pub complete:bool,pub faulted:bool}

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

/// 🎒️ Moves the existing canonical JSON and typed input owners through retained admission.
pub struct EvaluationInputPreparation {
    input:Option<String>,dependency:Option<String>,operator:Option<String>,version:Option<String>,
    parser:Option<semio_framework_pack_json::JsonParseCursor>,projection:Option<semio_framework_pack_json::JsonValueProjection>,
    writer:Option<semio_framework_pack_json::JsonWriteCursor<DslValue>>,encoding:Option<semio_framework_value::native_encoding::NativeEncodeContinuation>,decoding:Option<semio_framework_value::native_decoding::NativeDecodeContinuation>,
    binding:Option<neural_engine::retirement::RetainedDictionaryInput>,dictionary:Option<Dictionary>,
    canonical_input:Option<String>,canonical_dependency:Option<String>,identity:Option<String>,
    retirement:neural_engine::ValueRetirement,stage:u8,units:usize,cancelled:bool,
}

impl EvaluationInputPreparation {
    /// 🌱️ Takes one initial request's source allocation without scanning or copying its payload.
    pub fn new(input:String,dependency:String,operator:String,version:String)->Self {
        Self {input:Some(input),dependency:Some(dependency),operator:Some(operator),version:Some(version),parser:Some(semio_framework_pack_json::JsonParseCursor::new(semio_framework_pack_json::JsonMemberPolicy::Reject)),projection:None,writer:None,encoding:None,decoding:None,binding:None,dictionary:None,canonical_input:None,canonical_dependency:None,identity:None,retirement:Default::default(),stage:0,units:0,cancelled:false}
    }
    /// 🪪️ Exposes the still-owned source allocation for ownership laws.
    pub fn source_ptr(&self)->Option<*const u8> {self.input.as_ref().map(|source|source.as_ptr())}
    /// 📍️ Reports this same candidate's retained input and identity frontier.
    pub fn progress(&self)->(usize,usize,&'static str) {
        let phase=if !self.retirement.terminal_is_empty() {"input-retire"} else {match self.stage {0=>"input-parse",1=>"input-order",2=>"input-canonical",3=>"input-bind",4=>"dependency-parse",5=>"dependency-order",6=>"dependency-canonical",7=>"identity-canonical",_=>"input-retire"}};
        (self.units,self.units.saturating_add(usize::from(self.stage<9)),phase)
    }
    fn write_step(&mut self)->Result<Option<String>,String> {
        let mut admitted=|_|true;
        let mut control=match self.encoding.take() {Some(receipt)=>semio_framework_value::NativeEncodeControl::resume(receipt,&mut admitted),None=>Ok(semio_framework_value::NativeEncodeControl::new(256*1024*1024,&mut admitted))}.map_err(|error|error.to_string())?;
        let result=self.writer.as_mut().unwrap().step(1,&mut control).map_err(|error|error.to_string());
        self.encoding=Some(control.pause().map_err(|error|error.to_string())?);result
    }
    /// ⏱️ Advances shared parsing, ordering, physical writing, typed binding and retirement.
    pub fn step(&mut self,maximum_units:usize,maximum_bytes:usize)->Result<Option<(Dictionary,String)>,String> {
        if maximum_units==0 || maximum_bytes==0 {return Ok(None);}
        if self.cancelled {return Err("evaluation input admission canceled".into());}
        for _ in 0..maximum_units {
            if !self.retirement.terminal_is_empty() {self.retirement.close_step(1,maximum_bytes);}
            else {match self.stage {
                0|4=>{
                    let source=if self.stage==0 {self.input.as_deref().unwrap()}else{self.dependency.as_deref().unwrap()};
                    if source.len()>16*1024*1024 {return Err("evaluation source exceeds admission limit".into());}
                    let mut accepted=|_|true;let mut control=match self.decoding.take(){Some(receipt)=>semio_framework_value::NativeDecodeControl::resume(receipt,&mut accepted),None=>Ok(semio_framework_value::NativeDecodeControl::new(256*1024*1024,&mut accepted))}.map_err(|error|error.to_string())?;
                    let parsed=self.parser.as_mut().unwrap().step(source,1,&mut control);self.decoding=Some(control.pause().map_err(|error|error.to_string())?);
                    match parsed {
                        Ok(Some(value))=>{self.projection=Some(semio_framework_pack_json::JsonValueProjection::new_ordered(value));self.retirement.push_owned(self.parser.take().unwrap());if self.stage==0 {self.retirement.text(self.input.take().unwrap());}else{self.retirement.text(self.dependency.take().unwrap());}self.stage+=1;},
                        Ok(None)=>{},
                        Err(error) if self.stage==4 && error.kind()==semio_framework_value::ValueRefusalKind::InvalidValue=>{self.retirement.push_owned(self.parser.take().unwrap());self.canonical_dependency=self.dependency.take();self.writer=Some(semio_framework_pack_json::JsonWriteCursor::new(DslValue::Array(vec![DslValue::String(self.operator.take().unwrap()),DslValue::String(self.canonical_input.take().unwrap()),DslValue::String(self.canonical_dependency.take().unwrap()),DslValue::String(self.version.take().unwrap())])));self.stage=7;if let semio_framework_pack_json::JsonError::DuplicateMember {name,..}=error {self.retirement.text(name);}},
                        Err(error)=>return Err(error.to_string()),
                    }
                },
                1|5=>{let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::resume(self.decoding.take().unwrap(),&mut accepted).map_err(|error|error.to_string())?;let projected=self.projection.as_mut().unwrap().step(1,maximum_bytes,&mut control);self.decoding=Some(control.pause().map_err(|error|error.to_string())?);if let Some(value)=projected.map_err(|error|error.to_string())? {self.writer=Some(semio_framework_pack_json::JsonWriteCursor::new(value));self.retirement.push_owned(self.projection.take().unwrap());self.stage+=1;}},
                2|6=>{if let Some(text)=self.write_step()? {
                    let mut writer=self.writer.take().unwrap();
                    if self.stage==2 {self.canonical_input=Some(text);self.binding=Some(neural_engine::retirement::RetainedDictionaryInput::new(writer.take_source().unwrap()));self.stage=3;}
                    else {self.canonical_dependency=Some(text);self.stage=7;}
                    self.retirement.push_owned(writer);
                    if self.stage==7 {self.writer=Some(semio_framework_pack_json::JsonWriteCursor::new(DslValue::Array(vec![DslValue::String(self.operator.take().unwrap()),DslValue::String(self.canonical_input.take().unwrap()),DslValue::String(self.canonical_dependency.take().unwrap()),DslValue::String(self.version.take().unwrap())])));}
                }},
                3=>{if let Some(dictionary)=self.binding.as_mut().unwrap().step(1,maximum_bytes).map_err(|error|error.to_string())? {assert!(self.binding.as_ref().unwrap().terminal_is_empty());self.binding=None;self.dictionary=Some(dictionary);self.parser=Some(semio_framework_pack_json::JsonParseCursor::new(semio_framework_pack_json::JsonMemberPolicy::Reject));self.stage=4;}},
                7=>{if let Some(text)=self.write_step()? {self.identity=Some(text);self.retirement.push_owned(self.writer.take().unwrap());self.encoding=None;self.stage=8;}},
                8=>{self.stage=9;self.units=self.units.saturating_add(1);return Ok(Some((self.dictionary.take().unwrap(),self.identity.take().unwrap())));},
                _=>return Ok(None),
            }}
            self.units=self.units.saturating_add(1);
        }
        Ok(None)
    }
    /// 🛑️ Marks the current candidate for close without dropping a partial frontier.
    pub fn cancel(&mut self) {self.cancelled=true;if let Some(binding)=&mut self.binding {binding.cancel();}}
    /// ♻️ Retires exact remaining source, canonical and typed candidates under the same byte grant.
    pub fn close_step(&mut self,maximum_units:usize,maximum_bytes:usize)->neural_engine::ValueRetirementStep {
        use neural_engine::ValueRetirementStep;
        if maximum_units==0 || maximum_bytes==0 {return ValueRetirementStep::Blocked;}
        self.cancel();
        if !self.retirement.terminal_is_empty() {return self.retirement.close_step(1,maximum_bytes);}
        if let Some(binding)=&mut self.binding {let step=binding.close_step(1,maximum_bytes);if binding.terminal_is_empty(){self.binding=None;}return if matches!(step,ValueRetirementStep::Complete) {ValueRetirementStep::Pending {released_items:1,released_bytes:0}}else{step};}
        if let Some(value)=self.parser.take() {self.retirement.push_owned(value);}
        else if let Some(value)=self.projection.take() {self.retirement.push_owned(value);}
        else if let Some(value)=self.writer.take() {self.retirement.push_owned(value);self.encoding=None;}
        else if let Some(value)=self.dictionary.take() {self.retirement.push_dictionary(value);}
        else if let Some(value)=self.input.take().or_else(||self.dependency.take()).or_else(||self.operator.take()).or_else(||self.version.take()).or_else(||self.canonical_input.take()).or_else(||self.canonical_dependency.take()).or_else(||self.identity.take()) {self.retirement.text(value);}
        else {self.stage=9;return ValueRetirementStep::Complete;}
        ValueRetirementStep::Pending {released_items:1,released_bytes:0}
    }
    /// 🔒️ Confirms all private request frontiers have moved or reached terminal-empty.
    pub fn terminal_is_empty(&self)->bool {self.input.is_none()&&self.dependency.is_none()&&self.operator.is_none()&&self.version.is_none()&&self.parser.is_none()&&self.projection.is_none()&&self.writer.is_none()&&self.binding.is_none()&&self.dictionary.is_none()&&self.canonical_input.is_none()&&self.canonical_dependency.is_none()&&self.identity.is_none()&&self.retirement.terminal_is_empty()}
}
impl Drop for EvaluationInputPreparation {fn drop(&mut self) {assert!(std::thread::panicking()||self.terminal_is_empty(),"evaluation input candidate dropped before terminal-empty");}}
struct EvaluationInputRetirement(EvaluationInputPreparation);
impl semio_framework_value::retirement::RetirementCursor for EvaluationInputRetirement {
    fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep {
        use semio_framework_value::retirement::RetirementStep;
        if grant.maximum_items==0{return RetirementStep::BudgetExhausted;}
        let step=self.0.close_step(1,grant.maximum_copy_bytes);if self.0.terminal_is_empty(){return RetirementStep::Complete;}
        match step {neural_engine::ValueRetirementStep::Pending {released_bytes,..}=>RetirementStep::ProcessedBytes(released_bytes),_=>RetirementStep::BudgetExhausted}
    }
    fn terminal_is_empty(&self)->bool {self.0.terminal_is_empty()}
    fn next_work_byte_demand(&self)->usize {usize::from(!self.terminal_is_empty())}
}
impl semio_framework_value::retirement::RetireOwned for EvaluationInputPreparation {
    fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor> {Box::new(EvaluationInputRetirement(self))}
}


/// ⏱️ Retained resumable evaluations keyed by `(operatorId, nodeHash)`. Bounded: a new job past the
/// ceiling refuses admission until an existing owner finishes retirement.
const EVALUATION_JOB_CAPACITY: usize = 16;

enum EvaluationStep {Working(neural_engine::OperatorProgress),Done(EvaluationReply),Cancelled(neural_engine::OperatorProgress)}

struct EvaluationOutput {
    writer:Option<semio_framework_pack_json::JsonWriteCursor<Dictionary>>,envelope:Option<semio_framework_pack_json::JsonWriteCursor<DslValue>>,
    encoding:Option<semio_framework_value::native_encoding::NativeEncodeContinuation>,retirement:neural_engine::ValueRetirement,
    text:Option<String>,wire:Option<String>,stage:u8,units:usize,faulted:bool,
}
impl EvaluationOutput {
    fn new(output:Dictionary)->Self {let faulted=output.get("error").is_some();Self {writer:Some(semio_framework_pack_json::JsonWriteCursor::new(output)),envelope:None,encoding:None,retirement:Default::default(),text:None,wire:None,stage:0,units:0,faulted}}
    fn phase(&self)->&'static str {match self.stage {0=>if self.writer.as_ref().unwrap().progress().1 {"output-write"}else{"output-measure"},1=>"output-retire",2=>if self.envelope.as_ref().unwrap().progress().1 {"output-envelope-write"}else{"output-envelope-measure"},3=>"output-envelope-retire",_=>"complete"}}
    fn step(&mut self,maximum_units:usize,maximum_bytes:usize,base_units:usize)->Result<Option<EvaluationReply>,String> {
        if maximum_units==0||maximum_bytes==0 {return Ok(None);}
        for _ in 0..maximum_units {
            match self.stage {
                0|2=>{
                    let mut admitted=|_|true;let mut control=match self.encoding.take() {Some(receipt)=>semio_framework_value::NativeEncodeControl::resume(receipt,&mut admitted),None=>Ok(semio_framework_value::NativeEncodeControl::new(256*1024*1024,&mut admitted))}.map_err(|error|error.to_string())?;
                    let result=if self.stage==0 {self.writer.as_mut().unwrap().step(1,&mut control)}else{self.envelope.as_mut().unwrap().step(1,&mut control)};self.encoding=Some(control.pause().map_err(|error|error.to_string())?);
                    if self.stage==0 {self.units=self.units.saturating_add(1);}
                    if let Some(text)=result.map_err(|error|error.to_string())? {
                        if self.stage==0 {let mut writer=self.writer.take().unwrap();self.retirement.push_dictionary(writer.take_source().unwrap());self.retirement.push_owned(writer);self.text=Some(text);self.stage=1;}
                        else {let mut writer=self.envelope.take().unwrap();self.retirement.push_owned(writer.take_source().unwrap());self.retirement.push_owned(writer);self.wire=Some(text);self.stage=3;}
                    }
                },
                1|3=>{
                    if !self.retirement.terminal_is_empty(){self.retirement.close_step(1,maximum_bytes);if self.stage==1 {self.units=self.units.saturating_add(1);}continue;}
                    if self.stage==1 {let units=base_units.saturating_add(self.units);self.envelope=Some(semio_framework_pack_json::JsonWriteCursor::new(DslValue::object([("cancellable".into(),DslValue::Bool(false)),("done".into(),DslValue::Bool(true)),("outputJson".into(),DslValue::String(self.text.take().unwrap())),("phase".into(),DslValue::String("complete".into())),("unitsDone".into(),DslValue::Number(semio_framework_value::Number::UInt(units as u64))),("unitsTotal".into(),DslValue::Number(semio_framework_value::Number::UInt(units as u64)))])));self.stage=2;}
                    else {self.stage=4;return Ok(Some(EvaluationReply {wire:self.wire.take().unwrap(),complete:true,faulted:self.faulted}));}
                },
                _=>return Ok(None),
            }
        }
        Ok(None)
    }
    fn close_step(&mut self,maximum_bytes:usize)->neural_engine::ValueRetirementStep {
        use neural_engine::ValueRetirementStep;
        if maximum_bytes==0 {return ValueRetirementStep::Blocked;}
        if !self.retirement.terminal_is_empty(){return self.retirement.close_step(1,maximum_bytes);}
        if let Some(writer)=self.writer.take(){self.retirement.push_owned(writer);}
        else if let Some(writer)=self.envelope.take(){self.retirement.push_owned(writer);}
        else if let Some(text)=self.text.take().or_else(||self.wire.take()){self.retirement.text(text);}
        else {return ValueRetirementStep::Complete;}
        ValueRetirementStep::Pending {released_items:1,released_bytes:0}
    }
    fn terminal_is_empty(&self)->bool {self.writer.is_none()&&self.envelope.is_none()&&self.text.is_none()&&self.wire.is_none()&&self.retirement.terminal_is_empty()}
}
impl Drop for EvaluationOutput {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"evaluation output dropped before terminal-empty");}}

struct RetainedEvaluation {
    registry:neural_engine::SharedRegistry,
    job:Option<Box<dyn neural_engine::OperatorJob>>,admission:Option<EvaluationInputPreparation>,replacement:Option<EvaluationInputPreparation>,candidate:Option<(Dictionary,String)>,
    dependency_identity:String,version:String,cancellation_id:String,next_version:Option<String>,next_cancellation:Option<String>,
    retirement:neural_engine::ValueRetirement,finished:Option<Dictionary>,output:Option<EvaluationOutput>,fault:Option<String>,
    units:usize,identity_position:Option<usize>,cancelled:bool,restarting:bool,
}
impl RetainedEvaluation {
    fn new(registry:neural_engine::SharedRegistry,input:String,dependency:String,operator:String,version:String,cancellation_id:String)->Self {
        Self {registry,job:None,admission:Some(EvaluationInputPreparation::new(input,dependency,operator,version.clone())),replacement:None,candidate:None,dependency_identity:String::new(),version,cancellation_id,next_version:None,next_cancellation:None,retirement:Default::default(),finished:None,output:None,fault:None,units:0,identity_position:None,cancelled:false,restarting:false}
    }
    fn progress(&self)->neural_engine::OperatorProgress {
        let (done,total,phase)=if self.cancelled {(self.units,self.units.saturating_add(1),if self.restarting {"retire-invalidated"}else{"input-retire"})}
        else if !self.retirement.terminal_is_empty() {(self.units,self.units.saturating_add(1),"input-retire")}
        else if let Some(admission)=&self.admission {let (done,total,phase)=admission.progress();(self.units.saturating_add(done),self.units.saturating_add(total),phase)}
        else if let Some(job)=&self.job {let progress=job.progress();(self.units.saturating_add(progress.units_done),self.units.saturating_add(progress.units_total),progress.phase)}
        else if let Some(output)=&self.output {(self.units.saturating_add(output.units),self.units.saturating_add(output.units).saturating_add(1),output.phase())}
        else {(self.units,self.units.saturating_add(1),if self.identity_position.is_some(){"identity-compare"}else{"input-admit"})};
        neural_engine::OperatorProgress {units_done:done,units_total:total,phase}
    }
    fn cancel(&mut self) {
        if !self.cancelled {if let Some(output)=&mut self.output {self.units=self.units.saturating_add(output.units);output.units=0;}}
        self.cancelled=true;self.restarting=false;if let Some(admission)=&mut self.admission {admission.cancel();}if let Some(job)=&mut self.job {job.cancel();}if let Some(fault)=self.fault.take(){self.retirement.text(fault);}
    }
    fn close_step(&mut self,maximum_units:usize,maximum_bytes:usize,preserve_candidate:bool)->neural_engine::ValueRetirementStep {
        use neural_engine::ValueRetirementStep;
        let transferred=ValueRetirementStep::Pending {released_items:1,released_bytes:0};
        if maximum_units==0||maximum_bytes==0{return ValueRetirementStep::Blocked;}
        if !self.retirement.terminal_is_empty() {return self.retirement.close_step(1,maximum_bytes);}
        if let Some(admission)=&mut self.admission {let step=admission.close_step(1,maximum_bytes);if admission.terminal_is_empty(){self.admission=None;}return if matches!(step,ValueRetirementStep::Complete){transferred}else{step};}
        if !preserve_candidate {if let Some(replacement)=&mut self.replacement {let step=replacement.close_step(1,maximum_bytes);if replacement.terminal_is_empty(){self.replacement=None;}return if matches!(step,ValueRetirementStep::Complete){transferred}else{step};}}
        if let Some(job)=&mut self.job {match job.close_step(1,maximum_bytes) {Ok(neural_engine::OperatorJobStep::Working(_))=>{},Ok(neural_engine::OperatorJobStep::Done(value))=>{self.retirement.push_dictionary(value);self.job=None;},_=>self.job=None};return transferred;}
        if let Some(value)=self.finished.take(){self.retirement.push_dictionary(value);return transferred;}
        if let Some(output)=&mut self.output {let step=output.close_step(maximum_bytes);if matches!(step,ValueRetirementStep::Complete){self.output=None;return transferred;}return step;}
        if !self.dependency_identity.is_empty(){self.retirement.text(std::mem::take(&mut self.dependency_identity));return transferred;}
        if !self.version.is_empty(){self.retirement.text(std::mem::take(&mut self.version));return transferred;}
        if !self.cancellation_id.is_empty(){self.retirement.text(std::mem::take(&mut self.cancellation_id));return transferred;}
        if !preserve_candidate {
            if let Some((input,identity))=self.candidate.take(){self.retirement.push_dictionary(input);self.retirement.text(identity);return transferred;}
            if let Some(version)=self.next_version.take(){self.retirement.text(version);return transferred;}
            if let Some(id)=self.next_cancellation.take(){self.retirement.text(id);return transferred;}
        }
        ValueRetirementStep::Complete
    }
    fn advance(&mut self,registry:&Registry,operator:&str,maximum_units:usize,maximum_bytes:usize)->Result<EvaluationStep,String> {
        use neural_engine::OperatorJobStep;
        if maximum_units==0||maximum_bytes==0{return Ok(EvaluationStep::Working(self.progress()));}
        if self.cancelled {
            if matches!(self.close_step(maximum_units,maximum_bytes,self.restarting),neural_engine::ValueRetirementStep::Complete) {
                if let Some(fault)=self.fault.take(){return Err(fault);}
                if self.restarting {if self.replacement.is_some(){self.admission=self.replacement.take();}self.version=self.next_version.take().unwrap();self.cancellation_id=self.next_cancellation.take().unwrap();self.cancelled=false;self.restarting=false;self.units=0;self.identity_position=None;return Ok(EvaluationStep::Working(neural_engine::OperatorProgress {units_done:0,units_total:0,phase:"restart-admission"}));}
                return Ok(EvaluationStep::Cancelled(neural_engine::OperatorProgress {units_done:self.units,units_total:self.units,phase:"cancelled"}));
            }
            return Ok(EvaluationStep::Working(self.progress()));
        }
        if !self.retirement.terminal_is_empty(){self.retirement.close_step(1,maximum_bytes);return Ok(EvaluationStep::Working(self.progress()));}
        if let Some(admission)=&mut self.admission {
            let result=admission.step(maximum_units,maximum_bytes);
            match result {Ok(Some(value))=>{self.units=self.units.saturating_add(admission.progress().0);assert!(admission.terminal_is_empty());self.admission=None;self.candidate=Some(value);if self.job.is_some(){self.identity_position=Some(0);}},Ok(None)=>{},Err(error)=>{self.units=self.units.saturating_add(admission.progress().0);self.fault=Some(error);self.cancelled=true;if let Some(job)=&mut self.job{job.cancel();}}}
            return Ok(EvaluationStep::Working(self.progress()));
        }
        if let Some(position)=self.identity_position {
            let identity=&self.candidate.as_ref().unwrap().1;
            let changed=identity.len()!=self.dependency_identity.len()||(position<identity.len()&&identity.as_bytes()[position]!=self.dependency_identity.as_bytes()[position]);
            if changed {self.job.as_mut().unwrap().cancel();self.cancelled=true;self.restarting=true;self.identity_position=None;}
            else if position==identity.len() {let (input,identity)=self.candidate.take().unwrap();self.retirement.push_dictionary(input);self.retirement.text(identity);if let Some(version)=self.next_version.take(){self.retirement.text(std::mem::replace(&mut self.version,version));}if let Some(id)=self.next_cancellation.take(){self.retirement.text(std::mem::replace(&mut self.cancellation_id,id));}self.identity_position=None;}
            else {self.identity_position=Some(position+1);}
            self.units=self.units.saturating_add(1);return Ok(EvaluationStep::Working(self.progress()));
        }
        if let Some((input,identity))=self.candidate.take() {
            self.dependency_identity=identity;let input=ColdOwner::new(match registry.operator_info(operator){Some(info)=>inject_channel_defaults(input,info),None=>input});
            match registry.dispatch_job(operator,&input) {Ok(Some(job))=>self.job=Some(job),Ok(None)=>match registry.dispatch(operator,&input){Ok(output)=>self.finished=Some(output),Err(error)=>{self.fault=Some(error.to_string());self.cancelled=true;}},Err(error)=>{self.fault=Some(error.to_string());self.cancelled=true;}}
            self.retirement.push_dictionary(input.into_inner());self.units=self.units.saturating_add(1);
            if self.finished.is_some(){self.retirement.text(std::mem::take(&mut self.dependency_identity));}
            return Ok(EvaluationStep::Working(self.progress()));
        }
        if let Some(job)=&mut self.job {
            match job.step(maximum_units) {
                Ok(OperatorJobStep::Working(progress))=>return Ok(EvaluationStep::Working(neural_engine::OperatorProgress {units_done:self.units.saturating_add(progress.units_done),units_total:self.units.saturating_add(progress.units_total),phase:progress.phase})),
                Ok(OperatorJobStep::Done(value))=>{self.units=self.units.saturating_add(job.progress().units_total);self.finished=Some(value);self.job=None;self.retirement.text(std::mem::take(&mut self.dependency_identity));},
                Ok(OperatorJobStep::Cancelled(_))=>{self.cancelled=true;self.job=None;},
                Err(error)=>{self.fault=Some(error.to_string());self.cancelled=true;job.cancel();},
            }
            return Ok(EvaluationStep::Working(self.progress()));
        }
        if let Some(output)=self.finished.take(){match registry.finish_job(operator,output){Ok(output)=>self.output=Some(EvaluationOutput::new(output)),Err(error)=>{self.fault=Some(error.to_string());self.cancelled=true;}}return Ok(EvaluationStep::Working(self.progress()));}
        if let Some(output)=&mut self.output {
            match output.step(maximum_units,maximum_bytes,self.units) {Ok(Some(wire))=>{assert!(output.terminal_is_empty());self.output=None;return Ok(EvaluationStep::Done(wire));},Ok(None)=>{},Err(error)=>{self.fault=Some(error);self.cancelled=true;}}
        }
        Ok(EvaluationStep::Working(self.progress()))
    }
}

#[derive(Default)]
struct EvaluationJobRegistry {jobs:std::collections::HashMap<(neural_engine::RegistryIdentity,String,u64),RetainedEvaluation>}

static EVALUATION_JOBS: std::sync::OnceLock<std::sync::Mutex<EvaluationJobRegistry>> = std::sync::OnceLock::new();

fn evaluation_jobs() -> &'static std::sync::Mutex<EvaluationJobRegistry> {
    EVALUATION_JOBS.get_or_init(|| std::sync::Mutex::new(EvaluationJobRegistry::default()))
}

/// 🧬️ Canonical dependency identity keeps source, wiring, parameters and operator version together.
pub fn evaluation_dependency_identity(request: &EvaluateRequest) -> String {
    let canonical=|text:&str| {
        let value=match semio_framework_pack_json::parse(text,semio_framework_pack_json::JsonMemberPolicy::Reject) {Ok(value)=>value,Err(_)=>return text.to_string()};
        let mut projection=semio_framework_pack_json::JsonValueProjection::new_ordered(value);
        let mut accepted=|_|true;let mut decode=semio_framework_value::NativeDecodeControl::new(usize::MAX,&mut accepted);let value=loop {if let Some(value)=projection.step(4096,usize::MAX,&mut decode).expect("canonical identity projection"){break value;}};
        let mut writer=semio_framework_pack_json::JsonWriteCursor::new(value);let mut admitted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(usize::MAX,&mut admitted);
        let text=loop {if let Some(text)=writer.step(4096,&mut control).expect("canonical identity writer"){break text;}};
        let mut retirement=neural_engine::ValueRetirement::default();retirement.push_owned(projection);retirement.push_owned(writer);while !retirement.terminal_is_empty(){retirement.close_step(4096,usize::MAX);}text
    };
    semio_framework_pack_json::to_json_string(&DslValue::Array(vec![request.operator_id.to_value(),DslValue::String(canonical(&request.input_json)),DslValue::String(canonical(&request.dependency_json)),request.operator_version.to_value()]))
}

/// 🅿️ Parks this same admitted source, job or retirement frontier for the next round trip.
fn retain_evaluation_job(key:(neural_engine::RegistryIdentity,String,u64),retained:RetainedEvaluation) {
    let mut registry=evaluation_jobs().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    assert!(registry.jobs.len()<EVALUATION_JOB_CAPACITY||registry.jobs.contains_key(&key),"retained evaluations require admitted capacity");registry.jobs.insert(key,retained);
}

/// 🛑️ Retires retained work by the named inference cancellation identity.
pub fn cancel_inference_evaluation(owner:&neural_engine::SharedRegistry,cancellation_id: &str) -> bool {
    let mut accepted=false;
    if let Ok(mut registry)=evaluation_jobs().lock() {for retained in registry.jobs.values_mut().filter(|retained|retained.registry.owner_identity()==owner.owner_identity()&&retained.cancellation_id==cancellation_id && !retained.cancelled) {retained.cancel();accepted=true;}}
    retire_cancelled_evaluations_step(owner,EVALUATE_STEP_BUDGET);accepted
}

/// 🛑️ Acknowledges cancellation while retaining the same job until owned cleanup completes.
pub fn cancel_evaluation(owner:&neural_engine::SharedRegistry,operator_id: &str, node_hash: u64) -> bool {
    let accepted=if let Ok(mut registry)=evaluation_jobs().lock() {match registry.jobs.get_mut(&(owner.owner_identity(),operator_id.to_string(),node_hash)) {Some(retained) if !retained.cancelled=>{retained.cancel();true},_=>false}}else {false};
    retire_cancelled_evaluations_step(owner,EVALUATE_STEP_BUDGET);accepted
}

/// 🛑️ Acknowledges every active job without dropping pending retirement.
pub fn cancel_all_evaluations(owner:&neural_engine::SharedRegistry) -> usize {
    let mut accepted=0;
    if let Ok(mut registry)=evaluation_jobs().lock() {for retained in registry.jobs.values_mut().filter(|retained|retained.registry.owner_identity()==owner.owner_identity()&&!retained.cancelled) {retained.cancel();accepted+=1;}}
    retire_cancelled_evaluations_step(owner,EVALUATE_STEP_BUDGET);accepted
}

/// 🧹️ Advances cancelled entries of the existing map by at most the supplied work grant.
pub fn retire_cancelled_evaluations_step(owner:&neural_engine::SharedRegistry,maximum_units:usize)->neural_engine::ValueRetirementStep {retire_cancelled_evaluations_close_step(owner,maximum_units,4096)}

/// 🧹️ Advances the existing cancelled-job map with explicit byte credit.
pub fn retire_cancelled_evaluations_close_step(owner:&neural_engine::SharedRegistry,maximum_units:usize,maximum_bytes:usize)->neural_engine::ValueRetirementStep {
    use neural_engine::ValueRetirementStep;
    if maximum_units==0||maximum_bytes==0{return ValueRetirementStep::Blocked;}
    let Ok(mut registry)=evaluation_jobs().lock() else {return ValueRetirementStep::Blocked};
    let Some(key)=registry.jobs.iter().find(|(_,retained)|retained.registry.owner_identity()==owner.owner_identity()&&retained.cancelled).map(|(key,_)|key.clone()) else {return ValueRetirementStep::Complete};
    let step=registry.jobs.get_mut(&key).expect("retained cancellation").close_step(1,maximum_bytes,false);
    if matches!(step,ValueRetirementStep::Complete) {registry.jobs.remove(&key);return ValueRetirementStep::Pending {released_items:1,released_bytes:0};}
    step
}

/// 🧹️ Reports whether acknowledged cancellation still owns parked resources.
pub fn evaluation_retirement_pending(owner:&neural_engine::SharedRegistry)->bool {evaluation_jobs().lock().map_or(true,|registry|registry.jobs.values().any(|retained|retained.registry.owner_identity()==owner.owner_identity()&&retained.cancelled))}

/// 📈️ Progress of the parked evaluation of `operator_id` at `node_hash`, if one is retained.
pub fn evaluation_progress(owner:&neural_engine::SharedRegistry,operator_id: &str, node_hash: u64) -> Option<neural_engine::OperatorProgress> {
    retire_cancelled_evaluations_step(owner,1);
    let registry = evaluation_jobs().lock().ok()?;
    registry.jobs.get(&(owner.owner_identity(),operator_id.to_string(), node_hash)).map(|retained| retained.progress())
}

/// 🔌️ Retains the original extension registry and its existing exact retirement authority.
pub struct ExtensionEvaluationResources {registry:Option<neural_engine::SharedRegistry>,retirement:neural_engine::RegistryRetirement}
impl ExtensionEvaluationResources {
    pub fn new(registry:Registry)->Self {let (registry,retirement)=neural_engine::SharedRegistry::new(registry);Self {registry:Some(registry),retirement}}
    pub fn registry(&self)->&neural_engine::SharedRegistry {self.registry.as_ref().expect("open extension evaluation owner")}
}
impl semio_framework_plugin::ExtensionResourceOwner for ExtensionEvaluationResources {
    fn next_close_byte_demand(&self)->usize {self.retirement.next_close_byte_demand().max(1)}
    fn invoke(&self,capability:&str,request:&[u8])->Result<Vec<u8>,semio_framework::Fault> {
        match capability {"evaluate"=>evaluate_invoke_json(self.registry(),request).map_err(|message|semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin,semio_framework::FaultCode::new("extension.evaluate.bad-request"),message)),_=>Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin,semio_framework::FaultCode::new("extension.unknown-capability"),"unknown flow evaluation capability"))}
    }
    fn begin_close(&mut self) {if let Some(registry)=&self.registry {cancel_all_evaluations(registry);}}
    fn close_step(&mut self,items:usize,bytes:usize)->Result<semio_framework_plugin::PluginCloseStep,semio_framework::Fault> {
        use semio_framework_plugin::PluginCloseStep as Step;
        if self.registry.is_none()&&self.retirement.terminal_is_empty(){return Ok(Step::Complete);}
        if items==0||bytes==0{return Ok(Step::Pending {released_items:0,released_bytes:0});}
        if let Some(registry)=&self.registry {
            if evaluation_retirement_pending(registry){return Ok(match retire_cancelled_evaluations_close_step(registry,1,bytes) {neural_engine::ValueRetirementStep::Pending {released_items,released_bytes}=>Step::Pending {released_items,released_bytes},neural_engine::ValueRetirementStep::Blocked=>Step::Blocked {reason:"original evaluation awaits its child retirement byte grant"},neural_engine::ValueRetirementStep::Complete=>Step::Pending {released_items:0,released_bytes:0}});}
            drop(self.registry.take());return Ok(Step::Pending {released_items:1,released_bytes:0});
        }
        self.retirement.close_step(items,bytes).map(|step|match step {neural_engine::ValueRetirementStep::Pending {released_items,released_bytes}=>Step::Pending {released_items,released_bytes},neural_engine::ValueRetirementStep::Blocked=>Step::Blocked {reason:"original flow registry awaits source readers or its allocator byte grant"},neural_engine::ValueRetirementStep::Complete=>Step::Complete}).map_err(|message|semio_framework::Fault::new(semio_framework::FaultOrigin::Plugin,semio_framework::FaultCode::new("extension.registry-close"),message))
    }
    fn terminal_is_empty(&self)->bool {self.registry.is_none()&&self.retirement.terminal_is_empty()}
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
