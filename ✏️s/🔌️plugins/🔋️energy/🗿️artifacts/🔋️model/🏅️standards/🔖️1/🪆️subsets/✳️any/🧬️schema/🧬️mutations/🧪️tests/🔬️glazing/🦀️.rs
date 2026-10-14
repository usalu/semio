//! 🧪️ The `glazing` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_artifact_energy_model_mut_glazing as glazing_mutations;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

#[path = "."]
mod change_fenestration_shgc {
    pub use crate::glazing_mutations::change_fenestration_shgc::*;
    #[path = "../../🔍️glazing/🌇️change-shgc/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔍️glazing/🌇️change-shgc/🧪️tests/✅️dims/🦀️.rs"]
    mod tests_dims;
}

#[path = "."]
mod change_fenestration_vlt {
    pub use crate::glazing_mutations::change_fenestration_vlt::*;
    #[path = "../../🔍️glazing/🌈️change-vlt/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔍️glazing/🌈️change-vlt/🧪️tests/✅️dims/🦀️.rs"]
    mod tests_dims;
}

#[path = "."]
mod change_fenestration_u_value {
    pub use crate::glazing_mutations::change_fenestration_u_value::*;
    #[path = "../../🔍️glazing/🌐️change-u-value/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔍️glazing/🌐️change-u-value/🧪️tests/✅️swaps/🦀️.rs"]
    mod tests_swaps;
}

#[path = "."]
mod change_fenestration_frame_conductance {
    pub use crate::glazing_mutations::change_fenestration_frame_conductance::*;
    #[path = "../../🔍️glazing/🖼️change-frame-conductance/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔍️glazing/🖼️change-frame-conductance/🧪️tests/✅️adds-a/🦀️.rs"]
    mod tests_adds_a;
}

#[path = "."]
mod bind_fenestration_glazing_construction {
    pub use crate::glazing_mutations::bind_fenestration_glazing_construction::*;
    #[path = "../../🔍️glazing/🧊️bind-glazing-construction/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔍️glazing/🧊️bind-glazing-construction/🧪️tests/✅️glazes/🦀️.rs"]
    mod tests_glazes;
}

#[path = "."]
mod change_fenestration_divider_conductance {
    pub use crate::glazing_mutations::change_fenestration_divider_conductance::*;
    #[path = "../../🔍️glazing/🧷️change-divider-conductance/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔍️glazing/🧷️change-divider-conductance/🧪️tests/✅️adds/🦀️.rs"]
    mod tests_adds;
}

#[path = "."]
mod clear_fenestration_glazing_construction {
    pub use crate::glazing_mutations::clear_fenestration_glazing_construction::*;
    #[path = "../../🔍️glazing/🫗️clear-glazing-construction/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🔍️glazing/🫗️clear-glazing-construction/🧪️tests/✅️ungazes/🦀️.rs"]
    mod tests_ungazes;
}
