//! 🛏️ Energy model mutation domain `room-air`: `create-room-air-model-assignment`, `delete-room-air-model-assignment`, `change-room-air-model`.

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
pub mod create_room_air_model_assignment {
    #[path = "🛏️create-assignment/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🛏️create-assignment/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🛏️create-assignment/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🛏️create-assignment/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_room_air_model_assignment {
    #[path = "🧺️delete-assignment/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧺️delete-assignment/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🧺️delete-assignment/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧺️delete-assignment/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_room_air_model {
    #[path = "🪭️change/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪭️change/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🪭️change/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪭️change/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use create_room_air_model_assignment::{create_room_air_model_assignment, CreateRoomAirModelAssignment};
pub use delete_room_air_model_assignment::{delete_room_air_model_assignment, DeleteRoomAirModelAssignment};
pub use change_room_air_model::{change_room_air_model, ChangeRoomAirModel};
