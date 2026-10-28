//! 🍃️ Energy model mutation domain `outdoor-air`: `change-outdoor-air-system-air-loop`, `create-outdoor-air-system`, `delete-outdoor-air-system`, `change-outdoor-air-system-economizer-enabled`, `change-outdoor-air-system-min-oa-flow`.

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
pub mod change_outdoor_air_system_air_loop {
    #[path = "⛓️change-air-loop/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⛓️change-air-loop/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "⛓️change-air-loop/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⛓️change-air-loop/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_outdoor_air_system {
    #[path = "🌲️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌲️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌲️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌲️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_outdoor_air_system {
    #[path = "🍂️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🍂️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🍂️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🍂️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_outdoor_air_system_economizer_enabled {
    #[path = "💰️change-economizer-enabled/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "💰️change-economizer-enabled/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "💰️change-economizer-enabled/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "💰️change-economizer-enabled/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_outdoor_air_system_min_oa_flow {
    #[path = "🦋️change-min-oa-flow/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🦋️change-min-oa-flow/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🦋️change-min-oa-flow/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🦋️change-min-oa-flow/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_outdoor_air_system_air_loop::{change_outdoor_air_system_air_loop, ChangeOutdoorAirSystemAirLoop};
pub use create_outdoor_air_system::{create_outdoor_air_system, CreateOutdoorAirSystem};
pub use delete_outdoor_air_system::{delete_outdoor_air_system, DeleteOutdoorAirSystem};
pub use change_outdoor_air_system_economizer_enabled::{change_outdoor_air_system_economizer_enabled, ChangeOutdoorAirSystemEconomizerEnabled};
pub use change_outdoor_air_system_min_oa_flow::{change_outdoor_air_system_min_oa_flow, ChangeOutdoorAirSystemMinOaFlow};
