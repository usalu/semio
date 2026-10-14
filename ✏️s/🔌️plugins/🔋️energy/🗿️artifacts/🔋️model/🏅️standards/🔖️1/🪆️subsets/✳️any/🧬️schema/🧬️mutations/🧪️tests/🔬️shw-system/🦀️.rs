//! 🧪️ The `shw-system` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_mut_shw_system as shw_system_mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_shw_system_heater_capacity {
    pub use crate::shw_system_mutations::change_shw_system_heater_capacity::*;
    #[path = "../../🚿️shw-system/🍵️change-heater-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🚿️shw-system/🍵️change-heater-capacity/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_shw_system_setpoint {
    pub use crate::shw_system_mutations::change_shw_system_setpoint::*;
    #[path = "../../🚿️shw-system/🏹️change-setpoint/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🚿️shw-system/🏹️change-setpoint/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_shw_system_schedule {
    pub use crate::shw_system_mutations::change_shw_system_schedule::*;
    #[path = "../../🚿️shw-system/🕐️change-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🚿️shw-system/🕐️change-schedule/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_shw_system {
    pub use crate::shw_system_mutations::delete_shw_system::*;
    #[path = "../../🚿️shw-system/🚱️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🚿️shw-system/🚱️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_shw_system {
    pub use crate::shw_system_mutations::create_shw_system::*;
    #[path = "../../🚿️shw-system/🛀️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🚿️shw-system/🛀️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_shw_system_storage_volume {
    pub use crate::shw_system_mutations::change_shw_system_storage_volume::*;
    #[path = "../../🚿️shw-system/🛢️change-storage-volume/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🚿️shw-system/🛢️change-storage-volume/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
