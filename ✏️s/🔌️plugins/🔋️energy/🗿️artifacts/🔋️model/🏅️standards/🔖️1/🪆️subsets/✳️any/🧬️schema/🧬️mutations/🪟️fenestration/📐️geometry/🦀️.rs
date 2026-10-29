//! 📐️ Energy model mutation domain `fenestration-geometry`: `change-fenestration-height`, `change-fenestration-sill-height`, `change-fenestration-area`, `replace-fenestration-vertices`


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_geometry as geometry;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod change_fenestration_height {
    #[path = "../⬆️change-height/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../⬆️change-height/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../⬆️change-height/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../⬆️change-height/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fenestration_sill_height {
    #[path = "../⬇️change-sill-height/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../⬇️change-sill-height/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../⬇️change-sill-height/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../⬇️change-sill-height/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod replace_fenestration_vertices {
    #[path = "../🔶️replace-vertices/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔶️replace-vertices/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🔶️replace-vertices/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔶️replace-vertices/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fenestration_area {
    #[path = "../🟥️change-area/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🟥️change-area/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🟥️change-area/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🟥️change-area/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_fenestration_height::{change_fenestration_height, ChangeFenestrationHeight};
pub use change_fenestration_sill_height::{change_fenestration_sill_height, ChangeFenestrationSillHeight};
pub use replace_fenestration_vertices::{replace_fenestration_vertices, ReplaceFenestrationVertices};
pub use change_fenestration_area::{change_fenestration_area, ChangeFenestrationArea};
