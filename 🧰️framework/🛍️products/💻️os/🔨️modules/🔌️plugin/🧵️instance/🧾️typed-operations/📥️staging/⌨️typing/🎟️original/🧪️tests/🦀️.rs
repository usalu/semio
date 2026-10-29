//! 🧪️ Actual original typing transfer preserves transaction and mutation backings against the neutral policy fixture.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
use semio_framework_value::{RetainedCloneGrant,retirement::controlled::ControlledRetirement};
#[global_allocator]
static TYPING_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
fn clock()->Option<u64>{Some(1)}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:4096}}
fn caller()->OriginalJobFixtureCaller{OriginalJobFixtureCaller::admit(OperationId(947),Generation(47),RetainedCloneGrant{maximum_items:1<<24,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096},clock,u64::MAX,1<<24)}
fn turn<T>(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner,g:RetainedCloneGrant,run:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->T{let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn_with_decoder(g,decoder,run));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result.unwrap()}
fn retire<T:RetireOwned+Send+'static>(value:T,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){let mut owner=ControlledRetirement::retain(value);for step in 0..1000000{if owner.terminal_is_empty(){return}let d=owner.next_demand(32768).unwrap();turn(caller,decoder,grant(),|cx|cx.with_original_retirement(|cx|{close_original_retained_step(cx,d,|g|owner.step(g))?;Ok(())}));assert!(step<999999)}}
fn close(source:&mut Option<Box<OriginalTypingCommit<String>>>,caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){for step in 0..1000000{if source.is_none(){return}turn(caller,decoder,grant(),|cx|cx.with_original_retirement(|cx|{if source.as_ref().unwrap().terminal_is_empty(){let d=OriginalTypingCommit::<String>::release_demands();close_original_retained_step(cx,d,|_|{drop(source.take());Ok(RetainedCloneStep::Complete(receipt(d)))})?;}else{source.as_mut().unwrap().close_step(cx)?;}Ok(())}));assert!(step<999999)}}
fn finish(mut caller:OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){while !decoder.terminal_is_empty(){let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant(),|cx|cx.with_original_retirement(|cx|{let d=decoder.retirement_demands(32768)?;close_original_retained_step(cx,d,|g|decoder.close_step(g))?;Ok(())})));let p=caller.receipt();assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));result.unwrap()}caller.retire(grant());}
fn seed(rows:&serde_json::Value)->TypingLedger<String>{let mut ledger=TypingLedger::default();for row in rows.as_array().unwrap(){let leaves=row["leaves"].as_array().unwrap().iter().map(|v|v.as_str().unwrap().to_owned()).collect();ledger.send(row["window"].as_str().unwrap(),row["tool"].as_str().unwrap(),&protocol::ActorId("fixture".into()),semio_framework_tool_machine::TypingInput::Edit{buffer:row["buffer"].as_str().unwrap().into(),leaves},|_,next|semio_framework_tool_machine::TypingFold::Net(next.to_vec()),protocol::HybridLogicalTimestamp{actor:0,physical_ms:row["deadline"].as_u64().unwrap()-750,logical:0}).unwrap();let original=ledger.open(row["window"].as_str().unwrap()).unwrap();assert_eq!(serde_json::json!(original.states),row["states"]);assert_eq!(original.transaction.tool,original.tool);}ledger}
#[test]
fn original_typing_policy_and_move_only_rows_match_the_neutral_contract(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for case in law["cases"].as_array().unwrap(){
  let mut ledger=seed(&law["rows"]);let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let(mut windows,mut tools,mut leaves)=(Vec::new(),Vec::new(),Vec::new());
  let control=case["pass"].as_str()==Some("control");let(raw_suppress,raw_all)=classify_typing_dispatch(case["typing"].as_bool().unwrap(),case["commit"].as_str(),case["view"].as_bool().unwrap());let(suppress,all)=(control&&raw_suppress,!control&&raw_all);let roster_active=control&&case["typing"].as_bool().unwrap()&&(case["commit"].is_null()||raw_suppress);
  for row in law["rows"].as_array().unwrap(){
   let window=row["window"].as_str().unwrap();assert_eq!(ledger.open(window).unwrap().deadline_ms,row["deadline"].as_u64().unwrap());let roster=case["roster"].as_array().unwrap();let present=!roster_active||window.is_empty()||roster.is_empty()||roster.iter().any(|v|v.as_str()==Some(window));
   if original_typing_commit_required(all,suppress&&case["window"].as_str()==Some(window),row["deadline"].as_u64().unwrap(),Some(case["now"].as_u64().unwrap()),!control,Some(present))!=Some(true){continue}
   let original=ledger.open(window).unwrap();let identities:Vec<_>=original.entries.iter().map(|(_,m)|m.as_ptr()).collect();let tx=original.transaction.id.as_ptr();let key=ledger.resident_key(window).unwrap();let alias=ledger.next_retained_window(None).and_then(|_|{let mut after=None;loop{let next=ledger.next_retained_window(after);match next{Some((name,key))if name.as_str()==window=>break Some((name,key)),Some((name,_))=>after=Some(name.as_str()),None=>break None}}}).unwrap().0;
   let mut source=None;for denied in [RetainedCloneGrant{maximum_items:0,..grant()},RetainedCloneGrant{maximum_copy_bytes:0,..grant()},RetainedCloneGrant{maximum_capacity_bytes:0,..grant()},RetainedCloneGrant{maximum_depth:0,..grant()}]{assert!(!turn(&mut caller,&mut decoder,denied,|cx|OriginalTypingCommit::admit(&ledger,alias,key,cx,&mut source)));assert!(source.is_none());assert_eq!(caller.receipt(),Default::default());}
   assert!(turn(&mut caller,&mut decoder,grant(),|cx|OriginalTypingCommit::admit(&ledger,alias,key,cx,&mut source)));
   let mut foreign=OriginalJobDecodeOwner::empty();turn(&mut caller,&mut foreign,grant(),|cx|{assert!(source.as_mut().unwrap().advance(&mut ledger,cx).is_err());Ok(())});assert!(foreign.terminal_is_empty());
   for step in 0..1000000{if source.as_ref().unwrap().ready(){break}assert!(!turn(&mut caller,&mut decoder,RetainedCloneGrant{maximum_items:0,..grant()},|cx|source.as_mut().unwrap().advance(&mut ledger,cx)));assert_eq!(caller.receipt(),Default::default());turn(&mut caller,&mut decoder,grant(),|cx|source.as_mut().unwrap().advance(&mut ledger,cx));assert!(step<999999)}
   let(mut transaction,mut values)=(None,None);assert!(turn(&mut caller,&mut decoder,grant(),|cx|source.as_mut().unwrap().take_into(&ledger,&mut transaction,&mut values,cx)));assert_eq!(transaction.as_ref().unwrap().id.as_ptr(),tx);assert_eq!(values.as_ref().unwrap().iter().map(|value|value.as_ptr()).collect::<Vec<_>>(),identities);assert!(ledger.open(window).is_none());
   windows.push(window.to_owned());tools.push(transaction.as_ref().unwrap().tool.clone());leaves.push(values.as_ref().unwrap().clone());close(&mut source,&mut caller,&mut decoder);retire(transaction,&mut caller,&mut decoder);retire(values,&mut caller,&mut decoder);
  }
  assert_eq!(serde_json::json!(windows),case["expected"]);assert_eq!(suppress,case["suppress"].as_bool().unwrap());println!("[DEBUG] [original-typing] {}",serde_json::json!({"name":case["name"],"result":{"windows":windows,"leaves":leaves,"tools":tools,"suppress":suppress}}));finish(caller,&mut decoder);
 }
}
#[test]
fn original_typing_cancel_keeps_every_accepted_owner_until_paid_close(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for cut in law["cancelAfter"].as_array().unwrap(){
  let mut ledger=seed(&law["rows"]);let mut caller=caller();let mut decoder=OriginalJobDecodeOwner::empty();let(window,key)=ledger.next_retained_window(None).unwrap();let mut source=None;turn(&mut caller,&mut decoder,grant(),|cx|OriginalTypingCommit::admit(&ledger,window,key,cx,&mut source));
  for _ in 0..cut.as_u64().unwrap(){if source.as_ref().unwrap().ready(){break}turn(&mut caller,&mut decoder,grant(),|cx|source.as_mut().unwrap().advance(&mut ledger,cx));}
  caller.cancel().cancel_now();close(&mut source,&mut caller,&mut decoder);assert!(source.is_none());finish(caller,&mut decoder);
 }
}
