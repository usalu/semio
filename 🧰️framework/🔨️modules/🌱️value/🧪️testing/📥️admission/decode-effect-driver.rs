//! 🪙️ The decode driver of the shared effect-provider corpus: admitted, performed == committed <= ceiling, nothing live afterwards.
use semio_framework_value_testing_admission::{ReceiveFrame,RetainedCloneGrant,RetainedWorkOutcome,ValueError,WorkLoan};
use semio_framework_value_native_decoding::native_decoding::NativeDecodeControl;
#[test]
fn decode_control_with_effect_agrees_with_the_shared_provider_corpus(){
 let corpus:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️tests/🪙️effect-provider/🧫️fixtures/🔣️.json")).unwrap();
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:8};
 for case in corpus["cases"].as_array().unwrap(){
  let maximum=case["maximumWork"].as_u64().unwrap();let performs:Vec<u64>=case["performs"].as_array().unwrap().iter().map(|step|step.as_u64().unwrap()).collect();let performed=u128::from(case["performed"].as_u64().unwrap());
  let remaining=1u128<<20;let mut frame=ReceiveFrame::new(grant,WorkLoan::new(&remaining,1<<20));let mut callback=|_|true;let mut control=NativeDecodeControl::new(4096,&mut callback);
  let turn=frame.turn(&mut control,|control|{
   let admitted=control.with_effect(grant,maximum,|_,effect|{assert!(performs.iter().all(|step|effect.performed(*step).unwrap()));effect.performed_work()});
   RetainedWorkOutcome{result:admitted.map(|seen|(seen,control.has_pending_work())),work:0}
  });
  let(seen,live)=turn.result.map_err(|error:ValueError|error).unwrap();
  assert_eq!(seen,Some(performed),"{}",case["name"]);assert!(performed<=u128::from(maximum));assert!(!live,"{} leaves a live action",case["name"]);
 }
}
