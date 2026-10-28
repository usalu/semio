//! 💧️ Energy model mutation domain `humidistat`: `change-humidistat-humidifying-setpoint-schedule`, `create-humidistat`, `change-humidistat-humidifying-throttle-range`, `change-humidistat-zone`, `delete-humidistat`, `change-humidistat-dehumidifying-setpoint-schedule`, `change-humidistat-dehumidifying-throttle-range`.

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
pub mod change_humidistat_humidifying_setpoint_schedule {
    #[path = "☔️change-humidifying-setpoint-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "☔️change-humidifying-setpoint-schedule/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "☔️change-humidifying-setpoint-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "☔️change-humidifying-setpoint-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_humidistat {
    #[path = "🌂️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌂️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌂️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌂️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_humidistat_humidifying_throttle_range {
    #[path = "🌧️change-humidifying-throttle-range/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌧️change-humidifying-throttle-range/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌧️change-humidifying-throttle-range/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌧️change-humidifying-throttle-range/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_humidistat_zone {
    #[path = "🏙️change-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏙️change-zone/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🏙️change-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏙️change-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_humidistat {
    #[path = "🏜️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏜️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🏜️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏜️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_humidistat_dehumidifying_setpoint_schedule {
    #[path = "🏝️change-dehumidifying-setpoint-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏝️change-dehumidifying-setpoint-schedule/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🏝️change-dehumidifying-setpoint-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏝️change-dehumidifying-setpoint-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_humidistat_dehumidifying_throttle_range {
    #[path = "🧻️change-dehumidifying-throttle-range/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧻️change-dehumidifying-throttle-range/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🧻️change-dehumidifying-throttle-range/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧻️change-dehumidifying-throttle-range/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_humidistat_humidifying_setpoint_schedule::{change_humidistat_humidifying_setpoint_schedule, ChangeHumidistatHumidifyingSetpointSchedule};
pub use create_humidistat::{create_humidistat, CreateHumidistat};
pub use change_humidistat_humidifying_throttle_range::{change_humidistat_humidifying_throttle_range, ChangeHumidistatHumidifyingThrottleRange};
pub use change_humidistat_zone::{change_humidistat_zone, ChangeHumidistatZone};
pub use delete_humidistat::{delete_humidistat, DeleteHumidistat};
pub use change_humidistat_dehumidifying_setpoint_schedule::{change_humidistat_dehumidifying_setpoint_schedule, ChangeHumidistatDehumidifyingSetpointSchedule};
pub use change_humidistat_dehumidifying_throttle_range::{change_humidistat_dehumidifying_throttle_range, ChangeHumidistatDehumidifyingThrottleRange};
