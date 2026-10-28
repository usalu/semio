//! 🎚️ Energy model mutation domain `setpoint`: `delete-setpoint-manager`, `change-setpoint-manager-schedule`, `create-setpoint-manager`, `replace-setpoint-manager-kind`, `rename-setpoint-manager`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod delete_setpoint_manager {
    #[path = "🍄️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🍄️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🍄️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🍄️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_setpoint_manager_schedule {
    #[path = "🎼️change-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🎼️change-schedule/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🎼️change-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🎼️change-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_setpoint_manager {
    #[path = "📌️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "📌️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "📌️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📌️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod replace_setpoint_manager_kind {
    #[path = "🔃️replace-kind/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔃️replace-kind/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔃️replace-kind/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔃️replace-kind/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod rename_setpoint_manager {
    #[path = "🖇️rename/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🖇️rename/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🖇️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🖇️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use delete_setpoint_manager::{delete_setpoint_manager, DeleteSetpointManager};
pub use change_setpoint_manager_schedule::{change_setpoint_manager_schedule, ChangeSetpointManagerSchedule};
pub use create_setpoint_manager::{create_setpoint_manager, CreateSetpointManager};
pub use replace_setpoint_manager_kind::{replace_setpoint_manager_kind, ReplaceSetpointManagerKind};
pub use rename_setpoint_manager::{rename_setpoint_manager, RenameSetpointManager};
