//! 📋️ Energy model mutation domain `space-list`: `add-space-list-member`, `create-space-list`, `delete-space-list`, `remove-space-list-member`, `rename-space-list`.

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
pub mod add_space_list_member {
    #[path = "➡️add-member/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "➡️add-member/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "➡️add-member/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "➡️add-member/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod create_space_list {
    #[path = "📋️create/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "📋️create/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "📋️create/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📋️create/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod delete_space_list {
    #[path = "🗒️delete/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🗒️delete/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🗒️delete/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🗒️delete/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod remove_space_list_member {
    #[path = "🪤️remove-member/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪤️remove-member/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🪤️remove-member/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪤️remove-member/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

#[path = "."]
pub mod rename_space_list {
    #[path = "🪶️rename/🦀️.rs"]
    mod component;
    pub use component::*;
    #[path = "🪶️rename/🧊️reference/🦀️.rs"]
    mod reference;
    #[path = "🪶️rename/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪶️rename/↩️inverse/🦀️.rs"]
    pub mod inverse;
}

pub use add_space_list_member::{add_space_list_member, AddSpaceListMember};
pub use create_space_list::{create_space_list, CreateSpaceList};
pub use delete_space_list::{delete_space_list, DeleteSpaceList};
pub use remove_space_list_member::{remove_space_list_member, RemoveSpaceListMember};
pub use rename_space_list::{rename_space_list, RenameSpaceList};
