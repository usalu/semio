//! 🧪️ The `enclosure` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mut_enclosure as enclosure_mutations;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod create_thermal_enclosure {
    pub use crate::enclosure_mutations::create_thermal_enclosure::*;
    #[path = "../../🔒️enclosure/🏟️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔒️enclosure/🏟️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_thermal_enclosure {
    pub use crate::enclosure_mutations::delete_thermal_enclosure::*;
    #[path = "../../🔒️enclosure/🏯️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔒️enclosure/🏯️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod add_thermal_enclosure_zone {
    pub use crate::enclosure_mutations::add_thermal_enclosure_zone::*;
    #[path = "../../🔒️enclosure/🔒️add-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔒️enclosure/🔒️add-zone/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod remove_thermal_enclosure_zone {
    pub use crate::enclosure_mutations::remove_thermal_enclosure_zone::*;
    #[path = "../../🔒️enclosure/🔓️remove-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔒️enclosure/🔓️remove-zone/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod rename_thermal_enclosure {
    pub use crate::enclosure_mutations::rename_thermal_enclosure::*;
    #[path = "../../🔒️enclosure/🖋️rename/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔒️enclosure/🖋️rename/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
