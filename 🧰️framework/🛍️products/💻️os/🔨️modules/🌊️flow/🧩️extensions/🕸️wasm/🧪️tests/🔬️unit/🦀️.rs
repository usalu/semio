
use super::*;

/// 🪪️ The original raw recipient pays grammar effects and closes its same partial owner.
#[test]
fn original_sdk_raw_request_keeps_same_caller_source_and_actual_receipt(){
 use semio_framework_plugin::ExtensionResourceOwner;
 for copy in [1,3,64]{
  let mut caller=sdk_fixture_caller();let grant=RetainedCloneGrant{maximum_copy_bytes:copy,..SDK_NORMAL_TEST_POLICY};let request=serde_json::to_vec(&serde_json::json!({"neuronId":"tree.raw.雪","operatorId":"test.original","inputJson":"same source 雪".repeat(128)})).unwrap();let oracle:serde_json::Value=serde_json::from_slice(&request).unwrap();assert_eq!(oracle["neuronId"],"tree.raw.雪");let pointer=request.as_ptr();let mut owner=ExtensionEvaluationResources::new(Registry::new());
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let before=caller.fuel_remaining();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(denied,|cx|evaluate_invoke_json(&mut owner,&request,cx).map_err(Into::into)));let reply=result.unwrap();assert!(reply.payload.is_none()&&reply.refusal.is_none());assert_eq!(reply.retained_progress,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(before,caller.fuel_remaining());assert!(owner.invoke.lock().unwrap().is_none());}
  for _ in 0..16{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|evaluate_invoke_json(&mut owner,&request,cx).map_err(Into::into)));let reply=result.unwrap();assert!(reply.payload.is_none()&&reply.refusal.is_none());assert_eq!(reply.retained_progress,caller.receipt());assert_eq!((reply.retained_progress.retained_capacity_bytes,reply.retained_progress.released_bytes),(heap.requested_bytes,heap.released_bytes));assert_eq!(request.as_ptr(),pointer);assert!(owner.invoke.lock().unwrap().as_ref().unwrap().matches(71,3,&request));}
  caller.cancel().cancel_now();owner.begin_close();for turn in 0..65536{if owner.terminal_is_empty(){break}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|owner.close_step(cx)));result.unwrap();let receipt=caller.receipt();assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));assert!(turn<65535)}assert!(owner.terminal_is_empty());sdk_fixture_close_owned(&mut caller,request,copy);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let stock=caller.stock();caller.retire(stock);
 }
}

/// 🚪️ The actual gateway keeps caller bytes while every partial grammar owner closes.
#[test]
fn original_inference_gateway_keeps_same_request_and_actual_receipt(){
 for copy in [1,3,64]{
  let mut caller=sdk_fixture_caller();let grant=RetainedCloneGrant{maximum_copy_bytes:copy,..SDK_NORMAL_TEST_POLICY};let request=serde_json::to_vec(&serde_json::json!({"owner":"same source 雪".repeat(128),"canonicalPayload":[1,2,3]})).unwrap();let oracle:serde_json::Value=serde_json::from_slice(&request).unwrap();assert_eq!(oracle["canonicalPayload"],serde_json::json!([1,2,3]));let pointer=request.as_ptr();let(mut gateway,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_plugin::ArtifactInferenceGateway::new(71,3,&request));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let before=caller.fuel_remaining();let(reply,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(denied,|cx|Ok(gateway.step(&request,cx,None))));let reply=reply.unwrap();assert!(reply.payload.is_none()&&reply.refusal.is_none());assert_eq!(reply.retained_progress,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(before,caller.fuel_remaining());}
  for _ in 0..16{let(reply,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|Ok(gateway.step(&request,cx,None))));let reply=reply.unwrap();assert!(reply.payload.is_none()&&reply.refusal.is_none());assert_eq!(reply.retained_progress,caller.receipt());assert_eq!((reply.retained_progress.retained_capacity_bytes,reply.retained_progress.released_bytes),(heap.requested_bytes,heap.released_bytes));assert_eq!(request.as_ptr(),pointer);assert!(gateway.matches(71,3,&request));}
  caller.cancel().cancel_now();gateway.begin_close();for turn in 0..65536{if gateway.terminal_is_empty(){break}let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant,|cx|semio_framework_job::close_original_retained_step(cx,gateway.retirement_demands(copy)?,|grant|gateway.close_step(grant)).map(|_|())));result.unwrap();let receipt=caller.receipt();assert_eq!((receipt.retained_capacity_bytes,receipt.released_bytes),(heap.requested_bytes,heap.released_bytes));assert!(turn<65535)}assert!(gateway.terminal_is_empty());sdk_fixture_close_owned(&mut caller,request,copy);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(gateway));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let stock=caller.stock();caller.retire(stock);
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
/// `🧑‍🎨engine/🧪️tests/🪝️extension-invocation/🟦️.ts`. `outcome` is what the SHELL submits through
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

fn sdk_fixture_caller()->semio_framework_job::OriginalJobFixtureCaller{
 let law:serde_json::Value=serde_json::from_str(read_node("🧪️tests/🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).unwrap();let caller=&law["originalEnvelopeSource"]["caller"];let stock=&caller["stock"];let grant=RetainedCloneGrant{maximum_items:stock["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:stock["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:stock["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:stock["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:stock["maximumDepth"].as_u64().unwrap()as usize};
 semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(caller["operation"].as_u64().unwrap()),semio_framework_job::Generation(caller["generation"].as_u64().unwrap()),grant,||Some(1000),caller["deadline"].as_u64().unwrap(),caller["fuel"].as_u64().unwrap())
}
fn sdk_fixture_close_owned<T:RetireOwned>(caller:&mut semio_framework_job::OriginalJobFixtureCaller,value:T,body:usize){
 let mut original=Some(value);let mut active=None;for turn in 0..65536{if original.is_none()&&active.is_none(){return}let(answer,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(RetainedCloneGrant{maximum_copy_bytes:body,..SDK_NORMAL_TEST_POLICY},|cx|{let demand=if active.is_some(){evaluation_child_demands(&active,body)?}else{evaluation_owned_birth::<T>()};semio_framework_job::close_original_retained_step(cx,demand,|grant|if active.is_some(){evaluation_child_close(&mut active,grant)}else{evaluation_admit_original(&mut original,&mut active,grant)}).map(|_|())}));answer.unwrap();let progress=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));assert!(turn<65535)}unreachable!()
}
struct FixtureRegistry {owner:ExtensionEvaluationResources,caller:std::sync::Mutex<Option<semio_framework_job::OriginalJobFixtureCaller>>}
impl std::ops::Deref for FixtureRegistry {type Target=neural_engine::SharedRegistry;fn deref(&self)->&Self::Target{self.owner.registry().expect("ready original fixture registry")}}
impl FixtureRegistry{
 fn close_owned<T:RetireOwned>(&self,value:T){let mut caller=self.caller.lock().unwrap();sdk_fixture_close_owned(caller.as_mut().unwrap(),value,SDK_NORMAL_TEST_POLICY.maximum_copy_bytes);}

}
impl Drop for FixtureRegistry{fn drop(&mut self){use semio_framework_plugin::ExtensionResourceOwner;self.owner.begin_close();let caller=self.caller.get_mut().unwrap();let source=caller.as_mut().unwrap();for turn in 0..65536{if self.owner.terminal_is_empty(){break}source.turn(SDK_NORMAL_TEST_POLICY,|cx|self.owner.close_step(cx)).unwrap();assert!(turn<65535)}assert!(self.owner.terminal_is_empty());let source=caller.take().unwrap();let stock=source.stock();source.retire(stock);}}
fn fixture_raw_turn(owner:&mut FixtureRegistry,request:&[u8])->Result<Vec<u8>,semio_framework_job::OriginalTurnFailure>{
 let mut caller=owner.caller.lock().unwrap();let caller=caller.as_mut().unwrap();
 for turn in 0..65536{let reply=caller.turn(SDK_NORMAL_TEST_POLICY,|cx|evaluate_invoke_json(&mut owner.owner,request,cx).map_err(Into::into))?;assert_eq!(reply.retained_progress,caller.receipt());if let Some(original)=reply.refusal{return Err(original)}if let Some(bytes)=reply.payload{return Ok(bytes)}assert!(turn<65535)}unreachable!()
}
fn fixture_pack_tree(bytes:&[u8])->serde_json::Value{let value=semio_framework_os_kernel::pack_rt::decode_wire_value(bytes).unwrap();let logical=serde_json::Value::from(&value);evaluation_retire_cold(value);logical}

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
    let mut fixture=FixtureRegistry {owner:ExtensionEvaluationResources::new(registry),caller:std::sync::Mutex::new(Some(sdk_fixture_caller()))};
    for turn in 0..65536 {
        let source=fixture.caller.get_mut().unwrap().as_mut().unwrap();
        if source.turn(SDK_NORMAL_TEST_POLICY,|cx|fixture.owner.admit_registry_on_original(cx)).unwrap()==Some(true){return fixture;}
        assert!(turn<65535);
    }
    unreachable!()
}

#[test]
fn the_evaluate_wire_answers_every_fixture_row(){
 let fixture:ExtensionInvocationWireFixture=serde_json::from_str(read_node("🧪️tests/🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).unwrap();assert_eq!(fixture.capability,"evaluate");assert_eq!(fixture.request_fields,["neuronId","operatorId","inputJson"]);assert_eq!(fixture.envelope_fields,["done","cancellable","phase","unitsDone","unitsTotal","output"]);let mut registry=echo_registry(&fixture.operator_id,&fixture.output_channel);
 for row in &fixture.rows{match fixture_raw_turn(&mut registry,row.request_json.as_bytes()){
  Ok(bytes)=>{assert_eq!(row.outcome,"ok");let envelope=fixture_pack_tree(&bytes);assert_eq!(envelope["done"],row.done);assert_eq!(envelope["phase"],row.phase);let output=envelope["output"].as_object().unwrap();assert_eq!(output.keys().cloned().collect::<Vec<_>>(),row.output_keys);assert_eq!(output.values().next().is_some_and(|body|body.as_object().is_some_and(|body|body.is_empty())),row.output_empty);registry.close_owned(bytes);},
  Err(original)=>{assert_eq!(row.outcome,"fault");assert!(original.primary.message().contains(row.fault.as_ref().unwrap()));registry.close_owned(original);}
 }}
}
// #endregion 🔁️ExtensionInvocationWire

#[test]
fn retained_evaluation_input_preserves_canonical_identity_and_bounded_owned_binding() {
    let fixture:serde_json::Value=serde_json::from_str(read_node("🧪️tests/🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).unwrap();let law=&fixture["retainedAdmission"];
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
}

