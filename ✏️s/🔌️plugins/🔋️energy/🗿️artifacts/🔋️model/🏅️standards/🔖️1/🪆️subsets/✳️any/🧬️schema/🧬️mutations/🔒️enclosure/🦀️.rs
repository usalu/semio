//! 🔒️ Energy model mutation domain `enclosure`: `create-thermal-enclosure`, `delete-thermal-enclosure`, `add-thermal-enclosure-zone`, `remove-thermal-enclosure-zone`, `rename-thermal-enclosure`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;

#[path = "."]
pub mod create_thermal_enclosure {
    #[path = "🏟️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏟️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏟️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_thermal_enclosure {
    #[path = "🏯️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏯️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏯️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod add_thermal_enclosure_zone {
    #[path = "🔒️add-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔒️add-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔒️add-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod remove_thermal_enclosure_zone {
    #[path = "🔓️remove-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔓️remove-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔓️remove-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod rename_thermal_enclosure {
    #[path = "🖋️rename/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🖋️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🖋️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use create_thermal_enclosure::{create_thermal_enclosure, CreateThermalEnclosure};
pub use delete_thermal_enclosure::{delete_thermal_enclosure, DeleteThermalEnclosure};
pub use add_thermal_enclosure_zone::{add_thermal_enclosure_zone, AddThermalEnclosureZone};
pub use remove_thermal_enclosure_zone::{remove_thermal_enclosure_zone, RemoveThermalEnclosureZone};
pub use rename_thermal_enclosure::{rename_thermal_enclosure, RenameThermalEnclosure};
