
use super::*;
use neural_engine::{Atom, ChannelSpec, EvalError, Operator, OperatorImpl, OperatorInfo, Value, channel_output};

struct Echo;

impl Operator for Echo {
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

struct FixtureRegistry {source:Option<neural_engine::SharedRegistry>,retirement:neural_engine::RegistryRetirement}
impl std::ops::Deref for FixtureRegistry {type Target=neural_engine::SharedRegistry;fn deref(&self)->&Self::Target {self.source.as_ref().unwrap()}}
impl Drop for FixtureRegistry {fn drop(&mut self){if let Some(source)=self.source.take(){cancel_all_evaluations(&source);while evaluation_retirement_pending(&source){retire_cancelled_evaluations_close_step(&source,4096,4096);}drop(source);}while !self.retirement.terminal_is_empty(){self.retirement.close_step(4096,4096).unwrap();}}}

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
    let (source,retirement)=neural_engine::SharedRegistry::new(registry);FixtureRegistry {source:Some(source),retirement}
}

#[test]
fn the_evaluate_wire_answers_every_fixture_row() {
    let fixture: ExtensionInvocationWireFixture = serde_json::from_str(include_str!("../../🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).expect("the extension-invocation-wire fixture must parse");
    assert_eq!(fixture.capability, "evaluate");
    assert!(!fixture.note.is_empty(), "the fixture states what the wire is");
    assert_eq!(fixture.request_fields, vec!["operatorId".to_string(), "inputJson".to_string()], "the REQUIRED request fields");
    for field in ["nodeHash", "budget", "wallMicros"] {
        assert!(fixture.optional_request_fields.iter().any(|declared| declared == field), "the request offers the optional budget field {field}");
    }
    for field in ["done", "phase", "unitsDone", "unitsTotal", "outputJson"] {
        assert!(fixture.envelope_fields.iter().any(|declared| declared == field), "the envelope declares {field}");
    }
    let registry = echo_registry(&fixture.operator_id, &fixture.output_channel);
    for row in &fixture.rows {
        match evaluate_invoke_json(&registry, row.request_json.as_bytes()) {
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
                assert_eq!(Some(error), row.fault.clone(), "{}", row.name);
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
    let output=loop {let before=preparation.progress();assert!(preparation.step(0,maximum_bytes).unwrap().is_none());assert!(preparation.step(maximum_units,0).unwrap().is_none());assert_eq!(before,preparation.progress());phases.entry(preparation.progress().2).or_insert(turns);turns+=1;assert!(turns<250000);if let Some(output)=preparation.step(maximum_units,maximum_bytes).unwrap(){break output;}};
    assert_eq!(output.1,expected_identity);assert_eq!(output.0,*expected);drop(ColdOwner::new(output.0));assert!(preparation.terminal_is_empty());
    for phase in law["phases"].as_array().unwrap(){assert!(phases.contains_key(phase.as_str().unwrap()),"missing {phase}");}
    let cutoffs:std::collections::BTreeSet<usize>=law["cancelCutoffs"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as usize).chain(phases.values().copied()).collect();
    for cutoff in cutoffs{let mut candidate=EvaluationInputPreparation::new(input_text.clone(),dependency_text.clone(),"test.echo".into(),"test-v1".into());for _ in 0..cutoff{assert!(candidate.step(1,3).unwrap().is_none());}candidate.cancel();let before=candidate.progress();assert!(matches!(candidate.close_step(0,3),neural_engine::ValueRetirementStep::Blocked));assert!(matches!(candidate.close_step(1,0),neural_engine::ValueRetirementStep::Blocked));assert_eq!(before,candidate.progress());let mut close_turns=0;while !candidate.terminal_is_empty(){close_turns+=1;assert!(close_turns<250000);match candidate.close_step(1,3){neural_engine::ValueRetirementStep::Pending {released_bytes,..}=>assert!(released_bytes<=3),_=>{}}}}
    let invalid=law["invalidInputText"].as_str().unwrap();assert!(serde_json::from_str::<serde_json::Value>(invalid).unwrap()["bad"].is_array());let mut candidate=EvaluationInputPreparation::new(invalid.into(),dependency_text,"test.echo".into(),"test-v1".into());let mut fault_turns=0;loop {fault_turns+=1;assert!(fault_turns<250000);match candidate.step(1,3){Ok(None)=>{},Err(_)=>break,Ok(Some((value,_)))=>{drop(ColdOwner::new(value));panic!("malformed typed input admitted");}}}candidate.cancel();while !candidate.terminal_is_empty(){candidate.close_step(1,3);}
    println!("[DEBUG] Existing SDK input owner canonicalizes and binds without source cloning, turns={turns}, phases={phases:?}");
}

#[test]
fn retained_evaluation_terminal_metadata_matches_the_original_physical_wire() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).unwrap();let law=&fixture["terminalMetadata"];let registry=echo_registry("test.echo","x");
    let check=|name:&str,reply:EvaluationReply|{let row=law["rows"].as_array().unwrap().iter().find(|row|row["name"]==name).unwrap();let wire:serde_json::Value=serde_json::from_str(&reply.wire).unwrap();let faulted=wire["outputJson"].as_str().and_then(|text|serde_json::from_str::<serde_json::Value>(text).ok()).is_some_and(|output|output.get("error").is_some());assert_eq!(reply.complete,wire["done"].as_bool().unwrap());assert_eq!(reply.faulted,faulted);assert_eq!(reply.complete,row["complete"].as_bool().unwrap());assert_eq!(reply.faulted,row["faulted"].as_bool().unwrap());wire};
    for row in fixture["rows"].as_array().unwrap().iter().filter(|row|row["outcome"]=="ok") {let request=semio_framework_pack_json::from_json_str::<EvaluateRequest>(row["requestJson"].as_str().unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();check(row["name"].as_str().unwrap(),evaluate_step_envelope(&registry,request));}
    let hash=law["nodeHash"].as_u64().unwrap();let request=|resume:bool|{let wire=serde_json::json!({"operatorId":"test.echo","inputJson":if resume{""}else{"{\"number\":2}"},"dependencyJson":"","nodeHash":hash,"budget":law["roundUnits"],"roundUnits":law["roundUnits"],"wallMicros":1,"resume":resume});semio_framework_pack_json::from_json_str::<EvaluateRequest>(&serde_json::to_string(&wire).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()};
    check("pending",evaluate_step_envelope(&registry,request(false)));assert!(cancel_evaluation(&registry,"test.echo",hash));let mut turns=0;loop {turns+=1;assert!(turns<law["maximumTurns"].as_u64().unwrap());let reply=evaluate_step_envelope(&registry,request(true));if reply.complete {assert_eq!(check("cancelled",reply)["phase"],"cancelled");break;}check("pending",reply);}assert!(evaluation_progress(&registry,"test.echo",hash).is_none());println!("[DEBUG] Same SDK publication carries terminal metadata for all five independent serde wire outcomes, cancelTurns={turns}");
}

#[test]
fn extension_close_reports_the_original_child_source_retirement_bytes() {
    use semio_framework_plugin::{ExtensionResourceOwner,PluginCloseStep};
    let fixture:ExtensionInvocationWireFixture=serde_json::from_str(include_str!("../../🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).unwrap();let law=fixture.retained_close_receipt;
    let source=serde_json::to_string(&serde_json::json!({"text":law.label_unit.repeat(law.label_repeats)})).unwrap();let expected_bytes=source.len();let mut owner=ExtensionEvaluationResources::new(Registry::new());
    let reply=evaluate_step_envelope(owner.registry(),EvaluateRequest {operator_id:"test.close".into(),input_json:source,node_hash:law.node_hash,budget:law.maximum_units as u64,wall_micros:1,dependency_json:String::new(),operator_version:"close-v1".into(),cancellation_id:law.operation_id,round_units:law.maximum_units as u64,resume:false});assert!(!reply.complete);owner.begin_close();
    let before=evaluation_jobs().lock().unwrap().jobs.get(&(owner.registry().owner_identity(),"test.close".into(),law.node_hash)).unwrap().progress();
    assert!(matches!(owner.close_step(0,law.maximum_bytes).unwrap(),PluginCloseStep::Pending {released_items:0,released_bytes:0}));assert!(matches!(owner.close_step(law.maximum_units,0).unwrap(),PluginCloseStep::Pending {released_items:0,released_bytes:0}));
    assert_eq!(before,evaluation_jobs().lock().unwrap().jobs.get(&(owner.registry().owner_identity(),"test.close".into(),law.node_hash)).unwrap().progress());
    let mut charged=0;let mut turns=0;while evaluation_retirement_pending(owner.registry()) {turns+=1;assert!(turns<law.maximum_turns);if let PluginCloseStep::Pending {released_items,released_bytes}=owner.close_step(law.maximum_units,law.maximum_bytes).unwrap(){assert!(released_items<=law.maximum_units);assert!(released_bytes<=law.maximum_bytes);charged+=released_bytes;}}
    while !owner.terminal_is_empty(){turns+=1;assert!(turns<law.maximum_turns);owner.close_step(law.maximum_units,4096).unwrap();}
    assert!(charged>=expected_bytes,"original child source bytes were retired without their parent receipt: charged={charged}, expectedAtLeast={expected_bytes}");println!("[DEBUG] Original extension close propagates child source receipt bytes={charged}, sourceBytes={expected_bytes}, turns={turns}");
}

#[test]
fn retained_evaluation_map_owns_initial_input_and_accepts_only_compact_current_resumes() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).unwrap();let law=&fixture["retainedAdmission"];let wire=&law["compactResume"];let registry=echo_registry("test.echo","x");
    let label=law["labelUnit"].as_str().unwrap().repeat(law["labelRepeats"].as_u64().unwrap() as usize);let input=serde_json::json!({"text":label});let text=serde_json::to_string(&input).unwrap();let hash=wire["nodeHash"].as_u64().unwrap();
    let request=|source:&str,resume:bool,version:&str| {let request=serde_json::json!({"operatorId":"test.echo","inputJson":source,"dependencyJson":"","operatorVersion":version,"cancellationId":wire["cancellationId"],"nodeHash":hash,"budget":1,"roundUnits":1,"wallMicros":1,"resume":resume});semio_framework_pack_json::from_json_str::<EvaluateRequest>(&serde_json::to_string(&request).unwrap(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()};
    let answer=|request:EvaluateRequest|serde_json::from_str::<serde_json::Value>(&evaluate_step_envelope(&registry,request).wire).unwrap();
    let first=answer(request(&text,false,"test-v1"));assert_eq!(first["done"],false);assert_eq!(first["phase"],wire["initialPhase"]);assert_eq!(first["unitsDone"],1);assert!(evaluation_progress(&registry,"test.echo",hash).is_some());
    let stale=answer(request("",true,"wrong-version"));assert_eq!(stale["done"],true);assert!(stale["outputJson"].as_str().unwrap().contains("error"));assert!(evaluation_progress(&registry,"test.echo",hash).is_some());
    let mut phases=std::collections::BTreeSet::new();let mut turns=0;let completed=loop {let value=answer(request("",true,"test-v1"));turns+=1;assert!(turns<250000);phases.insert(value["phase"].as_str().unwrap().to_string());if value["done"]==true {break value;}};
    let output:serde_json::Value=serde_json::from_str(completed["outputJson"].as_str().unwrap()).unwrap();assert_eq!(output,serde_json::json!({"x":input}));assert!(evaluation_progress(&registry,"test.echo",hash).is_none());
    let orphan=answer(request("",true,"test-v1"));assert_eq!(orphan["done"],true);assert!(orphan["outputJson"].as_str().unwrap().contains("error"));
    assert_eq!(answer(request(&text,false,"test-v1"))["done"],false);let changed=answer(request(&text,false,wire["replacementVersion"].as_str().unwrap()));assert_eq!(changed["done"],false);assert_eq!(changed["phase"],wire["replacementPhases"][0]);let mut replacement_turns=0;loop {replacement_turns+=1;assert!(replacement_turns<250000);let value=answer(request("",true,wire["replacementVersion"].as_str().unwrap()));assert_eq!(value["done"],false);if value["phase"]==wire["replacementPhases"][1] {assert_eq!(value["unitsDone"],wire["restartUnits"][0]);assert_eq!(value["unitsTotal"],wire["restartUnits"][1]);break;}}let fresh=answer(request("",true,wire["replacementVersion"].as_str().unwrap()));assert_eq!(fresh["phase"],wire["initialPhase"]);assert_eq!(fresh["unitsDone"],1);assert!(cancel_evaluation(&registry,"test.echo",hash));while evaluation_retirement_pending(&registry){retire_cancelled_evaluations_close_step(&registry,1,3);}
    for target in law["phases"].as_array().unwrap().iter().map(|value|value.as_str().unwrap()).filter(|phase|!phase.starts_with("dependency-order")&&!phase.starts_with("dependency-canonical")) {let mut value=answer(request(&text,false,"test-v1"));let mut advances=0;while value["phase"]!=target && value["done"]==false {advances+=1;assert!(advances<250000);value=answer(request("",true,"test-v1"));}if value["done"]==true {continue;}assert!(cancel_evaluation(&registry,"test.echo",hash));let mut closes=0;while evaluation_retirement_pending(&registry){closes+=1;assert!(closes<250000);retire_cancelled_evaluations_close_step(&registry,1,3);}assert!(evaluation_progress(&registry,"test.echo",hash).is_none());}
    println!("[DEBUG] Existing evaluation map owns input admission and compact resumes, turns={turns}, phases={phases:?}");
}

#[test]
fn retained_evaluation_output_measures_writes_and_retires_the_same_typed_owner() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔁️extension-invocation-wire/🔣️.json")).unwrap();let law=&fixture["retainedOutput"];let registry=echo_registry("test.echo","x");
    let label=law["labelUnit"].as_str().unwrap().repeat(law["labelRepeats"].as_u64().unwrap() as usize);let input=serde_json::json!({"text":label,"nested":{"a":-3,"b":2.5,"c":true}});let text=serde_json::to_string(&input).unwrap();let hash=law["nodeHash"].as_u64().unwrap();
    let request=|resume|EvaluateRequest {operator_id:"test.echo".into(),input_json:if resume{String::new()}else{text.clone()},node_hash:hash,budget:1,wall_micros:1,dependency_json:String::new(),operator_version:law["operatorVersion"].as_str().unwrap().into(),cancellation_id:law["cancellationId"].as_str().unwrap().into(),round_units:1,resume};
    let answer=|resume|serde_json::from_str::<serde_json::Value>(&evaluate_step_envelope(&registry,request(resume)).wire).unwrap();let expected=serde_json::json!({"x":input});
    let mut phases=std::collections::BTreeSet::new();let mut turns=0;let mut value=answer(false);let mut previous_units=0;
    loop {turns+=1;assert!(turns<500000);let units=value["unitsDone"].as_u64().unwrap();assert!(units>=previous_units);previous_units=units;phases.insert(value["phase"].as_str().unwrap().to_string());if value["done"]==true {assert_eq!(value["unitsDone"],value["unitsTotal"]);assert_eq!(serde_json::from_str::<serde_json::Value>(value["outputJson"].as_str().unwrap()).unwrap(),expected);break;}value=answer(true);}
    assert!(evaluation_progress(&registry,"test.echo",hash).is_none());for phase in law["phases"].as_array().unwrap(){assert!(phases.contains(phase.as_str().unwrap()),"missing bounded {phase}");}
    for phase in law["phases"].as_array().unwrap().iter().map(|phase|phase.as_str().unwrap()) {
        let mut value=answer(false);let mut advances=0;while value["phase"]!=phase {advances+=1;assert!(advances<500000);assert_eq!(value["done"],false);value=answer(true);}
        {let mut jobs=evaluation_jobs().lock().unwrap();let retained=jobs.jobs.get_mut(&(registry.owner_identity(),"test.echo".into(),hash)).unwrap();let before=retained.progress();assert!(matches!(retained.advance(&registry,"test.echo",0,3).unwrap(),EvaluationStep::Working(_)));assert!(matches!(retained.advance(&registry,"test.echo",1,0).unwrap(),EvaluationStep::Working(_)));assert_eq!(before,retained.progress());}
        assert!(cancel_evaluation(&registry,"test.echo",hash));let mut closes=0;while evaluation_retirement_pending(&registry){closes+=1;assert!(closes<500000);assert!(matches!(retire_cancelled_evaluations_close_step(&registry,1,3),neural_engine::ValueRetirementStep::Pending {released_items:0..=1,released_bytes:0..=3}|neural_engine::ValueRetirementStep::Complete));}assert!(evaluation_progress(&registry,"test.echo",hash).is_none());
    }
    let mut value=answer(false);let mut advances=0;while value["phase"]!="output-measure" {advances+=1;assert!(advances<500000);assert_eq!(value["done"],false);value=answer(true);}let replacement=serde_json::json!({"text":"replacement 雪"});let mut changed=request(false);changed.input_json=serde_json::to_string(&replacement).unwrap();let replaced:serde_json::Value=serde_json::from_str(&evaluate_step_envelope(&registry,changed).wire).unwrap();assert_eq!(replaced["done"],false);assert_eq!(replaced["phase"],"retire-invalidated");let mut restarted=false;let mut advances=0;loop {advances+=1;assert!(advances<500000);let value=answer(true);if value["phase"]=="restart-admission" {restarted=true;assert_eq!(value["unitsDone"],0);}if value["done"]==true {assert!(restarted);assert_eq!(serde_json::from_str::<serde_json::Value>(value["outputJson"].as_str().unwrap()).unwrap(),serde_json::json!({"x":replacement}));break;}}
    let source:Dictionary=semio_framework_pack_json::from_json_str(&text,semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();let mut refused=EvaluationOutput::new(source);let mut admitted=|_|true;refused.encoding=Some(semio_framework_value::NativeEncodeControl::new(law["ownershipCeiling"].as_u64().unwrap()as usize,&mut admitted).pause().unwrap());assert!(refused.step(1,3,0).is_err());let phase=refused.phase();assert!(matches!(refused.close_step(0),neural_engine::ValueRetirementStep::Blocked));assert_eq!(refused.phase(),phase);let mut closes=0;while !refused.terminal_is_empty(){closes+=1;assert!(closes<500000);refused.close_step(3);}
    println!("[DEBUG] Existing SDK output owner retained measure/write/retire and phase cancellation, turns={turns}, phases={phases:?}");
}
