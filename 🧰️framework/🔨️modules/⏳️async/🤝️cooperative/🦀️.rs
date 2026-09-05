//#region 🧪️CooperativeMaintenance
use super::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct Fixture { cases: Vec<Case> }

#[derive(Deserialize)]
struct Case { lane: Lane, weight: u32, deficits: Vec<i64>, selected: Vec<bool> }

#[test]
fn cooperative_maintenance_retains_deficit_until_later_host_turn() {
    let fixture: Fixture = serde_json::from_str(include_str!("🧪️fixture/🔣️.json")).unwrap();
    for case in fixture.cases {
        let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::InteractiveNative, 2));
        let ran = Arc::new(AtomicU32::new(0));
        let captured = ran.clone();
        pool.submit(case.lane, Box::new(move || { captured.fetch_add(1, Ordering::SeqCst); }));
        assert_eq!(case.lane.weight(), case.weight);
        for (turn, selected) in case.selected.iter().enumerate() {
            assert_eq!(pool.pump(turn as u64), !selected);
            assert_eq!(ran.load(Ordering::SeqCst), u32::from(*selected));
            let snapshot = pool.try_cooperative_snapshot().unwrap();
            assert_eq!(snapshot.pump_calls, turn as u64 + 1);
            assert_eq!(snapshot.deficits[case.lane.index()], case.deficits[turn]);
            assert_eq!(snapshot.queued_by_lane[case.lane.index()], usize::from(!selected));
            assert_eq!(snapshot.selected_by_lane[case.lane.index()], u64::from(*selected));
            assert_eq!(snapshot.selections + snapshot.no_selection, snapshot.pump_calls);
        }
        assert!(!pool.has_pending_work());
        assert_eq!(pool.occupancy(), 0);
        pool.shutdown();
        eprintln!("[DEBUG] cooperative lane={:?} host_turns={} executions={}", case.lane, case.selected.len(), ran.load(Ordering::SeqCst));
    }
}

#[test]
fn cooperative_maintenance_snapshot_contention_preserves_queued_job() {
    let pool = WorkerPool::new(WorkerPoolConfig::new(ProcessKind::InteractiveNative, 2));
    let ran = Arc::new(AtomicU32::new(0));
    let captured = ran.clone();
    pool.submit(Lane::Maintenance, Box::new(move || { captured.fetch_add(1, Ordering::SeqCst); }));
    let held = pool.inner.state.lock().unwrap();
    assert!(pool.try_cooperative_snapshot().is_none());
    assert_eq!(held.queues[Lane::Maintenance.index()].len(), 1);
    drop(held);
    for turn in 0..8 { pool.pump(turn); }
    assert_eq!(ran.load(Ordering::SeqCst), 1);
    assert_eq!(pool.try_cooperative_snapshot().unwrap().selected_by_lane[Lane::Maintenance.index()], 1);
    pool.shutdown();
}

#[test]
fn cooperative_maintenance_live_host_revisits_queued_owner() {
    let source = include_str!("../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");
    assert!(exact_live_pump_binding(source), "queued maintenance must retain one actual host pump opportunity");
    for hostile in [
        source.replace("pump_runtime_live_cooperative_turn(&cell)?;", "other_pump(&cell)?;"),
        source.replace("pump_runtime_live_cooperative_turn(&cell)?;", ""),
        source.replace("pump_runtime_live_cooperative_turn(&cell)?;", "pump_runtime_live_cooperative_turn(&cell)?; pump_runtime_live_cooperative_turn(&cell)?;"),
        source.replace("pool.pump(now_ms);", "while pool.pump(now_ms) {}"),
        source.replace("let now_ms = semio_framework_job::default_now_ms();", "let now_ms = Some(0);"),
    ] {
        assert!(!exact_live_pump_binding(&hostile));
    }
}

fn exact_live_pump_binding(source: &str) -> bool {
    let Some(helper_start) = source.find("fn pump_runtime_live_cooperative_turn<") else { return false };
    if source.matches("fn pump_runtime_live_cooperative_turn<").count() != 1 { return false }
    let Some(start) = source.find("pub fn plugin_step_live_cleanup<") else { return false };
    if helper_start >= start { return false }
    let helper = &source[helper_start..start];
    if helper.matches("pool.pump(now_ms);").count() != 1 || !helper.contains("let now_ms = semio_framework_job::default_now_ms();") || ["while ", "loop {", "for "].iter().any(|pattern| helper.contains(pattern)) { return false }
    let source = &source[start..];
    let Some(start) = source.find("RUNTIME_MAINTENANCE_QUEUED | RUNTIME_MAINTENANCE_RUNNING =>") else { return false };
    let Some(length) = source[start..].find("_ => Err(") else { return false };
    let branch = &source[start..start + length];
    branch.matches("pump_runtime_live_cooperative_turn(&cell)").count() == 1 && !["while ", "loop {", "for "].iter().any(|pattern| branch.contains(pattern))
}
//#endregion 🧪️CooperativeMaintenance
