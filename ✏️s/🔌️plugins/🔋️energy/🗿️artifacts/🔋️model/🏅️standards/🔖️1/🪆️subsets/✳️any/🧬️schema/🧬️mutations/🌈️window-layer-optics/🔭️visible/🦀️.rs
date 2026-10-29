//! 🔭️ Energy model mutation domain `window-layer-optics-visible`: `change-glazing-material-visible-transmittance`, `change-glazing-material-visible-reflectance-front`, `change-glazing-material-visible-reflectance-back`


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod change_glazing_material_visible_transmittance {
    #[path = "../../🥂️window-layer/🥽️change-glazing-visible-transmittance/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../../🥂️window-layer/🥽️change-glazing-visible-transmittance/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../../🥂️window-layer/🥽️change-glazing-visible-transmittance/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../../🥂️window-layer/🥽️change-glazing-visible-transmittance/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_glazing_material_visible_reflectance_front {
    #[path = "../../🥂️window-layer/🔎️visible-front/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../../🥂️window-layer/🔎️visible-front/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../../🥂️window-layer/🔎️visible-front/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../../🥂️window-layer/🔎️visible-front/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_glazing_material_visible_reflectance_back {
    #[path = "../../🥂️window-layer/🪞️visible-back/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../../🥂️window-layer/🪞️visible-back/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../../🥂️window-layer/🪞️visible-back/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../../🥂️window-layer/🪞️visible-back/↩️inverse/🦀️.rs"]
    pub mod inverse;
}


pub use change_glazing_material_visible_transmittance::{change_glazing_material_visible_transmittance, ChangeGlazingMaterialVisibleTransmittance};
pub use change_glazing_material_visible_reflectance_front::{change_glazing_material_visible_reflectance_front, ChangeGlazingMaterialVisibleReflectanceFront};
pub use change_glazing_material_visible_reflectance_back::{change_glazing_material_visible_reflectance_back, ChangeGlazingMaterialVisibleReflectanceBack};
