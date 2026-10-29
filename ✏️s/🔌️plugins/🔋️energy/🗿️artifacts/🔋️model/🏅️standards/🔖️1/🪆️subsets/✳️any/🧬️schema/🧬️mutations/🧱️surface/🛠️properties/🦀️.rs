//! 🛠️ Energy model mutation domain `surface-properties`: `change-surface-multiplier`, `replace-surface-vertices`, `change-surface-class`, `change-surface-construction`


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod change_surface_multiplier {
    #[path = "../🔁️change-multiplier/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔁️change-multiplier/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🔁️change-multiplier/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔁️change-multiplier/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod replace_surface_vertices {
    #[path = "../🔺️replace-vertices/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🔺️replace-vertices/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🔺️replace-vertices/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🔺️replace-vertices/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_surface_class {
    #[path = "../🧩️change-class/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🧩️change-class/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🧩️change-class/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🧩️change-class/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_surface_construction {
    #[path = "../🧰️change-construction/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🧰️change-construction/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🧰️change-construction/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🧰️change-construction/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_surface_multiplier::{change_surface_multiplier, ChangeSurfaceMultiplier};
pub use replace_surface_vertices::{replace_surface_vertices, ReplaceSurfaceVertices};
pub use change_surface_class::{change_surface_class, ChangeSurfaceClass};
pub use change_surface_construction::{change_surface_construction, ChangeSurfaceConstruction};
