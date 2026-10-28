//! 📏️ Energy model mutation domain `sizing`: `change-sizing-object-design-day-type`, `change-sizing-object-zone`, `create-sizing-object`, `change-sizing-object-sizing-type`, `delete-sizing-object`.

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
pub mod change_sizing_object_design_day_type {
    #[path = "🌥️change-design-day-type/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌥️change-design-day-type/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌥️change-design-day-type/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌥️change-design-day-type/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_sizing_object_zone {
    #[path = "🏨️change-zone/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏨️change-zone/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🏨️change-zone/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏨️change-zone/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_sizing_object {
    #[path = "📶️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "📶️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "📶️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📶️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_sizing_object_sizing_type {
    #[path = "🧾️change-sizing-type/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧾️change-sizing-type/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🧾️change-sizing-type/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧾️change-sizing-type/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_sizing_object {
    #[path = "🪒️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪒️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🪒️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪒️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_sizing_object_design_day_type::{change_sizing_object_design_day_type, ChangeSizingObjectDesignDayType};
pub use change_sizing_object_zone::{change_sizing_object_zone, ChangeSizingObjectZone};
pub use create_sizing_object::{create_sizing_object, CreateSizingObject};
pub use change_sizing_object_sizing_type::{change_sizing_object_sizing_type, ChangeSizingObjectSizingType};
pub use delete_sizing_object::{delete_sizing_object, DeleteSizingObject};
