//! 🧪️ The actual Source and WebAssembly Poll retain output until both guest handbacks complete.
fn receive_scalar(runtime:&semio_plugin_interpreter_runtime::OwnedRuntime,state:&mut semio_plugin_interpreter_runtime::OwnedGuestInstance,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<semio_plugin_interpreter_runtime::OwnedOriginalCallResult>,semio_framework_job::OriginalTurnFailure>{let mut recipient=None;runtime.take_original_call_result_into_step(state,&mut recipient,cx)?;Ok(recipient)}

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
fn every_poll_crossing_requires_the_actual_enclosing_and_purpose_reserves(){let stock=grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(97),Generation(1),stock,clock,100_000_000,1_000_000);caller.turn(stock,|cx|{cx.bind_original_source_epoch(1)?;let input=RetainedTurnInput::borrow_original(cx)?;let budget=OriginalJobBudget::borrow_original(input,cx)?;for header in[144,152,184,200]{let(items,demand,work)=semio_plugin_guest_contract::original_actor_context::original_core_first_response_reserve(header)?;assert!(require_core_wire_reserve(budget,header).is_ok());let mut denied=budget;denied.input.grant.maximum_items=items-1;assert!(require_core_wire_reserve(denied,header).is_err());let mut denied=budget;denied.retained_policy.maximum_depth=0;assert!(require_core_wire_reserve(denied,header).is_err());let mut denied=budget;denied.input.grant.maximum_copy_bytes=demand.copy_bytes-1;assert!(require_core_wire_reserve(denied,header).is_err());let mut denied=budget;denied.retained_policy.maximum_copy_bytes=demand.copy_bytes-1;assert!(require_core_wire_reserve(denied,header).is_err());let mut denied=budget;denied.input.work.fuel=work-1;assert!(require_core_wire_reserve(denied,header).is_err());let mut denied=budget;denied.fuel=work-1;assert!(require_core_wire_reserve(denied,header).is_err());}Ok(())}).unwrap();caller.retire(stock);}

#[test]
fn actual_poll_source_captures_and_retires_the_wasmtime_result() { exercise_poll_on_stack(PollExercise::Deliver); }

#[test]
fn bootstrap_and_source_ready_reserves_are_alternatives_on_one_actual_loan(){
 let stock=grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(107),Generation(1),stock,clock,100_000_000,1_000_000);
 caller.turn(stock,|cx|{
  cx.bind_original_source_epoch(1)?;let input=RetainedTurnInput::borrow_original(cx)?;let mut budget=OriginalJobBudget::borrow_original(input,cx)?;
  let(a,x,m)=semio_plugin_guest_contract::original_actor_context::original_core_first_response_reserve(200)?;
  let(b,y,n)=semio_plugin_guest_contract::original_actor_context::OriginalActorBootstrapReceipt::first_bootstrap_response_reserve();
  let exact=RetainedCloneGrant{maximum_items:a.max(b),maximum_copy_bytes:x.copy_bytes.max(y.copy_bytes),maximum_capacity_bytes:x.capacity_bytes.max(y.capacity_bytes),maximum_release_bytes:x.release_bytes.max(y.release_bytes),maximum_depth:x.depth.max(y.depth)};
  assert!(exact.maximum_items<=128);assert!(a+b>128);
  budget.input.grant=exact;budget.retained_policy=exact;budget.input.work.fuel=m.max(n);budget.fuel=m.max(n);
  assert!(require_core_wire_reserve(budget,200).is_ok());
  let mut denied=budget;denied.input.grant.maximum_items-=1;assert!(require_core_wire_reserve(denied,200).is_err());
  let mut denied=budget;denied.retained_policy.maximum_copy_bytes-=1;assert!(require_core_wire_reserve(denied,200).is_err());
  let mut denied=budget;denied.input.work.fuel-=1;assert!(require_core_wire_reserve(denied,200).is_err());
  Ok(())
 }).unwrap();caller.retire(stock);
}

#[test]
fn actual_poll_callback_failure_keeps_input_until_real_receivers_accept() { exercise_poll_on_stack(PollExercise::RefuseCallback); }

#[test]
fn actual_poll_retires_interrupted_input_and_hands_back_returned_input() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️input-retirement.json")).unwrap();
    for row in law["cases"].as_array().unwrap() {
        let completed = match row["input"].as_str().unwrap() {
            "suspended" => false,
            "returned" => true,
            _ => panic!("unknown input retirement frontier"),
        };
        exercise_poll_on_stack(PollExercise::CloseInput(completed, row["closeGuestContext"].as_bool().unwrap()));
    }
}

#[test]
fn actual_poll_refuses_an_unfundable_ordinary_acknowledgment() {
    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️acknowledgment-stock.json")).unwrap();
    for frontier in law["frontiers"].as_array().unwrap() {
        let pending_progress = match frontier.as_str().unwrap() {
            "beforeInput" => false,
            "inputProgressReceipt" => true,
            _ => panic!("unknown acknowledgment frontier"),
        };
        exercise_poll_on_stack(PollExercise::ExhaustAcknowledgment(pending_progress));
    }
}

#[derive(Clone, Copy)]
enum PollExercise { Deliver, RefuseCallback, CloseInput(bool, bool), ExhaustAcknowledgment(bool) }

fn exercise_poll_on_stack(exercise: PollExercise) {
    std::thread::Builder::new().stack_size(8 * 1024 * 1024)
        .spawn(move || exercise_poll(exercise)).unwrap()
        .join().unwrap_or_else(|panic| std::panic::resume_unwind(panic));
}

fn exercise_poll(exercise: PollExercise) {
    let callback_fails = matches!(exercise, PollExercise::RefuseCallback);
    let close_input = match exercise { PollExercise::CloseInput(completed, close) => Some((completed, close)), _ => None };
    let exhaust_acknowledgment = match exercise { PollExercise::ExhaustAcknowledgment(progress) => Some(progress), _ => None };
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

    use semio_plugin_host_post_turn_relay::origin_registry;
    let runtime = std::sync::Arc::new(GuestRuntimes::Owned(OwnedRuntime::new(clock)));
    let package = PackageRef { package: semio_framework_actor::PackageId("poll-fixture".into()), hash: semio_framework_actor::PackageHash([8; 32]) };
    let compiled = semio_framework_async::block_on(runtime.compile(&package, &bytes)).unwrap();
    let actor = semio_framework_async::block_on(semio_framework_actor::ActorId::new(8, 1, 3, 0));
    let stock = grant();
    let mut cache_source = OriginalJobFixtureCaller::admit(OperationId(98), Generation(1), stock, clock, 100_000_000, 1_000_000);
    let identity = origin_registry::next_identity();
    let mut origin = None;
    for _ in 0..4096 {
        if cache_source.turn(stock, |cx| origin_registry::register_step(identity, &runtime, &compiled, &mut origin, cx)).unwrap() {
            break;
        }
    }
    let origin = origin.expect("fixture registry admits the original compiled parent");
    let mut session = GuestPollSession::<bool, GuestPollNoReceiverState>::new(
        fixture_operation_driver(stock, if close_input.is_some() { 8 } else { 10_000 }),
        actor,
        origin,
        OriginalNativeDecodePolicy { maximum_owned_bytes: stock.maximum_capacity_bytes },
        JsonReadLimits { maximum_bytes: 1024, maximum_allocation_bytes: stock.maximum_capacity_bytes, maximum_depth: 16, maximum_items: 1024 },
        GuestPollNoReceiverState::new(),
    );
    let mut opened = false;
    for _ in 0..4096 {
        match session.open_step() {
            GuestPollSessionStatus::Ready => { opened = true; break; }
            GuestPollSessionStatus::Pending => {}
            status => panic!("Poll fixture opening {status:?}: {:?}", session.failure()),
        }
    }
    assert!(opened);
    let events = DslValue::Bool(true);
    let input = std::pin::pin!(session.input(&events).unwrap());
    let mut callback_count = 0;
    let mut failure_observed = false;
    let mut delivered = false;
    let mut complete = false;
    let mut origin_lookup_delayed = false;
    let mut input_calls = 0;
    let mut before_input_execution = 0;
    let mut was_executing = false;
    let mut interrupted_input = false;
    let mut refused_acknowledgment = false;
    for _ in 0..100_000 {
        if let Some(pending_progress) = exhaust_acknowledgment {
            if let Some((native_step, issued, request)) = session.fixture_guest_request().filter(|(_, _, request)| request.header_bytes == 144) {
                let armed = if pending_progress {
                    let mut observed = None;
                    let status = session.receive_state_step(|poll, _| {
                        observed = Some(poll.input_progress_pending() && poll.execution_receipt().is_some_and(|receipt| receipt.submitted));
                        Ok(())
                    });
                    assert_ne!(status, GuestPollSessionStatus::Refused, "{:?}", session.cleanup_failure());
                    let Some(observed) = observed else { continue };
                    observed
                } else {
                    native_step
                };
                if armed {
                    let law: serde_json::Value = serde_json::from_str(include_str!("🧫️acknowledgment-stock.json")).unwrap();
                    let target = control_bytes() - law["copyBytesBelowMinimum"].as_u64().unwrap() as usize;
                    let owner = session.fixture_guest_owner();
                    if !pending_progress {
                        let status = while_origin_is_borrowed(origin, || session.drive_step(input.as_ref(), &mut |_, _, _| {
                            panic!("Input cannot deliver Poll output");
                        }));
                        assert_eq!(status, GuestPollSessionStatus::Pending);
                    }
                    spend_to_acknowledgment_limit(&mut session, stock.maximum_copy_bytes, target);
                    for _ in 0..4096 {
                        let status = session.drive_step(input.as_ref(), &mut |_, _, _| panic!("unfundable acknowledgment cannot deliver output"));
                        if status == GuestPollSessionStatus::Pending { continue; }
                        assert_eq!(format!("{status:?}"), law["ordinaryStatus"].as_str().unwrap());
                        let (_, retained, retained_request) = session.fixture_guest_request().unwrap();
                        assert_eq!(serde_json::json!({
                            "loanUnchanged": retained == issued,
                            "epochUnchanged": retained.input.epoch == issued.input.epoch,
                            "requestUnchanged": retained_request == request,
                            "ownerUnchanged": session.fixture_guest_owner() == owner,
                        }), law["pendingLoan"]);
                        let original = session.failure().unwrap().primary.as_ref().unwrap().primary_value().unwrap();
                        assert_eq!(original.kind, ValueRefusalKind::WorkLimit);
                        assert_eq!(Some(original.message.as_ref()), law["refusal"].as_str());
                        refused_acknowledgment = true;
                        break;
                    }
                    break;
                }
            }
        }
        if let Some((wait_for_result, _)) = close_input {
            if session.fixture_guest_request().is_some_and(|(_, _, request)| request.header_bytes == 144) {
                let mut executing = None;
                let status = session.with_native_step(|runtime, instance, _, _| {
                    executing = Some(runtime.original_unit_requires_execution(instance)?);
                    Ok(())
                });
                assert_ne!(status, GuestPollSessionStatus::Refused, "{:?}", session.failure());
                let Some(executing) = executing else { continue };
                if executing && !was_executing {
                    input_calls += 1;
                    if input_calls == 2 { before_input_execution = session.fixture_execution_spent(); }
                }
                was_executing = executing;
                if input_calls == 2 && executing != wait_for_result && session.fixture_execution_spent() > before_input_execution {
                    interrupted_input = true;
                    break;
                }
            }
        }
        if !origin_lookup_delayed && matches!(exercise, PollExercise::Deliver) {
            if let Some((true, issued, request)) = session.fixture_guest_request() {
                let law: serde_json::Value = serde_json::from_str(include_str!("🧫️origin-retry.json")).unwrap();
                assert_eq!(Some(request.header_bytes as u64), law["headerBytes"].as_u64());
                let status = while_origin_is_borrowed(origin, || session.drive_step(input.as_ref(), &mut |_, _, _| {
                    panic!("a blocked Boot lookup cannot deliver output");
                }));
                let (retrying_native, retained, retained_request) = session.fixture_guest_request().unwrap();
                assert_eq!(serde_json::json!({
                    "status": format!("{status:?}"),
                    "inputUnchanged": retained == issued,
                    "requestUnchanged": retained_request == request,
                }), law["duringContention"]);
                assert!(!retrying_native);
                origin_lookup_delayed = true;
            }
        }
        let status = session.drive_step(input.as_ref(), &mut |value, _, _| {
            assert!(*value);
            callback_count += 1;
            if callback_fails && callback_count == 1 {
                return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "fixture callback refused").into());
            }
            Ok(true)
        });
        if callback_fails && callback_count == 1 && !failure_observed {
            observe_and_retire_callback_failure(&mut session);
            failure_observed = true;
        }
        match status {
            GuestPollSessionStatus::Delivered => delivered = true,
            GuestPollSessionStatus::Complete => { complete = true; break; }
            GuestPollSessionStatus::Ready | GuestPollSessionStatus::Pending => {}
            status => panic!("actual Poll fixture {status:?}: {:?} {:?}", session.failure(), session.cleanup_failure()),
        }
    }
    if exhaust_acknowledgment.is_some() {
        assert!(refused_acknowledgment);
        assert_eq!(callback_count, 0);
    } else if close_input.is_some() {
        assert!(interrupted_input);
        assert_eq!(callback_count, 0);
    } else {
        assert!(delivered && complete);
        assert_eq!(origin_lookup_delayed, !callback_fails);
        assert_eq!(callback_count, if callback_fails { 2 } else { 1 });
        assert_eq!(failure_observed, callback_fails);
    }
    let mut closed_context = false;
    for _ in 0..100_000 {
        closed_context |= session.fixture_guest_request().is_some_and(|(_, budget, request)| budget.cancelled && request.header_bytes == 200);
        match session.close_step(&mut |_, _, _| panic!("Poll must not deliver during close")) {
            GuestPollSessionStatus::Closed => break,
            GuestPollSessionStatus::Pending => {}
            status => panic!("actual Poll fixture cleanup {status:?}: {:?} {:?}", session.failure(), session.cleanup_failure()),
        }
    }
    assert!(session.terminal_is_empty());
    if let Some((_, expected_context_close)) = close_input {
        assert_eq!(closed_context, expected_context_close);
    }
    for _ in 0..4096 {
        let closed = cache_source.turn(stock, |cx| cx.with_original_retirement(|cx| {
            origin_registry::close_step(identity, &runtime, &compiled, cx)
        })).unwrap();
        if closed {
            cache_source.retire(stock);
            return;
        }
    }
    panic!("fixture registry must receive the native home before closing");
}

fn spend_to_acknowledgment_limit(session: &mut GuestPollSession<bool, GuestPollNoReceiverState>, maximum_slice: usize, target: usize) {
    let bytes = vec![1; maximum_slice];
    let mut copied = vec![0; maximum_slice];
    for _ in 0..64 {
        if session.fixture_drive_copy() == target { return; }
        let status = session.with_native_step(|_, _, _, cx| {
            let count = cx.retained_grant().maximum_copy_bytes.saturating_sub(target);
            if count == 0 { return Ok(()); }
            cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: count, ..Default::default() })?;
            cx.consume_fuel(1);
            copied[..count].copy_from_slice(&bytes[..count]);
            assert_eq!(copied[count - 1], 1);
            Ok(())
        });
        assert_ne!(status, GuestPollSessionStatus::Refused, "{:?}", session.failure());
    }
    assert_eq!(session.fixture_drive_copy(), target);
}

fn while_origin_is_borrowed<T>(origin: semio_plugin_host_post_turn_relay::GuestRelayNativeOrigin, observe: impl FnOnce() -> T) -> T {
    std::thread::scope(|scope| {
        let (acquired, await_acquire) = std::sync::mpsc::sync_channel(0);
        let (release, await_release) = std::sync::mpsc::sync_channel(0);
        scope.spawn(move || {
            loop {
                let borrowed = (origin.resolve)(origin.cache_identity, origin.package_hash, &mut |_, _| {
                    acquired.send(()).unwrap();
                    let _ = await_release.recv();
                    Ok(())
                }).unwrap();
                if borrowed { break; }
                std::thread::yield_now();
            }
        });
        await_acquire.recv().unwrap();
        let observed = observe();
        release.send(()).unwrap();
        observed
    })
}

fn fixture_operation_driver(slice: RetainedCloneGrant, turn_execution_fuel: u64) -> semio_plugin_host_wake::GuestRelayWakeDriver {
    use semio_plugin_host_wake::{GuestRelayDriverPolicy, GuestRelayOperationCaps, GuestRelayOperationInfrastructure, GuestRelayOperationRequest, GuestRelayOperationReserve, GuestRelayOperationTransit, GuestRelayRootPolicy};
    let stock = RetainedCloneGrant {
        maximum_items: 10 * slice.maximum_items,
        maximum_copy_bytes: 10 * slice.maximum_copy_bytes,
        maximum_capacity_bytes: 10 * slice.maximum_capacity_bytes,
        maximum_release_bytes: 10 * slice.maximum_release_bytes,
        ..slice
    };
    let policy = GuestRelayDriverPolicy {
        admission: slice,
        drive: slice,
        wake: slice,
        operation: stock,
        root: GuestRelayRootPolicy {
            operation: 98, generation: 1,
            turn_fuel: 100_000, operation_fuel: 20_000_000,
            turn_execution_fuel, operation_execution_fuel: 2_000_000,
            maximum_turn_microseconds: 4_000_000, maximum_lifetime_microseconds: 10_000_000,
            max_effects: 256, max_patch_bytes: 1024, max_frames: 256,
        },
    };
    let cleanup = GuestRelayOperationReserve { retained: slice, work_fuel: 1_000_000, execution_fuel: 200_000, maximum_microseconds: 2_000_000 };
    let transit = GuestRelayOperationTransit { retained: slice, work_fuel: 1_000_000 };
    let infrastructure = GuestRelayOperationInfrastructure { retained: slice, work_fuel: 1_000_000, maximum_cleanup_microseconds: 1_000_000 };
    GuestRelayOperationCaps::project(policy, cleanup, transit, infrastructure)
        .issue(GuestRelayOperationRequest { work_units: 2_000_000, maximum_elapsed_microseconds: 10_000_000, retained: slice }, clock, Box::new(|| false))
        .unwrap()
}

fn observe_and_retire_callback_failure(session: &mut GuestPollSession<bool, GuestPollNoReceiverState>) {
    let mut failure = None;
    let mut observed = false;
    for _ in 0..4096 {
        let status = session.receive_state_step(|poll, cx| {
            if !observed {
                assert_eq!(poll.captured, Some(true));
                assert!(poll.deliver_step(cx, |_, _, _| panic!("unreceived callback failure permits retry")).is_err());
                observed = poll.take_transport_failure_step(cx, |original, _, _| {
                    let original = original.take().expect("retained callback refusal");
                    let primary = original.primary_value().unwrap();
                    assert_eq!(primary.kind, ValueRefusalKind::WorkLimit);
                    assert_eq!(primary.message, "fixture callback refused");
                    failure = Some(ControlledRetirement::retain(original));
                    Ok(())
                })?;
            } else if let Some(owner) = failure.as_mut() {
                if owner.terminal_is_empty() {
                    failure.take();
                } else {
                    let body = cx.retained_grant().maximum_copy_bytes;
                    let demand = owner.next_demand(body)?;
                    semio_framework_job::close_original_effect_step(cx, demand, |effect| owner.step(body, effect))?;
                }
            }
            Ok(())
        });
        assert_ne!(status, GuestPollSessionStatus::Refused, "{:?}", session.cleanup_failure());
        if observed && failure.is_none() {
            return;
        }
    }
    panic!("callback failure did not complete its Source handback");
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
  for _ in 0..100000{*epoch+=1;let result=caller.turn(grant(),|cx|{cx.bind_original_source_epoch(*epoch)?;cx.with_original_retirement(|cx|{let needs=state.original_call_requires_execution();if needs&&!runtime.prepare_original_call_retirement_step(state,cx)?{return Ok(None)}let mut input=RetainedTurnInput::borrow_original(cx)?;input.grant=cx.retained_grant();let execution=needs.then(||OriginalGuestExecutionBudget{input,fuel:1000,deadline_us:cx.deadline_us()});let progress=runtime.advance_original_call_step(state,execution,cx)?;if progress.result?==OwnedOriginalCallStatus::Ready{receive_scalar(runtime,state,cx)}else{Ok(None)}})}).unwrap();if let Some(result)=result{return result}}
  panic!("funded original cleanup did not finish")
 }
 let released=call(&runtime,&mut state,&mut caller,&mut epoch,OwnedSemioExport::Deallocate,&[Value::I32(2800),Value::I32(152)]);assert!(released.value.is_none());
 epoch+=1;let budget=caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;OriginalJobBudget::borrow_original(RetainedTurnInput::borrow_original(cx)?,cx).map_err(Into::into)}).unwrap();let args=budget.scalar_arguments().map(|value|Value::I64(value as i64));let closed=call(&runtime,&mut state,&mut caller,&mut epoch,OwnedSemioExport::CloseActorOutput,&args);let clock_reply=state.original_memory(0).unwrap();assert_eq!(&clock_reply[3008..3012],&1u32.to_le_bytes());assert_eq!(&clock_reply[3016..3024],&1000u64.to_le_bytes());let Some(Value::I64(word))=closed.value else{panic!("actual close returns its fixed receipt")};let pointer=word as u32;let length=(word as u64>>32)as usize;epoch+=1;caller.turn(stock,|cx|{cx.bind_original_source_epoch(epoch)?;runtime.with_original_memory_bytes(&state,pointer,length,cx,|bytes,_|{let(fixed,escrow)=OriginalActorFixedReceipt::read(bytes)?;assert_eq!((escrow.pointer,escrow.length),(3200,48));assert_eq!(fixed.input,budget.input);assert!(matches!(fixed.kind,OriginalActorFixedKind::Close{complete:true,blocked:false,refusal:None}));Ok(())})}).unwrap();
 let released=call(&runtime,&mut state,&mut caller,&mut epoch,OwnedSemioExport::Deallocate,&[Value::I32(pointer as i32),Value::I32(length as i32)]);assert!(released.value.is_none());close(&runtime,&compiled,state,&mut caller,&mut epoch);caller.retire(stock);
}
/// 🧳️ A retained consumer participates in actual native retirement after the Poll payload closes.
#[test]
fn actual_poll_receiver_state_blocks_native_terminal_until_source_retirement(){
 struct Receiver{closed:bool,failure:ActorFailureRecipient}
 impl GuestPollFailureReceiving for Receiver{fn actor_failure_recipient(&mut self)->&mut ActorFailureRecipient{&mut self.failure}}
 impl OriginalNativeRetirement for Receiver{
  fn close_on_source(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.closed{return self.failure.close_on_source(cx)}let grant=cx.retained_grant();let bytes=std::mem::size_of::<bool>();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0||cx.fuel_remaining()<1{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);self.closed=true;Ok(true)}
  fn terminal_is_empty(&self)->bool{self.closed&&self.failure.terminal_is_empty()}
 }
 let stock=grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(101),Generation(1),stock,clock,1000000,1000000);
 let mut poll=GuestPoll::<bool,Receiver>::new(OriginalNativeDecodePolicy{maximum_owned_bytes:stock.maximum_capacity_bytes},JsonReadLimits{maximum_bytes:1024,maximum_allocation_bytes:stock.maximum_capacity_bytes,maximum_depth:16,maximum_items:1024},Receiver{closed:false,failure:ActorFailureRecipient::empty()});
 assert!(!NativeInstanceRetirement::terminal_is_empty(&poll));let pointer=&poll.receiver as *const Receiver;
 let denied=RetainedCloneGrant{maximum_copy_bytes:0,..stock};let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(denied,|cx|poll.close_unopened(cx)));assert!(!result.unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(caller.receipt(),Default::default());assert!(!poll.receiver.closed);assert_eq!(&poll.receiver as *const Receiver,pointer);
 let mut done=false;for _ in 0..1000{if caller.turn(stock,|cx|poll.close_unopened(cx)).unwrap(){done=true;break}}
 assert!(done);assert!(poll.receiver.closed);assert!(NativeInstanceRetirement::terminal_is_empty(&poll));assert_eq!(&poll.receiver as *const Receiver,pointer);drop(poll);caller.retire(stock);
}

#[test]
fn actual_failure_control_owners_prevent_false_terminal_retirement(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️failure-retirement.json")).unwrap();
 let stock=grant();
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(109),Generation(1),stock,clock,100_000_000,1_000_000);
 let mut poll=GuestPoll::<bool,GuestPollNoReceiverState>::new(OriginalNativeDecodePolicy{maximum_owned_bytes:1024},JsonReadLimits{maximum_bytes:1024,maximum_allocation_bytes:1024,maximum_depth:16,maximum_items:1024},GuestPollNoReceiverState::new());
 finish_poll_payload(&mut poll,&mut caller);
 for row in corpus["cases"].as_array().unwrap(){
  for held in row["held"].as_array().unwrap(){
   match held.as_str().unwrap(){
    "born"=>poll.failure_born=Some(false),
    "active"=>poll.failure_active=true,
    "releasing"=>poll.failure_releasing=true,
    "pointer"=>poll.failure_pointer=1,
    "length"=>poll.failure_length=1,
    "plan"=>poll.failure_plan=Some(semio_plugin_host_actor_failure::ActorFailurePlan{kind:semio_framework_actor::ActorFailureCallKind::ReadActorFailure,prefix:[0;4],prefix_length:2,scoped:true}),
    _=>panic!("unknown neutral retained owner"),
   }
  }
  assert_eq!(poll.terminal_is_empty(),row["empty"].as_bool().unwrap(),"{row}");
  poll.failure_born=None;
  poll.failure_active=false;
  poll.failure_releasing=false;
  poll.failure_pointer=0;
  poll.failure_length=0;
  poll.failure_plan=None;
  assert!(poll.terminal_is_empty());
 }
 drop(poll);
 caller.retire(stock);
}

fn finish_poll_payload(poll:&mut GuestPoll<bool,GuestPollNoReceiverState>,caller:&mut OriginalJobFixtureCaller){
 for _ in 0..128{
  if caller.turn(grant(),|cx|cx.with_original_retirement(|cx|poll.close_unopened(cx))).unwrap(){return}
 }
 panic!("funded Poll fixture payload did not settle");
}

#[test]
fn next_round_component_preserves_its_original_source_and_denied_frontiers(){
 let law:serde_json::Value=serde_json::from_str(include_str!("rounds.json")).unwrap();let stock=grant();let mut caller=OriginalJobFixtureCaller::admit(OperationId(108),Generation(1),stock,clock,100_000_000,1_000_000);
 let mut poll=Box::new(GuestPoll::<bool,GuestPollNoReceiverState>::new(OriginalNativeDecodePolicy{maximum_owned_bytes:1024},JsonReadLimits{maximum_bytes:1024,maximum_allocation_bytes:1024,maximum_depth:16,maximum_items:1024},GuestPollNoReceiverState::new()));
 finish_poll_payload(&mut poll,&mut caller);
 caller.turn(stock,|cx|{
  assert!(poll.begin_next_step(cx).is_err());let bytes=std::mem::size_of_val(&poll.origin_binding)+std::mem::size_of_val(&poll.origin_address);assert!(charge_metadata(cx,false,bytes)?);poll.origin_binding=Some(OriginalSourceBinding::borrow(cx));poll.origin_address=Some(poll.as_ref()as*const _ as usize);
  let address=poll.as_ref()as*const _;let receiver=&poll.receiver as*const _;let denied=RetainedCloneGrant{maximum_items:0,..cx.retained_grant()};assert!(!cx.with_retained_loan(denied,|cx|poll.begin_next_step(cx))?);let denied=RetainedCloneGrant{maximum_copy_bytes:0,..cx.retained_grant()};assert!(!cx.with_retained_loan(denied,|cx|poll.begin_next_step(cx))?);let deadline=cx.deadline_us();assert!(!cx.with_original_work_loan(0,deadline,|cx|poll.begin_next_step(cx))?);
  poll.transport_failure=Some(invalid("retained round fixture cause"));assert!(!poll.begin_next_step(cx)?);assert!(poll.transport_failure.is_some());poll.transport_failure.take();assert!(!cx.with_original_retirement(|cx|poll.begin_next_step(cx))?);
  assert_eq!(poll.as_ref()as*const _,address);assert_eq!(&poll.receiver as*const _,receiver);Ok(())
 }).unwrap();
 let mut captured=false;for _ in 0..128{if caller.turn(stock,|cx|{let address=(&mut poll.transport_failure as*mut Option<OriginalTurnFailure>)as usize;let bytes=std::mem::size_of_val(&poll.transport_failure);if !poll.return_place.admit_sized(cx,address,bytes)?{return Ok(false)}poll.transport_failure=None;poll.return_place.record_sized_return(address,bytes)?;assert!(!poll.begin_next_step(cx)?);Ok(true)}).unwrap(){captured=true;break}}assert!(captured);for _ in 0..128{if poll.return_place.terminal_is_empty(){break}caller.turn(stock,|cx|poll.return_place.settle_return(cx)).unwrap();}assert!(poll.return_place.terminal_is_empty());
 let address=poll.as_ref()as*const _;let mut work=0u64;let mut births=0usize;
 for _ in 0..law["rounds"].as_u64().unwrap(){caller.turn(stock,|cx|{
  assert!(poll.origin_binding.unwrap().matches(cx));let before=cx.fuel_remaining();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||poll.begin_next_step(cx));assert!(result?);work+=before-cx.fuel_remaining();births+=heap.requested_bytes;assert_eq!(heap.released_bytes,0);assert!(poll.phase==Phase::Encode);assert_eq!(poll.as_ref()as*const _,address);assert!(!poll.begin_next_step(cx)?);Ok(())
 }).unwrap();finish_poll_payload(&mut poll,&mut caller);}
 assert_eq!(serde_json::json!({"births":births,"work":work}),serde_json::json!({"births":law["births"],"work":law["work"].as_u64().unwrap()*law["rounds"].as_u64().unwrap()}));caller.cancel().cancel_now();caller.turn(stock,|cx|{assert!(!poll.begin_next_step(cx)?);Ok(())}).unwrap();assert!(poll.terminal_is_empty());drop(poll);caller.retire(stock);
}

#[test]
fn scalar_cancellation_and_deadline_remain_actual_failure_receipts(){let corpus:serde_json::Value=serde_json::from_str(include_str!("🧫️failure-retirement.json")).unwrap();for row in corpus["scalar"].as_array().unwrap(){let status=match row["status"].as_str().unwrap(){"pending"=>OwnedOriginalCallStatus::Pending,"ready"=>OwnedOriginalCallStatus::Ready,"cancelled"=>OwnedOriginalCallStatus::Cancelled,"deadline"=>OwnedOriginalCallStatus::Deadline,_=>panic!("unknown scalar status")};let actual=OwnedOriginalCallProgress{result:Ok(status),execution:None};assert_eq!(scalar_failed(&actual),row["failed"].as_bool().unwrap())}}
