
use super::*;
use crate::calendar::RunPeriod;

fn build_checkpoint_packet(job: &mut EnergyJob) -> EnergyWirePacket {
    let operation = job.operation;
    let hour_index = job.hour_index as u64;
    job.start_wire(EnergyWireKind::Checkpoint, hour_index, semio_framework_job::JobPayloadStream::CheckpointState).expect("checkpoint preflight");
    let mut sequence = 0;
    for _ in 0..32 {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
        if job.step_wire_build(&mut context).expect("checkpoint fragment") {
            return job.wire_ready.take().expect("sealed checkpoint packet");
        }
    }
    panic!("checkpoint packet did not seal within fixed field bound")
}

fn retained_test_packet(operation: Operation, identity: EnergyWireIdentity, bytes: &[u8]) -> EnergyWirePacket {
    let mut writer = semio_framework_job::RetainedJobPayloadWriter::new(semio_framework_job::JobPayloadStream::CheckpointState);
    let mut sequence = 0;
    let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
    let mut page = writer.admit_page(&mut context).expect("test page admission");
    page.write(bytes).expect("single fixed checkpoint page");
    page.commit();
    EnergyWirePacket { kind: EnergyWireKind::Checkpoint, identity, payload: writer.finish().expect("sealed test packet"), preview: None, reservation: None }
}

#[test]
fn p7c2_wire_schema_maximum_plus_one_and_exact_restore_owner() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(91), Generation(7), 0x55aa);
    let mut source = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("source admission");
    let packet = build_checkpoint_packet(&mut source);
    assert_eq!(packet.payload.len(), ENERGY_CHECKPOINT_BYTES);
    assert_eq!(packet.payload.page_count(), 1);
    let model = test_model_single_zone();
    let config = SimulationConfig::default();
    let pointer = model.zones.as_ptr();
    let census = EnergyNumericalCensus::observe(&model, &config).expect("restore census");
    let mut maximum = EnergyNumericalBounds::default().0;
    maximum.zones = census.zones - 1;
    let rejected = EnergyRestoreJob::admit(operation, model, config, packet, EnergyNumericalBounds(maximum)).expect_err("zone MAX+1 rejects before restore mount");
    assert_eq!(rejected.model.zones.as_ptr(), pointer);
    assert_eq!(rejected.reason, EnergyCheckpointRejectionReason::Numerical(EnergyNumericalDimension::Zones));
    let restore = rejected.retry(EnergyNumericalBounds::default()).expect("exact packet and Model+Config retry");
    drop(restore);
    let mut recovered = EnergyRestoreJob::recover_abandoned(operation).expect("drop requeues exact restore authority");
    let mut sequence = 0;
    for _ in 0..4 {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
        let _ = recovered.step(&mut context).expect("field decode");
    }
    drop(recovered);
    assert!(EnergyRestoreJob::recover_abandoned(Operation { generation: Generation(8), ..operation }).is_none());
    let mut recovered = EnergyRestoreJob::recover_abandoned(operation).expect("same generation recovers once");
    for _ in 0..32 {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
        if recovered.step(&mut context).expect("restore rebuild") {
            break;
        }
    }
    let install_context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
    let mut restored = recovered.finish(&install_context).expect("restored authority");
    InteractiveJob::begin_close(&mut restored);
    for _ in 0..100_000 {
        if matches!(InteractiveJob::close_step(&mut restored, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
            break;
        }
    }
    InteractiveJob::begin_close(&mut source);
    for _ in 0..100_000 {
        if matches!(InteractiveJob::close_step(&mut source, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
            return;
        }
    }
    panic!("source wire authority did not close")
}

#[test]
fn p7c2_live_schema_mutations_reject_before_restore_mount() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(94), Generation(12), 0xdead);
    let mut source = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("source admission");
    let packet = build_checkpoint_packet(&mut source);
    let identity = packet.identity;
    let baseline = payload_bytes(packet.payload);
    let mutations: [fn(&mut Vec<u8>); 8] = [
        |bytes| bytes[0] ^= 0xff,
        |bytes| bytes[8] = bytes[8].wrapping_add(1),
        |bytes| bytes[10] = 0xff,
        |bytes| bytes[32] = bytes[32].wrapping_add(1),
        |bytes| bytes[56] = bytes[56].wrapping_add(1),
        |bytes| bytes[64] = bytes[64].wrapping_add(1),
        |bytes| bytes[72] = 2,
        |bytes| bytes[76] = 2,
    ];
    for mutate in mutations {
        let mut bytes = baseline.clone();
        mutate(&mut bytes);
        let packet = retained_test_packet(operation, identity, &bytes);
        let mut rejected = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect_err("hostile header/cap mutation");
        while !rejected.packet.terminal_is_empty() {
            let _ = rejected.packet.ack_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        }
    }
    let mut trailing = baseline.clone();
    trailing.push(0);
    let packet = retained_test_packet(operation, identity, &trailing);
    let mut rejected = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect_err("trailing byte mutation");
    while !rejected.packet.terminal_is_empty() {
        let _ = rejected.packet.ack_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    let mut ignored_digest = baseline;
    ignored_digest[156] ^= 0x80;
    let packet = retained_test_packet(operation, identity, &ignored_digest);
    let mut restore = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect("digest mutation passes bounded header admission");
    let mut sequence = 0;
    for _ in 0..64 {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
        assert!(!restore.step(&mut context).expect("digest mutation remains retained while replay proves mismatch"));
    }
    assert!(!restore.ready, "decoded numerical digest cannot be ignored by fresh-job restore");
    for offset in [132usize, 140, 148] {
        let mut ignored_count = ignored_digest.clone();
        ignored_count[156] ^= 0x80;
        ignored_count[offset] = 1;
        let packet = retained_test_packet(operation, identity, &ignored_count);
        let mut restore = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect("under-cap decoded count mutation passes bounded header admission");
        for _ in 0..64 {
            let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
            assert!(!restore.step(&mut context).expect("decoded count remains retained while replay proves mismatch"));
        }
        assert!(!restore.ready, "decoded table/history count cannot be cap-checked then discarded");
    }
    InteractiveJob::begin_close(&mut source);
    for _ in 0..100_000 {
        if matches!(InteractiveJob::close_step(&mut source, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
            return;
        }
    }
    panic!("schema mutation source did not close")
}

#[test]
fn p7c2_lossless_queue_saturation_retains_identity_and_fifo_order() {
    let mut queue = EnergyWireQueue::new(EnergyWireKind::Checkpoint);
    for sequence in 0..ENERGY_WIRE_QUEUE_SLOTS as u64 {
        let identity = EnergyWireIdentity { operation: 1, base_revision: 2, generation: 3, seed: 4, sequence };
        queue
            .push(EnergyWirePacket { kind: EnergyWireKind::Checkpoint, identity, payload: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState), preview: None, reservation: None })
            .expect("exact queue maximum");
    }
    let overflow_identity = EnergyWireIdentity { operation: 1, base_revision: 2, generation: 3, seed: 4, sequence: ENERGY_WIRE_QUEUE_SLOTS as u64 };
    let rejected = queue
        .push(EnergyWirePacket { kind: EnergyWireKind::Checkpoint, identity: overflow_identity, payload: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState), preview: None, reservation: None })
        .expect_err("queue MAX+1 retains packet");
    assert_eq!(rejected.identity, overflow_identity);
    let first = queue.take().expect("first lease");
    assert_eq!(first.identity().sequence, 0);
    queue.retry(first).expect("retry exact head");
    for sequence in 0..ENERGY_WIRE_QUEUE_SLOTS as u64 {
        let lease = queue.take().expect("FIFO lease");
        assert_eq!(lease.identity().sequence, sequence);
        queue.ack(lease).expect("empty packet ACK");
    }
}

#[test]
fn p7c2_lossless_queue_drop_and_panic_recover_exact_in_flight_head() {
    let identity = EnergyWireIdentity { operation: 11, base_revision: 12, generation: 13, seed: 14, sequence: 15 };
    let mut queue = EnergyWireQueue::new(EnergyWireKind::Checkpoint);
    queue
        .push(EnergyWirePacket { kind: EnergyWireKind::Checkpoint, identity, payload: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CheckpointState), preview: None, reservation: None })
        .expect("fixed queue accepts head");
    drop(queue.take().expect("drop lease"));
    let recovered = queue.take().expect("Drop republishes exact in-flight head");
    assert_eq!(recovered.identity(), identity);
    queue.retry(recovered).expect("retry returns exact head to original slot");
    let panic_lease = queue.take().expect("panic lease");
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let _owned = panic_lease;
        panic!("hostile consumer panic");
    }));
    let recovered = queue.take().expect("panic Drop republishes exact head");
    assert_eq!(recovered.identity(), identity);
    queue.ack(recovered).expect("empty recovered packet ACKs once");
    assert_eq!(queue.len, 0);
}

#[test]
fn p7c3_commit_lease_ack_is_the_exact_terminal_detach_witness() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(121), Generation(17), 0x7c3);
    let mut job = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("terminal witness admission");
    job.stage = EnergyJobStage::Complete;
    let identity = job.wire_identity(1);
    job.publication
        .commits
        .push(EnergyWirePacket { kind: EnergyWireKind::Commit, identity, payload: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput), preview: None, reservation: None })
        .expect("prepared exact commit");

    let lease = job.take_commit_packet(operation.generation).expect("fresh generation").expect("exact final lease");
    let mut sequence = 0;
    let mut leased = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
    assert_eq!(job.step(&mut leased), StepOutcome::Yield, "an unacknowledged exact lease must retain terminal ownership");
    assert_eq!(job.publication.commits.len, 1);
    assert!(job.publication.commits.in_flight.is_some());

    job.ack_commit_packet(lease).expect("exact empty commit ACK");
    assert_eq!(job.publication.commits.len, 0);
    assert!(job.publication.commits.in_flight.is_none());
    let mut acknowledged = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
    let mut terminal = job.step(&mut acknowledged);
    assert!(matches!(terminal, StepOutcome::Complete(_)), "the exact ACK must make the numerical authority terminal");
    assert!(terminal.terminal_is_empty(), "the consumer retained and closed the only commit owner before terminal detach");
    assert!(matches!(terminal.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete));

    InteractiveJob::begin_close(&mut job);
    for _ in 0..100_000 {
        if matches!(InteractiveJob::close_step(&mut job, 1, 4), semio_framework_job::InteractiveJobCloseStep::Complete) {
            return;
        }
    }
    panic!("ACK-terminal Energy authority did not close")
}

#[test]
fn p7c2_preview_typed_view_is_derived_from_canonical_wire_with_live_facility_total() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(96), Generation(15), 0xcafe);
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let mut job = EnergyJob::new(operation, test_model_single_zone(), config).expect("preview source admission");
    let mut sequence = 0;
    for _ in 0..50_000 {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
        match job.step(&mut context) {
            StepOutcome::PreviewReady(mut notice) => {
                close_retained_payload(&mut notice);
                let projected = job.preview().expect("installed packet owns typed preview projection").clone();
                let mut packet = job.take_preview_packet(operation.generation).expect("fresh preview generation").expect("canonical preview packet");
                assert_eq!(packet.payload.len(), ENERGY_WIRE_HEADER_BYTES + 20);
                let decoded = decode_preview_packet(&packet).expect("SMENERGY preview schema");
                assert_eq!(packet.preview(), Some(&decoded));
                assert_eq!(projected, decoded);
                close_retained_payload(&mut packet.payload);
                if decoded.facility_electricity_kwh > 0.0 {
                    return;
                }
            }
            StepOutcome::CheckpointReady(mut checkpoint) => {
                close_retained_payload(&mut checkpoint.state);
                let mut lease = job.take_checkpoint_packet(operation.generation).expect("checkpoint generation").expect("checkpoint lease");
                close_retained_payload(&mut lease.packet_mut().payload);
                job.ack_checkpoint_packet(lease).expect("checkpoint ACK");
            }
            StepOutcome::Yield => {}
            StepOutcome::Fault(fault) => panic!("preview source faulted: {fault:?}"),
            StepOutcome::Cancelled => panic!("preview source cancelled"),
            StepOutcome::Complete(_) => break,
        }
    }
    panic!("canonical preview never exposed a substantive retained facility total")
}

#[test]
fn p7c2_restore_stale_step_and_install_preserve_exact_replay_authority() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(95), Generation(14), 0xbeef);
    let mut source = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("source admission");
    let packet = build_checkpoint_packet(&mut source);
    let mut restore = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect("restore admission");
    let mut sequence = 0;
    let mut stale = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
    assert_eq!(restore.step(&mut stale), Err(EnergyWireRejection::Identity));
    assert_eq!(restore.field, 0);
    for _ in 0..64 {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
        if restore.step(&mut context).expect("bounded replay step") {
            break;
        }
    }
    assert!(restore.ready);
    let stale_install = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
    let restore = restore.finish(&stale_install).expect_err("stale install retains exact replay authority");
    let install = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
    let mut restored = restore.finish(&install).expect("fresh generation installs exact replay authority");
    InteractiveJob::begin_close(&mut restored);
    for _ in 0..100_000 {
        if matches!(InteractiveJob::close_step(&mut restored, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
            break;
        }
    }
    InteractiveJob::begin_close(&mut source);
    for _ in 0..100_000 {
        if matches!(InteractiveJob::close_step(&mut source, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
            return;
        }
    }
    panic!("stale restore fixture owners did not close")
}

#[test]
fn p7c2_cancel_deadline_and_stale_generation_gate_wire_before_mutation() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(92), Generation(5), 99);
    let mut job = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("admission");
    job.start_wire(EnergyWireKind::Checkpoint, 0, semio_framework_job::JobPayloadStream::CheckpointState).expect("wire start");
    let before = job.wire_build.as_ref().expect("wire").field;
    let cancel = CancelToken::root_now();
    cancel.cancel_now();
    let mut sequence = 0;
    let mut cancelled = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel, default_now_us, &mut sequence);
    assert_eq!(job.step(&mut cancelled), StepOutcome::Cancelled);
    assert_eq!(job.wire_build.as_ref().expect("wire").field, before);
    let mut deadline = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(0, 0), CancelToken::root_now(), default_now_us, &mut sequence);
    assert_eq!(job.step(&mut deadline), StepOutcome::Yield);
    assert_eq!(job.wire_build.as_ref().expect("wire").field, before);
    assert!(matches!(job.take_checkpoint_packet(Generation(6)), Err(EnergyWireRejection::Identity)));
}

#[test]
fn p7c2_restored_commit_bytes_match_one_and_four_fuel_chronology() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let operation = Operation::new(allocate_operation_id(), RevisionId(93), Generation(9), 777);
    let mut original = EnergyJob::new(operation, model.clone(), config.clone()).expect("original admission");
    let mut sequence = 0;
    let packet = loop {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
        match original.step(&mut context) {
            StepOutcome::PreviewReady(mut notice) => {
                close_retained_payload(&mut notice);
                let mut packet = original.take_preview_packet(operation.generation).expect("fresh preview").expect("preview packet");
                close_retained_payload(&mut packet.payload);
            }
            StepOutcome::CheckpointReady(mut checkpoint) => {
                close_retained_payload(&mut checkpoint.state);
                let packet = build_checkpoint_packet(&mut original);
                let mut lease = original.take_checkpoint_packet(operation.generation).expect("fresh checkpoint").expect("checkpoint lease");
                close_retained_payload(&mut lease.packet_mut().payload);
                original.ack_checkpoint_packet(lease).expect("checkpoint ACK advances only after close");
                break packet;
            }
            StepOutcome::Yield => {}
            other => panic!("checkpoint expected before terminal: {other:?}"),
        }
    };
    let mut restore = EnergyRestoreJob::admit(operation, model, config, packet, EnergyNumericalBounds::default()).expect("restore admission");
    for _ in 0..100_000 {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
        if restore.step(&mut context).expect("restore field/rebuild") {
            break;
        }
    }
    let install_context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut sequence);
    let restored = restore.finish(&install_context).expect("restore finish");
    let (_, _, _, original_bytes, _) = drive_energy_job_with_fuel(original, 1);
    let (_, _, _, restored_bytes, _) = drive_energy_job_with_fuel(restored, 4);
    assert_eq!(restored_bytes, original_bytes);
}

fn drive_energy_job(job: EnergyJob) -> (EnergyJob, Vec<EnergyJobPreview>, usize, Vec<u8>, std::time::Duration) {
    drive_energy_job_with_fuel(job, 1)
}

fn drive_energy_job_with_fuel(mut job: EnergyJob, fuel: u64) -> (EnergyJob, Vec<EnergyJobPreview>, usize, Vec<u8>, std::time::Duration) {
    let operation = job.operation.operation;
    let generation = job.operation.generation;
    let cancel = CancelToken::root_now();
    let mut preview_sequence = 0;
    let mut previews = Vec::new();
    let mut checkpoints = 0;
    let mut worst = std::time::Duration::ZERO;
    for _ in 0..50_000 {
        let start = Instant::now();
        let mut context = StepContext::new(operation, generation, semio_framework_job::StepBudget::new(fuel, u64::MAX), cancel.clone(), default_now_us, &mut preview_sequence);
        let outcome = job.step(&mut context);
        let elapsed = start.elapsed();
        worst = worst.max(elapsed);
        match outcome {
            StepOutcome::PreviewReady(mut notice) => {
                close_retained_payload(&mut notice);
                let mut packet = job.take_preview_packet(generation).expect("fresh preview generation").expect("preview packet");
                previews.push(packet.preview().expect("typed projection belongs to canonical preview packet").clone());
                close_retained_payload(&mut packet.payload);
            }
            StepOutcome::CheckpointReady(mut checkpoint) => {
                close_retained_payload(&mut checkpoint.state);
                let mut lease = job.take_checkpoint_packet(generation).expect("fresh checkpoint generation").expect("checkpoint lease");
                close_retained_payload(&mut lease.packet_mut().payload);
                job.ack_checkpoint_packet(lease).expect("checkpoint ACK");
                checkpoints += 1;
            }
            StepOutcome::Complete(candidate) => return (job, previews, checkpoints, payload_bytes(candidate.output), worst),
            StepOutcome::Fault(fault) => panic!("energy job faulted: {fault:?}"),
            StepOutcome::Cancelled => panic!("energy job unexpectedly cancelled"),
            StepOutcome::Yield => {}
        }
    }
    panic!("energy job did not complete within the deterministic step bound")
}

fn payload_bytes(mut payload: semio_framework_job::RetainedJobPayload) -> Vec<u8> {
    let mut bytes = Vec::new();
    for index in 0..payload.page_count() {
        bytes.extend_from_slice(payload.page(index).expect("retained output page"));
    }
    while !payload.terminal_is_empty() {
        let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    bytes
}

#[test]
fn engine_runs_single_zone() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 1, run_period_end_month: 1, run_period_end_day: 3, environment: SimulationEnvironment::WeatherRunPeriod, ..Default::default() };
    let results = Engine::run(model, config).unwrap();
    assert!(results.run_metadata.timesteps > 0);
    assert!(results.meters.facility_total_kwh(FuelType::Electricity) >= 0.0);
}

#[test]
fn engine_deterministic_repeatability() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
    let r1 = Engine::run(model.clone(), config.clone()).unwrap();
    let r2 = Engine::run(model, config).unwrap();
    assert_eq!(r1.run_metadata.timesteps, r2.run_metadata.timesteps);
    assert!((r1.meters.facility_total_kwh(FuelType::Electricity) - r2.meters.facility_total_kwh(FuelType::Electricity)).abs() < 1e-3);
}

#[test]
fn ashrae_140_case600_base() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let results = Engine::run(model, config).unwrap();
    let temps = results.time_series.get("Zone Air Temperature [Zone1]");
    assert!(temps.is_some());
}

#[test]
fn invalid_model_rejected() {
    let model = Model::default();
    assert!(Engine::run(model, SimulationConfig::default()).is_err());
}

#[test]
fn energy_conservation_order_of_magnitude() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
    let results = Engine::run(model, config).unwrap();
    let total_kwh = results.meters.facility_total_kwh(FuelType::Electricity);
    assert!(total_kwh < 1_000_000.0);
}

#[test]
fn full_topology_e2e() {
    let model = test_model_full_topology();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
    let results = Engine::run(model, config).unwrap();
    assert!(results.run_metadata.timesteps >= 48);
    assert!(results.summaries.annual_energy.len() >= 3);
}

#[test]
fn hvac_bestest_heating_day() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let results = Engine::run(model, config).unwrap();
    assert_eq!(results.run_metadata.timesteps, 24);
    assert!(results.time_series.get("Zone Air Temperature [Zone1]").is_some());
}

#[test]
fn run_period_honors_calendar() {
    let period = RunPeriod { start_month: 1, start_day: 1, end_month: 1, end_day: 7, year: 2026 };
    assert_eq!(period.total_hours(), 168);
    let config = SimulationConfig { run_period_start_month: 1, run_period_start_day: 1, run_period_end_month: 1, run_period_end_day: 7, warmup_days: 0, ..Default::default() };
    let model = test_model_single_zone();
    let results = Engine::run(model, config).unwrap();
    assert_eq!(results.run_metadata.timesteps, 168);
}

#[test]
fn energy_job_previews_checkpoints_and_commits_bounded_steps() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 1, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let (mut job, previews, checkpoints, output, worst) = drive_energy_job(Engine::job(model, config).expect("energy admission"));
    assert!(!previews.is_empty());
    assert!(previews.windows(2).all(|pair| pair[0].sequence < pair[1].sequence));
    assert!(previews.iter().any(|preview| preview.tier == EnergyQualityTier::SteadyStateEstimate));
    assert_eq!(previews.last().map(|preview| preview.tier), Some(EnergyQualityTier::Final));
    assert!(checkpoints > 0, "P7c2 publishes retained lossless checkpoints");
    assert!(output.starts_with(&ENERGY_WIRE_MAGIC));
    assert!(worst < std::time::Duration::from_millis(8), "worst energy step was {worst:?}");
    let results = job.take_results().expect("completed job retains typed results for the batch adapter");
    assert_eq!(results.run_metadata.timesteps, 24);
}

#[test]
fn energy_job_cancellation_and_freshness_precede_mutation() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, ..Default::default() };
    let operation = Operation::new(allocate_operation_id(), RevisionId(7), Generation(3), 11);
    let mut cancelled_job = EnergyJob::new(operation, model.clone(), config.clone()).expect("energy admission");
    let cancel = CancelToken::root_now();
    cancel.cancel_now();
    let mut sequence = 0;
    let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel, default_now_us, &mut sequence);
    assert_eq!(cancelled_job.step(&mut context), StepOutcome::Cancelled);
    assert_eq!(cancelled_job.stage(), EnergyJobStage::Validate);

    let mut stale_job = EnergyJob::new(operation, model, config).expect("energy admission");
    let mut stale_sequence = 0;
    let mut stale_context = StepContext::new(operation.operation, Generation(4), semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut stale_sequence);
    assert!(matches!(stale_job.step(&mut stale_context), StepOutcome::Fault(_)));
    assert_eq!(stale_job.stage(), EnergyJobStage::Validate);
}

#[test]
fn adversarial_timestep_work_unit_stays_below_watchdog() {
    let mut model = test_model_single_zone();
    let template = model.surfaces[0].clone();
    model.surfaces = (0..16_384)
        .map(|index| {
            let mut surface = template.clone();
            surface.id = crate::model::EntityId(1_000 + index);
            surface
        })
        .collect();
    let pre = PrecomputedModel::build(&model, 60, 15);
    let weather = design_day_hour(12, 35.0);
    let date = crate::calendar::SimDate::new(weather.year, weather.month, weather.day);
    let mut state = SimulationKernel::initialize(&model, &pre, &weather);
    let mut work = TimestepWork::new(&model, &pre, weather, date, 12.0, pre.zone_timestep_s);
    let start = Instant::now();
    work.step(&model, &SimulationConfig::default(), &pre, &mut state);
    assert!(start.elapsed() < std::time::Duration::from_millis(8), "one adversarial energy work unit exceeded watchdog: {:?}", start.elapsed());
}

#[test]
fn p7c1_exact_max_rejection_preserves_and_retries_same_owner() {
    let model = test_model_full_topology();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let census = EnergyNumericalCensus::observe(&model, &config).expect("checked census");
    let operation = Operation::new(allocate_operation_id(), RevisionId(17), Generation(9), 31);
    assert!(EnergyJob::admit(operation, model.clone(), config.clone(), EnergyNumericalBounds(census)).is_ok(), "exact observed MAX must succeed");

    let pointer = model.zones.as_ptr();
    let mut maximum = census;
    maximum.zones -= 1;
    let rejected = EnergyJob::admit(operation, model, config, EnergyNumericalBounds(maximum)).expect_err("MAX+1 zone owner must reject before transfer");
    assert_eq!(rejected.dimension, EnergyNumericalDimension::Zones);
    assert_eq!(rejected.model.zones.as_ptr(), pointer, "rejected authority retains the exact allocation identity");
    assert!(rejected.retry(EnergyNumericalBounds(census)).is_ok(), "exact rejected owner retries after sufficient credit");
}

#[test]
fn p7c1_every_independent_maximum_plus_one_dimension_rejects() {
    let census = EnergyNumericalCensus {
        zones: 1,
        surfaces: 1,
        fenestrations: 1,
        people: 1,
        lighting: 1,
        equipment: 1,
        infiltrations: 1,
        airflow_nodes: 1,
        airflow_links: 1,
        mechanical_ventilations: 1,
        thermostats: 1,
        humidistats: 1,
        ideal_loads: 1,
        faults: 1,
        zone_equipment: 1,
        plant_loops: 1,
        plant_equipment: 1,
        pv_systems: 1,
        batteries: 1,
        service_hot_water: 1,
        refrigeration: 1,
        water: 1,
        weather_records: 1,
        timesteps: 1,
        meters: 1,
        series: 1,
        samples: 1,
        history_values: 1,
        summary_rows: 1,
        identifier_bytes: 1,
        observed_items: 1,
        observed_bytes: 1,
        pages: 1,
        operations: 1,
        process_jobs: 1,
    };
    macro_rules! rejects {
        ($field:ident, $variant:ident) => {{
            let mut maximum = census;
            maximum.$field = 0;
            assert_eq!(census.first_exceeded(maximum), Some(EnergyNumericalDimension::$variant));
        }};
    }
    rejects!(zones, Zones);
    rejects!(surfaces, Surfaces);
    rejects!(fenestrations, Fenestrations);
    rejects!(people, People);
    rejects!(lighting, Lighting);
    rejects!(equipment, Equipment);
    rejects!(infiltrations, Infiltrations);
    rejects!(airflow_nodes, AirflowNodes);
    rejects!(airflow_links, AirflowLinks);
    rejects!(mechanical_ventilations, MechanicalVentilations);
    rejects!(thermostats, Thermostats);
    rejects!(humidistats, Humidistats);
    rejects!(ideal_loads, IdealLoads);
    rejects!(faults, Faults);
    rejects!(zone_equipment, ZoneEquipment);
    rejects!(plant_loops, PlantLoops);
    rejects!(plant_equipment, PlantEquipment);
    rejects!(pv_systems, PvSystems);
    rejects!(batteries, Batteries);
    rejects!(service_hot_water, ServiceHotWater);
    rejects!(refrigeration, Refrigeration);
    rejects!(water, Water);
    rejects!(weather_records, WeatherRecords);
    rejects!(timesteps, Timesteps);
    rejects!(meters, Meters);
    rejects!(series, Series);
    rejects!(samples, Samples);
    rejects!(history_values, HistoryValues);
    rejects!(summary_rows, SummaryRows);
    rejects!(identifier_bytes, IdentifierBytes);
    rejects!(observed_items, ObservedItems);
    rejects!(observed_bytes, ObservedBytes);
    rejects!(pages, Pages);
    rejects!(operations, Operations);
    rejects!(process_jobs, ProcessJobs);
}

#[test]
fn p7c1_live_owner_capacity_mutations_reject_the_declared_dimension() {
    macro_rules! capacity_law {
        ($field:ident, $census_field:ident, $variant:ident) => {{
            let mut model = test_model_full_topology();
            let config = SimulationConfig::default();
            model.$field.shrink_to_fit();
            let before = EnergyNumericalCensus::observe(&model, &config).expect("baseline census");
            model.$field.reserve_exact(1);
            let after = EnergyNumericalCensus::observe(&model, &config).expect("mutated census");
            assert!(after.$census_field > before.$census_field, "live mutation did not increase {}", stringify!($census_field));
            let mut maximum = EnergyNumericalBounds::default().0;
            maximum.$census_field = before.$census_field;
            let rejected = EnergyJob::admit(Operation::new(allocate_operation_id(), RevisionId(52), Generation(6), 211), model, config, EnergyNumericalBounds(maximum)).expect_err("live MAX+1 owner must reject");
            assert_eq!(rejected.dimension, EnergyNumericalDimension::$variant);
        }};
    }
    capacity_law!(zones, zones, Zones);
    capacity_law!(surfaces, surfaces, Surfaces);
    capacity_law!(fenestrations, fenestrations, Fenestrations);
    capacity_law!(people, people, People);
    capacity_law!(lighting, lighting, Lighting);
    capacity_law!(equipment, equipment, Equipment);
    capacity_law!(infiltrations, infiltrations, Infiltrations);
    capacity_law!(mechanical_ventilations, mechanical_ventilations, MechanicalVentilations);
    capacity_law!(thermostats, thermostats, Thermostats);
    capacity_law!(humidistats, humidistats, Humidistats);
    capacity_law!(ideal_loads, ideal_loads, IdealLoads);
    capacity_law!(faults, faults, Faults);
    capacity_law!(zone_equipment, zone_equipment, ZoneEquipment);
    capacity_law!(plant_loops, plant_loops, PlantLoops);
    capacity_law!(pv_systems, pv_systems, PvSystems);
    capacity_law!(battery_storage, batteries, Batteries);
    capacity_law!(shw_systems, service_hot_water, ServiceHotWater);
    capacity_law!(refrigeration_systems, refrigeration, Refrigeration);
    capacity_law!(water_systems, water, Water);
}

#[test]
fn p7c1_weather_owner_is_exactly_admitted_never_grows_and_retries_maximum_plus_one() {
    let model = test_model_single_zone();
    let mut records = vec![design_day_hour(0, -10.0), design_day_hour(1, -9.0)];
    records.shrink_to_fit();
    let logical_records = records.len().max(1);
    records.reserve_exact(3);
    assert!(records.capacity() > logical_records, "reserve-only mutation must increase owned weather backing");
    let pointer = records.as_ptr();
    let config = SimulationConfig { weather: Some(crate::site::EpwWeather { location: "fixed".into(), latitude_deg: 0.0, longitude_deg: 0.0, elevation_m: 0.0, time_zone_hours: 0.0, records }), ..Default::default() };
    let census = EnergyNumericalCensus::observe(&model, &config).expect("weather census");
    assert_eq!(census.weather_records, config.weather.as_ref().expect("weather").records.capacity());
    assert!(census.weather_records > logical_records, "census must not collapse backing capacity to len/max1");
    let operation = Operation::new(allocate_operation_id(), RevisionId(63), Generation(12), 251);
    let mut maximum = EnergyNumericalBounds::default().0;
    maximum.weather_records = logical_records;
    let rejected = EnergyJob::admit(operation, model, config, EnergyNumericalBounds(maximum)).expect_err("weather MAX+1 rejects exact owner");
    assert_eq!(rejected.dimension, EnergyNumericalDimension::WeatherRecords);
    assert_eq!(rejected.config.weather.as_ref().expect("weather owner").records.as_ptr(), pointer);
    let mut job = rejected.retry(EnergyNumericalBounds::default()).expect("weather owner retry");
    assert_eq!(job.weather.capacity(), census.weather_records);
    assert_eq!(job.weather.len(), 0);
    job.stage = EnergyJobStage::ResolveWeather;
    let mut preview_sequence = 0;
    for _ in 0..2 {
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut preview_sequence);
        assert_eq!(job.step(&mut context), StepOutcome::Yield);
    }
    assert_eq!(job.weather.len(), 2);
    assert_eq!(job.weather.capacity(), census.weather_records);
    let before_records = [*job.weather.get_index(0).expect("weather zero"), *job.weather.get_index(1).expect("weather one")];
    for index in logical_records..census.weather_records {
        assert!(job.weather.insert_stable(index, design_day_hour(index as u32, -8.0)).is_ok());
    }
    assert!(job.weather.insert_stable(census.weather_records, design_day_hour(census.weather_records as u32, -8.0)).is_err());
    assert_eq!(job.weather.len(), census.weather_records);
    assert_eq!(job.weather.capacity(), census.weather_records);
    assert_eq!(*job.weather.get_index(0).expect("weather zero"), before_records[0]);
    assert_eq!(*job.weather.get_index(1).expect("weather one"), before_records[1]);
    job.weather_cursor = 1;
    let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut preview_sequence);
    let mut outcome = job.step(&mut context);
    assert!(matches!(outcome, StepOutcome::Fault(_)));
    assert_eq!(job.weather_fault, Some(WeatherFault::SlotRejected));
    while !matches!(outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
    InteractiveJob::begin_close(&mut job);
    for _ in 0..100_000 {
        if matches!(InteractiveJob::close_step(&mut job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
            return;
        }
    }
    panic!("fixed weather authority did not close")
}

#[test]
fn p7c1_weather_reserve_only_capacity_is_independently_charged_to_items() {
    let model = test_model_single_zone();
    let mut config =
        SimulationConfig { weather: Some(crate::site::EpwWeather { location: "items".into(), latitude_deg: 0.0, longitude_deg: 0.0, elevation_m: 0.0, time_zone_hours: 0.0, records: vec![design_day_hour(0, -10.0)] }), ..Default::default() };
    config.weather.as_mut().expect("weather").records.shrink_to_fit();
    let before = EnergyNumericalCensus::observe(&model, &config).expect("baseline weather census");
    config.weather.as_mut().expect("weather").records.reserve_exact(7);
    let after = EnergyNumericalCensus::observe(&model, &config).expect("reserve-only weather census");
    assert!(after.weather_records > before.weather_records);
    let weather_delta = after.weather_records - before.weather_records;
    assert_eq!(after.observed_items - before.observed_items, weather_delta);
    assert_eq!(after.observed_bytes - before.observed_bytes, weather_delta * (size_of::<WeatherRecord>() + size_of::<Option<(usize, WeatherRecord)>>()));
    assert_eq!(after.pages, (after.observed_bytes + 16_383) / 16_384);
    let pointer = config.weather.as_ref().expect("weather").records.as_ptr();
    let mut maximum = EnergyNumericalBounds::default().0;
    maximum.observed_items = after.observed_items - 1;
    let operation = Operation::new(allocate_operation_id(), RevisionId(64), Generation(13), 257);
    let rejected = EnergyJob::admit(operation, model, config, EnergyNumericalBounds(maximum)).expect_err("weather backing item MAX+1 rejects before mount");
    assert_eq!(rejected.dimension, EnergyNumericalDimension::ObservedItems);
    assert_eq!(rejected.config.weather.as_ref().expect("weather").records.as_ptr(), pointer);
    let mut job = rejected.retry(EnergyNumericalBounds::default()).expect("weather item owner retry");
    InteractiveJob::begin_close(&mut job);
    for _ in 0..100_000 {
        if matches!(InteractiveJob::close_step(&mut job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
            return;
        }
    }
    panic!("weather item retry authority did not close")
}

#[test]
fn p7c1_one_fuel_chronology_is_identical_at_one_two_four_and_default_grants() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let operation = Operation::new(allocate_operation_id(), RevisionId(21), Generation(4), 91);
    let output = |fuel| drive_energy_job_with_fuel(EnergyJob::new(operation, model.clone(), config.clone()).expect("energy admission"), fuel).3;
    assert_eq!(output(1), output(2));
    assert_eq!(output(1), output(4));
    assert_eq!(output(1), output(32));
}

#[test]
fn p7c1_close_releases_no_more_than_one_owner_or_character_per_grant() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(1), Generation(1), 1);
    let mut job = EnergyJob::new(operation, test_model_full_topology(), SimulationConfig::default()).expect("energy admission");
    InteractiveJob::begin_close(&mut job);
    for _ in 0..100_000 {
        match InteractiveJob::close_step(&mut job, 1, 4) {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= 4);
            }
            semio_framework_job::InteractiveJobCloseStep::Complete => {
                assert!(InteractiveJob::terminal_is_empty(&job));
                return;
            }
            semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("Energy close cannot block"),
        }
    }
    panic!("Energy close did not retire within its admitted bound")
}

#[test]
fn p7c1_direct_drop_requeues_exact_generation_and_resumes_partial_close() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(33), Generation(7), 101);
    let mut job = EnergyJob::new(operation, test_model_full_topology(), SimulationConfig::default()).expect("energy admission");
    let retained_pointer = job.model.zones.as_ptr();
    InteractiveJob::begin_close(&mut job);
    assert!(matches!(InteractiveJob::close_step(&mut job, 1, 4), semio_framework_job::InteractiveJobCloseStep::Pending { .. }));
    drop(job);

    let mut recovered = EnergyJob::recover_abandoned(operation).expect("same generation abandonment authority");
    assert_eq!(recovered.model.zones.as_ptr(), retained_pointer);
    assert!(EnergyJob::recover_abandoned(Operation { generation: Generation(8), ..operation }).is_none());
    for _ in 0..100_000 {
        if matches!(InteractiveJob::close_step(&mut recovered, 1, 4), semio_framework_job::InteractiveJobCloseStep::Complete) {
            assert!(InteractiveJob::terminal_is_empty(&recovered));
            return;
        }
    }
    panic!("recovered Energy close did not retire within its admitted bound")
}

#[test]
fn p7c1_panic_unwind_requeues_the_same_incomplete_authority_once() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(34), Generation(11), 131);
    let job = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("energy admission");
    let retained_pointer = job.model.zones.as_ptr();
    let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let _authority = job;
        panic!("hostile Energy session unwind");
    }));
    assert!(unwind.is_err());
    let mut recovered = EnergyJob::recover_abandoned(operation).expect("panic requeued exact authority");
    assert_eq!(recovered.model.zones.as_ptr(), retained_pointer);
    assert!(EnergyJob::recover_abandoned(operation).is_none(), "recovery is single-owner");
    InteractiveJob::begin_close(&mut recovered);
    for _ in 0..100_000 {
        if matches!(InteractiveJob::close_step(&mut recovered, 1, 4), semio_framework_job::InteractiveJobCloseStep::Complete) {
            return;
        }
    }
    panic!("panic-recovered Energy authority did not close")
}

#[test]
fn p7c1_cancel_and_deadline_gate_every_declared_numerical_substage_before_mutation() {
    let stages = [
        EnergyJobStage::Validate,
        EnergyJobStage::ResolveWeather,
        EnergyJobStage::Precompute,
        EnergyJobStage::InitializeZones,
        EnergyJobStage::InitializeSurfaces,
        EnergyJobStage::InitializeWarmupHistory,
        EnergyJobStage::WarmupTimestep,
        EnergyJobStage::WarmupConvergence,
        EnergyJobStage::StartRun,
        EnergyJobStage::RunZoneTimestep,
        EnergyJobStage::AggregateZone,
        EnergyJobStage::AggregateFacility,
        EnergyJobStage::PublishTimestep,
        EnergyJobStage::Finalize,
        EnergyJobStage::Size,
        EnergyJobStage::FinalizeSummaries,
        EnergyJobStage::FinalizeMetrics,
        EnergyJobStage::FinalizeEconomics,
        EnergyJobStage::BuildResults,
        EnergyJobStage::PublishFinal,
        EnergyJobStage::EncodeOutput,
        EnergyJobStage::Complete,
    ];
    let operation = Operation::new(allocate_operation_id(), RevisionId(41), Generation(5), 151);
    let mut job = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("energy admission");
    for stage in stages {
        job.stage = stage;
        let before = job.numerical_cursor_signature();
        let cancel = CancelToken::root_now();
        cancel.cancel_now();
        let mut sequence = 0;
        let mut cancelled = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel, default_now_us, &mut sequence);
        assert_eq!(job.step(&mut cancelled), StepOutcome::Cancelled);
        assert_eq!(job.numerical_cursor_signature(), before, "cancel mutated {stage:?}");

        let mut deadline_sequence = 0;
        let mut deadline = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(0, 0), CancelToken::root_now(), default_now_us, &mut deadline_sequence);
        assert_eq!(job.step(&mut deadline), StepOutcome::Yield);
        assert_eq!(job.numerical_cursor_signature(), before, "deadline mutated {stage:?}");
    }
    InteractiveJob::begin_close(&mut job);
    for _ in 0..100_000 {
        if matches!(InteractiveJob::close_step(&mut job, 1, 4), semio_framework_job::InteractiveJobCloseStep::Complete) {
            return;
        }
    }
    panic!("gated Energy job did not close")
}

#[test]
fn p7c1_live_nested_authorities_gate_cancel_deadline_and_stale_before_mutation() {
    use crate::kernel::{P7C1_PLANT_STAGES, P7C1_SCHEDULE_LOOKUP_STAGES, P7C1_SYSTEM_SUBSTEP_STAGES, P7C1_TIMESTEP_BUILDER_STAGES, P7C1_TIMESTEP_STAGES, P7C1_ZONE_PREPARATION_STAGES};
    use crate::precompute::{P7C1_PRECOMPUTE_STAGES, P7C1_SURFACE_PRECOMPUTE_STAGES};
    use crate::sizing::P7C1_SIZING_STAGES;

    fn assert_gates(job: &mut EnergyJob, operation: Operation) {
        let before = job.numerical_cursor_signature();
        let cancel = CancelToken::root_now();
        cancel.cancel_now();
        let mut sequence = 0;
        let mut cancelled = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), cancel, default_now_us, &mut sequence);
        assert_eq!(job.step(&mut cancelled), StepOutcome::Cancelled);
        assert_eq!(job.numerical_cursor_signature(), before);

        let mut deadline_sequence = 0;
        let mut deadline = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(0, 0), CancelToken::root_now(), default_now_us, &mut deadline_sequence);
        assert_eq!(job.step(&mut deadline), StepOutcome::Yield);
        assert_eq!(job.numerical_cursor_signature(), before);

        let mut stale_sequence = 0;
        let mut stale = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut stale_sequence);
        let mut outcome = job.step(&mut stale);
        assert!(matches!(outcome, StepOutcome::Fault(_)));
        assert_eq!(job.numerical_cursor_signature(), before);
        while !matches!(outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
    }

    let operation = Operation::new(allocate_operation_id(), RevisionId(61), Generation(9), 241);
    let config = SimulationConfig { warmup_days: 1, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let mut job = EnergyJob::new(operation, test_model_full_topology(), config).expect("live nested gate admission");
    let validation_stage = job.validation.stage;
    for stage in P7C1_VALIDATION_STAGES {
        job.validation.stage = stage;
        assert_gates(&mut job, operation);
    }
    job.validation.stage = validation_stage;
    let finalization_stage = job.finalization.stage;
    for stage in P7C1_FINALIZATION_STAGES {
        job.finalization.stage = stage;
        assert_gates(&mut job, operation);
    }
    job.finalization.stage = finalization_stage;
    let result_stage = job.result_build.stage;
    for stage in P7C1_RESULT_BUILD_STAGES {
        job.result_build.stage = stage;
        assert_gates(&mut job, operation);
    }
    job.result_build.stage = result_stage;
    let initialize_backing_stage = job.initialize_backing_stage;
    for stage in 0..=2 {
        job.initialize_backing_stage = stage;
        assert_gates(&mut job, operation);
    }
    job.initialize_backing_stage = initialize_backing_stage;
    let run_backing_stage = job.run_backing_stage;
    for stage in 0..=6 {
        job.run_backing_stage = stage;
        assert_gates(&mut job, operation);
    }
    job.run_backing_stage = run_backing_stage;
    let encode_section = job.encode_section;
    for section in 0..=5 {
        job.encode_section = section;
        assert_gates(&mut job, operation);
    }
    job.encode_section = encode_section;
    let mut preview_sequence = 0;
    let mut precompute_gated = false;
    let mut surface_precompute_gated = false;
    let mut timestep_builder_gated = false;
    let mut timestep_gated = false;
    let mut zone_gated = false;
    let mut system_gated = false;
    let mut plant_gated = false;
    let mut schedule_gated = false;
    let mut warmup_gated = false;
    let mut aggregate_zone_gated = false;
    let mut aggregate_facility_gated = false;
    let mut sizing_gated = false;

    for _ in 0..200_000 {
        if !precompute_gated && job.precompute.is_some() {
            let original = job.precompute.as_ref().expect("precompute").stage();
            for stage in P7C1_PRECOMPUTE_STAGES {
                job.precompute.as_mut().expect("precompute").set_stage_for_gate(stage);
                assert_gates(&mut job, operation);
            }
            job.precompute.as_mut().expect("precompute").set_stage_for_gate(original);
            precompute_gated = true;
        }
        if !surface_precompute_gated && job.precompute.as_ref().and_then(PrecomputeBuilder::surface_stage_for_gate).is_some() {
            let original = job.precompute.as_ref().and_then(PrecomputeBuilder::surface_stage_for_gate).expect("surface precompute stage");
            for stage in P7C1_SURFACE_PRECOMPUTE_STAGES {
                assert!(job.precompute.as_mut().expect("precompute").set_surface_stage_for_gate(stage));
                assert_gates(&mut job, operation);
            }
            job.precompute.as_mut().expect("precompute").set_surface_stage_for_gate(original);
            surface_precompute_gated = true;
        }
        if !timestep_builder_gated && job.timestep_builder.is_some() {
            let original = job.timestep_builder.as_ref().expect("timestep builder").stage_for_gate();
            for stage in P7C1_TIMESTEP_BUILDER_STAGES {
                job.timestep_builder.as_mut().expect("timestep builder").set_stage_for_gate(stage);
                assert_gates(&mut job, operation);
            }
            job.timestep_builder.as_mut().expect("timestep builder").set_stage_for_gate(original);
            timestep_builder_gated = true;
        }
        if job.timestep_work.is_some() {
            if !timestep_gated {
                let original = job.timestep_work.as_ref().expect("timestep").stage();
                for stage in P7C1_TIMESTEP_STAGES {
                    job.timestep_work.as_mut().expect("timestep").set_stage_for_gate(stage);
                    assert_gates(&mut job, operation);
                }
                job.timestep_work.as_mut().expect("timestep").set_stage_for_gate(original);
                timestep_gated = true;
            }
            if !zone_gated && job.timestep_work.as_ref().and_then(TimestepWork::zone_preparation_stage).is_some() {
                let original = job.timestep_work.as_ref().and_then(TimestepWork::zone_preparation_stage).expect("zone stage");
                for stage in P7C1_ZONE_PREPARATION_STAGES {
                    assert!(job.timestep_work.as_mut().expect("timestep").set_zone_preparation_stage_for_gate(stage));
                    assert_gates(&mut job, operation);
                }
                job.timestep_work.as_mut().expect("timestep").set_zone_preparation_stage_for_gate(original);
                zone_gated = true;
            }
            if !system_gated && job.timestep_work.as_ref().and_then(TimestepWork::system_substep_stage).is_some() {
                let original = job.timestep_work.as_ref().and_then(TimestepWork::system_substep_stage).expect("system stage");
                for stage in P7C1_SYSTEM_SUBSTEP_STAGES {
                    assert!(job.timestep_work.as_mut().expect("timestep").set_system_substep_stage_for_gate(stage));
                    assert_gates(&mut job, operation);
                }
                job.timestep_work.as_mut().expect("timestep").set_system_substep_stage_for_gate(original);
                system_gated = true;
            }
            if !plant_gated && job.timestep_work.as_ref().and_then(TimestepWork::plant_stage).is_some() {
                let original = job.timestep_work.as_ref().and_then(TimestepWork::plant_stage).expect("plant stage");
                for stage in P7C1_PLANT_STAGES {
                    assert!(job.timestep_work.as_mut().expect("timestep").set_plant_stage_for_gate(stage));
                    assert_gates(&mut job, operation);
                }
                job.timestep_work.as_mut().expect("timestep").set_plant_stage_for_gate(original);
                plant_gated = true;
            }
            if !schedule_gated && job.timestep_work.as_ref().and_then(TimestepWork::schedule_lookup_stage).is_some() {
                let original = job.timestep_work.as_ref().and_then(TimestepWork::schedule_lookup_stage).expect("schedule stage");
                for stage in P7C1_SCHEDULE_LOOKUP_STAGES {
                    assert!(job.timestep_work.as_mut().expect("timestep").set_schedule_lookup_stage_for_gate(stage));
                    assert_gates(&mut job, operation);
                }
                job.timestep_work.as_mut().expect("timestep").set_schedule_lookup_stage_for_gate(original);
                schedule_gated = true;
            }
        }
        if !warmup_gated && job.warmup_convergence.is_some() {
            let original = job.warmup_convergence.as_ref().expect("warmup convergence").stage;
            for stage in P7C1_WARMUP_CONVERGENCE_STAGES {
                job.warmup_convergence.as_mut().expect("warmup convergence").stage = stage;
                assert_gates(&mut job, operation);
            }
            job.warmup_convergence.as_mut().expect("warmup convergence").stage = original;
            warmup_gated = true;
        }
        if !aggregate_zone_gated && job.aggregate_zone_work.is_some() {
            let original = job.aggregate_zone_work.as_ref().expect("zone aggregate").stage;
            for stage in P7C1_AGGREGATE_STAGES {
                job.aggregate_zone_work.as_mut().expect("zone aggregate").stage = stage;
                assert_gates(&mut job, operation);
            }
            job.aggregate_zone_work.as_mut().expect("zone aggregate").stage = original;
            aggregate_zone_gated = true;
        }
        if !aggregate_facility_gated && job.aggregate_facility_work.is_some() {
            let original = job.aggregate_facility_work.as_ref().expect("facility aggregate").stage;
            for stage in P7C1_AGGREGATE_STAGES {
                job.aggregate_facility_work.as_mut().expect("facility aggregate").stage = stage;
                assert_gates(&mut job, operation);
            }
            job.aggregate_facility_work.as_mut().expect("facility aggregate").stage = original;
            aggregate_facility_gated = true;
        }
        if !sizing_gated && job.sizing_builder.is_some() {
            let original = job.sizing_builder.as_ref().expect("sizing").stage_for_gate();
            for stage in P7C1_SIZING_STAGES {
                job.sizing_builder.as_mut().expect("sizing").set_stage_for_gate(stage);
                assert_gates(&mut job, operation);
            }
            job.sizing_builder.as_mut().expect("sizing").set_stage_for_gate(original);
            sizing_gated = true;
        }
        if precompute_gated && surface_precompute_gated && timestep_builder_gated && timestep_gated && zone_gated && system_gated && plant_gated && schedule_gated && warmup_gated && aggregate_zone_gated && aggregate_facility_gated && sizing_gated {
            InteractiveJob::begin_close(&mut job);
            for _ in 0..100_000 {
                if matches!(InteractiveJob::close_step(&mut job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::InteractiveJobCloseStep::Complete) {
                    return;
                }
            }
            panic!("live nested gate job did not close")
        }
        let mut context = StepContext::new(operation.operation, operation.generation, semio_framework_job::StepBudget::new(1, u64::MAX), CancelToken::root_now(), default_now_us, &mut preview_sequence);
        let mut outcome = job.step(&mut context);
        while !matches!(outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
    }
    panic!("live nested cursor family was never mounted")
}

#[test]
fn p7c1_language_agnostic_law_fixture_matches_reference_parser() {
    let source = include_str!("../../../../../../🪨️tests/🧮️p7c1-energy-numerical-laws.json");
    let reference: pack::json::Value = pack::json::parse(source).expect("reference JSON parser");
    let marker = "\"schema\": \"";
    let start = source.find(marker).expect("schema field") + marker.len();
    let end = start + source[start..].find('"').expect("schema terminator");
    let schema = &source[start..end];
    assert_eq!(reference["schema"].as_str(), Some(schema));
    assert_eq!(reference["step"]["fuel"].as_u64(), Some(1));
}
