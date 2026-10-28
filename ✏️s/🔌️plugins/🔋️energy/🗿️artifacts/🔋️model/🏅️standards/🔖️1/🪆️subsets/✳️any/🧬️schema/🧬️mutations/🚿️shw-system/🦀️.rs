//! 🚿️ Energy model mutation domain `shw-system`: `change-shw-system-heater-capacity`, `change-shw-system-setpoint`, `change-shw-system-schedule`, `delete-shw-system`, `create-shw-system`, `change-shw-system-storage-volume`.

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
pub mod change_shw_system_heater_capacity {
    #[path = "🍵️change-heater-capacity/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🍵️change-heater-capacity/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🍵️change-heater-capacity/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🍵️change-heater-capacity/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_shw_system_setpoint {
    #[path = "🏹️change-setpoint/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏹️change-setpoint/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🏹️change-setpoint/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏹️change-setpoint/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_shw_system_schedule {
    #[path = "🕐️change-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🕐️change-schedule/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🕐️change-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🕐️change-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_shw_system {
    #[path = "🚱️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🚱️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🚱️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🚱️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_shw_system {
    #[path = "🛀️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🛀️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🛀️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🛀️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_shw_system_storage_volume {
    #[path = "🛢️change-storage-volume/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🛢️change-storage-volume/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🛢️change-storage-volume/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🛢️change-storage-volume/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_shw_system_heater_capacity::{change_shw_system_heater_capacity, ChangeShwSystemHeaterCapacity};
pub use change_shw_system_setpoint::{change_shw_system_setpoint, ChangeShwSystemSetpoint};
pub use change_shw_system_schedule::{change_shw_system_schedule, ChangeShwSystemSchedule};
pub use delete_shw_system::{delete_shw_system, DeleteShwSystem};
pub use create_shw_system::{create_shw_system, CreateShwSystem};
pub use change_shw_system_storage_volume::{change_shw_system_storage_volume, ChangeShwSystemStorageVolume};
