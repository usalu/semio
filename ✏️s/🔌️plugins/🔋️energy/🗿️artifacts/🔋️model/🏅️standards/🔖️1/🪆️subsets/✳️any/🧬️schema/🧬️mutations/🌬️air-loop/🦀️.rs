//! 🌬️ Energy model mutation domain `air-loop`: `rename-air-loop`, `create-air-loop`, `delete-air-loop`, `remove-air-loop-terminal-zone`, `add-air-loop-terminal-zone`

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
pub mod rename_air_loop {
    #[path = "📇️rename/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "📇️rename/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "📇️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📇️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_air_loop {
    #[path = "🛞️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🛞️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🛞️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🛞️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_air_loop {
    #[path = "🥀️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🥀️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🥀️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🥀️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod remove_air_loop_terminal_zone {
    #[path = "🪹️remove-terminal-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪹️remove-terminal-zone/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🪹️remove-terminal-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪹️remove-terminal-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod add_air_loop_terminal_zone {
    #[path = "🪺️add-terminal-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪺️add-terminal-zone/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🪺️add-terminal-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪺️add-terminal-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use rename_air_loop::{rename_air_loop, RenameAirLoop};
pub use create_air_loop::{create_air_loop, CreateAirLoop};
pub use delete_air_loop::{delete_air_loop, DeleteAirLoop};
pub use remove_air_loop_terminal_zone::{remove_air_loop_terminal_zone, RemoveAirLoopTerminalZone};
pub use add_air_loop_terminal_zone::{add_air_loop_terminal_zone, AddAirLoopTerminalZone};
