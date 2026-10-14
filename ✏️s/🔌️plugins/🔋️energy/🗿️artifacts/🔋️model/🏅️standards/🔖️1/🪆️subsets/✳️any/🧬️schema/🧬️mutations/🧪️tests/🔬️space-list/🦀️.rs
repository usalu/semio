//! 🧪️ The `space-list` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_space_list as space_list_mutations;

#[path = "."]
mod add_space_list_member {
    pub use crate::space_list_mutations::add_space_list_member::*;
    #[path = "../../📋️space-list/➡️add-member/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📋️space-list/➡️add-member/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_space_list {
    pub use crate::space_list_mutations::create_space_list::*;
    #[path = "../../📋️space-list/📋️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📋️space-list/📋️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_space_list {
    pub use crate::space_list_mutations::delete_space_list::*;
    #[path = "../../📋️space-list/🗒️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📋️space-list/🗒️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod remove_space_list_member {
    pub use crate::space_list_mutations::remove_space_list_member::*;
    #[path = "../../📋️space-list/🪤️remove-member/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📋️space-list/🪤️remove-member/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod rename_space_list {
    pub use crate::space_list_mutations::rename_space_list::*;
    #[path = "../../📋️space-list/🪶️rename/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📋️space-list/🪶️rename/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
