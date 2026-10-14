//! 🧪️ The `equipment-gain` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mut_equipment_gain as equipment_gain_mutations;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_equipment_gain_schedule {
    pub use crate::equipment_gain_mutations::change_equipment_gain_schedule::*;
    #[path = "../../🖥️equipment-gain/⌛️change-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🖥️equipment-gain/⌛️change-schedule/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_equipment_gain_watts_per_area {
    pub use crate::equipment_gain_mutations::change_equipment_gain_watts_per_area::*;
    #[path = "../../🖥️equipment-gain/⚡️change-watts-per-area/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🖥️equipment-gain/⚡️change-watts-per-area/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_equipment_gain_radiant_fraction {
    pub use crate::equipment_gain_mutations::change_equipment_gain_radiant_fraction::*;
    #[path = "../../🖥️equipment-gain/🌠️change-radiant-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🖥️equipment-gain/🌠️change-radiant-fraction/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_equipment_gain_latent_fraction {
    pub use crate::equipment_gain_mutations::change_equipment_gain_latent_fraction::*;
    #[path = "../../🖥️equipment-gain/💦️change-latent-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🖥️equipment-gain/💦️change-latent-fraction/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_equipment_gain {
    pub use crate::equipment_gain_mutations::create_equipment_gain::*;
    #[path = "../../🖥️equipment-gain/🖥️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🖥️equipment-gain/🖥️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_equipment_gain_zone {
    pub use crate::equipment_gain_mutations::change_equipment_gain_zone::*;
    #[path = "../../🖥️equipment-gain/🖨️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🖥️equipment-gain/🖨️change-zone/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_equipment_gain {
    pub use crate::equipment_gain_mutations::delete_equipment_gain::*;
    #[path = "../../🖥️equipment-gain/🧯️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🖥️equipment-gain/🧯️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
