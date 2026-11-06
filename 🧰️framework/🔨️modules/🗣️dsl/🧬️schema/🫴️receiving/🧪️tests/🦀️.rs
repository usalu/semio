//! 🧪️ Genuine decoder work admission precedes scalar validation and successful payload publication.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
#[test]
fn borrowed_scalar_original_work_preserves_invalid_and_denied_receipts(){
 let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::repo_text!("🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/🧪️tests/scalar.json")).unwrap();
 let g=RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64};
 for row in fixture["cases"].as_array().unwrap(){
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(901),Generation(23),g,||Some(1),u64::MAX,1<<20);let mut decoder=OriginalJobDecodeOwner::empty();let mut captured=None;
  caller.turn_with_decoder(g,&mut decoder,|cx|cx.with_original_work_loan(row["fuel"].as_u64().unwrap(),u64::MAX,|cx|{
   let(turn,progress,output)=cx.with_native_receiving::<_,ValueError>(|native|{let mut owner=NativeSnapshotDecodeOwner::new(native,g);let mut output=None;let turn=u8::receive_value(&FieldValue::UInt(row["value"].as_u64().unwrap()),&mut output,&mut owner);Ok((turn,owner.progress(),output))})?;
   assert_eq!(turn.work,row["work"].as_u64().unwrap());assert_eq!(turn.result.is_ok(),row["valid"].as_bool().unwrap());assert_eq!(output,row["valid"].as_bool().unwrap().then(||row["value"].as_u64().unwrap()as u8));assert_eq!(progress.copied_bytes,row["copy"].as_u64().unwrap()as usize);cx.consume_fuel(turn.work);captured=Some(turn.result.err());Ok(())
  })).unwrap();
  let mut errors=semio_framework_value::retirement::controlled::ControlledRetirement::retain(captured.take().unwrap());
  for _ in 0..4096{if errors.terminal_is_empty(){break}caller.turn(g,|cx|cx.with_original_retirement(|cx|{let demand=errors.selected_demand(g.maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|{let result=errors.step(grant);RetainedWorkOutcome{result,work:errors.last_turn_work()}})?;Ok(())})).unwrap();}
  assert!(errors.terminal_is_empty());
  for _ in 0..4096{if decoder.terminal_is_empty(){break}caller.turn(g,|cx|cx.with_original_retirement(|cx|{let demand=decoder.retirement_demands(g.maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|decoder.close_step(grant))?;Ok(())})).unwrap();}
  assert!(decoder.terminal_is_empty());caller.retire(g);
 }
}
