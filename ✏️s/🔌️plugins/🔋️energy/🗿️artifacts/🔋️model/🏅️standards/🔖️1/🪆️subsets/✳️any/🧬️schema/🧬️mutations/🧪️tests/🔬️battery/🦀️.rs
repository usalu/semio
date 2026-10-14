//! 🧪️ The `battery` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mut_battery as battery_mutations;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_battery_max_charge {
    pub use crate::battery_mutations::change_battery_max_charge::*;
    #[path = "../../🪫️battery/⏫️change-max-charge/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪫️battery/⏫️change-max-charge/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_battery_max_discharge {
    pub use crate::battery_mutations::change_battery_max_discharge::*;
    #[path = "../../🪫️battery/⏬️change-max-discharge/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪫️battery/⏬️change-max-discharge/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_battery {
    pub use crate::battery_mutations::delete_battery::*;
    #[path = "../../🪫️battery/♒️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪫️battery/♒️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_battery_round_trip_efficiency {
    pub use crate::battery_mutations::change_battery_round_trip_efficiency::*;
    #[path = "../../🪫️battery/🥉️change-round-trip-efficiency/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪫️battery/🥉️change-round-trip-efficiency/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_battery_capacity {
    pub use crate::battery_mutations::change_battery_capacity::*;
    #[path = "../../🪫️battery/🥫️change-capacity/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪫️battery/🥫️change-capacity/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_battery {
    pub use crate::battery_mutations::create_battery::*;
    #[path = "../../🪫️battery/🪙️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪫️battery/🪙️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
