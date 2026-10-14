//! 🧪️ The `refrigeration` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_mut_refrigeration as refrigeration_mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod create_refrigeration_system {
    pub use crate::refrigeration_mutations::create_refrigeration_system::*;
    #[path = "../../❄️refrigeration/❄️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../❄️refrigeration/❄️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_refrigeration_system_design_load {
    pub use crate::refrigeration_mutations::change_refrigeration_system_design_load::*;
    #[path = "../../❄️refrigeration/🏋️change-design-load/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../❄️refrigeration/🏋️change-design-load/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_refrigeration_system_defrost_schedule {
    pub use crate::refrigeration_mutations::change_refrigeration_system_defrost_schedule::*;
    #[path = "../../❄️refrigeration/🕑️change-defrost-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../❄️refrigeration/🕑️change-defrost-schedule/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_refrigeration_system_case_count {
    pub use crate::refrigeration_mutations::change_refrigeration_system_case_count::*;
    #[path = "../../❄️refrigeration/🗄️change-case-count/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../❄️refrigeration/🗄️change-case-count/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_refrigeration_system {
    pub use crate::refrigeration_mutations::delete_refrigeration_system::*;
    #[path = "../../❄️refrigeration/🫠️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../❄️refrigeration/🫠️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
