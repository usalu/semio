//! 🌋️ Energy model mutation domain `site-ground`: `change-ground-deep`, `change-ground-building`, `change-ground-shallow`


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod change_ground_deep {
    #[path = "../⛏️change-ground-deep/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../⛏️change-ground-deep/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../⛏️change-ground-deep/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../⛏️change-ground-deep/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_ground_building {
    #[path = "../🌡️change-ground-building/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🌡️change-ground-building/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🌡️change-ground-building/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🌡️change-ground-building/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_ground_shallow {
    #[path = "../🌱️change-ground-shallow/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🌱️change-ground-shallow/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🌱️change-ground-shallow/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🌱️change-ground-shallow/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_ground_deep::{change_ground_deep, ChangeGroundDeep};
pub use change_ground_building::{change_ground_building, ChangeGroundBuilding};
pub use change_ground_shallow::{change_ground_shallow, ChangeGroundShallow};
