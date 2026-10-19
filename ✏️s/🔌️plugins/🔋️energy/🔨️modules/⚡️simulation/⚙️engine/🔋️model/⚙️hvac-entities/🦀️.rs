//! ⚙️ The thermostat, ideal loads, setpoint, plant, airflow, electrical, fault, output, sizing and ground temperature entities.

use super::*;

// #region 🔖️Hvac
/// 🌡️ Thermostat setpoint control.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Thermostat {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub heating_setpoint_schedule_id: ScheduleId,
    pub cooling_setpoint_schedule_id: ScheduleId,
    pub heating_throttle_range_k: f64,
    pub cooling_throttle_range_k: f64,
}

/// ❄️ Ideal loads air system for a zone.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct IdealLoadsSystem {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub max_heating_supply_air_temp_c: f64,
    pub min_cooling_supply_air_temp_c: f64,
    pub max_heating_capacity_w: Option<f64>,
    pub max_cooling_capacity_w: Option<f64>,
    pub outdoor_air_per_person_m3_s: f64,
    pub outdoor_air_per_area_m3_s_m2: f64,
}

/// 💧️ Humidistat control for a zone.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct Humidistat {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub humidifying_setpoint_schedule_id: ScheduleId,
    pub dehumidifying_setpoint_schedule_id: ScheduleId,
    pub humidifying_throttle_range: f64,
    pub dehumidifying_throttle_range: f64,
}

/// 🎛️ Setpoint manager type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum SetpointManagerKind {
    Scheduled,
    OutdoorAirReset { low_outdoor_c: f64, high_outdoor_c: f64, low_setpoint_c: f64, high_setpoint_c: f64 },
    WarmestZone,
    ColdestZone,
}

/// 🎛️ Setpoint manager for air/plant loops.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct SetpointManager {
    pub id: EntityId,
    pub name: String,
    pub kind: SetpointManagerKind,
    pub schedule_id: Option<ScheduleId>,
}

/// 🏠️ Zone equipment assignment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ZoneEquipmentAssignment {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub equipment_type: ZoneEquipmentType,
    pub priority: u8,
    pub heating_capacity_w: f64,
    pub cooling_capacity_w: f64,
}

/// 🏠️ Zone equipment catalog reference.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum ZoneEquipmentType {
    Baseboard,
    Radiant,
    FanCoil,
    Ptac,
    VrfTerminal,
    Erv,
    UnitHeater,
    WaterToAirHp,
}

/// 🌀️ Air loop configuration reference in model.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ModelAirLoop {
    pub id: EntityId,
    pub name: String,
    pub supply_node_id: u32,
    pub return_node_id: u32,
    pub design_supply_air_flow_m3_s: f64,
    pub terminal_zone_ids: Vec<EntityId>,
}

/// 🏭️ Plant loop configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct PlantLoopConfig {
    pub id: EntityId,
    pub name: String,
    pub loop_type: PlantLoopType,
    pub supply_temperature_c: f64,
    pub return_temperature_c: f64,
    pub design_flow_kg_s: f64,
    pub equipment_ids: Vec<EntityId>,
}

/// 🏭️ Plant loop fluid type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum PlantLoopType {
    Heating,
    Cooling,
    Condenser,
}

/// 🌬️ Outdoor air system.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct OutdoorAirSystem {
    pub id: EntityId,
    pub air_loop_id: EntityId,
    pub min_oa_flow_m3_s: f64,
    pub economizer_enabled: bool,
}

/// 🌳️ Shading surface for solar obstruction.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ShadingSurface {
    pub id: EntityId,
    pub name: String,
    pub vertices_m: Vec<[f64; 3]>,
    pub transmittance_schedule_id: Option<ScheduleId>,
}

/// 📋️ Space list grouping.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct SpaceList {
    pub id: EntityId,
    pub name: String,
    pub space_ids: Vec<EntityId>,
}

/// 🏠️ Thermal enclosure grouping zones.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ThermalEnclosure {
    pub id: EntityId,
    pub name: String,
    pub zone_ids: Vec<EntityId>,
}

/// 🔗️ Surface adjacency pair.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct AdjacencyPair {
    pub surface_a_id: EntityId,
    pub surface_b_id: EntityId,
}

/// 💨️ Mechanical ventilation specification.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct MechanicalVentilation {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub schedule_id: ScheduleId,
    pub design_flow_m3_s: f64,
    pub fan_total_efficiency: f64,
    pub fan_delta_pressure_pa: f64,
}

/// 🌐️ Airflow network definition in model.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct AirflowNetworkDefinition {
    pub zone_node_ids: Vec<(EntityId, u32)>,
    pub outdoor_node_id: u32,
    pub link_ids: Vec<u32>,
}

/// ⚡️ Electrical load center.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ElectricalLoadCenter {
    pub id: EntityId,
    pub name: String,
    pub generator_ids: Vec<EntityId>,
    pub pv_ids: Vec<EntityId>,
    pub battery_ids: Vec<EntityId>,
}

/// 🏠️ Mounting-dependent module-to-cell temperature gradients from the Sandia model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum PhotovoltaicMounting {
    OpenRack,
    CloseRoof,
    InsulatedBack,
}

/// ☀️ PV system assignment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct PvSystemAssignment {
    pub id: EntityId,
    pub dc_capacity_w: f64,
    pub area_m2: f64,
    pub tilt_deg: f64,
    pub azimuth_deg: f64,
    pub module_efficiency: f64,
    pub inverter_efficiency: f64,
    pub temperature_coefficient: f64,
    pub faiman_heat_loss_constant_w_m2k: f64,
    pub faiman_heat_loss_wind_w_s_m3k: f64,
    pub mounting: PhotovoltaicMounting,
}

impl PvSystemAssignment {
    /// 🧊️ Finite authored module parameters admit a strictly positive heat-loss denominator.
    pub fn thermal_parameters_valid(&self) -> bool {
        self.temperature_coefficient.is_finite() && self.faiman_heat_loss_constant_w_m2k.is_finite() && self.faiman_heat_loss_constant_w_m2k > 0.0 && self.faiman_heat_loss_wind_w_s_m3k.is_finite() && self.faiman_heat_loss_wind_w_s_m3k >= 0.0
    }
}

/// 🔋️ Battery storage assignment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct BatteryAssignment {
    pub id: EntityId,
    pub capacity_kwh: f64,
    pub max_charge_w: f64,
    pub max_discharge_w: f64,
    pub round_trip_efficiency: f64,
}

/// 🚿️ Service hot water system.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct ShwSystemConfig {
    pub id: EntityId,
    pub heater_capacity_w: f64,
    pub storage_volume_m3: f64,
    pub setpoint_c: f64,
    pub schedule_id: ScheduleId,
}

/// ☀️ Solar thermal collector system.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct SolarThermalConfig {
    pub id: EntityId,
    pub collector_area_m2: f64,
    pub efficiency: f64,
    pub storage_volume_m3: f64,
    pub tilt_deg: f64,
    pub azimuth_deg: f64,
}

/// ❄️ Refrigeration system.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct RefrigerationConfig {
    pub id: EntityId,
    pub case_count: u32,
    pub design_load_w: f64,
    pub defrost_schedule_id: ScheduleId,
}

/// 💧️ Water use system.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct WaterSystemConfig {
    pub id: EntityId,
    pub fixture_count: u32,
    pub peak_flow_l_s: f64,
    pub schedule_id: ScheduleId,
}

/// ⚠️ Fault definition.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct FaultDefinition {
    pub id: EntityId,
    pub target_equipment_id: EntityId,
    pub fault_type: FaultType,
    pub severity: f64,
    pub start_schedule_id: ScheduleId,
}

/// ⚠️ Fault type catalog.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum FaultType {
    SensorBias,
    CoilFouling,
    DamperStuck,
    ChillerFouling,
    BoilerEfficiencyDegradation,
}

/// 📊️ Output variable registration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct OutputVariableSpec {
    pub name: String,
    pub key: String,
    pub reporting_frequency: OutputReportFrequency,
}

/// 📊️ Output reporting frequency.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum OutputReportFrequency {
    Timestep,
    Hourly,
    Daily,
    Monthly,
    RunPeriod,
}

/// 📐️ Sizing object for design-day autosize.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct SizingObject {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub sizing_type: SizingType,
    pub design_day_type: DesignDayType,
}

/// 📐️ Sizing type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum SizingType {
    Heating,
    Cooling,
    OutdoorAir,
}

/// 📐️ Design day type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum DesignDayType {
    Heating,
    Cooling,
}

/// 💡️ Daylight zone configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct DaylightZoneConfig {
    pub id: EntityId,
    pub zone_id: EntityId,
    pub illuminance_target_lux: f64,
    pub glare_limit: f64,
    pub window_transmittance: f64,
}

/// 🌡️ Room air model selection per zone.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct RoomAirModelAssignment {
    pub zone_id: EntityId,
    pub model: RoomAirModelType,
}

/// 🌡️ Room air model type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value_derive::NativeReceive)]
#[canonical_json(owner = semio_framework_pack_json)]
pub enum RoomAirModelType {
    WellMixed,
    OneNodeDisplacement,
    TwoNodeBuoyancy,
    UnderFloorAirDistribution,
}

/// 🌡️ Ground temperature configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValueDerive, FromValueDerive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
pub struct GroundTemperatureConfig {
    pub building_surface_c: [f64; 12],
    pub shallow_c: [f64; 12],
    pub deep_c: f64,
}

impl Default for GroundTemperatureConfig {
    fn default() -> Self {
        Self { building_surface_c: [18.0; 12], shallow_c: [18.0; 12], deep_c: 18.0 }
    }
}
// #endregion 🔖️Hvac
