//! 🧪️ The `plant-loop` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_mut_plant_loop as plant_loop_mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_plant_loop_supply_temperature {
    pub use crate::plant_loop_mutations::change_plant_loop_supply_temperature::*;
    #[path = "../../🏭️plant-loop/☕️change-supply-temperature/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏭️plant-loop/☕️change-supply-temperature/🧪️tests/✅️cools/🦀️.rs"]
    mod tests_cools;
}

#[path = "."]
mod change_plant_loop_type {
    pub use crate::plant_loop_mutations::change_plant_loop_type::*;
    #[path = "../../🏭️plant-loop/♻️change-type/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏭️plant-loop/♻️change-type/🧪️tests/✅️turns/🦀️.rs"]
    mod tests_turns;
}

#[path = "."]
mod create_plant_loop {
    pub use crate::plant_loop_mutations::create_plant_loop::*;
    #[path = "../../🏭️plant-loop/⚗️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏭️plant-loop/⚗️create/🧪️tests/✅️adds-a-hot-loop/🦀️.rs"]
    mod tests_adds_a_hot_loop;
}

#[path = "."]
mod remove_plant_loop_equipment {
    pub use crate::plant_loop_mutations::remove_plant_loop_equipment::*;
    #[path = "../../🏭️plant-loop/⚙️remove-equipment/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏭️plant-loop/⚙️remove-equipment/🧪️tests/✅️drops/🦀️.rs"]
    mod tests_drops;
}

#[path = "."]
mod delete_plant_loop {
    pub use crate::plant_loop_mutations::delete_plant_loop::*;
    #[path = "../../🏭️plant-loop/💣️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏭️plant-loop/💣️delete/🧪️tests/✅️drops-the-loop/🦀️.rs"]
    mod tests_drops_the_loop;
}

#[path = "."]
mod rename_plant_loop {
    pub use crate::plant_loop_mutations::rename_plant_loop::*;
    #[path = "../../🏭️plant-loop/📛️rename/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏭️plant-loop/📛️rename/🧪️tests/✅️renames-the-loop/🦀️.rs"]
    mod tests_renames_the_loop;
}

#[path = "."]
mod add_plant_loop_equipment {
    pub use crate::plant_loop_mutations::add_plant_loop_equipment::*;
    #[path = "../../🏭️plant-loop/🔩️add-equipment/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏭️plant-loop/🔩️add-equipment/🧪️tests/✅️names/🦀️.rs"]
    mod tests_names;
}

#[path = "."]
mod change_plant_loop_design_flow {
    pub use crate::plant_loop_mutations::change_plant_loop_design_flow::*;
    #[path = "../../🏭️plant-loop/🚤️change-design-flow/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏭️plant-loop/🚤️change-design-flow/🧪️tests/✅️uprates/🦀️.rs"]
    mod tests_uprates;
}

#[path = "."]
mod change_plant_loop_return_temperature {
    pub use crate::plant_loop_mutations::change_plant_loop_return_temperature::*;
    #[path = "../../🏭️plant-loop/🫖️change-return-temperature/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏭️plant-loop/🫖️change-return-temperature/🧪️tests/✅️cools/🦀️.rs"]
    mod tests_cools;
}
