//! 🧮️ Retained schema projection agrees with the shared vector fixtures.
use super::*;
use semio_os_kernel_neural_engine_model::{FieldSpec,OperatorJobStep};

fn same_json(left:&serde_json::Value,right:&serde_json::Value)->bool{match(left,right){(serde_json::Value::Number(a),serde_json::Value::Number(b))=>a.as_f64()==b.as_f64(),(serde_json::Value::Array(a),serde_json::Value::Array(b))=>a.len()==b.len()&&a.iter().zip(b).all(|(x,y)|same_json(x,y)),(serde_json::Value::Object(a),serde_json::Value::Object(b))=>a.len()==b.len()&&a.iter().all(|(key,value)|b.get(key).is_some_and(|other|same_json(value,other))),_=>left==right}}

fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64}}
fn vector_registry()->Registry{let mut registry=Registry::new();registry.register_schema(Schema{id:"vector".into(),module:"math".into(),name:"Vector".into(),icon:String::new(),summary:String::new(),fields:vec![FieldSpec::decimal_default("x",0.0),FieldSpec::decimal_default("y",0.0),FieldSpec::decimal_default("z",0.0)]});registry.finalize();registry}
fn admit(registry:&Registry,id:&str,value:&serde_json::Value)->Box<dyn semio_os_kernel_neural_engine_model::OperatorJob>{let mut input=Some(serde_json::from_value::<Dictionary>(value.clone()).unwrap());let mut cursor=OperatorPlanCursor::new();for _ in 0..100000{match registry.dispatch_job(id,&mut input,&mut cursor,stock()).unwrap(){OperatorPlanStep::Working=>{},OperatorPlanStep::Admitted(OperatorPlanAdmission::Job(job))=>return job,_=>panic!("expected retained job")}}panic!("plan did not admit")}
fn close(job:&mut Box<dyn semio_os_kernel_neural_engine_model::OperatorJob>,caller:&mut EffectCaller){for _ in 0..100000{if job.terminal_is_empty(){return}caller.poll(stock(),WORK,|effect|job.close_step(effect)).unwrap();}panic!("job did not close")}
fn evaluate(registry:&Registry,id:&str,input:&serde_json::Value)->serde_json::Value{let mut job=admit(registry,id,input);let mut caller=EffectCaller::new();let mut result=None;for _ in 0..100000{match caller.turn(stock(),WORK,|cx|job.step(cx).unwrap().0){OperatorJobStep::Working(_)=>{},OperatorJobStep::Done(output)=>{result=Some(ColdOwner::new(output));break},OperatorJobStep::Cancelled(_)=>panic!("unexpected cancellation")}}let output=result.expect("job completes");let result=serde_json::to_value(&*output).unwrap();close(&mut job,&mut caller);drop(job);drop(output);result}

#[test]
fn retained_vector_schema_projects_construct_deconstruct_modify_and_missing(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/schema-plan.json")).unwrap();
 let grant=RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};
 for case in cases.as_array().unwrap(){
  let mut registry=Registry::new();
  registry.register_schema(Schema{id:"vector".into(),module:"math".into(),name:"Vector".into(),icon:String::new(),summary:String::new(),fields:vec![FieldSpec::decimal_default("x",0.0),FieldSpec::decimal_default("y",0.0),FieldSpec::decimal_default("z",0.0)]});
  registry.finalize();
  let mut input=Some(serde_json::from_value::<Dictionary>(case["input"].clone()).unwrap());
  let mut cursor=OperatorPlanCursor::new();let mut job=None;
  for _ in 0..100000{match registry.dispatch_job("math.vector",&mut input,&mut cursor,grant).unwrap(){OperatorPlanStep::Working=>{},OperatorPlanStep::Admitted(OperatorPlanAdmission::Job(value))=>{job=Some(value);break},_=>panic!("{} did not admit a retained job",case["name"] )}}
  let mut job=job.expect("retained schema job");let mut caller=EffectCaller::new();let mut output=None;
  for _ in 0..100000{let step=caller.turn(grant,WORK,|cx|job.step(cx).unwrap().0);match step{OperatorJobStep::Done(value)=>{output=Some(value);break},OperatorJobStep::Working(_)=>{},OperatorJobStep::Cancelled(_)=>panic!("unexpected cancellation")}}
  let output=ColdOwner::new(output.expect("schema job completes"));let actual=serde_json::to_value(&*output).unwrap();assert!(same_json(&actual,&case["expected"]),"{}: {actual}",case["name"]);
  for _ in 0..100000{if job.terminal_is_empty(){break}caller.poll(grant,WORK,|effect|job.close_step(effect)).unwrap();}
  assert!(job.terminal_is_empty());drop(job);drop(output);registry.retire_cold();
 }
}

#[test]
fn malformed_channels_and_instance_errors_match_cold_registry(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/schema-errors.json")).unwrap();let registry=vector_registry();
 for case in cases.as_array().unwrap(){let input=ColdOwner::new(serde_json::from_value::<Dictionary>(case["input"].clone()).unwrap());let cold=ColdOwner::new(registry.dispatch("math.vector",&input).unwrap());let expected=serde_json::to_value(&*cold).unwrap();let actual=evaluate(&registry,"math.vector",&case["input"]);assert!(same_json(&actual,&expected),"{}: {actual}",case["name"]);assert_eq!(actual["errors"]["0"]["value"],case["message"]);}
 registry.retire_cold();
}

#[test]
fn cancellation_during_apply_and_output_closes_original_owners(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/schema-plan.json")).unwrap();
 for phase in ["schema-apply","schema-output"]{let registry=vector_registry();let mut job=admit(&registry,"math.vector",&cases[0]["input"]);let mut caller=EffectCaller::new();let mut reached=false;for _ in 0..100000{if job.progress().phase==phase{reached=true;break}match caller.turn(stock(),WORK,|cx|job.step(cx).unwrap().0){OperatorJobStep::Working(_)=>{},OperatorJobStep::Done(value)=>{drop(ColdOwner::new(value));panic!("completed before {phase}")},OperatorJobStep::Cancelled(_)=>panic!("early cancellation")}}assert!(reached,"missing {phase}");assert!(matches!(caller.turn(stock(),WORK,|cx|job.step(cx).unwrap().0),OperatorJobStep::Working(_)),"{phase} finished before cancellation");job.cancel();assert!(matches!(caller.turn(stock(),WORK,|cx|job.step(cx).unwrap().0),OperatorJobStep::Cancelled(_)));close(&mut job,&mut caller);drop(job);registry.retire_cold();}
}

#[test]
fn text_nested_schema_and_list_match_cold_registry(){
 let case:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/schema-composite.json")).unwrap();let mut registry=Registry::new();
 registry.register_schema(Schema{id:"member".into(),module:"test".into(),name:"Member".into(),icon:String::new(),summary:String::new(),fields:vec![FieldSpec::new("label",ValueType::Text),FieldSpec::new("profile",ValueType::Schema("profile".into())),FieldSpec::new("tags",ValueType::List(Box::new(ValueType::Text)))]});registry.finalize();
 let input=ColdOwner::new(serde_json::from_value::<Dictionary>(case["input"].clone()).unwrap());let cold=ColdOwner::new(registry.dispatch("test.member",&input).unwrap());let expected=serde_json::to_value(&*cold).unwrap();let actual=evaluate(&registry,"test.member",&case["input"]);assert!(same_json(&actual,&expected),"{actual}");assert!(same_json(&actual,&case["expected"]),"{actual}");drop(cold);drop(input);registry.retire_cold();
}
