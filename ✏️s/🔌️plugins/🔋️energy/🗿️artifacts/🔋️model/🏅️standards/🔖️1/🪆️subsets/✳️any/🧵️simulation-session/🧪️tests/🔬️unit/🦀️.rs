use super::*;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::Terminology;
use semio_framework_tool_run::{ToolRunId, ToolRunTick};

const RUN_SCHEMA: &str = include_str!("../../../✏️editor/🧵️simulation-session/🔣️.json");
const RUN_FIXTURE: &str = include_str!("../../../✏️editor/🧵️simulation-session/🧪️tests/🧫️fixtures/🔣️.json");

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
    (Arc::new(crate::snapshot::energy_snapshot_with_state(crate::snapshot::ENERGY_MODEL_DOCUMENT_SCHEMA, &model, None)), template)
}

fn identity() -> ToolRunIdentity {
    ToolRunIdentity::new(ToolRunId { app_instance_id: 7, run: 1 }, [3; 32])
}

const TEST_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };

/// 🧭️ What one driven call of the run job lent to its caller.
#[derive(Debug, PartialEq, Eq)]
enum Stepped {
    Yield,
    Tick(Vec<u8>),
    Checkpoint,
    Complete,
    Fault,
    Cancelled,
}

/// 🎟️ A declared preborn run retains its one original caller and decoder through every refused turn and close.
struct RunFixture {
    job: Option<EnergySimulationRunJob>,
    caller: Option<semio_framework_job::OriginalJobFixtureCaller>,
    decoder: semio_framework_job::OriginalJobDecodeOwner,
}

impl RunFixture {
    fn new(identity: ToolRunIdentity, snapshot: Arc<EnergyModelSnapshot>, template: SimulationConfig) -> Self {
        let policy = source_policy();
        let source = &policy["source"];
        Self { job: Some(EnergySimulationRunJob::new(identity, snapshot, template)), caller: Some(semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(number(&source["operation"])), semio_framework_job::Generation(number(&source["generation"])), source_stock(), semio_framework_job::default_now_us, u64::MAX, number(&source["fuel"]))), decoder: semio_framework_job::OriginalJobDecodeOwner::empty() }
    }

    fn cancel(&self) { self.caller.as_ref().expect("original Energy caller").cancel().cancel_now(); }
}

impl std::ops::Deref for RunFixture {
    type Target = EnergySimulationRunJob;
    fn deref(&self) -> &Self::Target { self.job.as_ref().expect("original Energy run remains retained") }
}

impl std::ops::DerefMut for RunFixture {
    fn deref_mut(&mut self) -> &mut Self::Target { self.job.as_mut().expect("original Energy run remains retained") }
}

impl Drop for RunFixture {
    fn drop(&mut self) { if !std::thread::panicking() { close(self); } }
}

fn source_stock() -> RetainedCloneGrant {
    let law = source_policy();
    let row = &law["stock"];
    RetainedCloneGrant { maximum_items: number(&row["items"]) as usize, maximum_copy_bytes: number(&row["copyBytes"]) as usize, maximum_capacity_bytes: number(&row["capacityBytes"]) as usize, maximum_release_bytes: number(&row["releaseBytes"]) as usize, maximum_depth: number(&row["depth"]) as usize }
}

fn source_policy() -> serde_json::Value { serde_json::from_str(include_str!("../🎟️source/🔣️.json")).expect("original source policy") }

fn step_once(fixture: &mut RunFixture, fuel: u64, deadline_us: u64) -> Stepped {
    let mut outcome = None;
    let job = fixture.job.as_mut().expect("original Energy run");
    fixture.caller.as_mut().expect("original Energy caller").turn_with_decoder(TEST_GRANT, &mut fixture.decoder, |cx| {
        cx.with_original_work_loan(fuel, deadline_us, |cx| {
            let mut verdict = None;
            outcome = Some(match semio_framework_job::drive_step(job, cx, "energy.simulation.run.test", semio_framework_job::InteractiveStage::InteractiveStep, &mut verdict)? {
                None | Some(JobOutcomeBorrow::Yield { .. }) => Stepped::Yield,
                Some(JobOutcomeBorrow::PreviewReady { payload, .. }) => Stepped::Tick((0..payload.page_count()).flat_map(|index| payload.page(index).expect("tick page").to_vec()).collect()),
                Some(JobOutcomeBorrow::CheckpointReady { .. }) => Stepped::Checkpoint,
                Some(JobOutcomeBorrow::Complete { .. }) => Stepped::Complete,
                Some(JobOutcomeBorrow::Fault { .. }) => Stepped::Fault,
                Some(JobOutcomeBorrow::Cancelled { .. }) => Stepped::Cancelled,
            });
            Ok(())
        })
    }).expect("same original Energy run turn");
    outcome.expect("original run outcome")
}

/// 🦶️ Every observable outcome of driving a run job to settlement with a fixed fuel budget per call.
struct Drive {
    ticks: Vec<ToolRunTick>,
    settled: Stepped,
    calls: usize,
}

fn drive(job: &mut RunFixture, fuel: u64) -> Drive {
    let mut ticks = Vec::new();
    for calls in 1..=50_000_000 {
        let now = semio_framework_job::default_now_us().expect("clock");
        match step_once(job, fuel, now + semio_framework_job::INTERACTIVE_LANE_WALL_US * 4) {
            Stepped::Yield => {}
            Stepped::Tick(bytes) => ticks.push(ToolRunTick::decode(&bytes).expect("tick decodes")),
            Stepped::Checkpoint => panic!("a restart-policy run never reports a checkpoint"),
            settled => return Drive { ticks, settled, calls },
        }
    }
    panic!("the run never settled");
}

fn close(fixture: &mut RunFixture) {
    let Some(job) = fixture.job.as_mut() else { return };
    job.begin_close();
    for _ in 0..10_000_000 {
        let demand = RetirementDemand {
            copy_bytes: job.next_close_copy_byte_demand().expect("a locally owned run quotes its copy demand"),
            capacity_bytes: job.next_close_capacity_byte_demand(usize::MAX).expect("a locally owned run quotes its capacity demand"),
            release_bytes: job.next_close_release_byte_demand().expect("a locally owned run quotes its release demand"),
            depth: job.next_close_depth_demand().expect("a locally owned run quotes its depth demand"),
        };
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        match fixture.caller.as_mut().expect("original Energy caller remains live").turn_with_decoder(grant, &mut fixture.decoder, |cx| job.close_step(cx)).expect("same original Energy run close") {
            InteractiveJobCloseStep::Complete { .. } => {
                assert!(job.terminal_is_empty(), "a closed run job holds no owner");
                break;
            }
            InteractiveJobCloseStep::Pending { progress } => assert!(progress.fits(grant), "a close receipt fits its own quoted grant"),
            InteractiveJobCloseStep::Blocked => panic!("the run job close blocked"),
        }
    }
    assert!(job.terminal_is_empty(), "the run job close never completed");
    fixture.job.take();
    let caller = fixture.caller.as_mut().expect("original Energy source closes after run");
    while !fixture.decoder.terminal_is_empty() { caller.turn(source_stock(), |cx| { let demand = fixture.decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?; semio_framework_job::close_original_retained_step(cx, demand, |grant| fixture.decoder.close_step(grant))?; Ok(()) }).expect("same original Energy decoder close"); }
    fixture.caller.take().expect("original Energy caller closes once").retire(source_stock());
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
    let mut job = RunFixture::new(identity(), snapshot, template);
    let run = drive(&mut job, number(&fixture["fuel"]["fuelPerStep"]));
    assert_eq!(run.settled, Stepped::Complete, "the run completes");
    let (before, last) = run.ticks.split_at(run.ticks.len() - 1);
    let preparing: Vec<_> = before.iter().filter(|tick| tick.progress.as_ref().expect("preparing progress").completed == 0).collect();
    assert!(!preparing.is_empty(), "original input identity publishes progress before numerical timesteps");
    for tick in preparing {
        let progress = tick.progress.as_ref().unwrap();
        assert_eq!(progress.stage, EnergySimulationRunStage::Prepare.index());
        assert!(progress.total.is_none());
        assert!(progress.counters.iter().any(|entry| entry.counter == EnergySimulationRunCounter::InputIdentityBytes.index() && entry.value > 0));
    }
    let before: Vec<_> = before.iter().filter(|tick| tick.progress.as_ref().unwrap().completed > 0).collect();
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
    assert_eq!(run.settled, Stepped::Complete);
    close(&mut job);
}

/// 🔮️ The streamed run must agree with the caller-driven `EnergyBatchRun` — the exact engine path the
/// ANSI/ASHRAE 140 EnergyPlus comparison (`🧪️tests/🏛️simulate-bestest-energyplus`) validates.
#[test]
fn run_job_results_and_final_readout_agree_with_the_batch_engine_oracle() {
    let (snapshot, template) = scenario();
    let config = simulation_config_for(&template, &snapshot.model);
    let oracle = crate::sim::testing::run_preborn(snapshot.model.clone(), config).expect("batch engine run");
    let mut job = RunFixture::new(identity(), snapshot, template);
    let run = drive(&mut job, semio_framework_job::INTERACTIVE_LANE_FUEL);
    assert_eq!(run.settled, Stepped::Complete, "the streamed run completes");
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
    let mut job = RunFixture::new(identity(), snapshot, template);
    let mut ticks = 0;
    let deadline = |now: u64| now + semio_framework_job::INTERACTIVE_LANE_WALL_US * 4;
    while ticks < 3 {
        match step_once(&mut job, 1, deadline(semio_framework_job::default_now_us().expect("clock"))) {
            Stepped::Tick(_) => ticks += 1,
            Stepped::Yield => {}
            _ => panic!("the run settled before it could be cancelled"),
        }
    }
    assert!(job.numerical.is_some(), "the run is simulating");
    job.cancel();
    assert_eq!(step_once(&mut job, 1, deadline(semio_framework_job::default_now_us().expect("clock"))), Stepped::Cancelled);
    close(&mut job);
}

#[test]
fn a_model_beyond_numerical_admission_publishes_a_danger_step_before_the_fault() {
    let (_, template) = scenario();
    let mut model = crate::examples::bestest_600::model();
    model.zones.clear();
    let snapshot = Arc::new(crate::snapshot::energy_snapshot_with_state(crate::snapshot::ENERGY_MODEL_DOCUMENT_SCHEMA, &model, None));
    let mut job = RunFixture::new(identity(), snapshot, SimulationConfig { warmup_days: 0, ..template });
    let run = drive(&mut job, 1);
    assert_eq!(run.settled, Stepped::Fault, "an unsimulatable model faults the run");
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
    let mut close=semio_framework_value::retirement::controlled::ControlledRetirement::new(capture.finish()).unwrap_or_else(|_|panic!("captured Energy model supports original retirement"));
    for index in 0..10000{if close.terminal_is_empty(){return}let d=close.selected_demand(4096).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||close.step(g).unwrap());assert!(step.progress().fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(index<9999);}

}
//#endregion 🧮️Capture

//#region 🧱️SurfacePayload
#[test]
fn the_run_refreshes_exactly_the_three_d_model_and_simulation_window_bodies() {
    let schema: serde_json::Value = serde_json::from_str(RUN_SCHEMA).unwrap();
    let declared: Vec<String> = schema["x-semio-toolRun"]["windows"].as_array().expect("the schema declares the refreshed windows").iter().map(|id| id.as_str().unwrap().to_string()).collect();
    assert_eq!(energy_simulation_run_definition().windows, declared, "the run definition mirrors its own source of record");
    assert_eq!(declared, vec![ENERGY_MODEL_3D_WINDOW_KIND_ID.to_string(), ENERGY_SIMULATION_WINDOW_KIND_ID.to_string()]);
}

#[test]
fn a_tier_boundary_and_the_completing_tick_carry_the_per_surface_map() {
    let (snapshot, template) = scenario();
    let surfaces = snapshot.model.surfaces.len() + snapshot.model.fenestrations.len();
    let mut job = RunFixture::new(identity(), snapshot, template);
    let run = drive(&mut job, semio_framework_job::INTERACTIVE_LANE_FUEL);
    assert_eq!(run.settled, Stepped::Complete, "the streamed run completes");

    let carrying: Vec<&ToolRunTick> = run.ticks.iter().filter(|tick| tick.payload.is_some()).collect();
    assert!(!carrying.is_empty(), "no tick ever carried a per-surface payload");
    let tiers = run.ticks.iter().flat_map(|tick| tick.steps.iter()).filter(|step| matches!(EnergySimulationRunReason::from_code(step.reason), Some(EnergySimulationRunReason::DesignDay | EnergySimulationRunReason::CoarseTimestep | EnergySimulationRunReason::SteadyStateEstimate | EnergySimulationRunReason::Final))).count();
    assert!(carrying.len() <= tiers + 1, "a payload republishes only on a tier boundary or at completion");

    let bytes = carrying.last().expect("the last carrying tick").payload.as_ref().expect("payload");
    assert_eq!(&bytes[0..4], &ENERGY_SURFACE_PAYLOAD_MAGIC, "the wire carries its own magic");
    let count = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    assert_eq!(count, surfaces, "one row per opaque surface and per fenestration");
    assert_eq!(bytes.len(), energy_surface_payload_bytes(count));

    let results = job.numerical.as_mut().expect("numerical owner").take_results().expect("streamed results");
    let scalar = |row: &[u8], offset: usize| f64::from(f32::from_le_bytes(row[offset..offset + 4].try_into().unwrap()));
    let published: std::collections::HashMap<u32, (f64, f64)> = bytes[ENERGY_SURFACE_PAYLOAD_HEADER_BYTES..]
        .chunks_exact(ENERGY_SURFACE_PAYLOAD_ROW_BYTES)
        .map(|row| (u32::from_le_bytes(row[0..4].try_into().unwrap()), (scalar(row, 4), scalar(row, 12))))
        .collect();
    assert_eq!(published.len(), results.per_surface.len());
    for row in &results.per_surface {
        let (loss, transmitted) = *published.get(&row.id.0).expect("every result row is published");
        assert!((loss - row.conduction_loss_kwh).abs() <= row.conduction_loss_kwh.abs() * 1e-5 + 1e-5, "row {:?}", row.id);
        assert!((transmitted - row.solar_transmitted_kwh).abs() <= row.solar_transmitted_kwh.abs() * 1e-5 + 1e-5, "row {:?}", row.id);
    }
    close(&mut job);
}

#[test]
fn a_tick_carrying_two_thousand_surfaces_stays_far_inside_the_tick_byte_ceiling() {
    let rows: Vec<crate::results::SurfaceEnergySummary> = (0..2_000u32)
        .map(|id| crate::results::SurfaceEnergySummary { id: crate::model::EntityId(id), conduction_loss_kwh: f64::from(id), conduction_gain_kwh: 0.5, solar_transmitted_kwh: 1.5, solar_absorbed_kwh: 2.5 })
        .collect();
    let payload = encode_surface_energy_payload(rows.iter().copied());
    assert_eq!(payload.len(), energy_surface_payload_bytes(2_000));

    let mut writer = ToolRunTickWriter::new(identity());
    for index in 0..8u64 {
        let _ = writer.step(ToolRunStepKind::Success, EnergySimulationRunStage::Run.index(), EnergySimulationRunReason::CoarseTimestep.code(), None, &[ToolRunStepArg::Float(1.0), ToolRunStepArg::Unsigned(index), ToolRunStepArg::Unsigned(2_000)]);
    }
    writer.payload(payload.clone());
    writer.progress(EnergyRunCursor::new().progress(identity(), ToolRunState::Running, EnergySimulationRunStage::Run));
    let encoded = writer.finish().expect("a pending tick").encode().expect("2 000 surfaces encode inside the tick ceiling");
    assert!(encoded.len() * 2 < semio_framework_tool_run::TOOL_RUN_TICK_BYTES_MAX, "2 000 surfaces plus progress and a full step ring is {} of {} tick bytes", encoded.len(), semio_framework_tool_run::TOOL_RUN_TICK_BYTES_MAX);
    let decoded = ToolRunTick::decode(&encoded).expect("the tick round-trips");
    assert_eq!(decoded.payload.as_deref(), Some(payload.as_slice()), "the payload survives the tick codec byte for byte");
}
//#endregion 🧱️SurfacePayload

/// 🪪️ Original paid canonical bytes publish monotone activity with unknown total and untouched timestep units.
#[test]
fn input_identity_progress_matches_neutral_bytes_unknown_total_and_original_counter(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/⚡️simulation/⚙️engine/🧪️sim/🪪️input-identity/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let mut cursor=EnergyRunCursor::new();let mut writer=ToolRunTickWriter::new(identity());
 for row in fixture["progress"]["samples"].as_array().unwrap(){let live=EnergyJobCursor{input_identity_bytes:row[0].as_u64().unwrap()as usize,input_identity_ready:row[1].as_bool().unwrap(),stage:EnergyJobStage::Validate,tier:EnergyQualityTier::SteadyStateEstimate,warmup_hour:0,warmup_hours:0,timestep:0,total_timesteps:0,facility_electricity_kwh:0.0};let(activity,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.observe(live,&mut writer));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(activity,row[2].as_bool().unwrap());let progress=cursor.progress(identity(),ToolRunState::Running,EnergySimulationRunStage::Prepare);assert_eq!(progress.completed,0);assert!(progress.total.is_none());assert_eq!(progress.counters.len(),9);assert_eq!(progress.counters[4].counter,EnergySimulationRunCounter::InputIdentityBytes.index());assert_eq!(progress.counters[4].value,row[0].as_u64().unwrap());}
 let mut owner=ControlledRetirement::new(writer).unwrap_or_else(|_|panic!("original progress writer close"));while !owner.terminal_is_empty(){let demand=RetirementDemand{copy_bytes:owner.next_copy_byte_demand().unwrap(),capacity_bytes:owner.next_capacity_byte_demand(4096).unwrap(),release_bytes:owner.next_release_byte_demand().unwrap(),depth:owner.next_depth_demand().unwrap()};owner.step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}).unwrap();}
 eprintln!("[DEBUG] Energy input bytes8 neutral monotone progress events unknown total original fifth counter heap0");
}

/// 🎟️ Authored nested work loans debit the same original source once and restore only the unused parent ceiling.
#[test]
fn original_session_child_loans_match_neutral_source_without_duplicate_receipts() {
    let policy = source_policy();
    let source = &policy["source"];
    let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(number(&source["operation"])), semio_framework_job::Generation(number(&source["generation"])), source_stock(), semio_framework_job::default_now_us, u64::MAX, number(&source["fuel"]));
    for row in policy["loans"].as_array().unwrap() {
        let before = caller.fuel_remaining();
        let receipt = caller.cumulative_receipt();
        caller.turn(TEST_GRANT, |cx| {
            cx.with_original_work_loan(number(&row["parentFuel"]), cx.deadline_us(), |cx| {
                cx.with_original_work_loan(number(&row["requestedFuel"]), cx.deadline_us(), |cx| {
                    assert_eq!(cx.fuel_remaining(), number(&row["requestedFuel"]).min(number(&row["parentFuel"])));
                    cx.consume_fuel(number(&row["spentFuel"]));
                    Ok(())
                })?;
                assert_eq!(cx.fuel_remaining(), number(&row["remainingFuel"]));
                Ok(())
            })
        }).unwrap();
        assert_eq!(before - caller.fuel_remaining(), number(&row["spentFuel"]));
        assert_eq!(caller.cumulative_receipt(), receipt);
    }
    caller.retire(source_stock());
    eprintln!("[DEBUG] actual Energy nested original source loans3 preserve same source, finite fuel and zero duplicated retained receipts");
}

/// 🧮️ Actual retained meter values agree before and after the original final Results transfer.
#[test]
fn live_and_final_progress_publish_actual_heating_and_cooling_meters(){
 let(snapshot,template)=scenario();
 let mut run=RunFixture::new(identity(),snapshot,template);
 let mut live=false;
 let mut final_result=false;
 for turn in 0..10_000_000{
  let result=step_once(&mut run,semio_framework_job::INTERACTIVE_LANE_FUEL,u64::MAX);
  match result{
   Stepped::Tick(bytes)=>{
    let tick=ToolRunTick::decode(&bytes).unwrap();
    if let Some(progress)=tick.progress{
     if let Some(numerical)=run.numerical.as_ref(){
      let meters=numerical.live_meters();
      for(counter,end_use)in [(EnergySimulationRunCounter::HeatingWh,semio_s_plugin_energy_engine_meters::EndUse::Heating),(EnergySimulationRunCounter::CoolingWh,semio_s_plugin_energy_engine_meters::EndUse::Cooling)]{
       let joules:f64=meters.meters.values().filter(|meter|meter.end_use==end_use).map(|meter|meter.energy_j).sum();
       let value=progress.counters.iter().find(|entry|entry.counter==counter.index()).expect("actual meter counter").value;
       assert_eq!(value,(joules/3600.0).round().max(0.0)as u64);
       if value>0&&progress.state==ToolRunState::Running{live=true;}
       if value>0&&progress.state==ToolRunState::Complete{final_result=true;}
      }
     }
    }
   }
   Stepped::Complete=>break,
   Stepped::Yield=>{},
   other=>panic!("meter run settled unexpectedly: {other:?}"),
  }
  assert!(turn<9_999_999,"meter run must settle");
 }
 assert!(live,"real meter progress is observable during the numerical run");
 assert!(final_result,"real meter progress survives the original Results move");
 close(&mut run);
}
