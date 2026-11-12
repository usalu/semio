//! 📦️ Real sealed pages retain exact backing and ledger custody through funded detachment and close.
use semio_framework_job_payload::*;
use semio_framework_trace::{OperationId,Generation,observe_heap_allocations_on_this_thread as observe};
#[path="../../../📦️builder/🧪️testing/🦀️.rs"]
mod payload_case;
use payload_case::PayloadCase;
#[test]
fn original_page_transfer_keeps_exact_backing_ledger_and_independent_json_under_one_bank(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../case.json")).unwrap();
 let schema:serde_json::Value=serde_json::from_str(include_str!("../case.schema.json")).unwrap();
 for(key,value)in law.as_object().unwrap(){assert_eq!(&schema["properties"][key]["const"],value);}
 for row in law["cases"].as_array().unwrap(){
  let mut bank=PayloadCase::new(OperationId(7602),Generation(4));let mut builder=bank.initialize(JobPayloadStream::CommitOutput);
  for value in row["pages"].as_array().unwrap(){bank.page(&mut builder,&serde_json::to_vec(value).unwrap());}
  let mut payload=bank.finish(&mut builder);let index=payload.page_count()-1;let pointer=payload.page(index).unwrap().as_ptr();let ledger=payload.payload_authority_identity();let mut output=None;
  for axis in law["deniedAxes"].as_array().unwrap(){
   let d=payload.original_page_transfer_demand(&output).unwrap();let mut g=PayloadCase::exact(d);if axis=="items"{g.maximum_items=0}else{g.maximum_depth=0}
   let(step,heap)=observe(||bank.close_with(g,1,|effect|payload.transfer_last_original_page(&mut output,effect)));assert_eq!(step.progress(),Default::default());assert!(output.is_none());assert_eq!(payload.page(index).unwrap().as_ptr(),pointer);assert_eq!(payload.payload_authority_identity(),ledger);assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  }
  for turn in 0..row["transferTurns"].as_u64().unwrap(){
   let g=PayloadCase::exact(payload.original_page_transfer_demand(&output).unwrap());let mut observed=None;let step=bank.close_with(g,1,|effect|{let(outcome,heap)=observe(||payload.transfer_last_original_page(&mut output,effect));observed=Some((outcome.work,heap));outcome});let(work,heap)=observed.unwrap();assert_eq!(work,u128::from(law["metadataWork"].as_u64().unwrap()));assert_eq!(step.progress().copied_bytes,g.maximum_copy_bytes);assert_eq!(step.progress().copied_items,1);assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(output.as_ref().unwrap().is_ready(),turn+1==row["transferTurns"].as_u64().unwrap());
  }
  let transfer=output.as_ref().unwrap();assert_eq!(transfer.original_index(),index);assert_eq!(transfer.page_identity(),Some(pointer));assert_eq!(transfer.ledger_identity(),ledger);assert_eq!(serde_json::from_slice::<serde_json::Value>(transfer.bytes().unwrap()).unwrap(),row["pages"][index]);assert_eq!(payload.page_count(),row["remainingPages"].as_u64().unwrap()as usize);
  let g=PayloadCase::exact(payload.original_page_transfer_demand(&output).unwrap());let cached=bank.close_with(g,1,|effect|payload.transfer_last_original_page(&mut output,effect));assert_eq!(cached.progress(),Default::default());
  let transfer=output.as_mut().unwrap();let d=transfer.retirement_demands();let denied=RetainedCloneGrant{maximum_release_bytes:d.release_bytes.saturating_sub(1),..PayloadCase::exact(d)};let(step,heap)=observe(||bank.close_with(denied,1,|effect|transfer.close_step(effect)));assert_eq!(step.progress(),Default::default());assert_eq!(transfer.page_identity(),Some(pointer));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){if transfer.terminal_is_empty(){break}let g=PayloadCase::exact(transfer.retirement_demands());let mut heap=None;let step=bank.close_with(g,1,|effect|{let(turn,h)=semio_framework_value_testing_allocator::observe_retirement_allocations(||transfer.close_step(effect));heap=Some(h);turn});let h=heap.unwrap();assert_eq!(h,(step.progress().retained_capacity_bytes,step.progress().released_bytes));}
  assert!(transfer.terminal_is_empty());let(_,heap)=observe(||drop(output.take()));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  for _ in 0..law["maximumTurns"].as_u64().unwrap(){if payload.terminal_is_empty(){break}let d=payload.retirement_demands().unwrap();let work=payload.retirement_work().unwrap();bank.close(d,work,|effect|payload.close_step(effect));}assert!(payload.terminal_is_empty());drop(payload);drop(builder);let(ledger,mut bank)=bank.into_ledger();bank.close_ledger(&mut Some(ledger));drop(bank);
 }
}
