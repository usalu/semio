//! 🧪️ The `fixed-schedule` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mut_fixed_schedule as fixed_schedule_mutations;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod delete_constant_schedule {
    pub use crate::fixed_schedule_mutations::delete_constant_schedule::*;
    #[path = "../../🕐️fixed-schedule/📍️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🕐️fixed-schedule/📍️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_constant_schedule {
    pub use crate::fixed_schedule_mutations::create_constant_schedule::*;
    #[path = "../../🕐️fixed-schedule/🕜️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🕐️fixed-schedule/🕜️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_constant_schedule_value {
    pub use crate::fixed_schedule_mutations::change_constant_schedule_value::*;
    #[path = "../../🕐️fixed-schedule/🕝️change-value/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🕐️fixed-schedule/🕝️change-value/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
