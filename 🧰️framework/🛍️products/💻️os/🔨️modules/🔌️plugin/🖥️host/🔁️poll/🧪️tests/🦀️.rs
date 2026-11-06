//! 🧪️ The actual Source and WebAssembly Poll retain output until both guest handbacks complete.
use super::*;
use semio_framework_actor::RetainedTurnInput;
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
use semio_framework_value::RetainedCloneGrant;
use semio_plugin_host_guest_runtime::{GuestRuntime, PackageRef};
use semio_plugin_interpreter_runtime::{channel_fixture, OwnedRuntime};

fn clock() -> Option<u64> { Some(1000) }
fn grant() -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 100_000, maximum_copy_bytes: 4_000_000, maximum_capacity_bytes: 16_000_000, maximum_release_bytes: 16_000_000, maximum_depth: 64 } }
fn core_bytes(component: &[u8]) -> &[u8] { let mut at = 9; let mut len = 0; let mut shift = 0; loop { let byte = component[at]; at += 1; len |= usize::from(byte & 127) << shift; shift += 7; if byte & 128 == 0 { break; } } &component[at..at + len] }

#[test]
fn every_poll_crossing_requires_the_actual_enclosing_and_purpose_reserves(){let stock=grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(97),Generation(1),stock,clock,100_000_000,1_000_000);caller.turn(stock,|cx|{cx.bind_original_source_epoch(1)?;let input=RetainedTurnInput::borrow_original(cx)?;let budget=OriginalJobBudget::borrow_original(input,cx)?;for header in [144,152,184]{let demand=OriginalEscrowPublisher::core_wire_reserve(header)?;assert!(require_core_wire_reserve(budget,header).is_ok());let mut denied=budget;denied.input.grant.maximum_items=demand.items-1;assert!(require_core_wire_reserve(denied,header).is_err());let mut denied=budget;denied.retained_policy.maximum_depth=0;assert!(require_core_wire_reserve(denied,header).is_err());let mut denied=budget;denied.input.grant.maximum_copy_bytes=demand.copy_bytes-1;assert!(require_core_wire_reserve(denied,header).is_err());let mut denied=budget;denied.retained_policy.maximum_copy_bytes=demand.copy_bytes-1;assert!(require_core_wire_reserve(denied,header).is_err());let mut denied=budget;denied.input.work.fuel=demand.work-1;assert!(require_core_wire_reserve(denied,header).is_err());let mut denied=budget;denied.fuel=demand.work-1;assert!(require_core_wire_reserve(denied,header).is_err());}Ok(())}).unwrap();caller.retire(stock);}

#[test]
fn actual_poll_source_captures_and_retires_the_wasmtime_result() { exercise_poll(false); }

#[test]
fn actual_poll_callback_failure_is_owned_and_never_delivered_twice() { exercise_poll(true); }

fn exercise_poll(callback_fails: bool) {
    let bytes = channel_fixture::owned_poll_guest(semio_os_kernel::CHANNEL_VERSION);
    let engine = wasmtime::Engine::default();
    let module = wasmtime::Module::new(&engine, core_bytes(&bytes)).unwrap();
    let mut store = wasmtime::Store::new(&engine, ());
    let oracle = wasmtime::Instance::new(&mut store, &module, &[]).unwrap();
    let poll = oracle.get_func(&mut store, OwnedSemioExport::Poll.core_name()).unwrap();
    let mut result = [wasmtime::Val::I64(0)];
    poll.call(&mut store, &vec![wasmtime::Val::I64(0); 19], &mut result).unwrap();
    let wasmtime::Val::I64(word) = &result[0] else { panic!("Poll returns its scalar word") };
    let word = *word;
    let memory = oracle.get_memory(&mut store, "memory").unwrap().data(&store);
    let wire = &memory[word as u32 as usize..word as u32 as usize + (word as u64 >> 32) as usize];
    let (header,escrow)=OriginalActorFixedReceipt::read(wire).unwrap();assert_eq!((escrow.pointer,escrow.length),(3200,48));
    let OriginalActorFixedKind::TurnOutput{ready:true,captured:true,pointer,length,..}=header.kind else{panic!("Poll returns the actual ready capsule extent")};
    assert!(serde_json::from_slice::<bool>(&memory[pointer as usize..pointer as usize+length as usize]).unwrap());

    let runtime = GuestRuntimes::Owned(OwnedRuntime::new(clock));
    let package = PackageRef { package: semio_framework_actor::PackageId("poll-fixture".into()), hash: semio_framework_actor::PackageHash([8; 32]) };
    let compiled = semio_framework_async::block_on(runtime.compile(&package, &bytes)).unwrap();
    let actor = semio_framework_async::block_on(semio_framework_actor::ActorId::new(8, 1, 3, 0));
    let stock = grant();
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(98), Generation(1), stock, clock, 100_000_000, 1_000_000);
    let mut epoch = 1;
    let mut opening = caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; runtime.begin_original_actor_step(&compiled, actor, cx) }).unwrap();
    for _ in 0..4096 {
        if opening.as_ref().unwrap().ready() { break; }
        epoch += 1;
        caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; let owner = opening.as_mut().unwrap(); let input = RetainedTurnInput::borrow_original(cx)?; let execution = owner.requires_execution().then(|| OriginalGuestExecutionBudget { input, fuel: 1000, deadline_us: cx.deadline_us() }); runtime.advance_original_actor_step(owner, &compiled, execution, cx)?.result?; Ok(()) }).unwrap();
    }
    assert!(opening.as_ref().unwrap().ready());
    let mut instance = None;
    for _ in 0..4096 {
        if instance.is_some() { break; }
        epoch += 1;
        instance = caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; let input = RetainedTurnInput::borrow_original(cx)?; runtime.take_original_actor_step(&mut opening, &compiled, input, cx) }).unwrap();
    }
    let mut instance = instance.unwrap();
    let input = DslValue::Bool(true);
    let mut poll = GuestPoll::<bool>::new(OriginalNativeDecodePolicy { maximum_owned_bytes: stock.maximum_capacity_bytes }, JsonReadLimits { maximum_bytes: 1024, maximum_allocation_bytes: stock.maximum_capacity_bytes, maximum_depth: 16, maximum_items: 1024 });
    let mut observed = false;
    let mut callback_count = 0;
    let mut failure_observed = false;
    let mut failure: Option<ControlledRetirement<OriginalTurnFailure>> = None;
    for _ in 0..100_000 {
        if poll.terminal_is_empty() && failure.is_none() { break; }
        epoch += 1;
        caller.turn(stock, |cx| {
            cx.bind_original_source_epoch(epoch)?;
            if let Some(owner) = failure.as_mut() { if owner.terminal_is_empty() { failure.take(); } else { let result = owner.step(cx.retained_grant()); let work = owner.last_turn_work(); semio_framework_job::receive_original_retained_work(cx, RetainedWorkOutcome { result, work })?; } return Ok(()); }
            if poll.transport_failure().is_some() { poll.take_transport_failure_step(cx, |original, _| { failure_observed = true; failure = Some(ControlledRetirement::retain(original.take().unwrap())); Ok(()) })?; return Ok(()); }
            if poll.input_progress_pending() { poll.acknowledge_input_progress_step(cx)?; return Ok(()); }
            if poll.scalar_progress().is_some() { poll.acknowledge_scalar_progress_step(cx)?; return Ok(()); }
            if poll.scalar_result().is_some() { poll.acknowledge_scalar_result_step(cx)?; return Ok(()); }
            if poll.fixed_receipt().is_some() { poll.acknowledge_fixed_step(cx)?; return Ok(()); }
            if poll.raw_word().is_some() { poll.take_raw_word_step(cx)?; return Ok(()); }
            if poll.decoding_outcome().is_some() || poll.settlement_outcome().is_some() { poll.retire_outcome_step(cx)?; return Ok(()); }
            if !poll.prepare_step(&runtime, &mut instance, cx)? { return Ok(()); }
            let needs_execution = poll.requires_execution(&runtime, &instance)?;
            let original = RetainedTurnInput::borrow_original(cx)?;
            let budget = OriginalJobBudget::borrow_original(original, cx)?;
            let mut execution_input = original;
            execution_input.grant = cx.retained_grant();
            let execution = needs_execution.then(|| OriginalGuestExecutionBudget { input: execution_input, fuel: 1000, deadline_us: cx.deadline_us() });
            match poll.advance_step(&runtime, &mut instance, &input, Some(budget), execution, cx)? {
                GuestPollStatus::Output => { let delivered = poll.deliver_step(cx, |value, _| { assert!(*value); observed = true; callback_count += 1; if callback_fails { Err(ValueError::literal(ValueRefusalKind::WorkLimit, "fixture callback refused").into()) } else { Ok(()) } })?; if delivered && callback_fails { assert!(poll.deliver_step(cx, |_, _| panic!("failed callback cannot be retried")).is_err()); } }
                GuestPollStatus::AwaitBudget => { poll.repeat_input_step(&runtime, &mut instance, budget, cx)?; }
                GuestPollStatus::Refused => panic!("actual Poll unexpectedly refused"),
                _ => {}
            }
            Ok(())
        }).unwrap();
    }
    assert!(observed && poll.terminal_is_empty());
    assert_eq!(callback_count, 1);
    assert_eq!(failure_observed, callback_fails);
    drop(poll);
    let mut closed = false;
    for _ in 0..4096 {
        epoch += 1;
        if caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; runtime.close_original_instance_step(&mut instance, &compiled, cx) }).unwrap() { closed = true; break; }
    }
    assert!(closed);
    caller.retire(stock);
}

use semio_plugin_interpreter_runtime::OwnedGuestInstance;
use semio_plugin_host_guest_runtime::OriginalCoreValue as Value;
fn opened(bytes:&[u8])->(OwnedRuntime,semio_plugin_interpreter_runtime::OwnedCompiledGuest,OriginalJobFixtureCaller,OwnedGuestInstance,u64){let runtime=OwnedRuntime::new(clock);let compiled=runtime.compile_component(bytes).unwrap();let stock=grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(89),Generation(4),stock,clock,1000000,1000000);let mut epoch=1;let mut opening=caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;runtime.begin_actor_startup(&compiled,cx)}).unwrap();for _ in 0..4096{if opening.as_ref().unwrap().ready(){break}epoch+=1;caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;let owner=opening.as_mut().unwrap();let input=RetainedTurnInput::borrow_original(cx)?;let execution=owner.requires_execution().then(||OriginalGuestExecutionBudget{input,fuel:1,deadline_us:cx.deadline_us()});owner.advance_step(&compiled,cx,execution)?.result?;Ok(())}).unwrap();}let mut state=None;for _ in 0..8{epoch+=1;state=caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;runtime.take_actor_startup_step(&mut opening,&compiled,RetainedTurnInput::borrow_original(cx)?,cx)}).unwrap();if state.is_some(){break}}(runtime,compiled,caller,state.unwrap(),epoch)}
fn close(runtime:&OwnedRuntime,compiled:&semio_plugin_interpreter_runtime::OwnedCompiledGuest,state:OwnedGuestInstance,caller:&mut OriginalJobFixtureCaller,epoch:&mut u64){let mut state=Some(state);for _ in 0..4096{if state.is_none(){return}*epoch+=1;caller.turn(grant(),|cx|{cx.bind_original_source_epoch(*epoch)?;runtime.close_original_actor_step(&mut state,compiled,cx)}).unwrap();}assert!(state.is_none())}

#[test]
fn cancelled_source_executes_only_its_authenticated_guest_cleanup(){
 let bytes=channel_fixture::owned_poll_clock_guest(semio_os_kernel::CHANNEL_VERSION);let(runtime,compiled,mut caller,mut state,mut epoch)=opened(&bytes);let stock=grant();epoch+=1;assert!(caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;runtime.begin_original_call_step(&mut state,OwnedSemioExport::Poll,&[Value::I64(0);19],cx)}).unwrap());caller.cancel().cancel_now();epoch+=1;let resumed=caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;cx.with_original_retirement(|cx|runtime.advance_original_call_step(&mut state,None,cx).map(|_|()))});assert!(resumed.is_err());for _ in 0..4096{epoch+=1;if caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;cx.with_original_retirement(|cx|runtime.close_original_call_step(&mut state,cx))}).unwrap(){break}}assert!(!state.original_call_is_pending());
 epoch+=1;let denied=caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;cx.with_original_retirement(|cx|runtime.begin_original_call_step(&mut state,OwnedSemioExport::Poll,&[Value::I64(0);19],cx))});assert!(denied.is_err());assert!(!state.original_call_is_pending());
 fn call(runtime:&OwnedRuntime,state:&mut OwnedGuestInstance,caller:&mut OriginalJobFixtureCaller,epoch:&mut u64,export:OwnedSemioExport,args:&[Value])->semio_plugin_interpreter_runtime::OwnedOriginalCallResult{
  for _ in 0..4096{*epoch+=1;if caller.turn(grant(),|cx|{cx.bind_original_source_epoch(*epoch)?;cx.with_original_retirement(|cx|runtime.begin_original_call_step(state,export,args,cx))}).unwrap(){break}}
  assert!(state.original_call_is_pending());
  for _ in 0..100000{*epoch+=1;let result=caller.turn(grant(),|cx|{cx.bind_original_source_epoch(*epoch)?;cx.with_original_retirement(|cx|{let needs=state.original_call_requires_execution();if needs&&!runtime.prepare_original_call_retirement_step(state,cx)?{return Ok(None)}let mut input=RetainedTurnInput::borrow_original(cx)?;input.grant=cx.retained_grant();let execution=needs.then(||OriginalGuestExecutionBudget{input,fuel:1000,deadline_us:cx.deadline_us()});let progress=runtime.advance_original_call_step(state,execution,cx)?;if progress.result?==OwnedOriginalCallStatus::Ready{runtime.take_original_call_result_step(state,cx)}else{Ok(None)}})}).unwrap();if let Some(result)=result{return result}}
  panic!("funded original cleanup did not finish")
 }
 let released=call(&runtime,&mut state,&mut caller,&mut epoch,OwnedSemioExport::Deallocate,&[Value::I32(2800),Value::I32(152)]);assert!(released.value.is_none());
 epoch+=1;let budget=caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;OriginalJobBudget::borrow_original(RetainedTurnInput::borrow_original(cx)?,cx).map_err(Into::into)}).unwrap();let args=budget.scalar_arguments().map(|value|Value::I64(value as i64));let closed=call(&runtime,&mut state,&mut caller,&mut epoch,OwnedSemioExport::CloseActorOutput,&args);let clock_reply=state.original_memory(0).unwrap();assert_eq!(&clock_reply[3008..3012],&1u32.to_le_bytes());assert_eq!(&clock_reply[3016..3024],&1000u64.to_le_bytes());let Some(Value::I64(word))=closed.value else{panic!("actual close returns its fixed receipt")};let pointer=word as u32;let length=(word as u64>>32)as usize;epoch+=1;caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;runtime.with_original_memory_bytes(&state,pointer,length,cx,|bytes,_|{let(fixed,escrow)=OriginalActorFixedReceipt::read(bytes)?;assert_eq!((escrow.pointer,escrow.length),(3200,48));assert_eq!(fixed.input,budget.input);assert!(matches!(fixed.kind,OriginalActorFixedKind::Close{complete:true,blocked:false,refusal:None}));Ok(())})}).unwrap();
 let released=call(&runtime,&mut state,&mut caller,&mut epoch,OwnedSemioExport::Deallocate,&[Value::I32(pointer as i32),Value::I32(length as i32)]);assert!(released.value.is_none());close(&runtime,&compiled,state,&mut caller,&mut epoch);caller.retire(stock);
}
