//! 🧪️ The `infiltration` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_plugin_energy_engine_air_exchange as air_exchange;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_artifact_energy_model_mut_infiltration as infiltration_mutations;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_infiltration_schedule {
    pub use crate::infiltration_mutations::change_infiltration_schedule::*;
    #[path = "../../💨️infiltration/⏳️change-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💨️infiltration/⏳️change-schedule/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_infiltration_zone {
    pub use crate::infiltration_mutations::change_infiltration_zone::*;
    #[path = "../../💨️infiltration/🌀️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💨️infiltration/🌀️change-zone/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_infiltration_flow_per_exterior_area {
    pub use crate::infiltration_mutations::change_infiltration_flow_per_exterior_area::*;
    #[path = "../../💨️infiltration/🌫️change-flow-per-exterior-area/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💨️infiltration/🌫️change-flow-per-exterior-area/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_infiltration {
    pub use crate::infiltration_mutations::create_infiltration::*;
    #[path = "../../💨️infiltration/💨️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💨️infiltration/💨️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_infiltration_design_flow_ach {
    pub use crate::infiltration_mutations::change_infiltration_design_flow_ach::*;
    #[path = "../../💨️infiltration/🔄️change-design-flow-ach/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💨️infiltration/🔄️change-design-flow-ach/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_infiltration_method {
    pub use crate::infiltration_mutations::change_infiltration_method::*;
    #[path = "../../💨️infiltration/🔬️change-method/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💨️infiltration/🔬️change-method/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_infiltration {
    pub use crate::infiltration_mutations::delete_infiltration::*;
    #[path = "../../💨️infiltration/🧽️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💨️infiltration/🧽️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
