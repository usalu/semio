//! 🌍️ Puzzle3d target volume mutations — creating, deleting, posing and flagging the volumes a fill run targets.

#![allow(clippy::result_large_err)]


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

//#region 🌳️Modules
#[path = "."]
pub mod change_target_volume_hidden {
    #[path = "🙈change-target-volume-hidden/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🙈change-target-volume-hidden/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🙈change-target-volume-hidden/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod change_target_volume_locked {
    #[path = "🔐change-target-volume-locked/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🔐change-target-volume-locked/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🔐change-target-volume-locked/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod create_target_volume {
    #[path = "🌍create-target-volume/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌍create-target-volume/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🌍create-target-volume/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod delete_target_volume {
    #[path = "🪦delete-target-volume/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🪦delete-target-volume/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🪦delete-target-volume/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod move_target_volume {
    #[path = "🚀move-target-volume/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🚀move-target-volume/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🚀move-target-volume/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod rotate_target_volume {
    #[path = "🌀rotate-target-volume/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🌀rotate-target-volume/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🌀rotate-target-volume/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod scale_target_volume {
    #[path = "📐scale-target-volume/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📐scale-target-volume/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "📐scale-target-volume/🦀️.rs"]
    pub mod mutation;
}
//#endregion 🌳️Modules
