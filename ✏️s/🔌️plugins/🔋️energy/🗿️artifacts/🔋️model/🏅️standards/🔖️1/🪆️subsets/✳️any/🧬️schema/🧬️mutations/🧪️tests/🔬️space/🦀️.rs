//! 🧪️ The `space` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_space as space_mutations;

#[path = "."]
mod rename_space {
    pub use crate::space_mutations::rename_space::*;
    #[path = "../../🛋️space/🔤️rename/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🛋️space/🔤️rename/🧪️tests/✅️renames-a-space/🦀️.rs"]
    mod tests_renames_a_space;
}

#[path = "."]
mod change_space_zone {
    pub use crate::space_mutations::change_space_zone::*;
    #[path = "../../🛋️space/🚚️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🛋️space/🚚️change-zone/🧪️tests/✅️moves-a-space/🦀️.rs"]
    mod tests_moves_a_space;
}

#[path = "."]
mod change_space_floor_area {
    pub use crate::space_mutations::change_space_floor_area::*;
    #[path = "../../🛋️space/🧮️change-floor-area/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🛋️space/🧮️change-floor-area/🧪️tests/✅️resizes/🦀️.rs"]
    mod tests_resizes;
}

#[path = "."]
mod delete_space {
    pub use crate::space_mutations::delete_space::*;
    #[path = "../../🛋️space/🧹️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🛋️space/🧹️delete/🧪️tests/✅️deletes-a-free-space/🦀️.rs"]
    mod tests_deletes_a_free_space;
}

#[path = "."]
mod create_space {
    pub use crate::space_mutations::create_space::*;
    #[path = "../../🛋️space/🪑️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🛋️space/🪑️create/🧪️tests/✅️adds-a-space/🦀️.rs"]
    mod tests_adds_a_space;
}
