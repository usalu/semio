//! 🔀️ Energy model mutation domain `air-loop-nodes`: `change-air-loop-supply-node`, `change-air-loop-return-node`, `change-air-loop-design-supply-air-flow`


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod change_air_loop_supply_node {
    #[path = "../↗️change-supply-node/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../↗️change-supply-node/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../↗️change-supply-node/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../↗️change-supply-node/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_air_loop_return_node {
    #[path = "../↘️change-return-node/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../↘️change-return-node/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../↘️change-return-node/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../↘️change-return-node/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_air_loop_design_supply_air_flow {
    #[path = "../🍥️change-design-supply-air-flow/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "../🍥️change-design-supply-air-flow/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "../🍥️change-design-supply-air-flow/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "../🍥️change-design-supply-air-flow/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_air_loop_supply_node::{change_air_loop_supply_node, ChangeAirLoopSupplyNode};
pub use change_air_loop_return_node::{change_air_loop_return_node, ChangeAirLoopReturnNode};
pub use change_air_loop_design_supply_air_flow::{change_air_loop_design_supply_air_flow, ChangeAirLoopDesignSupplyAirFlow};
