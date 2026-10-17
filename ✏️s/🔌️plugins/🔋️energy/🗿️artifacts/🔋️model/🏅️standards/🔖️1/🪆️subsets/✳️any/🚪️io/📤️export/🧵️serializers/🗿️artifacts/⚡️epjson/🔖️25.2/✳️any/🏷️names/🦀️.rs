//! 🏷️ The epJSON release constants, the diagnostic record and the object naming functions.

use super::*;

//#region 🔖️Constants
/// ⚡️ The EnergyPlus release whose `Energy+.schema.epJSON` this codec targets.
pub const EPJSON_VERSION: &str = "25.2";
/// 🌾️ Declared convention #1 — see the module docstring.
pub const EPJSON_TERRAIN: &str = "Country";
/// ☀️ Declared convention #1 — see the module docstring.
pub const EPJSON_SOLAR_DISTRIBUTION: &str = "FullInteriorAndExterior";
/// 🌗️ Declared convention #1 — days between two `ShadowCalculation` sun-position and shading updates.
pub const SHADOW_UPDATE_FREQUENCY_DAYS: i64 = 1;
/// 📐️ Declared convention #3 — how far apart along a surface's own "up" two vertices may sit and still
/// tie for its top edge when the upper-left corner is picked [m].
pub const VERTEX_HEIGHT_TIE_M: f64 = 1e-6;
/// 💨️ EnergyPlus `(A, B, C, D)` of a constant design flow — what the engine's `ScheduledAch` and
/// `PerExteriorArea` infiltration methods are (see the module docstring).
pub const CONSTANT_DESIGN_FLOW_COEFFICIENTS: [f64; 4] = [1.0, 0.0, 0.0, 0.0];
/// 🪟️ Declared convention #2 — width/height of an aperture with no stated `height_m`.
pub const APERTURE_ASPECT: f64 = 1.5;
/// 🪟️ Declared convention #2 — sill of an aperture with no stated `sill_height_m`.
pub const APERTURE_SILL_M: f64 = 0.2;
/// 📅️ Name of the dimensionless `ScheduleTypeLimits` every emitted schedule references.
pub const SCHEDULE_LIMITS_ANY: &str = "semio Any Number";
/// 🎛️ Name of the `ScheduleTypeLimits` the thermostat control-type schedule references.
pub const SCHEDULE_LIMITS_CONTROL: &str = "semio Control Type";
/// 🎛️ Name of the constant `4` schedule that selects `ThermostatSetpoint:DualSetpoint`.
pub const DUAL_SETPOINT_CONTROL_SCHEDULE: &str = "semio Dual Setpoint Control";
/// 🧮️ Timesteps per hour written into `Timestep`; matches the ASHRAE 140 reference runs.
pub const TIMESTEPS_PER_HOUR: i64 = 6;
/// 📊️ The four zone variables `📓️bestest-contract.md` compares both producers on.
pub const CONTRACT_OUTPUT_VARIABLES: &[&str] = &["Zone Mean Air Temperature", "Zone Ideal Loads Supply Air Total Heating Energy", "Zone Ideal Loads Supply Air Total Cooling Energy", "Surface Window Transmitted Solar Radiation Energy"];
/// 🧮️ The two ideal-loads meters the contract's annual totals are read from.
pub const CONTRACT_OUTPUT_METERS: &[&str] = &["Heating:DistrictHeatingWater", "Cooling:DistrictCooling"];
//#endregion 🔖️Constants

//#region 🔖️Diagnostic
/// 🚨️ One thing the model states that this codec's epJSON subset cannot carry. Reported, never
/// dropped: `code` is a stable machine token, `subject` names the offending entity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EpJsonDiagnostic {
    pub code: String,
    pub subject: String,
    pub message: String,
}

impl EpJsonDiagnostic {
    pub fn new(code: impl Into<String>, subject: impl Into<String>, message: impl Into<String>) -> Self {
        Self { code: code.into(), subject: subject.into(), message: message.into() }
    }

    /// 🌳️ Wire form, so a caller can surface diagnostics next to the document itself.
    pub fn to_json(&self) -> Value {
        Value::Object(Object::from_iter([("code".to_string(), Value::from(self.code.as_str())), ("subject".to_string(), Value::from(self.subject.as_str())), ("message".to_string(), Value::from(self.message.as_str()))]))
    }
}
//#endregion 🔖️Diagnostic

//#region 🔖️Names
/// 🏷️ Deterministic epJSON name for a schedule, which the semio model identifies by number only.
/// The number is part of the name so the import leaf recovers the exact [`ScheduleId`].
pub fn schedule_name(id: ScheduleId) -> String {
    format!("Schedule {}", id.0)
}

/// 🏷️ Falls back to a stable id-derived name when an entity carries an empty one.
pub(super) fn entity_name(kind: &str, id: EntityId, name: &str) -> String {
    if name.trim().is_empty() {
        format!("{kind} {}", id.0)
    } else {
        name.to_string()
    }
}

/// 🪟️ Name of the single-layer glazing construction generated for one fenestration.
pub fn glazing_construction_name(window: &str) -> String {
    format!("{window} Glazing Construction")
}

/// 🪟️ Name of the `WindowMaterial:SimpleGlazingSystem` generated for one fenestration.
pub fn glazing_material_name(window: &str) -> String {
    format!("{window} Glazing")
}

/// 🌬️ Node/list/equipment names of one zone's ideal-loads plumbing.
pub fn ideal_loads_name(zone: &str) -> String {
    format!("{zone} Ideal Loads Air System")
}
pub fn equipment_list_name(zone: &str) -> String {
    format!("{zone} Equipment List")
}
pub fn inlet_node_list_name(zone: &str) -> String {
    format!("{zone} Inlet Node List")
}
pub fn supply_node_name(zone: &str) -> String {
    format!("{zone} Supply Inlet")
}
pub fn zone_air_node_name(zone: &str) -> String {
    format!("{zone} Zone Air Node")
}
pub fn return_node_name(zone: &str) -> String {
    format!("{zone} Return Outlet")
}
pub fn dual_setpoint_name(zone: &str) -> String {
    format!("{zone} Setpoints")
}
pub fn thermostat_name(zone: &str) -> String {
    format!("{zone} Thermostat")
}
//#endregion 🔖️Names
