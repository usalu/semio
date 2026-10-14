use super::*;
use semio_framework_job::{allocate_operation_id, default_now_us, CancelToken, Generation, JobPayloadStream, StepBudget};
use std::time::Instant;
use crate::retirement::energy_self_funded;
use crate::sim::{close_owned_job, ENERGY_BATCH_GRANT};
use crate::wire::{close_retained_packet, EnergyJobPreview};
use crate::model::test_model_single_zone;

#[derive(Debug, PartialEq, Eq)]
enum Seen {
    Yield,
    Preview,
    Checkpoint,
    Fault,
    Cancelled,
    Complete(Vec<u8>),
}

fn page_bytes(payload: &RetainedJobPayload) -> Vec<u8> {
    (0..payload.page_count()).flat_map(|index| payload.page(index).expect("retained output page").iter().copied()).collect()
}

fn seen_step(job: &mut EnergyJob, context: &mut StepContext<'_>) -> Seen {
    match InteractiveJob::step(job, context).expect("energy step admission") {
        None | Some(JobOutcomeBorrow::Yield { .. }) => Seen::Yield,
        Some(JobOutcomeBorrow::PreviewReady { .. }) => Seen::Preview,
        Some(JobOutcomeBorrow::CheckpointReady { .. }) => Seen::Checkpoint,
        Some(JobOutcomeBorrow::Fault { .. }) => Seen::Fault,
        Some(JobOutcomeBorrow::Cancelled { .. }) => Seen::Cancelled,
        Some(JobOutcomeBorrow::Complete { output, .. }) => Seen::Complete(output.map(page_bytes).unwrap_or_default()),
    }
}

fn build_checkpoint_packet(job: &mut EnergyJob) -> EnergyWirePacket {
    let operation = job.operation;
    let hour_index = job.hour_index as u64;
    job.start_wire(EnergyWireKind::Checkpoint, hour_index, JobPayloadStream::CheckpointState).expect("checkpoint preflight");
    let mut sequence = 0;
    for _ in 0..32 {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut sequence_receipt);
        if job.step_wire_build(&mut context).expect("checkpoint fragment") {
            return job.wire_ready.take().expect("sealed checkpoint packet");
        }
    }
    panic!("checkpoint packet did not seal within fixed field bound")
}

fn retained_test_packet(operation: Operation, identity: EnergyWireIdentity, bytes: &[u8]) -> EnergyWirePacket {
    let mut writer = semio_framework_job::RetainedJobPayloadWriter::new(JobPayloadStream::CheckpointState);
    let mut sequence = 0;
    let mut sequence_receipt = RetainedCloneProgress::default();
    let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut sequence_receipt);
    let mut page = writer.admit_page(&mut context).expect("test page admission");
    page.write(bytes).expect("single fixed checkpoint page");
    page.commit();
    EnergyWirePacket { kind: EnergyWireKind::Checkpoint, identity, payload: writer.finish().expect("sealed test packet"), preview: None, reservation: None }
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
        let mut preview_sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation, generation, StepBudget::new(fuel, u64::MAX, ENERGY_BATCH_GRANT), cancel.clone(), default_now_us, &mut preview_sequence, &mut preview_sequence_receipt);
        let outcome = seen_step(&mut job, &mut context);
        let elapsed = start.elapsed();
        worst = worst.max(elapsed);
        match outcome {
            Seen::Preview => {
                let mut packet = job.take_preview_packet(generation).expect("fresh preview generation").expect("preview packet");
                previews.push(packet.preview().expect("typed projection belongs to canonical preview packet").clone());
                close_retained_packet(&mut packet);
            }
            Seen::Checkpoint => {
                let mut lease = job.take_checkpoint_packet(generation).expect("fresh checkpoint generation").expect("checkpoint lease");
                close_retained_packet(lease.packet_mut());
                job.ack_checkpoint_packet(lease).expect("checkpoint ACK");
                checkpoints += 1;
            }
            Seen::Complete(bytes) => return (job, previews, checkpoints, bytes, worst),
            Seen::Fault => panic!("energy job faulted"),
            Seen::Cancelled => panic!("energy job unexpectedly cancelled"),
            Seen::Yield => {}
        }
    }
    panic!("energy job did not complete within the deterministic step bound")
}

fn payload_bytes(mut payload: RetainedJobPayload) -> Vec<u8> {
    let mut bytes = Vec::new();
    for index in 0..payload.page_count() {
        bytes.extend_from_slice(payload.page(index).expect("retained output page"));
    }
    while !payload.terminal_is_empty() {
        let grant = energy_self_funded(payload.retirement_demands().expect("a locally owned payload quotes its close demand"));
        let _ = payload.close_step(grant);
    }
    bytes
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
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut sequence_receipt);
        let _ = recovered.step(&mut context).expect("field decode");
    }
    drop(recovered);
    assert!(EnergyRestoreJob::recover_abandoned(Operation { generation: Generation(8), ..operation }).is_none());
    let mut recovered = EnergyRestoreJob::recover_abandoned(operation).expect("same generation recovers once");
    for _ in 0..32 {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut sequence_receipt);
        if recovered.step(&mut context).expect("restore rebuild") {
            break;
        }
    }
    let mut sequence_receipt = RetainedCloneProgress::default();
    let install_context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut sequence_receipt);
    let mut restored = recovered.finish(&install_context).expect("restored authority");
    close_owned_job(&mut restored);
    close_owned_job(&mut source);
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
        close_retained_packet(&mut rejected.packet);
    }
    let mut trailing = baseline.clone();
    trailing.push(0);
    let packet = retained_test_packet(operation, identity, &trailing);
    let mut rejected = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect_err("trailing byte mutation");
    close_retained_packet(&mut rejected.packet);
    let mut ignored_digest = baseline;
    ignored_digest[156] ^= 0x80;
    let packet = retained_test_packet(operation, identity, &ignored_digest);
    let mut restore = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect("digest mutation passes bounded header admission");
    let mut sequence = 0;
    for _ in 0..64 {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut sequence_receipt);
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
            let mut sequence_receipt = RetainedCloneProgress::default();
            let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut sequence_receipt);
            assert!(!restore.step(&mut context).expect("decoded count remains retained while replay proves mismatch"));
        }
        assert!(!restore.ready, "decoded table/history count cannot be cap-checked then discarded");
    }
    close_owned_job(&mut source);
}

#[test]
fn p7c2_restore_stale_step_and_install_preserve_exact_replay_authority() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(95), Generation(14), 0xbeef);
    let mut source = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("source admission");
    let packet = build_checkpoint_packet(&mut source);
    let mut restore = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect("restore admission");
    let mut sequence = 0;
    let mut sequence_receipt = RetainedCloneProgress::default();
    let mut stale = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut sequence_receipt);
    assert_eq!(restore.step(&mut stale), Err(EnergyWireRejection::Identity));
    assert_eq!(restore.field, 0);
    drop(stale);
    for _ in 0..64 {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut sequence_receipt);
        if restore.step(&mut context).expect("bounded replay step") {
            break;
        }
    }
    assert!(restore.ready);
    let mut sequence_receipt = RetainedCloneProgress::default();
    let stale_install = StepContext::new(operation.operation, Generation(operation.generation.0 + 1), StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut sequence_receipt);
    let restore = restore.finish(&stale_install).expect_err("stale install retains exact replay authority");
    drop(stale_install);
    let mut sequence_receipt = RetainedCloneProgress::default();
    let install = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut sequence_receipt);
    let mut restored = restore.finish(&install).expect("fresh generation installs exact replay authority");
    close_owned_job(&mut restored);
    close_owned_job(&mut source);
}

#[test]
fn p7c2_restored_commit_bytes_match_one_and_four_fuel_chronology() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let operation = Operation::new(allocate_operation_id(), RevisionId(93), Generation(9), 777);
    let mut original = EnergyJob::new(operation, model.clone(), config.clone()).expect("original admission");
    let mut sequence = 0;
    let packet = loop {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut sequence_receipt);
        match seen_step(&mut original, &mut context) {
            Seen::Preview => {
                let mut packet = original.take_preview_packet(operation.generation).expect("fresh preview").expect("preview packet");
                close_retained_packet(&mut packet);
            }
            Seen::Checkpoint => {
                let packet = build_checkpoint_packet(&mut original);
                let mut lease = original.take_checkpoint_packet(operation.generation).expect("fresh checkpoint").expect("checkpoint lease");
                close_retained_packet(lease.packet_mut());
                original.ack_checkpoint_packet(lease).expect("checkpoint ACK advances only after close");
                break packet;
            }
            Seen::Yield => {}
            other => panic!("checkpoint expected before terminal: {other:?}"),
        }
    };
    let mut restore = EnergyRestoreJob::admit(operation, model, config, packet, EnergyNumericalBounds::default()).expect("restore admission");
    for _ in 0..100_000 {
        let mut sequence_receipt = RetainedCloneProgress::default();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut sequence_receipt);
        if restore.step(&mut context).expect("restore field/rebuild") {
            break;
        }
    }
    let mut sequence_receipt = RetainedCloneProgress::default();
    let install_context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT), CancelToken::root_now(), default_now_us, &mut sequence, &mut sequence_receipt);
    let restored = restore.finish(&install_context).expect("restore finish");
    let (_, _, _, original_bytes, _) = drive_energy_job_with_fuel(original, 1);
    let (_, _, _, restored_bytes, _) = drive_energy_job_with_fuel(restored, 4);
    assert_eq!(restored_bytes, original_bytes);
}
