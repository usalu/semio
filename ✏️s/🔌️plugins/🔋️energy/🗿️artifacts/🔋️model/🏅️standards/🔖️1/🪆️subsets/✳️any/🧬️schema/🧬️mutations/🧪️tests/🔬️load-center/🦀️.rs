//! 🧪️ The `load-center` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_artifact_energy_model_mut_load_center as load_center_mutations;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod add_electrical_load_center_pv {
    pub use crate::load_center_mutations::add_electrical_load_center_pv::*;
    #[path = "../../🔌️load-center/☄️add-pv/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔌️load-center/☄️add-pv/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod remove_electrical_load_center_pv {
    pub use crate::load_center_mutations::remove_electrical_load_center_pv::*;
    #[path = "../../🔌️load-center/🌘️remove-pv/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔌️load-center/🌘️remove-pv/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod create_electrical_load_center {
    pub use crate::load_center_mutations::create_electrical_load_center::*;
    #[path = "../../🔌️load-center/🏦️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔌️load-center/🏦️create/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod add_electrical_load_center_battery {
    pub use crate::load_center_mutations::add_electrical_load_center_battery::*;
    #[path = "../../🔌️load-center/🔋️add-battery/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔌️load-center/🔋️add-battery/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod delete_electrical_load_center {
    pub use crate::load_center_mutations::delete_electrical_load_center::*;
    #[path = "../../🔌️load-center/🔻️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔌️load-center/🔻️delete/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod rename_electrical_load_center {
    pub use crate::load_center_mutations::rename_electrical_load_center::*;
    #[path = "../../🔌️load-center/🖊️rename/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔌️load-center/🖊️rename/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod remove_electrical_load_center_battery {
    pub use crate::load_center_mutations::remove_electrical_load_center_battery::*;
    #[path = "../../🔌️load-center/🪝️remove-battery/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔌️load-center/🪝️remove-battery/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
