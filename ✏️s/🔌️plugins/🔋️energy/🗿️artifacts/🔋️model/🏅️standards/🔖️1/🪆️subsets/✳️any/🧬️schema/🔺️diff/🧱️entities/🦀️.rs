//! 🧱️ The energy model's entity patches — one sparse patch per row type of [`crate::model::Model`], one per record, and the
//! whole-model patch that names them all. A row patch addresses its row by id and carries only the fields that change:
//! `Set` assigns a scalar, `Opt` sets or clears an optional field, `List` edits an ordered id list, `Arr` assigns
//! entries of a fixed array, `Coll`/`Rec` nest a collection or a record.

use super::patch::{patch, record_patch, row_patch, Arr, Coll, Field, FieldPatch, List, Opt, Rec, Row, RowPatch, Set, Unchanged};
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
    fn commit_onto(&self, target: &mut Self::Target) -> Result<(), MutationApplyError> {
        self.reporting_frequency.commit_onto(&mut target.reporting_frequency)
    }
    fn absorb(&mut self, later: Self) {
        self.reporting_frequency.absorb(later.reporting_frequency);
    }
    fn inverse(&self, base: &Self::Target) -> Self {
        Self { name: self.name.clone(), key: self.key.clone(), reporting_frequency: self.reporting_frequency.inverse(&base.reporting_frequency) }
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
    fn commit_onto(&self, _target: &mut Self::Target) -> Result<(), MutationApplyError> {
        Ok(())
    }
    fn absorb(&mut self, _later: Self) {}
    fn inverse(&self, _base: &Self::Target) -> Self {
        self.clone()
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
        Set name: String,
        Set volume_m3: f64,
        Set multiplier: u32,
        Set conditioned: bool,
        Set part_of_total_floor_area: bool,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`Space`] row.
    SpacePatch for crate::model::Space {
        key id: crate::model::EntityId;
        Set name: String,
        Set zone_id: crate::model::EntityId,
        Set floor_area_m2: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`Surface`] row.
    SurfacePatch for crate::model::Surface {
        key id: crate::model::EntityId;
        Set name: String,
        Set zone_id: crate::model::EntityId,
        Set class: crate::model::SurfaceClass,
        Set vertices_m: Vec<[f64; 3]>,
        Set construction_id: crate::model::EntityId,
        Set outside_boundary_condition: crate::model::OutsideBoundary,
        Set sun_exposed: bool,
        Set wind_exposed: bool,
        Set multiplier: u32,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`Fenestration`] row.
    FenestrationPatch for crate::model::Fenestration {
        key id: crate::model::EntityId;
        Set name: String,
        Set surface_id: crate::model::EntityId,
        Set u_value_w_m2k: f64,
        Set shgc: f64,
        Set vlt: f64,
        Set area_m2: f64,
        Set height_m: f64,
        Set sill_height_m: f64,
        Set frame_conductance_w_k: f64,
        Set divider_conductance_w_k: f64,
        Set overhang_depth_m: f64,
        Set overhang_offset_m: f64,
        Set fin_depth_m: f64,
        Set fin_offset_m: f64,
        Opt glazing_construction_id: crate::model::EntityId,
        Set vertices_m: Vec<[f64; 3]>,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`Material`] row.
    MaterialPatch for crate::model::Material {
        key id: crate::model::EntityId;
        Set name: String,
        Set roughness: crate::model::SurfaceRoughness,
        Set thickness_m: f64,
        Set conductivity_w_m_k: f64,
        Set density_kg_m3: f64,
        Set specific_heat_j_kg_k: f64,
        Set thermal_absorptance: f64,
        Set solar_absorptance: f64,
        Set visible_absorptance: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`GlazingMaterial`] row.
    GlazingMaterialPatch for crate::model::GlazingMaterial {
        key id: crate::model::EntityId;
        Set name: String,
        Set thickness_m: f64,
        Set conductivity_w_m_k: f64,
        Set solar_transmittance: f64,
        Set solar_reflectance_front: f64,
        Set solar_reflectance_back: f64,
        Set visible_transmittance: f64,
        Set visible_reflectance_front: f64,
        Set visible_reflectance_back: f64,
        Set infrared_transmittance: f64,
        Set infrared_emissivity_front: f64,
        Set infrared_emissivity_back: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`GasMaterial`] row.
    GasMaterialPatch for crate::model::GasMaterial {
        key id: crate::model::EntityId;
        Set name: String,
        Set thickness_m: f64,
        Set gas: crate::model::GasKind,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`Construction`] row.
    ConstructionPatch for crate::model::Construction {
        key id: crate::model::EntityId;
        Set name: String,
        List layer_material_ids: crate::model::EntityId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`PeopleGain`] row.
    PeopleGainPatch for crate::model::PeopleGain {
        key id: crate::model::EntityId;
        Set zone_id: crate::model::EntityId,
        Set schedule_id: crate::model::ScheduleId,
        Set activity_schedule_id: crate::model::ScheduleId,
        Set people_per_area: f64,
        Set sensible_fraction: f64,
        Set latent_fraction: f64,
        Set radiant_fraction: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`LightingGain`] row.
    LightingGainPatch for crate::model::LightingGain {
        key id: crate::model::EntityId;
        Set zone_id: crate::model::EntityId,
        Set schedule_id: crate::model::ScheduleId,
        Set watts_per_area: f64,
        Set radiant_fraction: f64,
        Set visible_fraction: f64,
        Set return_air_fraction: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`EquipmentGain`] row.
    EquipmentGainPatch for crate::model::EquipmentGain {
        key id: crate::model::EntityId;
        Set zone_id: crate::model::EntityId,
        Set schedule_id: crate::model::ScheduleId,
        Set watts_per_area: f64,
        Set radiant_fraction: f64,
        Set latent_fraction: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`Thermostat`] row.
    ThermostatPatch for crate::model::Thermostat {
        key id: crate::model::EntityId;
        Set zone_id: crate::model::EntityId,
        Set heating_setpoint_schedule_id: crate::model::ScheduleId,
        Set cooling_setpoint_schedule_id: crate::model::ScheduleId,
        Set heating_throttle_range_k: f64,
        Set cooling_throttle_range_k: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`Humidistat`] row.
    HumidistatPatch for crate::model::Humidistat {
        key id: crate::model::EntityId;
        Set zone_id: crate::model::EntityId,
        Set humidifying_setpoint_schedule_id: crate::model::ScheduleId,
        Set dehumidifying_setpoint_schedule_id: crate::model::ScheduleId,
        Set humidifying_throttle_range: f64,
        Set dehumidifying_throttle_range: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`SetpointManager`] row.
    SetpointManagerPatch for crate::model::SetpointManager {
        key id: crate::model::EntityId;
        Set name: String,
        Set kind: crate::model::SetpointManagerKind,
        Opt schedule_id: crate::model::ScheduleId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`IdealLoadsSystem`] row.
    IdealLoadsSystemPatch for crate::model::IdealLoadsSystem {
        key id: crate::model::EntityId;
        Set zone_id: crate::model::EntityId,
        Set max_heating_supply_air_temp_c: f64,
        Set min_cooling_supply_air_temp_c: f64,
        Opt max_heating_capacity_w: f64,
        Opt max_cooling_capacity_w: f64,
        Set outdoor_air_per_person_m3_s: f64,
        Set outdoor_air_per_area_m3_s_m2: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`ZoneEquipmentAssignment`] row.
    ZoneEquipmentAssignmentPatch for crate::model::ZoneEquipmentAssignment {
        key id: crate::model::EntityId;
        Set zone_id: crate::model::EntityId,
        Set equipment_type: crate::model::ZoneEquipmentType,
        Set priority: u8,
        Set heating_capacity_w: f64,
        Set cooling_capacity_w: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`ModelAirLoop`] row.
    ModelAirLoopPatch for crate::model::ModelAirLoop {
        key id: crate::model::EntityId;
        Set name: String,
        Set supply_node_id: u32,
        Set return_node_id: u32,
        Set design_supply_air_flow_m3_s: f64,
        List terminal_zone_ids: crate::model::EntityId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`PlantLoopConfig`] row.
    PlantLoopConfigPatch for crate::model::PlantLoopConfig {
        key id: crate::model::EntityId;
        Set name: String,
        Set loop_type: crate::model::PlantLoopType,
        Set supply_temperature_c: f64,
        Set return_temperature_c: f64,
        Set design_flow_kg_s: f64,
        List equipment_ids: crate::model::EntityId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`OutdoorAirSystem`] row.
    OutdoorAirSystemPatch for crate::model::OutdoorAirSystem {
        key id: crate::model::EntityId;
        Set air_loop_id: crate::model::EntityId,
        Set min_oa_flow_m3_s: f64,
        Set economizer_enabled: bool,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`Infiltration`] row.
    InfiltrationPatch for crate::model::Infiltration {
        key id: crate::model::EntityId;
        Set zone_id: crate::model::EntityId,
        Set schedule_id: crate::model::ScheduleId,
        Set method: crate::air_exchange::InfiltrationMethod,
        Set design_flow_ach: f64,
        Set flow_per_exterior_area_m3_s_m2: f64,
        Set effective_leakage_area_m2: f64,
        Set discharge_coefficient: f64,
        Set stack_height_m: f64,
        Set constant_term_coefficient: f64,
        Set temperature_term_coefficient: f64,
        Set velocity_term_coefficient: f64,
        Set velocity_squared_term_coefficient: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`MechanicalVentilation`] row.
    MechanicalVentilationPatch for crate::model::MechanicalVentilation {
        key id: crate::model::EntityId;
        Set zone_id: crate::model::EntityId,
        Set schedule_id: crate::model::ScheduleId,
        Set design_flow_m3_s: f64,
        Set fan_total_efficiency: f64,
        Set fan_delta_pressure_pa: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`ShadingSurface`] row.
    ShadingSurfacePatch for crate::model::ShadingSurface {
        key id: crate::model::EntityId;
        Set name: String,
        Set vertices_m: Vec<[f64; 3]>,
        Opt transmittance_schedule_id: crate::model::ScheduleId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`SpaceList`] row.
    SpaceListPatch for crate::model::SpaceList {
        key id: crate::model::EntityId;
        Set name: String,
        List space_ids: crate::model::EntityId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`ThermalEnclosure`] row.
    ThermalEnclosurePatch for crate::model::ThermalEnclosure {
        key id: crate::model::EntityId;
        Set name: String,
        List zone_ids: crate::model::EntityId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`ElectricalLoadCenter`] row.
    ElectricalLoadCenterPatch for crate::model::ElectricalLoadCenter {
        key id: crate::model::EntityId;
        Set name: String,
        List generator_ids: crate::model::EntityId,
        List pv_ids: crate::model::EntityId,
        List battery_ids: crate::model::EntityId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`PvSystemAssignment`] row.
    PvSystemAssignmentPatch for crate::model::PvSystemAssignment {
        key id: crate::model::EntityId;
        Set dc_capacity_w: f64,
        Set area_m2: f64,
        Set tilt_deg: f64,
        Set azimuth_deg: f64,
        Set module_efficiency: f64,
        Set inverter_efficiency: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`BatteryAssignment`] row.
    BatteryAssignmentPatch for crate::model::BatteryAssignment {
        key id: crate::model::EntityId;
        Set capacity_kwh: f64,
        Set max_charge_w: f64,
        Set max_discharge_w: f64,
        Set round_trip_efficiency: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`ShwSystemConfig`] row.
    ShwSystemConfigPatch for crate::model::ShwSystemConfig {
        key id: crate::model::EntityId;
        Set heater_capacity_w: f64,
        Set storage_volume_m3: f64,
        Set setpoint_c: f64,
        Set schedule_id: crate::model::ScheduleId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`SolarThermalConfig`] row.
    SolarThermalConfigPatch for crate::model::SolarThermalConfig {
        key id: crate::model::EntityId;
        Set collector_area_m2: f64,
        Set efficiency: f64,
        Set storage_volume_m3: f64,
        Set tilt_deg: f64,
        Set azimuth_deg: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`RefrigerationConfig`] row.
    RefrigerationConfigPatch for crate::model::RefrigerationConfig {
        key id: crate::model::EntityId;
        Set case_count: u32,
        Set design_load_w: f64,
        Set defrost_schedule_id: crate::model::ScheduleId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`WaterSystemConfig`] row.
    WaterSystemConfigPatch for crate::model::WaterSystemConfig {
        key id: crate::model::EntityId;
        Set fixture_count: u32,
        Set peak_flow_l_s: f64,
        Set schedule_id: crate::model::ScheduleId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`FaultDefinition`] row.
    FaultDefinitionPatch for crate::model::FaultDefinition {
        key id: crate::model::EntityId;
        Set target_equipment_id: crate::model::EntityId,
        Set fault_type: crate::model::FaultType,
        Set severity: f64,
        Set start_schedule_id: crate::model::ScheduleId,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`SizingObject`] row.
    SizingObjectPatch for crate::model::SizingObject {
        key id: crate::model::EntityId;
        Set zone_id: crate::model::EntityId,
        Set sizing_type: crate::model::SizingType,
        Set design_day_type: crate::model::DesignDayType,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`DaylightZoneConfig`] row.
    DaylightZoneConfigPatch for crate::model::DaylightZoneConfig {
        key id: crate::model::EntityId;
        Set zone_id: crate::model::EntityId,
        Set illuminance_target_lux: f64,
        Set glare_limit: f64,
        Set window_transmittance: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`RoomAirModelAssignment`] row.
    RoomAirModelAssignmentPatch for crate::model::RoomAirModelAssignment {
        key zone_id: crate::model::EntityId;
        Set model: crate::model::RoomAirModelType,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`ConstantSchedule`] row.
    ConstantSchedulePatch for crate::schedule::ConstantSchedule {
        key id: crate::model::ScheduleId;
        Set value: f64,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`DailySchedule`] row.
    DailySchedulePatch for crate::schedule::DailySchedule {
        key id: crate::model::ScheduleId;
        Arr hourly_values: [f64; 24],
        Set interpolation: crate::schedule::ScheduleInterpolation,
        Opt limits: crate::schedule::ScheduleLimits,
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`WeeklySchedule`] row.
    WeeklySchedulePatch for crate::schedule::WeeklySchedule {
        key id: crate::model::ScheduleId;
        Arr daily_schedule_ids: [crate::model::ScheduleId; 7],
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`AnnualSchedule`] row.
    AnnualSchedulePatch for crate::schedule::AnnualSchedule {
        key id: crate::model::ScheduleId;
        List rules: crate::schedule::CompactScheduleRule,
        Set default_daily_schedule_id: crate::model::ScheduleId,
        Opt holiday_daily_schedule_id: crate::model::ScheduleId,
        List holiday_dates: (u16, u8, u8),
    }
}

row_patch! {
    /// 🩹 Sparse patch over one [`TimeSeriesSchedule`] row.
    TimeSeriesSchedulePatch for crate::schedule::TimeSeriesSchedule {
        key id: crate::model::ScheduleId;
        Set values: Vec<f64>,
        Set timestep_seconds: u32,
    }
}

record_patch! {
    /// 🩹 Sparse patch over the [`Site`] record.
    SitePatch for crate::model::Site {
        Set latitude_deg: f64,
        Set longitude_deg: f64,
        Set elevation_m: f64,
        Set time_zone_hours: f64,
        Set north_axis_deg: f64,
    }
}

record_patch! {
    /// 🩹 Sparse patch over the [`GroundTemperatureConfig`] record.
    GroundTemperatureConfigPatch for crate::model::GroundTemperatureConfig {
        Arr building_surface_c: [f64; 12],
        Arr shallow_c: [f64; 12],
        Set deep_c: f64,
    }
}

record_patch! {
    /// 🩹 Sparse patch over the [`RunPeriod`] record.
    RunPeriodPatch for crate::calendar::RunPeriod {
        Set start_month: u8,
        Set start_day: u8,
        Set end_month: u8,
        Set end_day: u8,
        Set year: u16,
    }
}

record_patch! {
    /// 🩹 Sparse patch over the [`ScheduleSet`](crate::schedule::ScheduleSet) record.
    ScheduleSetPatch for crate::schedule::ScheduleSet {
        Coll constants: ConstantSchedulePatch,
        Coll daily: DailySchedulePatch,
        Coll weekly: WeeklySchedulePatch,
        Coll annual: AnnualSchedulePatch,
        Coll time_series: TimeSeriesSchedulePatch,
    }
}

record_patch! {
    /// 🩹 Sparse patch over the whole [`Model`](crate::model::Model).
    ModelPatch for crate::model::Model {
        Set name: String,
        Set version: String,
        Rec site: SitePatch,
        Coll zones: ZonePatch,
        Coll spaces: SpacePatch,
        Coll surfaces: SurfacePatch,
        Coll fenestrations: FenestrationPatch,
        Coll materials: MaterialPatch,
        Coll glazing_materials: GlazingMaterialPatch,
        Coll gas_materials: GasMaterialPatch,
        Coll constructions: ConstructionPatch,
        Coll people: PeopleGainPatch,
        Coll lighting: LightingGainPatch,
        Coll equipment: EquipmentGainPatch,
        Coll thermostats: ThermostatPatch,
        Coll humidistats: HumidistatPatch,
        Coll setpoint_managers: SetpointManagerPatch,
        Coll ideal_loads: IdealLoadsSystemPatch,
        Coll zone_equipment: ZoneEquipmentAssignmentPatch,
        Coll air_loops: ModelAirLoopPatch,
        Coll plant_loops: PlantLoopConfigPatch,
        Coll outdoor_air_systems: OutdoorAirSystemPatch,
        Coll infiltrations: InfiltrationPatch,
        Coll mechanical_ventilations: MechanicalVentilationPatch,
        Coll shading_surfaces: ShadingSurfacePatch,
        Coll space_lists: SpaceListPatch,
        Coll thermal_enclosures: ThermalEnclosurePatch,
        Coll adjacency_pairs: AdjacencyPairPatch,
        Opt airflow_network: crate::model::AirflowNetworkDefinition,
        Coll electrical_load_centers: ElectricalLoadCenterPatch,
        Coll pv_systems: PvSystemAssignmentPatch,
        Coll battery_storage: BatteryAssignmentPatch,
        Coll shw_systems: ShwSystemConfigPatch,
        Coll solar_thermal_systems: SolarThermalConfigPatch,
        Coll refrigeration_systems: RefrigerationConfigPatch,
        Coll water_systems: WaterSystemConfigPatch,
        Coll faults: FaultDefinitionPatch,
        Coll output_variables: OutputVariableSpecPatch,
        Coll sizing_objects: SizingObjectPatch,
        Coll daylight_zones: DaylightZoneConfigPatch,
        Coll room_air_models: RoomAirModelAssignmentPatch,
        Rec ground_temperature: GroundTemperatureConfigPatch,
        Rec run_period: RunPeriodPatch,
        Rec schedules: ScheduleSetPatch,
    }
}
//#endregion 🔖️Patches
