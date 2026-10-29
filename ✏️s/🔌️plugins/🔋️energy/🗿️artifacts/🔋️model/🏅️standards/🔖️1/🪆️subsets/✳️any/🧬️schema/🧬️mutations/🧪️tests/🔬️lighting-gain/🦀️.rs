//! 🧪️ The `lighting-gain` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_artifact_energy_model_mut_lighting_gain as lighting_gain_mutations;
extern crate semio_s_artifact_energy_model_mut_lighting_gain_emission as lighting_gain_emission_mutations;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_lighting_gain_schedule {
    pub use crate::lighting_gain_mutations::change_lighting_gain_schedule::*;
    #[path = "../../💡️lighting-gain/⏱️change-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💡️lighting-gain/⏱️change-schedule/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_lighting_gain_radiant_fraction {
    pub use crate::lighting_gain_emission_mutations::change_lighting_gain_radiant_fraction::*;
    #[path = "../../💡️lighting-gain/🌟️change-radiant-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💡️lighting-gain/🌟️change-radiant-fraction/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_lighting_gain_return_air_fraction {
    pub use crate::lighting_gain_emission_mutations::change_lighting_gain_return_air_fraction::*;
    #[path = "../../💡️lighting-gain/🎐️change-return-air-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💡️lighting-gain/🎐️change-return-air-fraction/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_lighting_gain {
    pub use crate::lighting_gain_mutations::create_lighting_gain::*;
    #[path = "../../💡️lighting-gain/💡️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💡️lighting-gain/💡️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_lighting_gain_visible_fraction {
    pub use crate::lighting_gain_emission_mutations::change_lighting_gain_visible_fraction::*;
    #[path = "../../💡️lighting-gain/🔅️change-visible-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💡️lighting-gain/🔅️change-visible-fraction/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_lighting_gain_watts_per_area {
    pub use crate::lighting_gain_emission_mutations::change_lighting_gain_watts_per_area::*;
    #[path = "../../💡️lighting-gain/🔌️change-watts-per-area/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💡️lighting-gain/🔌️change-watts-per-area/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_lighting_gain_zone {
    pub use crate::lighting_gain_mutations::change_lighting_gain_zone::*;
    #[path = "../../💡️lighting-gain/🔦️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💡️lighting-gain/🔦️change-zone/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_lighting_gain {
    pub use crate::lighting_gain_mutations::delete_lighting_gain::*;
    #[path = "../../💡️lighting-gain/🕯️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../💡️lighting-gain/🕯️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
