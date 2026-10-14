//! 🧪️ The `zone` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_zone as zone_mutations;

#[path = "."]
mod change_zone_multiplier {
    pub use crate::zone_mutations::change_zone_multiplier::*;
    #[path = "../../🏡️zone/✖️change-multiplier/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏡️zone/✖️change-multiplier/🧪️tests/✅️stacks/🦀️.rs"]
    mod tests_stacks;
}

#[path = "."]
mod change_zone_conditioned {
    pub use crate::zone_mutations::change_zone_conditioned::*;
    #[path = "../../🏡️zone/🌬️change-conditioned/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏡️zone/🌬️change-conditioned/🧪️tests/✅️frees/🦀️.rs"]
    mod tests_frees;
}

#[path = "."]
mod create_zone {
    pub use crate::zone_mutations::create_zone::*;
    #[path = "../../🏡️zone/🏘️create/🧪️tests/⛔️refuses-a-taken-id/🦀️.rs"]
    mod tests_refuses_a_taken_id;
    #[path = "../../🏡️zone/🏘️create/🧪️tests/✅️adds-a-second-zone/🦀️.rs"]
    mod tests_adds_a_second_zone;
}

#[path = "."]
mod delete_zone {
    pub use crate::zone_mutations::delete_zone::*;
    #[path = "../../🏡️zone/🏚️delete/🧪️tests/⛔️refuses-a-used-zone/🦀️.rs"]
    mod tests_refuses_a_used_zone;
    #[path = "../../🏡️zone/🏚️delete/🧪️tests/✅️deletes-a-free-zone/🦀️.rs"]
    mod tests_deletes_a_free_zone;
}

#[path = "."]
mod rename_zone {
    pub use crate::zone_mutations::rename_zone::*;
    #[path = "../../🏡️zone/🏠️rename/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏡️zone/🏠️rename/🧪️tests/✅️renames-zone-one/🦀️.rs"]
    mod tests_renames_zone_one;
}

#[path = "."]
mod change_zone_floor_area_participation {
    pub use crate::zone_mutations::change_zone_floor_area_participation::*;
    #[path = "../../🏡️zone/📐️change-floor-area-participation/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏡️zone/📐️change-floor-area-participation/🧪️tests/✅️excludes/🦀️.rs"]
    mod tests_excludes;
}

#[path = "."]
mod change_zone_volume {
    pub use crate::zone_mutations::change_zone_volume::*;
    #[path = "../../🏡️zone/🧊️change-volume/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🏡️zone/🧊️change-volume/🧪️tests/✅️resizes/🦀️.rs"]
    mod tests_resizes;
}
