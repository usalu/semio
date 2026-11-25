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
fn run_with_credit(bytes:&[u8],input:i32,expected:i32,name:&str,fuel:u64,continuation:Option<&str>){let module=Arc::new(CoreModule::parse(&bytes).unwrap());let stock=grant();let(mut caller,mut owner,mut epoch)=started(&module,input,93);let mut complete=false;
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
 for row in fixture["cases"].as_array().unwrap(){let body:Vec<u8>=row["body"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as u8).collect();let callee=row["callee"].as_array().map(|values|values.iter().map(|value|value.as_u64().unwrap()as u8).collect::<Vec<_>>());let bytes=binary(&body,callee.as_deref());let input=row["args"][0].as_i64().unwrap()as i32;let engine=wasmtime::Engine::default();let oracle=wasmtime::Module::new(&engine,&bytes).unwrap();let mut store=wasmtime::Store::new(&engine,());let instance=wasmtime::Instance::new(&mut store,&oracle,&[]).unwrap();let expected=instance.get_typed_func::<i32,i32>(&mut store,"run").unwrap().call(&mut store,input).unwrap();assert_eq!(i64::from(expected),row["expected"].as_i64().unwrap());
  if let Some(peak) = row["labelPeak"].as_u64() {
   paid_label_growth(&bytes, input, expected, peak as usize);
  } else if row["atomicTransfer"].is_object() {
   paid_atomic_transfer(&bytes, input, expected, &row["atomicTransfer"]);
  } else {
   run_with_credit(&bytes,input,expected,row["name"].as_str().unwrap(),row["executionFuel"].as_u64().unwrap_or(1),row["continuation"].as_str());
  }
 }
}

fn paid_label_growth(bytes: &[u8], input: i32, expected: i32, peak: usize) {
 let module = Arc::new(CoreModule::parse(bytes).unwrap());
 let stock = grant();
 let width = size_of::<ControlFrame>();
 let types = 2 * size_of::<ValueType>();
 let write = types + width + size_of::<Value>();
 for cancel_at_peak in [false, true] {
  let (mut caller, mut owner, mut epoch) = started(&module, input, 94);
  let mut live = caller.receipt().retained_capacity_bytes;
  let mut max_depth = 1;
  let mut relocated = 0;
  let mut constrained = false;
  let mut completed = false;
  for _ in 0..500 {
   let (depth, capacity) = label_extent(&owner);
   let mut turn_grant = stock;
   let tight = depth == 2 && capacity == 2 && !constrained && !label_machine(&owner).values.is_empty();
   if tight {
    let before = format!("{:?}", label_machine(&owner));
    assert_eq!(label_machine(&owner).values, [Value::I32(input)]);
    for denied in [
     RetainedCloneGrant { maximum_capacity_bytes: 3 * width + types - 1, ..stock },
     RetainedCloneGrant { maximum_release_bytes: 2 * width - 1, ..stock },
     RetainedCloneGrant { maximum_copy_bytes: 2 * width + write - 1, ..stock },
     RetainedCloneGrant { maximum_items: 0, ..stock },
    ] {
     assert_eq!(label_turn(&mut caller, &mut owner, &module, &mut epoch, denied),
      (CoreOriginalInvocationStatus::Pending, 0, RetainedCloneProgress::default()));
     assert_eq!(label_extent(&owner), (depth, capacity));
     assert_eq!(format!("{:?}", label_machine(&owner)), before);
     assert!(owner.control.is_none());
    }
    if cancel_at_peak {
     turn_grant.maximum_release_bytes = 3 * width;
    } else {
     turn_grant.maximum_capacity_bytes = 3 * width + types;
    }
    constrained = true;
   }
   let (status, spent, receipt) = label_turn(&mut caller, &mut owner, &module, &mut epoch, turn_grant);
   let (next_depth, next_capacity) = label_extent(&owner);
   live += receipt.retained_capacity_bytes;
   live -= receipt.released_bytes;
   max_depth = max_depth.max(next_depth);
   if next_depth > depth {
    let moved = if capacity == next_capacity { 0 } else { depth * width };
    let birth = if capacity == next_capacity { 0 } else { next_capacity * width };
    assert_eq!(spent, 1);
    assert_eq!((receipt.copied_bytes, receipt.retained_capacity_bytes, receipt.released_bytes),
     (moved + write, birth + types, moved));
    relocated += moved;
    if tight {
     assert_eq!(next_capacity, 3);
     assert!(label_machine(&owner).values.is_empty());
    }
   }
   if cancel_at_peak && max_depth == peak {
    let before = format!("{:?}", label_machine(&owner));
    caller.cancel().cancel_now();
    assert_eq!(label_turn(&mut caller, &mut owner, &module, &mut epoch, stock),
     (CoreOriginalInvocationStatus::Cancelled, 0, RetainedCloneProgress::default()));
    assert_eq!(label_extent(&owner), (next_depth, next_capacity));
    assert_eq!(format!("{:?}", label_machine(&owner)), before);
    break;
   }
   if status == CoreOriginalInvocationStatus::Complete {
    completed = true;
    assert_eq!(owner.invocation_values(), Some(&[Value::I32(expected)][..]));
    caller.cancel().cancel_now();
    break;
   }
  }
  assert!(constrained);
  assert_eq!(max_depth, peak);
  assert_eq!(completed, !cancel_at_peak);
  for _ in 0..500 {
   epoch += 1;
   let (closed, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| caller.turn(stock, |cx| {
    cx.bind_original_source_epoch(epoch)?;
    owner.close_invocation_step(&module, cx)
   }).unwrap());
   assert_eq!((heap.requested_bytes, heap.released_bytes), (0, caller.receipt().released_bytes));
   live -= caller.receipt().released_bytes;
   if closed { break; }
  }
  assert_eq!(live, 0);
  assert!(owner.core.as_ref().unwrap().machine.is_none());
  for _ in 0..100 {
   if owner.terminal_is_empty() { break; }
   epoch += 1;
   caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; owner.close_step(&module, cx) }).unwrap();
  }
  caller.retire(stock);
 }
}

fn started(module: &Arc<CoreModule>, input: i32, operation: u64) -> (OriginalJobFixtureCaller, CoreInstanceOpening, u64) {
 let stock = grant();
 let mut caller = OriginalJobFixtureCaller::admit(OperationId(operation), Generation(6), stock, clock, 1_000_000, 1_000_000);
 let mut epoch = 1;
 let mut owner = caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; CoreInstanceOpening::begin(module, cx) }).unwrap().unwrap();
 while !owner.resources_ready() {
  epoch += 1;
  caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; owner.advance(module, cx) }).unwrap();
 }
 epoch += 1;
 assert!(caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; owner.begin_function_step(module, 0, &[Value::I32(input)], cx) }).unwrap());
 (caller, owner, epoch)
}

fn label_machine(owner: &CoreInstanceOpening) -> &Machine {
 owner.core.as_ref().unwrap().machine.as_ref().unwrap()
}

fn label_extent(owner: &CoreInstanceOpening) -> (usize, usize) {
 let controls = &label_machine(owner).frames.last().unwrap().controls;
 (controls.len(), controls.capacity())
}

fn label_turn(caller: &mut OriginalJobFixtureCaller, owner: &mut CoreInstanceOpening, module: &Arc<CoreModule>,
 epoch: &mut u64, grant: RetainedCloneGrant) -> (CoreOriginalInvocationStatus, u64, RetainedCloneProgress) {
 *epoch += 1;
 let (call, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| caller.turn(grant, |cx| {
  cx.bind_original_source_epoch(*epoch)?;
  let input = RetainedTurnInput { grant: cx.retained_grant(), ..RetainedTurnInput::borrow_original(cx)? };
  owner.advance_scalar(module, OriginalGuestExecutionBudget { input, fuel: 1, deadline_us: cx.deadline_us() }, cx)
 }).unwrap());
 let receipt = caller.receipt();
 assert!(receipt.fits(grant));
 assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
 (call.result.unwrap(), call.execution.spent_fuel.unwrap(), receipt)
}

/// ⚙️ A branch that moves nothing posts one work unit and no retained item, copy byte, capacity or release, so a closed retained ledger still executes it atomically.
fn paid_atomic_transfer(bytes: &[u8], input: i32, expected: i32, fixture: &serde_json::Value) {
 let module = Arc::new(CoreModule::parse(bytes).unwrap());
 let before = fixture["before"].as_u64().unwrap() as usize;
 let after = fixture["after"].as_u64().unwrap() as usize;
 let retire_label = fixture["retireLabel"].as_bool().unwrap();
 let prefix_fuel = fixture["prefixFuel"].as_u64().unwrap();
 let expected_fuel = fixture["expectedFuel"].as_u64().unwrap();
 let stock = grant();
 let closed = RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: 0, ..stock };
 for cancelled in [false, true] {
  let (mut caller, mut owner, mut epoch) = started(&module, input, 95);
  for _ in 0..prefix_fuel {
   assert_eq!(label_turn(&mut caller, &mut owner, &module, &mut epoch, stock).1, 1);
  }
  assert!(owner.control.is_none());
  let machine = label_machine(&owner);
  let frame = machine.frames.last().unwrap();
  assert_eq!(frame.pc, before);
  let depth = frame.controls.len();
  let values = machine.values.clone();
  let height = frame.controls.last().unwrap().stack_height;
  assert!(height <= values.len());
  if cancelled {
   let machine = format!("{:?}", label_machine(&owner));
   caller.cancel().cancel_now();
   assert_eq!(label_turn(&mut caller, &mut owner, &module, &mut epoch, closed),
    (CoreOriginalInvocationStatus::Cancelled, 0, RetainedCloneProgress::default()));
   assert_eq!(format!("{:?}", label_machine(&owner)), machine);
  } else {
   epoch += 1;
   let ((call, work), heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| caller.turn(closed, |cx| {
    cx.bind_original_source_epoch(epoch)?;
    let input = RetainedTurnInput { grant: cx.retained_grant(), ..RetainedTurnInput::borrow_original(cx)? };
    let prior_work = cx.original_work_units_spent();
    let call = owner.advance_scalar(&module, OriginalGuestExecutionBudget { input, fuel: 1, deadline_us: cx.deadline_us() }, cx)?;
    Ok((call, cx.original_work_units_spent() - prior_work))
   }).unwrap());
   assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
   assert_eq!(caller.receipt(), RetainedCloneProgress::default());
   assert_eq!(work, 1);
   assert_eq!(call.execution.spent_fuel, Some(1));
   assert_eq!(call.result.unwrap(), CoreOriginalInvocationStatus::Pending);
   assert!(owner.control.is_none());
   let machine = label_machine(&owner);
   let frame = machine.frames.last().unwrap();
   assert_eq!(frame.pc, after);
   assert_eq!(frame.controls.len(), depth - usize::from(retire_label));
   assert_eq!(machine.values, values[..height]);
   let mut spent = prefix_fuel + 1;
   for _ in 0..100 {
    let (status, fuel, _) = label_turn(&mut caller, &mut owner, &module, &mut epoch, stock);
    spent += fuel;
    if status == CoreOriginalInvocationStatus::Complete { break }
   }
   assert_eq!(spent, expected_fuel);
   assert_eq!(owner.invocation_values(), Some(&[Value::I32(expected)][..]));
   caller.cancel().cancel_now();
  }
  for _ in 0..100 {
   if owner.terminal_is_empty() { break }
   epoch += 1;
   caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; owner.close_step(&module, cx) }).unwrap();
  }
  assert!(owner.terminal_is_empty());
  caller.retire(stock);
 }
}
