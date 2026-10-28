//! 🔥️ Energy model mutation domain `solar-thermal`: `change-solar-thermal-system-azimuth`, `create-solar-thermal-system`, `delete-solar-thermal-system`, `change-solar-thermal-system-efficiency`, `change-solar-thermal-system-tilt`, `change-solar-thermal-system-collector-area`, `change-solar-thermal-system-storage-volume`.

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
pub mod change_solar_thermal_system_azimuth {
    #[path = "⛵️change-azimuth/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⛵️change-azimuth/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "⛵️change-azimuth/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⛵️change-azimuth/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_solar_thermal_system {
    #[path = "🌄️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌄️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌄️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌄️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_solar_thermal_system {
    #[path = "🌆️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌆️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌆️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌆️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_solar_thermal_system_efficiency {
    #[path = "🏅️change-efficiency/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏅️change-efficiency/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🏅️change-efficiency/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏅️change-efficiency/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_solar_thermal_system_tilt {
    #[path = "🔼️change-tilt/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔼️change-tilt/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔼️change-tilt/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔼️change-tilt/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_solar_thermal_system_collector_area {
    #[path = "🟩️change-collector-area/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🟩️change-collector-area/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🟩️change-collector-area/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🟩️change-collector-area/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_solar_thermal_system_storage_volume {
    #[path = "🧃️change-storage-volume/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧃️change-storage-volume/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🧃️change-storage-volume/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧃️change-storage-volume/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_solar_thermal_system_azimuth::{change_solar_thermal_system_azimuth, ChangeSolarThermalSystemAzimuth};
pub use create_solar_thermal_system::{create_solar_thermal_system, CreateSolarThermalSystem};
pub use delete_solar_thermal_system::{delete_solar_thermal_system, DeleteSolarThermalSystem};
pub use change_solar_thermal_system_efficiency::{change_solar_thermal_system_efficiency, ChangeSolarThermalSystemEfficiency};
pub use change_solar_thermal_system_tilt::{change_solar_thermal_system_tilt, ChangeSolarThermalSystemTilt};
pub use change_solar_thermal_system_collector_area::{change_solar_thermal_system_collector_area, ChangeSolarThermalSystemCollectorArea};
pub use change_solar_thermal_system_storage_volume::{change_solar_thermal_system_storage_volume, ChangeSolarThermalSystemStorageVolume};
