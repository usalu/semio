#![cfg(test)]
#![allow(unused_imports, unused_qualifications)]

extern crate semio_s_plugin_energy_engine_calendar as calendar;
extern crate semio_s_plugin_energy_engine_kernel as kernel;
extern crate semio_s_plugin_energy_engine_meters as meters;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_output as output;
extern crate semio_s_plugin_energy_engine_precompute as precompute;
extern crate semio_s_plugin_energy_engine_results as results;
extern crate semio_s_plugin_energy_engine_site as site;
extern crate semio_s_plugin_energy_engine_sizing as sizing;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_plugin_energy_engine_sim_wire as wire;
use semio_s_plugin_energy_engine_sim::*;
use semio_s_plugin_energy_engine_sim::testkit::*;
use crate::wire::{ENERGY_WIRE_HEADER_BYTES, EnergyJobPreview, EnergyJobStage, EnergyNumericalBounds, EnergyNumericalCensus, EnergyNumericalDimension, EnergyQualityTier, EnergyWireKind, EnergyWirePacket, EnergyWireRejection, close_retained_packet, decode_preview_packet};
use crate::kernel::{SimulationConfig, SimulationEnvironment, TimestepWork};
use crate::meters::{EndUse, FuelType};
use crate::model::Model;
use crate::results::Results;
use crate::site::WeatherRecord;
use semio_framework_job::{allocate_operation_id, default_now_us, CancelToken, Generation, InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, Operation, RetainedCloneGrant, RetainedCloneProgress, RevisionId, StepBudget, StepContext};
use semio_framework_job_payload::{JobPayloadStream, RetainedJobPayload};
use std::time::Instant;

use crate::wire::ENERGY_WIRE_MAGIC;
use crate::calendar::RunPeriod;
use crate::model::{test_model_full_topology, test_model_single_zone};
use semio_s_plugin_energy_engine_precompute::PrecomputeBuilder;
use semio_framework_job::OriginalTurnFailure;
use semio_s_plugin_energy_engine_sim_wire::EnergyWireLease;

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

fn project_step(job: &mut EnergyJob, context: &mut StepContext<'_>) -> Result<Seen,OriginalTurnFailure> {
    Ok(match InteractiveJob::step(job, context)? {
        None | Some(JobOutcomeBorrow::Yield { .. }) => Seen::Yield,
        Some(JobOutcomeBorrow::PreviewReady { .. }) => Seen::Preview,
        Some(JobOutcomeBorrow::CheckpointReady { .. }) => Seen::Checkpoint,
        Some(JobOutcomeBorrow::Fault { .. }) => Seen::Fault,
        Some(JobOutcomeBorrow::Cancelled { .. }) => Seen::Cancelled,
        Some(JobOutcomeBorrow::Complete { output, .. }) => Seen::Complete(output.map(page_bytes).unwrap_or_default()),
    })
}


struct NativeEnergyJob{job:EnergyJob,original:Option<crate::testing::OriginalEnergyCaller>}
impl std::ops::Deref for NativeEnergyJob{type Target=EnergyJob;fn deref(&self)->&Self::Target{&self.job}}
impl std::ops::DerefMut for NativeEnergyJob{fn deref_mut(&mut self)->&mut Self::Target{&mut self.job}}
impl NativeEnergyJob{
 fn from_preborn(job:EnergyJob)->Self{let original=crate::testing::OriginalEnergyCaller::new(job.operation.operation,job.operation.generation);Self{job,original:Some(original)}}
 fn turn<R>(&mut self,grant:RetainedCloneGrant,fuel:u64,deadline:u64,run:impl FnOnce(&mut EnergyJob,&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{self.original.as_mut().unwrap().turn(grant,fuel,deadline,|cx|run(&mut self.job,cx))}
 fn close_on_original(&mut self,grant:RetainedCloneGrant)->InteractiveJobCloseStep{self.turn(grant,64,u64::MAX,|job,cx|cx.with_original_retirement(|cx|job.close_step(cx))).expect("native exact original Energy close")}
 fn close_packet(&mut self,packet:&mut EnergyWirePacket){self.original.as_mut().unwrap().close_packet(packet);}
 fn acknowledge(&mut self,lease:&mut Option<EnergyWireLease>,kind:EnergyWireKind){self.original.as_mut().unwrap().acknowledge(&mut self.job,lease,kind);}
 fn close_original(&mut self){if self.original.is_some(){self.original.as_mut().unwrap().close_job(&mut self.job);self.original.take().unwrap().finish();}}
}
impl Drop for NativeEnergyJob{fn drop(&mut self){if !std::thread::panicking(){self.close_original();}}}
#[derive(Clone,Copy)]
enum Probe{Original,Cancelled,Foreign(Generation)}
fn fixture_step(job:&mut NativeEnergyJob,budget:StepBudget,probe:Probe)->Seen{match probe{Probe::Original=>job.turn(budget.retained,budget.fuel,budget.deadline_us,|job,cx|project_step(job,cx)).expect("same original numerical step"),other=>{let generation=match other{Probe::Foreign(generation)=>generation,_=>job.operation.generation};let mut original=crate::testing::OriginalEnergyCaller::new(job.operation.operation,generation);if matches!(other,Probe::Cancelled){original.cancel()}let observed=original.turn(budget.retained,budget.fuel,budget.deadline_us,|cx|project_step(&mut job.job,cx)).expect("authentic negative source probe");original.finish();observed}}}
fn close_owned_job(job:&mut NativeEnergyJob){job.close_original();}



fn prepare_input_identity(job:&mut NativeEnergyJob){for _ in 0..1_000_000{if job.turn(ENERGY_BATCH_GRANT,1,u64::MAX,|job,cx|Ok(job.step_input_identity(cx))).expect("same original input identity source").expect("complete input identity semantic refusal"){return}}panic!("input identity did not finish within authored fixture bound")}

fn drive_energy_job(job: NativeEnergyJob) -> (NativeEnergyJob, Vec<EnergyJobPreview>, usize, Vec<u8>, std::time::Duration) {
    drive_energy_job_with_fuel(job, 1)
}

fn drive_energy_job_with_fuel(mut job: NativeEnergyJob, fuel: u64) -> (NativeEnergyJob, Vec<EnergyJobPreview>, usize, Vec<u8>, std::time::Duration) {
    let operation = job.operation.operation;
    let generation = job.operation.generation;
    let mut previews = Vec::new();
    let mut checkpoints = 0;
    let mut worst = std::time::Duration::ZERO;
    let mut completed_bytes=None;
    for _ in 0..50_000 {
        let start = Instant::now();

        let outcome = fixture_step(&mut job,StepBudget::new(fuel, u64::MAX, ENERGY_BATCH_GRANT),Probe::Original);
        let elapsed = start.elapsed();
        worst = worst.max(elapsed);
        if let Some(mut lease)=job.take_commit_packet(generation).unwrap(){completed_bytes=Some(page_bytes(&lease.packet().payload));job.close_packet(lease.packet_mut());let mut lease=Some(lease);job.acknowledge(&mut lease,EnergyWireKind::Commit);}
        match outcome {
            Seen::Preview => {
                let mut packet = job.take_preview_packet(generation).expect("fresh preview generation").expect("preview packet");
                previews.push(packet.preview().expect("typed projection belongs to canonical preview packet").clone());
                job.close_packet(&mut packet);
            }
            Seen::Checkpoint => {
                let mut lease = job.take_checkpoint_packet(generation).expect("fresh checkpoint generation").expect("checkpoint lease");
                job.close_packet(lease.packet_mut());
                let mut lease=Some(lease);job.acknowledge(&mut lease,EnergyWireKind::Checkpoint);
                checkpoints += 1;
            }
            Seen::Complete(bytes) => return (job, previews, checkpoints, completed_bytes.unwrap_or(bytes), worst),
            Seen::Fault => panic!("energy job faulted"),
            Seen::Cancelled => panic!("energy job unexpectedly cancelled"),
            Seen::Yield => {}
        }
    }
    panic!("energy job did not complete within the deterministic step bound")
}

/// 🧱️ `test_model_single_zone` with the one thermostat the fixture lacks, so the ideal loads actually
/// deliver heat and the zone heating meter is non-zero.
fn thermostatted_single_zone() -> (Model, SimulationConfig) {
    use crate::model::{EntityId, Thermostat};
    use crate::schedule::ScheduleId;
    use crate::schedule::{ConstantSchedule, ScheduleSet};
    let mut model = test_model_single_zone();
    model.thermostats.push(Thermostat {
        id: EntityId(50),
        zone_id: EntityId(1),
        heating_setpoint_schedule_id: ScheduleId(1),
        cooling_setpoint_schedule_id: ScheduleId(2),
        heating_throttle_range_k: 2.0,
        cooling_throttle_range_k: 2.0,
    });
    let config = SimulationConfig {
        warmup_days: 3,
        run_period_end_month: 1,
        run_period_end_day: 1,
        environment: SimulationEnvironment::HeatingDesignDay,
        schedules: ScheduleSet { constants: vec![ConstantSchedule { id: ScheduleId(1), value: 20.0 }, ConstantSchedule { id: ScheduleId(2), value: 27.0 }], ..ScheduleSet::default() },
        ..Default::default()
    };
    (model, config)
}

#[test]
fn p7c3_commit_lease_ack_is_the_exact_terminal_detach_witness() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(121), Generation(17), 0x7c3);
    let mut job = NativeEnergyJob::from_preborn(EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("terminal witness admission"));
    job.stage = EnergyJobStage::Complete;
    let identity = job.wire_identity(1);
    job.publication
        .commits
        .push(EnergyWirePacket { kind: EnergyWireKind::Commit, identity, payload: RetainedJobPayload::empty(JobPayloadStream::CommitOutput), preview: None, reservation: None })
        .expect("prepared exact commit");

    let lease = job.take_commit_packet(operation.generation).expect("fresh generation").expect("exact final lease");

    assert_eq!(fixture_step(&mut job,StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT),Probe::Original), Seen::Yield, "an unacknowledged exact lease must retain terminal ownership");
    assert_eq!(job.publication.commits.len, 1);
    assert!(job.publication.commits.in_flight.is_some());

    let mut lease=Some(lease);job.acknowledge(&mut lease,EnergyWireKind::Commit);
    assert_eq!(job.publication.commits.len, 0);
    assert!(job.publication.commits.in_flight.is_none());

    let terminal = fixture_step(&mut job,StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT),Probe::Original);
    assert_eq!(terminal, Seen::Complete(Vec::new()), "the exact ACK must make the numerical authority terminal with an empty lent output");
    assert!(job.commit_output.is_none(), "the consumer retained and closed the only commit owner before terminal detach");

    close_owned_job(&mut job);
}

#[test]
fn p7c2_preview_typed_view_is_derived_from_canonical_wire_with_live_facility_total() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(96), Generation(15), 0xcafe);
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let mut job = NativeEnergyJob::from_preborn(EnergyJob::new(operation, test_model_full_topology(), config).expect("preview source admission"));
    for _ in 0..50_000 {

        match fixture_step(&mut job,StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT),Probe::Original) {
            Seen::Preview => {
                let projected = job.preview().expect("installed packet owns typed preview projection").clone();
                let mut packet = job.take_preview_packet(operation.generation).expect("fresh preview generation").expect("canonical preview packet");
                assert_eq!(packet.payload.len(), ENERGY_WIRE_HEADER_BYTES + 20);
                let decoded = decode_preview_packet(&packet).expect("SMENERGY preview schema");
                assert_eq!(packet.preview(), Some(&decoded));
                assert_eq!(projected, decoded);
                job.close_packet(&mut packet);
                if decoded.facility_electricity_kwh > 0.0 {
                    close_owned_job(&mut job);
                    return;
                }
            }
            Seen::Checkpoint => {
                let mut lease = job.take_checkpoint_packet(operation.generation).expect("checkpoint generation").expect("checkpoint lease");
                job.close_packet(lease.packet_mut());
                let mut lease=Some(lease);job.acknowledge(&mut lease,EnergyWireKind::Checkpoint);
            }
            Seen::Yield => {}
            Seen::Fault => panic!("preview source faulted"),
            Seen::Cancelled => panic!("preview source cancelled"),
            Seen::Complete(_) => break,
        }
    }
    panic!("canonical preview never exposed a substantive retained facility total")
}

#[test]
fn p7c2_cancel_deadline_and_stale_generation_gate_wire_before_mutation() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(92), Generation(5), 99);
    let mut job = NativeEnergyJob::from_preborn(EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("admission"));
    prepare_input_identity(&mut job);
    job.start_wire(EnergyWireKind::Checkpoint, 0, JobPayloadStream::CheckpointState).expect("wire start");
    let before = job.wire_build.as_ref().expect("wire").field;

    assert_eq!(fixture_step(&mut job,StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT),Probe::Cancelled), Seen::Cancelled);
    assert_eq!(job.wire_build.as_ref().expect("wire").field, before);

    assert_eq!(fixture_step(&mut job,StepBudget::new(0, 0, ENERGY_BATCH_GRANT),Probe::Original), Seen::Yield);
    assert_eq!(job.wire_build.as_ref().expect("wire").field, before);
    assert!(matches!(job.take_checkpoint_packet(Generation(6)), Err(EnergyWireRejection::Identity)));
}

#[test]
fn engine_runs_single_zone() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 1, run_period_end_month: 1, run_period_end_day: 3, environment: SimulationEnvironment::WeatherRunPeriod, ..Default::default() };
    let results = crate::testing::run_preborn(model, config).unwrap();
    assert!(results.run_metadata.timesteps > 0);
    assert!(results.meters.facility_total_kwh(FuelType::Electricity) >= 0.0);
}

#[test]
fn engine_deterministic_repeatability() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
    let r1 = crate::testing::run_preborn(model.clone(), config.clone()).unwrap();
    let r2 = crate::testing::run_preborn(model, config).unwrap();
    assert_eq!(r1.run_metadata.timesteps, r2.run_metadata.timesteps);
    assert!((r1.meters.facility_total_kwh(FuelType::Electricity) - r2.meters.facility_total_kwh(FuelType::Electricity)).abs() < 1e-3);
}

#[test]
fn ashrae_140_case600_base() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let results = crate::testing::run_preborn(model, config).unwrap();
    let temps = results.time_series.get("Zone Air Temperature [Zone1]");
    assert!(temps.is_some());
}

#[test]
fn invalid_model_rejected() {
    let model = Model::default();
    assert!(crate::testing::run_preborn(model, SimulationConfig::default()).is_err());
}

#[test]
fn energy_conservation_order_of_magnitude() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
    let results = crate::testing::run_preborn(model, config).unwrap();
    let total_kwh = results.meters.facility_total_kwh(FuelType::Electricity);
    assert!(total_kwh < 1_000_000.0);
}

#[test]
fn full_topology_e2e() {
    let model = test_model_full_topology();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 2, ..Default::default() };
    let results = crate::testing::run_preborn(model, config).unwrap();
    assert!(results.run_metadata.timesteps >= 48);
    assert!(results.summaries.annual_energy.len() >= 3);
}

#[test]
fn hvac_bestest_heating_day() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let results = crate::testing::run_preborn(model, config).unwrap();
    assert_eq!(results.run_metadata.timesteps, 24);
    assert!(results.time_series.get("Zone Air Temperature [Zone1]").is_some());
}

#[test]
fn run_period_honors_calendar() {
    let period = RunPeriod { start_month: 1, start_day: 1, end_month: 1, end_day: 7, year: 2026 };
    assert_eq!(period.total_hours(), 168);
    let config = SimulationConfig { run_period_start_month: 1, run_period_start_day: 1, run_period_end_month: 1, run_period_end_day: 7, warmup_days: 0, ..Default::default() };
    let model = test_model_single_zone();
    let results = crate::testing::run_preborn(model, config).unwrap();
    assert_eq!(results.run_metadata.timesteps, 168);
}

#[test]
fn energy_job_previews_checkpoints_and_commits_bounded_steps() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 1, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let (mut job, previews, checkpoints, output, worst) = drive_energy_job(NativeEnergyJob::from_preborn(Engine::job(model, config).expect("energy admission")));
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
    let mut cancelled_job = NativeEnergyJob::from_preborn(EnergyJob::new(operation, model.clone(), config.clone()).expect("energy admission"));

    assert_eq!(fixture_step(&mut cancelled_job,StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT),Probe::Cancelled), Seen::Cancelled);
    assert_eq!(cancelled_job.stage(), EnergyJobStage::Validate);

    let mut stale_job = NativeEnergyJob::from_preborn(EnergyJob::new(operation, model, config).expect("energy admission"));

    assert_eq!(fixture_step(&mut stale_job,StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT),Probe::Foreign(Generation(4))), Seen::Fault);
    assert_eq!(stale_job.stage(), EnergyJobStage::Validate);
}

#[test]
fn p7c1_exact_max_rejection_preserves_and_retries_same_owner() {
    let model = test_model_full_topology();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let census = EnergyNumericalCensus::observe(&model, &config).expect("checked census");
    let operation = Operation::new(allocate_operation_id(), RevisionId(17), Generation(9), 31);
    assert!(EnergyJob::admit(operation, model.clone(), config.clone(), EnergyNumericalBounds(census)).map(NativeEnergyJob::from_preborn).is_ok(), "exact observed MAX must succeed");

    let pointer = model.zones.as_ptr();
    let mut maximum = census;
    maximum.zones -= 1;
    let rejected = EnergyJob::admit(operation, model, config, EnergyNumericalBounds(maximum)).expect_err("MAX+1 zone owner must reject before transfer");
    assert_eq!(rejected.dimension, EnergyNumericalDimension::Zones);
    assert_eq!(rejected.model.zones.as_ptr(), pointer, "rejected authority retains the exact allocation identity");
    assert!(rejected.retry(EnergyNumericalBounds(census)).map(NativeEnergyJob::from_preborn).is_ok(), "exact rejected owner retries after sufficient credit");
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
    let mut records = vec![site::design_day_hour(0, -10.0), site::design_day_hour(1, -9.0)];
    records.shrink_to_fit();
    let logical_records = records.len().max(1);
    records.reserve_exact(3);
    assert!(records.capacity() > logical_records, "reserve-only mutation must increase owned weather backing");
    let pointer = records.as_ptr();
    let config = SimulationConfig { weather: Some(site::WeatherData { calendar: crate::site::WeatherCalendar::ActualDates, location: "fixed".into(), latitude_deg: 0.0, longitude_deg: 0.0, elevation_m: 0.0, time_zone_hours: 0.0, records }), ..Default::default() };
    let census = EnergyNumericalCensus::observe(&model, &config).expect("weather census");
    assert_eq!(census.weather_records, config.weather.as_ref().expect("weather").records.capacity());
    assert!(census.weather_records > logical_records, "census must not collapse backing capacity to len/max1");
    let operation = Operation::new(allocate_operation_id(), RevisionId(63), Generation(12), 251);
    let mut maximum = EnergyNumericalBounds::default().0;
    maximum.weather_records = logical_records;
    let rejected = EnergyJob::admit(operation, model, config, EnergyNumericalBounds(maximum)).expect_err("weather MAX+1 rejects exact owner");
    assert_eq!(rejected.dimension, EnergyNumericalDimension::WeatherRecords);
    assert_eq!(rejected.config.weather.as_ref().expect("weather owner").records.as_ptr(), pointer);
    let mut job = NativeEnergyJob::from_preborn(rejected.retry(EnergyNumericalBounds::default()).expect("weather owner retry"));
    assert_eq!(job.weather.capacity(), census.weather_records);
    assert_eq!(job.weather.len(), 0);
    job.stage = EnergyJobStage::ResolveWeather;
    for _ in 0..2 {

        assert_eq!(fixture_step(&mut job,StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT),Probe::Original), Seen::Yield);
    }
    assert_eq!(job.weather.len(), 2);
    assert_eq!(job.weather.capacity(), census.weather_records);
    let before_records = [*job.weather.get_index(0).expect("weather zero"), *job.weather.get_index(1).expect("weather one")];
    for index in logical_records..census.weather_records {
        assert!(job.weather.insert_stable(index, site::design_day_hour(index as u32, -8.0)).is_ok());
    }
    assert!(job.weather.insert_stable(census.weather_records, site::design_day_hour(census.weather_records as u32, -8.0)).is_err());
    assert_eq!(job.weather.len(), census.weather_records);
    assert_eq!(job.weather.capacity(), census.weather_records);
    assert_eq!(*job.weather.get_index(0).expect("weather zero"), before_records[0]);
    assert_eq!(*job.weather.get_index(1).expect("weather one"), before_records[1]);
    job.weather_cursor = 1;
    let mut outcome = Seen::Yield;
    for _ in 0..1024 {

        outcome = fixture_step(&mut job,StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT),Probe::Original);
        if outcome == Seen::Fault {
            break;
        }
    }
    assert_eq!(outcome, Seen::Fault);
    assert_eq!(job.weather_fault, Some(WeatherFault::SlotRejected));
    close_owned_job(&mut job);
}

#[test]
fn p7c1_weather_reserve_only_capacity_is_independently_charged_to_items() {
    let model = test_model_single_zone();
    let mut config =
        SimulationConfig { weather: Some(crate::site::WeatherData { calendar: crate::site::WeatherCalendar::ActualDates, location: "items".into(), latitude_deg: 0.0, longitude_deg: 0.0, elevation_m: 0.0, time_zone_hours: 0.0, records: vec![design_day_hour(0, -10.0)] }), ..Default::default() };
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
    let mut job = NativeEnergyJob::from_preborn(rejected.retry(EnergyNumericalBounds::default()).expect("weather item owner retry"));
    close_owned_job(&mut job);
}

#[test]
fn p7c1_one_fuel_chronology_is_identical_at_one_two_four_and_default_grants() {
    let model = test_model_single_zone();
    let config = SimulationConfig { warmup_days: 0, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let operation = Operation::new(allocate_operation_id(), RevisionId(21), Generation(4), 91);
    let output = |fuel| drive_energy_job_with_fuel(NativeEnergyJob::from_preborn(EnergyJob::new(operation, model.clone(), config.clone()).expect("energy admission")), fuel).3;
    assert_eq!(output(1), output(2));
    assert_eq!(output(1), output(4));
    assert_eq!(output(1), output(32));
}

#[test]
fn p7c1_close_releases_no_more_than_one_owner_or_character_per_grant(){
 let operation=Operation::new(allocate_operation_id(),RevisionId(1),Generation(1),1);let mut job=EnergyJob::new(operation,test_model_full_topology(),SimulationConfig::default()).expect("energy admission");let mut original=crate::testing::OriginalEnergyCaller::new(operation.operation,operation.generation);job.begin_close();
 for _ in 0..100_000{if job.terminal_is_empty(){original.finish();return}let demand=job.original_close_demands().unwrap();let grant=energy_self_funded(demand);if demand.release_bytes>4{let denied=RetainedCloneGrant{maximum_release_bytes:4,..grant};let before=job.original_close_demands().unwrap();let step=original.turn(denied,1,u64::MAX,|cx|job.close_step(cx)).expect("original subexact owner denial");assert_eq!(step.progress(),Default::default());assert_eq!(job.original_close_demands().unwrap(),before);}let step=original.turn(grant,1,u64::MAX,|cx|job.close_step(cx)).expect("same original exact numerical frontier");assert!(step.progress().copied_items<=1);assert!(step.progress().released_bytes<=grant.maximum_release_bytes);assert!(step.progress().fits(grant));}panic!("Energy exact close did not retire within its admitted bound")
}

#[test]
fn p7c1_direct_drop_requeues_exact_generation_and_resumes_partial_close(){
 let operation=Operation::new(allocate_operation_id(),RevisionId(33),Generation(7),101);let mut job=EnergyJob::new(operation,test_model_full_topology(),SimulationConfig::default()).expect("energy admission");let mut original=crate::testing::OriginalEnergyCaller::new(operation.operation,operation.generation);let retained_pointer=job.model.zones.as_ptr();job.begin_close();let grant=energy_self_funded(job.original_close_demands().unwrap());assert!(matches!(original.turn(grant,1,u64::MAX,|cx|job.close_step(cx)).unwrap(),InteractiveJobCloseStep::Pending{..}));drop(job);
 let mut recovered=EnergyJob::recover_abandoned(operation).expect("same generation abandonment authority");assert_eq!(recovered.model.zones.as_ptr(),retained_pointer);assert!(EnergyJob::recover_abandoned(Operation{generation:Generation(8),..operation}).is_none());original.close_job(&mut recovered);assert!(recovered.terminal_is_empty());original.finish();
}

#[test]
fn p7c1_panic_unwind_requeues_the_same_incomplete_authority_once() {
    let operation = Operation::new(allocate_operation_id(), RevisionId(34), Generation(11), 131);
    let job = EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("energy admission");
    let mut original=crate::testing::OriginalEnergyCaller::new(operation.operation,operation.generation);
    let retained_pointer = job.model.zones.as_ptr();
    let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let _authority = job;
        panic!("hostile Energy session unwind");
    }));
    assert!(unwind.is_err());
    let mut recovered = EnergyJob::recover_abandoned(operation).expect("panic requeued exact authority");
    assert_eq!(recovered.model.zones.as_ptr(), retained_pointer);
    assert!(EnergyJob::recover_abandoned(operation).is_none(), "recovery is single-owner");
    original.close_job(&mut recovered);original.finish();
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
    let mut job = NativeEnergyJob::from_preborn(EnergyJob::new(operation, test_model_single_zone(), SimulationConfig::default()).expect("energy admission"));
    for stage in stages {
        job.stage = stage;
        let before = job.numerical_cursor_signature();

        assert_eq!(fixture_step(&mut job,StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT),Probe::Cancelled), Seen::Cancelled);
        assert_eq!(job.numerical_cursor_signature(), before, "cancel mutated {stage:?}");


        assert_eq!(fixture_step(&mut job,StepBudget::new(0, 0, ENERGY_BATCH_GRANT),Probe::Original), Seen::Yield);
        assert_eq!(job.numerical_cursor_signature(), before, "deadline mutated {stage:?}");
    }
    close_owned_job(&mut job);
}

#[test]
fn p7c1_live_nested_authorities_gate_cancel_deadline_and_stale_before_mutation() {
    use crate::kernel::{P7C1_PLANT_STAGES, P7C1_SCHEDULE_LOOKUP_STAGES, P7C1_BALANCE_STAGES, P7C1_TIMESTEP_BUILDER_STAGES, P7C1_TIMESTEP_STAGES, P7C1_ZONE_PREPARATION_STAGES};
    use crate::precompute::P7C1_PRECOMPUTE_STAGES;
    use crate::sizing::P7C1_SIZING_STAGES;

    fn assert_gates(job: &mut NativeEnergyJob, operation: Operation) {
        let before = job.numerical_cursor_signature();

        assert_eq!(fixture_step(job,StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT),Probe::Cancelled), Seen::Cancelled);
        assert_eq!(job.numerical_cursor_signature(), before);


        assert_eq!(fixture_step(job,StepBudget::new(0, 0, ENERGY_BATCH_GRANT),Probe::Original), Seen::Yield);
        assert_eq!(job.numerical_cursor_signature(), before);


        assert_eq!(fixture_step(job,StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT),Probe::Foreign(Generation(operation.generation.0 + 1))), Seen::Fault);
        assert_eq!(job.numerical_cursor_signature(), before);
    }

    let operation = Operation::new(allocate_operation_id(), RevisionId(61), Generation(9), 241);
    let config = SimulationConfig { warmup_days: 1, run_period_end_month: 1, run_period_end_day: 1, environment: SimulationEnvironment::HeatingDesignDay, ..Default::default() };
    let mut job = NativeEnergyJob::from_preborn(EnergyJob::new(operation, test_model_full_topology(), config).expect("live nested gate admission"));
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
    let mut precompute_gated = false;
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
            if !system_gated && job.timestep_work.as_ref().and_then(TimestepWork::balance_stage).is_some() {
                let original = job.timestep_work.as_ref().and_then(TimestepWork::balance_stage).expect("balance stage");
                for stage in P7C1_BALANCE_STAGES {
                    assert!(job.timestep_work.as_mut().expect("timestep").set_balance_stage_for_gate(stage));
                    assert_gates(&mut job, operation);
                }
                job.timestep_work.as_mut().expect("timestep").set_balance_stage_for_gate(original);
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
        if precompute_gated && timestep_builder_gated && timestep_gated && zone_gated && system_gated && plant_gated && schedule_gated && warmup_gated && aggregate_zone_gated && aggregate_facility_gated && sizing_gated {
            close_owned_job(&mut job);
            return;
        }

        let _ = fixture_step(&mut job,StepBudget::new(1, u64::MAX, ENERGY_BATCH_GRANT),Probe::Original);
    }
    panic!("live nested cursor family was never mounted")
}

#[test]
fn per_surface_conduction_losses_close_the_zone_air_balance_against_the_heating_meter() {
    let (model, config) = thermostatted_single_zone();
    let results = crate::testing::run_preborn(model, config).expect("heating design day runs");

    assert_eq!(results.per_surface.len(), 1, "one opaque surface, no fenestrations");
    let wall = results.per_surface[0];
    assert_eq!(wall.id, model::EntityId(30), "the row is keyed by the surface's own EntityId");
    assert!(wall.conduction_loss_kwh > 0.0, "a 20 °C zone behind a −10 °C design day must lose heat through its one wall, got {wall:?}");

    let net_loss_kwh: f64 = results.per_surface.iter().map(|row| row.conduction_loss_kwh - row.conduction_gain_kwh).sum();
    let heating = results.meters.meters.get_index(0).expect("the zone's heating meter is the first admitted slot");
    assert_eq!(heating.end_use, EndUse::Heating);
    let heating_kwh = heating.energy_kwh();
    assert!(heating_kwh > 0.0, "the ideal loads must have delivered heat, got {heating_kwh}");

    let relative = (net_loss_kwh - heating_kwh).abs() / heating_kwh.max(1e-9);
    assert!(relative <= 1e-6, "per-surface conduction {net_loss_kwh} kWh vs heating meter {heating_kwh} kWh is {:.6} % apart", relative * 100.0);
}

#[test]
fn warmup_never_contributes_to_the_per_surface_totals() {
    let (model, short) = thermostatted_single_zone();
    let long = SimulationConfig { warmup_days: short.warmup_days + 4, ..short.clone() };
    let with_short_warmup = crate::testing::run_preborn(model.clone(), short).expect("short warmup runs");
    let with_long_warmup = crate::testing::run_preborn(model, long).expect("long warmup runs");
    let total = |results: &Results| results.per_surface.iter().map(|row| row.conduction_loss_kwh).sum::<f64>();
    let (a, b) = (total(&with_short_warmup), total(&with_long_warmup));
    assert!(a > 0.0 && b > 0.0);
    assert!((a - b).abs() / a.max(1e-9) < 0.05, "warmup days leak into the run-period totals: {a} vs {b}");
}



/// 🌦️ The original retained job admits authored rows, selects next-day sources and refuses gaps before precompute.
#[test]
fn weather_job_original48_midnight_and_chronology_refusals_match_neutral_sources(){
 let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../../📍️site/📅️chronology/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 for case in fixture["cases"].as_array().unwrap(){let records:Vec<WeatherRecord>=serde_json::from_value(case["records"].clone()).unwrap();let policy:crate::site::WeatherCalendar=serde_json::from_value(case["calendar"].clone()).unwrap();let p:RunPeriod=serde_json::from_value(case["run"].clone()).unwrap();let original=records.clone();let config=SimulationConfig{run_period_year:p.year,run_period_start_month:p.start_month,run_period_start_day:p.start_day,run_period_end_month:p.end_month,run_period_end_day:p.end_day,warmup_days:0,weather:Some(crate::site::WeatherData{calendar:policy,location:"neutral original".into(),latitude_deg:52.37,longitude_deg:9.74,elevation_m:55.0,time_zone_hours:1.0,records}),..Default::default()};let mut job=NativeEnergyJob::from_preborn(Engine::job(test_model_single_zone(),config).unwrap());let operation=job.operation;job.stage=EnergyJobStage::ResolveWeather;let mut sequence=0;
 for _ in 0..original.len()+2{let mut receipt=RetainedCloneProgress::default();let _=fixture_step(&mut job,semio_framework_job::StepBudget::new(1,u64::MAX,ENERGY_BATCH_GRANT),Probe::Original);if job.weather_fault.is_some()||job.stage!=EnergyJobStage::ResolveWeather{break;}}assert_eq!(job.weather_fault.is_none(),case["runAdmitted"].as_bool().unwrap(),"{}",case["id"]);assert_eq!(job.config.weather.as_ref().unwrap().records,original);
 if job.weather_fault.is_none(){assert_eq!(job.stage,EnergyJobStage::Precompute);for query in case["queries"].as_array().unwrap(){if query["expected"].is_null(){continue;}let date=&query["date"];let selected=job.weather_index(crate::calendar::SimDate::new(date[0].as_u64().unwrap()as u16,date[1].as_u64().unwrap()as u8,date[2].as_u64().unwrap()as u8),query["hour"].as_u64().unwrap()as u8).unwrap();let actual=job.hour_weather(selected,query["policy"]=="RepeatDay").unwrap();for(i,row)in[actual.previous,actual.current,actual.next].iter().enumerate(){assert_eq!(*row,original[query["expected"][i].as_u64().unwrap()as usize]);}}}
 close_owned_job(&mut job);}
 eprintln!("[DEBUG] Energy original retained weather10 sources midnight/leap/rollover/refusal posts preserved native source authority");
}

/// 🪪️ Genuine complete input projections match Serde's independent fields and retain exact paid cursor ownership.
#[test]
fn input_identity_original_full_fields_match_serde_and_paid_cancellation_frontiers(){
 use semio_framework_os_kernel::os_store::{ArtifactCanonicalSourceIdentityCursor as Cursor,ArtifactCanonicalTreeSource,ArtifactCanonicalJson};
 let fixture:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../../📍️site/📅️chronology/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let records:Vec<WeatherRecord>=serde_json::from_value(fixture["cases"][0]["records"].clone()).unwrap();
 let mut model=test_model_single_zone();model.name="München 🪪️ authored model".into();
 let config=SimulationConfig{weather:Some(crate::site::WeatherData{calendar:crate::site::WeatherCalendar::ActualDates,location:"full original48".into(),latitude_deg:52.37,longitude_deg:9.74,elevation_m:55.0,time_zone_hours:1.0,records}),..Default::default()};
 let mut expected=b"semio.canonical-source-identity/v1".to_vec();expected.push(0);expected.extend_from_slice(&2u64.to_be_bytes());serde_json::to_writer(&mut expected,&(&model,&config)).unwrap();
 let(mut cursor,_)=Cursor::admit(2,energy_self_funded(Cursor::constructor_demand())).unwrap().unwrap();let mut actual=Vec::with_capacity(expected.len());
 for _ in 0..1_000_000{let source:Option<&dyn ArtifactCanonicalJson>=match cursor.source_index(){Some(0)=>Some(ArtifactCanonicalTreeSource::view(&model)),Some(1)=>Some(ArtifactCanonicalTreeSource::view(&config)),None=>None,_=>unreachable!()};let step=cursor.advance(source,energy_self_funded(cursor.next_demand())).unwrap();if let Some(byte)=step.byte{actual.push(byte);}if step.ready{break;}}
 assert!(cursor.is_ready());assert_eq!(actual,expected,"all original fields and authored UTF8 match third-party Serde JSON bytes");let expected_digest=*cursor.digest().unwrap();cursor.begin_close();cursor.close_step(energy_self_funded(cursor.next_demand())).unwrap();drop(cursor);
 for stop in [None,Some(0),Some(1),Some(17),Some(113),Some(1000)]{let mut job=NativeEnergyJob::from_preborn(Engine::job(model.clone(),config.clone()).unwrap());let pointers=(&job.model as*const Model as usize,&job.config as*const SimulationConfig as usize,job.config.weather.as_ref().unwrap().records.as_ptr());let mut completed=false;
 for turn in 0..1_000_000{if stop==Some(turn){break;}let demand=job.input_identity.next_demand();assert!(demand.copy_bytes<=4096&&demand.capacity_bytes<=4096);let denied=RetainedCloneGrant{maximum_items:0,..energy_self_funded(demand)};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let authority=job.authority.as_mut().unwrap();authority.input_identity.advance(&authority.model,&authority.config,denied).unwrap()});assert_eq!(step,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let grant=energy_self_funded(demand);let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let authority=job.authority.as_mut().unwrap();authority.input_identity.advance(&authority.model,&authority.config,grant).unwrap()});assert!(step.fits(grant));assert_ne!(step,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(step.retained_capacity_bytes,step.released_bytes));assert_eq!(pointers,(&job.model as*const Model as usize,&job.config as*const SimulationConfig as usize,job.config.weather.as_ref().unwrap().records.as_ptr()));if let Some(digest)=job.input_identity.digest(){assert_eq!(*digest,expected_digest);assert_eq!(job.cursor().input_identity_bytes,expected.len());assert!(job.cursor().input_identity_ready);completed=true;break;}}
 assert_eq!(completed,stop.is_none());while !job.input_identity.terminal_is_empty(){let demand=job.input_identity.close_demand();assert!(demand.copy_bytes<=4096&&demand.release_bytes<=4096);let denied=RetainedCloneGrant{maximum_items:0,..energy_self_funded(demand)};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.input_identity.close_step(denied).unwrap());assert_eq!(step,Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let grant=energy_self_funded(demand);let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.input_identity.close_step(grant).unwrap());assert!(step.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.retained_capacity_bytes,step.released_bytes));}close_owned_job(&mut job);}
 eprintln!("[DEBUG] Energy full Model+Config original projections match Serde48 weather fields and6 paid cancellation/allocation/source-pointer prefixes");
}
/// ♻️ Real Energy cancellation delegates original builder/model custody and every physical release to shared quotes.
#[test]
fn energy_precompute_close_delegates_original_factory_quotes_and_physical_receipts(){
 let corpus:serde_json::Value=serde_json::from_str(semio_framework_test_fixtures::fixture_text!("../../../../../🧠️precompute/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let mut observed_turns=0;
 for case in corpus["cases"].as_array().unwrap(){for cutoff in corpus["cutoffs"].as_array().unwrap(){
  let model=match case["model"].as_str().unwrap(){"empty"=>Model::default(),"single"=>test_model_single_zone(),"600"=>semio_s_plugin_energy_engine_bestest::model("600").unwrap(),_=>unreachable!()};let mut builder=PrecomputeBuilder::new(10,10);let limit=if case["complete"].as_bool().unwrap(){usize::MAX}else{cutoff.as_u64().unwrap() as usize};let mut steps=0;while !builder.is_complete()&&steps<limit{builder.step(&model);steps+=1;assert!(steps<100_000);}
  let mut job=NativeEnergyJob::from_preborn(Engine::job(model,SimulationConfig::default()).unwrap());if case["complete"].as_bool().unwrap(){job.pre=Some(builder.finish());}else{job.precompute=Some(builder);}job.begin_close();
  for _ in 0..100_000{if job.input_identity.terminal_is_empty(){break;}let demand=job.close_demands().unwrap();let step=job.close_on_original(energy_self_funded(demand));assert!(matches!(step,InteractiveJobCloseStep::Pending{..}|InteractiveJobCloseStep::Complete{..}));}assert!(job.input_identity.terminal_is_empty());
  while job.precompute.is_some()||job.pre.is_some()||!job.precompute_close.is_empty(){
   let demand=job.close_demands().unwrap();assert_eq!(job.precompute_close_demands(0).unwrap().unwrap(),demand);let full=energy_self_funded(demand);
   for axis in 0..5{let mut denied=full;match axis{0=>denied.maximum_items=0,1 if demand.copy_bytes>0=>denied.maximum_copy_bytes-=1,2 if demand.capacity_bytes>0=>denied.maximum_capacity_bytes-=1,3 if demand.release_bytes>0=>denied.maximum_release_bytes-=1,4 if demand.depth>0=>denied.maximum_depth-=1,_=>continue};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_on_original(denied));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));match step{InteractiveJobCloseStep::Pending{progress}=>assert_eq!(progress,RetainedCloneProgress::default()),_=>panic!("denied original precompute frontier changed state")};assert_eq!(job.close_demands().unwrap(),demand);}
   let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||job.close_on_original(full));assert!(!heap.overflowed);let progress=match step{InteractiveJobCloseStep::Pending{progress}|InteractiveJobCloseStep::Complete{progress}=>progress,_=>panic!("quoted original precompute frontier refused")};assert!(progress.fits(full));assert!(heap.requested_bytes<=progress.retained_capacity_bytes);assert!(heap.released_bytes<=progress.released_bytes);observed_turns+=1;assert!(observed_turns<1_000_000);
  }
  close_owned_job(&mut job);assert!(job.terminal_is_empty());
 }}
 eprintln!("[DEBUG] Energy original precompute builders/models delegated {observed_turns} actual physical paid retirement turns");
}
