//! 🔌️ Energy model mutation domain `load-center`: `add-electrical-load-center-pv`, `remove-electrical-load-center-pv`, `create-electrical-load-center`, `add-electrical-load-center-battery`, `delete-electrical-load-center`, `rename-electrical-load-center`, `remove-electrical-load-center-battery`.

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
pub mod add_electrical_load_center_pv {
    #[path = "☄️add-pv/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "☄️add-pv/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "☄️add-pv/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod remove_electrical_load_center_pv {
    #[path = "🌘️remove-pv/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌘️remove-pv/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌘️remove-pv/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_electrical_load_center {
    #[path = "🏦️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏦️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏦️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod add_electrical_load_center_battery {
    #[path = "🔋️add-battery/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔋️add-battery/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔋️add-battery/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_electrical_load_center {
    #[path = "🔻️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔻️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔻️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod rename_electrical_load_center {
    #[path = "🖊️rename/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🖊️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🖊️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod remove_electrical_load_center_battery {
    #[path = "🪝️remove-battery/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪝️remove-battery/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪝️remove-battery/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use add_electrical_load_center_pv::{add_electrical_load_center_pv, AddElectricalLoadCenterPv};
pub use remove_electrical_load_center_pv::{remove_electrical_load_center_pv, RemoveElectricalLoadCenterPv};
pub use create_electrical_load_center::{create_electrical_load_center, CreateElectricalLoadCenter};
pub use add_electrical_load_center_battery::{add_electrical_load_center_battery, AddElectricalLoadCenterBattery};
pub use delete_electrical_load_center::{delete_electrical_load_center, DeleteElectricalLoadCenter};
pub use rename_electrical_load_center::{rename_electrical_load_center, RenameElectricalLoadCenter};
pub use remove_electrical_load_center_battery::{remove_electrical_load_center_battery, RemoveElectricalLoadCenterBattery};
