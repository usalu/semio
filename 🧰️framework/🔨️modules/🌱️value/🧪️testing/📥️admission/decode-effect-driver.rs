//! 🪙️ The decode driver of the shared effect-provider corpus: admitted, performed == committed <= ceiling, nothing live afterwards.
use semio_framework_value_testing_admission::{Authorities,RetainedCloneGrant};
use semio_framework_value_native_decoding::native_decoding::NativeDecodeControl;
#[test]
fn decode_control_with_effect_agrees_with_the_shared_provider_corpus(){
 let corpus:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🧵️job/⏱️context/🧪️tests/🪙️effect-provider/🧫️fixtures/🔣️.json")).unwrap();
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:8};
 for case in corpus["cases"].as_array().unwrap(){
  let maximum=case["maximumWork"].as_u64().unwrap();let performs:Vec<u64>=case["performs"].as_array().unwrap().iter().map(|step|step.as_u64().unwrap()).collect();let performed=u128::from(case["performed"].as_u64().unwrap());
  let mut payer=Authorities::new();let mut callback=|_|true;let mut control=NativeDecodeControl::new(4096,&mut callback);
  let seen=payer.effect(&mut control,grant,maximum,|effect|{assert!(performs.iter().all(|step|effect.performed(*step).unwrap()));effect.performed_work()});
  assert_eq!(seen,Some(performed),"{}",case["name"]);assert!(performed<=u128::from(maximum));assert!(!control.has_pending_work(),"{} leaves a live action",case["name"]);
 }
}
