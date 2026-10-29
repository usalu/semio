//! 🧪️ The `air-loop` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mut_air_loop as air_loop_mutations;
extern crate semio_s_artifact_energy_model_mut_air_loop_nodes as air_loop_nodes_mutations;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_air_loop_supply_node {
    pub use crate::air_loop_nodes_mutations::change_air_loop_supply_node::*;
    #[path = "../../🌬️air-loop/↗️change-supply-node/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌬️air-loop/↗️change-supply-node/🧪️tests/✅️repoints/🦀️.rs"]
    mod tests_repoints;
}

#[path = "."]
mod change_air_loop_return_node {
    pub use crate::air_loop_nodes_mutations::change_air_loop_return_node::*;
    #[path = "../../🌬️air-loop/↘️change-return-node/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌬️air-loop/↘️change-return-node/🧪️tests/✅️repoints/🦀️.rs"]
    mod tests_repoints;
}

#[path = "."]
mod change_air_loop_design_supply_air_flow {
    pub use crate::air_loop_nodes_mutations::change_air_loop_design_supply_air_flow::*;
    #[path = "../../🌬️air-loop/🍥️change-design-supply-air-flow/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌬️air-loop/🍥️change-design-supply-air-flow/🧪️tests/✅️uprates/🦀️.rs"]
    mod tests_uprates;
}

#[path = "."]
mod rename_air_loop {
    pub use crate::air_loop_mutations::rename_air_loop::*;
    #[path = "../../🌬️air-loop/📇️rename/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌬️air-loop/📇️rename/🧪️tests/✅️renames-the-loop/🦀️.rs"]
    mod tests_renames_the_loop;
}

#[path = "."]
mod create_air_loop {
    pub use crate::air_loop_mutations::create_air_loop::*;
    #[path = "../../🌬️air-loop/🛞️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌬️air-loop/🛞️create/🧪️tests/✅️adds-a-main-loop/🦀️.rs"]
    mod tests_adds_a_main_loop;
}

#[path = "."]
mod delete_air_loop {
    pub use crate::air_loop_mutations::delete_air_loop::*;
    #[path = "../../🌬️air-loop/🥀️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌬️air-loop/🥀️delete/🧪️tests/✅️drops-the-loop/🦀️.rs"]
    mod tests_drops_the_loop;
}

#[path = "."]
mod remove_air_loop_terminal_zone {
    pub use crate::air_loop_mutations::remove_air_loop_terminal_zone::*;
    #[path = "../../🌬️air-loop/🪹️remove-terminal-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌬️air-loop/🪹️remove-terminal-zone/🧪️tests/✅️stops/🦀️.rs"]
    mod tests_stops;
}

#[path = "."]
mod add_air_loop_terminal_zone {
    pub use crate::air_loop_mutations::add_air_loop_terminal_zone::*;
    #[path = "../../🌬️air-loop/🪺️add-terminal-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🌬️air-loop/🪺️add-terminal-zone/🧪️tests/✅️serves/🦀️.rs"]
    mod tests_serves;
}
