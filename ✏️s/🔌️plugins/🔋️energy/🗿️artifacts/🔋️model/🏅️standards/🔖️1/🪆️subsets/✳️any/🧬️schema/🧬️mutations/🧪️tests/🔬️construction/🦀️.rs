//! 🧪️ The `construction` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mut_construction as construction_mutations;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod add_construction_layer {
    pub use crate::construction_mutations::add_construction_layer::*;
    #[path = "../../🏗️construction/➕️add-layer/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏗️construction/➕️add-layer/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod remove_construction_layer {
    pub use crate::construction_mutations::remove_construction_layer::*;
    #[path = "../../🏗️construction/➖️remove-layer/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏗️construction/➖️remove-layer/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_construction {
    pub use crate::construction_mutations::create_construction::*;
    #[path = "../../🏗️construction/🏗️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏗️construction/🏗️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod reorder_construction_layers {
    pub use crate::construction_mutations::reorder_construction_layers::*;
    #[path = "../../🏗️construction/🔀️reorder-layers/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏗️construction/🔀️reorder-layers/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_construction {
    pub use crate::construction_mutations::delete_construction::*;
    #[path = "../../🏗️construction/🧨️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏗️construction/🧨️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod rename_construction {
    pub use crate::construction_mutations::rename_construction::*;
    #[path = "../../🏗️construction/🪪️rename/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏗️construction/🪪️rename/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
