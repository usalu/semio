use semio_plugin_interpreter::*;
use std::sync::Arc;
use semio_framework_test_fixtures::fixture_text;
use semio_plugin_interpreter_numeric::execute_numeric_fixed;
use semio_framework_job::{OriginalJobFixtureCaller,OperationId,Generation};
use semio_framework_actor::{RetainedTurnInput,OriginalGuestExecutionBudget};
use semio_framework_value::RetainedCloneGrant;
fn clock()->Option<u64>{Some(1000)}
fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:2_000_000,maximum_capacity_bytes:2_000_000,maximum_release_bytes:2_000_000,maximum_depth:64}}
fn binary(opcode:u8,count:usize)->Vec<u8>{let mut bytes=b"\0asm\x01\0\0\0".to_vec();let mut section=|id:u8,payload:Vec<u8>|{bytes.push(id);bytes.push(payload.len()as u8);bytes.extend(payload)};let mut ty=vec![1,0x60,count as u8];ty.extend(std::iter::repeat_n(0x7f,count));ty.extend([1,0x7f]);section(1,ty);section(3,vec![1,0]);section(7,vec![1,3,b'r',b'u',b'n',0,0]);let mut body=vec![0];for index in 0..count{body.extend([0x20,index as u8])}body.extend([opcode,0x0b]);let mut code=vec![1,body.len()as u8];code.extend(body);section(10,code);bytes}
#[test]
fn original_numeric_primitives_preserve_measured_heap_and_match_independent_vm(){
 let fixture:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧮️numeric-execution/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){
  let args:Vec<Value>=row["args"].as_array().unwrap().iter().map(|value|Value::I32(value.as_i64().unwrap()as i32)).collect();let opcode=row["opcode"].as_u64().unwrap()as u8;
  let(fixed,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||execute_numeric_fixed(opcode,&args));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
  let bytes=binary(opcode,args.len());let engine=wasmtime::Engine::default();let module=wasmtime::Module::new(&engine,&bytes).unwrap();let mut store=wasmtime::Store::new(&engine,());let oracle=wasmtime::Instance::new(&mut store,&module,&[]).unwrap();let argument=|index:usize|match args[index]{Value::I32(value)=>value,_=>unreachable!()};let actual=if args.len()==1{oracle.get_typed_func::<i32,i32>(&mut store,"run").unwrap().call(&mut store,argument(0))}else{oracle.get_typed_func::<(i32,i32),i32>(&mut store,"run").unwrap().call(&mut store,(argument(0),argument(1)))};
  assert_eq!(actual.is_err(),row["trap"].as_bool().unwrap_or(false));if let Ok(value)=actual{assert_eq!(fixed,Ok(Value::I32(value)));assert_eq!(i64::from(value),row["expected"].as_i64().unwrap());}else{assert!(fixed.is_err())}
  let module=Arc::new(CoreModule::parse(&bytes).unwrap());let stock=grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(91),Generation(5),stock,clock,1000000,1000000);let mut epoch=1;let mut owner=caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;CoreInstanceOpening::begin(&module,cx)}).unwrap().unwrap();for _ in 0..100{if owner.resources_ready(){break}epoch+=1;caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.advance(&module,cx)}).unwrap();}epoch+=1;assert!(caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.begin_function_step(&module,0,&args,cx)}).unwrap());
  let mut refused=false;for _ in 0..100{epoch+=1;let(call,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;let input=RetainedTurnInput::borrow_original(cx)?;owner.advance_scalar(&module,OriginalGuestExecutionBudget{input,fuel:1,deadline_us:cx.deadline_us()},cx)}).unwrap());assert_eq!((caller.receipt().retained_capacity_bytes,caller.receipt().released_bytes),(heap.requested_bytes,heap.released_bytes));assert!(call.execution.spent_fuel.unwrap()<=1);match call.result{Ok(CoreOriginalInvocationStatus::Complete)=>break,Err(_)=>{refused=true;break},_=>{}}}
  assert_eq!(refused,row["trap"].as_bool().unwrap_or(false));if !refused{assert_eq!(owner.invocation_values(),Some(&[fixed.unwrap()][..]));}
  caller.cancel().cancel_now();for _ in 0..100{if owner.terminal_is_empty(){break}epoch+=1;caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;owner.close_step(&module,cx)}).unwrap();}assert!(owner.terminal_is_empty());caller.retire(stock);
 }
}
