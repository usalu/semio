use super::*;
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::{ObjRef,PdfObject};

const ONE: CosGrant = CosGrant { work: 1, allocation_bytes: 256 * 1024, release_bytes: 256 * 1024 };

fn native(number: u32, value: &[u8]) -> PdfIndirectObject {
    PdfIndirectObject { id: ObjRef { num: number, gen: 3 }, value: PdfObject::Str(value.to_vec()) }
}

fn ready(builder: &mut RetainedGraphBuilder) -> usize {
    let mut allocated = 0;
    for _ in 0..10_000 {
        let step = builder.advance(ONE, false);
        assert!(step.work <= 1);
        allocated += step.allocated_bytes;
        if step.event.unwrap() == GraphMaterializationEvent::Ready { return allocated; }
    }
    panic!("graph materialization did not progress");
}

#[test]
fn graph_materialization_moves_values_and_couples_positions_to_authored_order() {
    let mut builder = RetainedGraphBuilder::new(3);
    let first = native(73, b"first");
    let pointer = first.value.as_str_bytes().unwrap().as_ptr();
    let first = builder.offer(first).expect_err("graph is initially unfunded");
    assert_eq!(first.value.as_str_bytes().unwrap().as_ptr(), pointer);
    let no_allocation = CosGrant { allocation_bytes: 0, ..ONE };
    assert!(matches!(builder.advance(no_allocation, false).event, Ok(GraphMaterializationEvent::NeedsAllocation { .. })));
    ready(&mut builder);
    builder.offer(first).unwrap();
    ready(&mut builder);
    builder.offer(native(2, b"at the second offset")).unwrap();
    ready(&mut builder);

    let (objects, positions) = builder.finish().unwrap();
    assert_eq!(objects.iter().map(|value| value.id.num).collect::<Vec<_>>(), [73, 2]);
    assert_eq!(objects[0].value.as_str_bytes().unwrap().as_ptr(), pointer);
    assert_eq!(positions.get(&73), Some(&XrefEntry::Normal { offset: 0, gen: 3 }));
    assert_eq!(positions.get(&2), Some(&XrefEntry::Normal { offset: 1, gen: 3 }));
    assert!(builder.finish().is_none());
}

#[test]
fn cancellation_lends_pending_and_committed_values_to_original_retirement() {
    let mut builder = RetainedGraphBuilder::new(3);
    let mut allocated = ready(&mut builder);
    let first = native(73, b"first");
    let pointer = first.value.as_str_bytes().unwrap().as_ptr();
    builder.offer(first).unwrap();
    allocated += ready(&mut builder);
    builder.offer(native(2, b"still pending")).unwrap();
    assert_eq!(builder.advance(ONE, true).event, Ok(GraphMaterializationEvent::Cancelled));
    assert!(builder.finish().is_none());

    let mut released = 0;
    let mut received = Vec::new();
    for _ in 0..10_000 {
        let step = builder.close_step(ONE);
        assert!(step.work <= 1);
        released += step.released_bytes;
        match step.event.unwrap() {
            GraphMaterializationEvent::More => {}
            GraphMaterializationEvent::RetireObject => received.push(builder.take_retired_object().unwrap()),
            GraphMaterializationEvent::Closed => break,
            event => panic!("unexpected retained graph retirement {event:?}"),
        }
    }
    assert_eq!(received.iter().map(|object| object.id.num).collect::<Vec<_>>(), [2, 73]);
    assert_eq!(received[1].value.as_str_bytes().unwrap().as_ptr(), pointer);
    assert_eq!(released, allocated);
}
