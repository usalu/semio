//! 🧪️ The `fenestration` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mut_fenestration as fenestration_mutations;
extern crate semio_s_artifact_energy_model_mut_fenestration_geometry as fenestration_geometry_mutations;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_fenestration_height {
    pub use crate::fenestration_geometry_mutations::change_fenestration_height::*;
    #[path = "../../🪟️fenestration/⬆️change-height/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪟️fenestration/⬆️change-height/🧪️tests/✅️raises/🦀️.rs"]
    mod tests_raises;
}

#[path = "."]
mod change_fenestration_sill_height {
    pub use crate::fenestration_geometry_mutations::change_fenestration_sill_height::*;
    #[path = "../../🪟️fenestration/⬇️change-sill-height/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪟️fenestration/⬇️change-sill-height/🧪️tests/✅️raises/🦀️.rs"]
    mod tests_raises;
}

#[path = "."]
mod rename_fenestration {
    pub use crate::fenestration_mutations::rename_fenestration::*;
    #[path = "../../🪟️fenestration/🏁️rename/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪟️fenestration/🏁️rename/🧪️tests/✅️renames/🦀️.rs"]
    mod tests_renames;
}

#[path = "."]
mod replace_fenestration_vertices {
    pub use crate::fenestration_geometry_mutations::replace_fenestration_vertices::*;
    #[path = "../../🪟️fenestration/🔶️replace-vertices/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪟️fenestration/🔶️replace-vertices/🧪️tests/✅️shapes/🦀️.rs"]
    mod tests_shapes;
}

#[path = "."]
mod delete_fenestration {
    pub use crate::fenestration_mutations::delete_fenestration::*;
    #[path = "../../🪟️fenestration/🚪️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪟️fenestration/🚪️delete/🧪️tests/✅️removes/🦀️.rs"]
    mod tests_removes;
}

#[path = "."]
mod change_fenestration_area {
    pub use crate::fenestration_geometry_mutations::change_fenestration_area::*;
    #[path = "../../🪟️fenestration/🟥️change-area/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪟️fenestration/🟥️change-area/🧪️tests/✅️doubles/🦀️.rs"]
    mod tests_doubles;
}

#[path = "."]
mod change_fenestration_surface {
    pub use crate::fenestration_mutations::change_fenestration_surface::*;
    #[path = "../../🪟️fenestration/🧲️change-surface/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪟️fenestration/🧲️change-surface/🧪️tests/✅️rehosts/🦀️.rs"]
    mod tests_rehosts;
}

#[path = "."]
mod create_fenestration {
    pub use crate::fenestration_mutations::create_fenestration::*;
    #[path = "../../🪟️fenestration/🪟️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🪟️fenestration/🪟️create/🧪️tests/✅️adds/🦀️.rs"]
    mod tests_adds;
}
