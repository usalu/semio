//! 🧩️ A real produced Puzzle actor must export material, cancel interior work and still dispatch its editor.
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;
#[path="⚙️driver/🦀️.rs"]mod driver;
#[path="🧱️material/🦀️.rs"]mod material;
#[path="👆️interaction/🦀️.rs"]mod interaction;
use semio_framework_actor::OriginalJobOutputState;
fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🧩️guest-io/🔣️.json")).unwrap()}
fn actual_model(law:&serde_json::Value)->Vec<u8>{
 let read=|name:&str|std::fs::read(std::env::var_os(name).unwrap_or_else(||panic!("{name} must come from the genuine model-owning test-guest-io ingress"))).unwrap();
 let bytes=read("SEMIO_TEST_ACTOR_COMPONENT");let receipt:serde_json::Value=serde_json::from_slice(&read("SEMIO_TEST_ACTOR_RECEIPT")).unwrap();let descriptor=read("SEMIO_TEST_ACTOR_DESCRIPTOR");
 for key in ["pluginId","packageId","profile","target"]{assert_eq!(receipt[key],law["component"][key],"actual actor receipt {key}");}assert_eq!(receipt["wasmBytes"].as_u64(),Some(bytes.len()as u64));assert!(!descriptor.is_empty());assert_eq!(&bytes[..8],&[0,97,115,109,13,0,1,0]);bytes
}
fn execute_progress(bytes:&[u8],law:&serde_json::Value)->bool{
 let Ok(value)=serde_json::from_slice::<serde_json::Value>(bytes)else{return false};
 let Some(completed)=value["completed"].as_u64()else{return false};let Some(total)=value["total"].as_u64()else{return false};
 value["phase"]==law["progress"]["phase"]&&completed>=law["progress"]["minimumCompleted"].as_u64().unwrap()&&completed<total
}
#[test]
fn real_puzzle_actor_exports_material_and_cancels_execute_before_real_editor_dispatch(){
 let law=fixture();let bytes=actual_model(&law);let material=material::prepare(&law);let mut actor=driver::Actor::open(&bytes,&law);
 let mut observed=|_:semio_framework_value::native_encoding::NativeEncodeProgress|true;let mut recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();let mut identity=semio_plugin_host_operation::test_native_authority::original(&mut observed,&mut recipient);
 let session=interaction::setup(&mut actor,&law,&mut identity);
 actor.start(material.source(1));let mut complete=false;
 for _ in 0..law["execution"]["maximumTurns"].as_u64().unwrap(){if let Some(outcome)=actor.step(1){let state=outcome.state;let output=actor.output(outcome);match state{OriginalJobOutputState::Running=>{},OriginalJobOutputState::Done=>{material.verify(&output);complete=true;break},OriginalJobOutputState::Failed=>panic!("actual completed export failed: {}",String::from_utf8_lossy(&output))}}}
 assert!(complete,"actual large material export completes");actor.drain(1,true,&law);assert!(actor.closed_sources.contains(&1));
 actor.start(material.source(2));let mut interior=false;let mut already_completed=false;
 for _ in 0..law["execution"]["maximumTurns"].as_u64().unwrap(){if let Some(outcome)=actor.step(2){let state=outcome.state;let output=actor.output(outcome);match state{OriginalJobOutputState::Running=>{if execute_progress(&output,&law){interior=true;break}},OriginalJobOutputState::Done=>{material.verify(&output);already_completed=true;break},OriginalJobOutputState::Failed=>panic!("actual cancellation candidate failed: {}",String::from_utf8_lossy(&output))}}}
 if interior{actor.cancel(2);actor.drain(2,false,&law)}else if already_completed{actor.drain(2,true,&law)}else{actor.cancel(2);actor.drain(2,false,&law)}
 assert!(actor.closed_sources.contains(&2),"actual guest source closure is distinct from cancellation admission");
 interaction::assert_after_cancel(&mut actor,&law,&mut identity,&session);
 actor.close_jobs();interaction::close(&mut actor,&law,&mut identity,session);actor.close();semio_plugin_host_operation::test_native_authority::close(&mut identity);
 assert!(interior,"actual conversion must publish positive execute-phase progress before completion; the decoded hop identity alone does not qualify interior cancellation");
}
