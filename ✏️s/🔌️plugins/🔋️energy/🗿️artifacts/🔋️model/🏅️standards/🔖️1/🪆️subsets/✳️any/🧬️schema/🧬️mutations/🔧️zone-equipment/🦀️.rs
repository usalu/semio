//! 🔧️ Energy model mutation domain `zone-equipment`: `change-zone-equipment-cooling-capacity`, `change-zone-equipment-priority`, `change-zone-equipment-zone`, `change-zone-equipment-type`, `delete-zone-equipment`, `create-zone-equipment`, `change-zone-equipment-heating-capacity`.

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
pub mod change_zone_equipment_cooling_capacity {
    #[path = "🍧️change-cooling-capacity/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🍧️change-cooling-capacity/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🍧️change-cooling-capacity/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🍧️change-cooling-capacity/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_zone_equipment_priority {
    #[path = "🎗️change-priority/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🎗️change-priority/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🎗️change-priority/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🎗️change-priority/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_zone_equipment_zone {
    #[path = "🏬️change-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏬️change-zone/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🏬️change-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏬️change-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_zone_equipment_type {
    #[path = "🔧️change-type/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔧️change-type/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔧️change-type/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔧️change-type/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_zone_equipment {
    #[path = "🗑️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🗑️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🗑️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🗑️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_zone_equipment {
    #[path = "🛠️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🛠️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🛠️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🛠️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_zone_equipment_heating_capacity {
    #[path = "🧇️change-heating-capacity/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧇️change-heating-capacity/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🧇️change-heating-capacity/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧇️change-heating-capacity/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_zone_equipment_cooling_capacity::{change_zone_equipment_cooling_capacity, ChangeZoneEquipmentCoolingCapacity};
pub use change_zone_equipment_priority::{change_zone_equipment_priority, ChangeZoneEquipmentPriority};
pub use change_zone_equipment_zone::{change_zone_equipment_zone, ChangeZoneEquipmentZone};
pub use change_zone_equipment_type::{change_zone_equipment_type, ChangeZoneEquipmentType};
pub use delete_zone_equipment::{delete_zone_equipment, DeleteZoneEquipment};
pub use create_zone_equipment::{create_zone_equipment, CreateZoneEquipment};
pub use change_zone_equipment_heating_capacity::{change_zone_equipment_heating_capacity, ChangeZoneEquipmentHeatingCapacity};
