//! 🧪️ The `window-layer` domain's committed vectors, each held to the fixture laws through the aggregate.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_mutation_fixtures as fixtures;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
extern crate semio_s_artifact_energy_model_mut_window_layer as window_layer_mutations;
extern crate semio_s_artifact_energy_model_mut_window_layer_optics as window_layer_optics_mutations;

#[path = "."]
mod rename_gas_material {
    pub use crate::window_layer_mutations::rename_gas_material::*;
    #[path = "../../🥂️window-layer/🔘️rename-gas/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🥂️window-layer/🔘️rename-gas/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_glazing_material_thickness {
    pub use crate::window_layer_mutations::change_glazing_material_thickness::*;
    #[path = "../../🥂️window-layer/🔷️change-glazing-thickness/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🥂️window-layer/🔷️change-glazing-thickness/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_glazing_material_conductivity {
    pub use crate::window_layer_mutations::change_glazing_material_conductivity::*;
    #[path = "../../🥂️window-layer/🟠️change-glazing-conductivity/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🥂️window-layer/🟠️change-glazing-conductivity/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_glazing_material_solar_transmittance {
    pub use crate::window_layer_optics_mutations::change_glazing_material_solar_transmittance::*;
    #[path = "../../🥂️window-layer/🟡️change-glazing-solar-transmittance/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🥂️window-layer/🟡️change-glazing-solar-transmittance/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod rename_glazing_material {
    pub use crate::window_layer_mutations::rename_glazing_material::*;
    #[path = "../../🥂️window-layer/🟢️rename-glazing/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🥂️window-layer/🟢️rename-glazing/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_gas_material_thickness {
    pub use crate::window_layer_mutations::change_gas_material_thickness::*;
    #[path = "../../🥂️window-layer/🟣️change-gas-thickness/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🥂️window-layer/🟣️change-gas-thickness/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_gas_material_gas {
    pub use crate::window_layer_mutations::change_gas_material_gas::*;
    #[path = "../../🥂️window-layer/🟤️change-gas-type/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🥂️window-layer/🟤️change-gas-type/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_glazing_material_visible_transmittance {
    pub use crate::window_layer_optics_mutations::change_glazing_material_visible_transmittance::*;
    #[path = "../../🥂️window-layer/🥽️glazing-visible-transmittance/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🥂️window-layer/🥽️glazing-visible-transmittance/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}

#[path = "."]
mod change_glazing_material_infrared_emissivity {
    pub use crate::window_layer_optics_mutations::change_glazing_material_infrared_emissivity::*;
    #[path = "../../🥂️window-layer/🩻️change-glazing-infrared-emissivity/🧪️tests/⛔️refuses/🦀️.rs"]
    mod tests_refuses;
    #[path = "../../🥂️window-layer/🩻️change-glazing-infrared-emissivity/🧪️tests/✅️applies/🦀️.rs"]
    mod tests_applies;
}
