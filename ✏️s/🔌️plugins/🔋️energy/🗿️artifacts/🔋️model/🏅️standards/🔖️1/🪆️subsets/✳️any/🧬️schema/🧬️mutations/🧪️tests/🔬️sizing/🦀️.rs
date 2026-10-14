//! 🧪️ The `sizing` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_mut_sizing as sizing_mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_sizing_object_design_day_type {
    pub use crate::sizing_mutations::change_sizing_object_design_day_type::*;
    #[path = "../../📏️sizing/🌥️change-design-day-type/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📏️sizing/🌥️change-design-day-type/🧪️tests/✅️reads/🦀️.rs"]
    mod tests_reads;
}

#[path = "."]
mod change_sizing_object_zone {
    pub use crate::sizing_mutations::change_sizing_object_zone::*;
    #[path = "../../📏️sizing/🏨️change-zone/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📏️sizing/🏨️change-zone/🧪️tests/✅️moves/🦀️.rs"]
    mod tests_moves;
}

#[path = "."]
mod create_sizing_object {
    pub use crate::sizing_mutations::create_sizing_object::*;
    #[path = "../../📏️sizing/📶️create/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📏️sizing/📶️create/🧪️tests/✅️sizes/🦀️.rs"]
    mod tests_sizes;
}

#[path = "."]
mod change_sizing_object_sizing_type {
    pub use crate::sizing_mutations::change_sizing_object_sizing_type::*;
    #[path = "../../📏️sizing/🧾️change-sizing-type/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📏️sizing/🧾️change-sizing-type/🧪️tests/✅️sizes/🦀️.rs"]
    mod tests_sizes;
}

#[path = "."]
mod delete_sizing_object {
    pub use crate::sizing_mutations::delete_sizing_object::*;
    #[path = "../../📏️sizing/🪒️delete/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../📏️sizing/🪒️delete/🧪️tests/✅️drops/🦀️.rs"]
    mod tests_drops;
}
