//! 🧪️ The `room-air` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_mut_room_air as room_air_mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod create_room_air_model_assignment {
    pub use crate::room_air_mutations::create_room_air_model_assignment::*;
    #[path = "../../🛏️room-air/🛏️create-assignment/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🛏️room-air/🛏️create-assignment/🧪️tests/✅️stratifies/🦀️.rs"]
    mod tests_stratifies;
}

#[path = "."]
mod delete_room_air_model_assignment {
    pub use crate::room_air_mutations::delete_room_air_model_assignment::*;
    #[path = "../../🛏️room-air/🧺️delete-assignment/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🛏️room-air/🧺️delete-assignment/🧪️tests/✅️falls-back/🦀️.rs"]
    mod tests_falls_back;
}

#[path = "."]
mod change_room_air_model {
    pub use crate::room_air_mutations::change_room_air_model::*;
    #[path = "../../🛏️room-air/🪭️change/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🛏️room-air/🪭️change/🧪️tests/✅️stratifies/🦀️.rs"]
    mod tests_stratifies;
}
