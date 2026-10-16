use super::*;

//#region 🧪️CanonicalRootLaws
fn document_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()
}
fn complete_document_job(current: SurfaceReconciler, name: &str, generation: u64) -> SurfaceReconciler {
    let mut job = SurfaceReconcileJob::try_new(current, tree(leaf(name)), generation).expect("real reconciliation admission");
    let mut sequence = 0;
    for _ in 0..100_000 {
        let mut actual_retained_progress=semio_framework_job::RetainedCloneProgress::default();
        let mut cx = semio_framework_job::StepContext::new(
            semio_framework_job::allocate_operation_id(),
            semio_framework_job::Generation(generation),
            semio_framework_job::StepBudget::new(1, u64::MAX,ui_contract::UI_WORKER_RETIREMENT_POLICY),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut sequence,
        &mut actual_retained_progress);
        match job.drive_one(cx.generation().0, &mut cx).expect("original reconcile native resource source") {
            SurfaceReconcileJobStep::MoreWork => {}
            SurfaceReconcileJobStep::Ready => {
                let (current, output) = job.take_ready().unwrap_or_else(|_| panic!("completed job retains its exact ready output"));
                if let Some(mut output) = output {
                    while !output.close_step() {}
                }
                return current;
            }
            SurfaceReconcileJobStep::Fault => panic!("real reconciliation fault: {:?}", job.fault()),
        }
    }
    panic!("real reconciliation did not finish")
}
#[test]
fn surface_canonical_document_nine_live_reconcilers_share_the_original_root_with_readers() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    let fixture = document_fixture();
    let mut owners = Vec::new();
    let mut readers = Vec::new();
    for index in 0..fixture["surfaces"].as_u64().unwrap() {
        let owner = complete_document_job(SurfaceReconciler::from_text(format!("canonical-{index}")), "root", 910_000 + index);
        let mut reader = None;
        assert!(owner.capture_document(&mut reader, fixture["physicalBytes"].as_u64().unwrap() as usize).unwrap());
        let reader = reader.unwrap();
        assert!(owner.shadow.document().unwrap().same_root(&reader));
        assert_eq!(reader.try_read().unwrap().len(), 1);
        owners.push(owner);
        readers.push(reader);
    }
    assert_eq!(owners.len(), 9);
    for owner in &mut owners {
        while !owner.retire_one() {}
    }
    for reader in &readers {
        assert_eq!(reader.try_read().unwrap().len(), 1);
    }
    for reader in &mut readers {
        while !reader.close_read_step_with_grant(1, 64).unwrap().complete {}
    }
}
#[test]
fn surface_canonical_document_old_reader_keeps_original_credit_during_replacement() {
    let _guard = crate::surface_reconcile_registry_test_guard();
    for grant in document_fixture()["readerReleaseGrants"].as_array().unwrap() {
        let grant = grant.as_u64().unwrap() as usize;
        let current = complete_document_job(SurfaceReconciler::from_text("canonical-reader-pressure"), "before", 920_000 + grant as u64);
        let mut reader = None;
        assert!(current.capture_document(&mut reader, 32768).unwrap());
        let mut reader = reader.unwrap();
        let before = serde_json::to_value(reader.try_read().unwrap().node_at(0).unwrap()).unwrap();
        let mut replacement = complete_document_job(current, "after", 930_000 + grant as u64);
        assert!(!replacement.shadow.document().unwrap().same_root(&reader));
        assert_eq!(serde_json::to_value(reader.try_read().unwrap().node_at(0).unwrap()).unwrap(), before);
        assert!(reader.try_read().unwrap().resident_limits().bytes > 0);
        while !replacement.retire_one() {}
        assert_eq!(serde_json::to_value(reader.try_read().unwrap().node_at(0).unwrap()).unwrap(), before);
        while !reader.close_read_step_with_grant(1, grant).unwrap().complete {}
    }
}
//#endregion 🧪️CanonicalRootLaws
