//! 🧊️ Puzzle3d object mutations — creating and deleting a placed object and editing its kind, anchor, mesh, label,
//! visibility and lock.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

//#region 🌳️Modules
#[path = "."]
pub mod change_object_anchor {
    #[path = "⚓change-object-anchor/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "⚓change-object-anchor/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "⚓change-object-anchor/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod change_object_hidden {
    #[path = "👁️change-object-hidden/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "👁️change-object-hidden/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "👁️change-object-hidden/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod change_object_kind {
    #[path = "🏗️change-object-kind/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🏗️change-object-kind/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🏗️change-object-kind/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod change_object_locked {
    #[path = "🔒change-object-locked/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔒change-object-locked/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🔒change-object-locked/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod change_object_mesh {
    #[path = "🧱change-object-mesh/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧱change-object-mesh/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧱change-object-mesh/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod create_object {
    #[path = "🌱create-object/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌱create-object/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🌱create-object/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod delete_object {
    #[path = "🗑️delete-object/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🗑️delete-object/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🗑️delete-object/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod edit_object_label {
    #[path = "🖋️edit-object-label/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🖋️edit-object-label/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🖋️edit-object-label/🦀️.rs"]
    pub mod mutation;
}
//#endregion 🌳️Modules
