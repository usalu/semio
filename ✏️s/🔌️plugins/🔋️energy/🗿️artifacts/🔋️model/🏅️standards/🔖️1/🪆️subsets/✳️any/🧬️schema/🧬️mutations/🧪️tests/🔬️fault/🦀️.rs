//! 🧪️ The `fault` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mut_fault as fault_mutations;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod create_fault {
    pub use crate::fault_mutations::create_fault::*;
    #[path = "../../⚠️fault/⚠️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../⚠️fault/⚠️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_fault_severity {
    pub use crate::fault_mutations::change_fault_severity::*;
    #[path = "../../⚠️fault/🌶️change-severity/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../⚠️fault/🌶️change-severity/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_fault_target_equipment {
    pub use crate::fault_mutations::change_fault_target_equipment::*;
    #[path = "../../⚠️fault/🎣️change-target-equipment/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../⚠️fault/🎣️change-target-equipment/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_fault_type {
    pub use crate::fault_mutations::change_fault_type::*;
    #[path = "../../⚠️fault/🐛️change-type/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../⚠️fault/🐛️change-type/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_fault_start_schedule {
    pub use crate::fault_mutations::change_fault_start_schedule::*;
    #[path = "../../⚠️fault/🕓️change-start-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../⚠️fault/🕓️change-start-schedule/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_fault {
    pub use crate::fault_mutations::delete_fault::*;
    #[path = "../../⚠️fault/🩹️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../⚠️fault/🩹️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
