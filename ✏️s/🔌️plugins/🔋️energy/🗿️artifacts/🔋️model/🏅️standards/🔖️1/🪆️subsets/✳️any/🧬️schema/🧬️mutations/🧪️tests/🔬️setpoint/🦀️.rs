//! 🧪️ The `setpoint` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_mut_setpoint as setpoint_mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod delete_setpoint_manager {
    pub use crate::setpoint_mutations::delete_setpoint_manager::*;
    #[path = "../../🎚️setpoint/🍄️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🎚️setpoint/🍄️delete/🧪️tests/✅️drops/🦀️.rs"]
    mod tests_drops;
}

#[path = "."]
mod change_setpoint_manager_schedule {
    pub use crate::setpoint_mutations::change_setpoint_manager_schedule::*;
    #[path = "../../🎚️setpoint/🎼️change-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🎚️setpoint/🎼️change-schedule/🧪️tests/✅️repoints/🦀️.rs"]
    mod tests_repoints;
}

#[path = "."]
mod create_setpoint_manager {
    pub use crate::setpoint_mutations::create_setpoint_manager::*;
    #[path = "../../🎚️setpoint/📌️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🎚️setpoint/📌️create/🧪️tests/✅️adds/🦀️.rs"]
    mod tests_adds;
}

#[path = "."]
mod replace_setpoint_manager_kind {
    pub use crate::setpoint_mutations::replace_setpoint_manager_kind::*;
    #[path = "../../🎚️setpoint/🔃️replace-kind/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🎚️setpoint/🔃️replace-kind/🧪️tests/✅️resets/🦀️.rs"]
    mod tests_resets;
}

#[path = "."]
mod rename_setpoint_manager {
    pub use crate::setpoint_mutations::rename_setpoint_manager::*;
    #[path = "../../🎚️setpoint/🖇️rename/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🎚️setpoint/🖇️rename/🧪️tests/✅️renames/🦀️.rs"]
    mod tests_renames;
}
