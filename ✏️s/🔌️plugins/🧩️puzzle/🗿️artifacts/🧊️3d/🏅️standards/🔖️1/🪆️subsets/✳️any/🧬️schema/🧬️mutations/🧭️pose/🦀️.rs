//! 🧭️ Puzzle3d pose mutations — the absolute move, rotation and scale of one placed object.

#![allow(clippy::result_large_err)]


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

//#region 🌳️Modules
#[path = "."]
pub mod move_object {
    #[path = "📍move-object/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📍move-object/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "📍move-object/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod rotate_object {
    #[path = "🔃rotate-object/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔃rotate-object/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🔃rotate-object/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod scale_object {
    #[path = "📏scale-object/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📏scale-object/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "📏scale-object/🦀️.rs"]
    pub mod mutation;
}
//#endregion 🌳️Modules
