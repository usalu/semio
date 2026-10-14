//! ⛱️ Energy model mutation domain `shading-device`: `change-fenestration-overhang-offset`, `change-fenestration-fin-offset`, `change-fenestration-fin-depth`, `change-fenestration-overhang-depth`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::EnergyModelDiff;
use crate::snapshot::EnergyModelSnapshot;

#[path = "."]
pub mod change_fenestration_overhang_offset {
    #[path = "🎩️change-overhang-offset/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🎩️change-overhang-offset/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🎩️change-overhang-offset/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fenestration_fin_offset {
    #[path = "🐋️change-fin-offset/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🐋️change-fin-offset/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🐋️change-fin-offset/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fenestration_fin_depth {
    #[path = "🐬️change-fin-depth/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🐬️change-fin-depth/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🐬️change-fin-depth/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_fenestration_overhang_depth {
    #[path = "🧢️change-overhang-depth/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧢️change-overhang-depth/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧢️change-overhang-depth/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_fenestration_overhang_offset::{change_fenestration_overhang_offset, ChangeFenestrationOverhangOffset};
pub use change_fenestration_fin_offset::{change_fenestration_fin_offset, ChangeFenestrationFinOffset};
pub use change_fenestration_fin_depth::{change_fenestration_fin_depth, ChangeFenestrationFinDepth};
pub use change_fenestration_overhang_depth::{change_fenestration_overhang_depth, ChangeFenestrationOverhangDepth};
