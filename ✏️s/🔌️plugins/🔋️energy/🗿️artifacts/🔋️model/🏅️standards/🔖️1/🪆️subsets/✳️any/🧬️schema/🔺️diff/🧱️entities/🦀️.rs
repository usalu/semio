//! 🧱️ The energy model's entity patches — one sparse patch per row type of [`crate::model::Model`], one per record, and the
//! whole-model patch that names them all. A row patch addresses its row by id and carries only the fields that change:
//! `set` assigns a scalar, `opt` sets or clears an optional field, `list` edits an ordered id list, `slots` assigns
//! entries of a fixed array, `rows`/`with` nest a collection or a record.

use super::patch::{patch, patch_field_ty, record_patch, row_patch, ArrayShape, FieldPatch, ListEdit, OptionChange, Row, RowPatch, Rows, Slots, Unchanged};
use protocol::MutationApplyError;
use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};


//#region 🔖️CompositeKeys
/// 🩹 Sparse patch over one [`OutputVariableSpec`](crate::model::OutputVariableSpec) row, addressed by `(name, key)`.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive)]
pub struct OutputVariableSpecPatch {
    pub name: String,
    pub key: String,
    #[value(default, skip_serializing_if = "Unchanged::unchanged")]
    pub reporting_frequency: Option<crate::model::OutputReportFrequency>,
}

impl OutputVariableSpecPatch {
    pub fn of(name: String, key: String) -> Self {
        Self { name, key, reporting_frequency: None }
    }
}

impl Unchanged for OutputVariableSpecPatch {
    fn unchanged(&self) -> bool {
        self.reporting_frequency.unchanged()
    }
}

impl FieldPatch for OutputVariableSpecPatch {
    type Target = crate::model::OutputVariableSpec;
    fn apply(&self, target: &mut Self::Target) -> Result<(), MutationApplyError> {
        self.reporting_frequency.apply(&mut target.reporting_frequency)
    }
    fn absorb(&mut self, later: Self) {
        self.reporting_frequency.absorb(later.reporting_frequency);
    }
    fn inverse(&self, base: &Self::Target) -> Self {
        Self { name: self.name.clone(), key: self.key.clone(), reporting_frequency: self.reporting_frequency.inverse(&base.reporting_frequency) }
    }
    fn between(base: &Self::Target, other: &Self::Target) -> Self {
        Self { name: base.name.clone(), key: base.key.clone(), reporting_frequency: <Option<crate::model::OutputReportFrequency> as FieldPatch>::between(&base.reporting_frequency, &other.reporting_frequency) }
    }
}

impl Row for crate::model::OutputVariableSpec {
    type Key = (String, String);
    fn key(&self) -> Self::Key {
        (self.name.clone(), self.key.clone())
    }
}

impl RowPatch for OutputVariableSpecPatch {
    fn key(&self) -> (String, String) {
        (self.name.clone(), self.key.clone())
    }
}

/// 🩹 Sparse patch over one [`AdjacencyPair`](crate::model::AdjacencyPair) row, addressed by its two surfaces; a pair has no
/// field to modify, so only insertion and removal ever name it.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive)]
pub struct AdjacencyPairPatch {
    pub surface_a_id: crate::model::EntityId,
    pub surface_b_id: crate::model::EntityId,
}

impl AdjacencyPairPatch {
    pub fn of(surface_a_id: crate::model::EntityId, surface_b_id: crate::model::EntityId) -> Self {
        Self { surface_a_id, surface_b_id }
    }
}

impl Unchanged for AdjacencyPairPatch {
    fn unchanged(&self) -> bool {
        true
    }
}

impl FieldPatch for AdjacencyPairPatch {
    type Target = crate::model::AdjacencyPair;
    fn apply(&self, _target: &mut Self::Target) -> Result<(), MutationApplyError> {
        Ok(())
    }
    fn absorb(&mut self, _later: Self) {}
    fn inverse(&self, _base: &Self::Target) -> Self {
        self.clone()
    }
    fn between(base: &Self::Target, _other: &Self::Target) -> Self {
        Self { surface_a_id: base.surface_a_id, surface_b_id: base.surface_b_id }
    }
}

impl Row for crate::model::AdjacencyPair {
    type Key = (crate::model::EntityId, crate::model::EntityId);
    fn key(&self) -> Self::Key {
        (self.surface_a_id, self.surface_b_id)
    }
}

impl RowPatch for AdjacencyPairPatch {
    fn key(&self) -> (crate::model::EntityId, crate::model::EntityId) {
        (self.surface_a_id, self.surface_b_id)
    }
}
//#endregion 🔖️CompositeKeys

//#region 🔖️Patches
row_patch! {
    /// 🩹 Sparse patch over one [`Zone`] row.
    ZonePatch for crate::model::Zone {
        key id: crate::model::EntityId;
        set name: String,
        set volume_m3: f64,
        set multiplier: u32,
        set conditioned: bool,
        set part_of_total_floor_area: bool,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`Space`] row.
    SpacePatch for crate::model::Space {
        key id: crate::model::EntityId;
        set name: String,
        set zone_id: crate::model::EntityId,
        set floor_area_m2: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`Surface`] row.
    SurfacePatch for crate::model::Surface {
        key id: crate::model::EntityId;
        set name: String,
        set zone_id: crate::model::EntityId,
        set class: crate::model::SurfaceClass,
        set vertices_m: Vec<[f64; 3]>,
        set construction_id: crate::model::EntityId,
        set outside_boundary_condition: crate::model::OutsideBoundary,
        set sun_exposed: bool,
        set wind_exposed: bool,
        set multiplier: u32,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`Fenestration`] row.
    FenestrationPatch for crate::model::Fenestration {
        key id: crate::model::EntityId;
        set name: String,
        set surface_id: crate::model::EntityId,
        set u_value_w_m2k: f64,
        set shgc: f64,
        set vlt: f64,
        set area_m2: f64,
        set height_m: f64,
        set sill_height_m: f64,
        set frame_conductance_w_k: f64,
        set divider_conductance_w_k: f64,
        set overhang_depth_m: f64,
        set overhang_offset_m: f64,
        set fin_depth_m: f64,
        set fin_offset_m: f64,
        opt glazing_construction_id: crate::model::EntityId,
        set vertices_m: Vec<[f64; 3]>,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`Material`] row.
    MaterialPatch for crate::model::Material {
        key id: crate::model::EntityId;
        set name: String,
        set roughness: crate::model::SurfaceRoughness,
        set thickness_m: f64,
        set conductivity_w_m_k: f64,
        set density_kg_m3: f64,
        set specific_heat_j_kg_k: f64,
        set thermal_absorptance: f64,
        set solar_absorptance: f64,
        set visible_absorptance: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`GlazingMaterial`] row.
    GlazingMaterialPatch for crate::model::GlazingMaterial {
        key id: crate::model::EntityId;
        set name: String,
        set thickness_m: f64,
        set conductivity_w_m_k: f64,
        set solar_transmittance: f64,
        set solar_reflectance_front: f64,
        set solar_reflectance_back: f64,
        set visible_transmittance: f64,
        set visible_reflectance_front: f64,
        set visible_reflectance_back: f64,
        set infrared_transmittance: f64,
        set infrared_emissivity_front: f64,
        set infrared_emissivity_back: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`GasMaterial`] row.
    GasMaterialPatch for crate::model::GasMaterial {
        key id: crate::model::EntityId;
        set name: String,
        set thickness_m: f64,
        set gas: crate::model::GasKind,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`Construction`] row.
    ConstructionPatch for crate::model::Construction {
        key id: crate::model::EntityId;
        set name: String,
        list layer_material_ids: crate::model::EntityId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`PeopleGain`] row.
    PeopleGainPatch for crate::model::PeopleGain {
        key id: crate::model::EntityId;
        set zone_id: crate::model::EntityId,
        set schedule_id: crate::model::ScheduleId,
        set activity_schedule_id: crate::model::ScheduleId,
        set people_per_area: f64,
        set sensible_fraction: f64,
        set latent_fraction: f64,
        set radiant_fraction: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`LightingGain`] row.
    LightingGainPatch for crate::model::LightingGain {
        key id: crate::model::EntityId;
        set zone_id: crate::model::EntityId,
        set schedule_id: crate::model::ScheduleId,
        set watts_per_area: f64,
        set radiant_fraction: f64,
        set visible_fraction: f64,
        set return_air_fraction: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`EquipmentGain`] row.
    EquipmentGainPatch for crate::model::EquipmentGain {
        key id: crate::model::EntityId;
        set zone_id: crate::model::EntityId,
        set schedule_id: crate::model::ScheduleId,
        set watts_per_area: f64,
        set radiant_fraction: f64,
        set latent_fraction: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`Thermostat`] row.
    ThermostatPatch for crate::model::Thermostat {
        key id: crate::model::EntityId;
        set zone_id: crate::model::EntityId,
        set heating_setpoint_schedule_id: crate::model::ScheduleId,
        set cooling_setpoint_schedule_id: crate::model::ScheduleId,
        set heating_throttle_range_k: f64,
        set cooling_throttle_range_k: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`Humidistat`] row.
    HumidistatPatch for crate::model::Humidistat {
        key id: crate::model::EntityId;
        set zone_id: crate::model::EntityId,
        set humidifying_setpoint_schedule_id: crate::model::ScheduleId,
        set dehumidifying_setpoint_schedule_id: crate::model::ScheduleId,
        set humidifying_throttle_range: f64,
        set dehumidifying_throttle_range: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`SetpointManager`] row.
    SetpointManagerPatch for crate::model::SetpointManager {
        key id: crate::model::EntityId;
        set name: String,
        set kind: crate::model::SetpointManagerKind,
        opt schedule_id: crate::model::ScheduleId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`IdealLoadsSystem`] row.
    IdealLoadsSystemPatch for crate::model::IdealLoadsSystem {
        key id: crate::model::EntityId;
        set zone_id: crate::model::EntityId,
        set max_heating_supply_air_temp_c: f64,
        set min_cooling_supply_air_temp_c: f64,
        opt max_heating_capacity_w: f64,
        opt max_cooling_capacity_w: f64,
        set outdoor_air_per_person_m3_s: f64,
        set outdoor_air_per_area_m3_s_m2: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`ZoneEquipmentAssignment`] row.
    ZoneEquipmentAssignmentPatch for crate::model::ZoneEquipmentAssignment {
        key id: crate::model::EntityId;
        set zone_id: crate::model::EntityId,
        set equipment_type: crate::model::ZoneEquipmentType,
        set priority: u8,
        set heating_capacity_w: f64,
        set cooling_capacity_w: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`ModelAirLoop`] row.
    ModelAirLoopPatch for crate::model::ModelAirLoop {
        key id: crate::model::EntityId;
        set name: String,
        set supply_node_id: u32,
        set return_node_id: u32,
        set design_supply_air_flow_m3_s: f64,
        list terminal_zone_ids: crate::model::EntityId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`PlantLoopConfig`] row.
    PlantLoopConfigPatch for crate::model::PlantLoopConfig {
        key id: crate::model::EntityId;
        set name: String,
        set loop_type: crate::model::PlantLoopType,
        set supply_temperature_c: f64,
        set return_temperature_c: f64,
        set design_flow_kg_s: f64,
        list equipment_ids: crate::model::EntityId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`OutdoorAirSystem`] row.
    OutdoorAirSystemPatch for crate::model::OutdoorAirSystem {
        key id: crate::model::EntityId;
        set air_loop_id: crate::model::EntityId,
        set min_oa_flow_m3_s: f64,
        set economizer_enabled: bool,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`Infiltration`] row.
    InfiltrationPatch for crate::model::Infiltration {
        key id: crate::model::EntityId;
        set zone_id: crate::model::EntityId,
        set schedule_id: crate::model::ScheduleId,
        set method: crate::air_exchange::InfiltrationMethod,
        set design_flow_ach: f64,
        set flow_per_exterior_area_m3_s_m2: f64,
        set effective_leakage_area_m2: f64,
        set discharge_coefficient: f64,
        set stack_height_m: f64,
        set constant_term_coefficient: f64,
        set temperature_term_coefficient: f64,
        set velocity_term_coefficient: f64,
        set velocity_squared_term_coefficient: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`MechanicalVentilation`] row.
    MechanicalVentilationPatch for crate::model::MechanicalVentilation {
        key id: crate::model::EntityId;
        set zone_id: crate::model::EntityId,
        set schedule_id: crate::model::ScheduleId,
        set design_flow_m3_s: f64,
        set fan_total_efficiency: f64,
        set fan_delta_pressure_pa: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`ShadingSurface`] row.
    ShadingSurfacePatch for crate::model::ShadingSurface {
        key id: crate::model::EntityId;
        set name: String,
        set vertices_m: Vec<[f64; 3]>,
        opt transmittance_schedule_id: crate::model::ScheduleId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`SpaceList`] row.
    SpaceListPatch for crate::model::SpaceList {
        key id: crate::model::EntityId;
        set name: String,
        list space_ids: crate::model::EntityId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`ThermalEnclosure`] row.
    ThermalEnclosurePatch for crate::model::ThermalEnclosure {
        key id: crate::model::EntityId;
        set name: String,
        list zone_ids: crate::model::EntityId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`ElectricalLoadCenter`] row.
    ElectricalLoadCenterPatch for crate::model::ElectricalLoadCenter {
        key id: crate::model::EntityId;
        set name: String,
        list generator_ids: crate::model::EntityId,
        list pv_ids: crate::model::EntityId,
        list battery_ids: crate::model::EntityId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`PvSystemAssignment`] row.
    PvSystemAssignmentPatch for crate::model::PvSystemAssignment {
        key id: crate::model::EntityId;
        set dc_capacity_w: f64,
        set area_m2: f64,
        set tilt_deg: f64,
        set azimuth_deg: f64,
        set module_efficiency: f64,
        set inverter_efficiency: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`BatteryAssignment`] row.
    BatteryAssignmentPatch for crate::model::BatteryAssignment {
        key id: crate::model::EntityId;
        set capacity_kwh: f64,
        set max_charge_w: f64,
        set max_discharge_w: f64,
        set round_trip_efficiency: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`ShwSystemConfig`] row.
    ShwSystemConfigPatch for crate::model::ShwSystemConfig {
        key id: crate::model::EntityId;
        set heater_capacity_w: f64,
        set storage_volume_m3: f64,
        set setpoint_c: f64,
        set schedule_id: crate::model::ScheduleId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`SolarThermalConfig`] row.
    SolarThermalConfigPatch for crate::model::SolarThermalConfig {
        key id: crate::model::EntityId;
        set collector_area_m2: f64,
        set efficiency: f64,
        set storage_volume_m3: f64,
        set tilt_deg: f64,
        set azimuth_deg: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`RefrigerationConfig`] row.
    RefrigerationConfigPatch for crate::model::RefrigerationConfig {
        key id: crate::model::EntityId;
        set case_count: u32,
        set design_load_w: f64,
        set defrost_schedule_id: crate::model::ScheduleId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`WaterSystemConfig`] row.
    WaterSystemConfigPatch for crate::model::WaterSystemConfig {
        key id: crate::model::EntityId;
        set fixture_count: u32,
        set peak_flow_l_s: f64,
        set schedule_id: crate::model::ScheduleId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`FaultDefinition`] row.
    FaultDefinitionPatch for crate::model::FaultDefinition {
        key id: crate::model::EntityId;
        set target_equipment_id: crate::model::EntityId,
        set fault_type: crate::model::FaultType,
        set severity: f64,
        set start_schedule_id: crate::model::ScheduleId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`SizingObject`] row.
    SizingObjectPatch for crate::model::SizingObject {
        key id: crate::model::EntityId;
        set zone_id: crate::model::EntityId,
        set sizing_type: crate::model::SizingType,
        set design_day_type: crate::model::DesignDayType,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`DaylightZoneConfig`] row.
    DaylightZoneConfigPatch for crate::model::DaylightZoneConfig {
        key id: crate::model::EntityId;
        set zone_id: crate::model::EntityId,
        set illuminance_target_lux: f64,
        set glare_limit: f64,
        set window_transmittance: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`RoomAirModelAssignment`] row.
    RoomAirModelAssignmentPatch for crate::model::RoomAirModelAssignment {
        key zone_id: crate::model::EntityId;
        set model: crate::model::RoomAirModelType,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`ConstantSchedule`] row.
    ConstantSchedulePatch for crate::schedule::ConstantSchedule {
        key id: crate::model::ScheduleId;
        set value: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`DailySchedule`] row.
    DailySchedulePatch for crate::schedule::DailySchedule {
        key id: crate::model::ScheduleId;
        slots hourly_values: [f64; 24],
        set interpolation: crate::schedule::ScheduleInterpolation,
        opt limits: crate::schedule::ScheduleLimits,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`WeeklySchedule`] row.
    WeeklySchedulePatch for crate::schedule::WeeklySchedule {
        key id: crate::model::ScheduleId;
        slots daily_schedule_ids: [crate::model::ScheduleId; 7],
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`AnnualSchedule`] row.
    AnnualSchedulePatch for crate::schedule::AnnualSchedule {
        key id: crate::model::ScheduleId;
        list rules: crate::schedule::CompactScheduleRule,
        set default_daily_schedule_id: crate::model::ScheduleId,
        opt holiday_daily_schedule_id: crate::model::ScheduleId,
        list holiday_dates: (u16, u8, u8),
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`TimeSeriesSchedule`] row.
    TimeSeriesSchedulePatch for crate::schedule::TimeSeriesSchedule {
        key id: crate::model::ScheduleId;
        set values: Vec<f64>,
        set timestep_seconds: u32,
    }
}

record_patch! {
    /// 🩹 Sparse patch over the [`Site`] record.
    SitePatch for crate::model::Site {
        set latitude_deg: f64,
        set longitude_deg: f64,
        set elevation_m: f64,
        set time_zone_hours: f64,
        set north_axis_deg: f64,
    }
}

record_patch! {
    /// 🩹 Sparse patch over the [`GroundTemperatureConfig`] record.
    GroundTemperatureConfigPatch for crate::model::GroundTemperatureConfig {
        slots building_surface_c: [f64; 12],
        slots shallow_c: [f64; 12],
        set deep_c: f64,
    }
}

record_patch! {
    /// 🩹 Sparse patch over the [`RunPeriod`] record.
    RunPeriodPatch for crate::calendar::RunPeriod {
        set start_month: u8,
        set start_day: u8,
        set end_month: u8,
        set end_day: u8,
        set year: u16,
    }
}

record_patch! {
    /// 🩹 Sparse patch over the [`ScheduleSet`](crate::schedule::ScheduleSet) record.
    ScheduleSetPatch for crate::schedule::ScheduleSet {
        rows constants: ConstantSchedulePatch,
        rows daily: DailySchedulePatch,
        rows weekly: WeeklySchedulePatch,
        rows annual: AnnualSchedulePatch,
        rows time_series: TimeSeriesSchedulePatch,
    }
}

record_patch! {
    /// 🩹 Sparse patch over the whole [`Model`](crate::model::Model).
    ModelPatch for crate::model::Model {
        set name: String,
        set version: String,
        with site: SitePatch,
        rows zones: ZonePatch,
        rows spaces: SpacePatch,
        rows surfaces: SurfacePatch,
        rows fenestrations: FenestrationPatch,
        rows materials: MaterialPatch,
        rows glazing_materials: GlazingMaterialPatch,
        rows gas_materials: GasMaterialPatch,
        rows constructions: ConstructionPatch,
        rows people: PeopleGainPatch,
        rows lighting: LightingGainPatch,
        rows equipment: EquipmentGainPatch,
        rows thermostats: ThermostatPatch,
        rows humidistats: HumidistatPatch,
        rows setpoint_managers: SetpointManagerPatch,
        rows ideal_loads: IdealLoadsSystemPatch,
        rows zone_equipment: ZoneEquipmentAssignmentPatch,
        rows air_loops: ModelAirLoopPatch,
        rows plant_loops: PlantLoopConfigPatch,
        rows outdoor_air_systems: OutdoorAirSystemPatch,
        rows infiltrations: InfiltrationPatch,
        rows mechanical_ventilations: MechanicalVentilationPatch,
        rows shading_surfaces: ShadingSurfacePatch,
        rows space_lists: SpaceListPatch,
        rows thermal_enclosures: ThermalEnclosurePatch,
        rows adjacency_pairs: AdjacencyPairPatch,
        opt airflow_network: crate::model::AirflowNetworkDefinition,
        rows electrical_load_centers: ElectricalLoadCenterPatch,
        rows pv_systems: PvSystemAssignmentPatch,
        rows battery_storage: BatteryAssignmentPatch,
        rows shw_systems: ShwSystemConfigPatch,
        rows solar_thermal_systems: SolarThermalConfigPatch,
        rows refrigeration_systems: RefrigerationConfigPatch,
        rows water_systems: WaterSystemConfigPatch,
        rows faults: FaultDefinitionPatch,
        rows output_variables: OutputVariableSpecPatch,
        rows sizing_objects: SizingObjectPatch,
        rows daylight_zones: DaylightZoneConfigPatch,
        rows room_air_models: RoomAirModelAssignmentPatch,
        with ground_temperature: GroundTemperatureConfigPatch,
        with run_period: RunPeriodPatch,
        with schedules: ScheduleSetPatch,
    }
}
//#endregion 🔖️Patches
