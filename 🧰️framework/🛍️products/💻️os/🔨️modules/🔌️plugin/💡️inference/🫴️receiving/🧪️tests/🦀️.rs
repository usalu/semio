use super::*;
use semio_framework_job::{OriginalJobDecodeOwner, OriginalJobFixtureCaller, OperationId, Generation};
use semio_framework_value::{RetainedCloneGrant, RetireOwned};

fn stock() -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 100_000, maximum_copy_bytes: 64 << 20, maximum_capacity_bytes: 64 << 20, maximum_release_bytes: 64 << 20, maximum_depth: 128 } }
fn caller() -> OriginalJobFixtureCaller { OriginalJobFixtureCaller::admit(OperationId(11), Generation(3), stock(), || Some(0), u64::MAX, 1_000_000) }
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap() }
fn receive(source: &mut Option<ArtifactDocumentPayload>, caller: &mut OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner) -> OriginalArtifactDocumentReceiving {
    let mut output = None;
    let (admitted, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| caller.turn_with_decoder(stock(), decoder, |cx| OriginalArtifactDocumentReceiving::admit_original(source, &mut output, 4096, cx)));
    assert!(admitted.unwrap()); assert!(source.is_none()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); output.unwrap()
}
fn close(owner: &mut OriginalArtifactDocumentReceiving, caller: &mut OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner) {
    let mut turns = 0;
    while !owner.terminal_is_empty() {
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| caller.turn_with_decoder(stock(), decoder, |cx| cx.with_original_retirement(|cx| owner.close_step(cx))));
        let progress = step.unwrap().progress(); assert_eq!(caller.receipt(), progress); assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes)); turns += 1; assert!(turns < 100_000);
    }
}
fn close_value<T: RetireOwned>(value: T, caller: &mut OriginalJobFixtureCaller, decoder: &mut OriginalJobDecodeOwner) {
    let mut owner = ControlledRetirement::retain(value);
    while !owner.terminal_is_empty() { caller.turn_with_decoder(stock(), decoder, |cx| cx.with_original_retirement(|cx| close_original_retained_step(cx, owner.demands()?, |grant| owner.step(grant)))).unwrap(); }
}
fn finish(caller: OriginalJobFixtureCaller, mut decoder: OriginalJobDecodeOwner) {
    let mut caller = caller;
    while !decoder.terminal_is_empty() { caller.turn(stock(), |cx| { let step = decoder.close_step(cx.retained_grant())?; cx.consume_retained(step.progress())?; cx.consume_fuel(step.progress().copied_items as u64); Ok(()) }).unwrap(); }
    caller.retire(stock());
}

#[test]
fn artifact_document_original_pair_receiving_preserves_exact_sources_and_physical_receipts() {
    let law = fixture();
    for row in law["valid"].as_array().unwrap() {
        let mut caller = caller(); let mut decoder = OriginalJobDecodeOwner::empty();
        let mut source = Some(ArtifactDocumentPayload { pack: row["pack"].as_str().unwrap().to_owned(), spr: row["spr"].as_str().unwrap().to_owned() });
        let pointers = [source.as_ref().unwrap().pack.as_ptr(), source.as_ref().unwrap().spr.as_ptr()];
        let mut owner = receive(&mut source, &mut caller, &mut decoder);
        let mut done = false;
        for _ in 0..100_000 {
            let demand = owner.next_demands().unwrap();
            for axis in law["deniedAxes"].as_array().unwrap() {
                let mut denied = stock();
                match axis.as_str().unwrap() { "items" => denied.maximum_items = 0, "copy" => denied.maximum_copy_bytes = demand.copy_bytes.saturating_sub(1), "capacity" if demand.capacity_bytes != 0 => denied.maximum_capacity_bytes = demand.capacity_bytes - 1, "capacity" => continue, "depth" => denied.maximum_depth = demand.depth.saturating_sub(1), _ => unreachable!() }
                let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| caller.turn_with_decoder(denied, &mut decoder, |cx| owner.step(cx)));
                assert!(!result.unwrap()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0)); assert_eq!(caller.receipt(), Default::default());
            }
            let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| caller.turn_with_decoder(stock(), &mut decoder, |cx| owner.step(cx)));
            let progress = caller.receipt(); assert_eq!((heap.requested_bytes, heap.released_bytes), (progress.retained_capacity_bytes, progress.released_bytes));
            for (index, cursor) in owner.cursors.iter().enumerate() { if let Base64Input::OwnedText(text) = cursor.as_ref().unwrap().input() { assert_eq!(text.as_ptr(), pointers[index]); } else { panic!("original document text owner changed"); } }
            if result.unwrap() { done = true; break; }
        }
        assert!(done);
        let decoded = [owner.decoded[0].as_ref().unwrap().as_ptr(), owner.decoded[1].as_ref().unwrap().as_ptr()];
        let mut pair = None; assert!(caller.turn_with_decoder(stock(), &mut decoder, |cx| owner.take_pair(&mut pair, cx)).unwrap());
        let pair = pair.unwrap(); assert_eq!(pair.0.as_ptr(), decoded[0]); assert_eq!(pair.1.as_ptr(), decoded[1]);
        let expected = |key: &str| row[key].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect::<Vec<_>>();
        assert_eq!(pair.0, expected("packBytes")); assert_eq!(pair.1, expected("sprBytes"));
        close(&mut owner, &mut caller, &mut decoder);
        let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner)); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        close_value(pair, &mut caller, &mut decoder); finish(caller, decoder);
    }
    eprintln!("[DEBUG] original artifact document pair retains exact text and decoded pointers, refuses all deficient axes and reconciles physical construction/close receipts");
}

#[test]
fn artifact_document_original_receiving_refusal_cancellation_and_foreign_source_retain_partial_custody() {
    for encoded in fixture()["invalid"].as_array().unwrap() {
        let mut caller = caller(); let mut decoder = OriginalJobDecodeOwner::empty();
        let mut source = Some(ArtifactDocumentPayload { pack: encoded.as_str().unwrap().to_owned(), spr: "".into() });
        let mut owner = receive(&mut source, &mut caller, &mut decoder);
        let mut refused = false;
        for _ in 0..100 { if caller.turn_with_decoder(stock(), &mut decoder, |cx| owner.step(cx)).is_err() { refused = true; break; } }
        assert!(refused); assert!(owner.fault().is_some()); assert!(owner.cursors[0].is_some());
        close(&mut owner, &mut caller, &mut decoder); drop(owner); finish(caller, decoder);
    }
    let mut original = caller(); let mut decoder = OriginalJobDecodeOwner::empty(); let mut foreign = caller(); let mut foreign_decoder = OriginalJobDecodeOwner::empty();
    let mut source = Some(ArtifactDocumentPayload { pack: "AAEC/w==".into(), spr: "c3ByAAEC".into() }); let mut owner = receive(&mut source, &mut original, &mut decoder);
    let (refusal, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| foreign.turn_with_decoder(stock(), &mut foreign_decoder, |cx| owner.step(cx)));
    assert!(refusal.is_err()); assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    for _ in 0..100 { original.turn_with_decoder(stock(), &mut decoder, |cx| owner.step(cx)).unwrap(); if owner.cursors[0].as_ref().unwrap().output_capacity() != 0 { break; } }
    assert!(owner.cursors[0].as_ref().unwrap().output_capacity() != 0); original.cancel().cancel_now();
    assert!(original.turn_with_decoder(stock(), &mut decoder, |cx| owner.step(cx)).is_err());
    close(&mut owner, &mut original, &mut decoder); drop(owner); finish(original, decoder); finish(foreign, foreign_decoder);
    eprintln!("[DEBUG] original document malformed input, cancelled partial decode and equal-number foreign caller preserve actual source custody until paid close");
}
