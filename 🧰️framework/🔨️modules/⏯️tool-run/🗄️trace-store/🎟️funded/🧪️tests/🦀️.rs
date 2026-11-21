//! 🧪️ Checks funded application against the declared trace replay contract.

use super::*;
use semio_framework_value::RetainedCloneGrant;
use semio_framework_value_testing_effect_bank::EffectBank;

#[path = "../../../🧪️tests/⏱️effect/🦀️.rs"]
mod effect_harness;

fn identity() -> ToolRunIdentity {
    ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 5 }, [0; 32])
}

fn upsert(key: u64, verdict: ToolRunVerdict) -> ToolRunTraceOp {
    ToolRunTraceOp::Upsert { key, verdict, reason: 1, subject: ToolRunTraceSubject::Entity { entity: key } }
}

fn page(identity: ToolRunIdentity, ops: Vec<ToolRunTraceOp>) -> [ToolRunTracePage; 1] {
    [ToolRunTracePage { identity, page: 0, ops }]
}

fn apply_one(
    bank: &mut EffectBank,
    store: &mut ToolRunTraceStore,
    cursor: &mut ToolRunTraceApplyCursor,
    pages: &[ToolRunTracePage],
    heap: &mut (usize, usize),
) -> ToolRunTraceFundedStep {
    let demand = store.next_funded_apply_demand(cursor, pages).unwrap();
    let grant = effect_harness::grant(demand);
    let (denied, observed) = effect_harness::primitive(bank, RetainedCloneGrant::default(), |effect| {
        store.step_funded_apply(cursor, pages, effect)
    });
    let denied = denied.unwrap();
    assert!(denied.applied.is_none() && !denied.complete);
    assert_eq!(denied.progress, RetainedCloneProgress::default());
    assert_eq!(observed, (0, 0));
    let (step, observed) = effect_harness::primitive(bank, grant, |effect| {
        store.step_funded_apply(cursor, pages, effect)
    });
    let step = step.unwrap_or_else(|error| panic!("funded demand {demand:?}: {error:?}"));
    assert!(step.progress.fits(grant));
    assert_eq!(observed, (step.progress.retained_capacity_bytes, step.progress.released_bytes));
    assert!(step.complete || step.progress != RetainedCloneProgress::default(), "unsettled demand {demand:?}");
    bank.post(step.progress, grant, observed);
    heap.0 += observed.0;
    heap.1 += observed.1;
    step
}

fn apply(
    bank: &mut EffectBank,
    store: &mut ToolRunTraceStore,
    cursor: &mut ToolRunTraceApplyCursor,
    pages: &[ToolRunTracePage],
    heap: &mut (usize, usize),
) -> (Vec<ToolRunTraceOp>, u32, u32) {
    let (mut applied, mut evicted, mut overflowed) = (Vec::new(), 0, 0);
    for _ in 0..512 {
        let step = apply_one(bank, store, cursor, pages, heap);
        applied.extend(step.applied);
        evicted += step.evicted;
        overflowed += step.overflowed;
        if step.complete {
            return (applied, evicted, overflowed);
        }
    }
    panic!("funded trace apply never completed")
}

fn read_ops(rows: &serde_json::Value) -> Vec<ToolRunTraceOp> {
    rows.as_array().unwrap().iter().map(|row| {
        let key = row[1].as_u64().unwrap();
        match row[0].as_str().unwrap() {
            "upsert" => upsert(key, match row[2].as_str().unwrap() {
                "success" => ToolRunVerdict::Success,
                "danger" => ToolRunVerdict::Danger,
                "warning" => ToolRunVerdict::Warning,
                "testing" => ToolRunVerdict::Testing,
                verdict => panic!("unknown fixture verdict {verdict}"),
            }),
            "retire" => ToolRunTraceOp::Retire { key },
            op => panic!("unknown fixture op {op}"),
        }
    }).collect()
}

#[test]
fn original_trace_reports_every_input_once_and_keeps_window_replay() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️funded-replay.json")).unwrap();
    let mut bank = effect_harness::bank();
    let mut store = ToolRunTraceStore::with_limits(identity(), fixture["capacity"].as_u64().unwrap() as usize, 100);
    let pages = page(identity(), read_ops(&fixture["operations"]));
    let mut heap = (0, 0);
    let (applied, evicted, overflowed) = apply(&mut bank, &mut store, &mut Default::default(), &pages, &mut heap);
    assert_eq!(applied, pages[0].ops);
    assert_eq!(u64::from(evicted), fixture["evicted"].as_u64().unwrap());
    assert_eq!(u64::from(overflowed), fixture["overflowed"].as_u64().unwrap());
    let keys = store.records().map(|(key, _)| key).collect::<Vec<_>>();
    assert_eq!(serde_json::json!(keys), fixture["resident"]);
    let complete = store.delta_after(None, usize::MAX);
    assert!(complete.clear);
    let expected = read_ops(&fixture["replay"]);
    assert_eq!(complete.pages.iter().flat_map(|page| page.ops.iter().copied()).collect::<Vec<_>>(), expected);
    assert_eq!(ToolRunTraceDelta::decode(&complete.encode().unwrap()).unwrap(), complete);
    let mut cursor = None;
    let mut replay = Vec::new();
    for _ in 0..complete.pages.len() {
        let delta = store.delta_after(cursor, 1);
        assert_eq!(delta.clear, cursor.is_none());
        assert_eq!(delta.pages.len(), 1);
        replay.extend_from_slice(&delta.pages[0].ops);
        cursor = Some(ToolRunTraceCursor { generation: identity().generation, page: delta.next, run: identity().id.run });
    }
    assert_eq!(replay, expected);
    assert_eq!(store.delta_after(None, usize::MAX), complete);
    assert!(store.delta_after(cursor, 1).pages.is_empty());
    let closed = effect_harness::retire(&mut bank, store);
    assert_eq!(heap.0 + closed.0, heap.1 + closed.1);
}

#[test]
fn trace_compaction_and_retirement_resume_after_source_abandonment() {
    let mut bank = effect_harness::bank();
    let mut store = ToolRunTraceStore::with_limits(identity(), 3, 1);
    let mut heap = (0, 0);
    let pages = page(identity(), vec![
        upsert(3, ToolRunVerdict::Warning),
        upsert(3, ToolRunVerdict::Success),
        upsert(5, ToolRunVerdict::Danger),
        ToolRunTraceOp::Clear,
        upsert(9, ToolRunVerdict::Success),
    ]);
    let mut abandoned = ToolRunTraceApplyCursor::new();
    for _ in 0..128 {
        apply_one(&mut bank, &mut store, &mut abandoned, &pages, &mut heap);
        if store.compaction.first().is_some_and(|state| state.rank != 0) {
            break;
        }
    }
    assert!(store.compaction.first().is_some_and(|state| state.rank != 0));
    assert_eq!(store.len(), 0);
    assert_eq!(store.log_base(), 0);
    let generation = identity().next_generation();
    store.rebind(generation);
    assert!(store.next_funded_apply_demand(&abandoned, &pages).is_err());
    let next = page(generation, vec![upsert(7, ToolRunVerdict::Success), ToolRunTraceOp::Retire { key: 3 }]);
    let observed = apply(&mut bank, &mut store, &mut Default::default(), &next, &mut heap);
    assert_eq!(observed.0, next[0].ops);
    assert!(!store.retiring && store.compaction.is_empty());
    assert!(store.log_base() > 0);
    assert!(heap.1 > 0);
    let old = ToolRunTraceCursor { run: identity().id.run, generation: identity().generation, page: 0 };
    let delta = store.delta_after(Some(old), usize::MAX);
    assert!(delta.clear);
    assert_eq!(delta.identity, generation);
    assert_eq!(delta.pages.iter().flat_map(|page| page.ops.iter().copied()).collect::<Vec<_>>(), [
        ToolRunTraceOp::Clear, next[0].ops[0]
    ]);
    let closed = effect_harness::retire(&mut bank, store);
    assert_eq!(heap.0 + closed.0, heap.1 + closed.1);
}
