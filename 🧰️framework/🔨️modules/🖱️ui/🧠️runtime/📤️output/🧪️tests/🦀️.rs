use super::*;

//#region 🧪️OutputAdmission
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixture/🔣️.json")).unwrap() }

fn ready(surface: &str, generation: u64) -> (SurfaceReconciler, Option<SurfaceReconcileReadyPatch>) {
    let mut job = SurfaceReconcileJob::try_new(SurfaceReconciler::new(surface), tree(leaf("root")), generation).unwrap_or_else(|_| panic!("admitted output producer"));
    let mut sequence = 0;
    for _ in 0..100_000 {
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(generation), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence);
        match job.drive_one(&mut cx) {
            SurfaceReconcileJobStep::Ready => return job.take_ready().unwrap_or_else(|_| panic!("exact paired output")),
            SurfaceReconcileJobStep::MoreWork => {}
            SurfaceReconcileJobStep::Fault => panic!("output producer fault"),
        }
    }
    panic!("output producer never completed")
}

#[test]
fn surface_output_pool_reserves_before_producer_and_refuses_the_sixty_fifth() {
    let fixture = fixture();
    let mut queue = SurfaceReconcileOutputs::default();
    assert!(queue.try_reserve(1, 0).unwrap().is_none());
    let mut reservations = Vec::new();
    let mut invoked = 0;
    for generation in 1..=fixture["entrySlots"].as_u64().unwrap() {
        let reservation = queue.try_reserve(generation, 32768).unwrap().expect("one shared entry per producer");
        invoked += 1;
        reservations.push(reservation);
    }
    assert!(queue.try_reserve(65, 32768).unwrap().is_none());
    assert_eq!(invoked, fixture["saturatedInvocations"].as_u64().unwrap());
    for reservation in &mut reservations { while !reservation.close_step(1).unwrap().complete {} }
    while !queue.close_step(1, 4096).unwrap().complete {}
    assert!(queue.terminal_is_empty());
    eprintln!("[DEBUG] output-pool preproducer={invoked} extra=false entry-limit=64 independent-payload-quota=false");
}

#[test]
fn surface_output_pool_static_backing_joins_existing_ledger_once_without_a_root_slot() {
    let before = ui_contract::UiResidentPermit::snapshot().unwrap();
    let expected = std::mem::size_of::<LazyLock<Mutex<SurfaceReconcileHandbackRegistry>>>() + SurfaceReconcileOutputs::static_backing_bytes();
    let mut first = SurfaceReconcileOutputs::default();
    let mut second = SurfaceReconcileOutputs::default();
    assert!(first.try_reserve(1, 0).unwrap().is_none());
    assert_eq!(ui_contract::UiResidentPermit::snapshot().unwrap(), before);
    let mut a = first.try_reserve(1, 32768).unwrap().unwrap();
    let mut b = second.try_reserve(2, 32768).unwrap().unwrap();
    let live = ui_contract::UiResidentPermit::snapshot().unwrap();
    assert_eq!(live.bytes - before.bytes, expected);
    assert_eq!(live.used_slots, before.used_slots);
    while !a.close_step(1).unwrap().complete {}
    while !b.close_step(1).unwrap().complete {}
    while !first.close_step(1, 1).unwrap().complete {}
    while !second.close_step(1, 1).unwrap().complete {}
    assert_eq!(ui_contract::UiResidentPermit::snapshot().unwrap(), live);
    eprintln!("[DEBUG] output-pool static-ledger contract={} runtime={expected} total={} additional-root-slots=0 final-release-retains-static=true", before.bytes, live.bytes);
}

#[test]
fn surface_output_pool_keeps_exact_paired_ready_on_refusal_and_fifo_handoff() {
    for grant in fixture()["closeGrants"].as_array().unwrap() {
        let mut queue = SurfaceReconcileOutputs::default();
        let mut currents = Vec::new();
        for (index, surface) in fixture()["surfaces"].as_array().unwrap().iter().enumerate() {
            let mut reservation = queue.try_reserve(700_000 + index as u64, 32768).unwrap();
            assert!(reservation.is_some());
            let (current, mut source) = ready(surface.as_str().unwrap(), 700_000 + index as u64);
            let pointer = source.as_ref().unwrap().patch.get().unwrap().ops.get(0).unwrap() as *const _;
            assert!(!queue.put(&mut reservation, &mut source, 0).unwrap());
            assert_eq!(source.as_ref().unwrap().patch.get().unwrap().ops.get(0).unwrap() as *const _, pointer);
            assert!(queue.put(&mut reservation, &mut source, 32768).unwrap());
            assert!(reservation.is_none() && source.is_none());
            currents.push(current);
        }
        for surface in fixture()["surfaces"].as_array().unwrap() {
            let mut target = None;
            assert!(!queue.take_front_into(&mut target, 0).unwrap());
            assert!(queue.take_front_into(&mut target, 32768).unwrap());
            assert_eq!(target.as_ref().unwrap().surface().unwrap().0.as_str(), surface.as_str().unwrap());
            let before = target.as_ref().unwrap().generation();
            assert!(!queue.take_front_into(&mut target, 32768).unwrap());
            assert_eq!(target.as_ref().unwrap().generation(), before);
            let mut target = target.unwrap();
            while !target.close_step_with_grant(1, grant.as_u64().unwrap() as usize).unwrap().complete {}
        }
        while !queue.close_step(1, grant.as_u64().unwrap() as usize).unwrap().complete {}
        for current in &mut currents { while !current.retire_one() {} }
        assert!(queue.terminal_is_empty());
    }
    eprintln!("[DEBUG] output-pool fifo=2 exact-rejected-pointer=true paired-credit=true close-grants=1,64,4096");
}
//#endregion 🧪️OutputAdmission

//#region 🧪️HandbackAdmission
#[test]
fn surface_output_admission_refuses_before_producer_when_only_one_handback_is_free() {
    let law = &fixture()["handbackAdmission"];
    assert_eq!(SURFACE_RECONCILE_HANDBACK_SLOTS, law["slots"].as_u64().unwrap() as usize);
    let mut occupied = Vec::new();
    for _ in 1..SURFACE_RECONCILE_HANDBACK_SLOTS { occupied.push(reserve_surface_reconcile_handback(910_001).unwrap()); }
    let reservation = SurfaceReconcileReservation::try_new(910_002);
    let accepted = reservation.is_some();
    drop(reservation);
    for owner in occupied { release_surface_reconcile_handback(owner); }
    assert_eq!(accepted, law["onlyOneFreeAccepted"].as_bool().unwrap());
    eprintln!("[DEBUG] handback-admission one-free-accepted={accepted} producer-invoked=false");
}

#[test]
fn surface_output_admission_transfers_after_seal_with_no_unreserved_handback() {
    let reservation = SurfaceReconcileReservation::try_new(920_001).unwrap();
    let mut job = SurfaceReconcileJob::try_new_reserved(SurfaceReconciler::new("retained-é"), tree(leaf("root")), reservation).unwrap();
    let mut sequence = 0;
    for _ in 0..100_000 {
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(920_001), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence);
        match job.drive_one(&mut cx) { SurfaceReconcileJobStep::Ready => break, SurfaceReconcileJobStep::MoreWork => {}, SurfaceReconcileJobStep::Fault => panic!("admitted job fault: {:?}", job.fault()) }
    }
    let mut occupied = Vec::new();
    while let Some(owner) = reserve_surface_reconcile_handback(920_002) { occupied.push(owner); }
    let output = job.take_ready();
    let transferred = output.is_ok();
    for owner in occupied { release_surface_reconcile_handback(owner); }
    match output {
        Ok((mut current, ready)) => {
            if let Some(mut ready) = ready { while !ready.close_step_with_grant(1, 4096).unwrap().complete {} }
            while !current.retire_one() {}
        }
        Err(job) => { let mut terminal = job.into_terminal(); while !terminal.close_step() {} }
    }
    assert_eq!(transferred, fixture()["handbackAdmission"]["saturatedAfterSealTransfers"].as_bool().unwrap());
    eprintln!("[DEBUG] handback-admission post-seal-transfer={transferred} late-slot-acquisition=false");
}
//#endregion 🧪️HandbackAdmission

//#region 🧪️StructuralReadyTransfer
#[test]
fn surface_output_admission_inplace_transfer_retains_source_on_refusal_and_targets_on_unwind() {
    let mut job = SurfaceReconcileJob::try_new(SurfaceReconciler::new("in-place-é"), tree(leaf("root")), 930_001).unwrap();
    let mut sequence = 0;
    for _ in 0..100_000 {
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(930_001), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence);
        match job.drive_one(&mut cx) { SurfaceReconcileJobStep::Ready => break, SurfaceReconcileJobStep::MoreWork => {}, SurfaceReconcileJobStep::Fault => panic!("admitted job fault: {:?}", job.fault()) }
    }
    let shell = job.state.as_ref().unwrap().as_ref() as *const _;
    let payload = job.state.as_ref().unwrap().patch.get().unwrap().ops.get(0).unwrap() as *const _;
    let original_handback = job.handback_key().unwrap();
    let bytes = SurfaceReconcileJob::required_ready_transfer_bytes();
    assert!(bytes <= fixture()["physicalGrant"].as_u64().unwrap() as usize);
    let mut current = None;
    let mut ready = None;
    assert!(!job.take_ready_into(&mut current, &mut ready, bytes - 1).unwrap());
    assert_eq!(job.state.as_ref().unwrap().patch.get().unwrap().ops.get(0).unwrap() as *const _, payload);
    current = Some(SurfaceReconciler::new("occupied"));
    assert!(!job.take_ready_into(&mut current, &mut ready, bytes).unwrap());
    while !current.as_mut().unwrap().retire_one() {}
    current = None;
    let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert!(job.take_ready_into(&mut current, &mut ready, bytes).unwrap());
        panic!("[DEBUG] actual ready transfer callback unwind");
    }));
    assert!(caught.is_err());
    assert_eq!(job.state.as_ref().unwrap().as_ref() as *const _, shell);
    assert_eq!(ready.as_ref().unwrap().patch.get().unwrap().ops.get(0).unwrap() as *const _, payload);
    assert_eq!(current.as_ref().unwrap().handback.as_ref().unwrap().key, original_handback);
    assert!(!job.take_ready_into(&mut current, &mut ready, bytes).unwrap());
    let mut terminal = job.into_terminal();
    while !terminal.close_step() {}
    while !ready.as_mut().unwrap().close_step_with_grant(1, 4096).unwrap().complete {}
    while !current.as_mut().unwrap().retire_one() {}
    eprintln!("[DEBUG] ready-transfer bytes={bytes} shell-preserved=true refused-payload-preserved=true unwind-targets-retained=true");
}
//#endregion 🧪️StructuralReadyTransfer

//#region 🧪️ReadyRevalidation
#[test]
fn surface_output_admission_ready_rechecks_cancel_generation_fuel_and_deadline_before_transfer() {
    fn now() -> Option<u64> { Some(5) }
    let mut observed = Vec::new();
    let law = fixture();
    for row in law["readyRevalidation"].as_array().unwrap() {
        let mut job = SurfaceReconcileJob::try_new(SurfaceReconciler::new("ready-revalidation"), tree(leaf("root")), 940_001).unwrap();
        let mut sequence = 0;
        for _ in 0..100_000 {
            let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(940_001), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), now, &mut sequence);
            match job.drive_one(&mut cx) { SurfaceReconcileJobStep::Ready => break, SurfaceReconcileJobStep::MoreWork => {}, SurfaceReconcileJobStep::Fault => panic!("admitted job fault") }
        }
        let payload = job.state.as_ref().unwrap().patch.get().unwrap().ops.get(0).unwrap() as *const _;
        let cancel = semio_framework_job::root_cancel_token();
        if row["cancelled"].as_bool().unwrap() { cancel.cancel_now(); }
        let generation = 940_001 + u64::from(!row["sameGeneration"].as_bool().unwrap());
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(generation), semio_framework_job::StepBudget::new(row["fuel"].as_u64().unwrap(), row["deadline"].as_u64().unwrap()), cancel, now, &mut sequence);
        let outcome = match job.drive_one(&mut cx) { SurfaceReconcileJobStep::Ready => "ready", SurfaceReconcileJobStep::MoreWork => "pending", SurfaceReconcileJobStep::Fault => "fault" };
        assert_eq!(job.state.as_ref().unwrap().patch.get().unwrap().ops.get(0).unwrap() as *const _, payload);
        observed.push(outcome);
        let mut terminal = job.into_terminal();
        while !terminal.close_step() {}
    }
    assert_eq!(observed, law["readyRevalidation"].as_array().unwrap().iter().map(|row| row["outcome"].as_str().unwrap()).collect::<Vec<_>>());
    eprintln!("[DEBUG] ready-revalidation actual={observed:?} exact-source-preserved=true");
}
//#endregion 🧪️ReadyRevalidation

//#region 🧪️DirectPoolReceiver
#[test]
fn surface_output_pool_direct_job_receiver_keeps_exact_roots_across_refusal_and_callback_unwind() {
    let law = fixture();
    let generation = 950_001;
    let mut outputs = SurfaceReconcileOutputs::default();
    let mut reservation = outputs.try_reserve(generation, 32768).unwrap();
    let mut job = SurfaceReconcileJob::try_new(SurfaceReconciler::new("direct-pool-é"), tree(leaf("root")), generation).unwrap();
    let mut sequence = 0;
    for _ in 0..100_000 {
        let mut cx = semio_framework_job::StepContext::new(semio_framework_job::allocate_operation_id(), semio_framework_job::Generation(generation), semio_framework_job::StepBudget::new(1, u64::MAX), semio_framework_job::root_cancel_token(), semio_framework_job::default_now_us, &mut sequence);
        match job.drive_one(&mut cx) { SurfaceReconcileJobStep::Ready => break, SurfaceReconcileJobStep::MoreWork => {}, SurfaceReconcileJobStep::Fault => panic!("admitted job fault") }
    }
    let payload = job.state.as_ref().unwrap().patch.get().unwrap().ops.get(0).unwrap() as *const _;
    let shell = job.state.as_ref().unwrap().as_ref() as *const _;
    let bytes = SurfaceReconcileOutputs::required_job_transfer_bytes();
    assert!(bytes <= law["physicalGrant"].as_u64().unwrap() as usize);
    let mut current = None;
    assert_eq!(outputs.receive_job_into(&mut reservation, &mut job, &mut current, bytes - 1).unwrap(), SurfaceReconcileOutputTransfer::Pending);
    assert_eq!(job.state.as_ref().unwrap().patch.get().unwrap().ops.get(0).unwrap() as *const _, payload);
    current = Some(SurfaceReconciler::new("occupied"));
    assert_eq!(outputs.receive_job_into(&mut reservation, &mut job, &mut current, bytes).unwrap(), SurfaceReconcileOutputTransfer::Pending);
    while !current.as_mut().unwrap().retire_one() {}
    current = None;
    let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        assert_eq!(outputs.receive_job_into(&mut reservation, &mut job, &mut current, bytes).unwrap(), SurfaceReconcileOutputTransfer::Published);
        panic!("[DEBUG] direct pool receiver callback after actual transfer");
    }));
    assert!(caught.is_err());
    assert!(reservation.is_none());
    assert_eq!(job.state.as_ref().unwrap().as_ref() as *const _, shell);
    let mut ready = None;
    assert!(!outputs.take_front_into(&mut ready, 0).unwrap());
    assert!(outputs.take_front_into(&mut ready, 32768).unwrap());
    assert_eq!(ready.as_ref().unwrap().patch.get().unwrap().ops.get(0).unwrap() as *const _, payload);
    assert_eq!(ready.as_ref().unwrap().generation(), generation);
    while !ready.as_mut().unwrap().close_step_with_grant(1, 4096).unwrap().complete {}
    while !outputs.close_step(1, 4096).unwrap().complete {}
    let mut terminal = job.into_terminal();
    while !terminal.close_step() {}
    while !current.as_mut().unwrap().retire_one() {}
    assert_eq!(outputs.terminal_is_empty(), law["directReceiver"]["terminal"].as_bool().unwrap());
    eprintln!("[DEBUG] direct-pool receiver-bytes={bytes} original-payload=true original-shell=true callback-unwind-retained=true");
}
//#endregion 🧪️DirectPoolReceiver
