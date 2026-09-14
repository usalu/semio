//! ⚡️ Energy simulation tool run (`📋️tool-run-contract.md` §2.4, §3.7): the `ToolRunDefinition` vocabulary
//! (stages of the tier/timestep pipeline, counters, reasons) and the run job that captures the base model
//! in bounded grants, drives the numerical `EnergyJob` inline and publishes progress ticks. One unit of
//! fuel is one computed timestep (warmup or run), so a single step of a paused run advances exactly one.
//! The run is read-only: it holds no provisional document ops, so finalize publishes nothing.
//! Source of record: `✏️editor/🧵️simulation-session/🔣️.json`.

use crate::{EnergyAdmissionRejected, EnergyJob, EnergyJobCursor, EnergyJobStage, EnergyModelCloseCursor, EnergyModelSnapshot, EnergyNumericalBounds, EnergyQualityTier, Model, SimulationConfig};
use semio_framework_job::{Generation, InteractiveJob, InteractiveJobCloseStep, JobFault, JobPayloadStream, OperationId, RetainedJobPayload, StepBudget, StepContext, StepOutcome};
use semio_framework_plugin::LocalizedLabel;
use semio_framework_tool_run::{
    JobKindId, ToolRunCounter, ToolRunCounterDefinition, ToolRunDefinition, ToolRunIdentity, ToolRunProgress, ToolRunReasonDefinition, ToolRunRebasePolicy, ToolRunReconfigurePolicy, ToolRunStageDefinition, ToolRunState, ToolRunStepArg, ToolRunStepKind, ToolRunSettingsReads, ToolRunStepRing,
    ToolRunTickWriter, ToolRunTraceKind, ToolRunVerdict,
};
use std::sync::Arc;

//#region 🔖️Contract
pub const ENERGY_SIMULATION_RUN_JOB_KIND: &str = "energy.simulation.run";
pub const ENERGY_SIMULATION_RUN_SCHEMA: &str = "energy.simulation.run.v1";
/// 🎚️ The editor config fields a run reads; a publication changing one of them reconfigures a live run.
pub const ENERGY_SIMULATION_RUN_SETTINGS: [&str; 3] = ["/zoneTimestepMinutes", "/systemTimestepMinutes", "/warmupDays"];
const MAXIMUM_CAPTURE_ITEMS: usize = 4_194_304;
const MAXIMUM_CAPTURE_BYTES: usize = 512 * 1_024 * 1_024;
const CAPTURE_LAST_LANE: u8 = 45;

/// 🧭️ Stages of the run as the panel names them: the plugin capture, then the numerical pipeline grouped by tier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnergySimulationRunStage {
    Capture,
    Prepare,
    Warmup,
    Run,
    Finalize,
    Encode,
}

impl EnergySimulationRunStage {
    pub const ALL: [Self; 6] = [Self::Capture, Self::Prepare, Self::Warmup, Self::Run, Self::Finalize, Self::Encode];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::Capture => "capture",
            Self::Prepare => "prepare",
            Self::Warmup => "warmup",
            Self::Run => "run",
            Self::Finalize => "finalize",
            Self::Encode => "encode",
        }
    }

    pub fn label(self) -> LocalizedLabel {
        match self {
            Self::Capture => LocalizedLabel::native("Capturing the model", "Modell wird erfasst"),
            Self::Prepare => LocalizedLabel::native("Preparing weather, geometry and zones", "Wetter, Geometrie und Zonen werden vorbereitet"),
            Self::Warmup => LocalizedLabel::native("Warming up", "Einschwingen"),
            Self::Run => LocalizedLabel::native("Solving timesteps", "Zeitschritte werden gelöst"),
            Self::Finalize => LocalizedLabel::native("Sizing and summarizing", "Auslegung und Zusammenfassung"),
            Self::Encode => LocalizedLabel::native("Encoding the final result", "Endergebnis wird kodiert"),
        }
    }

    /// 🧭️ The run stage a numerical pipeline stage belongs to.
    pub fn of(stage: EnergyJobStage) -> Self {
        match stage {
            EnergyJobStage::Validate | EnergyJobStage::ResolveWeather | EnergyJobStage::Precompute | EnergyJobStage::InitializeZones | EnergyJobStage::InitializeSurfaces | EnergyJobStage::InitializeWarmupHistory => Self::Prepare,
            EnergyJobStage::WarmupTimestep | EnergyJobStage::WarmupConvergence => Self::Warmup,
            EnergyJobStage::StartRun | EnergyJobStage::RunZoneTimestep | EnergyJobStage::AggregateZone | EnergyJobStage::AggregateFacility | EnergyJobStage::PublishTimestep => Self::Run,
            EnergyJobStage::Finalize | EnergyJobStage::Size | EnergyJobStage::FinalizeSummaries | EnergyJobStage::FinalizeMetrics | EnergyJobStage::FinalizeEconomics | EnergyJobStage::BuildResults | EnergyJobStage::PublishFinal => Self::Finalize,
            EnergyJobStage::EncodeOutput | EnergyJobStage::Complete => Self::Encode,
        }
    }
}

/// 🔢️ Counters every progress snapshot carries, in this order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnergySimulationRunCounter {
    WarmupTimesteps,
    RunTimesteps,
    TiersPublished,
    FacilityElectricityWh,
}

impl EnergySimulationRunCounter {
    pub const ALL: [Self; 4] = [Self::WarmupTimesteps, Self::RunTimesteps, Self::TiersPublished, Self::FacilityElectricityWh];

    pub fn index(self) -> u16 {
        self as u16
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::WarmupTimesteps => "warmupTimesteps",
            Self::RunTimesteps => "runTimesteps",
            Self::TiersPublished => "tiersPublished",
            Self::FacilityElectricityWh => "facilityElectricityWh",
        }
    }

    pub fn label(self) -> LocalizedLabel {
        match self {
            Self::WarmupTimesteps => LocalizedLabel::native("Warmup timesteps", "Einschwing-Zeitschritte"),
            Self::RunTimesteps => LocalizedLabel::native("Run timesteps", "Lauf-Zeitschritte"),
            Self::TiersPublished => LocalizedLabel::native("Quality tiers published", "Veröffentlichte Qualitätsstufen"),
            Self::FacilityElectricityWh => LocalizedLabel::native("Facility electricity (Wh)", "Anlagenelektrizität (Wh)"),
        }
    }
}

/// 🗒️ Step reasons. The four tier reasons carry `[kWh, run timesteps, total run timesteps]`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnergySimulationRunReason {
    SteadyStateEstimate,
    DesignDay,
    CoarseTimestep,
    Final,
    WarmupConverged,
    CaptureRejected,
    AdmissionRejected,
    SimulationFaulted,
}

impl EnergySimulationRunReason {
    pub const ALL: [Self; 8] = [Self::SteadyStateEstimate, Self::DesignDay, Self::CoarseTimestep, Self::Final, Self::WarmupConverged, Self::CaptureRejected, Self::AdmissionRejected, Self::SimulationFaulted];

    pub fn code(self) -> u16 {
        self as u16
    }

    pub fn from_code(code: u16) -> Option<Self> {
        Self::ALL.get(usize::from(code)).copied()
    }

    pub fn id(self) -> &'static str {
        match self {
            Self::SteadyStateEstimate => "steadyStateEstimate",
            Self::DesignDay => "designDay",
            Self::CoarseTimestep => "coarseTimestep",
            Self::Final => "final",
            Self::WarmupConverged => "warmupConverged",
            Self::CaptureRejected => "captureRejected",
            Self::AdmissionRejected => "admissionRejected",
            Self::SimulationFaulted => "simulationFaulted",
        }
    }

    pub fn verdict(self) -> ToolRunVerdict {
        match self {
            Self::SteadyStateEstimate | Self::DesignDay | Self::CoarseTimestep | Self::Final | Self::WarmupConverged => ToolRunVerdict::Success,
            Self::CaptureRejected | Self::AdmissionRejected | Self::SimulationFaulted => ToolRunVerdict::Danger,
        }
    }

    pub fn template(self) -> LocalizedLabel {
        match self {
            Self::SteadyStateEstimate => LocalizedLabel::native("Steady-state estimate (provisional): {0} kWh facility electricity", "Stationäre Schätzung (vorläufig): {0} kWh Anlagenelektrizität"),
            Self::DesignDay => LocalizedLabel::native("Design day (provisional): {0} kWh after {1} of {2} timesteps", "Auslegungstag (vorläufig): {0} kWh nach {1} von {2} Zeitschritten"),
            Self::CoarseTimestep => LocalizedLabel::native("Coarse timestep (provisional): {0} kWh after {1} of {2} timesteps", "Grober Zeitschritt (vorläufig): {0} kWh nach {1} von {2} Zeitschritten"),
            Self::Final => LocalizedLabel::native("Final: {0} kWh after {1} of {2} timesteps", "Endgültig: {0} kWh nach {1} von {2} Zeitschritten"),
            Self::WarmupConverged => LocalizedLabel::native("Warmup converged after {0} timesteps", "Einschwingen nach {0} Zeitschritten konvergiert"),
            Self::CaptureRejected => LocalizedLabel::native("The model exceeds the capture capacity", "Das Modell überschreitet die Erfassungskapazität"),
            Self::AdmissionRejected => LocalizedLabel::native("The model exceeds the simulation capacity", "Das Modell überschreitet die Simulationskapazität"),
            Self::SimulationFaulted => LocalizedLabel::native("The simulation failed after {0} timesteps", "Die Simulation ist nach {0} Zeitschritten fehlgeschlagen"),
        }
    }

    /// 🏅️ The reason a published quality tier reports under.
    pub fn of_tier(tier: EnergyQualityTier) -> Self {
        match tier {
            EnergyQualityTier::SteadyStateEstimate => Self::SteadyStateEstimate,
            EnergyQualityTier::DesignDay => Self::DesignDay,
            EnergyQualityTier::CoarseTimestep => Self::CoarseTimestep,
            EnergyQualityTier::Final => Self::Final,
        }
    }
}

/// ⏯️ The simulation run declaration: read-only, restarted on any base or settings change, no trace subjects,
/// reading the three run settings of the editor config.
pub fn energy_simulation_run_definition() -> ToolRunDefinition {
    ToolRunDefinition {
        mutating: false,
        rebase: ToolRunRebasePolicy::Restart,
        reconfigure: ToolRunReconfigurePolicy::Restart,
        unit: LocalizedLabel::native("timesteps", "Zeitschritte"),
        stages: EnergySimulationRunStage::ALL.iter().map(|stage| ToolRunStageDefinition { id: stage.id().into(), label: stage.label() }).collect(),
        counters: EnergySimulationRunCounter::ALL.iter().map(|counter| ToolRunCounterDefinition { id: counter.id().into(), label: counter.label() }).collect(),
        reasons: EnergySimulationRunReason::ALL.iter().map(|reason| ToolRunReasonDefinition { code: reason.code(), id: reason.id().into(), verdict: reason.verdict(), template: reason.template() }).collect(),
        trace: ToolRunTraceKind::None,
        run_job: JobKindId::new(ENERGY_SIMULATION_RUN_JOB_KIND),
        revalidate_job: None,
        settings: ToolRunSettingsReads { config: ENERGY_SIMULATION_RUN_SETTINGS.iter().map(|pointer| pointer.to_string()).collect(), window_config: Default::default() },
        windows: Vec::new(),
    }
}

/// ⚙️ Folds the captured model's persisted run period and schedule tables into the settings template.
pub fn simulation_config_for(template: &SimulationConfig, model: &Model) -> SimulationConfig {
    SimulationConfig {
        run_period_start_month: model.run_period.start_month,
        run_period_start_day: model.run_period.start_day,
        run_period_end_month: model.run_period.end_month,
        run_period_end_day: model.run_period.end_day,
        schedules: model.schedules.clone(),
        ..template.clone()
    }
}
//#endregion 🔖️Contract

//#region 🧮️RetainedInput
#[derive(Clone, Copy)]
struct CaptureCensus {
    lane: u8,
    index: usize,
    items: usize,
    bytes: usize,
}

impl CaptureCensus {
    fn new() -> Self {
        Self { lane: 0, index: 0, items: 2, bytes: size_of::<Model>() + size_of::<SimulationConfig>() }
    }

    fn charge_backing(&mut self, capacity: usize, item_bytes: usize) -> Result<(), &'static str> {
        self.charge_observed(capacity, capacity.checked_mul(item_bytes).ok_or("energy.session.capture-byte-overflow")?)
    }

    fn charge_observed(&mut self, items: usize, bytes: usize) -> Result<(), &'static str> {
        self.items = self.items.checked_add(items).ok_or("energy.session.capture-item-overflow")?;
        self.bytes = self.bytes.checked_add(bytes).ok_or("energy.session.capture-byte-overflow")?;
        if self.items > MAXIMUM_CAPTURE_ITEMS || self.bytes > MAXIMUM_CAPTURE_BYTES {
            return Err("energy.session.capture-admission-exceeded");
        }
        Ok(())
    }

    fn charge(&mut self, capacity: usize, item_bytes: usize) -> Result<(), &'static str> {
        self.charge_backing(capacity, item_bytes)?;
        self.lane = self.lane.checked_add(1).ok_or("energy.session.capture-lane-overflow")?;
        self.index = 0;
        Ok(())
    }

    fn step_one(&mut self, source: &Model) -> Result<bool, &'static str> {
        macro_rules! vector {
            ($($field:ident).+) => {{
                let (capacity, bytes) = vector_credit(&source.$($field).+);
                self.charge(capacity, bytes)?;
            }};
        }
        macro_rules! nested {
            ($($field:ident).+, $backing:expr) => {{
                if let Some(item) = source.$($field).+.get(self.index) {
                    let (items, bytes) = $backing(item);
                    self.charge_observed(items, bytes)?;
                    self.index += 1;
                } else {
                    let (capacity, bytes) = vector_credit(&source.$($field).+);
                    self.charge(capacity, bytes)?;
                }
            }};
        }
        match self.lane {
            0 => self.charge(source.name.capacity(), 1)?,
            1 => self.charge(source.version.capacity(), 1)?,
            2 => self.charge(1, size_of_val(&source.site))?,
            3 => nested!(zones, |item: &crate::model::Zone| (item.name.capacity(), item.name.capacity())),
            4 => nested!(spaces, |item: &crate::model::Space| (item.name.capacity(), item.name.capacity())),
            5 => {
                nested!(surfaces, |item: &crate::model::Surface| { (item.name.capacity().saturating_add(item.vertices_m.capacity()), item.name.capacity().saturating_add(item.vertices_m.capacity().saturating_mul(size_of::<[f64; 3]>())),) });
            }
            6 => nested!(fenestrations, |item: &crate::model::Fenestration| (item.name.capacity(), item.name.capacity())),
            7 => nested!(materials, |item: &crate::model::Material| (item.name.capacity(), item.name.capacity())),
            8 => nested!(constructions, |item: &crate::model::Construction| {
                (item.name.capacity().saturating_add(item.layer_material_ids.capacity()), item.name.capacity().saturating_add(item.layer_material_ids.capacity().saturating_mul(size_of::<crate::model::EntityId>())))
            }),
            9 => vector!(people),
            10 => vector!(lighting),
            11 => vector!(equipment),
            12 => vector!(thermostats),
            13 => vector!(humidistats),
            14 => nested!(setpoint_managers, |item: &crate::model::SetpointManager| (item.name.capacity(), item.name.capacity())),
            15 => vector!(ideal_loads),
            16 => vector!(zone_equipment),
            17 => nested!(air_loops, |item: &crate::model::ModelAirLoop| {
                (item.name.capacity().saturating_add(item.terminal_zone_ids.capacity()), item.name.capacity().saturating_add(item.terminal_zone_ids.capacity().saturating_mul(size_of::<crate::model::EntityId>())))
            }),
            18 => nested!(plant_loops, |item: &crate::model::PlantLoopConfig| {
                (item.name.capacity().saturating_add(item.equipment_ids.capacity()), item.name.capacity().saturating_add(item.equipment_ids.capacity().saturating_mul(size_of::<crate::model::EntityId>())))
            }),
            19 => vector!(outdoor_air_systems),
            20 => vector!(infiltrations),
            21 => vector!(mechanical_ventilations),
            22 => nested!(shading_surfaces, |item: &crate::model::ShadingSurface| {
                (item.name.capacity().saturating_add(item.vertices_m.capacity()), item.name.capacity().saturating_add(item.vertices_m.capacity().saturating_mul(size_of::<[f64; 3]>())))
            }),
            23 => nested!(space_lists, |item: &crate::model::SpaceList| {
                (item.name.capacity().saturating_add(item.space_ids.capacity()), item.name.capacity().saturating_add(item.space_ids.capacity().saturating_mul(size_of::<crate::model::EntityId>())))
            }),
            24 => nested!(thermal_enclosures, |item: &crate::model::ThermalEnclosure| {
                (item.name.capacity().saturating_add(item.zone_ids.capacity()), item.name.capacity().saturating_add(item.zone_ids.capacity().saturating_mul(size_of::<crate::model::EntityId>())))
            }),
            25 => vector!(adjacency_pairs),
            26 => {
                if self.index == 0 {
                    if let Some(item) = source.airflow_network.as_ref() {
                        self.charge_backing(item.zone_node_ids.capacity(), size_of::<(crate::model::EntityId, u32)>())?;
                        self.charge_backing(item.link_ids.capacity(), size_of::<u32>())?;
                    }
                    self.index = 1;
                } else {
                    self.charge(usize::from(source.airflow_network.is_some()), source.airflow_network.as_ref().map_or(0, size_of_val))?;
                }
            }
            27 => nested!(electrical_load_centers, |item: &crate::model::ElectricalLoadCenter| {
                (
                    item.name.capacity().saturating_add(item.generator_ids.capacity()).saturating_add(item.pv_ids.capacity()).saturating_add(item.battery_ids.capacity()),
                    item.name.capacity().saturating_add(item.generator_ids.capacity().saturating_add(item.pv_ids.capacity()).saturating_add(item.battery_ids.capacity()).saturating_mul(size_of::<crate::model::EntityId>())),
                )
            }),
            28 => vector!(pv_systems),
            29 => vector!(battery_storage),
            30 => vector!(shw_systems),
            31 => vector!(solar_thermal_systems),
            32 => vector!(refrigeration_systems),
            33 => vector!(water_systems),
            34 => vector!(faults),
            35 => nested!(output_variables, |item: &crate::model::OutputVariableSpec| {
                let bytes = item.name.capacity().saturating_add(item.key.capacity());
                (bytes, bytes)
            }),
            36 => vector!(sizing_objects),
            37 => vector!(daylight_zones),
            38 => vector!(room_air_models),
            39 => self.charge(1, size_of_val(&source.ground_temperature))?,
            40 => self.charge(1, size_of_val(&source.run_period))?,
            41 => vector!(schedules.constants),
            42 => vector!(schedules.daily),
            43 => vector!(schedules.weekly),
            44 => nested!(schedules.annual, |item: &crate::schedule::AnnualSchedule| {
                (item.rules.capacity().saturating_add(item.holiday_dates.capacity()), item.rules.capacity().saturating_mul(size_of::<crate::schedule::CompactScheduleRule>()).saturating_add(item.holiday_dates.capacity().saturating_mul(size_of::<(u16, u8, u8)>())))
            }),
            45 => nested!(schedules.time_series, |item: &crate::schedule::TimeSeriesSchedule| (item.values.capacity(), item.values.capacity().saturating_mul(size_of::<f64>()))),
            _ => return Ok(true),
        }
        Ok(self.lane > CAPTURE_LAST_LANE)
    }
}

fn vector_credit<T>(owner: &Vec<T>) -> (usize, usize) {
    (owner.capacity(), size_of::<T>())
}

struct ModelCapture {
    model: Model,
    lane: u8,
    index: usize,
    substage: u8,
    text_byte: usize,
}

impl ModelCapture {
    fn new() -> Self {
        Self { model: Model::default(), lane: 0, index: 0, substage: 0, text_byte: 0 }
    }

    fn next_lane(&mut self) {
        self.lane += 1;
        self.index = 0;
        self.substage = 0;
        self.text_byte = 0;
    }

    fn copy_text(target: &mut String, source: &String, byte: &mut usize) -> Result<bool, &'static str> {
        if target.capacity() == 0 && source.capacity() != 0 {
            target.try_reserve_exact(source.capacity()).map_err(|_| "energy.session.capture-string-reserve")?;
            return Ok(false);
        }
        if *byte < source.len() {
            let tail = &source[*byte..];
            let ch = tail.chars().next().ok_or("energy.session.capture-string-boundary")?;
            target.push(ch);
            *byte += ch.len_utf8();
            return Ok(false);
        }
        Ok(true)
    }

    fn copy_items<T: Copy>(target: &mut Vec<T>, source: &Vec<T>) -> Result<bool, &'static str> {
        if target.capacity() == 0 && source.capacity() != 0 {
            target.try_reserve_exact(source.capacity()).map_err(|_| "energy.session.capture-nested-vector-reserve")?;
            return Ok(false);
        }
        if let Some(item) = source.get(target.len()) {
            target.push(*item);
            return Ok(false);
        }
        Ok(true)
    }

    fn copy_rules(target: &mut Vec<crate::schedule::CompactScheduleRule>, source: &Vec<crate::schedule::CompactScheduleRule>) -> Result<bool, &'static str> {
        if target.capacity() == 0 && source.capacity() != 0 {
            target.try_reserve_exact(source.capacity()).map_err(|_| "energy.session.capture-nested-vector-reserve")?;
            return Ok(false);
        }
        if let Some(rule) = source.get(target.len()) {
            target.push(crate::schedule::CompactScheduleRule { start_month: rule.start_month, start_day: rule.start_day, end_month: rule.end_month, end_day: rule.end_day, daily_schedule_id: rule.daily_schedule_id });
            return Ok(false);
        }
        Ok(true)
    }

    fn step_one(&mut self, source: &Model) -> Result<bool, &'static str> {
        macro_rules! plain {
            ($($field:ident).+, $value:expr) => {{
                if self.model.$($field).+.capacity() == 0 && source.$($field).+.capacity() != 0 {
                    self.model.$($field).+.try_reserve_exact(source.$($field).+.capacity()).map_err(|_| "energy.session.capture-vector-reserve")?;
                } else if let Some(item) = source.$($field).+.get(self.index) {
                    let value = $value(item);
                    self.model.$($field).+.push(value);
                    self.index += 1;
                } else {
                    self.next_lane();
                }
                return Ok(false);
            }};
        }
        macro_rules! dynamic {
            ($($field:ident).+, $source_item:ident, $target_item:ident, $empty:expr, $body:block) => {{
                if self.model.$($field).+.capacity() == 0 && source.$($field).+.capacity() != 0 {
                    self.model.$($field).+.try_reserve_exact(source.$($field).+.capacity()).map_err(|_| "energy.session.capture-vector-reserve")?;
                    return Ok(false);
                }
                let Some($source_item) = source.$($field).+.get(self.index) else {
                    self.next_lane();
                    return Ok(false);
                };
                if self.model.$($field).+.len() == self.index {
                    self.model.$($field).+.push($empty($source_item));
                    return Ok(false);
                }
                let $target_item = self.model.$($field).+.get_mut(self.index).ok_or("energy.session.capture-record-missing")?;
                $body
                return Ok(false);
            }};
        }
        macro_rules! text {
            ($target:expr, $source:expr) => {{
                if Self::copy_text($target, $source, &mut self.text_byte)? {
                    self.text_byte = 0;
                    self.substage += 1;
                }
            }};
        }
        macro_rules! items {
            ($target:expr, $source:expr) => {{
                if Self::copy_items($target, $source)? {
                    self.substage += 1;
                }
            }};
        }
        macro_rules! finish_record {
            () => {{
                self.index += 1;
                self.substage = 0;
            }};
        }
        match self.lane {
            0 => {
                if Self::copy_text(&mut self.model.name, &source.name, &mut self.text_byte)? {
                    self.next_lane();
                }
            }
            1 => {
                if Self::copy_text(&mut self.model.version, &source.version, &mut self.text_byte)? {
                    self.next_lane();
                }
            }
            2 => {
                self.model.site = source.site;
                self.next_lane();
            }
            3 => dynamic!(
                zones,
                source_item,
                target_item,
                |item: &crate::model::Zone| crate::model::Zone { id: item.id, name: String::new(), volume_m3: item.volume_m3, multiplier: item.multiplier, conditioned: item.conditioned, part_of_total_floor_area: item.part_of_total_floor_area },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        _ => finish_record!(),
                    }
                }
            ),
            4 => dynamic!(spaces, source_item, target_item, |item: &crate::model::Space| crate::model::Space { id: item.id, name: String::new(), zone_id: item.zone_id, floor_area_m2: item.floor_area_m2 }, {
                match self.substage {
                    0 => text!(&mut target_item.name, &source_item.name),
                    _ => finish_record!(),
                }
            }),
            5 => dynamic!(
                surfaces,
                source_item,
                target_item,
                |item: &crate::model::Surface| crate::model::Surface {
                    id: item.id,
                    name: String::new(),
                    zone_id: item.zone_id,
                    class: item.class,
                    vertices_m: Vec::new(),
                    construction_id: item.construction_id,
                    outside_boundary_condition: item.outside_boundary_condition,
                    sun_exposed: item.sun_exposed,
                    wind_exposed: item.wind_exposed,
                    multiplier: item.multiplier,
                },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        1 => items!(&mut target_item.vertices_m, &source_item.vertices_m),
                        _ => finish_record!(),
                    }
                }
            ),
            6 => dynamic!(
                fenestrations,
                source_item,
                target_item,
                |item: &crate::model::Fenestration| crate::model::Fenestration {
                    id: item.id,
                    name: String::new(),
                    surface_id: item.surface_id,
                    u_value_w_m2k: item.u_value_w_m2k,
                    shgc: item.shgc,
                    vlt: item.vlt,
                    area_m2: item.area_m2,
                    height_m: item.height_m,
                    sill_height_m: item.sill_height_m,
                    frame_conductance_w_k: item.frame_conductance_w_k,
                    divider_conductance_w_k: item.divider_conductance_w_k,
                    overhang_depth_m: item.overhang_depth_m,
                    overhang_offset_m: item.overhang_offset_m,
                    fin_depth_m: item.fin_depth_m,
                    fin_offset_m: item.fin_offset_m,
                    glazing_construction_id: item.glazing_construction_id,
                },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        _ => finish_record!(),
                    }
                }
            ),
            7 => dynamic!(
                materials,
                source_item,
                target_item,
                |item: &crate::model::Material| crate::model::Material {
                    id: item.id,
                    name: String::new(),
                    thickness_m: item.thickness_m,
                    conductivity_w_m_k: item.conductivity_w_m_k,
                    density_kg_m3: item.density_kg_m3,
                    specific_heat_j_kg_k: item.specific_heat_j_kg_k,
                    thermal_absorptance: item.thermal_absorptance,
                    solar_absorptance: item.solar_absorptance,
                    visible_absorptance: item.visible_absorptance,
                },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        _ => finish_record!(),
                    }
                }
            ),
            8 => dynamic!(constructions, source_item, target_item, |item: &crate::model::Construction| crate::model::Construction { id: item.id, name: String::new(), layer_material_ids: Vec::new() }, {
                match self.substage {
                    0 => text!(&mut target_item.name, &source_item.name),
                    1 => items!(&mut target_item.layer_material_ids, &source_item.layer_material_ids),
                    _ => finish_record!(),
                }
            }),
            9 => plain!(people, |item: &crate::model::PeopleGain| crate::model::PeopleGain {
                id: item.id,
                zone_id: item.zone_id,
                schedule_id: item.schedule_id,
                activity_schedule_id: item.activity_schedule_id,
                people_per_area: item.people_per_area,
                sensible_fraction: item.sensible_fraction,
                latent_fraction: item.latent_fraction,
                radiant_fraction: item.radiant_fraction,
            }),
            10 => plain!(lighting, |item: &crate::model::LightingGain| crate::model::LightingGain {
                id: item.id,
                zone_id: item.zone_id,
                schedule_id: item.schedule_id,
                watts_per_area: item.watts_per_area,
                radiant_fraction: item.radiant_fraction,
                visible_fraction: item.visible_fraction,
                return_air_fraction: item.return_air_fraction,
            }),
            11 => plain!(equipment, |item: &crate::model::EquipmentGain| crate::model::EquipmentGain {
                id: item.id,
                zone_id: item.zone_id,
                schedule_id: item.schedule_id,
                watts_per_area: item.watts_per_area,
                radiant_fraction: item.radiant_fraction,
                latent_fraction: item.latent_fraction,
            }),
            12 => plain!(thermostats, |item: &crate::model::Thermostat| crate::model::Thermostat {
                id: item.id,
                zone_id: item.zone_id,
                heating_setpoint_schedule_id: item.heating_setpoint_schedule_id,
                cooling_setpoint_schedule_id: item.cooling_setpoint_schedule_id,
                heating_throttle_range_k: item.heating_throttle_range_k,
                cooling_throttle_range_k: item.cooling_throttle_range_k,
            }),
            13 => plain!(humidistats, |item: &crate::model::Humidistat| crate::model::Humidistat {
                id: item.id,
                zone_id: item.zone_id,
                humidifying_setpoint_schedule_id: item.humidifying_setpoint_schedule_id,
                dehumidifying_setpoint_schedule_id: item.dehumidifying_setpoint_schedule_id,
                humidifying_throttle_range: item.humidifying_throttle_range,
                dehumidifying_throttle_range: item.dehumidifying_throttle_range,
            }),
            14 => dynamic!(
                setpoint_managers,
                source_item,
                target_item,
                |item: &crate::model::SetpointManager| crate::model::SetpointManager {
                    id: item.id,
                    name: String::new(),
                    kind: match &item.kind {
                        crate::model::SetpointManagerKind::Scheduled => crate::model::SetpointManagerKind::Scheduled,
                        crate::model::SetpointManagerKind::OutdoorAirReset { low_outdoor_c, high_outdoor_c, low_setpoint_c, high_setpoint_c } =>
                            crate::model::SetpointManagerKind::OutdoorAirReset { low_outdoor_c: *low_outdoor_c, high_outdoor_c: *high_outdoor_c, low_setpoint_c: *low_setpoint_c, high_setpoint_c: *high_setpoint_c },
                        crate::model::SetpointManagerKind::WarmestZone => crate::model::SetpointManagerKind::WarmestZone,
                        crate::model::SetpointManagerKind::ColdestZone => crate::model::SetpointManagerKind::ColdestZone,
                    },
                    schedule_id: item.schedule_id,
                },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        _ => finish_record!(),
                    }
                }
            ),
            15 => plain!(ideal_loads, |item: &crate::model::IdealLoadsSystem| crate::model::IdealLoadsSystem {
                id: item.id,
                zone_id: item.zone_id,
                max_heating_supply_air_temp_c: item.max_heating_supply_air_temp_c,
                min_cooling_supply_air_temp_c: item.min_cooling_supply_air_temp_c,
                max_heating_capacity_w: item.max_heating_capacity_w,
                max_cooling_capacity_w: item.max_cooling_capacity_w,
                outdoor_air_per_person_m3_s: item.outdoor_air_per_person_m3_s,
                outdoor_air_per_area_m3_s_m2: item.outdoor_air_per_area_m3_s_m2,
            }),
            16 => plain!(zone_equipment, |item: &crate::model::ZoneEquipmentAssignment| crate::model::ZoneEquipmentAssignment {
                id: item.id,
                zone_id: item.zone_id,
                equipment_type: match &item.equipment_type {
                    crate::model::ZoneEquipmentType::Baseboard => crate::model::ZoneEquipmentType::Baseboard,
                    crate::model::ZoneEquipmentType::Radiant => crate::model::ZoneEquipmentType::Radiant,
                    crate::model::ZoneEquipmentType::FanCoil => crate::model::ZoneEquipmentType::FanCoil,
                    crate::model::ZoneEquipmentType::Ptac => crate::model::ZoneEquipmentType::Ptac,
                    crate::model::ZoneEquipmentType::VrfTerminal => crate::model::ZoneEquipmentType::VrfTerminal,
                    crate::model::ZoneEquipmentType::Erv => crate::model::ZoneEquipmentType::Erv,
                    crate::model::ZoneEquipmentType::UnitHeater => crate::model::ZoneEquipmentType::UnitHeater,
                    crate::model::ZoneEquipmentType::WaterToAirHp => crate::model::ZoneEquipmentType::WaterToAirHp,
                },
                priority: item.priority,
                heating_capacity_w: item.heating_capacity_w,
                cooling_capacity_w: item.cooling_capacity_w,
            }),
            17 => dynamic!(
                air_loops,
                source_item,
                target_item,
                |item: &crate::model::ModelAirLoop| crate::model::ModelAirLoop {
                    id: item.id,
                    name: String::new(),
                    supply_node_id: item.supply_node_id,
                    return_node_id: item.return_node_id,
                    design_supply_air_flow_m3_s: item.design_supply_air_flow_m3_s,
                    terminal_zone_ids: Vec::new(),
                },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        1 => items!(&mut target_item.terminal_zone_ids, &source_item.terminal_zone_ids),
                        _ => finish_record!(),
                    }
                }
            ),
            18 => dynamic!(
                plant_loops,
                source_item,
                target_item,
                |item: &crate::model::PlantLoopConfig| crate::model::PlantLoopConfig {
                    id: item.id,
                    name: String::new(),
                    loop_type: item.loop_type,
                    supply_temperature_c: item.supply_temperature_c,
                    return_temperature_c: item.return_temperature_c,
                    design_flow_kg_s: item.design_flow_kg_s,
                    equipment_ids: Vec::new(),
                },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        1 => items!(&mut target_item.equipment_ids, &source_item.equipment_ids),
                        _ => finish_record!(),
                    }
                }
            ),
            19 => {
                plain!(outdoor_air_systems, |item: &crate::model::OutdoorAirSystem| crate::model::OutdoorAirSystem { id: item.id, air_loop_id: item.air_loop_id, min_oa_flow_m3_s: item.min_oa_flow_m3_s, economizer_enabled: item.economizer_enabled })
            }
            20 => plain!(infiltrations, |item: &crate::model::Infiltration| crate::model::Infiltration {
                id: item.id,
                zone_id: item.zone_id,
                schedule_id: item.schedule_id,
                method: item.method,
                design_flow_ach: item.design_flow_ach,
                flow_per_exterior_area_m3_s_m2: item.flow_per_exterior_area_m3_s_m2,
                effective_leakage_area_m2: item.effective_leakage_area_m2,
                discharge_coefficient: item.discharge_coefficient,
                stack_height_m: item.stack_height_m,
                constant_term_coefficient: item.constant_term_coefficient,
                temperature_term_coefficient: item.temperature_term_coefficient,
                velocity_term_coefficient: item.velocity_term_coefficient,
                velocity_squared_term_coefficient: item.velocity_squared_term_coefficient
            }),
            21 => plain!(mechanical_ventilations, |item: &crate::model::MechanicalVentilation| crate::model::MechanicalVentilation {
                id: item.id,
                zone_id: item.zone_id,
                schedule_id: item.schedule_id,
                design_flow_m3_s: item.design_flow_m3_s,
                fan_total_efficiency: item.fan_total_efficiency,
                fan_delta_pressure_pa: item.fan_delta_pressure_pa
            }),
            22 => dynamic!(
                shading_surfaces,
                source_item,
                target_item,
                |item: &crate::model::ShadingSurface| crate::model::ShadingSurface { id: item.id, name: String::new(), vertices_m: Vec::new(), transmittance_schedule_id: item.transmittance_schedule_id },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        1 => items!(&mut target_item.vertices_m, &source_item.vertices_m),
                        _ => finish_record!(),
                    }
                }
            ),
            23 => dynamic!(space_lists, source_item, target_item, |item: &crate::model::SpaceList| crate::model::SpaceList { id: item.id, name: String::new(), space_ids: Vec::new() }, {
                match self.substage {
                    0 => text!(&mut target_item.name, &source_item.name),
                    1 => items!(&mut target_item.space_ids, &source_item.space_ids),
                    _ => finish_record!(),
                }
            }),
            24 => dynamic!(thermal_enclosures, source_item, target_item, |item: &crate::model::ThermalEnclosure| crate::model::ThermalEnclosure { id: item.id, name: String::new(), zone_ids: Vec::new() }, {
                match self.substage {
                    0 => text!(&mut target_item.name, &source_item.name),
                    1 => items!(&mut target_item.zone_ids, &source_item.zone_ids),
                    _ => finish_record!(),
                }
            }),
            25 => plain!(adjacency_pairs, |item: &crate::model::AdjacencyPair| crate::model::AdjacencyPair { surface_a_id: item.surface_a_id, surface_b_id: item.surface_b_id }),
            26 => {
                let Some(source_item) = source.airflow_network.as_ref() else {
                    self.next_lane();
                    return Ok(false);
                };
                if self.model.airflow_network.is_none() {
                    self.model.airflow_network = Some(crate::model::AirflowNetworkDefinition { zone_node_ids: Vec::new(), outdoor_node_id: source_item.outdoor_node_id, link_ids: Vec::new() });
                    return Ok(false);
                }
                let target_item = self.model.airflow_network.as_mut().expect("mounted airflow capture");
                match self.substage {
                    0 => items!(&mut target_item.zone_node_ids, &source_item.zone_node_ids),
                    1 => items!(&mut target_item.link_ids, &source_item.link_ids),
                    _ => self.next_lane(),
                }
            }
            27 => dynamic!(
                electrical_load_centers,
                source_item,
                target_item,
                |item: &crate::model::ElectricalLoadCenter| crate::model::ElectricalLoadCenter { id: item.id, name: String::new(), generator_ids: Vec::new(), pv_ids: Vec::new(), battery_ids: Vec::new() },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        1 => items!(&mut target_item.generator_ids, &source_item.generator_ids),
                        2 => items!(&mut target_item.pv_ids, &source_item.pv_ids),
                        3 => items!(&mut target_item.battery_ids, &source_item.battery_ids),
                        _ => finish_record!(),
                    }
                }
            ),
            28 => plain!(pv_systems, |item: &crate::model::PvSystemAssignment| crate::model::PvSystemAssignment {
                id: item.id,
                dc_capacity_w: item.dc_capacity_w,
                area_m2: item.area_m2,
                tilt_deg: item.tilt_deg,
                azimuth_deg: item.azimuth_deg,
                module_efficiency: item.module_efficiency,
                inverter_efficiency: item.inverter_efficiency
            }),
            29 => plain!(battery_storage, |item: &crate::model::BatteryAssignment| crate::model::BatteryAssignment {
                id: item.id,
                capacity_kwh: item.capacity_kwh,
                max_charge_w: item.max_charge_w,
                max_discharge_w: item.max_discharge_w,
                round_trip_efficiency: item.round_trip_efficiency
            }),
            30 => plain!(shw_systems, |item: &crate::model::ShwSystemConfig| crate::model::ShwSystemConfig {
                id: item.id,
                heater_capacity_w: item.heater_capacity_w,
                storage_volume_m3: item.storage_volume_m3,
                setpoint_c: item.setpoint_c,
                schedule_id: item.schedule_id
            }),
            31 => plain!(solar_thermal_systems, |item: &crate::model::SolarThermalConfig| crate::model::SolarThermalConfig {
                id: item.id,
                collector_area_m2: item.collector_area_m2,
                efficiency: item.efficiency,
                storage_volume_m3: item.storage_volume_m3,
                tilt_deg: item.tilt_deg,
                azimuth_deg: item.azimuth_deg
            }),
            32 => {
                plain!(refrigeration_systems, |item: &crate::model::RefrigerationConfig| crate::model::RefrigerationConfig { id: item.id, case_count: item.case_count, design_load_w: item.design_load_w, defrost_schedule_id: item.defrost_schedule_id })
            }
            33 => plain!(water_systems, |item: &crate::model::WaterSystemConfig| crate::model::WaterSystemConfig { id: item.id, fixture_count: item.fixture_count, peak_flow_l_s: item.peak_flow_l_s, schedule_id: item.schedule_id }),
            34 => plain!(faults, |item: &crate::model::FaultDefinition| crate::model::FaultDefinition {
                id: item.id,
                target_equipment_id: item.target_equipment_id,
                fault_type: item.fault_type,
                severity: item.severity,
                start_schedule_id: item.start_schedule_id
            }),
            35 => dynamic!(output_variables, source_item, target_item, |item: &crate::model::OutputVariableSpec| crate::model::OutputVariableSpec { name: String::new(), key: String::new(), reporting_frequency: item.reporting_frequency }, {
                match self.substage {
                    0 => text!(&mut target_item.name, &source_item.name),
                    1 => text!(&mut target_item.key, &source_item.key),
                    _ => finish_record!(),
                }
            }),
            36 => plain!(sizing_objects, |item: &crate::model::SizingObject| crate::model::SizingObject { id: item.id, zone_id: item.zone_id, sizing_type: item.sizing_type, design_day_type: item.design_day_type }),
            37 => plain!(daylight_zones, |item: &crate::model::DaylightZoneConfig| crate::model::DaylightZoneConfig {
                id: item.id,
                zone_id: item.zone_id,
                illuminance_target_lux: item.illuminance_target_lux,
                glare_limit: item.glare_limit,
                window_transmittance: item.window_transmittance
            }),
            38 => plain!(room_air_models, |item: &crate::model::RoomAirModelAssignment| crate::model::RoomAirModelAssignment { zone_id: item.zone_id, model: item.model }),
            39 => {
                self.model.ground_temperature = crate::model::GroundTemperatureConfig { building_surface_c: source.ground_temperature.building_surface_c, shallow_c: source.ground_temperature.shallow_c, deep_c: source.ground_temperature.deep_c };
                self.next_lane();
            }
            40 => {
                self.model.run_period = source.run_period;
                self.next_lane();
            }
            41 => plain!(schedules.constants, |item: &crate::schedule::ConstantSchedule| crate::schedule::ConstantSchedule { id: item.id, value: item.value }),
            42 => plain!(schedules.daily, |item: &crate::schedule::DailySchedule| crate::schedule::DailySchedule { id: item.id, hourly_values: item.hourly_values, interpolation: item.interpolation, limits: item.limits }),
            43 => plain!(schedules.weekly, |item: &crate::schedule::WeeklySchedule| crate::schedule::WeeklySchedule { id: item.id, daily_schedule_ids: item.daily_schedule_ids }),
            44 => dynamic!(
                schedules.annual,
                source_item,
                target_item,
                |item: &crate::schedule::AnnualSchedule| crate::schedule::AnnualSchedule { id: item.id, rules: Vec::new(), default_daily_schedule_id: item.default_daily_schedule_id, holiday_daily_schedule_id: item.holiday_daily_schedule_id, holiday_dates: Vec::new() },
                {
                    match self.substage {
                        0 => {
                            if Self::copy_rules(&mut target_item.rules, &source_item.rules)? {
                                self.substage += 1;
                            }
                        }
                        1 => items!(&mut target_item.holiday_dates, &source_item.holiday_dates),
                        _ => finish_record!(),
                    }
                }
            ),
            45 => dynamic!(schedules.time_series, source_item, target_item, |item: &crate::schedule::TimeSeriesSchedule| crate::schedule::TimeSeriesSchedule { id: item.id, values: Vec::new(), timestep_seconds: item.timestep_seconds }, {
                match self.substage {
                    0 => items!(&mut target_item.values, &source_item.values),
                    _ => finish_record!(),
                }
            }),
            _ => return Ok(true),
        }
        Ok(self.lane > CAPTURE_LAST_LANE)
    }

    fn finish(self) -> Model {
        self.model
    }
}
//#endregion 🧮️RetainedInput

//#region 🧵️RunJob
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EnergyRunPhase {
    Census,
    Capture,
    Simulate,
    Settled,
}

/// 🎟️ What one bounded grant of the run produced.
enum EnergyRunGrant {
    Continue,
    Timestep,
    Settled(StepOutcome),
}

/// 🧭️ The run's projection of the numerical cursor: timestep units, tier publications and the live meter.
#[derive(Clone, Copy, Debug)]
struct EnergyRunCursor {
    stage: EnergyJobStage,
    tier: EnergyQualityTier,
    timestep: u32,
    total_timesteps: u32,
    warmup_hours: u32,
    warmup_timesteps: u64,
    run_timesteps: u64,
    tiers_published: u64,
    facility_electricity_kwh: f64,
}

impl EnergyRunCursor {
    fn new() -> Self {
        Self { stage: EnergyJobStage::Validate, tier: EnergyQualityTier::SteadyStateEstimate, timestep: 0, total_timesteps: 0, warmup_hours: 0, warmup_timesteps: 0, run_timesteps: 0, tiers_published: 0, facility_electricity_kwh: 0.0 }
    }

    fn completed(&self) -> u64 {
        self.warmup_timesteps + self.run_timesteps
    }

    fn is_warmup(stage: EnergyJobStage) -> bool {
        matches!(stage, EnergyJobStage::WarmupTimestep | EnergyJobStage::WarmupConvergence)
    }

    fn publish_tier(&mut self, writer: &mut ToolRunTickWriter) {
        let kwh = (self.facility_electricity_kwh * 1_000.0).round() / 1_000.0;
        let args = [ToolRunStepArg::Float(kwh), ToolRunStepArg::Unsigned(self.run_timesteps), ToolRunStepArg::Unsigned(u64::from(self.total_timesteps))];
        let _ = writer.step(ToolRunStepKind::Success, EnergySimulationRunStage::of(self.stage).index(), EnergySimulationRunReason::of_tier(self.tier).code(), None, &args);
        self.tiers_published += 1;
    }

    /// 🔭️ Folds one numerical cursor; `true` when exactly one warmup or run timestep was computed.
    fn observe(&mut self, cursor: EnergyJobCursor, writer: &mut ToolRunTickWriter) -> bool {
        let warmup = self.stage == EnergyJobStage::WarmupTimestep && cursor.stage == EnergyJobStage::WarmupConvergence;
        let run = cursor.timestep > self.timestep;
        self.warmup_timesteps += u64::from(warmup);
        self.run_timesteps += u64::from(cursor.timestep.saturating_sub(self.timestep));
        if Self::is_warmup(self.stage) && !Self::is_warmup(cursor.stage) && self.warmup_timesteps < u64::from(cursor.warmup_hours) {
            let _ = writer.step(ToolRunStepKind::Info, EnergySimulationRunStage::Warmup.index(), EnergySimulationRunReason::WarmupConverged.code(), None, &[ToolRunStepArg::Unsigned(self.warmup_timesteps)]);
        }
        self.timestep = cursor.timestep;
        self.total_timesteps = cursor.total_timesteps;
        self.warmup_hours = cursor.warmup_hours;
        self.facility_electricity_kwh = cursor.facility_electricity_kwh;
        if cursor.tier != self.tier {
            self.publish_tier(writer);
            self.tier = cursor.tier;
        }
        self.stage = cursor.stage;
        warmup || run
    }

    fn progress(&self, identity: ToolRunIdentity, state: ToolRunState, stage: EnergySimulationRunStage) -> ToolRunProgress {
        let values = [self.warmup_timesteps, self.run_timesteps, self.tiers_published, (self.facility_electricity_kwh * 1_000.0).round().max(0.0) as u64];
        ToolRunProgress {
            identity,
            sequence: 0,
            state,
            stage: stage.index(),
            completed: self.completed(),
            total: (stage.index() >= EnergySimulationRunStage::Run.index()).then(|| self.warmup_timesteps + u64::from(self.total_timesteps)),
            counters: EnergySimulationRunCounter::ALL.iter().zip(values).map(|(counter, value)| ToolRunCounter { counter: counter.index(), value }).collect(),
            units_per_second: 0.0,
            conflicts: 0,
            steps: ToolRunStepRing::new(),
        }
    }
}

fn close_payload(payload: &mut RetainedJobPayload) {
    while !payload.terminal_is_empty() {
        let _ = payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
}

fn fault_outcome() -> StepOutcome {
    StepOutcome::Fault(JobFault { detail: RetainedJobPayload::empty(JobPayloadStream::Fault) })
}

/// ⏯️ The simulation `runJob`: captures the run's base model in bounded grants, admits the numerical
/// `EnergyJob` and steps it inline under the caller's deadline. It consumes one unit of fuel per computed
/// timestep and flushes one tick whenever at least one timestep was computed or the run settled; the
/// settled outcome (`Complete` with the numerical commit candidate, `Fault`, `Cancelled`) follows on the next call.
pub struct EnergySimulationRunJob {
    identity: ToolRunIdentity,
    writer: ToolRunTickWriter,
    snapshot: Option<Arc<EnergyModelSnapshot>>,
    template: SimulationConfig,
    phase: EnergyRunPhase,
    census: CaptureCensus,
    capture: Option<ModelCapture>,
    capture_close: Option<EnergyModelCloseCursor>,
    numerical: Option<EnergyJob>,
    rejected: Option<EnergyAdmissionRejected>,
    operation: OperationId,
    generation: Generation,
    numerical_sequence: u64,
    cursor: EnergyRunCursor,
    settled: Option<StepOutcome>,
    closing: bool,
}

impl EnergySimulationRunJob {
    pub fn new(identity: ToolRunIdentity, snapshot: Arc<EnergyModelSnapshot>, template: SimulationConfig) -> Self {
        Self {
            identity,
            writer: ToolRunTickWriter::new(identity),
            snapshot: Some(snapshot),
            template,
            phase: EnergyRunPhase::Census,
            census: CaptureCensus::new(),
            capture: None,
            capture_close: None,
            numerical: None,
            rejected: None,
            operation: semio_framework_job::allocate_operation_id(),
            generation: Generation(u64::from(identity.generation)),
            numerical_sequence: 0,
            cursor: EnergyRunCursor::new(),
            settled: None,
            closing: false,
        }
    }

    fn stage(&self) -> EnergySimulationRunStage {
        if self.numerical.is_some() {
            EnergySimulationRunStage::of(self.cursor.stage)
        } else {
            EnergySimulationRunStage::Capture
        }
    }

    fn refuse(&mut self, reason: EnergySimulationRunReason, args: &[ToolRunStepArg]) -> EnergyRunGrant {
        let _ = self.writer.step(ToolRunStepKind::Danger, self.stage().index(), reason.code(), None, args);
        self.phase = EnergyRunPhase::Settled;
        EnergyRunGrant::Settled(fault_outcome())
    }

    fn census_one(&mut self) -> EnergyRunGrant {
        let Some(snapshot) = self.snapshot.as_ref() else { return self.refuse(EnergySimulationRunReason::CaptureRejected, &[]) };
        match self.census.step_one(&snapshot.model) {
            Ok(false) => EnergyRunGrant::Continue,
            Ok(true) => {
                self.capture = Some(ModelCapture::new());
                self.phase = EnergyRunPhase::Capture;
                EnergyRunGrant::Continue
            }
            Err(_) => self.refuse(EnergySimulationRunReason::CaptureRejected, &[]),
        }
    }

    fn capture_one(&mut self) -> EnergyRunGrant {
        let (Some(snapshot), Some(capture)) = (self.snapshot.as_ref(), self.capture.as_mut()) else { return self.refuse(EnergySimulationRunReason::CaptureRejected, &[]) };
        match capture.step_one(&snapshot.model) {
            Ok(false) => EnergyRunGrant::Continue,
            Ok(true) => self.admit(),
            Err(_) => self.refuse(EnergySimulationRunReason::CaptureRejected, &[]),
        }
    }

    fn admit(&mut self) -> EnergyRunGrant {
        let Some(capture) = self.capture.take() else { return self.refuse(EnergySimulationRunReason::CaptureRejected, &[]) };
        let model = capture.finish();
        let config = simulation_config_for(&self.template, &model);
        let base = self.identity.base_revision[..8].try_into().map_or(0, u64::from_be_bytes);
        let operation = semio_framework_job::Operation::new(self.operation, semio_framework_job::RevisionId(base), self.generation, self.identity.id.run ^ base);
        self.snapshot = None;
        match EnergyJob::admit(operation, model, config, EnergyNumericalBounds::default()) {
            Ok(job) => {
                self.numerical = Some(job);
                self.phase = EnergyRunPhase::Simulate;
                EnergyRunGrant::Continue
            }
            Err(rejected) => {
                self.rejected = Some(rejected);
                self.refuse(EnergySimulationRunReason::AdmissionRejected, &[])
            }
        }
    }

    /// 🦶️ Steps the numerical job until one timestep is computed, the run settles or the caller must yield. Each
/// numerical step gets its own context, because a step context grants at most one payload page.
    fn simulate(&mut self, cx: &StepContext<'_>) -> EnergyRunGrant {
        let Self { numerical, numerical_sequence, operation, generation, cursor, writer, phase, .. } = self;
        let Some(job) = numerical.as_mut() else { return EnergyRunGrant::Settled(fault_outcome()) };
        loop {
            if cx.should_yield() {
                return EnergyRunGrant::Continue;
            }
            let mut inner = StepContext::new(*operation, *generation, StepBudget::new(semio_framework_job::INTERACTIVE_LANE_FUEL, cx.deadline_us()), cx.cancel_token(), semio_framework_job::default_now_us, numerical_sequence);
            let settled = match job.step(&mut inner) {
                StepOutcome::Yield => None,
                StepOutcome::PreviewReady(mut payload) => {
                    close_payload(&mut payload);
                    None
                }
                StepOutcome::CheckpointReady(mut checkpoint) => {
                    close_payload(&mut checkpoint.state);
                    (!acknowledge_checkpoint(job, *generation)).then(fault_outcome)
                }
                StepOutcome::Complete(candidate) => {
                    cursor.observe(job.cursor(), writer);
                    cursor.publish_tier(writer);
                    Some(StepOutcome::Complete(candidate))
                }
                StepOutcome::Fault(mut fault) => {
                    close_payload(&mut fault.detail);
                    let _ = writer.step(ToolRunStepKind::Danger, EnergySimulationRunStage::of(cursor.stage).index(), EnergySimulationRunReason::SimulationFaulted.code(), None, &[ToolRunStepArg::Unsigned(cursor.completed())]);
                    Some(fault_outcome())
                }
                StepOutcome::Cancelled => Some(StepOutcome::Cancelled),
            };
            if let Some(outcome) = settled {
                *phase = EnergyRunPhase::Settled;
                return EnergyRunGrant::Settled(outcome);
            }
            if cursor.observe(job.cursor(), writer) {
                return EnergyRunGrant::Timestep;
            }
        }
    }
}

/// 📮️ Retires the numerical job's retained restore checkpoint: a run restarts rather than restores.
fn acknowledge_checkpoint(job: &mut EnergyJob, generation: Generation) -> bool {
    let mut lease = match job.take_checkpoint_packet(generation) {
        Ok(Some(lease)) => lease,
        Ok(None) => return true,
        Err(_) => return false,
    };
    while !lease.packet().terminal_is_empty() {
        let _ = lease.packet_mut().ack_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    }
    job.ack_checkpoint_packet(lease).is_ok()
}

impl InteractiveJob for EnergySimulationRunJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if self.closing || cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if let Some(outcome) = self.settled.take() {
            return outcome;
        }
        let mut computed = false;
        let settled = loop {
            if cx.should_yield() || self.phase == EnergyRunPhase::Settled {
                break None;
            }
            let grant = match self.phase {
                EnergyRunPhase::Census => self.census_one(),
                EnergyRunPhase::Capture => self.capture_one(),
                EnergyRunPhase::Simulate => self.simulate(cx),
                EnergyRunPhase::Settled => EnergyRunGrant::Continue,
            };
            match grant {
                EnergyRunGrant::Continue => {}
                EnergyRunGrant::Timestep => {
                    computed = true;
                    cx.consume_fuel(1);
                }
                EnergyRunGrant::Settled(outcome) => break Some(outcome),
            }
        };
        if !computed && settled.is_none() {
            return StepOutcome::Yield;
        }
        let state = if matches!(settled, Some(StepOutcome::Complete(_))) { ToolRunState::Complete } else { ToolRunState::Running };
        self.writer.progress(self.cursor.progress(self.identity, state, self.stage()));
        let payload = self.writer.finish().and_then(|tick| tick.encode().ok()).and_then(|bytes| cx.payload_from_bytes(JobPayloadStream::Preview, &bytes).map_err(|rejected| drop(rejected.into_source())).ok());
        match payload {
            Some(payload) => {
                self.settled = settled;
                StepOutcome::PreviewReady(payload)
            }
            None => {
                if let Some(mut outcome) = settled {
                    while !matches!(outcome.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES), semio_framework_job::JobPayloadCloseStep::Complete) {}
                }
                fault_outcome()
            }
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(job) = self.numerical.as_mut() {
            InteractiveJob::begin_close(job);
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        self.begin_close();
        if maximum_items == 0 {
            return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if let Some(outcome) = self.settled.as_mut() {
            if matches!(outcome.close_step(1, maximum_bytes), semio_framework_job::JobPayloadCloseStep::Complete) {
                self.settled = None;
            }
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(job) = self.numerical.as_mut() {
            return match InteractiveJob::close_step(job, maximum_items, maximum_bytes) {
                InteractiveJobCloseStep::Complete => {
                    self.numerical = None;
                    InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
                pending => pending,
            };
        }
        if let Some(rejected) = self.rejected.as_mut() {
            return match rejected.close_step(maximum_items, maximum_bytes) {
                InteractiveJobCloseStep::Complete if rejected.terminal_is_empty() => {
                    self.rejected = None;
                    InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
                InteractiveJobCloseStep::Complete => InteractiveJobCloseStep::Blocked,
                pending => pending,
            };
        }
        if let Some(capture) = self.capture.take() {
            self.capture_close = Some(EnergyModelCloseCursor::new(capture.finish()));
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(close) = self.capture_close.as_mut() {
            return match close.close_step(maximum_bytes) {
                InteractiveJobCloseStep::Complete => {
                    self.capture_close = None;
                    InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
                pending => pending,
            };
        }
        if self.snapshot.take().is_some() {
            return InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        InteractiveJobCloseStep::Complete
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.settled.is_none() && self.numerical.is_none() && self.rejected.is_none() && self.capture.is_none() && self.capture_close.is_none() && self.snapshot.is_none()
    }
}
//#endregion 🧵️RunJob

//#region 🧪️Laws
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Laws
