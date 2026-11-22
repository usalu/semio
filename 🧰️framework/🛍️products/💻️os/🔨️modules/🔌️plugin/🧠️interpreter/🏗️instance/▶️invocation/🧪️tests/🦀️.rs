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
    let call_copy = child_bytes + size_of::<Frame>() + size_of::<Value>() + 2 * size_of::<usize>();
    let push_copy = size_of::<Value>() + size_of::<usize>();
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
            let mut before = stack_state(&owner);
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
                if axis == 0 {
                    let growth_only = RetainedCloneGrant {
                        maximum_capacity_bytes: birth,
                        maximum_copy_bytes: copy + push_copy - 1,
                        ..stock
                    };
                    let (status, spent, receipt) = scalar(&mut caller, &mut owner, &module, &mut epoch, growth_only, &mut traced_fuel);
                    assert_eq!((status, spent), (CoreOriginalInvocationStatus::Pending, 0));
                    assert_eq!((receipt.copied_bytes, receipt.retained_capacity_bytes, receipt.released_bytes), (copy, birth, release));
                    let after_growth = stack_state(&owner);
                    assert_eq!(after_growth, StackState { capacities: [3, before.capacities[1]], ..before.clone() });
                    live += birth - release;
                    relocated[axis] += copy;
                    before = after_growth;
                    grant.maximum_capacity_bytes = 0;
                } else {
                    grant.maximum_capacity_bytes = birth;
                    grant.maximum_release_bytes = release + widths[axis];
                }
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
                    let (child, write) = if index == 0 { (0, push_copy) } else { (child_bytes, call_copy) };
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
        assert_eq!(live, 0);
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
