//! 🌡️ Energy model mutation domain `thermostat`: `change-thermostat-heating-throttle-range`, `change-thermostat-cooling-throttle-range`, `change-thermostat-cooling-setpoint-schedule`, `delete-thermostat`, `change-thermostat-zone`, `change-thermostat-heating-setpoint-schedule`, `create-thermostat`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod change_thermostat_heating_throttle_range {
    #[path = "🎚️change-heating-throttle-range/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🎚️change-heating-throttle-range/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🎚️change-heating-throttle-range/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🎚️change-heating-throttle-range/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_thermostat_cooling_throttle_range {
    #[path = "🎛️change-cooling-throttle-range/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🎛️change-cooling-throttle-range/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🎛️change-cooling-throttle-range/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🎛️change-cooling-throttle-range/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_thermostat_cooling_setpoint_schedule {
    #[path = "🐧️change-cooling-setpoint-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🐧️change-cooling-setpoint-schedule/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🐧️change-cooling-setpoint-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🐧️change-cooling-setpoint-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_thermostat {
    #[path = "🛑️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🛑️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🛑️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🛑️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_thermostat_zone {
    #[path = "🛖️change-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🛖️change-zone/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🛖️change-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🛖️change-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_thermostat_heating_setpoint_schedule {
    #[path = "🥵️change-heating-setpoint-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🥵️change-heating-setpoint-schedule/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🥵️change-heating-setpoint-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🥵️change-heating-setpoint-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_thermostat {
    #[path = "🩺️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🩺️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🩺️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🩺️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_thermostat_heating_throttle_range::{change_thermostat_heating_throttle_range, ChangeThermostatHeatingThrottleRange};
pub use change_thermostat_cooling_throttle_range::{change_thermostat_cooling_throttle_range, ChangeThermostatCoolingThrottleRange};
pub use change_thermostat_cooling_setpoint_schedule::{change_thermostat_cooling_setpoint_schedule, ChangeThermostatCoolingSetpointSchedule};
pub use delete_thermostat::{delete_thermostat, DeleteThermostat};
pub use change_thermostat_zone::{change_thermostat_zone, ChangeThermostatZone};
pub use change_thermostat_heating_setpoint_schedule::{change_thermostat_heating_setpoint_schedule, ChangeThermostatHeatingSetpointSchedule};
pub use create_thermostat::{create_thermostat, CreateThermostat};
