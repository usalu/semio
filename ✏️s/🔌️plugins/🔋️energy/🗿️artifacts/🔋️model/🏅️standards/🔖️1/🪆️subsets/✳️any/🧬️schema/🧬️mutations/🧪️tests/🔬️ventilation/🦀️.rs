//! 🧪️ The `ventilation` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_ventilation as ventilation_mutations;

#[path = "."]
mod create_mechanical_ventilation {
    pub use crate::ventilation_mutations::create_mechanical_ventilation::*;
    #[path = "../../🌪️ventilation/🌪️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌪️ventilation/🌪️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_mechanical_ventilation_fan_delta_pressure {
    pub use crate::ventilation_mutations::change_mechanical_ventilation_fan_delta_pressure::*;
    #[path = "../../🌪️ventilation/🎈️change-fan-delta-pressure/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌪️ventilation/🎈️change-fan-delta-pressure/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_mechanical_ventilation_fan_total_efficiency {
    pub use crate::ventilation_mutations::change_mechanical_ventilation_fan_total_efficiency::*;
    #[path = "../../🌪️ventilation/💠️change-fan-total-efficiency/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌪️ventilation/💠️change-fan-total-efficiency/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_mechanical_ventilation_schedule {
    pub use crate::ventilation_mutations::change_mechanical_ventilation_schedule::*;
    #[path = "../../🌪️ventilation/📆️change-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌪️ventilation/📆️change-schedule/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_mechanical_ventilation {
    pub use crate::ventilation_mutations::delete_mechanical_ventilation::*;
    #[path = "../../🌪️ventilation/🚫️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌪️ventilation/🚫️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_mechanical_ventilation_design_flow {
    pub use crate::ventilation_mutations::change_mechanical_ventilation_design_flow::*;
    #[path = "../../🌪️ventilation/🚿️change-design-flow/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌪️ventilation/🚿️change-design-flow/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_mechanical_ventilation_zone {
    pub use crate::ventilation_mutations::change_mechanical_ventilation_zone::*;
    #[path = "../../🌪️ventilation/🧭️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌪️ventilation/🧭️change-zone/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
