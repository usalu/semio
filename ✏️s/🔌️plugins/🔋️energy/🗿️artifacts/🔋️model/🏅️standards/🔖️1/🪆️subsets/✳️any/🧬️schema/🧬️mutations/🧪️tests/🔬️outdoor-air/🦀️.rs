//! 🧪️ The `outdoor-air` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_mut_outdoor_air as outdoor_air_mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_outdoor_air_system_air_loop {
    pub use crate::outdoor_air_mutations::change_outdoor_air_system_air_loop::*;
    #[path = "../../🍃️outdoor-air/⛓️change-air-loop/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🍃️outdoor-air/⛓️change-air-loop/🧪️tests/✅️moves/🦀️.rs"]
    mod tests_moves;
}

#[path = "."]
mod create_outdoor_air_system {
    pub use crate::outdoor_air_mutations::create_outdoor_air_system::*;
    #[path = "../../🍃️outdoor-air/🌲️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🍃️outdoor-air/🌲️create/🧪️tests/✅️ventilates/🦀️.rs"]
    mod tests_ventilates;
}

#[path = "."]
mod delete_outdoor_air_system {
    pub use crate::outdoor_air_mutations::delete_outdoor_air_system::*;
    #[path = "../../🍃️outdoor-air/🍂️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🍃️outdoor-air/🍂️delete/🧪️tests/✅️drops/🦀️.rs"]
    mod tests_drops;
}

#[path = "."]
mod change_outdoor_air_system_economizer_enabled {
    pub use crate::outdoor_air_mutations::change_outdoor_air_system_economizer_enabled::*;
    #[path = "../../🍃️outdoor-air/💰️change-economizer-enabled/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🍃️outdoor-air/💰️change-economizer-enabled/🧪️tests/✅️frees/🦀️.rs"]
    mod tests_frees;
}

#[path = "."]
mod change_outdoor_air_system_min_oa_flow {
    pub use crate::outdoor_air_mutations::change_outdoor_air_system_min_oa_flow::*;
    #[path = "../../🍃️outdoor-air/🦋️change-min-oa-flow/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🍃️outdoor-air/🦋️change-min-oa-flow/🧪️tests/✅️raises/🦀️.rs"]
    mod tests_raises;
}
