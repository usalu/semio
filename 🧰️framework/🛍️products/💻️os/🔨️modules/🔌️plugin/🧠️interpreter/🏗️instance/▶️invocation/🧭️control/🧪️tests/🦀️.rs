use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
use semio_framework_actor::{RetainedTurnInput,OriginalGuestExecutionBudget};
use semio_framework_value::RetainedCloneGrant;
fn clock()->Option<u64>{Some(1000)}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:2_000_000,maximum_capacity_bytes:2_000_000,maximum_release_bytes:2_000_000,maximum_depth:64}}
fn binary(body:&[u8],callee:Option<&[u8]>)->Vec<u8>{let mut bytes=b"\0asm\x01\0\0\0".to_vec();let mut section=|id:u8,data:Vec<u8>|{bytes.extend([id,data.len()as u8]);bytes.extend(data)};section(1,vec![1,0x60,1,0x7f,1,0x7f]);section(3,if callee.is_some(){vec![2,0,0]}else{vec![1,0]});if callee.is_some(){section(4,vec![1,0x70,0,1]);}section(7,vec![1,3,b'r',b'u',b'n',0,0]);if callee.is_some(){section(9,vec![1,0,0x41,0,0x0b,1,1]);}let mut code=vec![if callee.is_some(){2}else{1},body.len()as u8];code.extend_from_slice(body);if let Some(callee)=callee{code.push(callee.len()as u8);code.extend_from_slice(callee);}section(10,code);bytes}

pub(crate) fn run_original_program(bytes:&[u8],input:i32,expected:i32,name:&str){
 run_with_credit(bytes,input,expected,name,1,None);
}
fn run_with_credit(bytes:&[u8],input:i32,expected:i32,name:&str,fuel:u64,continuation:Option<&str>){let module=Arc::new(CoreModule::parse(&bytes).unwrap());let stock=grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(93),Generation(6),stock,clock,1000000,1000000);let mut epoch=1;let mut owner=caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;CoreInstanceOpening::begin(&module,cx)}).unwrap().unwrap();for _ in 0..100{if owner.resources_ready(){break}epoch+=1;caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.advance(&module,cx)}).unwrap();}epoch+=1;assert!(caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.begin_function_step(&module,0,&[Value::I32(input)],cx)}).unwrap());let mut complete=false;
 let mut continued=false;
 let mut tested_blocked=false;
 for _ in 0..2000{
  epoch+=1;
  let crossing=match continuation{
   Some("settledControl")=>owner.control.is_some(),
   Some("retiredCallee")=>owner.returning_frame,
   _=>false
  };
  let blocked=crossing&&!tested_blocked;
  let turn_grant=RetainedCloneGrant{
   maximum_copy_bytes:if blocked{0}else{512},
   maximum_items:if continuation.is_some()&&!tested_blocked{1}else{stock.maximum_items},
   ..stock
  };
  let (call,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(turn_grant,|cx|{
   cx.bind_original_source_epoch(epoch)?;
   let input=RetainedTurnInput{grant:cx.retained_grant(),..RetainedTurnInput::borrow_original(cx)?};
   owner.advance_scalar(&module,OriginalGuestExecutionBudget{input,fuel,deadline_us:cx.deadline_us()},cx)
  }).unwrap());
  let spent=call.execution.spent_fuel.unwrap();
  assert!(spent<=fuel && caller.receipt().fits(turn_grant));
  assert_eq!((caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes),(heap.requested_bytes,heap.released_bytes));
  let status=call.result.unwrap();
  if blocked{
   assert_eq!(status,CoreOriginalInvocationStatus::Pending);
   assert_eq!(spent,0);
   assert_eq!(caller.receipt(),RetainedCloneProgress::default());
   tested_blocked=true;
  }
  continued|=crossing&&spent>0;
  if status==CoreOriginalInvocationStatus::Complete{complete=true;break}
 }
 assert!(continuation.is_none()||(tested_blocked&&continued),"{name}: a continuation must yield without progress and resume within its next issued grant");
  assert!(complete,"{}",name);assert_eq!(owner.invocation_values(),Some(&[Value::I32(expected)][..]));caller.cancel().cancel_now();for _ in 0..100{if owner.terminal_is_empty(){break}epoch+=1;caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.close_step(&module,cx)}).unwrap();}assert!(owner.terminal_is_empty());caller.retire(stock);}
/// 🧭️ Branch planning and label release preserve exact heap receipts while the independent VM produces the same result.
#[test]
fn original_structured_control_matches_independent_vm_and_paid_label_retirement(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){let body:Vec<u8>=row["body"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect();let callee=row["callee"].as_array().map(|values|values.iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>());let bytes=binary(&body,callee.as_deref());let input=row["args"][0].as_i64().unwrap()as i32;let engine=wasmtime::Engine::default();let oracle=wasmtime::Module::new(&engine,&bytes).unwrap();let mut store=wasmtime::Store::new(&engine,());let instance=wasmtime::Instance::new(&mut store,&oracle,&[]).unwrap();let expected=instance.get_typed_func::<i32,i32>(&mut store,"run").unwrap().call(&mut store,input).unwrap();assert_eq!(i64::from(expected),row["expected"].as_i64().unwrap());run_with_credit(&bytes,input,expected,row["name"].as_str().unwrap(),row["executionFuel"].as_u64().unwrap_or(1),row["continuation"].as_str());
 }
}
