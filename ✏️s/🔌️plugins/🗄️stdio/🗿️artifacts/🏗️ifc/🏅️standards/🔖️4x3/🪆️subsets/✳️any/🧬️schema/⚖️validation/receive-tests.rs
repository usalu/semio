//! 🧪️ Semantic receiving keeps original text borrowed and charges each real arena frontier.
use super::{overlay::*, receive::*};
use semio_framework_plugin::app_testing::OriginalWorkFixture;
use semio_framework_value::retained_clone::RetainedCloneGrant;

fn measured<'context, T>(
    cx: &mut semio_framework_job::StepContext<'context>,
    transition: impl FnOnce(&mut semio_framework_job::StepContext<'context>) -> Result<T, semio_framework_job::OriginalTurnFailure>,
) -> Result<T, semio_framework_job::OriginalTurnFailure> {
    let before = cx.retained_progress();
    let (answer, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| transition(cx));
    let after = cx.retained_progress();
    assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
    answer
}

#[test]
fn semantic_receiving_preserves_original_text_and_closes_every_completed_prefix() {
    let source = IfcSnapshot {
        header: semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcHeader { file_schema: vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3_ADD2".into())])], ..Default::default() },
        entities: vec![IfcEntity { id: 1, name: "IFCPROPERTYSINGLEVALUE".into(), args: vec![IfcValue::String("x".repeat(8192)), IfcValue::Unset, IfcValue::Unset, IfcValue::Unset], complex: vec![] }],
        ..Default::default()
    };
    let grant = RetainedCloneGrant { maximum_items: 100_000, maximum_copy_bytes: 16_384, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 256 };
    let mut prefixes = vec![None];
    while let Some(cut) = prefixes.pop() {
        let mut caller = OriginalWorkFixture::with_identity(semio_framework_job::OperationId(922), semio_framework_job::Generation(1));
        let mut overlay = PopulationOverlay::default();
        while !caller.work_turn(grant, 1, |cx| overlay.advance(&source, &[], cx)).unwrap() {}
        let mut receiver = PopulationReceiver::default();
        let mut complete = false;
        for turn in 0..10_000 {
            if cut == Some(turn) {
                caller.cancel().cancel_now();
            }
            let step = caller.work_turn(grant, 1, |cx| {
                let before = cx.retained_progress();
                let (answer, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| receiver.advance(&overlay, &source, &[], cx));
                let after = cx.retained_progress();
                assert_eq!((heap.requested_bytes, heap.released_bytes), (after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes));
                answer
            });
            match step {
                Ok(true) => {
                    complete = true;
                    if cut.is_none() {
                        prefixes.extend((0..=turn).map(Some));
                    }
                    break;
                }
                Ok(false) => {}
                Err(_) => break,
            }
        }
        if cut.is_none() {
            assert!(complete);
            let arena = receiver.arena().unwrap();
            assert_eq!(arena.nodes.len(), 4);
            let original = source.entities[0].args[0].as_str().unwrap();
            let actual = receiver.source_value(&overlay, &source, &[], 0).unwrap().as_str().unwrap();
            assert_eq!(original.as_ptr(), actual.as_ptr());
        }
        receiver.begin_close();
        for _ in 0..10_000 {
            if receiver.terminal_is_empty() {
                break;
            }
            let demand = receiver.close_demands(4096).unwrap();
            if demand.release_bytes > 0 {
                let denied = RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant };
                caller.turn(denied, |cx| cx.with_original_retirement(|cx| measured(cx, |cx| semio_framework_job::close_original_retained_step(cx, demand, |_| panic!("underfunded original release advanced its owner"))))).unwrap();
            }
            caller.turn(grant, |cx| cx.with_original_retirement(|cx| measured(cx, |cx| semio_framework_job::close_original_retained_step(cx, demand, |loan| receiver.close_step(loan))))).unwrap();
        }
        assert!(receiver.terminal_is_empty());
        overlay.begin_close();
        for _ in 0..10_000 {
            if overlay.terminal_is_empty() {
                break;
            }
            caller.turn(grant, |cx| cx.with_original_retirement(|cx| measured(cx, |cx| semio_framework_job::close_original_retained_step(cx, overlay.close_demands(4096)?, |loan| overlay.close_step(loan))))).unwrap();
        }
        assert!(overlay.terminal_is_empty());
        caller.finish();
    }
}
