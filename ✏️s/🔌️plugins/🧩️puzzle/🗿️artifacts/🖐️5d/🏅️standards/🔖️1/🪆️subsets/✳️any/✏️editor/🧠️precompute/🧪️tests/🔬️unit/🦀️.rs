use super::*;
use semio_framework_tool_run::{ToolRunId, ToolRunStep, ToolRunStepKind};

fn upsert(key: u64) -> ToolRunTraceOp {
    ToolRunTraceOp::Upsert { key, verdict: ToolRunVerdict::Testing, reason: 0, subject: ToolRunTraceSubject::Instance3d { mesh: 1, position: [key as f32, 0.0, 0.0], rotation: [0.0, 0.0, 0.0, 1.0], scale: 1.0 } }
}

/// 📃️ A translated tick too large for one job payload page splits into consecutive ticks that each fit one page,
/// replay every trace record in order, carry the retraction, placements and entities only on the first and the
/// steps and plugin payload only on the last; a tick that fits stays one tick byte for byte.
#[test]
fn a_tick_over_one_payload_page_splits_into_in_order_pages() {
    let identity = ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 7 }, [3; 32]);
    let step = ToolRunStep { sequence: 1, kind: ToolRunStepKind::Info, stage: 0, reason: 0, subject: None, repeat: 1, args: Vec::new() };
    let ops: Vec<ToolRunTraceOp> = (0..2_000).map(upsert).collect();
    let tick = ToolRunTick { identity, sequence: 9, progress: None, steps: vec![step.clone()], trace: vec![ToolRunTracePage { identity, page: 0, ops: ops.clone() }], append_ops: vec![vec![1, 2, 3], vec![4]], append_entities: vec![11], retract_to: Some(2), payload: Some(vec![5, 6]) };
    let whole = tick.encode().expect("whole tick encodes");
    assert!(whole.len() > JOB_PAYLOAD_PAGE_BYTES, "the sample must overflow one page");
    let pages = puzzle5d_planner_tick_pages(tick).expect("split");
    assert!(pages.len() > 1);
    let ticks: Vec<ToolRunTick> = pages.iter().map(|bytes| {
        assert!(bytes.len() <= JOB_PAYLOAD_PAGE_BYTES);
        ToolRunTick::decode(bytes).expect("page decodes")
    }).collect();
    assert_eq!(ticks.iter().flat_map(|tick| tick.trace.iter().flat_map(|page| page.ops.iter().copied())).collect::<Vec<_>>(), ops);
    assert_eq!((ticks[0].append_ops.len(), ticks[0].append_entities.as_slice(), ticks[0].retract_to), (2, &[11][..], Some(2)));
    assert!(ticks[1..].iter().all(|tick| tick.append_ops.is_empty() && tick.append_entities.is_empty() && tick.retract_to.is_none()));
    assert_eq!((ticks.last().expect("last").steps.clone(), ticks.last().expect("last").payload.clone()), (vec![step], Some(vec![5, 6])));
    assert!(ticks[..ticks.len() - 1].iter().all(|tick| tick.steps.is_empty() && tick.payload.is_none()));
    let small = ToolRunTick { identity, sequence: 1, progress: None, steps: Vec::new(), trace: vec![ToolRunTracePage { identity, page: 0, ops: vec![upsert(1)] }], append_ops: Vec::new(), append_entities: Vec::new(), retract_to: None, payload: None };
    assert_eq!(puzzle5d_planner_tick_pages(small.clone()).expect("small"), vec![small.encode().expect("small encodes")]);
}
