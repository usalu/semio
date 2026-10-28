//! 🧪️ Original producer fixture laws retain source identity and close every owner through the initiating caller.
use super::*;
use semio_framework_job::{OriginalCallerSourceParent,OperationId,Generation,StepBudget,close_original_retained_step};
use semio_framework_job_payload::JobPayloadAuthority;
use semio_framework_value::retirement::{RetireOwned,controlled::ControlledRetirement};

trait ByteOracle {fn representation(bytes:&[u8])->Vec<u8>;}
struct JsonByteOracle;
impl ByteOracle for JsonByteOracle {fn representation(bytes:&[u8])->Vec<u8>{serde_json::to_vec(bytes).unwrap()}}
fn now_us()->Option<u64>{Some(1000)}
fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:65536,maximum_copy_bytes:134217728,maximum_capacity_bytes:268435456,maximum_release_bytes:536870912,maximum_depth:4096}}
fn parent()->OriginalCallerSourceParent{let grant=stock();let(cancel,_)=semio_framework_job::CancelToken::admit_root(grant).unwrap().unwrap();let(authority,_)=JobPayloadAuthority::admit(OperationId(807),Generation(3),grant).unwrap().unwrap();OriginalCallerSourceParent::admit(authority,cancel,grant).ok().unwrap().0}
fn close<T:RetireOwned>(value:T,cx:&mut semio_framework_job::StepContext<'_>){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|_|panic!("original producer owners support controlled retirement"));for turn in 0..65536{if owner.terminal_is_empty(){break}let demand=owner.selected_demand(stock().maximum_copy_bytes).unwrap();if demand.release_bytes!=0{let denied=RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..cx.retained_grant()};let before=cx.retained_progress();let fuel=cx.fuel_remaining();let step=cx.with_retained_loan(denied,|same|close_original_retained_step(same,demand,|grant|owner.step(grant))).unwrap();assert_eq!(step.progress(),Default::default());assert_eq!(cx.retained_progress(),before);assert_eq!(cx.fuel_remaining(),fuel);}let before=cx.retained_progress();let step=close_original_retained_step(cx,demand,|grant|owner.step(grant)).unwrap();let progress=step.progress();assert_eq!(cx.retained_progress(),before.checked_add(progress).unwrap());assert!(turn<65535)}assert!(owner.terminal_is_empty());}

#[test]
fn original_producer_neutral_cases_preserve_or_transfer_the_exact_vector(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let id=row["id"].as_str().unwrap();let mut parent=parent();
  let length=row["length"].as_u64().map(|length|length as usize);let mut source=length.map(|length|{let mut bytes=Vec::with_capacity(row["capacity"].as_u64().unwrap_or(length as u64)as usize);bytes.extend((0..length).map(|index|(index%251)as u8));bytes});
  let expected=source.as_ref().map(|bytes|JsonByteOracle::representation(bytes));let source_identity=source.as_ref().map(|bytes|(bytes.as_ptr(),bytes.len(),bytes.capacity()));
  let chunks=ArtifactOutputChunks::new(row["maximum"].as_u64().unwrap_or(8192)as usize);
  if row["slotsFull"]==true{for _ in 0..chunks.inner.slots{chunks.push(vec![9]).unwrap();}}
  if row["allocationAdmitted"]==false{chunks.inner.state.try_lock().unwrap().chunks.allocation_admitted=false;}
  if row["sealed"]==true{chunks.seal().unwrap();}
  let credit=(row["credit"]!=false).then(||ArtifactMediaExportCredit::new(row["creditMaximum"].as_u64().unwrap_or(8192)as usize));
  if let Some(credit)=credit.as_ref(){credit.credit(row["creditInitial"].as_u64().unwrap_or(0)as usize).unwrap();if row["creditExceeded"]==true{assert!(credit.credit(credit.maximum+1).is_err());}}
  if row["cancelled"]==true{parent.cancellation().unwrap().cancel_now();}
  let queue_before=(chunks.bytes(),chunks.bytes_remaining(),chunks.chunks_remaining());let credit_before=credit.as_ref().map(|credit|(credit.bytes(),credit.exceeded()));
  let queue_guard=(row["busy"]=="queue").then(||chunks.inner.state.try_lock().unwrap());
  let credit_guard=(row["busy"]=="credit").then(||credit.as_ref().unwrap().state.writer.try_lock().unwrap());
  let mut sequence=0;let mut recipient=Default::default();let fuel=if row["grant"]=="fuel-zero"{0}else{32768};let mut cx=parent.borrow_context(StepBudget::new(fuel,1500,stock()),now_us,&mut sequence,&mut recipient).unwrap();let identity=cx.original_recipient_identity();
  let mut grant=stock();if row["grant"]=="items-zero"{grant.maximum_items=0}if row["grant"]=="copy-short"{grant.maximum_copy_bytes=ArtifactOutputChunks::original_push_demands().copy_bytes-1}if row["grant"]=="depth-zero"{grant.maximum_depth=0}
  let before=cx.retained_progress();let before_fuel=cx.fuel_remaining();let result=cx.with_retained_loan(grant,|same|chunks.push_original(&mut source,credit.as_ref(),same));
  let outcome=row["outcome"].as_str().unwrap();match outcome{"yield"=>assert!(matches!(result,Ok(false)),"{id}"),"refusal"=>assert!(result.is_err(),"{id}"),"published"=>assert!(matches!(result,Ok(true)),"{id}"),_=>unreachable!()}
  let charged=row["charged"].as_bool().unwrap();let progress=RetainedCloneProgress{copied_items:usize::from(charged),copied_bytes:if charged{ArtifactOutputChunks::original_push_demands().copy_bytes}else{0},..Default::default()};assert_eq!(cx.retained_progress(),before.checked_add(progress).unwrap(),"{id}");if let Err(failure)=&result{assert_eq!(failure.retained_progress(),progress,"{id}");}assert_eq!(cx.fuel_remaining(),before_fuel-u64::from(charged),"{id}");assert_eq!(cx.original_recipient_identity(),identity);
  drop(queue_guard);drop(credit_guard);
  if outcome=="published"{assert!(source.is_none(),"{id}");assert_eq!((chunks.bytes(),chunks.bytes_remaining(),chunks.chunks_remaining()),(queue_before.0+length.unwrap(),queue_before.1+length.unwrap(),queue_before.2+1));if let Some(credit)=credit.as_ref(){assert_eq!((credit.bytes(),credit.exceeded()),(credit_before.unwrap().0+length.unwrap(),false));}let state=chunks.inner.state.try_lock().unwrap();let queued=state.chunks.items.back().unwrap();assert_eq!((queued.as_ptr(),queued.len(),queued.capacity()),source_identity.unwrap());assert_eq!(JsonByteOracle::representation(queued),expected.unwrap());}
  else{assert_eq!(source.as_ref().map(|bytes|(bytes.as_ptr(),bytes.len(),bytes.capacity())),source_identity,"{id}");assert_eq!(source.as_ref().map(|bytes|JsonByteOracle::representation(bytes)),expected,"{id}");assert_eq!((chunks.bytes(),chunks.bytes_remaining(),chunks.chunks_remaining()),queue_before,"{id}");assert_eq!(credit.as_ref().map(|credit|(credit.bytes(),credit.exceeded())),credit_before,"{id}");}
  drop(result);drop(cx);let mut close_receipt=Default::default();let mut close_cx=parent.borrow_context(StepBudget::new(65536,1500,stock()),now_us,&mut sequence,&mut close_receipt).unwrap();let close_identity=close_cx.original_recipient_identity();close((source,chunks,credit),&mut close_cx);assert_eq!(close_cx.original_recipient_identity(),close_identity);drop(close_cx);
  for turn in 0..256{if parent.terminal_is_empty(){break}let step=parent.close_step(stock()).unwrap();assert!(step.progress().fits(stock()));assert!(turn<255)}assert!(parent.terminal_is_empty());
 }
}

#[test]
fn direct_credit_and_original_producer_share_one_nonblocking_writer_authority(){
 let credit=ArtifactMediaExportCredit::new(8);let guard=credit.state.writer.try_lock().unwrap();assert!(credit.credit(3).is_err());assert_eq!(credit.bytes(),0);assert!(!credit.exceeded());drop(guard);assert_eq!(credit.credit(3).unwrap(),3);assert!(credit.credit(6).is_err());assert_eq!(credit.bytes(),3);assert!(credit.exceeded());
}

#[test]
fn original_seal_neutral_cases_preserve_every_queue_owner(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["sealCases"].as_array().unwrap(){
  let id=row["id"].as_str().unwrap();let mut parent=parent();let chunks=ArtifactOutputChunks::new(4096);chunks.push(vec![7,8,9]).unwrap();if row["sealed"]==true{chunks.seal().unwrap();}if row["cancelled"]==true{parent.cancellation().unwrap().cancel_now();}
  let original=(chunks.bytes(),chunks.bytes_remaining(),chunks.chunks_remaining());let pointer=chunks.inner.state.try_lock().unwrap().chunks.items.front().unwrap().as_ptr();let guard=(row["busy"]=="queue").then(||chunks.inner.state.try_lock().unwrap());
  let mut sequence=0;let mut recipient=Default::default();let fuel=if row["grant"]=="fuel-zero"{0}else{32768};let mut cx=parent.borrow_context(StepBudget::new(fuel,1500,stock()),now_us,&mut sequence,&mut recipient).unwrap();let identity=cx.original_recipient_identity();
  let mut grant=stock();if row["grant"]=="items-zero"{grant.maximum_items=0}if row["grant"]=="copy-short"{grant.maximum_copy_bytes=ArtifactOutputChunks::original_seal_demands().copy_bytes-1}if row["grant"]=="depth-zero"{grant.maximum_depth=0}
  let before=cx.retained_progress();let before_fuel=cx.fuel_remaining();let result=cx.with_retained_loan(grant,|same|chunks.seal_original(same));let outcome=row["outcome"].as_str().unwrap();match outcome{"yield"=>assert!(matches!(result,Ok(false)),"{id}"),"refusal"=>assert!(result.is_err(),"{id}"),"sealed"=>assert!(matches!(result,Ok(true)),"{id}"),_=>unreachable!()}
  let charged=row["charged"].as_bool().unwrap();let progress=RetainedCloneProgress{copied_items:usize::from(charged),copied_bytes:if charged{ArtifactOutputChunks::original_seal_demands().copy_bytes}else{0},..Default::default()};assert_eq!(cx.retained_progress(),before.checked_add(progress).unwrap(),"{id}");assert_eq!(cx.fuel_remaining(),before_fuel-u64::from(charged));assert_eq!(cx.original_recipient_identity(),identity);drop(guard);
  assert_eq!((chunks.bytes(),chunks.bytes_remaining(),chunks.chunks_remaining()),original);assert_eq!(chunks.inner.state.try_lock().unwrap().chunks.items.front().unwrap().as_ptr(),pointer);assert_eq!(chunks.is_sealed(),row["sealed"]==true||outcome=="sealed");drop(result);drop(cx);
  let mut close_receipt=Default::default();let mut close_cx=parent.borrow_context(StepBudget::new(65536,1500,stock()),now_us,&mut sequence,&mut close_receipt).unwrap();close(chunks,&mut close_cx);drop(close_cx);for turn in 0..256{if parent.terminal_is_empty(){break}parent.close_step(stock()).unwrap();assert!(turn<255)}assert!(parent.terminal_is_empty());
 }
}

#[test]
fn original_ports_refuse_an_unsupplied_context_without_moving_its_source(){
 let mut source=Some(vec![1,2,3]);let pointer=source.as_ref().unwrap().as_ptr();let chunks=ArtifactOutputChunks::new(4096);let mut sequence=0;let mut recipient=Default::default();let mut cx=semio_framework_job::StepContext::new(OperationId(807),Generation(3),StepBudget::new(32768,1500,stock()),semio_framework_job::root_cancel_token(),now_us,&mut sequence,&mut recipient);
 assert!(chunks.push_original(&mut source,None,&mut cx).is_err());assert_eq!(source.as_ref().unwrap().as_ptr(),pointer);assert_eq!(chunks.bytes(),0);assert_eq!(chunks.chunks_remaining(),0);assert!(chunks.seal_original(&mut cx).is_err());assert!(!chunks.is_sealed());assert_eq!(cx.retained_progress(),Default::default());
}
