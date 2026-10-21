use super::*;
use semio_framework_value::retained_clone::RetainedCloneSource;
use semio_framework_value_testing_allocator::observe_retirement_allocations as observe_heap_allocations_on_this_thread;


fn bytes(hex: &str) -> Vec<u8> {
    hex.as_bytes().chunks_exact(2).map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()).collect()
}

fn admitted_source(wire:Vec<u8>)->RetainedCloneSource<Vec<u8>>{
 let pointer=wire.as_ptr();let capacity=RetainedCloneSource::<Vec<u8>>::constructor_capacity_bytes::<()>();
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<Vec<u8>>::constructor_copy_bytes(),maximum_capacity_bytes:capacity,maximum_depth:1,..Default::default()};
 let(result,heap)=observe_heap_allocations_on_this_thread(||RetainedCloneSource::admit_owned(wire,(),grant));let(source,p)=result.unwrap_or_else(|_|panic!("original wire capture"));assert_eq!(source.borrow().get().as_ptr(),pointer);assert!(p.fits(grant));assert_eq!((heap.0,heap.1),(p.retained_capacity_bytes,p.released_bytes));source
}
fn hand_back_source_wire(source:&mut RetainedCloneSource<Vec<u8>>){
    use semio_framework_value::{retained_clone::RetainedCloneSourceTake,retirement::controlled::ControlledRetirement};
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let inline=law["maximumTurnBytes"].as_u64().unwrap()as usize;let fixed=law["sourceOwnership"]["fixedCloseTurns"].as_u64().unwrap()as usize;
    let pointer=source.borrow().get().as_ptr();let backing=source.borrow().get().capacity();let original=source.borrow().get().clone();let maximum=original.len().checked_add(fixed).expect("original wire work overflow");
    let mut born=backing.checked_add(RetainedCloneSource::<Vec<u8>>::constructor_capacity_bytes::<()>()).unwrap();let mut freed=0;let mut returned=None;let mut idle=0;
    for _ in 0..maximum{
        let(d,heap)=observe_heap_allocations_on_this_thread(||source.next_take_demand(inline).unwrap());assert_eq!(heap,(0,0));assert!(d.copy_bytes+d.capacity_bytes+d.release_bytes<=inline);
        let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(source.next_take_demand(g.maximum_copy_bytes).unwrap(),d);
        let(step,heap)=observe_heap_allocations_on_this_thread(||source.take_authority(g).unwrap());let p=match step{RetainedCloneSourceTake::Pending(p)=>p,RetainedCloneSourceTake::Ready(wire,p)=>{assert_eq!(wire.as_ptr(),pointer);assert_eq!(wire.capacity(),backing);assert_eq!(wire,original);returned=Some(wire);p}};assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;freed+=heap.1;if returned.is_some(){break;}let unchanged=p==RetainedCloneProgress::default()&&source.next_take_demand(inline).unwrap()==d;idle=if unchanged{idle+1}else{0};assert!(idle<16,"source handback repeated an unchanged empty event");
    }
    let wire=returned.expect("source must hand back its original caller wire");assert!(source.terminal_is_empty());let mut close=ControlledRetirement::new(wire).unwrap_or_else(|_|panic!("caller wire retains typed retirement"));idle=0;
    for _ in 0..maximum{
        if close.terminal_is_empty(){break;}let(d,heap)=observe_heap_allocations_on_this_thread(||close.next_demand(inline).unwrap());assert_eq!(heap,(0,0));assert!(d.copy_bytes+d.capacity_bytes<=inline);assert!(d.release_bytes<=inline||d.release_bytes==backing,"caller release must be the exact original backing extent");
        let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(close.next_demand(g.maximum_copy_bytes).unwrap(),d);let(step,heap)=observe_heap_allocations_on_this_thread(||close.step(g).unwrap());let p=step.progress();assert!(p.fits(g));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=heap.0;freed+=heap.1;let unchanged=p==RetainedCloneProgress::default()&&!close.terminal_is_empty()&&close.next_demand(inline).unwrap()==d;idle=if unchanged{idle+1}else{0};assert!(idle<16,"caller wire repeated an unchanged empty event");
    }
    assert!(close.terminal_is_empty());assert_eq!(born,freed);assert_eq!(observe_heap_allocations_on_this_thread(||drop(close)).1,(0,0));
}

fn observed(cursor: &mut RetainedInlineSymbols, source: &RetainedCloneSource<Vec<u8>>, grant: RetainedCloneGrant, closing: bool) -> Result<RetainedCloneProgress, InlineSymbolRefusal> {
    let (step, heap) = observe_heap_allocations_on_this_thread(|| if closing { cursor.close_granted(grant) } else { cursor.advance(source.borrow(), grant) });
    if let Ok(progress) = step {
        assert!(progress.fits(grant));
        assert_eq!(heap.0, progress.retained_capacity_bytes);
        assert_eq!(heap.1, progress.released_bytes);
        assert!(progress.copied_bytes + progress.retained_capacity_bytes + progress.released_bytes <= 4096);
    } else { assert_eq!((heap.0, heap.1), (0, 0)); }
    step
}

fn close(cursor: &mut RetainedInlineSymbols, source: &RetainedCloneSource<Vec<u8>>) {
    cursor.begin_close();
    for turn in 0..10000 {
        let (copy, heap) = observe_heap_allocations_on_this_thread(|| cursor.next_close_copy_byte_demand().unwrap());
        assert_eq!((heap.0, heap.1), (0, 0));
        let (release, heap) = observe_heap_allocations_on_this_thread(|| cursor.next_close_release_byte_demand().unwrap());
        assert_eq!((heap.0, heap.1), (0, 0));
        let ((capacity, depth), heap) = observe_heap_allocations_on_this_thread(|| (cursor.next_close_capacity_byte_demand(copy).unwrap(), cursor.next_close_depth_demand().unwrap()));
        assert_eq!((heap.0, heap.1), (0, 0));
        assert!(copy + capacity + release <= 4096);
        let before = cursor.len();
        assert_eq!(observed(cursor, source, RetainedCloneGrant::default(), true).unwrap(), RetainedCloneProgress::default());
        assert_eq!(cursor.len(), before);
        if copy > 0 { assert_eq!(observed(cursor, source, RetainedCloneGrant::one_payload_turn(copy - 1, 64), true).unwrap(), RetainedCloneProgress::default()); }
        if release > 0 { assert_eq!(observed(cursor, source, RetainedCloneGrant::one_release_turn(release - 1, 64), true).unwrap(), RetainedCloneProgress::default()); }
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: capacity, maximum_release_bytes: release, maximum_depth: depth };
        observed(cursor, source, grant, true).unwrap();
        if cursor.terminal_is_empty() {  return; }
    }
    panic!("retained symbols close did not reach terminal");
}

fn prepared(wire: Vec<u8>, expected: &[String], prefix: usize, cancel_at: Option<usize>, symbol_offset: usize) {
    let mut source = admitted_source(wire);
    let (mut cursor, heap) = observe_heap_allocations_on_this_thread(|| RetainedInlineSymbols::new(symbol_offset, 2048, 262144));
    assert_eq!((heap.0, heap.1), (0, 0));
    assert!(size_of::<RetainedInlineSymbols>() <= 4096);
    let mut other = admitted_source(vec![0]);
    for turn in 0..100000 {
        assert_eq!(observed(&mut cursor, &source, RetainedCloneGrant::default(), false).unwrap(), RetainedCloneProgress::default());
        let ((capacity, copied), heap) = observe_heap_allocations_on_this_thread(|| (cursor.next_capacity_byte_demand().unwrap(), cursor.next_copy_byte_demand()));
        assert_eq!((heap.0, heap.1), (0, 0));
        assert!(capacity + copied <= 4096);
        if capacity > 0 { assert_eq!(observed(&mut cursor, &source, RetainedCloneGrant::one_capacity_turn(capacity - 1, 64), false).unwrap(), RetainedCloneProgress::default()); }
        if copied > 0 { assert_eq!(observed(&mut cursor, &source, RetainedCloneGrant::one_payload_turn(copied - 1, 64), false).unwrap(), RetainedCloneProgress::default()); }
        let grant = if turn % 2 == 0 { RetainedCloneGrant::one_capacity_turn(4096, 64) } else { RetainedCloneGrant::one_payload_turn(4096, 64) };
        observed(&mut cursor, &source, grant, false).unwrap();
        if cancel_at == Some(turn) { close(&mut cursor, &source); break; }
        if cancel_at.is_some() && cursor.is_finished() { close(&mut cursor, &source); break; }
        if cursor.is_finished() {
            assert_eq!(cursor.prefix_bytes(), Some(prefix));
            assert_eq!(cursor.len(), expected.len());
            for (index, text) in expected.iter().enumerate() { assert_eq!(cursor.symbol_bytes(source.borrow(), index).unwrap(), Some(text.as_bytes())); }
            assert_eq!(cursor.symbol_bytes(source.borrow(), expected.len()).unwrap(), None);
            assert_eq!(cursor.symbol_bytes(other.borrow(), 0), Err(InlineSymbolRefusal::Source));
            close(&mut cursor, &source);
            break;
        }
    }
    assert!(cursor.terminal_is_empty());
    let (_, heap) = observe_heap_allocations_on_this_thread(|| drop(cursor));
    assert_eq!((heap.0, heap.1), (0, 0));
    hand_back_source_wire(&mut other);
    hand_back_source_wire(&mut source);
}

#[test]
fn retained_inline_symbols_native_canonical_utf8_paged_indices_and_exact_granted_close() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
    let mut original = admitted_source(vec![1, 1, b'x']);
    let mut changed = admitted_source(vec![1, 1, b'y']);
    let mut bound = RetainedInlineSymbols::new(0, 2048, 262144);
    observed(&mut bound, &original, RetainedCloneGrant::one_payload_turn(RetainedCloneBinding::alias_copy_bytes(), 64), false).unwrap();
    assert_eq!(observed(&mut bound, &changed, RetainedCloneGrant::one_payload_turn(RetainedCloneBinding::alias_copy_bytes(), 64), false), Err(InlineSymbolRefusal::Source));
    close(&mut bound, &original);
    hand_back_source_wire(&mut changed);
    hand_back_source_wire(&mut original);
    for row in law["valid"].as_array().unwrap() {
        let wire = bytes(row["hex"].as_str().unwrap());
        let expected: Vec<String> = row["symbols"].as_array().unwrap().iter().map(|text| text.as_str().unwrap().to_owned()).collect();
        let symbol_offset = row["sourceOffset"].as_u64().unwrap() as usize;
        prepared(wire.clone(), &expected, row["prefixBytes"].as_u64().unwrap() as usize, None, symbol_offset);
        prepared(wire, &expected, 0, Some(3), symbol_offset);
    }
    for row in law["invalid"].as_array().unwrap() {
        let mut source = admitted_source(bytes(row["hex"].as_str().unwrap()));
        let mut cursor = RetainedInlineSymbols::new(0, law["maximumSymbols"].as_u64().unwrap() as usize, law["maximumSymbolBytes"].as_u64().unwrap() as usize);
        let mut refusal = None;
        for turn in 0..100 {
            let grant = if turn % 2 == 0 { RetainedCloneGrant::one_capacity_turn(4096, 64) } else { RetainedCloneGrant::one_payload_turn(4096, 64) };
            if let Err(fault) = observed(&mut cursor, &source, grant, false) { refusal = Some(fault); break; }
        }
        let expected = match row["refusal"].as_str().unwrap() { "incomplete" => InlineSymbolRefusal::Incomplete, "noncanonical" => InlineSymbolRefusal::Noncanonical, "utf8" => InlineSymbolRefusal::Utf8, "symbol-count" => InlineSymbolRefusal::SymbolCount, "symbol-length" => InlineSymbolRefusal::SymbolLength, _ => unreachable!() };
        assert_eq!(refusal, Some(expected));
        close(&mut cursor, &source);
        let (_, heap) = observe_heap_allocations_on_this_thread(|| drop(cursor));
        assert_eq!((heap.0, heap.1), (0, 0));
        hand_back_source_wire(&mut source);
    }
    let large = law["large"]["word"].as_str().unwrap().repeat(law["large"]["repetitions"].as_u64().unwrap() as usize);
    assert_eq!(large.len(), law["large"]["bytes"].as_u64().unwrap() as usize);
    let mut wire = Vec::new();
    semio_framework_replication_codec::write_varint_u64(&mut wire, 1);
    semio_framework_replication_codec::write_varint_u64(&mut wire, large.len() as u64);
    wire.extend_from_slice(large.as_bytes());
    let prefix = wire.len();
    prepared(wire.clone(), &[large.clone()], prefix, None, 0);
    prepared(wire, &[large], 0, Some(17), 0);
    let mut wire = Vec::new();
    let empty_symbols = law["large"]["emptySymbols"].as_u64().unwrap() as usize;
    semio_framework_replication_codec::write_varint_u64(&mut wire, empty_symbols as u64);
    wire.resize(wire.len() + empty_symbols, 0);
    let prefix = wire.len();
    prepared(wire.clone(), &vec![String::new(); empty_symbols], prefix, None, 0);
    prepared(wire, &[], 0, Some(2000), 0);
}
                                  