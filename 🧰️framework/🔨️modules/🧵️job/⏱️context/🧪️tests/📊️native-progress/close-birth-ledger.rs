//! ⚖️ A release settles against the ledger that recorded the birth of the freed backing, so the closing caller is charged only for what no ledger recorded.
use super::{caller,quantum};
use semio_framework_value::{birth_ledger,RetainedCloneProgress};
use serde_json::Value;

#[test]
fn a_release_is_charged_to_the_ledger_that_recorded_its_birth(){
 let law:Value=serde_json::from_str(include_str!("🧫️fixtures/close-birth-ledger.json")).unwrap();let n=|value:&Value|value.as_u64().unwrap() as usize;
 for case in law["cases"].as_array().unwrap(){
  let id=case["id"].as_str().unwrap();birth_ledger::settle_original_birth(usize::MAX);birth_ledger::record_original_birth(n(&case["born"]));
  let mut caller=caller();let mut charged=Vec::new();
  for release in case["releases"].as_array().unwrap(){
   let release=n(release);let before=caller.cumulative_receipt().released_bytes;
   caller.bare_turn(quantum(),|cx|{let start=cx.retained_progress();cx.consume_retained(RetainedCloneProgress{copied_items:1,released_bytes:release,..Default::default()})?;let settled=cx.settle_original_birth(release,start)?;assert!(settled<=release,"{id} settled more than it released");Ok(())}).unwrap();
   charged.push(caller.cumulative_receipt().released_bytes-before);
  }
  assert_eq!(charged,case["business"].as_array().unwrap().iter().map(n).collect::<Vec<_>>(),"{id} charged the wrong caller share");
  assert_eq!(birth_ledger::original_birth_ledger().outstanding(),n(&case["outstanding"]),"{id} left the wrong births outstanding");
  caller.retire(quantum());
 }
}
