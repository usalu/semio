//! 🧪️ The `water-system` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_water_system as water_system_mutations;

#[path = "."]
mod change_water_system_schedule {
    pub use crate::water_system_mutations::change_water_system_schedule::*;
    #[path = "../../🚰️water-system/🕒️change-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🚰️water-system/🕒️change-schedule/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_water_system {
    pub use crate::water_system_mutations::create_water_system::*;
    #[path = "../../🚰️water-system/🚽️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🚰️water-system/🚽️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_water_system_peak_flow {
    pub use crate::water_system_mutations::change_water_system_peak_flow::*;
    #[path = "../../🚰️water-system/🚾️change-peak-flow/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🚰️water-system/🚾️change-peak-flow/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_water_system {
    pub use crate::water_system_mutations::delete_water_system::*;
    #[path = "../../🚰️water-system/🧼️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🚰️water-system/🧼️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_water_system_fixture_count {
    pub use crate::water_system_mutations::change_water_system_fixture_count::*;
    #[path = "../../🚰️water-system/🪣️change-fixture-count/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🚰️water-system/🪣️change-fixture-count/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
