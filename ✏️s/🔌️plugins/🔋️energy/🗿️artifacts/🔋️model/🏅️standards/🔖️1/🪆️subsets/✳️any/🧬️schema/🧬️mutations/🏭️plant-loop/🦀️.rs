//! 🏭️ Energy model mutation domain `plant-loop`: `change-plant-loop-supply-temperature`, `change-plant-loop-type`, `create-plant-loop`, `remove-plant-loop-equipment`, `delete-plant-loop`, `rename-plant-loop`, `add-plant-loop-equipment`, `change-plant-loop-design-flow`, `change-plant-loop-return-temperature`.

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
pub mod change_plant_loop_supply_temperature {
    #[path = "☕️change-supply-temperature/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "☕️change-supply-temperature/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "☕️change-supply-temperature/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_plant_loop_type {
    #[path = "♻️change-type/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "♻️change-type/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "♻️change-type/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_plant_loop {
    #[path = "⚗️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⚗️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⚗️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod remove_plant_loop_equipment {
    #[path = "⚙️remove-equipment/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⚙️remove-equipment/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⚙️remove-equipment/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_plant_loop {
    #[path = "💣️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "💣️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "💣️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod rename_plant_loop {
    #[path = "📛️rename/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "📛️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📛️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod add_plant_loop_equipment {
    #[path = "🔩️add-equipment/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔩️add-equipment/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔩️add-equipment/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_plant_loop_design_flow {
    #[path = "🚤️change-design-flow/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🚤️change-design-flow/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🚤️change-design-flow/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_plant_loop_return_temperature {
    #[path = "🫖️change-return-temperature/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🫖️change-return-temperature/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🫖️change-return-temperature/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_plant_loop_supply_temperature::{change_plant_loop_supply_temperature, ChangePlantLoopSupplyTemperature};
pub use change_plant_loop_type::{change_plant_loop_type, ChangePlantLoopType};
pub use create_plant_loop::{create_plant_loop, CreatePlantLoop};
pub use remove_plant_loop_equipment::{remove_plant_loop_equipment, RemovePlantLoopEquipment};
pub use delete_plant_loop::{delete_plant_loop, DeletePlantLoop};
pub use rename_plant_loop::{rename_plant_loop, RenamePlantLoop};
pub use add_plant_loop_equipment::{add_plant_loop_equipment, AddPlantLoopEquipment};
pub use change_plant_loop_design_flow::{change_plant_loop_design_flow, ChangePlantLoopDesignFlow};
pub use change_plant_loop_return_temperature::{change_plant_loop_return_temperature, ChangePlantLoopReturnTemperature};
