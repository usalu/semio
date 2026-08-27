//! ⚡️ Artifact-neutral mounted Energy product session: admitted capture, worker job and immutable view.

use crate::artifacts::model::{EnergyModelReadLease, EnergyModelSnapshot};
use crate::{
    EnergyAdmissionRejected, EnergyCheckpointRejected, EnergyJob, EnergyJobPreview, EnergyJobStage, EnergyModelCloseCursor, EnergyNumericalBounds, EnergyQualityTier, EnergyRestoreJob, EnergyWireLease, EnergyWirePacket, Model, SimulationConfig,
};
use semio_framework::kernel::{Effect, JobPlacement};
use semio_framework_job::{CancelToken, Generation, InteractiveJob, Operation, OperationId, RevisionId, StepBudget, StepContext, StepOutcome};
use semio_framework_plugin::reactor::jobs::{BoundedJob, BoundedJobFactory, JobBudget, JobStep};
use semio_framework_plugin::{AppRenderOperationContext, ArtifactView, PluginCloseStep};
use std::cell::RefCell;
use std::rc::Rc;

//#region 🔖️Contract
pub const ENERGY_SIMULATION_JOB_KIND: &str = "semio.energy.mounted-simulation.v1";
pub const ENERGY_SIMULATION_EVENT_SCHEMA: &str = "semio.energy.simulation-event.v1";
const ACTIVE_SLOTS: usize = 16;
const SHELL_SLOTS: usize = 32;
const EVENT_SLOTS: usize = 64;
const MAXIMUM_CAPTURE_ITEMS: usize = 4_194_304;
const MAXIMUM_CAPTURE_BYTES: usize = 512 * 1_024 * 1_024;
const INPUT_BYTES: usize = 95;
const JOB_TAG: u64 = 0xe7c3_0000_0000_0000;
const JOB_COUNTER_MAXIMUM: u64 = 0x0000_ffff_ffff_ffff;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EnergySimulationConfigProjection {
    pub locale_de: bool,
    pub checkpoint_token: u64,
    pub zone_timestep_minutes: u32,
    pub system_timestep_minutes: u32,
    pub warmup_days: u32,
    pub run_period_start_month: u8,
    pub run_period_start_day: u8,
    pub run_period_end_month: u8,
    pub run_period_end_day: u8,
}

impl Default for EnergySimulationConfigProjection {
    fn default() -> Self {
        let config = SimulationConfig::default();
        Self {
            locale_de: false,
            checkpoint_token: 0,
            zone_timestep_minutes: config.zone_timestep_minutes,
            system_timestep_minutes: config.system_timestep_minutes,
            warmup_days: config.warmup_days,
            run_period_start_month: config.run_period_start_month,
            run_period_start_day: config.run_period_start_day,
            run_period_end_month: config.run_period_end_month,
            run_period_end_day: config.run_period_end_day,
        }
    }
}

impl EnergySimulationConfigProjection {
    fn validate(self) -> bool {
        (1..=60).contains(&self.zone_timestep_minutes)
            && (1..=60).contains(&self.system_timestep_minutes)
            && self.warmup_days <= 365
            && (1..=12).contains(&self.run_period_start_month)
            && (1..=31).contains(&self.run_period_start_day)
            && (1..=12).contains(&self.run_period_end_month)
            && (1..=31).contains(&self.run_period_end_day)
    }

    fn build(self) -> SimulationConfig {
        SimulationConfig {
            zone_timestep_minutes: self.zone_timestep_minutes,
            system_timestep_minutes: self.system_timestep_minutes,
            warmup_days: self.warmup_days,
            run_period_start_month: self.run_period_start_month,
            run_period_start_day: self.run_period_start_day,
            run_period_end_month: self.run_period_end_month,
            run_period_end_day: self.run_period_end_day,
            ..SimulationConfig::default()
        }
    }

    fn digest(self) -> u64 {
        let mut digest = 0xcbf2_9ce4_8422_2325u64;
        for byte in [
            self.zone_timestep_minutes.to_le_bytes().as_slice(),
            self.system_timestep_minutes.to_le_bytes().as_slice(),
            self.warmup_days.to_le_bytes().as_slice(),
            &[self.run_period_start_month],
            &[self.run_period_start_day],
            &[self.run_period_end_month],
            &[self.run_period_end_day],
        ]
        .into_iter()
        .flatten()
        {
            digest = (digest ^ u64::from(*byte)).wrapping_mul(0x100_0000_01b3);
        }
        digest
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EnergySimulationRequestIdentity {
    pub request: u64,
    pub operation: u64,
    pub generation: u64,
    pub config_digest: u64,
}

impl EnergySimulationRequestIdentity {
    fn valid(self) -> bool {
        self.request != 0 && self.operation != 0 && self.generation != 0 && self.config_digest != 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnergySimulationEventKind {
    Start { request: u64, config: EnergySimulationConfigProjection },
    Cancel(EnergySimulationRequestIdentity),
    Retry(EnergySimulationRequestIdentity),
    Discard(EnergySimulationRequestIdentity),
    Adopt(EnergySimulationRequestIdentity),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnergySimulationStatus {
    Idle,
    Admitting,
    Queued,
    Running,
    Cancelled,
    Faulted,
    FinalReady,
    Adopted,
    Closing,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnergyTierProjection {
    pub app_instance_id: u32,
    pub document_revision: RevisionId,
    pub document_generation: Generation,
    pub canonical_base_revision: [u8; 32],
    pub operation: OperationId,
    pub generation: Generation,
    pub config_digest: u64,
    pub sequence: u64,
    pub tier: EnergyQualityTier,
    pub stage: EnergyJobStage,
    pub warmup_hour: u32,
    pub timestep: u32,
    pub total_timesteps: u32,
    pub facility_electricity_kwh: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnergySimulationProjection {
    pub status: EnergySimulationStatus,
    pub request: u64,
    pub operation: OperationId,
    pub generation: Generation,
    pub config_digest: u64,
    pub locale_de: bool,
    pub latest_sequence: u64,
    pub latest_tier: Option<EnergyQualityTier>,
    pub tiers: [Option<EnergyTierProjection>; 4],
    pub checkpoint_ready: bool,
    pub checkpoint_token: u64,
    pub final_ready: bool,
    pub fault_ready: bool,
    pub adopted: bool,
}

impl EnergySimulationProjection {
    fn new(identity: MountedIdentity) -> Self {
        Self {
            status: EnergySimulationStatus::Admitting,
            request: identity.request,
            operation: identity.operation,
            generation: identity.generation,
            config_digest: identity.config_digest,
            locale_de: false,
            latest_sequence: 0,
            latest_tier: None,
            tiers: [None; 4],
            checkpoint_ready: false,
            checkpoint_token: 0,
            final_ready: false,
            fault_ready: false,
            adopted: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MountedIdentity {
    app_instance_id: u32,
    request: u64,
    document_revision: RevisionId,
    document_generation: Generation,
    canonical_base_revision: [u8; 32],
    operation: OperationId,
    generation: Generation,
    config_digest: u64,
    operation_seed: u64,
    job: u64,
}

impl MountedIdentity {
    fn operation(self) -> Operation {
        Operation::new(self.operation, self.document_revision, self.generation, self.operation_seed)
    }

    fn matches_render(self, render: AppRenderOperationContext) -> bool {
        self.app_instance_id == render.app_instance_id
            && self.document_revision == render.base_revision
            && self.document_generation == render.generation
            && self.canonical_base_revision == render.canonical_base_revision
            && self.generation.0 != 0
            && self.operation.0 != 0
    }

    fn matches_request(self, request: EnergySimulationRequestIdentity) -> bool {
        request.request == self.request && request.operation == self.operation.0 && request.generation == self.generation.0 && request.config_digest == self.config_digest
    }
}

fn quality_tier_index(tier: EnergyQualityTier) -> usize {
    match tier {
        EnergyQualityTier::SteadyStateEstimate => 0,
        EnergyQualityTier::DesignDay => 1,
        EnergyQualityTier::CoarseTimestep => 2,
        EnergyQualityTier::Final => 3,
    }
}

fn retain_worker_outcome(slot: &mut Option<StepOutcome>, outcome: StepOutcome) -> JobStep {
    let terminal = matches!(outcome, StepOutcome::Complete(_));
    *slot = Some(outcome);
    if terminal {
        JobStep::Done(Vec::new())
    } else {
        JobStep::Running(None)
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
        Self { lane: 0, index: 0, items: 2, bytes: std::mem::size_of::<Model>() + std::mem::size_of::<SimulationConfig>() }
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
            ($field:ident) => {{
                let (capacity, bytes) = vector_credit(&source.$field);
                self.charge(capacity, bytes)?;
            }};
        }
        macro_rules! nested {
            ($field:ident, $backing:expr) => {{
                if let Some(item) = source.$field.get(self.index) {
                    let (items, bytes) = $backing(item);
                    self.charge_observed(items, bytes)?;
                    self.index += 1;
                } else {
                    let (capacity, bytes) = vector_credit(&source.$field);
                    self.charge(capacity, bytes)?;
                }
            }};
        }
        match self.lane {
            0 => self.charge(source.name.capacity(), 1)?,
            1 => self.charge(source.version.capacity(), 1)?,
            2 => self.charge(1, std::mem::size_of_val(&source.site))?,
            3 => nested!(zones, |item: &crate::model::Zone| (item.name.capacity(), item.name.capacity())),
            4 => nested!(spaces, |item: &crate::model::Space| (item.name.capacity(), item.name.capacity())),
            5 => {
                nested!(surfaces, |item: &crate::model::Surface| { (item.name.capacity().saturating_add(item.vertices_m.capacity()), item.name.capacity().saturating_add(item.vertices_m.capacity().saturating_mul(std::mem::size_of::<[f64; 3]>())),) })
            }
            6 => nested!(fenestrations, |item: &crate::model::Fenestration| (item.name.capacity(), item.name.capacity())),
            7 => nested!(materials, |item: &crate::model::Material| (item.name.capacity(), item.name.capacity())),
            8 => nested!(constructions, |item: &crate::model::Construction| {
                (item.name.capacity().saturating_add(item.layer_material_ids.capacity()), item.name.capacity().saturating_add(item.layer_material_ids.capacity().saturating_mul(std::mem::size_of::<crate::model::EntityId>())))
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
                (item.name.capacity().saturating_add(item.terminal_zone_ids.capacity()), item.name.capacity().saturating_add(item.terminal_zone_ids.capacity().saturating_mul(std::mem::size_of::<crate::model::EntityId>())))
            }),
            18 => nested!(plant_loops, |item: &crate::model::PlantLoopConfig| {
                (item.name.capacity().saturating_add(item.equipment_ids.capacity()), item.name.capacity().saturating_add(item.equipment_ids.capacity().saturating_mul(std::mem::size_of::<crate::model::EntityId>())))
            }),
            19 => vector!(outdoor_air_systems),
            20 => vector!(infiltrations),
            21 => vector!(mechanical_ventilations),
            22 => nested!(shading_surfaces, |item: &crate::model::ShadingSurface| {
                (item.name.capacity().saturating_add(item.vertices_m.capacity()), item.name.capacity().saturating_add(item.vertices_m.capacity().saturating_mul(std::mem::size_of::<[f64; 3]>())))
            }),
            23 => nested!(space_lists, |item: &crate::model::SpaceList| {
                (item.name.capacity().saturating_add(item.space_ids.capacity()), item.name.capacity().saturating_add(item.space_ids.capacity().saturating_mul(std::mem::size_of::<crate::model::EntityId>())))
            }),
            24 => nested!(thermal_enclosures, |item: &crate::model::ThermalEnclosure| {
                (item.name.capacity().saturating_add(item.zone_ids.capacity()), item.name.capacity().saturating_add(item.zone_ids.capacity().saturating_mul(std::mem::size_of::<crate::model::EntityId>())))
            }),
            25 => vector!(adjacency_pairs),
            26 => {
                if self.index == 0 {
                    if let Some(item) = source.airflow_network.as_ref() {
                        self.charge_backing(item.zone_node_ids.capacity(), std::mem::size_of::<(crate::model::EntityId, u32)>())?;
                        self.charge_backing(item.link_ids.capacity(), std::mem::size_of::<u32>())?;
                    }
                    self.index = 1;
                } else {
                    self.charge(usize::from(source.airflow_network.is_some()), source.airflow_network.as_ref().map_or(0, std::mem::size_of_val))?;
                }
            }
            27 => nested!(electrical_load_centers, |item: &crate::model::ElectricalLoadCenter| {
                (
                    item.name.capacity().saturating_add(item.generator_ids.capacity()).saturating_add(item.pv_ids.capacity()).saturating_add(item.battery_ids.capacity()),
                    item.name.capacity().saturating_add(item.generator_ids.capacity().saturating_add(item.pv_ids.capacity()).saturating_add(item.battery_ids.capacity()).saturating_mul(std::mem::size_of::<crate::model::EntityId>())),
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
            39 => self.charge(1, std::mem::size_of_val(&source.ground_temperature))?,
            _ => return Ok(true),
        }
        Ok(self.lane > 39)
    }
}

fn vector_credit<T>(owner: &Vec<T>) -> (usize, usize) {
    (owner.capacity(), std::mem::size_of::<T>())
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

    fn step_one(&mut self, source: &Model) -> Result<bool, &'static str> {
        macro_rules! plain {
            ($field:ident, $value:expr) => {{
                if self.model.$field.capacity() == 0 && source.$field.capacity() != 0 {
                    self.model.$field.try_reserve_exact(source.$field.capacity()).map_err(|_| "energy.session.capture-vector-reserve")?;
                } else if let Some(item) = source.$field.get(self.index) {
                    let value = $value(item);
                    self.model.$field.push(value);
                    self.index += 1;
                } else {
                    self.next_lane();
                }
                return Ok(false);
            }};
        }
        macro_rules! dynamic {
            ($field:ident, $empty:expr, $body:block) => {{
                if self.model.$field.capacity() == 0 && source.$field.capacity() != 0 {
                    self.model.$field.try_reserve_exact(source.$field.capacity()).map_err(|_| "energy.session.capture-vector-reserve")?;
                    return Ok(false);
                }
                let Some(source_item) = source.$field.get(self.index) else {
                    self.next_lane();
                    return Ok(false);
                };
                if self.model.$field.len() == self.index {
                    self.model.$field.push($empty(source_item));
                    return Ok(false);
                }
                let target_item = self.model.$field.get_mut(self.index).ok_or("energy.session.capture-record-missing")?;
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
                |item: &crate::model::Zone| crate::model::Zone { id: item.id, name: String::new(), volume_m3: item.volume_m3, multiplier: item.multiplier, conditioned: item.conditioned, part_of_total_floor_area: item.part_of_total_floor_area },
                {
                    match self.substage {
                        0 => text!(&mut target_item.name, &source_item.name),
                        _ => finish_record!(),
                    }
                }
            ),
            4 => dynamic!(spaces, |item: &crate::model::Space| crate::model::Space { id: item.id, name: String::new(), zone_id: item.zone_id, floor_area_m2: item.floor_area_m2 }, {
                match self.substage {
                    0 => text!(&mut target_item.name, &source_item.name),
                    _ => finish_record!(),
                }
            }),
            5 => dynamic!(
                surfaces,
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
                |item: &crate::model::Fenestration| crate::model::Fenestration {
                    id: item.id,
                    name: String::new(),
                    surface_id: item.surface_id,
                    u_value_w_m2k: item.u_value_w_m2k,
                    shgc: item.shgc,
                    vlt: item.vlt,
                    area_m2: item.area_m2,
                    frame_conductance_w_k: item.frame_conductance_w_k,
                    divider_conductance_w_k: item.divider_conductance_w_k,
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
            8 => dynamic!(constructions, |item: &crate::model::Construction| crate::model::Construction { id: item.id, name: String::new(), layer_material_ids: Vec::new() }, {
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
                flow_per_exterior_area_m3_s_m2: item.flow_per_exterior_area_m3_s_m2,
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
            22 => dynamic!(shading_surfaces, |item: &crate::model::ShadingSurface| crate::model::ShadingSurface { id: item.id, name: String::new(), vertices_m: Vec::new(), transmittance_schedule_id: item.transmittance_schedule_id }, {
                match self.substage {
                    0 => text!(&mut target_item.name, &source_item.name),
                    1 => items!(&mut target_item.vertices_m, &source_item.vertices_m),
                    _ => finish_record!(),
                }
            }),
            23 => dynamic!(space_lists, |item: &crate::model::SpaceList| crate::model::SpaceList { id: item.id, name: String::new(), space_ids: Vec::new() }, {
                match self.substage {
                    0 => text!(&mut target_item.name, &source_item.name),
                    1 => items!(&mut target_item.space_ids, &source_item.space_ids),
                    _ => finish_record!(),
                }
            }),
            24 => dynamic!(thermal_enclosures, |item: &crate::model::ThermalEnclosure| crate::model::ThermalEnclosure { id: item.id, name: String::new(), zone_ids: Vec::new() }, {
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
            27 => dynamic!(electrical_load_centers, |item: &crate::model::ElectricalLoadCenter| crate::model::ElectricalLoadCenter { id: item.id, name: String::new(), generator_ids: Vec::new(), pv_ids: Vec::new(), battery_ids: Vec::new() }, {
                match self.substage {
                    0 => text!(&mut target_item.name, &source_item.name),
                    1 => items!(&mut target_item.generator_ids, &source_item.generator_ids),
                    2 => items!(&mut target_item.pv_ids, &source_item.pv_ids),
                    3 => items!(&mut target_item.battery_ids, &source_item.battery_ids),
                    _ => finish_record!(),
                }
            }),
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
            35 => dynamic!(output_variables, |item: &crate::model::OutputVariableSpec| crate::model::OutputVariableSpec { name: String::new(), key: String::new(), reporting_frequency: item.reporting_frequency }, {
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
            _ => return Ok(true),
        }
        Ok(self.lane > 39)
    }

    fn finish(self) -> Model {
        self.model
    }
}

fn take_captured_model_for_admission(
    capture: &mut Option<ModelCapture>,
    mounted: MountedIdentity,
    expected: MountedIdentity,
    render: AppRenderOperationContext,
    live_request: u64,
    retained_config_digest: u64,
    snapshot_fresh: bool,
    cancelled: bool,
) -> Result<Model, &'static str> {
    if mounted != expected || !mounted.matches_render(render) || mounted.request != live_request || mounted.config_digest != retained_config_digest || !snapshot_fresh || cancelled {
        return Err("energy.session.admission-stale-or-cancelled");
    }
    Ok(std::mem::replace(capture, None).ok_or("energy.session.capture-missing")?.finish())
}
//#endregion 🧮️RetainedInput

//#region 🧰️FixedArena
#[derive(Clone, Copy)]
struct SessionEvent {
    sequence: u64,
    render: AppRenderOperationContext,
    kind: EnergySimulationEventKind,
}

struct MountedState {
    identity: MountedIdentity,
    snapshot: EnergyModelReadLease,
    capture: Option<ModelCapture>,
    capture_close: Option<EnergyModelCloseCursor>,
    config: EnergySimulationConfigProjection,
    job: Option<EnergyJob>,
    restore: Option<EnergyRestoreJob>,
    rejected: Option<EnergyAdmissionRejected>,
    restore_rejected: Option<EnergyCheckpointRejected>,
    cancel: CancelToken,
    preview_sequence: u64,
    outcome: Option<StepOutcome>,
    preview_packet: Option<EnergyWirePacket>,
    checkpoint: Option<EnergyWireLease>,
    checkpoint_packet: Option<(u64, EnergyWirePacket)>,
    retiring_checkpoint_packet: Option<EnergyWirePacket>,
    commit: Option<EnergyWireLease>,
    fault: Option<EnergyWireLease>,
    projection: EnergySimulationProjection,
    adopt_requested: bool,
    admission_blocked: bool,
    spawned: bool,
    worker_attached: bool,
    worker_returned: bool,
    closing: bool,
    close_lane: u8,
}

enum MountedAdmissionError {
    Stale { checkpoint: Option<EnergyWirePacket> },
    Rejected(&'static str),
}

impl MountedState {
    fn new(identity: MountedIdentity, snapshot: store::SnapshotRead<EnergyModelSnapshot>, config: EnergySimulationConfigProjection) -> Self {
        let mut projection = EnergySimulationProjection::new(identity);
        projection.locale_de = config.locale_de;
        projection.checkpoint_token = config.checkpoint_token;
        Self {
            identity,
            snapshot: EnergyModelReadLease::new(snapshot, identity.document_generation.0, identity.canonical_base_revision),
            capture: Some(ModelCapture::new()),
            capture_close: None,
            config,
            job: None,
            restore: None,
            rejected: None,
            restore_rejected: None,
            cancel: semio_framework_job::root_cancel_token(),
            preview_sequence: 0,
            outcome: None,
            preview_packet: None,
            checkpoint: None,
            checkpoint_packet: None,
            retiring_checkpoint_packet: None,
            commit: None,
            fault: None,
            projection,
            adopt_requested: false,
            admission_blocked: false,
            spawned: false,
            worker_attached: false,
            worker_returned: false,
            closing: false,
            close_lane: 0,
        }
    }

    fn snapshot_is_fresh(&self) -> bool {
        self.snapshot.model().is_some()
    }

    fn capture_one(&mut self) -> Result<bool, &'static str> {
        if !self.snapshot_is_fresh() || self.cancel.is_cancelled_now() {
            return Err("energy.session.capture-stale-or-cancelled");
        }
        let source = self.snapshot.model().ok_or("energy.session.snapshot-missing")?;
        let capture = self.capture.as_mut().ok_or("energy.session.capture-missing")?;
        capture.step_one(source)
    }

    fn admit_job(&mut self, render: AppRenderOperationContext, live_request: u64, expected: MountedIdentity, checkpoint: Option<EnergyWirePacket>) -> Result<(), MountedAdmissionError> {
        if self.config.checkpoint_token != 0 && checkpoint.is_none() {
            self.admission_blocked = true;
            self.projection.status = EnergySimulationStatus::Faulted;
            return Err(MountedAdmissionError::Rejected("energy.session.checkpoint-token-not-found"));
        }
        let config = self.config.build();
        let snapshot_fresh = self.snapshot_is_fresh();
        let cancelled = self.cancel.is_cancelled_now();
        let model = match take_captured_model_for_admission(&mut self.capture, self.identity, expected, render, live_request, self.config.digest(), snapshot_fresh, cancelled) {
            Ok(model) => model,
            Err(_) => return Err(MountedAdmissionError::Stale { checkpoint }),
        };
        if self.config.checkpoint_token != 0 {
            let packet = checkpoint.expect("checkpoint presence was retained before owner move");
            return match EnergyRestoreJob::admit(self.identity.operation(), model, config, packet, EnergyNumericalBounds::default()) {
                Ok(restore) => {
                    self.restore = Some(restore);
                    self.projection.status = EnergySimulationStatus::Queued;
                    Ok(())
                }
                Err(rejected) => {
                    self.restore_rejected = Some(rejected);
                    self.projection.status = EnergySimulationStatus::Faulted;
                    Err(MountedAdmissionError::Rejected("energy.session.restore-admission-rejected"))
                }
            };
        }
        match EnergyJob::admit(self.identity.operation(), model, config, EnergyNumericalBounds::default()) {
            Ok(job) => {
                self.job = Some(job);
                self.projection.status = EnergySimulationStatus::Queued;
                Ok(())
            }
            Err(rejected) => {
                self.rejected = Some(rejected);
                self.projection.status = EnergySimulationStatus::Faulted;
                Err(MountedAdmissionError::Rejected("energy.session.numerical-admission-rejected"))
            }
        }
    }

    fn retry_rejected(&mut self) -> bool {
        if let Some(rejected) = self.rejected.take() {
            return match rejected.retry(EnergyNumericalBounds::default()) {
                Ok(job) => {
                    self.job = Some(job);
                    self.admission_blocked = false;
                    self.spawned = false;
                    self.projection.status = EnergySimulationStatus::Queued;
                    true
                }
                Err(rejected) => {
                    self.rejected = Some(rejected);
                    false
                }
            };
        }
        if let Some(rejected) = self.restore_rejected.take() {
            return match rejected.retry(EnergyNumericalBounds::default()) {
                Ok(restore) => {
                    self.restore = Some(restore);
                    self.admission_blocked = false;
                    self.spawned = false;
                    self.projection.status = EnergySimulationStatus::Queued;
                    true
                }
                Err(rejected) => {
                    self.restore_rejected = Some(rejected);
                    false
                }
            };
        }
        false
    }

    fn packet_is_fresh(&self, identity: crate::EnergyWireIdentity) -> bool {
        self.snapshot_is_fresh() && identity.operation == self.identity.operation.0 && identity.base_revision == self.identity.document_revision.0 && identity.generation == self.identity.generation.0 && identity.seed == self.identity.operation().seed
    }

    fn install_preview(&mut self, preview: EnergyJobPreview) -> bool {
        let tier_index = quality_tier_index(preview.tier);
        let prior_tier = self.projection.latest_tier.map_or(0, quality_tier_index);
        if preview.sequence <= self.projection.latest_sequence || tier_index < prior_tier {
            return false;
        }
        let projection = EnergyTierProjection {
            app_instance_id: self.identity.app_instance_id,
            document_revision: self.identity.document_revision,
            document_generation: self.identity.document_generation,
            canonical_base_revision: self.identity.canonical_base_revision,
            operation: self.identity.operation,
            generation: self.identity.generation,
            config_digest: self.identity.config_digest,
            sequence: preview.sequence,
            tier: preview.tier,
            stage: preview.stage,
            warmup_hour: preview.warmup_hour,
            timestep: preview.timestep,
            total_timesteps: preview.total_timesteps,
            facility_electricity_kwh: preview.facility_electricity_kwh,
        };
        self.projection.tiers[tier_index] = Some(projection);
        self.projection.latest_sequence = preview.sequence;
        self.projection.latest_tier = Some(preview.tier);
        self.projection.status = EnergySimulationStatus::Running;
        true
    }

    fn collect_channels_one(&mut self) -> Result<(), &'static str> {
        if self.preview_packet.is_none() {
            let packet = self.job.as_mut().ok_or("energy.session.job-missing")?.take_preview_packet(self.identity.generation).map_err(|_| "energy.session.preview-identity")?;
            if let Some(packet) = packet {
                let fresh = self.packet_is_fresh(packet.identity());
                if fresh {
                    if let Some(preview) = packet.preview().map(|preview| EnergyJobPreview {
                        sequence: preview.sequence,
                        tier: preview.tier,
                        stage: preview.stage,
                        warmup_hour: preview.warmup_hour,
                        timestep: preview.timestep,
                        total_timesteps: preview.total_timesteps,
                        facility_electricity_kwh: preview.facility_electricity_kwh,
                    }) {
                        let _ = self.install_preview(preview);
                    }
                }
                self.preview_packet = Some(packet);
                return Ok(());
            }
        }
        if self.checkpoint.is_none() {
            self.checkpoint = self.job.as_mut().ok_or("energy.session.job-missing")?.take_checkpoint_packet(self.identity.generation).map_err(|_| "energy.session.checkpoint-identity")?;
            if let Some(lease) = self.checkpoint.as_ref() {
                if !self.packet_is_fresh(lease.identity()) {
                    return Err("energy.session.checkpoint-stale");
                }
                self.projection.checkpoint_ready = true;
                return Ok(());
            }
        }
        if self.commit.is_none() {
            self.commit = self.job.as_mut().ok_or("energy.session.job-missing")?.take_commit_packet(self.identity.generation).map_err(|_| "energy.session.commit-identity")?;
            if let Some(lease) = self.commit.as_ref() {
                if !self.packet_is_fresh(lease.identity()) {
                    return Err("energy.session.commit-stale");
                }
                self.projection.final_ready = true;
                self.projection.status = EnergySimulationStatus::FinalReady;
                return Ok(());
            }
        }
        if self.fault.is_none() {
            self.fault = self.job.as_mut().ok_or("energy.session.job-missing")?.take_fault_packet(self.identity.generation).map_err(|_| "energy.session.fault-identity")?;
            if let Some(lease) = self.fault.as_ref() {
                if !self.packet_is_fresh(lease.identity()) {
                    return Err("energy.session.fault-stale");
                }
                self.projection.fault_ready = true;
                self.projection.status = EnergySimulationStatus::Faulted;
            }
        }
        Ok(())
    }

    fn close_outcome_one(&mut self, maximum_bytes: usize) -> bool {
        let Some(outcome) = self.outcome.as_mut() else { return true };
        if matches!(outcome.close_step(1, maximum_bytes), semio_framework_job::JobPayloadCloseStep::Complete) {
            self.outcome = None;
        }
        false
    }

    fn worker_step(&mut self, budget: JobBudget) -> JobStep {
        if self.closing || self.cancel.is_cancelled_now() {
            self.projection.status = EnergySimulationStatus::Cancelled;
            return JobStep::Done(Vec::new());
        }
        if budget.fuel == 0 || budget.deadline_ms == 0 || self.outcome.is_some() {
            return JobStep::Running(None);
        }
        let Some(now) = semio_framework_job::default_now_us() else { return JobStep::Running(None) };
        let Some(deadline) = now.checked_add(u64::from(budget.deadline_ms).min(7) * 1_000) else { return JobStep::Running(None) };
        let mut context = StepContext::new(self.identity.operation, self.identity.generation, StepBudget::new(budget.fuel.min(1), deadline), self.cancel.clone(), semio_framework_job::default_now_us, &mut self.preview_sequence);
        if context.should_yield() {
            return JobStep::Running(None);
        }
        if let Some(restore) = self.restore.as_mut() {
            match restore.step(&mut context) {
                Ok(false) => return JobStep::Running(None),
                Ok(true) => {
                    let restore = self.restore.take().expect("ready Energy restore remains mounted");
                    match restore.finish(&context) {
                        Ok(job) => {
                            self.job = Some(job);
                            self.projection.status = EnergySimulationStatus::Running;
                            return JobStep::Running(None);
                        }
                        Err(restore) => {
                            self.restore = Some(restore);
                            self.projection.status = EnergySimulationStatus::Faulted;
                            return JobStep::Failed(b"energy.session.restore-finish".to_vec());
                        }
                    }
                }
                Err(_) => {
                    self.projection.status = EnergySimulationStatus::Faulted;
                    return JobStep::Failed(b"energy.session.restore-replay".to_vec());
                }
            }
        }
        if let Err(detail) = self.collect_channels_one() {
            self.projection.status = EnergySimulationStatus::Faulted;
            return JobStep::Failed(detail.as_bytes().to_vec());
        }
        if self.commit.is_some() || self.fault.is_some() {
            return JobStep::Running(None);
        }
        let Some(job) = self.job.as_mut() else { return JobStep::Failed(b"energy.session.job-missing".to_vec()) };
        let outcome = job.step(&mut context);
        match outcome {
            StepOutcome::Yield => JobStep::Running(None),
            StepOutcome::Cancelled => {
                self.projection.status = EnergySimulationStatus::Cancelled;
                JobStep::Done(Vec::new())
            }
            other => retain_worker_outcome(&mut self.outcome, other),
        }
    }

    fn maintenance_one(&mut self, maximum_bytes: usize) -> PluginCloseStep {
        if !self.close_outcome_one(maximum_bytes) {
            return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(packet) = self.preview_packet.as_mut() {
            let step = packet.ack_step(1, maximum_bytes);
            if matches!(step, semio_framework_job::JobPayloadCloseStep::Complete) {
                self.preview_packet = None;
            }
            return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(packet) = self.retiring_checkpoint_packet.as_mut() {
            let _ = packet.ack_step(1, maximum_bytes);
            if packet.terminal_is_empty() {
                self.retiring_checkpoint_packet = None;
            }
            return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if let Some(lease) = self.checkpoint.take() {
            if !self.packet_is_fresh(lease.identity()) {
                self.checkpoint = Some(lease);
                return PluginCloseStep::Blocked { reason: "stale Energy checkpoint retained for retirement" };
            }
            if self.checkpoint_packet.is_some() && self.retiring_checkpoint_packet.is_some() {
                self.checkpoint = Some(lease);
                return PluginCloseStep::Blocked { reason: "Energy checkpoint replacement retirement saturated" };
            }
            let Some(job) = self.job.as_mut() else {
                self.checkpoint = Some(lease);
                return PluginCloseStep::Blocked { reason: "checkpoint job owner missing" };
            };
            let identity = lease.identity();
            let packet = match job.ack_checkpoint_for_restore(lease) {
                Ok(packet) => packet,
                Err(lease) => {
                    self.checkpoint = Some(lease);
                    return PluginCloseStep::Blocked { reason: "checkpoint transfer ACK rejected" };
                }
            };
            self.retiring_checkpoint_packet = self.checkpoint_packet.take().map(|(_, packet)| packet);
            let token = identity.sequence.max(1);
            self.checkpoint_packet = Some((token, packet));
            self.projection.checkpoint_token = token;
            self.projection.checkpoint_ready = true;
            return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        if self.adopt_requested {
            if let Some(mut lease) = self.commit.take() {
                if !self.packet_is_fresh(lease.identity()) {
                    self.commit = Some(lease);
                    return PluginCloseStep::Blocked { reason: "stale Energy commit retained for retirement" };
                }
                if !lease.packet().terminal_is_empty() {
                    let _ = lease.packet_mut().ack_step(1, maximum_bytes);
                    self.commit = Some(lease);
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                let Some(job) = self.job.as_mut() else {
                    self.commit = Some(lease);
                    return PluginCloseStep::Blocked { reason: "commit job owner missing" };
                };
                if let Err(lease) = job.ack_commit_packet(lease) {
                    self.commit = Some(lease);
                    return PluginCloseStep::Blocked { reason: "commit ACK rejected" };
                }
                self.projection.final_ready = false;
                self.projection.adopted = true;
                self.projection.status = EnergySimulationStatus::Adopted;
                self.adopt_requested = false;
                return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
        }
        PluginCloseStep::Complete
    }

    fn begin_close(&mut self) {
        self.closing = true;
        self.projection.status = EnergySimulationStatus::Closing;
        self.cancel.cancel_now();
        if let Some(job) = self.job.as_mut() {
            job.begin_close();
        }
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep {
        if maximum_items == 0 {
            return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        self.begin_close();
        match self.close_lane {
            0 => {
                if !self.close_outcome_one(maximum_bytes) {
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                } else {
                    self.close_lane += 1;
                    PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
            }
            1 => {
                if let Some(packet) = self.preview_packet.as_mut() {
                    let _ = packet.ack_step(1, maximum_bytes);
                    if packet.terminal_is_empty() {
                        self.preview_packet = None;
                    }
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                } else {
                    self.close_lane += 1;
                    PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
            }
            2 => {
                if let Some((_, packet)) = self.checkpoint_packet.as_mut() {
                    let _ = packet.ack_step(1, maximum_bytes);
                    if packet.terminal_is_empty() {
                        self.checkpoint_packet = None;
                    }
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                self.close_lane += 1;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            3 => {
                if let Some(packet) = self.retiring_checkpoint_packet.as_mut() {
                    let _ = packet.ack_step(1, maximum_bytes);
                    if packet.terminal_is_empty() {
                        self.retiring_checkpoint_packet = None;
                    }
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                self.close_lane += 1;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            4..=6 => {
                let lease = match self.close_lane {
                    4 => &mut self.checkpoint,
                    5 => &mut self.commit,
                    _ => &mut self.fault,
                };
                if let Some(owner) = lease.as_mut() {
                    if !owner.packet().terminal_is_empty() {
                        let _ = owner.packet_mut().ack_step(1, maximum_bytes);
                        return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                    }
                }
                if let Some(owner) = lease.take() {
                    let Some(job) = self.job.as_mut() else {
                        *lease = Some(owner);
                        return PluginCloseStep::Blocked { reason: "Energy lease job owner missing" };
                    };
                    let result = match self.close_lane {
                        4 => job.ack_checkpoint_packet(owner),
                        5 => job.ack_commit_packet(owner),
                        _ => job.ack_fault_packet(owner),
                    };
                    if let Err(owner) = result {
                        *lease = Some(owner);
                        return PluginCloseStep::Blocked { reason: "Energy lease close ACK rejected" };
                    }
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                self.close_lane += 1;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            7 => {
                if let Some(job) = self.job.as_mut() {
                    match job.close_step(1, maximum_bytes) {
                        semio_framework_job::InteractiveJobCloseStep::Complete => {
                            self.job = None;
                            return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                        }
                        semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => return PluginCloseStep::Pending { released_items, released_bytes },
                        semio_framework_job::InteractiveJobCloseStep::Blocked => return PluginCloseStep::Blocked { reason: "Energy numerical owner close blocked" },
                    }
                } else {
                    self.close_lane += 1;
                    PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
                }
            }
            8 => {
                if let Some(restore) = self.restore.as_mut() {
                    match restore.close_step(1, maximum_bytes) {
                        semio_framework_job::InteractiveJobCloseStep::Complete => {
                            if !restore.terminal_is_empty() {
                                return PluginCloseStep::Blocked { reason: "Energy restore false terminal" };
                            }
                            self.restore = None;
                            return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                        }
                        semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => return PluginCloseStep::Pending { released_items, released_bytes },
                        semio_framework_job::InteractiveJobCloseStep::Blocked => return PluginCloseStep::Blocked { reason: "Energy restore close blocked" },
                    }
                }
                self.close_lane += 1;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            9 => {
                if let Some(rejected) = self.rejected.as_mut() {
                    match rejected.close_step(1, maximum_bytes) {
                        semio_framework_job::InteractiveJobCloseStep::Complete => {
                            if !rejected.terminal_is_empty() {
                                return PluginCloseStep::Blocked { reason: "Energy rejected owner false terminal" };
                            }
                            self.rejected = None;
                            return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                        }
                        semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => return PluginCloseStep::Pending { released_items, released_bytes },
                        semio_framework_job::InteractiveJobCloseStep::Blocked => return PluginCloseStep::Blocked { reason: "Energy rejected owner close blocked" },
                    }
                }
                self.close_lane += 1;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            10 => {
                if let Some(rejected) = self.restore_rejected.as_mut() {
                    match rejected.close_step(1, maximum_bytes) {
                        semio_framework_job::InteractiveJobCloseStep::Complete => {
                            if !rejected.terminal_is_empty() {
                                return PluginCloseStep::Blocked { reason: "Energy restore rejection false terminal" };
                            }
                            self.restore_rejected = None;
                            return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                        }
                        semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => return PluginCloseStep::Pending { released_items, released_bytes },
                        semio_framework_job::InteractiveJobCloseStep::Blocked => return PluginCloseStep::Blocked { reason: "Energy restore rejection close blocked" },
                    }
                }
                self.close_lane += 1;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            11 => {
                if let Some(capture) = std::mem::replace(&mut self.capture, None) {
                    self.capture_close = Some(EnergyModelCloseCursor::new(capture.finish()));
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                self.close_lane += 1;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            12 => {
                if let Some(cursor) = self.capture_close.as_mut() {
                    match cursor.close_step(maximum_bytes) {
                        semio_framework_job::InteractiveJobCloseStep::Complete => {
                            self.capture_close = None;
                            return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                        }
                        semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => return PluginCloseStep::Pending { released_items, released_bytes },
                        semio_framework_job::InteractiveJobCloseStep::Blocked => return PluginCloseStep::Blocked { reason: "Energy partial capture close blocked" },
                    }
                }
                self.close_lane += 1;
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            }
            13 => {
                if !self.snapshot.close_step() {
                    return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
                }
                self.close_lane += 1;
                return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            14 => {
                self.projection.tiers = [None; 4];
                self.close_lane += 1;
                return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            _ if self.worker_attached && !self.worker_returned => return PluginCloseStep::Blocked { reason: "Energy process owner has not published its fixed recovery witness" },
            _ => return PluginCloseStep::Complete,
        }
    }

    fn terminal_is_empty(&self) -> bool {
        !self.worker_attached
            && self.job.is_none()
            && self.restore.is_none()
            && self.rejected.is_none()
            && self.restore_rejected.is_none()
            && self.capture.is_none()
            && self.capture_close.as_ref().is_none_or(EnergyModelCloseCursor::terminal_is_empty)
            && self.snapshot.terminal_is_empty()
            && self.outcome.is_none()
            && self.preview_packet.is_none()
            && self.checkpoint.is_none()
            && self.checkpoint_packet.is_none()
            && self.retiring_checkpoint_packet.is_none()
            && self.commit.is_none()
            && self.fault.is_none()
            && self.projection.tiers.iter().all(Option::is_none)
    }
}

#[derive(Clone, Copy)]
struct CurrentSession {
    app_instance_id: u32,
    shell: u16,
    identity: MountedIdentity,
}

#[derive(Clone, Copy)]
struct AdoptedProjectionAuthority {
    render: AppRenderOperationContext,
    identity: MountedIdentity,
    projection: EnergySimulationProjection,
}

impl AdoptedProjectionAuthority {
    fn new(identity: MountedIdentity, projection: EnergySimulationProjection) -> Option<Self> {
        let render = AppRenderOperationContext { app_instance_id: identity.app_instance_id, base_revision: identity.document_revision, generation: identity.document_generation, canonical_base_revision: identity.canonical_base_revision };
        let authority = Self { render, identity, projection };
        authority.matches_render(render).then_some(authority)
    }

    fn matches_render(&self, render: AppRenderOperationContext) -> bool {
        self.render == render
            && self.identity.matches_render(render)
            && self.projection.adopted
            && self.projection.request == self.identity.request
            && self.projection.operation == self.identity.operation
            && self.projection.generation == self.identity.generation
            && self.projection.config_digest == self.identity.config_digest
            && self.projection.tiers.iter().flatten().all(|tier| {
                tier.app_instance_id == self.identity.app_instance_id
                    && tier.document_revision == self.identity.document_revision
                    && tier.document_generation == self.identity.document_generation
                    && tier.canonical_base_revision == self.identity.canonical_base_revision
                    && tier.operation == self.identity.operation
                    && tier.generation == self.identity.generation
                    && tier.config_digest == self.identity.config_digest
            })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RecoveryRecord {
    shell: u16,
    identity: MountedIdentity,
}

#[derive(Clone, Copy)]
struct PendingPreflight {
    render: AppRenderOperationContext,
    request: u64,
    config: EnergySimulationConfigProjection,
    census: CaptureCensus,
}

struct Registry {
    shells: [Rc<RefCell<Option<MountedState>>>; SHELL_SLOTS],
    apps: [Option<u32>; ACTIVE_SLOTS],
    last_request: [u64; ACTIVE_SLOTS],
    current: [Option<CurrentSession>; ACTIVE_SLOTS],
    adopted: [Option<AdoptedProjectionAuthority>; ACTIVE_SLOTS],
    pending: [Option<u16>; ACTIVE_SLOTS],
    preflight: [Option<PendingPreflight>; ACTIVE_SLOTS],
    retiring: [Option<u16>; SHELL_SLOTS],
    retirement_reserved: [bool; SHELL_SLOTS],
    shell_retirement: [Option<u8>; SHELL_SLOTS],
    recovery_reserved: [bool; SHELL_SLOTS],
    events: [Option<SessionEvent>; EVENT_SLOTS],
    event_head: usize,
    event_len: usize,
    free: [u16; SHELL_SLOTS],
    free_head: usize,
    free_len: usize,
    next_event: u64,
    next_generation: u64,
    next_job: u64,
}

impl Registry {
    fn new() -> Self {
        Self {
            shells: std::array::from_fn(|_| Rc::new(RefCell::new(None))),
            apps: [None; ACTIVE_SLOTS],
            last_request: [0; ACTIVE_SLOTS],
            current: [None; ACTIVE_SLOTS],
            adopted: [None; ACTIVE_SLOTS],
            pending: [None; ACTIVE_SLOTS],
            preflight: [None; ACTIVE_SLOTS],
            retiring: [None; SHELL_SLOTS],
            retirement_reserved: [false; SHELL_SLOTS],
            shell_retirement: [None; SHELL_SLOTS],
            recovery_reserved: [false; SHELL_SLOTS],
            events: [None; EVENT_SLOTS],
            event_head: 0,
            event_len: 0,
            free: std::array::from_fn(|index| index as u16),
            free_head: 0,
            free_len: SHELL_SLOTS,
            next_event: 0,
            next_generation: 0,
            next_job: 0,
        }
    }

    fn push_event(&mut self, render: AppRenderOperationContext, kind: EnergySimulationEventKind) -> Result<(), &'static str> {
        if self.event_len == EVENT_SLOTS || render.app_instance_id == 0 {
            return Err("energy.session.event-log-saturated");
        }
        let slot = self.apps.iter().position(|app| *app == Some(render.app_instance_id)).or_else(|| self.apps.iter().position(Option::is_none)).ok_or("energy.session.active-slots-saturated")?;
        self.next_event = self.next_event.checked_add(1).ok_or("energy.session.event-sequence-exhausted")?;
        self.apps[slot] = Some(render.app_instance_id);
        let write = (self.event_head + self.event_len) % EVENT_SLOTS;
        self.events[write] = Some(SessionEvent { sequence: self.next_event, render, kind });
        self.event_len += 1;
        Ok(())
    }

    fn pop_event(&mut self) -> Option<SessionEvent> {
        let event = self.events[self.event_head].take()?;
        self.event_head = (self.event_head + 1) % EVENT_SLOTS;
        self.event_len -= 1;
        Some(event)
    }

    fn allocate(&mut self) -> Option<u16> {
        if self.free_len == 0 {
            return None;
        }
        let shell = self.free[self.free_head];
        self.free_head = (self.free_head + 1) % SHELL_SLOTS;
        self.free_len -= 1;
        Some(shell)
    }

    fn release(&mut self, shell: u16) {
        let write = (self.free_head + self.free_len) % SHELL_SLOTS;
        self.free[write] = shell;
        self.free_len += 1;
    }

    fn reserve_retirement(&mut self) -> Option<u8> {
        let index = (0..SHELL_SLOTS).find(|index| self.retiring[*index].is_none() && !self.retirement_reserved[*index])?;
        self.retirement_reserved[index] = true;
        Some(index as u8)
    }

    fn install_retirement(&mut self, reservation: u8, shell: u16) -> bool {
        let index = reservation as usize;
        if index >= SHELL_SLOTS || !self.retirement_reserved[index] || self.retiring[index].is_some() {
            return false;
        }
        self.retirement_reserved[index] = false;
        self.retiring[index] = Some(shell);
        true
    }

    fn release_retirement(&mut self, reservation: u8) {
        let index = reservation as usize;
        if index < SHELL_SLOTS {
            self.retirement_reserved[index] = false;
        }
    }

    fn cancel_preflight(&mut self, slot: usize) -> bool {
        self.preflight[slot].take().is_some()
    }

    fn reserve_shell_retirement(&mut self, shell: u16) -> bool {
        let Some(reservation) = self.reserve_retirement() else { return false };
        self.shell_retirement[shell as usize] = Some(reservation);
        true
    }

    fn reserve_shell_recovery(&mut self, shell: u16) -> bool {
        let slot = &mut self.recovery_reserved[shell as usize];
        if *slot {
            return false;
        }
        *slot = true;
        true
    }

    fn release_shell_recovery(&mut self, shell: u16) {
        self.recovery_reserved[shell as usize] = false;
    }

    fn release_shell_retirement(&mut self, shell: u16) {
        if let Some(reservation) = self.shell_retirement[shell as usize].take() {
            self.release_retirement(reservation);
        }
    }

    fn retire_shell(&mut self, shell: u16) -> bool {
        let Some(reservation) = self.shell_retirement[shell as usize].take() else { return false };
        self.install_retirement(reservation, shell)
    }

    fn slot_for(&self, app_instance_id: u32) -> Option<usize> {
        self.apps.iter().position(|app| *app == Some(app_instance_id))
    }

    fn adopted_projection(&self, render: AppRenderOperationContext) -> Option<&EnergySimulationProjection> {
        let slot = self.slot_for(render.app_instance_id)?;
        let authority = self.adopted[slot].as_ref()?;
        (self.last_request[slot] == authority.identity.request && authority.matches_render(render)).then_some(&authority.projection)
    }

    fn app_terminal_is_empty(&self, slot: usize, app_instance_id: u32) -> bool {
        self.current[slot].is_none()
            && self.pending[slot].is_none()
            && self.preflight[slot].is_none()
            && !self.retiring.iter().any(|entry| entry.is_some_and(|shell| self.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(|state| state.identity.app_instance_id == app_instance_id))))
            && self.events.iter().flatten().all(|event| event.render.app_instance_id != app_instance_id)
    }

    fn checkpoint_identity(&self, slot: usize, render: AppRenderOperationContext, config: EnergySimulationConfigProjection) -> Result<Option<crate::EnergyWireIdentity>, &'static str> {
        if config.checkpoint_token == 0 {
            return Ok(None);
        }
        let current = self.current[slot].filter(|current| current.app_instance_id == render.app_instance_id).ok_or("Energy checkpoint token is unavailable")?;
        let owner = self.shells[current.shell as usize].try_borrow().map_err(|_| "Energy checkpoint owner is busy")?;
        let state = owner.as_ref().filter(|state| state.identity.matches_render(render) && state.identity.config_digest == config.digest()).ok_or("Energy checkpoint provenance is stale")?;
        let (_, packet) = state.checkpoint_packet.as_ref().filter(|(token, _)| *token == config.checkpoint_token).ok_or("Energy checkpoint token is unavailable")?;
        let identity = packet.identity();
        if identity.operation != state.identity.operation.0 || identity.base_revision != render.base_revision.0 || identity.generation != state.identity.generation.0 || identity.seed != state.identity.operation_seed {
            return Err("Energy checkpoint provenance is stale");
        }
        Ok(Some(identity))
    }
}

thread_local! {
    static REGISTRY: RefCell<Registry> = RefCell::new(Registry::new());
    static RECOVERY: RefCell<[Option<RecoveryRecord>; SHELL_SLOTS]> = RefCell::new([None; SHELL_SLOTS]);
}
//#endregion 🧰️FixedArena

//#region 💼️ProcessBridge
struct EnergyMountedBoundedJob {
    shell_index: u16,
    shell: Rc<RefCell<Option<MountedState>>>,
    identity: MountedIdentity,
}

impl BoundedJob for EnergyMountedBoundedJob {
    fn step(&mut self, budget: JobBudget) -> JobStep {
        let Ok(mut owner) = self.shell.try_borrow_mut() else { return JobStep::Running(None) };
        let Some(state) = owner.as_mut() else { return JobStep::Failed(b"energy.session.owner-missing".to_vec()) };
        if state.identity != self.identity {
            return JobStep::Failed(b"energy.session.aba".to_vec());
        }
        state.worker_step(budget)
    }

    fn cancel(&mut self) {
        if let Ok(owner) = self.shell.try_borrow() {
            if let Some(state) = owner.as_ref() {
                state.cancel.cancel_now();
            }
        }
    }

    fn checkpoint(&self) -> Option<Vec<u8>> {
        None
    }

    fn terminal_drop_is_shallow(&self) -> bool {
        true
    }
}

impl Drop for EnergyMountedBoundedJob {
    fn drop(&mut self) {
        RECOVERY.with(|recovery| {
            let mut recovery = recovery.borrow_mut();
            let slot = &mut recovery[self.shell_index as usize];
            assert!(slot.is_none() || slot.is_some_and(|record| record.identity == self.identity), "fixed Energy recovery slot cannot replace a different generation owner");
            *slot = Some(RecoveryRecord { shell: self.shell_index, identity: self.identity });
        });
    }
}

fn encode_input(shell: u16, identity: MountedIdentity) -> Vec<u8> {
    let mut input = Vec::with_capacity(INPUT_BYTES);
    input.push(1);
    input.extend_from_slice(&shell.to_le_bytes());
    input.extend_from_slice(&identity.app_instance_id.to_le_bytes());
    input.extend_from_slice(&identity.request.to_le_bytes());
    input.extend_from_slice(&identity.document_revision.0.to_le_bytes());
    input.extend_from_slice(&identity.document_generation.0.to_le_bytes());
    input.extend_from_slice(&identity.generation.0.to_le_bytes());
    input.extend_from_slice(&identity.canonical_base_revision);
    input.extend_from_slice(&identity.config_digest.to_le_bytes());
    input.extend_from_slice(&identity.operation_seed.to_le_bytes());
    input.extend_from_slice(&identity.operation.0.to_le_bytes());
    input
}

fn decode_input(job: u64, input: &[u8]) -> Option<(u16, MountedIdentity)> {
    if input.len() != INPUT_BYTES || input[0] != 1 || job & !JOB_COUNTER_MAXIMUM != JOB_TAG {
        return None;
    }
    let shell = u16::from_le_bytes(input[1..3].try_into().ok()?);
    let app_instance_id = u32::from_le_bytes(input[3..7].try_into().ok()?);
    let request = u64::from_le_bytes(input[7..15].try_into().ok()?);
    let document_revision = RevisionId(u64::from_le_bytes(input[15..23].try_into().ok()?));
    let document_generation = Generation(u64::from_le_bytes(input[23..31].try_into().ok()?));
    let generation = Generation(u64::from_le_bytes(input[31..39].try_into().ok()?));
    let canonical_base_revision = input[39..71].try_into().ok()?;
    let config_digest = u64::from_le_bytes(input[71..79].try_into().ok()?);
    let operation_seed = u64::from_le_bytes(input[79..87].try_into().ok()?);
    let operation = OperationId(u64::from_le_bytes(input[87..95].try_into().ok()?));
    if generation.0 == 0 {
        return None;
    }
    Some((shell, MountedIdentity { app_instance_id, request, document_revision, document_generation, canonical_base_revision, operation, generation, config_digest, operation_seed, job }))
}

fn factory(job: u64, input: &[u8]) -> Result<Box<dyn BoundedJob>, Vec<u8>> {
    let (shell, identity) = decode_input(job, input).ok_or_else(|| b"energy.session.factory-input".to_vec())?;
    REGISTRY.with(|registry| {
        let registry = registry.borrow();
        let owner = registry.shells.get(shell as usize).ok_or_else(|| b"energy.session.factory-shell".to_vec())?.clone();
        if !registry.recovery_reserved[shell as usize] {
            return Err(b"energy.session.factory-recovery-unreserved".to_vec());
        }
        let Ok(mut state_owner) = owner.try_borrow_mut() else { return Err(b"energy.session.factory-busy".to_vec()) };
        let Some(state) = state_owner.as_mut().filter(|state| state.identity == identity && (state.job.is_some() || state.restore.is_some()) && !state.worker_attached) else {
            return Err(b"energy.session.factory-stale".to_vec());
        };
        state.worker_attached = true;
        drop(state_owner);
        Ok(Box::new(EnergyMountedBoundedJob { shell_index: shell, shell: owner, identity }) as Box<dyn BoundedJob>)
    })
}

pub fn initialize() {
    REGISTRY.with(|registry| {
        let _ = registry.borrow().free_len;
    });
    semio_framework_plugin::reactor::jobs::register_bounded_job_kind(ENERGY_SIMULATION_JOB_KIND, factory as BoundedJobFactory);
}
//#endregion 💼️ProcessBridge

//#region 🎛️ProductSession
pub fn record_event(render: AppRenderOperationContext, kind: EnergySimulationEventKind) -> Result<(), &'static str> {
    if matches!(kind, EnergySimulationEventKind::Start { request: 0, .. }) || matches!(kind, EnergySimulationEventKind::Start { config, .. } if !config.validate()) {
        return Err("energy.session.config-invalid");
    }
    if match kind {
        EnergySimulationEventKind::Cancel(identity) | EnergySimulationEventKind::Retry(identity) | EnergySimulationEventKind::Discard(identity) | EnergySimulationEventKind::Adopt(identity) => !identity.valid(),
        EnergySimulationEventKind::Start { .. } => false,
    } {
        return Err("energy.session.request-identity-invalid");
    }
    REGISTRY.with(|registry| registry.borrow_mut().push_event(render, kind))
}

fn apply_event_one(registry: &mut Registry) {
    let Some(event) = registry.pop_event() else { return };
    let _ = event.sequence;
    let Some(slot) = registry.slot_for(event.render.app_instance_id) else { return };
    match event.kind {
        EnergySimulationEventKind::Start { request, config } => {
            if request == 0 || request <= registry.last_request[slot] {
                return;
            }
            registry.last_request[slot] = request;
            if let Some(shell) = registry.pending[slot] {
                if !registry.retire_shell(shell) {
                    return;
                }
                registry.pending[slot] = None;
            }
            registry.preflight[slot] = Some(PendingPreflight { render: event.render, request, config, census: CaptureCensus::new() });
        }
        EnergySimulationEventKind::Retry(request) => {
            let Some(current) = registry.current[slot].filter(|current| current.app_instance_id == event.render.app_instance_id && current.identity.matches_render(event.render) && current.identity.matches_request(request)) else { return };
            if let Ok(mut owner) = registry.shells[current.shell as usize].try_borrow_mut() {
                if owner.as_mut().is_some_and(MountedState::retry_rejected) {
                    return;
                }
            }
            if let Some(shell) = registry.pending[slot] {
                if !registry.retire_shell(shell) {
                    return;
                }
                registry.pending[slot] = None;
            }
            let Ok(owner) = registry.shells[current.shell as usize].try_borrow() else { return };
            let Some(config) = owner.as_ref().map(|state| state.config) else { return };
            drop(owner);
            registry.preflight[slot] = Some(PendingPreflight { render: event.render, request: request.request, config, census: CaptureCensus::new() });
        }
        EnergySimulationEventKind::Cancel(request) => {
            if registry.preflight[slot].is_some_and(|preflight| preflight.render == event.render && preflight.request == request.request) {
                registry.cancel_preflight(slot);
            }
            if let Some(shell) = registry.pending[slot] {
                if let Ok(owner) = registry.shells[shell as usize].try_borrow() {
                    if let Some(state) = owner.as_ref().filter(|state| state.identity.matches_render(event.render) && state.identity.matches_request(request)) {
                        state.cancel.cancel_now();
                    }
                }
            }
            if let Some(current) = registry.current[slot].filter(|current| current.app_instance_id == event.render.app_instance_id && current.identity.matches_render(event.render) && current.identity.matches_request(request)) {
                if let Ok(owner) = registry.shells[current.shell as usize].try_borrow() {
                    if let Some(state) = owner.as_ref() {
                        state.cancel.cancel_now();
                    }
                }
            }
        }
        EnergySimulationEventKind::Discard(request) => {
            if registry.preflight[slot].is_some_and(|preflight| preflight.render == event.render && preflight.request == request.request) {
                registry.cancel_preflight(slot);
            }
            if let Some(shell) = registry.pending[slot] {
                let exact = registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(|state| state.identity.matches_render(event.render) && state.identity.matches_request(request)));
                if exact && registry.retire_shell(shell) {
                    registry.pending[slot] = None;
                }
            }
            if let Some(current) = registry.current[slot].filter(|current| current.app_instance_id == event.render.app_instance_id && current.identity.matches_render(event.render) && current.identity.matches_request(request)) {
                if let Ok(owner) = registry.shells[current.shell as usize].try_borrow() {
                    if let Some(state) = owner.as_ref() {
                        state.cancel.cancel_now();
                    }
                }
                if registry.retire_shell(current.shell) {
                    registry.current[slot] = None;
                }
            }
        }
        EnergySimulationEventKind::Adopt(request) => {
            if let Some(current) = registry.current[slot].filter(|current| current.app_instance_id == event.render.app_instance_id) {
                if current.identity.matches_render(event.render) && current.identity.matches_request(request) {
                    if let Ok(mut owner) = registry.shells[current.shell as usize].try_borrow_mut() {
                        if let Some(state) = owner.as_mut() {
                            state.adopt_requested = true;
                        }
                    }
                }
            }
        }
    }
}

pub fn prepare_snapshot_read(render: AppRenderOperationContext, snapshot: &EnergyModelSnapshot) -> bool {
    REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        apply_event_one(&mut registry);
        let Some(slot) = registry.slot_for(render.app_instance_id) else { return false };
        let Some(mut preflight) = registry.preflight[slot] else { return false };
        if preflight.render != render {
            registry.cancel_preflight(slot);
            return false;
        }
        let step = preflight.census.step_one(&snapshot.model);
        match step {
            Ok(true) => {
                registry.preflight[slot] = Some(preflight);
                true
            }
            Ok(false) => {
                registry.preflight[slot] = Some(preflight);
                false
            }
            Err(_) => {
                registry.cancel_preflight(slot);
                false
            }
        }
    })
}

pub fn reconcile(doc: &ArtifactView<'_, EnergyModelSnapshot>) -> Vec<Effect> {
    let Some(render) = doc.render_operation() else { return Vec::new() };
    REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        apply_event_one(&mut registry);
        let Some(slot) = registry.slot_for(render.app_instance_id) else { return Vec::new() };
        if let Some(preflight) = registry.preflight[slot] {
            if preflight.render != render || preflight.census.lane <= 39 {
                return Vec::new();
            }
            let checkpoint_identity = match registry.checkpoint_identity(slot, render, preflight.config) {
                Ok(identity) => identity,
                Err(message) => return vec![Effect::Notify { message: message.into() }],
            };
            let Some(shell) = registry.allocate() else { return vec![Effect::Notify { message: "Energy simulation slots are full".into() }] };
            if !registry.reserve_shell_retirement(shell) {
                registry.release(shell);
                return vec![Effect::Notify { message: "Energy retirement capacity is full".into() }];
            }
            if !registry.reserve_shell_recovery(shell) {
                registry.release_shell_retirement(shell);
                registry.release(shell);
                return vec![Effect::Notify { message: "Energy recovery capacity is full".into() }];
            }
            let Some(generation_value) = registry.next_generation.checked_add(1) else {
                registry.release_shell_retirement(shell);
                registry.release_shell_recovery(shell);
                registry.release(shell);
                return Vec::new();
            };
            let Some(counter) = registry.next_job.checked_add(1).filter(|counter| *counter <= JOB_COUNTER_MAXIMUM) else {
                registry.release_shell_retirement(shell);
                registry.release_shell_recovery(shell);
                registry.release(shell);
                return Vec::new();
            };
            registry.next_generation = generation_value;
            registry.next_job = counter;
            let job = JOB_TAG | counter;
            let operation = checkpoint_identity.map_or(OperationId(job), |identity| OperationId(identity.operation));
            let generation = checkpoint_identity.map_or(Generation(generation_value), |identity| Generation(identity.generation));
            let operation_seed = checkpoint_identity.map_or(OperationId(job).0.rotate_left(19) ^ preflight.config.digest(), |identity| identity.seed);
            let identity = MountedIdentity {
                app_instance_id: render.app_instance_id,
                request: preflight.request,
                document_revision: render.base_revision,
                document_generation: render.generation,
                canonical_base_revision: render.canonical_base_revision,
                operation,
                generation,
                config_digest: preflight.config.digest(),
                operation_seed,
                job,
            };
            let snapshot = match doc.take_snapshot_read() {
                Ok(snapshot) => snapshot,
                Err(_) => {
                    registry.release_shell_retirement(shell);
                    registry.release_shell_recovery(shell);
                    registry.release(shell);
                    return Vec::new();
                }
            };
            *registry.shells[shell as usize].borrow_mut() = Some(MountedState::new(identity, snapshot, preflight.config));
            registry.pending[slot] = Some(shell);
            registry.preflight[slot] = None;
            return Vec::new();
        }
        if let Some(current) = registry.current[slot].filter(|current| current.app_instance_id == render.app_instance_id && current.identity.matches_render(render)) {
            if let Ok(mut owner) = registry.shells[current.shell as usize].try_borrow_mut() {
                if let Some(state) = owner.as_mut().filter(|state| !state.spawned && !state.closing && (state.job.is_some() || state.restore.is_some())) {
                    state.spawned = true;
                    return vec![Effect::SpawnJob { job: state.identity.job, kind: ENERGY_SIMULATION_JOB_KIND.into(), input: encode_input(current.shell, state.identity), placement: JobPlacement::Isolated }];
                }
            }
        }
        let pending_shell = registry.pending[slot].filter(|shell| {
            registry.shells[*shell as usize]
                .try_borrow()
                .is_ok_and(|state| state.as_ref().is_some_and(|state| state.identity.app_instance_id == render.app_instance_id && state.job.is_none() && state.capture.is_some() && !state.admission_blocked && !state.closing))
        });
        if let Some(shell) = pending_shell {
            let (identity, checkpoint_token) = {
                let mut owner = match registry.shells[shell as usize].try_borrow_mut() {
                    Ok(owner) => owner,
                    Err(_) => return Vec::new(),
                };
                let state = owner.as_mut().expect("matched pending Energy shell");
                if !state.identity.matches_render(render) {
                    state.begin_close();
                    drop(owner);
                    if registry.retire_shell(shell) {
                        registry.pending[slot] = None;
                    }
                    return Vec::new();
                }
                match state.capture_one() {
                    Ok(false) => return Vec::new(),
                    Ok(true) => (state.identity, state.config.checkpoint_token),
                    Err(_) => {
                        state.begin_close();
                        drop(owner);
                        if registry.retire_shell(shell) {
                            registry.pending[slot] = None;
                        }
                        return vec![Effect::Notify { message: "Energy simulation snapshot became stale".into() }];
                    }
                }
            };
            let checkpoint = if checkpoint_token == 0 {
                None
            } else {
                let Some(previous) = registry.current[slot].filter(|previous| previous.app_instance_id == render.app_instance_id && previous.shell != shell) else {
                    return vec![Effect::Notify { message: "Energy checkpoint owner is unavailable".into() }];
                };
                let mut previous_owner = match registry.shells[previous.shell as usize].try_borrow_mut() {
                    Ok(owner) => owner,
                    Err(_) => return Vec::new(),
                };
                let Some(previous_state) = previous_owner.as_mut().filter(|state| {
                    state.identity.matches_render(render)
                        && state.identity.operation == identity.operation
                        && state.identity.generation == identity.generation
                        && state.identity.operation_seed == identity.operation_seed
                        && state.identity.config_digest == identity.config_digest
                }) else {
                    return vec![Effect::Notify { message: "Energy checkpoint owner is stale".into() }];
                };
                let Some((token, packet)) = previous_state.checkpoint_packet.take() else {
                    return vec![Effect::Notify { message: "Energy checkpoint owner is unavailable".into() }];
                };
                if token != checkpoint_token || !previous_state.packet_is_fresh(packet.identity()) {
                    previous_state.checkpoint_packet = Some((token, packet));
                    return vec![Effect::Notify { message: "Energy checkpoint owner is stale".into() }];
                }
                Some(packet)
            };
            let mut owner = match registry.shells[shell as usize].try_borrow_mut() {
                Ok(owner) => owner,
                Err(_) => return Vec::new(),
            };
            let state = owner.as_mut().expect("captured Energy shell remains mounted");
            match state.admit_job(render, registry.last_request[slot], identity, checkpoint) {
                Ok(()) => {}
                Err(MountedAdmissionError::Stale { checkpoint }) => {
                    if let Some(packet) = checkpoint {
                        state.checkpoint_packet = Some((checkpoint_token, packet));
                    }
                    state.begin_close();
                    drop(owner);
                    if registry.retire_shell(shell) {
                        registry.pending[slot] = None;
                    }
                    return vec![Effect::Notify { message: "Energy simulation admission authority became stale".into() }];
                }
                Err(MountedAdmissionError::Rejected(message)) => {
                    let rejected_identity = state.identity;
                    drop(owner);
                    if let Some(previous) = registry.current[slot] {
                        if !registry.retire_shell(previous.shell) {
                            return vec![Effect::Notify { message: "Energy retirement arena is saturated".into() }];
                        }
                    }
                    registry.current[slot] = Some(CurrentSession { app_instance_id: render.app_instance_id, shell, identity: rejected_identity });
                    registry.pending[slot] = None;
                    return vec![Effect::Notify { message: message.into() }];
                }
            }
            drop(owner);
            let previous = registry.current[slot];
            if let Some(previous) = previous {
                if !registry.retire_shell(previous.shell) {
                    return Vec::new();
                }
                if let Ok(owner) = registry.shells[previous.shell as usize].try_borrow() {
                    if let Some(state) = owner.as_ref() {
                        state.cancel.cancel_now();
                    }
                }
            }
            registry.current[slot] = Some(CurrentSession { app_instance_id: render.app_instance_id, shell, identity });
            registry.pending[slot] = None;
            if let Ok(mut owner) = registry.shells[shell as usize].try_borrow_mut() {
                if let Some(state) = owner.as_mut() {
                    state.spawned = true;
                }
            }
            let mut effects = Vec::with_capacity(2);
            if let Some(previous) = previous {
                effects.push(Effect::CancelJob { job: previous.identity.job });
            }
            effects.push(Effect::SpawnJob { job: identity.job, kind: ENERGY_SIMULATION_JOB_KIND.into(), input: encode_input(shell, identity), placement: JobPlacement::Isolated });
            return effects;
        }
        Vec::new()
    })
}

pub fn with_projection<R>(render: Option<AppRenderOperationContext>, read: impl FnOnce(Option<&EnergySimulationProjection>) -> R) -> R {
    let Some(render) = render else { return read(None) };
    let shell = REGISTRY.with(|registry| {
        let registry = registry.borrow();
        let current = registry.current[registry.slot_for(render.app_instance_id)?]?;
        if current.app_instance_id != render.app_instance_id || !current.identity.matches_render(render) {
            return None;
        }
        Some(registry.shells[current.shell as usize].clone())
    });
    let Some(shell) = shell else { return read(None) };
    let Ok(owner) = shell.try_borrow() else { return read(None) };
    read(owner.as_ref().map(|state| &state.projection))
}

pub fn with_adopted_projection<R>(render: Option<AppRenderOperationContext>, read: impl FnOnce(Option<&EnergySimulationProjection>) -> R) -> R {
    let Some(render) = render else { return read(None) };
    REGISTRY.with(|registry| {
        let registry = registry.borrow();
        read(registry.adopted_projection(render))
    })
}

fn retire_one(app_instance_id: u32, maximum_bytes: usize) -> PluginCloseStep {
    REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let Some(index) = registry.retiring.iter().position(|entry| entry.is_some_and(|shell| registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_some_and(|state| state.identity.app_instance_id == app_instance_id))))
        else {
            return PluginCloseStep::Complete;
        };
        let shell = registry.retiring[index].expect("matched Energy retirement");
        let step = {
            let mut owner = match registry.shells[shell as usize].try_borrow_mut() {
                Ok(owner) => owner,
                Err(_) => return PluginCloseStep::Blocked { reason: "Energy worker owns retirement shell" },
            };
            owner.as_mut().map_or(PluginCloseStep::Complete, |state| state.close_step(1, maximum_bytes))
        };
        if step == PluginCloseStep::Complete {
            if !registry.shells[shell as usize].try_borrow().is_ok_and(|owner| owner.as_ref().is_none_or(MountedState::terminal_is_empty)) {
                return PluginCloseStep::Blocked { reason: "Energy false terminal shell" };
            }
            *registry.shells[shell as usize].borrow_mut() = None;
            registry.retiring[index] = None;
            registry.release_shell_recovery(shell);
            registry.release(shell);
            return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
        }
        step
    })
}

pub fn maintenance_step(app_instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep {
    if maximum_items == 0 {
        return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
    }
    let recovered = RECOVERY.with(|recovery| {
        let mut recovery = recovery.borrow_mut();
        let index = recovery.iter().position(|record| record.is_some_and(|record| record.identity.app_instance_id == app_instance_id))?;
        recovery[index].take()
    });
    if let Some(recovered) = recovered {
        return REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();
            let shell = recovered.shell;
            let mut matched = false;
            if let Ok(mut owner) = registry.shells[shell as usize].try_borrow_mut() {
                if let Some(state) = owner.as_mut().filter(|state| state.identity == recovered.identity) {
                    matched = true;
                    state.worker_attached = false;
                    state.worker_returned = true;
                    state.begin_close();
                }
            }
            if !matched && !registry.retiring.iter().any(|entry| *entry == Some(shell)) {
                return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
            }
            if let Some(slot) = registry.slot_for(app_instance_id) {
                if registry.current[slot].is_some_and(|current| current.shell == shell && current.identity == recovered.identity) {
                    registry.current[slot] = None;
                }
                if registry.pending[slot] == Some(shell) {
                    registry.pending[slot] = None;
                }
            }
            if registry.retiring.iter().any(|entry| *entry == Some(shell)) || registry.retire_shell(shell) {
                PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }
            } else {
                PluginCloseStep::Blocked { reason: "Energy fixed recovery retirement reservation is unavailable" }
            }
        });
    }
    let retired = retire_one(app_instance_id, maximum_bytes);
    if retired != PluginCloseStep::Complete {
        return retired;
    }
    REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let Some(slot) = registry.slot_for(app_instance_id) else { return PluginCloseStep::Complete };
        let Some(current) = registry.current[slot].filter(|current| current.app_instance_id == app_instance_id) else { return PluginCloseStep::Complete };
        let mut owner = match registry.shells[current.shell as usize].try_borrow_mut() {
            Ok(owner) => owner,
            Err(_) => return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 },
        };
        let Some(state) = owner.as_mut() else { return PluginCloseStep::Complete };
        let step = state.maintenance_one(maximum_bytes);
        let adopted = state.projection.adopted.then(|| AdoptedProjectionAuthority::new(state.identity, state.projection)).flatten();
        drop(owner);
        if let Some(adopted) = adopted {
            registry.adopted[slot] = Some(adopted);
        }
        step
    })
}

pub fn close_step(app_instance_id: u32, maximum_items: usize, maximum_bytes: usize) -> PluginCloseStep {
    if maximum_items == 0 {
        return PluginCloseStep::Pending { released_items: 0, released_bytes: 0 };
    }
    let scheduled = REGISTRY.with(|registry| {
        let mut registry = registry.borrow_mut();
        let Some(slot) = registry.slot_for(app_instance_id) else { return false };
        if registry.preflight[slot].take().is_some() {
            return true;
        }
        if let Some(shell) = registry.pending[slot] {
            if registry.retire_shell(shell) {
                registry.pending[slot] = None;
                return true;
            }
            return false;
        }
        if let Some(current) = registry.current[slot].filter(|current| current.app_instance_id == app_instance_id) {
            if registry.retire_shell(current.shell) {
                registry.current[slot] = None;
                return true;
            }
        }
        false
    });
    if scheduled {
        return PluginCloseStep::Pending { released_items: 1, released_bytes: 0 };
    }
    let step = retire_one(app_instance_id, maximum_bytes);
    if step == PluginCloseStep::Complete {
        REGISTRY.with(|registry| {
            let mut registry = registry.borrow_mut();
            if let Some(slot) = registry.slot_for(app_instance_id).filter(|slot| registry.app_terminal_is_empty(*slot, app_instance_id)) {
                registry.apps[slot] = None;
                registry.last_request[slot] = 0;
                registry.adopted[slot] = None;
            }
        });
    }
    step
}

pub fn terminal_is_empty(app_instance_id: u32) -> bool {
    let registry_empty = REGISTRY.with(|registry| {
        let registry = registry.borrow();
        let Some(slot) = registry.slot_for(app_instance_id) else { return true };
        registry.app_terminal_is_empty(slot, app_instance_id)
    });
    registry_empty && RECOVERY.with(|recovery| recovery.borrow().iter().flatten().all(|record| record.identity.app_instance_id != app_instance_id))
}
//#endregion 🎛️ProductSession

//#region 🧪️Laws
#[cfg(test)]
mod tests {
    use super::*;

    fn render(app: u32, generation: u64) -> AppRenderOperationContext {
        AppRenderOperationContext { app_instance_id: app, base_revision: RevisionId(9), generation: Generation(generation), canonical_base_revision: [generation as u8; 32] }
    }

    fn request(request: u64) -> EnergySimulationRequestIdentity {
        EnergySimulationRequestIdentity { request, operation: 1, generation: 1, config_digest: EnergySimulationConfigProjection::default().digest() }
    }

    #[test]
    fn event_log_max_plus_one_preserves_existing_chronology() {
        let mut registry = Registry::new();
        for index in 0..EVENT_SLOTS {
            registry.push_event(render(1, 1), if index == 0 { EnergySimulationEventKind::Start { request: 1, config: EnergySimulationConfigProjection::default() } } else { EnergySimulationEventKind::Cancel(request(1)) }).unwrap();
        }
        assert_eq!(registry.push_event(render(1, 1), EnergySimulationEventKind::Discard(request(1))), Err("energy.session.event-log-saturated"));
        for expected in 1..=EVENT_SLOTS as u64 {
            assert_eq!(registry.pop_event().unwrap().sequence, expected);
        }
    }

    #[test]
    fn fixed_shell_max_plus_one_never_reuses_a_live_owner() {
        let mut registry = Registry::new();
        let mut shells = [0u16; SHELL_SLOTS];
        for shell in &mut shells {
            *shell = registry.allocate().unwrap();
        }
        assert!(registry.allocate().is_none());
        assert_eq!(shells[0], 0);
        assert_eq!(shells[SHELL_SLOTS - 1], (SHELL_SLOTS - 1) as u16);
    }

    #[test]
    fn active_app_slot_max_plus_one_rejects_without_aliasing() {
        let mut registry = Registry::new();
        for app in 1..=ACTIVE_SLOTS as u32 {
            registry.push_event(render(app, 1), EnergySimulationEventKind::Cancel(request(u64::from(app)))).unwrap();
        }
        assert_eq!(registry.push_event(render(ACTIVE_SLOTS as u32 + 1, 1), EnergySimulationEventKind::Cancel(request(99))), Err("energy.session.active-slots-saturated"));
        for app in 1..=ACTIVE_SLOTS as u32 {
            assert!(registry.slot_for(app).is_some());
        }
    }

    #[test]
    fn capture_admission_rejects_item_and_byte_max_plus_one_before_mount() {
        let mut items = CaptureCensus { lane: 0, index: 0, items: MAXIMUM_CAPTURE_ITEMS, bytes: 0 };
        assert_eq!(items.charge_backing(1, 0), Err("energy.session.capture-admission-exceeded"));
        let mut bytes = CaptureCensus { lane: 0, index: 0, items: 0, bytes: MAXIMUM_CAPTURE_BYTES };
        assert_eq!(bytes.charge_backing(1, 1), Err("energy.session.capture-admission-exceeded"));
    }

    #[test]
    fn retirement_max_plus_one_retains_the_rejected_shell() {
        let mut registry = Registry::new();
        for shell in 0..SHELL_SLOTS as u16 {
            assert!(registry.reserve_shell_retirement(shell));
            assert!(registry.retire_shell(shell));
        }
        assert!(registry.reserve_retirement().is_none());
        assert_eq!(registry.retiring[0], Some(0));
        assert_eq!(registry.retiring[SHELL_SLOTS - 1], Some((SHELL_SLOTS - 1) as u16));
    }

    #[test]
    fn checkpoint_selection_and_locale_do_not_change_numerical_digest() {
        let base = EnergySimulationConfigProjection::default();
        let mut restored = base;
        restored.checkpoint_token = u64::MAX;
        restored.locale_de = true;
        assert_eq!(base.digest(), restored.digest());
        restored.zone_timestep_minutes += 1;
        assert_ne!(base.digest(), restored.digest());
    }

    #[test]
    fn invalid_config_is_rejected_before_event_owner_move() {
        let mut invalid = EnergySimulationConfigProjection::default();
        invalid.warmup_days = 366;
        assert!(!invalid.validate());
        invalid = EnergySimulationConfigProjection::default();
        invalid.zone_timestep_minutes = 0;
        assert!(!invalid.validate());
    }

    #[test]
    fn cancel_before_snapshot_admission_retires_the_exact_preflight() {
        let mut registry = Registry::new();
        let operation = render(4, 7);
        registry.push_event(operation, EnergySimulationEventKind::Start { request: 7, config: EnergySimulationConfigProjection::default() }).unwrap();
        apply_event_one(&mut registry);
        let slot = registry.slot_for(operation.app_instance_id).unwrap();
        assert!(registry.preflight[slot].is_some());
        registry.push_event(operation, EnergySimulationEventKind::Cancel(request(7))).unwrap();
        apply_event_one(&mut registry);
        assert!(registry.preflight[slot].is_none());
        assert_eq!(registry.free_len, SHELL_SLOTS);
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
        assert_eq!(census.bytes, 8 + 3 * std::mem::size_of::<[f64; 3]>());
        assert!(!census.step_one(&source).unwrap());
        assert_eq!(census.items, 12);
        assert_eq!(census.bytes, 8 + 3 * std::mem::size_of::<[f64; 3]>() + std::mem::size_of::<crate::model::Surface>());
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
        let source = include_str!("component.rs");
        for forbidden in [concat!("item", ".clone()"), concat!("airflow_network", ".clone()"), concat!("ground_temperature", ".clone()")] {
            assert!(!source.contains(forbidden), "whole record mutation survived: {forbidden}");
        }
        assert!(source.contains("capture-nested-vector-reserve"));
    }

    #[test]
    fn chronology_is_identical_for_one_two_four_and_default_fuel() {
        let expected = [EnergyQualityTier::SteadyStateEstimate, EnergyQualityTier::DesignDay, EnergyQualityTier::CoarseTimestep, EnergyQualityTier::Final];
        for fuel in [1u64, 2, 4, 64] {
            let mut cursor = 0;
            let mut observed = [EnergyQualityTier::SteadyStateEstimate; 4];
            while cursor < expected.len() {
                let admitted = usize::try_from(fuel.min(1)).unwrap();
                for _ in 0..admitted {
                    observed[cursor] = expected[cursor];
                    cursor += 1;
                }
            }
            assert_eq!(observed, expected);
            assert_eq!(observed.map(quality_tier_index), [0, 1, 2, 3]);
        }
    }

    #[test]
    fn lower_tier_and_stale_sequence_cannot_replace_visible_authority() {
        let identity = MountedIdentity {
            app_instance_id: 3,
            request: 1,
            document_revision: RevisionId(9),
            document_generation: Generation(4),
            canonical_base_revision: [4; 32],
            operation: OperationId(7),
            generation: Generation(2),
            config_digest: 8,
            operation_seed: 9,
            job: 7,
        };
        let mut projection = EnergySimulationProjection::new(identity);
        projection.latest_sequence = 9;
        projection.latest_tier = Some(EnergyQualityTier::CoarseTimestep);
        let tier = match EnergyQualityTier::SteadyStateEstimate {
            EnergyQualityTier::SteadyStateEstimate => 0,
            _ => 3,
        };
        assert!(tier < 2);
        assert_eq!(projection.latest_sequence, 9);
    }

    #[test]
    fn process_token_rejects_stale_generation_and_tags_the_exact_job() {
        let identity = MountedIdentity {
            app_instance_id: 1,
            request: 1,
            document_revision: RevisionId(2),
            document_generation: Generation(3),
            canonical_base_revision: [4; 32],
            operation: OperationId(JOB_TAG | 1),
            generation: Generation(5),
            config_digest: 6,
            operation_seed: 7,
            job: JOB_TAG | 1,
        };
        let bytes = encode_input(0, identity);
        assert_eq!(decode_input(identity.job, &bytes).unwrap().1, identity);
        assert_ne!(decode_input(identity.job + 1, &bytes).unwrap().1, identity);
        let mut stale = bytes;
        stale[31..39].copy_from_slice(&0u64.to_le_bytes());
        assert!(decode_input(identity.job, &stale).is_none());
    }

    #[test]
    fn schema_and_accessibility_vocabulary_is_complete() {
        let source = include_str!("../✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️component.rs");
        for law in ["Start simulation", "Simulation starten", "Cancel simulation", "Simulation abbrechen", "aria-live", "busy", "Final result"] {
            assert!(source.contains(law), "missing {law}");
        }
    }

    #[test]
    fn artifact_read_path_has_no_process_cache_clone_or_serde_key_authority() {
        let model = Model { name: "store-owned".into(), ..Model::default() };
        let snapshot = crate::artifacts::model::energy_snapshot_with_state(crate::artifacts::model::ENERGY_MODEL_DOCUMENT_SCHEMA, &model, None);
        assert_eq!(snapshot.model, model, "the event-sourced snapshot, not a side cache, is the exact numerical read authority");
        let artifact = include_str!("../../../../../🦀️component.rs");
        for forbidden in ["ENERGY_SCRATCH", "with_energy_model_ref", "HashMap<String, EnergyWorkingScene>", "energy_scene_id"] {
            assert!(!artifact.contains(forbidden), "process cache authority survived: {forbidden}");
        }
        assert!(artifact.contains("pub struct EnergyModelReadLease"));
        assert!(artifact.contains("commit_authority_matches"));
        assert!(artifact.contains("return_to_registry_witness"));
    }

    #[test]
    fn exact_request_identity_rejects_each_stale_lifecycle_dimension() {
        let identity = MountedIdentity {
            app_instance_id: 8,
            request: 19,
            document_revision: RevisionId(2),
            document_generation: Generation(3),
            canonical_base_revision: [4; 32],
            operation: OperationId(5),
            generation: Generation(6),
            config_digest: 7,
            operation_seed: 8,
            job: JOB_TAG | 9,
        };
        let exact = EnergySimulationRequestIdentity { request: 19, operation: 5, generation: 6, config_digest: 7 };
        assert!(identity.matches_request(exact));
        for stale in [
            EnergySimulationRequestIdentity { request: 20, ..exact },
            EnergySimulationRequestIdentity { operation: 9, ..exact },
            EnergySimulationRequestIdentity { generation: 9, ..exact },
            EnergySimulationRequestIdentity { config_digest: 9, ..exact },
        ] {
            assert!(!identity.matches_request(stale));
        }
    }

    #[test]
    fn completed_capture_revalidates_every_live_authority_before_model_transfer() {
        let mounted = MountedIdentity {
            app_instance_id: 61,
            request: 19,
            document_revision: RevisionId(2),
            document_generation: Generation(3),
            canonical_base_revision: [4; 32],
            operation: OperationId(5),
            generation: Generation(6),
            config_digest: 7,
            operation_seed: 8,
            job: JOB_TAG | 9,
        };
        let exact_render = AppRenderOperationContext { app_instance_id: mounted.app_instance_id, base_revision: mounted.document_revision, generation: mounted.document_generation, canonical_base_revision: mounted.canonical_base_revision };
        let stale_expected = MountedIdentity { generation: Generation(99), ..mounted };
        let stale_app = AppRenderOperationContext { app_instance_id: 62, ..exact_render };
        let stale_revision = AppRenderOperationContext { base_revision: RevisionId(99), ..exact_render };
        let stale_generation = AppRenderOperationContext { generation: Generation(99), ..exact_render };
        let stale_canonical = AppRenderOperationContext { canonical_base_revision: [99; 32], ..exact_render };
        for (expected, render, request, digest, snapshot_fresh, cancelled) in [
            (stale_expected, exact_render, mounted.request, mounted.config_digest, true, false),
            (mounted, stale_app, mounted.request, mounted.config_digest, true, false),
            (mounted, stale_revision, mounted.request, mounted.config_digest, true, false),
            (mounted, stale_generation, mounted.request, mounted.config_digest, true, false),
            (mounted, stale_canonical, mounted.request, mounted.config_digest, true, false),
            (mounted, exact_render, mounted.request + 1, mounted.config_digest, true, false),
            (mounted, exact_render, mounted.request, mounted.config_digest + 1, true, false),
            (mounted, exact_render, mounted.request, mounted.config_digest, false, false),
            (mounted, exact_render, mounted.request, mounted.config_digest, true, true),
        ] {
            let mut capture = ModelCapture::new();
            capture.model.name = "stale-capture".into();
            capture.model.zones.push(crate::model::Zone { id: crate::model::EntityId(1), name: "retained-zone".into(), volume_m3: 1.0, multiplier: 1, conditioned: true, part_of_total_floor_area: true });
            let mut capture = Some(capture);
            assert!(take_captured_model_for_admission(&mut capture, mounted, expected, render, request, digest, snapshot_fresh, cancelled).is_err());
            assert_eq!(capture.as_ref().expect("stale capture remains exact").model.zones[0].name, "retained-zone");
            let mut close = EnergyModelCloseCursor::new(std::mem::replace(&mut capture, None).expect("retained stale capture").finish());
            for _ in 0..128 {
                match close.close_step(4) {
                    semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                        assert!(released_items <= 1);
                        assert!(released_bytes <= 4);
                    }
                    semio_framework_job::InteractiveJobCloseStep::Complete => break,
                    semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("stale captured Model close cannot block"),
                }
            }
            assert!(close.terminal_is_empty());
        }

        let mut exact = ModelCapture::new();
        exact.model.name = "exact-capture".into();
        let mut exact = Some(exact);
        let model = take_captured_model_for_admission(&mut exact, mounted, mounted, exact_render, mounted.request, mounted.config_digest, true, false).expect("only the current uncancelled capture transfers");
        assert_eq!(model.name, "exact-capture");
        assert!(exact.is_none());
    }

    #[test]
    fn adopted_projection_is_partitioned_by_application_and_rejects_every_aba_dimension() {
        let identity = MountedIdentity {
            app_instance_id: 71,
            request: 29,
            document_revision: RevisionId(12),
            document_generation: Generation(13),
            canonical_base_revision: [14; 32],
            operation: OperationId(15),
            generation: Generation(16),
            config_digest: 17,
            operation_seed: 18,
            job: JOB_TAG | 19,
        };
        let render = AppRenderOperationContext { app_instance_id: identity.app_instance_id, base_revision: identity.document_revision, generation: identity.document_generation, canonical_base_revision: identity.canonical_base_revision };
        let mut projection = EnergySimulationProjection::new(identity);
        projection.adopted = true;
        projection.status = EnergySimulationStatus::Adopted;
        projection.tiers[0] = Some(EnergyTierProjection {
            app_instance_id: identity.app_instance_id,
            document_revision: identity.document_revision,
            document_generation: identity.document_generation,
            canonical_base_revision: identity.canonical_base_revision,
            operation: identity.operation,
            generation: identity.generation,
            config_digest: identity.config_digest,
            sequence: 1,
            tier: EnergyQualityTier::SteadyStateEstimate,
            stage: EnergyJobStage::Complete,
            warmup_hour: 0,
            timestep: 1,
            total_timesteps: 1,
            facility_electricity_kwh: 1.0,
        });
        let exact = AdoptedProjectionAuthority::new(identity, projection).expect("exact adopted authority");
        let mut registry = Registry::new();
        registry.apps[0] = Some(identity.app_instance_id);
        registry.apps[1] = Some(identity.app_instance_id + 1);
        registry.last_request[0] = identity.request;
        registry.adopted[0] = Some(exact);
        assert_eq!(registry.adopted_projection(render).map(|projection| projection.request), Some(identity.request));
        let other_app_same_document = AppRenderOperationContext { app_instance_id: identity.app_instance_id + 1, ..render };
        assert!(registry.adopted_projection(other_app_same_document).is_none(), "matching document provenance cannot cross the application partition");

        for mutation in 0..5 {
            let mut stale = exact;
            match mutation {
                0 => stale.projection.request += 1,
                1 => stale.projection.operation = OperationId(stale.projection.operation.0 + 1),
                2 => stale.projection.generation = Generation(stale.projection.generation.0 + 1),
                3 => stale.projection.config_digest += 1,
                _ => stale.projection.tiers[0].as_mut().expect("tier").app_instance_id += 1,
            }
            registry.adopted[0] = Some(stale);
            assert!(registry.adopted_projection(render).is_none(), "ABA mutation {mutation} leaked an adopted projection");
        }
        registry.adopted[0] = Some(exact);
        registry.last_request[0] += 1;
        assert!(registry.adopted_projection(render).is_none(), "a newer request must invalidate the retained adopted authority");
    }

    #[test]
    fn acknowledged_numerical_complete_is_retained_then_detaches_the_process_owner() {
        let mut retained = None;
        let complete = StepOutcome::Complete(semio_framework_job::CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
        });
        assert!(matches!(retain_worker_outcome(&mut retained, complete), JobStep::Done(bytes) if bytes.is_empty()));
        assert!(retained.as_ref().is_some_and(StepOutcome::is_terminal));
        assert!(retained.as_ref().is_some_and(StepOutcome::terminal_is_empty));
        let identity = MountedIdentity {
            app_instance_id: 81,
            request: 82,
            document_revision: RevisionId(83),
            document_generation: Generation(84),
            canonical_base_revision: [85; 32],
            operation: OperationId(86),
            generation: Generation(87),
            config_digest: 88,
            operation_seed: 89,
            job: JOB_TAG | 90,
        };
        let shell = 5;
        drop(EnergyMountedBoundedJob { shell_index: shell, shell: Rc::new(RefCell::new(None)), identity });
        let recovered = RECOVERY.with(|recovery| recovery.borrow_mut()[shell as usize].take()).expect("terminal process Drop publishes the exact fixed recovery witness");
        assert_eq!(recovered, RecoveryRecord { shell, identity });
    }

    #[test]
    fn reused_or_older_start_request_cannot_replace_current_preflight_authority() {
        let render = render(18, 1);
        let mut registry = Registry::new();
        registry.push_event(render, EnergySimulationEventKind::Start { request: 9, config: EnergySimulationConfigProjection::default() }).unwrap();
        apply_event_one(&mut registry);
        let slot = registry.slot_for(18).unwrap();
        assert_eq!(registry.preflight[slot].map(|preflight| preflight.request), Some(9));
        registry.push_event(render, EnergySimulationEventKind::Start { request: 9, config: EnergySimulationConfigProjection { warmup_days: 99, ..EnergySimulationConfigProjection::default() } }).unwrap();
        apply_event_one(&mut registry);
        assert_eq!(registry.preflight[slot].map(|preflight| (preflight.request, preflight.config.warmup_days)), Some((9, EnergySimulationConfigProjection::default().warmup_days)));
        assert_eq!(registry.last_request[slot], 9);
    }

    #[test]
    fn retry_path_has_no_default_config_fallback_or_unrelated_preflight() {
        let source = include_str!("component.rs");
        let retry = &source[source.find("EnergySimulationEventKind::Retry(request)").expect("retry")..source.find("EnergySimulationEventKind::Cancel(request)").expect("cancel")];
        assert!(!retry.contains("unwrap_or_default"));
        assert!(retry.contains("matches_request(request)"));
        assert!(retry.contains("state.config"));
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
                semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1);
                    assert!(released_bytes <= 4);
                }
                semio_framework_job::InteractiveJobCloseStep::Complete => {}
                semio_framework_job::InteractiveJobCloseStep::Blocked => panic!("owned partial model cannot block"),
            }
            turns += 1;
            assert!(turns < 64);
        }
    }

    #[test]
    fn retirement_is_reserved_before_snapshot_or_shell_owner_move() {
        let mut registry = Registry::new();
        let mut shells = [0u16; SHELL_SLOTS];
        for shell in &mut shells {
            *shell = registry.allocate().expect("fixed shell");
            assert!(registry.reserve_shell_retirement(*shell));
            assert!(registry.reserve_shell_recovery(*shell));
        }
        assert!(registry.allocate().is_none());
        assert!(registry.retire_shell(shells[0]));
        let source = include_str!("component.rs");
        let reconcile = &source[source.find("pub fn reconcile(").expect("reconcile")..source.find("pub fn with_projection").expect("projection")];
        assert!(reconcile.find("reserve_shell_retirement").expect("reservation") < reconcile.find("take_snapshot_read").expect("snapshot move"));
        assert!(reconcile.find("reserve_shell_recovery").expect("recovery reservation") < reconcile.find("take_snapshot_read").expect("snapshot move"));
        assert!(reconcile.find("reserve_shell_retirement").expect("reservation") < reconcile.find("MountedState::new").expect("shell owner move"));
    }

    #[test]
    fn every_worker_drop_publishes_exact_fixed_recovery_identity() {
        let identity = MountedIdentity {
            app_instance_id: 31,
            request: 41,
            document_revision: RevisionId(2),
            document_generation: Generation(3),
            canonical_base_revision: [4; 32],
            operation: OperationId(5),
            generation: Generation(6),
            config_digest: 7,
            operation_seed: 8,
            job: JOB_TAG | 9,
        };
        let shell = 3;
        drop(EnergyMountedBoundedJob { shell_index: shell, shell: Rc::new(RefCell::new(None)), identity });
        let recovered = RECOVERY.with(|recovery| recovery.borrow_mut()[shell as usize].take()).expect("normal, lost, panic and cancellation share one unconditional Drop publisher");
        assert_eq!(recovered.shell, shell);
        assert_eq!(recovered.identity, identity);
    }

    #[test]
    fn rejected_whole_capture_and_terminal_loss_mutations_are_absent() {
        let source = include_str!("component.rs");
        for forbidden in [concat!("self.capture", ".take()"), "clean_terminal", "state.abandoned", "try_borrow_mut() {\n            if let Some(state) = owner.as_mut() {\n                state.abandoned"] {
            assert!(!source.contains(forbidden), "owner-loss mutation survived: {forbidden}");
        }
        assert!(source.contains("capture_close"));
        assert!(source.contains("RECOVERY.with"));
        assert!(source.contains("worker_returned"));
    }
}
//#endregion 🧪️Laws
