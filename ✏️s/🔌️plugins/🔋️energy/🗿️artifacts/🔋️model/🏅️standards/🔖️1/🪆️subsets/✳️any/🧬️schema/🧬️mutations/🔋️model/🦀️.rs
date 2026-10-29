//! 🔋️ Energy model mutation domain `model`: `disconnect-referenced-model`, `unbind-weather-file`, `bind-weather-file`, `rename-model`, `change-model-version`, `connect-referenced-model`

#![allow(async_fn_in_trait)]
#![allow(clippy::too_many_arguments)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_diff_patch as diff_patch;
extern crate semio_s_plugin_energy_engine_model as model;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;

use crate::diff::ReferenceEnergyModelDiff as EnergyModelDiff;
use crate::snapshot::reference::EnergyModelSnapshot;

//#region 🔖️LinkRoles
/// 🌦️ The `weather_link` slot's role name — one constant shared by the bind/unbind pair.
pub const WEATHER_LINK_ROLE: &str = "weather";
/// 🪢️ The `referenced_model` slot's role name — one constant shared by the connect/disconnect pair.
pub const REFERENCED_MODEL_LINK_ROLE: &str = "model";
//#endregion 🔖️LinkRoles

#[path = "."]
pub mod disconnect_referenced_model {
    #[path = "✂️disconnect-referenced-model/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "✂️disconnect-referenced-model/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "✂️disconnect-referenced-model/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "✂️disconnect-referenced-model/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod unbind_weather_file {
    #[path = "🌤️unbind-weather-file/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌤️unbind-weather-file/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌤️unbind-weather-file/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌤️unbind-weather-file/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod bind_weather_file {
    #[path = "🌦️bind-weather-file/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🌦️bind-weather-file/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🌦️bind-weather-file/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌦️bind-weather-file/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod rename_model {
    #[path = "🏷️rename/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🏷️rename/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🏷️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏷️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod change_model_version {
    #[path = "🔢️change-version/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🔢️change-version/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🔢️change-version/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔢️change-version/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod connect_referenced_model {
    #[path = "🪢️connect-referenced-model/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪢️connect-referenced-model/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🪢️connect-referenced-model/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪢️connect-referenced-model/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use disconnect_referenced_model::{disconnect_referenced_model, DisconnectReferencedModel};
pub use unbind_weather_file::{unbind_weather_file, UnbindWeatherFile};
pub use bind_weather_file::{bind_weather_file, BindWeatherFile};
pub use rename_model::{rename_model, RenameModel};
pub use change_model_version::{change_model_version, ChangeModelVersion};
pub use connect_referenced_model::{connect_referenced_model, ConnectReferencedModel};

#[path="🧳️retained/🦀️.rs"]pub mod retained;

#[cfg(test)]#[global_allocator]
static ORIGINAL_MODEL_MUTATION_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
