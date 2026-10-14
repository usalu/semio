//! 🏗️ Energy model mutation domain `construction`: `add-construction-layer`, `remove-construction-layer`, `create-construction`, `reorder-construction-layers`, `delete-construction`, `rename-construction`.

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
pub mod add_construction_layer {
    #[path = "➕️add-layer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "➕️add-layer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "➕️add-layer/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod remove_construction_layer {
    #[path = "➖️remove-layer/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "➖️remove-layer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "➖️remove-layer/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_construction {
    #[path = "🏗️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏗️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏗️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod reorder_construction_layers {
    #[path = "🔀️reorder-layers/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔀️reorder-layers/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔀️reorder-layers/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_construction {
    #[path = "🧨️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧨️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧨️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod rename_construction {
    #[path = "🪪️rename/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪪️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪪️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use add_construction_layer::{add_construction_layer, AddConstructionLayer};
pub use remove_construction_layer::{remove_construction_layer, RemoveConstructionLayer};
pub use create_construction::{create_construction, CreateConstruction};
pub use reorder_construction_layers::{reorder_construction_layers, ReorderConstructionLayers};
pub use delete_construction::{delete_construction, DeleteConstruction};
pub use rename_construction::{rename_construction, RenameConstruction};
