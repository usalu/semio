//! 🏡️ Energy model mutation domain `zone`: `change-zone-multiplier`, `change-zone-conditioned`, `create-zone`, `delete-zone`, `rename-zone`, `change-zone-floor-area-participation`, `change-zone-volume`.

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
pub mod change_zone_multiplier {
    #[path = "✖️change-multiplier/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "✖️change-multiplier/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "✖️change-multiplier/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "✖️change-multiplier/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_zone_conditioned {
    #[path = "🌬️change-conditioned/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌬️change-conditioned/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌬️change-conditioned/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌬️change-conditioned/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_zone {
    #[path = "🏘️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏘️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🏘️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏘️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_zone {
    #[path = "🏚️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏚️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🏚️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏚️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod rename_zone {
    #[path = "🏠️rename/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏠️rename/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🏠️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏠️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_zone_floor_area_participation {
    #[path = "📐️change-floor-area-participation/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "📐️change-floor-area-participation/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "📐️change-floor-area-participation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📐️change-floor-area-participation/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_zone_volume {
    #[path = "🧊️change-volume/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🧊️change-volume/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🧊️change-volume/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧊️change-volume/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use change_zone_multiplier::{change_zone_multiplier, ChangeZoneMultiplier};
pub use change_zone_conditioned::{change_zone_conditioned, ChangeZoneConditioned};
pub use create_zone::{create_zone, CreateZone};
pub use delete_zone::{delete_zone, DeleteZone};
pub use rename_zone::{rename_zone, RenameZone};
pub use change_zone_floor_area_participation::{change_zone_floor_area_participation, ChangeZoneFloorAreaParticipation};
pub use change_zone_volume::{change_zone_volume, ChangeZoneVolume};

#[path="🧳️retained/🦀️.rs"]pub mod retained;
#[cfg(test)]#[global_allocator]
static ORIGINAL_ZONE_MUTATION_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
