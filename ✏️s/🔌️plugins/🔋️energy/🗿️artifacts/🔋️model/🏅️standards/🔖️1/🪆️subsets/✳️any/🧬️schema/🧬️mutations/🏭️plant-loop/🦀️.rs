//! 🏭️ Energy model mutation domain `plant-loop`: `change-plant-loop-type`, `create-plant-loop`, `remove-plant-loop-equipment`, `delete-plant-loop`, `rename-plant-loop`, `add-plant-loop-equipment`

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
pub mod change_plant_loop_type {
    #[path = "♻️change-type/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "♻️change-type/🧊️reference/🦀️.rs"]
    mod reference;
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
    #[path = "⚗️create/🧊️reference/🦀️.rs"]
    mod reference;
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
    #[path = "⚙️remove-equipment/🧊️reference/🦀️.rs"]
    mod reference;
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
    #[path = "💣️delete/🧊️reference/🦀️.rs"]
    mod reference;
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
    #[path = "📛️rename/🧊️reference/🦀️.rs"]
    mod reference;
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
    #[path = "🔩️add-equipment/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔩️add-equipment/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔩️add-equipment/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_plant_loop_type::{change_plant_loop_type, ChangePlantLoopType};
pub use create_plant_loop::{create_plant_loop, CreatePlantLoop};
pub use remove_plant_loop_equipment::{remove_plant_loop_equipment, RemovePlantLoopEquipment};
pub use delete_plant_loop::{delete_plant_loop, DeletePlantLoop};
pub use rename_plant_loop::{rename_plant_loop, RenamePlantLoop};
pub use add_plant_loop_equipment::{add_plant_loop_equipment, AddPlantLoopEquipment};
