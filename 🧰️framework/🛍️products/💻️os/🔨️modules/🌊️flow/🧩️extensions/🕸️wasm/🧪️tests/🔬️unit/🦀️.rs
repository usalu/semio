
use super::*;

/// 🪪️ The original raw recipient pays grammar effects and closes its same partial owner.
#[test]
fn original_sdk_raw_request_keeps_same_caller_source_and_actual_receipt(){
 use semio_framework_job::{OperationId,Generation,StepBudget,StepContext,root_cancel_token};use semio_framework_plugin::ExtensionResourceOwner;
 for copy in [1,3,64]{
  let grant=RetainedCloneGrant{maximum_copy_bytes:copy,..SDK_NORMAL_TEST_POLICY};
  let request=serde_json::to_vec(&serde_json::json!({"neuronId":"tree.raw.雪","operatorId":"test.original","inputJson":"same source 雪".repeat(128),"retained":grant})).unwrap();let oracle:serde_json::Value=serde_json::from_slice(&request).unwrap();assert_eq!(oracle["neuronId"],"tree.raw.雪");let pointer=request.as_ptr();
  let mut owner=ExtensionEvaluationResources::new(Registry::new());let cancel=root_cancel_token();let mut sequence=0;
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let mut physical=Default::default();let mut cx=StepContext::new(OperationId(17),Generation(9),StepBudget::new(1,u64::MAX,denied),cancel.clone(),||Some(1),&mut sequence,&mut physical);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||evaluate_invoke_json(&owner,&request,&mut cx));let reply=result.unwrap();assert!(reply.payload.is_none());assert!(reply.refusal.is_none());assert_eq!(reply.retained_progress,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(owner.invoke.lock().unwrap().is_none());}
  for _ in 0..16{let mut physical=Default::default();let mut cx=StepContext::new(OperationId(17),Generation(9),StepBudget::new(1,u64::MAX,grant),cancel.clone(),||Some(1),&mut sequence,&mut physical);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||evaluate_invoke_json(&owner,&request,&mut cx));let reply=result.unwrap();assert!(reply.payload.is_none());assert!(reply.refusal.is_none());assert_eq!(reply.retained_progress,cx.retained_progress());assert!(reply.retained_progress.fits(grant));assert_eq!((reply.retained_progress.retained_capacity_bytes,reply.retained_progress.released_bytes),(heap.requested_bytes,heap.released_bytes));assert_eq!(request.as_ptr(),pointer);assert!(owner.invoke.lock().unwrap().as_ref().unwrap().matches(17,9,&request));}
  owner.begin_close();let mut turns=0;while !owner.terminal_is_empty(){turns+=1;assert!(turns<250000);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant));let receipt=result.unwrap().progress().unwrap();assert!(receipt.fits(grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));}let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  eprintln!("[DEBUG] actual raw SDK sameCaller=true sameSource=true copy={copy} pendingNone=true zeroHeap0=true receiptOnce=true fullCloseTurns={turns} terminalDrop0=true");
 }
}

/// 🚪️ The actual gateway keeps caller bytes while every partial grammar owner closes.
#[test]
fn original_inference_gateway_keeps_same_request_and_actual_receipt(){
 use semio_framework_job::{OperationId,Generation,StepBudget,StepContext,root_cancel_token};
 for copy in [1,3,64]{
  let grant=RetainedCloneGrant{maximum_copy_bytes:copy,..SDK_NORMAL_TEST_POLICY};
  let request=serde_json::to_vec(&serde_json::json!({"owner":"same source 雪".repeat(128),"canonicalPayload":[1,2,3],"retained":grant})).unwrap();let oracle:serde_json::Value=serde_json::from_slice(&request).unwrap();assert_eq!(oracle["canonicalPayload"],serde_json::json!([1,2,3]));let pointer=request.as_ptr();
  let (mut gateway,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_plugin::ArtifactInferenceGateway::new(17,9,&request));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let cancel=root_cancel_token();let mut sequence=0;
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let mut physical=Default::default();let mut cx=StepContext::new(OperationId(17),Generation(9),StepBudget::new(1,u64::MAX,denied),cancel.clone(),||Some(1),&mut sequence,&mut physical);let(reply,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||gateway.step(&request,&mut cx,None));assert!(reply.payload.is_none());assert!(reply.refusal.is_none());assert_eq!(reply.retained_progress,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  for _ in 0..16{let mut physical=Default::default();let mut cx=StepContext::new(OperationId(17),Generation(9),StepBudget::new(1,u64::MAX,grant),cancel.clone(),||Some(1),&mut sequence,&mut physical);let(reply,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||gateway.step(&request,&mut cx,None));assert!(reply.payload.is_none());assert!(reply.refusal.is_none());assert_eq!(reply.retained_progress,cx.retained_progress());assert!(reply.retained_progress.fits(grant));assert_eq!((reply.retained_progress.retained_capacity_bytes,reply.retained_progress.released_bytes),(heap.requested_bytes,heap.released_bytes));assert_eq!(request.as_ptr(),pointer);assert!(gateway.matches(17,9,&request));}
  gateway.begin_close();let mut turns=0;while !gateway.terminal_is_empty(){turns+=1;assert!(turns<250000);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||gateway.close_step(grant));let receipt=result.unwrap().progress();assert!(receipt.fits(grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));}let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(gateway));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  eprintln!("[DEBUG] actual gateway sameCaller=true sameSource=true copy={copy} pendingNone=true constructorHeap0=true receiptOnce=true fullCloseTurns={turns} terminalDrop0=true");
 }
}

const SDK_NORMAL_TEST_POLICY:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:1048576,maximum_release_bytes:256*1024*1024,maximum_depth:1024};
use neural_engine::{Atom, ChannelSpec, EvalError, Operator, OperatorImpl, OperatorInfo, Value, channel_output};

struct Echo;

impl Operator for Echo {
    fn step_plan(&self,input:Dictionary,grant:RetainedCloneGrant)->Result<(neural_engine::OperatorPlanAdmission,RetainedCloneProgress),(EvalError,Dictionary)>{neural_engine::OperatorPlanAdmission::immediate(input,grant)}
    fn next_plan_copy_byte_demand(&self,_input:&Dictionary)->Result<usize,ValueError>{Ok(std::mem::size_of::<Dictionary>())}
    fn next_plan_capacity_byte_demand(&self,_input:&Dictionary,_copy:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_plan_release_byte_demand(&self,_input:&Dictionary)->Result<usize,ValueError>{Ok(0)}
    fn next_plan_depth_demand(&self,_input:&Dictionary)->Result<usize,ValueError>{Ok(1)}

    fn evaluate(&self, input: &Dictionary) -> Result<Dictionary, EvalError> {
        Ok(channel_output("x", input.clone()))
    }
}

#[test]
fn manifest_lists_catalogue() {
    let mut reg = Registry::new();
    reg.register_operator(
        OperatorInfo {
            id: "test.echo".into(),
            extension: "test".into(),
            name: "Echo".into(),
            abbreviation: "Echo".into(),
            icon: "emoji:📣️".into(),
            summary: "Echo".into(),
            inputs: vec![ChannelSpec::any("x")],
            outputs: vec![ChannelSpec::named("X", "x", "x", "Echoed")],
            ..Default::default()
        },
        vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }],
        &[],
    );
    let json = build_manifest_json("test", "Test", "0.1.0", &reg, vec!["onStartup".into()], vec![], vec![], vec![]);
    assert!(json.contains("flow.extension"));
    assert!(json.contains("test.echo"));
}

#[test]
fn evaluate_round_trips_dictionary() {
    let mut reg = Registry::new();
    reg.register_operator(
        OperatorInfo {
            id: "test.echo".into(),
            extension: "test".into(),
            name: "Echo".into(),
            abbreviation: "Echo".into(),
            icon: "emoji:📣️".into(),
            summary: "Echo".into(),
            inputs: vec![ChannelSpec::any("x")],
            outputs: vec![ChannelSpec::named("X", "x", "x", "Echoed")],
            ..Default::default()
        },
        vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }],
        &[],
    );
    let input = Dictionary::new().insert("number", Value::Atom(Atom::Decimal(2.0)));
    let out_json = evaluate_json(&reg, "test.echo", &semio_framework_pack_json::to_json_string(&input));
    let out: Dictionary = semio_framework_pack_json::from_json_str(&out_json, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    assert_eq!(out.get("x").and_then(|v| v.as_dictionary()), Some(&input));
    neural_engine::ColdRetire::retire_cold(out);
    neural_engine::ColdRetire::retire_cold(input);
}

// #region 🔁️ExtensionInvocationWire
/// 🔁️ The `extension::invoke` "evaluate" wire, driven by the language-agnostic
/// `🧫️fixtures/🔁️extension-invocation-wire/🔣️.json` the host twin reads in
/// `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`. `outcome` is what the SHELL submits through
/// `PluginExtensionCompletion::complete`: an extension that answered — even with an `{"error": …}`
/// body — is an `ok` completion, and only a request this SDK could not decode is a `fault` one.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ExtensionInvocationWireFixture {
    capability: String,
    note: String,
    request_fields: Vec<String>,
    optional_request_fields: Vec<String>,
    envelope_fields: Vec<String>,
    operator_id: String,
    output_channel: String,
    rows: Vec<ExtensionInvocationWireRow>,
    retained_admission: serde_json::Value,
    retained_output: serde_json::Value,
    terminal_metadata: serde_json::Value,
    retained_close_receipt: RetainedCloseReceiptFixture,
    failed_normal_receipt: serde_json::Value,
    command_retained_authority: serde_json::Value,
    original_inference_close_authority: serde_json::Value,
    original_operator_plan_custody: serde_json::Value,
    original_operator_finish_custody:serde_json::Value,
    original_turn_wallet:serde_json::Value,
    original_typed_failure:serde_json::Value,
    original_request_admission:serde_json::Value,
    original_envelope_source:serde_json::Value,
    original_registry_removal:serde_json::Value,
    original_pending_wire:serde_json::Value,
    original_payload_wire:serde_json::Value,
    neuron_id:serde_json::Value,
    original_gateway_ownership:serde_json::Value,
    original_geometry_preparation:serde_json::Value,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RetainedCloseReceiptFixture {node_hash:u64,label_unit:String,label_repeats:usize,maximum_units:usize,maximum_bytes:usize,maximum_turns:usize,operation_id:String}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ExtensionInvocationWireRow {
    name: String,
    request_json: String,
    outcome: String,
    #[serde(default)]
    done: bool,
    #[serde(default)]
    phase: String,
    #[serde(default)]
    output_keys: Vec<String>,
    #[serde(default)]
    output_empty: bool,
    #[serde(default)]
    fault: Option<String>,
}

struct FixtureRegistry {owner:ExtensionEvaluationResources}
impl std::ops::Deref for FixtureRegistry {type Target=neural_engine::SharedRegistry;fn deref(&self)->&Self::Target {self.owner.registry()}}
impl Drop for FixtureRegistry {fn drop(&mut self){use semio_framework_plugin::ExtensionResourceOwner;self.owner.begin_close();while !self.owner.terminal_is_empty(){self.owner.close_step(SDK_NORMAL_TEST_POLICY).unwrap();}}}
fn fixture_raw_turn(owner:&ExtensionEvaluationResources,request:&[u8])->Result<Vec<u8>,semio_framework::Fault>{
 use semio_framework_job::{OperationId,Generation,StepBudget,StepContext,root_cancel_token};let cancel=root_cancel_token();let mut sequence=0;
 for _ in 0..250000{let mut physical=Default::default();let mut cx=StepContext::new(OperationId(1),Generation(1),StepBudget::new(1,u64::MAX,SDK_NORMAL_TEST_POLICY),cancel.clone(),||Some(1),&mut sequence,&mut physical);let reply=evaluate_invoke_json(owner,request,&mut cx)?;assert_eq!(reply.retained_progress,cx.retained_progress());assert!(reply.refusal.is_none());if let Some(bytes)=reply.payload{return Ok(bytes);}}
 panic!("actual original invocation failed to produce its funded response")
}

fn echo_registry(operator_id: &str, output_channel: &str) -> FixtureRegistry {
    let mut registry = Registry::new();
    registry.register_operator(
        OperatorInfo {
            id: operator_id.into(),
            extension: "test".into(),
            name: "Echo".into(),
            abbreviation: "Echo".into(),
            icon: "emoji:📣️".into(),
            summary: "Echo".into(),
            inputs: vec![ChannelSpec::any(output_channel)],
            outputs: vec![ChannelSpec::named("X", output_channel, output_channel, "Echoed")],
            ..Default::default()
        },
        vec![OperatorImpl { schemas: vec![], operator: Box::new(Echo) }],
        &[],
    );
    FixtureRegistry {owner:ExtensionEvaluationResources::new(registry)}
}

#[test]
fn the_evaluate_wire_answers_every_fixture_row() {
    let fixture: ExtensionInvocationWireFixture = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).expect("the extension-invocation-wire fixture must parse");
    assert_eq!(fixture.capability, "evaluate");
    assert!(!fixture.note.is_empty(), "the fixture states what the wire is");
    assert_eq!(fixture.request_fields, vec!["neuronId".to_string(), "operatorId".to_string(), "inputJson".to_string(), "retained".to_string()], "the REQUIRED request fields");
    for field in ["nodeHash", "budget", "wallMicros"] {
        assert!(fixture.optional_request_fields.iter().any(|declared| declared == field), "the request offers the optional budget field {field}");
    }
    for field in ["done", "phase", "unitsDone", "unitsTotal", "outputJson"] {
        assert!(fixture.envelope_fields.iter().any(|declared| declared == field), "the envelope declares {field}");
    }
    let registry = echo_registry(&fixture.operator_id, &fixture.output_channel);
    for row in &fixture.rows {
        match fixture_raw_turn(&registry.owner, row.request_json.as_bytes()) {
            Ok(bytes) => {
                assert_eq!(row.outcome, "ok", "{}", row.name);
                let text = String::from_utf8(bytes).expect("the answer is UTF-8 JSON");
                // ⏱️ The answer is the BUDGET ENVELOPE; the out dictionary rides inside `outputJson`.
                // A `test.echo` operator offers no resumable job, so it always finishes in the first
                // round trip (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
                let envelope = semio_framework_pack_json::parse(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the envelope decodes");
                assert_eq!(envelope.get("done").and_then(semio_framework_pack_json::Value::as_bool), Some(row.done), "{}: done", row.name);
                assert_eq!(envelope.get("phase").and_then(semio_framework_pack_json::Value::as_str), Some(row.phase.as_str()), "{}: phase", row.name);
                let output_json = envelope.get("outputJson").and_then(semio_framework_pack_json::Value::as_str).expect("outputJson");
                let answer: DslValue = semio_framework_pack_json::from_json_str(output_json, semio_framework_pack_json::JsonMemberPolicy::Reject).expect("the answer decodes");
                let DslValue::Object(entries) = &answer else { panic!("{} must answer an object", row.name) };
                let keys: Vec<String> = entries.iter().map(|(key, _)| key.clone()).collect();
                assert_eq!(keys, row.output_keys, "{}", row.name);
                let body_is_empty = matches!(entries.first().map(|(_, value)| value), Some(DslValue::Object(body)) if body.is_empty());
                assert_eq!(body_is_empty, row.output_empty, "{}", row.name);
            }
            Err(error) => {
                assert_eq!(row.outcome, "fault", "{}", row.name);
                assert_eq!(Some(error.message), row.fault.clone(), "{}", row.name);
            }
        }
    }
}
// #endregion 🔁️ExtensionInvocationWire

#[test]
fn retained_evaluation_input_preserves_canonical_identity_and_bounded_owned_binding() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).unwrap();let law=&fixture["retainedAdmission"];
    let key=law["keyUnit"].as_str().unwrap().repeat(law["keyRepeats"].as_u64().unwrap() as usize);
    let label=law["labelUnit"].as_str().unwrap().repeat(law["labelRepeats"].as_u64().unwrap() as usize);
    let input=serde_json::json!({"z":{"text":label},key:7,"a":{"x":2}});let dependency=serde_json::json!({"z":{"b":2,"a":1},"a":"雪"});
    let input_text=serde_json::to_string(&input).unwrap();let dependency_text=serde_json::to_string(&dependency).unwrap();
    let expected_identity=serde_json::to_string(&serde_json::json!(["test.echo",input_text,dependency_text,"test-v1"])).unwrap();
    let expected:Dictionary=semio_framework_pack_json::from_json_str(&input_text,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let expected=ColdOwner::new(expected);
    let maximum_units=law["maximumUnits"].as_u64().unwrap() as usize;let maximum_bytes=law["maximumBytes"].as_u64().unwrap() as usize;
    let source=input_text.clone();let pointer=source.as_ptr();let mut preparation=EvaluationInputPreparation::new(source,dependency_text.clone(),"test.echo".into(),"test-v1".into());assert_eq!(preparation.source_ptr(),Some(pointer));
    let mut phases=std::collections::BTreeMap::new();let mut turns=0;
    let output=loop {let before=preparation.progress();assert!(preparation.step(0,maximum_bytes,SDK_NORMAL_TEST_POLICY).unwrap().0.is_none());assert!(preparation.step(maximum_units,0,SDK_NORMAL_TEST_POLICY).unwrap().0.is_none());assert_eq!(before,preparation.progress());phases.entry(preparation.progress().2).or_insert(turns);turns+=1;assert!(turns<250000);if let Some(output)=preparation.step(maximum_units,maximum_bytes,SDK_NORMAL_TEST_POLICY).unwrap().0{break output;}};
    assert_eq!(output.1,expected_identity);assert_eq!(output.0,*expected);drop(ColdOwner::new(output.0));assert!(preparation.terminal_is_empty());
    for phase in law["phases"].as_array().unwrap(){assert!(phases.contains_key(phase.as_str().unwrap()),"missing {phase}");}
    let cutoffs:std::collections::BTreeSet<usize>=law["cancelCutoffs"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as usize).chain(phases.values().copied()).collect();
    for cutoff in cutoffs{let mut candidate=EvaluationInputPreparation::new(input_text.clone(),dependency_text.clone(),"test.echo".into(),"test-v1".into());for _ in 0..cutoff{assert!(candidate.step(1,3,SDK_NORMAL_TEST_POLICY).unwrap().0.is_none());}candidate.cancel();let before=candidate.progress();assert_eq!(candidate.close_step(RetainedCloneGrant{maximum_items:0,..SDK_NORMAL_TEST_POLICY}).unwrap().progress(),Default::default());assert_eq!(candidate.close_step(RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,..SDK_NORMAL_TEST_POLICY}).unwrap().progress(),Default::default());assert_eq!(before,candidate.progress());let mut close_turns=0;while !candidate.terminal_is_empty(){close_turns+=1;assert!(close_turns<250000);let step=candidate.close_step(SDK_NORMAL_TEST_POLICY).unwrap();assert!(step.progress().fits(SDK_NORMAL_TEST_POLICY));}}
    let invalid=law["invalidInputText"].as_str().unwrap();assert!(serde_json::from_str::<serde_json::Value>(invalid).unwrap()["bad"].is_array());let mut candidate=EvaluationInputPreparation::new(invalid.into(),dependency_text,"test.echo".into(),"test-v1".into());let mut fault_turns=0;loop {fault_turns+=1;assert!(fault_turns<250000);match candidate.step(1,3,SDK_NORMAL_TEST_POLICY){Ok((None,_))=>{},Err(_)=>break,Ok((Some((value,_)),_))=>{drop(ColdOwner::new(value));panic!("malformed typed input admitted");}}}candidate.cancel();while !candidate.terminal_is_empty(){candidate.close_step(SDK_NORMAL_TEST_POLICY).unwrap();}
    println!("[DEBUG] Existing SDK input owner canonicalizes and binds without source cloning, turns={turns}, phases={phases:?}");
}

#[test]
fn retained_evaluation_terminal_metadata_matches_the_original_physical_wire() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).unwrap();let law=&fixture["terminalMetadata"];let registry=echo_registry("test.echo","x");
    let check=|name:&str,reply:EvaluationReply|{let row=law["rows"].as_array().unwrap().iter().find(|row|row["name"]==name).unwrap();let wire:serde_json::Value=serde_json::from_str(reply.wire.as_deref().expect("original paid response")).unwrap();let faulted=wire["outputJson"].as_str().and_then(|text|serde_json::from_str::<serde_json::Value>(text).ok()).is_some_and(|output|output.get("error").is_some());assert_eq!(reply.complete,wire["done"].as_bool().unwrap());assert_eq!(reply.faulted,faulted);assert_eq!(reply.complete,row["complete"].as_bool().unwrap());assert_eq!(reply.faulted,row["faulted"].as_bool().unwrap());wire};
    for row in fixture["rows"].as_array().unwrap().iter().filter(|row|row["outcome"]=="ok") {let request=semio_framework_pack_json::from_json_str::<EvaluateRequest>(row["requestJson"].as_str().unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();check(row["name"].as_str().unwrap(),evaluate_step_envelope_cold(&registry,request));}
    let hash=law["nodeHash"].as_u64().unwrap();let request=|resume:bool|{let wire=serde_json::json!({"retained":SDK_NORMAL_TEST_POLICY,"neuronId":"tree.fixture.echo","operatorId":"test.echo","inputJson":if resume{""}else{"{\"number\":2}"},"dependencyJson":"","nodeHash":hash,"budget":law["roundUnits"],"roundUnits":law["roundUnits"],"wallMicros":1,"resume":resume});semio_framework_pack_json::from_json_str::<EvaluateRequest>(&serde_json::to_string(&wire).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()};
    check("pending",evaluate_step_envelope_cold(&registry,request(false)));assert!(cancel_evaluation(&registry,"test.echo",hash));let mut turns=0;loop {turns+=1;assert!(turns<law["maximumTurns"].as_u64().unwrap());let reply=evaluate_step_envelope_cold(&registry,request(true));if reply.complete {assert_eq!(check("cancelled",reply)["phase"],"cancelled");break;}check("pending",reply);}assert!(evaluation_progress(&registry,"test.echo",hash).is_none());println!("[DEBUG] Same SDK publication carries terminal metadata for all five independent serde wire outcomes, cancelTurns={turns}");
}

#[test]
fn extension_close_reports_the_original_child_source_retirement_bytes() {
    use semio_framework_plugin::{ExtensionResourceOwner,PluginLifecycleStep};
    let fixture:ExtensionInvocationWireFixture=serde_json::from_str(include_str!("../../🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).unwrap();let law=fixture.retained_close_receipt;
    let source=serde_json::to_string(&serde_json::json!({"text":law.label_unit.repeat(law.label_repeats)})).unwrap();let expected_bytes=source.len();let mut owner=ExtensionEvaluationResources::new(Registry::new());
    let reply=evaluate_step_envelope_cold(owner.registry(),EvaluateRequest {neuron_id:"tree.fixture.echo".into(),retained:SDK_NORMAL_TEST_POLICY,operator_id:"test.close".into(),input_json:source,node_hash:law.node_hash,budget:law.maximum_units as u64,wall_micros:1,dependency_json:String::new(),operator_version:"close-v1".into(),cancellation_id:law.operation_id,round_units:law.maximum_units as u64,resume:false,external_result:None});assert!(!reply.complete);owner.begin_close();
    let before=evaluation_jobs().lock().unwrap().jobs.get(&(owner.registry().owner_identity(),"test.close".into(),law.node_hash)).unwrap().progress();
    assert!(matches!(owner.close_step(RetainedCloneGrant{maximum_items:0,..SDK_NORMAL_TEST_POLICY}).unwrap(),PluginLifecycleStep::Progress(progress) if progress==Default::default()));assert!(matches!(owner.close_step(RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,..SDK_NORMAL_TEST_POLICY}).unwrap(),PluginLifecycleStep::Progress(progress) if progress==Default::default()));
    assert_eq!(before,evaluation_jobs().lock().unwrap().jobs.get(&(owner.registry().owner_identity(),"test.close".into(),law.node_hash)).unwrap().progress());
    let mut charged=0;let mut turns=0;while evaluation_retirement_pending(owner.registry()) {turns+=1;assert!(turns<law.maximum_turns);if let PluginLifecycleStep::Progress(progress)|PluginLifecycleStep::Complete(progress)=owner.close_step(SDK_NORMAL_TEST_POLICY).unwrap(){assert!(progress.fits(SDK_NORMAL_TEST_POLICY));charged+=progress.released_bytes;}}
    while !owner.terminal_is_empty(){turns+=1;assert!(turns<law.maximum_turns);owner.close_step(SDK_NORMAL_TEST_POLICY).unwrap();}
    assert!(charged>=expected_bytes,"original child source bytes were retired without their parent receipt: charged={charged}, expectedAtLeast={expected_bytes}");println!("[DEBUG] Original extension close propagates child source receipt bytes={charged}, sourceBytes={expected_bytes}, turns={turns}");
}

#[test]
fn retained_evaluation_map_owns_initial_input_and_accepts_only_compact_current_resumes() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).unwrap();let law=&fixture["retainedAdmission"];let wire=&law["compactResume"];let registry=echo_registry("test.echo","x");
    let label=law["labelUnit"].as_str().unwrap().repeat(law["labelRepeats"].as_u64().unwrap() as usize);let input=serde_json::json!({"text":label});let text=serde_json::to_string(&input).unwrap();let hash=wire["nodeHash"].as_u64().unwrap();
    let request=|source:&str,resume:bool,version:&str| {let request=serde_json::json!({"retained":SDK_NORMAL_TEST_POLICY,"neuronId":"tree.fixture.echo","operatorId":"test.echo","inputJson":source,"dependencyJson":"","operatorVersion":version,"cancellationId":wire["cancellationId"],"nodeHash":hash,"budget":1,"roundUnits":1,"wallMicros":1,"resume":resume});semio_framework_pack_json::from_json_str::<EvaluateRequest>(&serde_json::to_string(&request).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()};
    let answer=|request:EvaluateRequest|serde_json::from_str::<serde_json::Value>(evaluate_step_envelope_cold(&registry,request).wire.as_deref().expect("original paid response")).unwrap();
    let first=answer(request(&text,false,"test-v1"));assert_eq!(first["done"],false);assert_eq!(first["phase"],wire["initialPhase"]);assert_eq!(first["unitsDone"],1);assert!(evaluation_progress(&registry,"test.echo",hash).is_some());
    let stale=answer(request("",true,"wrong-version"));assert_eq!(stale["done"],true);assert!(stale["outputJson"].as_str().unwrap().contains("error"));assert!(evaluation_progress(&registry,"test.echo",hash).is_some());
    let mut phases=std::collections::BTreeSet::new();let mut turns=0;let completed=loop {let value=answer(request("",true,"test-v1"));turns+=1;assert!(turns<250000);phases.insert(value["phase"].as_str().unwrap().to_string());if value["done"]==true {break value;}};
    let output:serde_json::Value=serde_json::from_str(completed["outputJson"].as_str().unwrap()).unwrap();assert_eq!(output,serde_json::json!({"x":input}));assert!(evaluation_progress(&registry,"test.echo",hash).is_none());
    let orphan=answer(request("",true,"test-v1"));assert_eq!(orphan["done"],true);assert!(orphan["outputJson"].as_str().unwrap().contains("error"));
    assert_eq!(answer(request(&text,false,"test-v1"))["done"],false);let changed=answer(request(&text,false,wire["replacementVersion"].as_str().unwrap()));assert_eq!(changed["done"],false);assert_eq!(changed["phase"],wire["replacementPhases"][0]);let mut replacement_turns=0;loop {replacement_turns+=1;assert!(replacement_turns<250000);let value=answer(request("",true,wire["replacementVersion"].as_str().unwrap()));assert_eq!(value["done"],false);if value["phase"]==wire["replacementPhases"][1] {assert_eq!(value["unitsDone"],wire["restartUnits"][0]);assert_eq!(value["unitsTotal"],wire["restartUnits"][1]);break;}}let fresh=answer(request("",true,wire["replacementVersion"].as_str().unwrap()));assert_eq!(fresh["phase"],wire["initialPhase"]);assert_eq!(fresh["unitsDone"],1);assert!(cancel_evaluation(&registry,"test.echo",hash));while evaluation_retirement_pending(&registry){retire_cancelled_evaluations_close_step(&registry,SDK_NORMAL_TEST_POLICY);}
    for target in law["phases"].as_array().unwrap().iter().map(|value|value.as_str().unwrap()).filter(|phase|!phase.starts_with("dependency-order")&&!phase.starts_with("dependency-canonical")) {let mut value=answer(request(&text,false,"test-v1"));let mut advances=0;while value["phase"]!=target && value["done"]==false {advances+=1;assert!(advances<250000);value=answer(request("",true,"test-v1"));}if value["done"]==true {continue;}assert!(cancel_evaluation(&registry,"test.echo",hash));let mut closes=0;while evaluation_retirement_pending(&registry){closes+=1;assert!(closes<250000);retire_cancelled_evaluations_close_step(&registry,SDK_NORMAL_TEST_POLICY);}assert!(evaluation_progress(&registry,"test.echo",hash).is_none());}
    println!("[DEBUG] Existing evaluation map owns input admission and compact resumes, turns={turns}, phases={phases:?}");
}

#[test]
fn retained_evaluation_output_measures_writes_and_retires_the_same_typed_owner() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).unwrap();let law=&fixture["retainedOutput"];let registry=echo_registry("test.echo","x");
    let label=law["labelUnit"].as_str().unwrap().repeat(law["labelRepeats"].as_u64().unwrap() as usize);let input=serde_json::json!({"text":label,"nested":{"a":-3,"b":2.5,"c":true}});let text=serde_json::to_string(&input).unwrap();let hash=law["nodeHash"].as_u64().unwrap();
    let request=|resume|EvaluateRequest {neuron_id:"tree.fixture.echo".into(),retained:SDK_NORMAL_TEST_POLICY,operator_id:"test.echo".into(),input_json:if resume{String::new()}else{text.clone()},node_hash:hash,budget:1,wall_micros:1,dependency_json:String::new(),operator_version:law["operatorVersion"].as_str().unwrap().into(),cancellation_id:law["cancellationId"].as_str().unwrap().into(),round_units:1,resume,external_result:None};
    let answer=|resume|serde_json::from_str::<serde_json::Value>(evaluate_step_envelope_cold(&registry,request(resume)).wire.as_deref().expect("original paid response")).unwrap();let expected=serde_json::json!({"x":input});
    let mut phases=std::collections::BTreeSet::new();let mut turns=0;let mut value=answer(false);let mut previous_units=0;
    loop {turns+=1;assert!(turns<500000);let units=value["unitsDone"].as_u64().unwrap();assert!(units>=previous_units);previous_units=units;phases.insert(value["phase"].as_str().unwrap().to_string());if value["done"]==true {assert_eq!(value["unitsDone"],value["unitsTotal"]);assert_eq!(serde_json::from_str::<serde_json::Value>(value["outputJson"].as_str().unwrap()).unwrap(),expected);break;}value=answer(true);}
    assert!(evaluation_progress(&registry,"test.echo",hash).is_none());for phase in law["phases"].as_array().unwrap(){assert!(phases.contains(phase.as_str().unwrap()),"missing bounded {phase}");}
    for phase in law["phases"].as_array().unwrap().iter().map(|phase|phase.as_str().unwrap()) {
        let mut value=answer(false);let mut advances=0;while value["phase"]!=phase {advances+=1;assert!(advances<500000);assert_eq!(value["done"],false);value=answer(true);}
        {let mut jobs=evaluation_jobs().lock().unwrap();let retained=jobs.jobs.get_mut(&(registry.owner_identity(),"test.echo".into(),hash)).unwrap();let before=retained.progress();assert!(matches!(retained.advance(&registry,"test.echo",0,3,SDK_NORMAL_TEST_POLICY).unwrap().0,EvaluationStep::Working(_)));assert!(matches!(retained.advance(&registry,"test.echo",1,0,SDK_NORMAL_TEST_POLICY).unwrap().0,EvaluationStep::Working(_)));assert_eq!(before,retained.progress());}
        assert!(cancel_evaluation(&registry,"test.echo",hash));let mut closes=0;while evaluation_retirement_pending(&registry){closes+=1;assert!(closes<500000);assert!(retire_cancelled_evaluations_close_step(&registry,SDK_NORMAL_TEST_POLICY).unwrap().progress().fits(SDK_NORMAL_TEST_POLICY));}assert!(evaluation_progress(&registry,"test.echo",hash).is_none());
    }
    let mut value=answer(false);let mut advances=0;while value["phase"]!="output-measure" {advances+=1;assert!(advances<500000);assert_eq!(value["done"],false);value=answer(true);}let replacement=serde_json::json!({"text":"replacement 雪"});let mut changed=request(false);changed.input_json=serde_json::to_string(&replacement).unwrap();let replaced:serde_json::Value=serde_json::from_str(evaluate_step_envelope_cold(&registry,changed).wire.as_deref().expect("original paid response")).unwrap();assert_eq!(replaced["done"],false);assert_eq!(replaced["phase"],"retire-invalidated");let mut restarted=false;let mut advances=0;loop {advances+=1;assert!(advances<500000);let value=answer(true);if value["phase"]=="restart-admission" {restarted=true;assert_eq!(value["unitsDone"],0);}if value["done"]==true {assert!(restarted);assert_eq!(serde_json::from_str::<serde_json::Value>(value["outputJson"].as_str().unwrap()).unwrap(),serde_json::json!({"x":replacement}));break;}}
    let source:Dictionary=semio_framework_pack_json::from_json_str(&text,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let mut refused=EvaluationOutput::new(source);let mut admitted=|_|true;refused.encoding=Some(semio_framework_value::NativeEncodeControl::new(law["ownershipCeiling"].as_u64().unwrap()as usize,&mut admitted).pause().unwrap());assert!(refused.step(1,0,SDK_NORMAL_TEST_POLICY).is_err());let phase=refused.phase();assert_eq!(refused.close_step(RetainedCloneGrant{maximum_items:0,..SDK_NORMAL_TEST_POLICY}).unwrap().progress(),Default::default());assert_eq!(refused.phase(),phase);let mut closes=0;while !refused.terminal_is_empty(){closes+=1;assert!(closes<500000);refused.close_step(SDK_NORMAL_TEST_POLICY).unwrap();}
    println!("[DEBUG] Existing SDK output owner retained measure/write/retire and phase cancellation, turns={turns}, phases={phases:?}");
}

#[test]
fn original_sdk_failed_normal_turn_retains_actual_codec_receipt(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).unwrap();let law=&fixture["failedNormalReceipt"];let source=law["source"].as_str().unwrap().to_string();assert!(serde_json::from_str::<serde_json::Value>(&source).is_err());let pointer=source.as_ptr();let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();let mut owner=EvaluationInputPreparation::new(source,String::new(),"test.echo".into(),"v1".into());let mut turns=0;let mut physical=RetainedCloneProgress::default();
    loop{
        turns+=1;assert!(turns<1000);let before=owner.progress();let (zero,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(1,4096,RetainedCloneGrant{maximum_items:0,..grant}));assert!(zero.unwrap().0.is_none());assert_eq!(owner.normal_step_progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.progress(),before);assert_eq!(owner.source_ptr(),Some(pointer));let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(1,4096,grant));let receipt=owner.normal_step_progress();assert!(receipt.fits(grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));physical=physical.checked_add(receipt).unwrap();match result{Ok((None,actual))=>assert_eq!(actual,receipt),Err(_)=>break,Ok((Some(_),_))=>panic!("original malformed JSON was admitted")}
    }
    assert!(physical.retained_capacity_bytes>0);owner.cancel();while !owner.terminal_is_empty(){owner.close_step(SDK_NORMAL_TEST_POLICY).unwrap();}eprintln!("[DEBUG] actual SDK codec receipt retained on failed normal turn; same source pointer, zero allocator0, physical={physical:?}, turns={turns}");
}


#[test]
fn original_sdk_roundtrip_wallet_preserves_actual_child_source_when_exhausted(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).unwrap();let law=&fixture["originalTurnWallet"];let grant:RetainedCloneGrant=serde_json::from_value(law["grant"].clone()).unwrap();let source=fixture["failedNormalReceipt"]["source"].as_str().unwrap().to_string();let pointer=source.as_ptr();let mut owner=EvaluationInputPreparation::new(source,String::new(),"test.echo".into(),"v1".into());
    let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(1,4096,grant));let receipt=owner.normal_step_progress();let (_,actual)=result.unwrap();assert_eq!(actual,receipt);assert!(receipt.fits(grant));assert_eq!(receipt.copied_items,grant.maximum_items);assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));let remaining=evaluation_remaining_grant(grant,receipt);let independent:serde_json::Value=serde_json::to_value(grant).unwrap();assert_eq!(remaining.maximum_items,independent["maximumItems"].as_u64().unwrap()as usize-receipt.copied_items);assert_eq!(remaining.maximum_copy_bytes,independent["maximumCopyBytes"].as_u64().unwrap()as usize-receipt.copied_bytes);assert_eq!(remaining.maximum_capacity_bytes,independent["maximumCapacityBytes"].as_u64().unwrap()as usize-receipt.retained_capacity_bytes);assert_eq!(remaining.maximum_release_bytes,independent["maximumReleaseBytes"].as_u64().unwrap()as usize-receipt.released_bytes);assert_eq!(remaining.maximum_depth,grant.maximum_depth);
    let before=owner.progress();let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(8,4096,remaining));assert!(result.unwrap().0.is_none());assert_eq!(owner.normal_step_progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.progress(),before);assert_eq!(owner.source_ptr(),Some(pointer));owner.cancel();let mut turns=0;while !owner.terminal_is_empty(){turns+=1;assert!(turns<1000);let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(grant));let receipt=step.unwrap().progress();assert!(receipt.fits(grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));}let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original SDK same source survives exhausted one-roundtrip wallet; actualReceipt={receipt:?}, nextItems0, freshCloseTurns={turns}, terminalDrop0");
}

#[test]
fn original_sdk_binding_failure_preserves_same_original_child(){
    let source="original same binding child 雪".to_string();let pointer=source.as_ptr();let mut owner=EvaluationInputPreparation::new(String::new(),String::new(),String::new(),String::new());owner.parser=None;owner.binding=Some(neural_engine::retirement::RetainedDictionaryInput::new(DslValue::String(source)));owner.stage=3;
    let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(1,4096,SDK_NORMAL_TEST_POLICY));assert!(result.is_err());assert_eq!(owner.normal_step_progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(owner.binding.is_some());owner.cancel();let mut turns=0;while !owner.terminal_is_empty(){turns+=1;assert!(turns<1000);let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(SDK_NORMAL_TEST_POLICY));let receipt=step.unwrap().progress();assert!(receipt.fits(SDK_NORMAL_TEST_POLICY));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));}let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original SDK binding failure preserves original child allocation={pointer:p}; actual failed child0, closureTurns={turns}, terminalDrop0");
}


#[test]
fn original_sdk_request_admission_keeps_same_source_under_copy_one_three_and_sixty_four(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).unwrap();
 for copy in fixture["originalRequestAdmission"]["copyPolicies"].as_array().unwrap(){
  let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy.as_u64().unwrap()as usize,..SDK_NORMAL_TEST_POLICY};let request=EvaluateRequest{neuron_id:"tree.fixture.echo".into(),retained:grant,operator_id:"test.original.雪".into(),input_json:"{\"value\":\"same original 雪\"}".into(),operator_version:"version.雪".into(),node_hash:77,budget:1,wall_micros:1,dependency_json:String::new(),cancellation_id:"original.cancel".into(),round_units:1,resume:false,external_result:None};let pointer=request.input_json.as_ptr();let(mut owner,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||EvaluationRequestAdmission::new(request));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut physical=RetainedCloneProgress::default();let mut turns=0;
  while !owner.ready(){turns+=1;assert!(turns<1000);let(before,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(RetainedCloneGrant{maximum_items:0,..grant}));assert_eq!(before.unwrap(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.original_request_source_ptr(),Some(pointer));let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant));let receipt=result.unwrap();assert!(receipt.fits(grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));physical=physical.checked_add(receipt).unwrap();}
  let((request,key,version),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.take_ready().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(request.input_json.as_ptr(),pointer);assert_eq!(key,request.operator_id);assert_eq!(version,request.operator_version);let independent:serde_json::Value=serde_json::from_str("{\"operatorId\":\"test.original.雪\",\"operatorVersion\":\"version.雪\"}").unwrap();assert_eq!(key,independent["operatorId"].as_str().unwrap());assert_eq!(version,independent["operatorVersion"].as_str().unwrap());let mut original=Some((request,key,version));let mut active=None;while original.is_some()||active.is_some(){let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||if active.is_some(){evaluation_child_close(&mut active,grant)}else{evaluation_admit_original(&mut original,&mut active,grant)});let receipt=step.unwrap().progress();assert!(receipt.fits(grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));}let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] Original SDK request metadata copy{copy} same source={pointer:p}, actualReceipt={physical:?}, turns={turns}, terminalDrop0");
 }
}


#[test]
fn original_sdk_response_source_moves_same_output_and_accounts_actual_codec(){
 let text="{\"value\":\"same output 雪\"}".to_string();let pointer=text.as_ptr();let(source,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||EvaluationEnvelopeSource{output_json:text,done:true,cancellable:false,phase:"complete",units_done:7,units_total:7,pending:None,fault:None});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(source.output_json.as_ptr(),pointer);let mut writer=Some(semio_framework_pack_json::JsonWriteCursor::new(source));let mut encoding=None;let mut physical=RetainedCloneProgress::default();let mut turns=0;let wire=loop{turns+=1;assert!(turns<10000);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let mut accepted=|_|true;let mut control=match encoding.take(){Some(saved)=>semio_framework_value::NativeEncodeControl::resume(saved,&mut accepted).unwrap(),None=>semio_framework_value::NativeEncodeControl::new_retained(&mut accepted)};let result=writer.as_mut().unwrap().step(1,&mut control,SDK_NORMAL_TEST_POLICY);encoding=Some(control.pause().unwrap());result});let receipt=writer.as_ref().unwrap().normal_step_progress();assert!(receipt.fits(SDK_NORMAL_TEST_POLICY));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));physical=physical.checked_add(receipt).unwrap();if let Some(text)=result.unwrap(){break text;}};let independent:serde_json::Value=serde_json::from_str(&wire).unwrap();assert_eq!(independent,serde_json::json!({"cancellable":false,"done":true,"outputJson":"{\"value\":\"same output 雪\"}","phase":"complete","unitsDone":7,"unitsTotal":7}));let mut active=None;while writer.is_some()||active.is_some(){let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||if active.is_some(){evaluation_child_close(&mut active,SDK_NORMAL_TEST_POLICY)}else{evaluation_admit_original(&mut writer,&mut active,SDK_NORMAL_TEST_POLICY)});let receipt=step.unwrap().progress();assert!(receipt.fits(SDK_NORMAL_TEST_POLICY));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));}let mut original_wire=Some(wire);while original_wire.is_some()||active.is_some(){let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||if active.is_some(){evaluation_child_close(&mut active,SDK_NORMAL_TEST_POLICY)}else{evaluation_admit_original(&mut original_wire,&mut active,SDK_NORMAL_TEST_POLICY)});let receipt=step.unwrap().progress();assert!(receipt.fits(SDK_NORMAL_TEST_POLICY));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));}let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(writer));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] Original SDK output pointer={pointer:p}, inline envelope source0/0, actualCodec={physical:?}, turns={turns}, original wire close, terminalDrop0");
}


#[test]
fn original_sdk_initial_owner_keeps_request_through_paid_metadata_and_full_close(){
 for copy in [1,3,64]{
  let grant=RetainedCloneGrant{maximum_copy_bytes:copy,..SDK_NORMAL_TEST_POLICY};let request=EvaluateRequest{neuron_id:"tree.fixture.echo".into(),retained:grant,operator_id:"test.original.雪".into(),input_json:"{\"value\":\"same original 雪\"}".into(),operator_version:"version.雪".into(),node_hash:99,budget:1,wall_micros:1,dependency_json:String::new(),cancellation_id:"original.cancel".into(),round_units:1,resume:false,external_result:None};let pointer=request.input_json.as_ptr();let mut resources=ExtensionEvaluationResources::new(Registry::new());let registry=resources.registry();let(mut retained,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||RetainedEvaluation::new(registry.clone(),request));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(retained.request_admission.as_ref().unwrap().original_request_source_ptr(),Some(pointer));let mut turns=0;while retained.request_admission.is_some(){turns+=1;assert!(turns<1000);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||retained.advance(registry,"test.original.雪",1,4096,RetainedCloneGrant{maximum_items:0,..grant}));assert_eq!(result.unwrap().1,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(retained.request_admission.as_ref().unwrap().original_request_source_ptr(),Some(pointer));let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||retained.advance(registry,"test.original.雪",1,4096,grant));let receipt=result.unwrap().1;assert!(receipt.fits(grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));}assert_eq!(retained.admission.as_ref().unwrap().source_ptr(),Some(pointer));retained.cancel();let mut closes=0;while !retained.close_fields_terminal(false,false){closes+=1;assert!(closes<100000);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||retained.close_step(grant,false,false));let receipt=result.unwrap().progress();assert!(receipt.fits(grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));}let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(retained));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));semio_framework_plugin::ExtensionResourceOwner::begin_close(&mut resources);let mut closes=0;while !semio_framework_plugin::ExtensionResourceOwner::terminal_is_empty(&resources){closes+=1;assert!(closes<100000);semio_framework_plugin::ExtensionResourceOwner::close_step(&mut resources,grant).unwrap();}eprintln!("[DEBUG] Original retained SDK owner copy{copy} same input={pointer:p}; paid metadata turns={turns}, actual closure, terminalDrop0");
 }
}


#[test]
fn original_sdk_registry_close_keeps_unrelated_original_owner(){
 let mut original=ExtensionEvaluationResources::new(Registry::new());let mut unrelated=ExtensionEvaluationResources::new(Registry::new());let mut unrelated_pointer=None;
 for resources in [&original,&unrelated]{let registry=resources.registry();let operator="test.retirement.雪".to_string();let pointer=operator.as_ptr();let key=(registry.owner_identity(),operator,700707);let request=EvaluateRequest{neuron_id:"tree.fixture.echo".into(),retained:SDK_NORMAL_TEST_POLICY,operator_id:String::new(),input_json:"{\"original\":true}".into(),operator_version:"v1".into(),node_hash:700707,budget:1,wall_micros:1,dependency_json:String::new(),cancellation_id:"original-cancel".into(),round_units:1,resume:false,external_result:None};let mut retained=RetainedEvaluation::new(registry.clone(),request);retained.cancel();let mut turns=0;while !retained.close_fields_terminal(false,false){turns+=1;assert!(turns<100000);retained.close_step(SDK_NORMAL_TEST_POLICY,false,false).unwrap();}let mut jobs=evaluation_jobs().lock().unwrap();let mut turns=0;while jobs.jobs.next_insert_capacity_byte_demand(&key,SDK_NORMAL_TEST_POLICY.maximum_copy_bytes).unwrap()!=0{turns+=1;assert!(turns<100000);jobs.jobs.reserve_insert_step(&key,SDK_NORMAL_TEST_POLICY).unwrap();}assert!(jobs.jobs.insert_reserved(key,retained,SDK_NORMAL_TEST_POLICY).unwrap_or_else(|(error,_,_)|panic!("original fixture row admission: {error}")).0.is_none());jobs.lookup_epoch+=1;if registry.owner_identity()==unrelated.registry().owner_identity(){unrelated_pointer=Some(pointer);}}
 let mut turns=0;while evaluation_retirement_pending(original.registry()){turns+=1;assert!(turns<100000);let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||retire_cancelled_evaluations_close_step(original.registry(),SDK_NORMAL_TEST_POLICY));let receipt=result.unwrap().progress();assert!(receipt.fits(SDK_NORMAL_TEST_POLICY));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));let jobs=evaluation_jobs().lock().unwrap();let key=jobs.jobs.keys().find(|key|key.0==unrelated.registry().owner_identity()).unwrap();assert_eq!(Some(key.1.as_ptr()),unrelated_pointer);}
 assert!(!evaluation_jobs().lock().unwrap().jobs.keys().any(|key|key.0==original.registry().owner_identity()));for resources in [&mut original,&mut unrelated]{semio_framework_plugin::ExtensionResourceOwner::begin_close(resources);let mut turns=0;while !semio_framework_plugin::ExtensionResourceOwner::terminal_is_empty(resources){turns+=1;assert!(turns<100000);semio_framework_plugin::ExtensionResourceOwner::close_step(resources,SDK_NORMAL_TEST_POLICY).unwrap();}}eprintln!("[DEBUG] Original registry close removed only original terminal row, unrelated key pointer={unrelated_pointer:?}, actual physical receipts, turns={turns}, full source close");
}

/// 📬️ The admitted external output source binds under the unchanged caller wallet and full close.
#[test]
fn original_sdk_external_completion_preserves_source_system_receipts_and_finish_input(){
 for copy in [1,3,64]{let original=EvaluationExternalResult{neuron_id:"same.neuron.雪".into(),extension_id:"same.extension".into(),operator_id:"same.operator".into(),node_hash:71,output_json:"{\"number\":7}".into()};let pointer=original.output_json.as_ptr();let(mut owner,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||EvaluationExternalCompletion::new(original));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,..SDK_NORMAL_TEST_POLICY};let mut output=None;
  for turn in 0..100000{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(RetainedCloneGrant{maximum_items:0,..grant}));assert!(result.unwrap().is_none());assert_eq!(owner.normal_step_progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if let Some(original)=owner.original.as_ref(){assert_eq!(original.output_json.as_ptr(),pointer)}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant));let receipt=owner.normal_step_progress();assert!(receipt.fits(grant));assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));if let Some(dictionary)=result.unwrap(){output=Some(dictionary);break}assert!(turn<99999,"external completion copy={copy} failed to advance");}
  assert!(owner.terminal_is_empty());let output=output.unwrap();let produced:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&output)).unwrap();let independent:serde_json::Value=serde_json::from_str("{\"number\":7}").unwrap();assert_eq!(produced,independent);evaluation_retire_cold(output);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] Original SDK external output copy{copy} pointer={pointer:p}, typed dictionary matches independent Serde, System receipts and full source closure, terminalDrop0");
 }
}

/// 🪪️ Original continuation fields are compared under unchanged copy1/3/64 with actual System0.
#[test]
fn original_sdk_continuation_identity_keeps_same_source_under_bounded_copy(){
 for copy in [1,3,64]{for mismatch in [false,true]{
  let pending=neural_engine::PendingExtensionEval{neuron_id:"same nested neuron 源".into(),extension_id:"same extension 源".into(),operator_id:"same operator 源".into(),node_hash:71,input_json:"same retained input".into()};
  let mut request=EvaluateRequest{neuron_id:"same outer neuron 源".into(),retained:SDK_NORMAL_TEST_POLICY,operator_id:"same outer operator".into(),input_json:String::new(),node_hash:73,budget:1,wall_micros:1,dependency_json:String::new(),operator_version:"same version 源".into(),cancellation_id:"same cancellation 源".into(),round_units:1,resume:true,external_result:Some(EvaluationExternalResult{neuron_id:pending.neuron_id.clone(),extension_id:pending.extension_id.clone(),operator_id:if mismatch{"other operator 源".into()}else{pending.operator_id.clone()},node_hash:pending.node_hash,output_json:"{\"number\":7}".into()})};let source=request.external_result.as_ref().unwrap().output_json.as_ptr();let(mut identity,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||EvaluationContinuationIdentity::new(true));assert_eq!((birth.requested_bytes,birth.released_bytes),(0,0));let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,..SDK_NORMAL_TEST_POLICY};let mut result=None;
  for turn in 0..10000{let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||identity.step(&request,Some(&pending),"same outer neuron 源","same version 源","same cancellation 源",false,RetainedCloneGrant{maximum_items:0,..grant}));assert_eq!(answer.unwrap(),(None,Default::default()));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||identity.step(&request,Some(&pending),"same outer neuron 源","same version 源","same cancellation 源",false,grant));let(complete,receipt)=answer.unwrap();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(request.external_result.as_ref().unwrap().output_json.as_ptr(),source);if complete.is_some(){result=complete;break}assert!(turn<9999,"original identity stalled copy={copy}");}
  assert_eq!(result,Some(!mismatch));let oracle=serde_json::json!({"neuronId":pending.neuron_id,"extensionId":pending.extension_id,"operatorId":pending.operator_id,"nodeHash":pending.node_hash});assert_eq!(request.external_result.as_ref().unwrap().operator_id==oracle["operatorId"].as_str().unwrap(),!mismatch);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(identity));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut original=Some(request);let mut active=None;while original.is_some()||active.is_some(){let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||if active.is_some(){evaluation_child_close(&mut active,grant)}else{evaluation_admit_original(&mut original,&mut active,grant)});let receipt=step.unwrap().progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));}eprintln!("[DEBUG] Original external identity copy={copy} mismatch={mismatch} sameOutputSource=true boundedCompare=true System0=true wholeRequestClose=true");
 }}
}
