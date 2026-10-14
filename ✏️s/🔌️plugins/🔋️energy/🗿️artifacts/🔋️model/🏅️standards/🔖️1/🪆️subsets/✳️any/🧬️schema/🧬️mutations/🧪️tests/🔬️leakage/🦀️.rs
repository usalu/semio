//! 🧪️ The `leakage` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_plugin_energy_engine_air_exchange as air_exchange;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_artifact_energy_model_mut_leakage as leakage_mutations;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_infiltration_constant_term_coefficient {
    pub use crate::leakage_mutations::change_infiltration_constant_term_coefficient::*;
    #[path = "../../🕳️leakage/🅰️change-constant-term-coefficient/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🕳️leakage/🅰️change-constant-term-coefficient/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_infiltration_temperature_term_coefficient {
    pub use crate::leakage_mutations::change_infiltration_temperature_term_coefficient::*;
    #[path = "../../🕳️leakage/🅱️change-temperature-term-coefficient/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🕳️leakage/🅱️change-temperature-term-coefficient/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_infiltration_velocity_term_coefficient {
    pub use crate::leakage_mutations::change_infiltration_velocity_term_coefficient::*;
    #[path = "../../🕳️leakage/🆎️change-velocity-term-coefficient/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🕳️leakage/🆎️change-velocity-term-coefficient/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_infiltration_velocity_squared_term_coefficient {
    pub use crate::leakage_mutations::change_infiltration_velocity_squared_term_coefficient::*;
    #[path = "../../🕳️leakage/🆑️change-velocity-squared-term-coefficient/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🕳️leakage/🆑️change-velocity-squared-term-coefficient/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_infiltration_stack_height {
    pub use crate::leakage_mutations::change_infiltration_stack_height::*;
    #[path = "../../🕳️leakage/🏭️change-stack-height/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🕳️leakage/🏭️change-stack-height/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_infiltration_effective_leakage_area {
    pub use crate::leakage_mutations::change_infiltration_effective_leakage_area::*;
    #[path = "../../🕳️leakage/🕳️change-effective-leakage-area/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🕳️leakage/🕳️change-effective-leakage-area/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_infiltration_discharge_coefficient {
    pub use crate::leakage_mutations::change_infiltration_discharge_coefficient::*;
    #[path = "../../🕳️leakage/🚰️change-discharge-coefficient/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🕳️leakage/🚰️change-discharge-coefficient/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
