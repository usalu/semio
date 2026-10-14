//! 🧪️ The `zone-equipment` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_zone_equipment as zone_equipment_mutations;

#[path = "."]
mod change_zone_equipment_cooling_capacity {
    pub use crate::zone_equipment_mutations::change_zone_equipment_cooling_capacity::*;
    #[path = "../../🔧️zone-equipment/🍧️change-cooling-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔧️zone-equipment/🍧️change-cooling-capacity/🧪️tests/✅️uprates/🦀️.rs"]
    mod tests_uprates;
}

#[path = "."]
mod change_zone_equipment_priority {
    pub use crate::zone_equipment_mutations::change_zone_equipment_priority::*;
    #[path = "../../🔧️zone-equipment/🎗️change-priority/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔧️zone-equipment/🎗️change-priority/🧪️tests/✅️demotes-it/🦀️.rs"]
    mod tests_demotes_it;
}

#[path = "."]
mod change_zone_equipment_zone {
    pub use crate::zone_equipment_mutations::change_zone_equipment_zone::*;
    #[path = "../../🔧️zone-equipment/🏬️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔧️zone-equipment/🏬️change-zone/🧪️tests/✅️moves/🦀️.rs"]
    mod tests_moves;
}

#[path = "."]
mod change_zone_equipment_type {
    pub use crate::zone_equipment_mutations::change_zone_equipment_type::*;
    #[path = "../../🔧️zone-equipment/🔧️change-type/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔧️zone-equipment/🔧️change-type/🧪️tests/✅️swaps/🦀️.rs"]
    mod tests_swaps;
}

#[path = "."]
mod delete_zone_equipment {
    pub use crate::zone_equipment_mutations::delete_zone_equipment::*;
    #[path = "../../🔧️zone-equipment/🗑️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔧️zone-equipment/🗑️delete/🧪️tests/✅️drops/🦀️.rs"]
    mod tests_drops;
}

#[path = "."]
mod create_zone_equipment {
    pub use crate::zone_equipment_mutations::create_zone_equipment::*;
    #[path = "../../🔧️zone-equipment/🛠️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔧️zone-equipment/🛠️create/🧪️tests/✅️adds/🦀️.rs"]
    mod tests_adds;
}

#[path = "."]
mod change_zone_equipment_heating_capacity {
    pub use crate::zone_equipment_mutations::change_zone_equipment_heating_capacity::*;
    #[path = "../../🔧️zone-equipment/🧇️change-heating-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔧️zone-equipment/🧇️change-heating-capacity/🧪️tests/✅️uprates/🦀️.rs"]
    mod tests_uprates;
}
