//! 🧪️ The `people-gain` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_mut_people_gain as people_gain_mutations;
extern crate semio_s_artifact_energy_model_mut_people_gain_fractions as people_gain_fractions_mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_people_gain_schedule {
    pub use crate::people_gain_mutations::change_people_gain_schedule::*;
    #[path = "../../👥️people-gain/⏰️change-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../👥️people-gain/⏰️change-schedule/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_people_gain_sensible_fraction {
    pub use crate::people_gain_fractions_mutations::change_people_gain_sensible_fraction::*;
    #[path = "../../👥️people-gain/🌞️change-sensible-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../👥️people-gain/🌞️change-sensible-fraction/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_people_gain_activity_schedule {
    pub use crate::people_gain_mutations::change_people_gain_activity_schedule::*;
    #[path = "../../👥️people-gain/🏃️change-activity-schedule/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../👥️people-gain/🏃️change-activity-schedule/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_people_gain {
    pub use crate::people_gain_mutations::create_people_gain::*;
    #[path = "../../👥️people-gain/👤️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../👥️people-gain/👤️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_people_gain_people_per_area {
    pub use crate::people_gain_mutations::change_people_gain_people_per_area::*;
    #[path = "../../👥️people-gain/👥️change-people-per-area/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../👥️people-gain/👥️change-people-per-area/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_people_gain_latent_fraction {
    pub use crate::people_gain_fractions_mutations::change_people_gain_latent_fraction::*;
    #[path = "../../👥️people-gain/💧️change-latent-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../👥️people-gain/💧️change-latent-fraction/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_people_gain_radiant_fraction {
    pub use crate::people_gain_fractions_mutations::change_people_gain_radiant_fraction::*;
    #[path = "../../👥️people-gain/📡️change-radiant-fraction/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../👥️people-gain/📡️change-radiant-fraction/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_people_gain_zone {
    pub use crate::people_gain_mutations::change_people_gain_zone::*;
    #[path = "../../👥️people-gain/🚶️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../👥️people-gain/🚶️change-zone/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_people_gain {
    pub use crate::people_gain_mutations::delete_people_gain::*;
    #[path = "../../👥️people-gain/🚷️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../👥️people-gain/🚷️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
