use super::*;
use semio_framework_plugin::{Locale, Terminology};
use semio_framework_tool_run::{ToolRunId, ToolRunTick};

const RUN_SCHEMA: &str = include_str!("../../../✏️editor/🧵️simulation-session/🔣️.json");
const RUN_FIXTURE: &str = include_str!("../../../✏️editor/🧵️simulation-session/🧫️fixtures/🔣️.json");

fn fixture() -> serde_json::Value {
    serde_json::from_str(RUN_FIXTURE).expect("run fixture parses")
}

fn number(value: &serde_json::Value) -> u64 {
    value.as_u64().unwrap_or_else(|| panic!("fixture number expected, found {value}"))
}

/// 🧫️ The fixture scenario: the ANSI/ASHRAE 140 case model with the fixture run period and settings.
fn scenario() -> (Arc<EnergyModelSnapshot>, SimulationConfig) {
    let fixture = fixture();
    let scenario = &fixture["scenario"];
    assert_eq!(scenario["example"], "bestest-600");
    let mut model = crate::examples::bestest_600::model();
    let period = &scenario["runPeriod"];
    model.run_period.start_month = number(&period["startMonth"]) as u8;
    model.run_period.start_day = number(&period["startDay"]) as u8;
    model.run_period.end_month = number(&period["endMonth"]) as u8;
    model.run_period.end_day = number(&period["endDay"]) as u8;
    let settings = &scenario["settings"];
    let template = SimulationConfig { zone_timestep_minutes: number(&settings["zoneTimestepMinutes"]) as u32, system_timestep_minutes: number(&settings["systemTimestepMinutes"]) as u32, warmup_days: number(&settings["warmupDays"]) as u32, ..SimulationConfig::default() };
    (Arc::new(crate::energy_snapshot_with_state(crate::ENERGY_MODEL_DOCUMENT_SCHEMA, &model, None)), template)
}

fn identity() -> ToolRunIdentity {
    ToolRunIdentity::new(ToolRunId { app_instance_id: 7, run: 1 }, [3; 32])
}

/// 🦶️ Every observable outcome of driving a run job to settlement with a fixed fuel budget per call.
struct Drive {
    ticks: Vec<ToolRunTick>,
    settled: StepOutcome,
    calls: usize,
}

fn drive(job: &mut EnergySimulationRunJob, fuel: u64) -> Drive {
    let (operation, generation, cancel) = (semio_framework_job::allocate_operation_id(), Generation(1), semio_framework_job::root_cancel_token());
    let mut sequence = 0;
    let mut ticks = Vec::new();
    for calls in 1..=50_000_000 {
        let now = semio_framework_job::default_now_us().expect("clock");
        let budget = StepBudget::new(fuel, now + semio_framework_job::INTERACTIVE_LANE_WALL_US * 4);
        let mut verdict = None;
        match semio_framework_job::drive_step(job, "energy.simulation.run.test", operation, generation, semio_framework_job::InteractiveStage::InteractiveStep, budget, cancel.clone(), semio_framework_job::default_now_us, &mut sequence, &mut verdict) {
            StepOutcome::Yield => {}
            StepOutcome::PreviewReady(mut payload) => {
                let bytes: Vec<u8> = (0..payload.page_count()).flat_map(|index| payload.page(index).expect("tick page").to_vec()).collect();
                close_payload(&mut payload);
                ticks.push(ToolRunTick::decode(&bytes).expect("tick decodes"));
            }
            StepOutcome::CheckpointReady(_) => panic!("a restart-policy run never reports a checkpoint"),
            settled => return Drive { ticks, settled, calls },
        }
    }
    panic!("the run never settled");
}

fn close(job: &mut EnergySimulationRunJob) {
    job.begin_close();
    for _ in 0..10_000_000 {
        match job.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
            InteractiveJobCloseStep::Complete => {
                assert!(job.terminal_is_empty(), "a closed run job holds no owner");
                return;
            }
            InteractiveJobCloseStep::Pending { released_items, .. } => assert!(released_items <= 1),
            InteractiveJobCloseStep::Blocked => panic!("the run job close blocked"),
        }
    }
    panic!("the run job close never completed");
}

fn reason_ids(ticks: &[ToolRunTick]) -> Vec<&'static str> {
    ticks.iter().flat_map(|tick| tick.steps.iter()).filter_map(|step| EnergySimulationRunReason::from_code(step.reason)).map(EnergySimulationRunReason::id).collect()
}

//#region ⏯️Definition
#[test]
fn run_definition_matches_the_schema_source_of_record() {
    let schema: serde_json::Value = serde_json::from_str(RUN_SCHEMA).unwrap();
    let table = &schema["x-semio-toolRun"];
    let definition = energy_simulation_run_definition();
    definition.validate().expect("the run definition is valid");
    let text = |label: &LocalizedLabel, locale| label.resolve(Terminology::Native, locale).to_string();
    assert_eq!(definition.mutating, table["mutating"].as_bool().unwrap());
    assert_eq!(serde_json::to_value(definition.rebase).unwrap(), table["rebase"]);
    assert_eq!(serde_json::to_value(definition.reconfigure).unwrap(), table["reconfigure"]);
    assert_eq!(serde_json::to_value(definition.trace).unwrap(), table["trace"]);
    assert_eq!(definition.run_job.as_str(), table["runJob"]);
    assert!(definition.revalidate_job.is_none());
    assert_eq!(serde_json::to_value(&definition.settings).unwrap(), table["settings"]);
    assert_eq!((text(&definition.unit, Locale::En), text(&definition.unit, Locale::De)), (table["unit"]["en"].as_str().unwrap().to_string(), table["unit"]["de"].as_str().unwrap().to_string()));
    let rows = |key: &str| table[key].as_array().unwrap().clone();
    assert_eq!(definition.stages.len(), rows("stages").len());
    for (stage, row) in definition.stages.iter().zip(rows("stages")) {
        assert_eq!((stage.id.as_str(), text(&stage.label, Locale::En), text(&stage.label, Locale::De)), (row["id"].as_str().unwrap(), row["en"].as_str().unwrap().to_string(), row["de"].as_str().unwrap().to_string()));
    }
    assert_eq!(definition.counters.len(), rows("counters").len());
    for (counter, row) in definition.counters.iter().zip(rows("counters")) {
        assert_eq!((counter.id.as_str(), text(&counter.label, Locale::En), text(&counter.label, Locale::De)), (row["id"].as_str().unwrap(), row["en"].as_str().unwrap().to_string(), row["de"].as_str().unwrap().to_string()));
    }
    assert_eq!(definition.reasons.len(), rows("reasons").len());
    for (reason, row) in definition.reasons.iter().zip(rows("reasons")) {
        assert_eq!(u64::from(reason.code), number(&row["code"]));
        assert_eq!(reason.id, row["id"].as_str().unwrap());
        assert_eq!(serde_json::to_value(reason.verdict).unwrap(), row["verdict"]);
        assert_eq!((text(&reason.template, Locale::En), text(&reason.template, Locale::De)), (row["en"].as_str().unwrap().to_string(), row["de"].as_str().unwrap().to_string()));
        let placeholders = row["args"].as_array().unwrap().len();
        for locale in [Locale::En, Locale::De] {
            assert_eq!((0..4).filter(|index| text(&reason.template, locale).contains(&format!("{{{index}}}"))).count(), placeholders, "reason {} {locale:?} placeholders", reason.id);
        }
        assert_ne!(text(&reason.template, Locale::En), text(&reason.template, Locale::De), "reason {} is not really translated", reason.id);
    }
}

#[test]
fn every_numerical_stage_maps_into_the_declared_run_pipeline_in_order() {
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
    let mapped: Vec<u16> = stages.iter().map(|stage| EnergySimulationRunStage::of(*stage).index()).collect();
    assert!(mapped.windows(2).all(|pair| pair[0] <= pair[1]), "run stages advance monotonically with the numerical pipeline");
    assert_eq!(mapped.first(), Some(&EnergySimulationRunStage::Prepare.index()));
    assert_eq!(mapped.last(), Some(&EnergySimulationRunStage::Encode.index()));
}
//#endregion ⏯️Definition

//#region 🦶️RunJob
#[test]
fn the_pause_step_fuel_unit_is_one_computed_timestep() {
    let fixture = fixture();
    let (snapshot, template) = scenario();
    let mut job = EnergySimulationRunJob::new(identity(), snapshot, template);
    let run = drive(&mut job, number(&fixture["fuel"]["fuelPerStep"]));
    assert!(matches!(run.settled, StepOutcome::Complete(_)), "the run completes");
    let (before, last) = run.ticks.split_at(run.ticks.len() - 1);
    assert_eq!(before.len() as u64, number(&fixture["fuel"]["ticksBeforeSettle"]));
    let mut completed = 0;
    for tick in before {
        let progress = tick.progress.as_ref().expect("every tick carries progress");
        assert_eq!(progress.completed, completed + number(&fixture["fuel"]["completedAdvancePerTick"]), "one fuel unit advanced exactly one timestep");
        assert_eq!(progress.state, ToolRunState::Running);
        completed = progress.completed;
    }
    let settled = last[0].progress.as_ref().expect("the settling tick carries progress");
    assert_eq!(settled.completed - completed, number(&fixture["fuel"]["settledTickCompletedAdvance"]));
    assert_eq!(settled.state.as_str(), fixture["run"]["finalState"]);
    assert_eq!(EnergySimulationRunStage::ALL[usize::from(settled.stage)].id(), fixture["run"]["finalStage"]);
    let counter = |counter: EnergySimulationRunCounter| settled.counters.iter().find(|entry| entry.counter == counter.index()).map(|entry| entry.value);
    assert_eq!(counter(EnergySimulationRunCounter::WarmupTimesteps), Some(number(&fixture["run"]["warmupTimesteps"])));
    assert_eq!(counter(EnergySimulationRunCounter::RunTimesteps), Some(number(&fixture["run"]["runTimesteps"])));
    let tiers: Vec<&str> = fixture["run"]["tiers"].as_array().unwrap().iter().map(|tier| tier.as_str().unwrap()).collect();
    assert_eq!(counter(EnergySimulationRunCounter::TiersPublished), Some(tiers.len() as u64));
    assert_eq!(reason_ids(&run.ticks), tiers, "every quality tier publishes exactly once, in order");
    let mut visited: Vec<&str> = run.ticks.iter().map(|tick| EnergySimulationRunStage::ALL[usize::from(tick.progress.as_ref().unwrap().stage)].id()).collect();
    visited.dedup();
    assert_eq!(visited, fixture["run"]["stagesVisited"].as_array().unwrap().iter().map(|stage| stage.as_str().unwrap()).collect::<Vec<_>>());
    assert!(run.calls as u64 > number(&fixture["fuel"]["ticksBeforeSettle"]), "preparation and finalization yield without a tick");
    let StepOutcome::Complete(mut candidate) = run.settled else { unreachable!() };
    close_payload(&mut candidate.state);
    close_payload(&mut candidate.output);
    close(&mut job);
}

/// 🔮️ The streamed run must agree with the batch adapter `Engine::run` — the exact engine path the
/// ANSI/ASHRAE 140 EnergyPlus comparison (`🧪️tests/🏛️simulate-bestest-energyplus`) validates.
#[test]
fn run_job_results_and_final_readout_agree_with_the_batch_engine_oracle() {
    let (snapshot, template) = scenario();
    let config = simulation_config_for(&template, &snapshot.model);
    let oracle = crate::Engine::run(snapshot.model.clone(), config).expect("batch engine run");
    let mut job = EnergySimulationRunJob::new(identity(), snapshot, template);
    let run = drive(&mut job, semio_framework_job::INTERACTIVE_LANE_FUEL);
    let StepOutcome::Complete(mut candidate) = run.settled else { panic!("the streamed run completes") };
    close_payload(&mut candidate.state);
    close_payload(&mut candidate.output);
    let results = job.numerical.as_mut().expect("numerical owner").take_results().expect("streamed results");
    assert_eq!(results.meters, oracle.meters);
    assert_eq!(results.summaries, oracle.summaries);
    assert_eq!(results.time_series, oracle.time_series);
    assert_eq!(results.run_metadata.timesteps, oracle.run_metadata.timesteps);
    let final_step = run.ticks.iter().flat_map(|tick| tick.steps.iter()).find(|step| step.reason == EnergySimulationRunReason::Final.code()).expect("final tier step");
    let ToolRunStepArg::Float(kwh) = final_step.args[0] else { panic!("the final readout is a kWh float") };
    let oracle_kwh = job.numerical.as_ref().unwrap().cursor().facility_electricity_kwh;
    assert!((kwh - (oracle_kwh * 1_000.0).round() / 1_000.0).abs() < 1e-9, "the final step reports the facility electricity meter");
    assert_eq!(final_step.args[1], ToolRunStepArg::Unsigned(u64::from(oracle.run_metadata.timesteps)));
    close(&mut job);
}

#[test]
fn cancellation_mid_run_settles_cancelled_and_close_retires_every_owner() {
    let (snapshot, template) = scenario();
    let mut job = EnergySimulationRunJob::new(identity(), snapshot, template);
    let cancel = semio_framework_job::root_cancel_token();
    let (operation, generation) = (semio_framework_job::allocate_operation_id(), Generation(1));
    let mut sequence = 0;
    let mut ticks = 0;
    while ticks < 3 {
        let budget = StepBudget::new(1, u64::MAX);
        let mut verdict = None;
        match semio_framework_job::drive_step(&mut job, "energy.simulation.cancel.test", operation, generation, semio_framework_job::InteractiveStage::InteractiveStep, budget, cancel.clone(), semio_framework_job::default_now_us, &mut sequence, &mut verdict) {
            StepOutcome::PreviewReady(mut payload) => {
                close_payload(&mut payload);
                ticks += 1;
            }
            StepOutcome::Yield => {}
            _ => panic!("the run settled before it could be cancelled"),
        }
    }
    assert!(job.numerical.is_some(), "the run is simulating");
    cancel.cancel_now();
    let mut verdict = None;
    assert!(matches!(semio_framework_job::drive_step(&mut job, "energy.simulation.cancel.test", operation, generation, semio_framework_job::InteractiveStage::InteractiveStep, StepBudget::new(1, u64::MAX), cancel.clone(), semio_framework_job::default_now_us, &mut sequence, &mut verdict), StepOutcome::Cancelled));
    close(&mut job);
}

#[test]
fn a_model_beyond_numerical_admission_publishes_a_danger_step_before_the_fault() {
    let (_, template) = scenario();
    let mut model = crate::examples::bestest_600::model();
    model.zones.clear();
    let snapshot = Arc::new(crate::energy_snapshot_with_state(crate::ENERGY_MODEL_DOCUMENT_SCHEMA, &model, None));
    let mut job = EnergySimulationRunJob::new(identity(), snapshot, SimulationConfig { warmup_days: 0, ..template });
    let run = drive(&mut job, 1);
    assert!(matches!(run.settled, StepOutcome::Fault(_)), "an unsimulatable model faults the run");
    let last = run.ticks.last().expect("the refusal travels in a tick before the fault");
    let step = last.steps.last().expect("danger step");
    assert_eq!(step.kind, ToolRunStepKind::Danger);
    assert!(matches!(EnergySimulationRunReason::from_code(step.reason), Some(EnergySimulationRunReason::AdmissionRejected | EnergySimulationRunReason::SimulationFaulted)));
    close(&mut job);
}
//#endregion 🦶️RunJob

//#region 🧮️Capture
#[test]
fn the_bounded_capture_reproduces_every_model_field_including_run_period_and_schedules() {
    let (snapshot, _) = scenario();
    let mut census = CaptureCensus::new();
    while !census.step_one(&snapshot.model).expect("census admits the case model") {}
    let mut capture = ModelCapture::new();
    while !capture.step_one(&snapshot.model).expect("capture copies the case model") {}
    let captured = capture.finish();
    assert!(!captured.schedules.constants.is_empty() || !captured.schedules.daily.is_empty(), "the case model carries schedules");
    assert_eq!(captured, snapshot.model, "the captured model equals the base document model field for field");
}

#[test]
fn capture_admission_rejects_item_and_byte_max_plus_one_before_mount() {
    let mut items = CaptureCensus { lane: 0, index: 0, items: MAXIMUM_CAPTURE_ITEMS, bytes: 0 };
    assert_eq!(items.charge_backing(1, 0), Err("energy.session.capture-admission-exceeded"));
    let mut bytes = CaptureCensus { lane: 0, index: 0, items: 0, bytes: MAXIMUM_CAPTURE_BYTES };
    assert_eq!(bytes.charge_backing(1, 1), Err("energy.session.capture-admission-exceeded"));
}

#[test]
fn dynamic_record_capture_mutation_is_one_record_character_or_item_per_grant() {
    let mut source = Model::default();
    source.surfaces.try_reserve_exact(1).unwrap();
    let mut name = String::new();
    name.try_reserve_exact(8).unwrap();
    name.push_str("Wände");
    let mut vertices_m = Vec::new();
    vertices_m.try_reserve_exact(3).unwrap();
    vertices_m.extend([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0]]);
    source.surfaces.push(crate::model::Surface {
        id: crate::model::EntityId(1),
        name,
        zone_id: crate::model::EntityId(2),
        class: crate::model::SurfaceClass::ExteriorWall,
        vertices_m,
        construction_id: crate::model::EntityId(3),
        outside_boundary_condition: crate::model::OutsideBoundary::OutdoorAir,
        sun_exposed: true,
        wind_exposed: true,
        multiplier: 1,
    });
    let mut census = CaptureCensus { lane: 5, index: 0, items: 0, bytes: 0 };
    assert!(!census.step_one(&source).unwrap());
    assert_eq!(census.items, 11);
    assert_eq!(census.bytes, 8 + 3 * size_of::<[f64; 3]>());
    assert!(!census.step_one(&source).unwrap());
    assert_eq!(census.items, 12);
    assert_eq!(census.bytes, 8 + 3 * size_of::<[f64; 3]>() + size_of::<crate::model::Surface>());
    let mut capture = ModelCapture::new();
    capture.lane = 5;
    for _ in 0..64 {
        let before = capture.model.surfaces.len() + capture.model.surfaces.first().map_or(0, |surface| surface.name.chars().count() + surface.vertices_m.len());
        capture.step_one(&source).unwrap();
        let after = capture.model.surfaces.len() + capture.model.surfaces.first().map_or(0, |surface| surface.name.chars().count() + surface.vertices_m.len());
        assert!(after.saturating_sub(before) <= 1);
        if capture.lane > 5 {
            break;
        }
    }
    assert_eq!(capture.model.surfaces, source.surfaces);
}

#[test]
fn admitted_capture_source_has_no_whole_record_clone_backdoor() {
    let source = include_str!("../../🦀️.rs");
    for forbidden in [concat!("item", ".clone()"), concat!("airflow_network", ".clone()"), concat!("ground_temperature", ".clone()")] {
        assert!(!source.contains(forbidden), "whole record mutation survived: {forbidden}");
    }
    assert!(source.contains("capture-nested-vector-reserve"));
}

#[test]
fn partial_capture_closes_one_nested_character_or_item_per_grant() {
    let mut capture = ModelCapture::new();
    capture.model.name = "Gebäude".into();
    capture.model.zones.push(crate::model::Zone { id: crate::model::EntityId(1), name: "Raum".into(), volume_m3: 1.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true });
    let mut close = EnergyModelCloseCursor::new(capture.finish());
    let mut turns = 0;
    while !close.terminal_is_empty() {
        match close.close_step(4) {
            InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= 4);
            }
            InteractiveJobCloseStep::Complete => {}
            InteractiveJobCloseStep::Blocked => panic!("owned partial model cannot block"),
        }
        turns += 1;
        assert!(turns < 64);
    }
}
//#endregion 🧮️Capture
