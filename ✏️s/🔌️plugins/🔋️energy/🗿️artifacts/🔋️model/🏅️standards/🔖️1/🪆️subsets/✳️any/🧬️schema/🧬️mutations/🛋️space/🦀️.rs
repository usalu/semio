//! 🛋️ Energy model mutation domain `space`: `rename-space`, `change-space-zone`, `change-space-floor-area`, `delete-space`, `create-space`.

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
pub mod rename_space {
    #[path = "🔤️rename/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔤️rename/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔤️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔤️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_space_zone {
    #[path = "🚚️change-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🚚️change-zone/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🚚️change-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🚚️change-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_space_floor_area {
    #[path = "🧮️change-floor-area/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧮️change-floor-area/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🧮️change-floor-area/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧮️change-floor-area/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_space {
    #[path = "🧹️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧹️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🧹️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧹️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_space {
    #[path = "🪑️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪑️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🪑️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪑️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use rename_space::{rename_space, RenameSpace};
pub use change_space_zone::{change_space_zone, ChangeSpaceZone};
pub use change_space_floor_area::{change_space_floor_area, ChangeSpaceFloorArea};
pub use delete_space::{delete_space, DeleteSpace};
pub use create_space::{create_space, CreateSpace};
