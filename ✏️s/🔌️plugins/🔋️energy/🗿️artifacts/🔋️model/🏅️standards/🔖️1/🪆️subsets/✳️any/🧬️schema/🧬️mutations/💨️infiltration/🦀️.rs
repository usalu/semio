//! 💨️ Energy model mutation domain `infiltration`: `change-infiltration-schedule`, `change-infiltration-zone`, `change-infiltration-flow-per-exterior-area`, `create-infiltration`, `change-infiltration-design-flow-ach`, `change-infiltration-method`, `delete-infiltration`.

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_s_plugin_energy_engine_air_exchange as air_exchange;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_plugin_energy_engine_schedule as schedule;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

#[path = "."]
pub mod change_infiltration_schedule {
    #[path = "⏳️change-schedule/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "⏳️change-schedule/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "⏳️change-schedule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⏳️change-schedule/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_infiltration_zone {
    #[path = "🌀️change-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌀️change-zone/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌀️change-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌀️change-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_infiltration_flow_per_exterior_area {
    #[path = "🌫️change-flow-per-exterior-area/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌫️change-flow-per-exterior-area/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌫️change-flow-per-exterior-area/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌫️change-flow-per-exterior-area/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_infiltration {
    #[path = "💨️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "💨️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "💨️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "💨️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_infiltration_design_flow_ach {
    #[path = "🔄️change-design-flow-ach/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔄️change-design-flow-ach/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔄️change-design-flow-ach/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔄️change-design-flow-ach/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_infiltration_method {
    #[path = "🔬️change-method/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔬️change-method/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔬️change-method/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔬️change-method/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_infiltration {
    #[path = "🧽️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧽️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🧽️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧽️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_infiltration_schedule::{change_infiltration_schedule, ChangeInfiltrationSchedule};
pub use change_infiltration_zone::{change_infiltration_zone, ChangeInfiltrationZone};
pub use change_infiltration_flow_per_exterior_area::{change_infiltration_flow_per_exterior_area, ChangeInfiltrationFlowPerExteriorArea};
pub use create_infiltration::{create_infiltration, CreateInfiltration};
pub use change_infiltration_design_flow_ach::{change_infiltration_design_flow_ach, ChangeInfiltrationDesignFlowAch};
pub use change_infiltration_method::{change_infiltration_method, ChangeInfiltrationMethod};
pub use delete_infiltration::{delete_infiltration, DeleteInfiltration};
