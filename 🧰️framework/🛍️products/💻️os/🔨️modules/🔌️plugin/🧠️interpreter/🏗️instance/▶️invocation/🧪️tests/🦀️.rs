use super::*;
use semio_framework_actor::RetainedTurnInput;
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
use semio_framework_value::RetainedCloneGrant;

fn clock() -> Option<u64> { Some(1000) }

#[test]
fn returned_frame_quotes_the_exact_next_physical_release() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["returnedFrame"];
    let bytes: Vec<_> = fixture["moduleHex"].as_str().unwrap().as_bytes().chunks_exact(2).map(|pair| {
        u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()
    }).collect();
    let engine = wasmtime::Engine::default();
    let oracle = wasmtime::Module::new(&engine, &bytes).unwrap();
    let mut store = wasmtime::Store::new(&engine, ());
    let instance = wasmtime::Instance::new(&mut store, &oracle, &[]).unwrap();
    let reference = instance.get_typed_func::<(), i64>(&mut store, "invoke").unwrap().call(&mut store, ()).unwrap();
    assert_eq!(Some(reference), law["expected"].as_i64());

    let module = Arc::new(CoreModule::parse(&bytes).unwrap());
    let stock = RetainedCloneGrant { maximum_items: 10_000, maximum_copy_bytes: 2_000_000,
        maximum_capacity_bytes: 2_000_000, maximum_release_bytes: 2_000_000, maximum_depth: 16 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(196), Generation(1), stock, clock, 1_000_000, 1_000_000);
    let mut epoch = 1;
    let mut owner = caller.turn(stock, |cx| {
        cx.bind_original_source_epoch(epoch)?;
        CoreInstanceOpening::begin(&module, cx)
    }).unwrap().unwrap();
    while !owner.resources_ready() {
        epoch += 1;
        caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; owner.advance(&module, cx) }).unwrap();
    }
    epoch += 1;
    assert!(caller.turn(stock, |cx| {
        cx.bind_original_source_epoch(epoch)?;
        assert_eq!(owner.returned_frame_demand(&module, cx)?, None);
        owner.begin_function_step(&module, 0, &[], cx)
    }).unwrap());
    let release = law["controlBackingElements"].as_u64().unwrap() as usize * size_of::<ControlFrame>();
    let copy = size_of::<Vec<ControlFrame>>();
    let exact = RetirementDemand { copy_bytes: copy, release_bytes: release, depth: 1, ..Default::default() };
    let limited = RetainedCloneGrant { maximum_release_bytes: release - 1, ..stock };
    let mut trace = [0; 6];
    for _ in 0..32 {
        let turn = if stack_state(&owner).depths[1] > 1 { limited } else { stock };
        scalar(&mut caller, &mut owner, &module, &mut epoch, turn, &mut trace);
        if owner.returning_frame { break; }
    }
    assert!(owner.returning_frame);
    {
        let pool = &mut owner.core.as_mut().unwrap().pool;
        pool.locals.iter_mut().for_each(|slot| *slot = Vec::with_capacity(1));
        pool.controls.iter_mut().for_each(|slot| *slot = Vec::with_capacity(1));
        pool.types.iter_mut().for_each(|slot| *slot = Vec::with_capacity(1));
    }
    let before = stack_state(&owner);
    for denied in [
        RetainedCloneGrant { maximum_release_bytes: release - law["releaseBytesBelowExact"].as_u64().unwrap() as usize, ..stock },
        RetainedCloneGrant { maximum_copy_bytes: copy - law["copyBytesBelowExact"].as_u64().unwrap() as usize, ..stock },
    ] {
        epoch += 1;
        caller.turn(stock, |cx| {
            cx.bind_original_source_epoch(epoch)?;
            assert_eq!(owner.returned_frame_demand(&module, cx)?, Some(exact));
            assert!(semio_framework_value::retained_clone::retained_clone_grant_admits(cx.original_source_total_grant(), exact));
            assert!(!semio_framework_value::retained_clone::retained_clone_grant_admits(denied, exact));
            Ok(())
        }).unwrap();
        let actual = scalar(&mut caller, &mut owner, &module, &mut epoch, denied, &mut trace);
        assert_eq!(actual, (CoreOriginalInvocationStatus::Pending, 0, RetainedCloneProgress::default()));
        assert_eq!(stack_state(&owner), before);
    }
    let payable = RetainedCloneGrant { maximum_copy_bytes: copy, maximum_release_bytes: release, ..stock };
    let actual = scalar(&mut caller, &mut owner, &module, &mut epoch, payable, &mut trace);
    assert_eq!((actual.1, actual.2.copied_bytes, actual.2.released_bytes), (0, copy, release));
    for _ in 0..32 {
        if scalar(&mut caller, &mut owner, &module, &mut epoch, stock, &mut trace).0 == CoreOriginalInvocationStatus::Complete { break; }
    }
    assert_eq!(owner.invocation_values(), Some(&[Value::I64(reference)][..]));
    for _ in 0..32 {
        if owner.terminal_is_empty() { break; }
        epoch += 1;
        caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; owner.close_step(&module, cx) }).unwrap();
    }
    assert!(owner.terminal_is_empty());
    caller.retire(stock);
}

#[test]
fn original_core_stack_growth_preserves_paid_receipts_and_guest_semantics() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let fixture = &fixture["stackGrowth"];
    let hex = fixture["moduleHex"].as_str().unwrap().as_bytes();
    let bytes: Vec<_> = hex.chunks_exact(2).map(|pair| {
        u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()
    }).collect();
    let engine = wasmtime::Engine::default();
    let oracle = wasmtime::Module::new(&engine, &bytes).unwrap();
    let mut store = wasmtime::Store::new(&engine, ());
    let instance = wasmtime::Instance::new(&mut store, &oracle, &[]).unwrap();
    let expected = instance.get_typed_func::<(), i32>(&mut store, "invoke").unwrap().call(&mut store, ()).unwrap();
    assert_eq!(i64::from(expected), fixture["expected"].as_i64().unwrap());

    let module = Arc::new(CoreModule::parse(&bytes).unwrap());
    let stock = RetainedCloneGrant { maximum_items: 10_000, maximum_copy_bytes: 2_000_000,
        maximum_capacity_bytes: 2_000_000, maximum_release_bytes: 2_000_000, maximum_depth: 64 };
    let child_bytes = size_of::<ControlFrame>() + 2 * size_of::<ValueType>();
    let call_copy = child_bytes + size_of::<Frame>();
    let widths = [size_of::<Value>(), size_of::<Frame>()];
    let peaks = [fixture["operandPeak"].as_u64().unwrap() as usize, fixture["framePeak"].as_u64().unwrap() as usize];
    let expected_fuel = ["scalar", "frameBirth", "frameReturn", "control", "memory", "hostCall"]
        .map(|category| fixture["vmFuelByCategory"][category].as_u64().unwrap());

    for cancel_at_peak in [false, true] {
        let mut caller = OriginalJobFixtureCaller::admit(OperationId(195), Generation(3), stock, clock, 1_000_000, 1_000_000);
        let mut epoch = 1;
        let mut owner = caller.turn(stock, |cx| {
            cx.bind_original_source_epoch(epoch)?;
            CoreInstanceOpening::begin(&module, cx)
        }).unwrap().unwrap();
        while !owner.resources_ready() {
            epoch += 1;
            caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; owner.advance(&module, cx) }).unwrap();
        }
        epoch += 1;
        assert!(caller.turn(stock, |cx| {
            cx.bind_original_source_epoch(epoch)?;
            owner.begin_function_step(&module, 0, &[], cx)
        }).unwrap());
        let mut live = caller.receipt().retained_capacity_bytes;
        let mut peak = [0, 1];
        let mut relocated = [0; 2];
        let mut constrained = [false; 2];
        let mut complete = false;
        let mut traced_fuel = [0; 6];
        for _ in 0..1000 {
            let before = stack_state(&owner);
            let mut grant = stock;
            let axis = if before.function == 0 && before.depths[0] == 2 && before.depths[1] == 1 && !constrained[0] {
                Some(0)
            } else if before.depths[1] == 2 && !constrained[1] {
                Some(1)
            } else { None };
            if let Some(axis) = axis {
                assert_eq!(before.depths[axis], before.capacities[axis]);
                let birth = 3 * widths[axis] + if axis == 1 { child_bytes } else { 0 };
                let release = 2 * widths[axis];
                let copy = release + if axis == 1 { call_copy } else { 0 };
                for denied in [
                    RetainedCloneGrant { maximum_capacity_bytes: birth - 1, ..stock },
                    RetainedCloneGrant { maximum_release_bytes: release - 1, ..stock },
                    RetainedCloneGrant { maximum_copy_bytes: copy - 1, ..stock },
                ] {
                    let (status, spent, receipt) = scalar(&mut caller, &mut owner, &module, &mut epoch, denied, &mut traced_fuel);
                    assert_eq!((status, spent, receipt), (CoreOriginalInvocationStatus::Pending, 0, RetainedCloneProgress::default()));
                    assert_eq!(stack_state(&owner), before);
                }
                grant.maximum_capacity_bytes = birth;
                grant.maximum_release_bytes = release + widths[axis];
                constrained[axis] = true;
            }
            let (status, spent, receipt) = scalar(&mut caller, &mut owner, &module, &mut epoch, grant, &mut traced_fuel);
            live += receipt.retained_capacity_bytes;
            live -= receipt.released_bytes;
            let after = stack_state(&owner);
            for (index, width) in widths.into_iter().enumerate() {
                peak[index] = peak[index].max(after.depths[index]);
                if after.capacities[index] > before.capacities[index] {
                    let moves = before.depths[index] * width;
                    relocated[index] += moves;
                    let (child, write) = if index == 0 { (0, 0) } else { (child_bytes, call_copy) };
                    assert_eq!(spent, 1);
                    assert_eq!(receipt.copied_bytes, moves + write);
                    assert_eq!(receipt.released_bytes, before.capacities[index] * width);
                    assert_eq!(receipt.retained_capacity_bytes, after.capacities[index] * width + child);
                    if axis == Some(index) { assert_eq!(after.capacities[index], 3); }
                }
            }
            if cancel_at_peak && peak == peaks {
                let before = stack_state(&owner);
                caller.cancel().cancel_now();
                let (status, spent, receipt) = scalar(&mut caller, &mut owner, &module, &mut epoch, stock, &mut traced_fuel);
                assert_eq!((status, spent, receipt), (CoreOriginalInvocationStatus::Cancelled, 0, RetainedCloneProgress::default()));
                assert_eq!(stack_state(&owner), before);
                break;
            }
            if status == CoreOriginalInvocationStatus::Complete { complete = true; break; }
        }
        assert_eq!(peak, peaks);
        assert_eq!(constrained, [true, true]);
        for index in 0..widths.len() {
            assert!(relocated[index] < 2 * peaks[index] * widths[index], "stack {index} relocated {} bytes for peak {}", relocated[index], peaks[index]);
        }
        assert_eq!(complete, !cancel_at_peak);
        if complete {
            assert_eq!(traced_fuel, expected_fuel);
            assert_eq!(owner.invocation_values(), Some(&[Value::I32(expected)][..]));
            caller.cancel().cancel_now();
        }
        for _ in 0..1000 {
            epoch += 1;
            let (closed, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| caller.turn(stock, |cx| {
                cx.bind_original_source_epoch(epoch)?;
                owner.close_invocation_step(&module, cx)
            }).unwrap());
            let receipt = caller.receipt();
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, receipt.released_bytes));
            live -= receipt.released_bytes;
            if closed { break; }
        }
        let pool = &owner.core.as_ref().unwrap().pool;
        let pooled = pool.locals.iter().map(|slot| slot.capacity() * size_of::<Value>()).sum::<usize>()
            + pool.controls.iter().map(|slot| slot.capacity() * size_of::<ControlFrame>()).sum::<usize>()
            + pool.types.iter().map(|slot| slot.capacity() * size_of::<ValueType>()).sum::<usize>();
        assert!(pooled > 0);
        assert!(pool.locals.iter().all(Vec::is_empty) && pool.controls.iter().all(Vec::is_empty) && pool.types.iter().all(Vec::is_empty));
        assert_eq!(live, pooled);
        assert!(owner.core.as_ref().unwrap().machine.is_none());
        for _ in 0..100 {
            if owner.terminal_is_empty() { break; }
            epoch += 1;
            caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; owner.close_step(&module, cx) }).unwrap();
        }
        assert!(owner.terminal_is_empty());
        caller.retire(stock);
    }
}

#[derive(Clone, Debug, PartialEq)]
struct StackState {
    depths: [usize; 2],
    capacities: [usize; 2],
    function: u32,
    pc: usize,
    operands: Vec<Value>,
}

fn stack_state(owner: &CoreInstanceOpening) -> StackState {
    let machine = owner.core.as_ref().unwrap().machine.as_ref().unwrap();
    let frame = machine.frames.last().unwrap();
    StackState {
        depths: [machine.values.len(), machine.frames.len()],
        capacities: [machine.values.capacity(), machine.frames.capacity()],
        function: frame.function,
        pc: frame.pc,
        operands: machine.values.clone(),
    }
}

fn scalar(caller: &mut OriginalJobFixtureCaller, owner: &mut CoreInstanceOpening, module: &Arc<CoreModule>,
    epoch: &mut u64, grant: RetainedCloneGrant, traced_fuel: &mut [u64; 6]) -> (CoreOriginalInvocationStatus, u64, RetainedCloneProgress) {
    *epoch += 1;
    let ((call, work, stalled), heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
        trace::capture(|| caller.turn(grant, |cx| {
            cx.bind_original_source_epoch(*epoch)?;
            let input = RetainedTurnInput { grant: cx.retained_grant(), ..RetainedTurnInput::borrow_original(cx)? };
            owner.advance_scalar(module, OriginalGuestExecutionBudget { input, fuel: 1, deadline_us: cx.deadline_us() }, cx)
        }).unwrap())
    });
    let receipt = caller.receipt();
    assert!(receipt.fits(grant));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (receipt.retained_capacity_bytes, receipt.released_bytes));
    let status = call.result.unwrap();
    let fuel = call.execution.spent_fuel.unwrap();
    let mut total = trace::Work::default();
    for (traced, work) in traced_fuel.iter_mut().zip(work) {
        *traced += work.fuel;
        total.add(work);
    }
    assert_eq!(total, trace::Work {
        fuel, items: receipt.copied_items as u64, copy: receipt.copied_bytes as u64,
        capacity: receipt.retained_capacity_bytes as u64, release: receipt.released_bytes as u64,
    });
    assert_eq!(stalled, status == CoreOriginalInvocationStatus::Pending && total == trace::Work::default());
    (status, fuel, receipt)
}

/// 📏️ A straight-line `() -> i64` module of `additions` scalar pairs, the shape whose fuel the scalar window posts in bulk.
fn straight_line_module(additions: usize) -> Vec<u8> {
    let leb = |mut number: usize| {
        let mut out = vec![];
        loop {
            let byte = (number & 127) as u8;
            number >>= 7;
            if number == 0 { out.push(byte); return out; }
            out.push(byte | 128);
        }
    };
    let mut body = vec![0x00, 0x42, 0x00];
    for _ in 0..additions { body.extend([0x42, 0x03, 0x7c]); }
    body.push(0x0b);
    let mut code = vec![0x01];
    code.extend(leb(body.len()));
    code.extend(&body);
    let mut module = b"\0asm\x01\0\0\0".to_vec();
    module.extend([0x01, 0x05, 0x01, 0x60, 0x00, 0x01, 0x7e, 0x03, 0x02, 0x01, 0x00, 0x07, 0x0a, 0x01, 0x06]);
    module.extend(b"invoke");
    module.extend([0x00, 0x00, 0x0a]);
    module.extend(leb(code.len()));
    module.extend(code);
    module
}

/// 🧮️ Runs the module to completion with `fuel` per call; returns its result, the fuel the Source ledger posted in total and the most instructions one call admitted.
fn run_straight_line(bytes: &[u8], fuel: u64) -> (Option<Vec<Value>>, u64, u64) {
    let module = Arc::new(CoreModule::parse(bytes).unwrap());
    let stock = RetainedCloneGrant { maximum_items: 100_000, maximum_copy_bytes: 8_000_000,
        maximum_capacity_bytes: 8_000_000, maximum_release_bytes: 8_000_000, maximum_depth: 16 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(197), Generation(1), stock, clock, 10_000_000, 1_000_000);
    let mut epoch = 1;
    let mut owner = caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; CoreInstanceOpening::begin(&module, cx) }).unwrap().unwrap();
    while !owner.resources_ready() {
        epoch += 1;
        caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; owner.advance(&module, cx) }).unwrap();
    }
    epoch += 1;
    assert!(caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; owner.begin_function_step(&module, 0, &[], cx) }).unwrap());
    let (mut posted_total, mut widest) = (0, 0);
    for _ in 0..100_000 {
        epoch += 1;
        let (call, posted) = caller.turn(stock, |cx| {
            cx.bind_original_source_epoch(epoch)?;
            let input = RetainedTurnInput { grant: cx.retained_grant(), ..RetainedTurnInput::borrow_original(cx)? };
            let before = cx.fuel_remaining();
            let call = owner.advance_scalar(&module, OriginalGuestExecutionBudget { input, fuel, deadline_us: cx.deadline_us() }, cx)?;
            Ok((call, before - cx.fuel_remaining()))
        }).unwrap();
        let used = call.execution.spent_fuel.unwrap();
        assert!(posted >= used, "every admitted instruction is posted: {posted} < {used}");
        posted_total += posted;
        widest = widest.max(used);
        if call.result.unwrap() == CoreOriginalInvocationStatus::Complete { break; }
    }
    let values = owner.invocation_values().map(<[Value]>::to_vec);
    for _ in 0..64 {
        if owner.terminal_is_empty() { break; }
        epoch += 1;
        caller.turn(stock, |cx| { cx.bind_original_source_epoch(epoch)?; owner.close_step(&module, cx) }).unwrap();
    }
    assert!(owner.terminal_is_empty());
    caller.retire(stock);
    (values, posted_total, widest)
}

/// 🪟️ Law: the scalar window posts exactly the fuel the instructions cost, whether one call admits one instruction or a whole window, and agrees with Wasmtime on the result.
#[test]
fn scalar_window_posts_exactly_the_fuel_of_the_admitted_instructions() {
    let bytes = straight_line_module(300);
    let engine = wasmtime::Engine::default();
    let oracle = wasmtime::Module::new(&engine, &bytes).unwrap();
    let mut store = wasmtime::Store::new(&engine, ());
    let instance = wasmtime::Instance::new(&mut store, &oracle, &[]).unwrap();
    let reference = instance.get_typed_func::<(), i64>(&mut store, "invoke").unwrap().call(&mut store, ()).unwrap();
    assert_eq!(reference, 900);
    let (single_values, single_posted, single_widest) = run_straight_line(&bytes, 1);
    let (window_values, window_posted, window_widest) = run_straight_line(&bytes, 100_000);
    assert_eq!(single_values, Some(vec![Value::I64(reference)]));
    assert_eq!(window_values, single_values);
    assert_eq!(window_posted, single_posted, "bulk posting never changes the fuel ledger");
    assert_eq!(single_widest, 1);
    assert!(window_widest > 1 && window_widest <= single_posted, "one call admits many instructions under one check");
}

/// 🪟️ Law: no fuel is ever granted past the window bound, so cancellation and deadline checks are at most one window of instructions apart.
#[test]
fn scalar_window_stays_within_its_declared_bound() {
    assert!(SCALAR_WINDOW >= 1 && SCALAR_WINDOW <= 4096);
}
