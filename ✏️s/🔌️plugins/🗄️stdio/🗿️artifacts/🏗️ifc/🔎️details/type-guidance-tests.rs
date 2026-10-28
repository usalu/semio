//! 🧾️ A refused original construction keeps its born string on the same typed row owner.
use super::*;
#[test]
fn original_multiple_reference_row_retains_constructed_text_when_receipt_quote_refuses() {
    use semio_framework_plugin::app_testing::OriginalWorkFixture;
    let mut caller = OriginalWorkFixture::new();
    let stock = OriginalWorkFixture::stock();
    let mut parts = Some(RowParts { strings: Default::default(), outer: None, inner: None, bits:None, structured:None });
    let demand = OriginalConstructionDemand { items: 1, copy_bytes: 4096, capacity_bytes: 19, release_bytes: 0, depth: 1 };
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| caller.work_turn(stock, 1, |cx| construct_row_string(&mut parts.as_mut().unwrap().strings[0], 20, "original typed draft", demand, cx)));
    assert!(result.is_err());
    assert_eq!(parts.as_ref().unwrap().strings[0].as_deref(), Some("original typed draft"));
    assert!(heap.requested_bytes >= 20);
    assert_eq!(heap.released_bytes, 0);
    let mut closing = RetainedCloneClose::default();
    let mut released = 0;
    for turn in 0..100000 {
        if parts.is_none() && closing.is_empty() {
            break;
        }
        let demand = closing.next_owner_demands::<RowParts>(parts.is_some(), &None, 4096).unwrap();
        caller
            .turn(stock, |cx| {
                cx.with_original_retirement(|cx| {
                    let step = semio_framework_job::close_original_retained_step(cx, demand, |grant| if closing.is_empty() { closing.begin_granted(&mut parts, grant)?.ok_or_else(invalid) } else { closing.step_granted(grant) })?;
                    released += step.progress().released_bytes;
                    Ok(())
                })
            })
            .unwrap();
        assert!(turn < 99999);
    }
    assert!(parts.is_none() && closing.is_empty());
    assert!(released >= 20);
    caller.finish();
}
