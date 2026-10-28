//! 💯️ Energy model mutation domain `ideal-loads`: `change-ideal-loads-system-max-heating-capacity`, `change-ideal-loads-system-zone`, `change-ideal-loads-system-outdoor-air-per-area`, `change-ideal-loads-system-max-heating-supply-air-temp`, `change-ideal-loads-system-min-cooling-supply-air-temp`, `change-ideal-loads-system-max-cooling-capacity`, `change-ideal-loads-system-outdoor-air-per-person`, `create-ideal-loads-system`, `delete-ideal-loads-system`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod change_ideal_loads_system_max_heating_capacity {
    #[path = "⛽️change-max-heating-capacity/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⛽️change-max-heating-capacity/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "⛽️change-max-heating-capacity/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⛽️change-max-heating-capacity/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_ideal_loads_system_zone {
    #[path = "🏢️change-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏢️change-zone/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🏢️change-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏢️change-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_ideal_loads_system_outdoor_air_per_area {
    #[path = "🔳️change-outdoor-air-per-area/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔳️change-outdoor-air-per-area/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔳️change-outdoor-air-per-area/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔳️change-outdoor-air-per-area/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_ideal_loads_system_max_heating_supply_air_temp {
    #[path = "🔴️change-max-heating-supply-air-temp/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔴️change-max-heating-supply-air-temp/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔴️change-max-heating-supply-air-temp/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔴️change-max-heating-supply-air-temp/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_ideal_loads_system_min_cooling_supply_air_temp {
    #[path = "🔵️change-min-cooling-supply-air-temp/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔵️change-min-cooling-supply-air-temp/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔵️change-min-cooling-supply-air-temp/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔵️change-min-cooling-supply-air-temp/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_ideal_loads_system_max_cooling_capacity {
    #[path = "🟧️change-max-cooling-capacity/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🟧️change-max-cooling-capacity/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🟧️change-max-cooling-capacity/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🟧️change-max-cooling-capacity/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_ideal_loads_system_outdoor_air_per_person {
    #[path = "🧍️change-outdoor-air-per-person/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧍️change-outdoor-air-per-person/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🧍️change-outdoor-air-per-person/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧍️change-outdoor-air-per-person/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_ideal_loads_system {
    #[path = "🫁️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🫁️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🫁️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🫁️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_ideal_loads_system {
    #[path = "🫥️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🫥️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🫥️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🫥️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_ideal_loads_system_max_heating_capacity::{change_ideal_loads_system_max_heating_capacity, ChangeIdealLoadsSystemMaxHeatingCapacity};
pub use change_ideal_loads_system_zone::{change_ideal_loads_system_zone, ChangeIdealLoadsSystemZone};
pub use change_ideal_loads_system_outdoor_air_per_area::{change_ideal_loads_system_outdoor_air_per_area, ChangeIdealLoadsSystemOutdoorAirPerArea};
pub use change_ideal_loads_system_max_heating_supply_air_temp::{change_ideal_loads_system_max_heating_supply_air_temp, ChangeIdealLoadsSystemMaxHeatingSupplyAirTemp};
pub use change_ideal_loads_system_min_cooling_supply_air_temp::{change_ideal_loads_system_min_cooling_supply_air_temp, ChangeIdealLoadsSystemMinCoolingSupplyAirTemp};
pub use change_ideal_loads_system_max_cooling_capacity::{change_ideal_loads_system_max_cooling_capacity, ChangeIdealLoadsSystemMaxCoolingCapacity};
pub use change_ideal_loads_system_outdoor_air_per_person::{change_ideal_loads_system_outdoor_air_per_person, ChangeIdealLoadsSystemOutdoorAirPerPerson};
pub use create_ideal_loads_system::{create_ideal_loads_system, CreateIdealLoadsSystem};
pub use delete_ideal_loads_system::{delete_ideal_loads_system, DeleteIdealLoadsSystem};
