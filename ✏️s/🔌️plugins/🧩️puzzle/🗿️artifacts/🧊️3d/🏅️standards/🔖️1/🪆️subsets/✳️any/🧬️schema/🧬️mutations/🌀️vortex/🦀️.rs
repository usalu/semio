//! 🌀️ Puzzle3d vortex mutations — an object's vortex membership and the attractions that connect two vortices.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

//#region 🌳️Modules
#[path = "."]
pub mod add_object_vortex {
    #[path = "➕add-object-vortex/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "➕add-object-vortex/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "➕add-object-vortex/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod connect_vortices {
    #[path = "🪢️connect-vortices/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪢️connect-vortices/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🪢️connect-vortices/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod disconnect_vortices {
    #[path = "✂️disconnect-vortices/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "✂️disconnect-vortices/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "✂️disconnect-vortices/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod remove_object_vortex {
    #[path = "➖remove-object-vortex/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "➖remove-object-vortex/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "➖remove-object-vortex/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod replace_attraction_geometry {
    #[path = "🧮replace-attraction-geometry/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧮replace-attraction-geometry/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧮replace-attraction-geometry/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod replace_object_vortex {
    #[path = "🔌replace-object-vortex/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔌replace-object-vortex/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🔌replace-object-vortex/🦀️.rs"]
    pub mod mutation;
}
//#endregion 🌳️Modules
