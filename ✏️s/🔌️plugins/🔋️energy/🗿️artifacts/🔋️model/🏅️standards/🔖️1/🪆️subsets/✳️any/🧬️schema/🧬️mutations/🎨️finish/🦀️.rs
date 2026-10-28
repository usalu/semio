//! 🎨️ Energy model mutation domain `finish`: `change-material-solar-absorptance`, `change-material-visible-absorptance`, `change-material-thermal-absorptance`, `change-material-roughness`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod change_material_solar_absorptance {
    #[path = "☀️change-solar-absorptance/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "☀️change-solar-absorptance/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "☀️change-solar-absorptance/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "☀️change-solar-absorptance/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_material_visible_absorptance {
    #[path = "👁️change-visible-absorptance/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "👁️change-visible-absorptance/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "👁️change-visible-absorptance/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "👁️change-visible-absorptance/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_material_thermal_absorptance {
    #[path = "🔆️change-thermal-absorptance/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔆️change-thermal-absorptance/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔆️change-thermal-absorptance/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔆️change-thermal-absorptance/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_material_roughness {
    #[path = "🗻️change-roughness/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🗻️change-roughness/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🗻️change-roughness/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🗻️change-roughness/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_material_solar_absorptance::{change_material_solar_absorptance, ChangeMaterialSolarAbsorptance};
pub use change_material_visible_absorptance::{change_material_visible_absorptance, ChangeMaterialVisibleAbsorptance};
pub use change_material_thermal_absorptance::{change_material_thermal_absorptance, ChangeMaterialThermalAbsorptance};
pub use change_material_roughness::{change_material_roughness, ChangeMaterialRoughness};
