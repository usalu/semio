use super::*;
use semio_framework_job::{allocate_operation_id, Generation, OriginalTurnFailure};
use semio_framework_job_payload::{JobPayloadStream};
use std::time::Instant;
use crate::sim::ENERGY_BATCH_GRANT;
use crate::wire::EnergyJobPreview;
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

fn seen_step(job: &mut EnergyJob, context: &mut StepContext<'_>) -> Result<Seen,OriginalTurnFailure> {
    Ok(match InteractiveJob::step(job, context)? {
        None | Some(JobOutcomeBorrow::Yield { .. }) => Seen::Yield,
        Some(JobOutcomeBorrow::PreviewReady { .. }) => Seen::Preview,
        Some(JobOutcomeBorrow::CheckpointReady { .. }) => Seen::Checkpoint,
        Some(JobOutcomeBorrow::Fault { .. }) => Seen::Fault,
        Some(JobOutcomeBorrow::Cancelled { .. }) => Seen::Cancelled,
        Some(JobOutcomeBorrow::Complete { output, .. }) => Seen::Complete(output.map(page_bytes).unwrap_or_default()),
    })
}

use crate::sim::testing::OriginalEnergyCaller;
fn prepare_input_identity(job:&mut EnergyJob,caller:&mut OriginalEnergyCaller){for _ in 0..1_000_000{if caller.turn(ENERGY_BATCH_GRANT,1,u64::MAX,|cx|Ok(job.step_input_identity(cx))).unwrap().expect("original complete input identity"){return}}panic!("original input identity remained pending")}
fn build_checkpoint_packet(job:&mut EnergyJob,caller:&mut OriginalEnergyCaller)->EnergyWirePacket{prepare_input_identity(job,caller);job.start_wire(EnergyWireKind::Checkpoint,job.hour_index as u64,JobPayloadStream::CheckpointState).expect("checkpoint preflight");for _ in 0..1_000_000{if caller.turn(ENERGY_BATCH_GRANT,1,u64::MAX,|cx|Ok(job.step_wire_build(cx))).unwrap().expect("original checkpoint fragment"){return job.wire_ready.take().expect("sealed original checkpoint")}}panic!("original checkpoint did not seal")}
fn retained_test_packet(_operation:Operation,identity:EnergyWireIdentity,bytes:&[u8],caller:&mut OriginalEnergyCaller)->EnergyWirePacket{
 let mut writer=Some(semio_framework_job_payload::RetainedJobPayloadWriter::new(JobPayloadStream::CheckpointState));let mut preborn=Some(semio_framework_job_payload::JobPayloadPageSource::new());let mut output=None;let copied_bytes=bytes.len()+std::mem::size_of::<EnergyWirePacket>()+std::mem::size_of::<Option<semio_framework_job_payload::RetainedJobPayloadWriter>>();
 caller.turn(ENERGY_BATCH_GRANT,1,u64::MAX,|cx|{let grant=cx.retained_grant();if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"native original checkpoint source requires its quoted copy loan").into())}let source=preborn.take().unwrap();match cx.admit_payload_page(writer.as_mut().unwrap(),source){Ok(mut page)=>{page.write(bytes).expect("authored bounded native source");page.commit();},Err(rejected)=>{preborn=Some(rejected.into_source());return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"native checkpoint source remains original on page refusal").into())}}match writer.take().unwrap().finish(){Ok(payload)=>output=Some(EnergyWirePacket{kind:EnergyWireKind::Checkpoint,identity,payload,preview:None,reservation:None}),Err(original)=>{writer=Some(original);return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"native packet keeps the original refused writer").into())}}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()})?;Ok(())}).expect("same original checkpoint page binding and retained sealed candidate");assert!(preborn.is_none());output.unwrap()
}
fn step_restore(job:&mut EnergyRestoreJob,caller:&mut OriginalEnergyCaller)->Result<bool,EnergyWireRejection>{caller.turn(ENERGY_BATCH_GRANT,1,u64::MAX,|cx|Ok(job.step(cx))).expect("original restore source turn")}
fn finish_restore(job:EnergyRestoreJob,caller:&mut OriginalEnergyCaller)->Result<EnergyJob,EnergyRestoreJob>{let mut input=Some(job);let mut output=None;let copied_bytes=std::mem::size_of::<EnergyRestoreJob>()+std::mem::size_of::<EnergyJob>();for _ in 0..4096{caller.turn(ENERGY_BATCH_GRANT,1,u64::MAX,|cx|{let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return Ok(())}output=Some(input.take().unwrap().finish(cx));cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()})?;Ok(())}).expect("original install retains candidate before receipt acceptance");if let Some(output)=output.take(){return output}}panic!("original install remained pending")}
fn close_restore(job:&mut EnergyRestoreJob,caller:&mut OriginalEnergyCaller){for _ in 0..1_000_000{if job.terminal_is_empty(){return}caller.turn(ENERGY_BATCH_GRANT,64,u64::MAX,|cx|cx.with_original_retirement(|cx|job.close_step(cx))).expect("same original restore close");}panic!("original restore close remained pending")}
fn close_rejected(job:&mut EnergyCheckpointRejected,caller:&mut OriginalEnergyCaller){for _ in 0..1_000_000{if job.terminal_is_empty(){return}let demand=job.close_demands().unwrap();caller.turn(ENERGY_BATCH_GRANT,64,u64::MAX,|cx|{semio_framework_job::close_original_retained_step(cx,demand,|loan|job.close_step(loan).map(|step|RetainedCloneStep::Progress(step.progress())))?;Ok(())}).expect("same original rejected restore close");}panic!("original rejected restore remained pending")}
fn drive_energy_job_with_fuel(mut job:EnergyJob,caller:&mut OriginalEnergyCaller,fuel:u64)->(EnergyJob,Vec<EnergyJobPreview>,usize,Vec<u8>,std::time::Duration){let generation=job.operation.generation;let mut previews=Vec::new();let mut checkpoints=0;let mut worst=std::time::Duration::ZERO;let mut completed_bytes=None;for _ in 0..1_000_000{let start=Instant::now();let observed=caller.turn(ENERGY_BATCH_GRANT,fuel,u64::MAX,|cx|seen_step(&mut job,cx)).unwrap();worst=worst.max(start.elapsed());if let Some(mut lease)=job.take_commit_packet(generation).unwrap(){completed_bytes=Some(page_bytes(&lease.packet().payload));caller.close_packet(lease.packet_mut());let mut lease=Some(lease);caller.acknowledge(&mut job,&mut lease,EnergyWireKind::Commit);}match observed{Seen::Preview=>{let mut packet=job.take_preview_packet(generation).unwrap().unwrap();previews.push(packet.preview().unwrap().clone());caller.close_packet(&mut packet);},Seen::Checkpoint=>{let mut lease=job.take_checkpoint_packet(generation).unwrap().unwrap();caller.close_packet(lease.packet_mut());let mut lease=Some(lease);caller.acknowledge(&mut job,&mut lease,EnergyWireKind::Checkpoint);checkpoints+=1;},Seen::Complete(bytes)=>{caller.close_job(&mut job);return(job,previews,checkpoints,completed_bytes.unwrap_or(bytes),worst)},Seen::Yield=>{},other=>panic!("original energy replay ended with {other:?}")}}panic!("native original replay remained pending")}
fn payload_bytes(mut payload:RetainedJobPayload,caller:&mut OriginalEnergyCaller)->Vec<u8>{let bytes=(0..payload.page_count()).flat_map(|i|payload.page(i).unwrap().iter().copied()).collect();while !payload.terminal_is_empty(){let demand=payload.retirement_demands().unwrap();caller.turn(ENERGY_BATCH_GRANT,64,u64::MAX,|cx|{semio_framework_job::close_original_retained_step(cx,demand,|loan|payload.close_step(loan))?;Ok(())}).unwrap();}bytes}

#[test]
fn p7c2_wire_schema_maximum_plus_one_and_exact_restore_owner() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(91), Generation(7), 0x55aa);let mut caller=OriginalEnergyCaller::new(operation.operation,operation.generation);
    let mut source = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("source admission");
    let packet = build_checkpoint_packet(&mut source,&mut caller);
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
    for _ in 0..4 {

        let _ = step_restore(&mut recovered,&mut caller).expect("field decode");
    }
    drop(recovered);
    assert!(EnergyRestoreJob::recover_abandoned(Operation { generation: Generation(8), ..operation }).is_none());
    let mut recovered = EnergyRestoreJob::recover_abandoned(operation).expect("same generation recovers once");
    for _ in 0..1_000_000 {

        if step_restore(&mut recovered,&mut caller).expect("restore rebuild") {
            break;
        }
    }

    let mut restored = finish_restore(recovered,&mut caller).expect("restored authority");
    caller.close_job(&mut restored);
    caller.close_job(&mut source);
caller.finish();}

#[test]
fn p7c2_live_schema_mutations_reject_before_restore_mount() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(94), Generation(12), 0xdead);let mut caller=OriginalEnergyCaller::new(operation.operation,operation.generation);
    let mut source = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("source admission");
    let packet = build_checkpoint_packet(&mut source,&mut caller);
    let identity = packet.identity;
    let baseline = payload_bytes(packet.payload,&mut caller);
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
        let packet = retained_test_packet(operation, identity, &bytes,&mut caller);
        let mut rejected = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect_err("hostile header/cap mutation");
        close_rejected(&mut rejected,&mut caller);
    }
    let mut trailing = baseline.clone();
    trailing.push(0);
    let packet = retained_test_packet(operation, identity, &trailing,&mut caller);
    let mut rejected = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect_err("trailing byte mutation");
    close_rejected(&mut rejected,&mut caller);
    let mut ignored_digest = baseline;
    ignored_digest[156] ^= 0x80;
    let packet = retained_test_packet(operation, identity, &ignored_digest,&mut caller);
    let mut restore = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect("digest mutation passes bounded header admission");
    for _ in 0..64 {

        assert!(!step_restore(&mut restore,&mut caller).expect("digest mutation remains retained while replay proves mismatch"));
    }
    assert!(!restore.ready, "decoded numerical digest cannot be ignored by fresh-job restore");close_restore(&mut restore,&mut caller);
    for offset in [132usize, 140, 148] {
        let mut ignored_count = ignored_digest.clone();
        ignored_count[156] ^= 0x80;
        ignored_count[offset] = 1;
        let packet = retained_test_packet(operation, identity, &ignored_count,&mut caller);
        let mut restore = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect("under-cap decoded count mutation passes bounded header admission");
        for _ in 0..64 {

            assert!(!step_restore(&mut restore,&mut caller).expect("decoded count remains retained while replay proves mismatch"));
        }
        assert!(!restore.ready, "decoded table/history count cannot be cap-checked then discarded");close_restore(&mut restore,&mut caller);
    }
    caller.close_job(&mut source);
caller.finish();}

#[test]
fn p7c2_restore_stale_step_and_install_preserve_exact_replay_authority() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(95), Generation(14), 0xbeef);let mut foreign_caller=OriginalEnergyCaller::new(operation.operation,Generation(operation.generation.0+1));let mut caller=OriginalEnergyCaller::new(operation.operation,operation.generation);
    let mut source = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("source admission");
    let packet = build_checkpoint_packet(&mut source,&mut caller);
    let mut restore = EnergyRestoreJob::admit(operation, test_model_single_zone(), SimulationConfig::default(), packet, EnergyNumericalBounds::default()).expect("restore admission");

    assert_eq!(step_restore(&mut restore,&mut foreign_caller), Err(EnergyWireRejection::Identity));
    assert_eq!(restore.field, 0);
    for _ in 0..1_000_000 {

        if step_restore(&mut restore,&mut caller).expect("bounded replay step") {
            break;
        }
    }
    assert!(restore.ready);

    let restore = finish_restore(restore,&mut foreign_caller).expect_err("stale install retains exact replay authority");

    let mut restored = finish_restore(restore,&mut caller).expect("fresh generation installs exact replay authority");
    caller.close_job(&mut restored);
    caller.close_job(&mut source);
foreign_caller.finish();caller.finish();}

#[test]
fn p7c2_restored_commit_bytes_match_one_and_four_fuel_chronology() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let operation = Operation::new(allocate_operation_id(), RevisionId(93), Generation(9), 777);let mut caller=OriginalEnergyCaller::new(operation.operation,operation.generation);
    let mut original = EnergyJob::new(operation, model.clone(), config.clone()).expect("original admission");
    let packet = loop {

        match caller.turn(ENERGY_BATCH_GRANT,1,u64::MAX,|cx|seen_step(&mut original,cx)).expect("same original replay numerical turn") {
            Seen::Preview => {
                let mut packet = original.take_preview_packet(operation.generation).expect("fresh preview").expect("preview packet");
                caller.close_packet(&mut packet);
            }
            Seen::Checkpoint => {
                let packet = build_checkpoint_packet(&mut original,&mut caller);
                let mut lease = original.take_checkpoint_packet(operation.generation).expect("fresh checkpoint").expect("checkpoint lease");
                caller.close_packet(lease.packet_mut());
                let mut lease=Some(lease);caller.acknowledge(&mut original,&mut lease,EnergyWireKind::Checkpoint);
                break packet;
            }
            Seen::Yield => {}
            other => panic!("checkpoint expected before terminal: {other:?}"),
        }
    };
    let mut restore = EnergyRestoreJob::admit(operation, model, config, packet, EnergyNumericalBounds::default()).expect("restore admission");
    for _ in 0..100_000 {

        if step_restore(&mut restore,&mut caller).expect("restore field/rebuild") {
            break;
        }
    }

    let restored = finish_restore(restore,&mut caller).expect("restore finish");
    let (_, _, _, original_bytes, _) = drive_energy_job_with_fuel(original,&mut caller,1);
    let (_, _, _, restored_bytes, _) = drive_energy_job_with_fuel(restored,&mut caller,4);
    assert_eq!(restored_bytes, original_bytes);
caller.finish();}

use semio_s_plugin_energy_engine_site as site;
use site::WeatherRecord;
/// 🧷️ The same model and numerical census cannot admit foreign weather/calendar/run-year checkpoint authority.
#[test]
fn weather_checkpoint_rejects_foreign_config_and_preserves_same_model_authority(){
 let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../📍️site/📅️chronology/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let records:Vec<WeatherRecord>=serde_json::from_value(fixture["cases"][0]["records"].clone()).unwrap();let model=test_model_single_zone();let config=SimulationConfig{run_period_year:2026,run_period_start_month:1,run_period_start_day:15,run_period_end_month:1,run_period_end_day:16,warmup_days:0,weather:Some(site::WeatherData{calendar:site::WeatherCalendar::ActualDates,location:"authored48".into(),latitude_deg:52.37,longitude_deg:9.74,elevation_m:55.0,time_zone_hours:1.0,records}),..Default::default()};
 let operation=Operation::new(allocate_operation_id(),RevisionId(194),Generation(22),0xa11c);let mut caller=OriginalEnergyCaller::new(operation.operation,operation.generation);let mut original=EnergyJob::new(operation,model.clone(),config.clone()).unwrap();let packet=build_checkpoint_packet(&mut original,&mut caller);let identity=packet.identity;let baseline=payload_bytes(packet.payload,&mut caller);
 for row in fixture["authorityChanges"].as_array().unwrap(){let mut foreign=config.clone();match row["id"].as_str().unwrap(){"weather-record-value"=>foreign.weather.as_mut().unwrap().records[0].dry_bulb_c=1.25,"weather-calendar"=>foreign.weather.as_mut().unwrap().calendar=site::WeatherCalendar::TypicalYear,"run-period-year"=>foreign.run_period_year=2027,_=>panic!("unknown authored authority probe")};assert_ne!(foreign,config);assert_eq!(EnergyNumericalCensus::observe(&model,&foreign),EnergyNumericalCensus::observe(&model,&config));let packet=retained_test_packet(operation,identity,&baseline,&mut caller);match EnergyRestoreJob::admit(operation,model.clone(),foreign,packet,EnergyNumericalBounds::default()){Err(mut rejected)=>{close_rejected(&mut rejected,&mut caller);},Ok(mut restore)=>{let mut sequence=0;let mut ready=false;let mut failure=None;for _ in 0..1_000_000{let mut receipt=RetainedCloneProgress::default();match step_restore(&mut restore,&mut caller){Ok(true)=>{ready=true;break;},Ok(false)=>{},Err(error)=>{failure=Some(error);break;}}}close_restore(&mut restore,&mut caller);assert!(!ready,"foreign input config mounted same-model checkpoint: {}",row["id"]);assert_eq!(failure,Some(EnergyWireRejection::Identity),"full original input hash must actually reject the authored change");}}}
 let packet=retained_test_packet(operation,identity,&baseline,&mut caller);let mut same=EnergyRestoreJob::admit(operation,model.clone(),config.clone(),packet,EnergyNumericalBounds::default()).unwrap();let mut sequence=0;let mut ready=false;for _ in 0..1_000_000{let mut receipt=RetainedCloneProgress::default();if step_restore(&mut same,&mut caller).unwrap(){ready=true;break;}}assert!(ready,"equal-valued independently admitted original roots must restore");close_restore(&mut same,&mut caller);
 caller.close_job(&mut original);eprintln!("[DEBUG] Energy checkpoint full Model+Config SHA256 foreign authority refused3 equal-value readmission1 and originals closed");
caller.finish();}
