//! 🖼️ Puzzle3d reference mutations — creating, deleting, placing, resizing, re-sourcing and flagging reference images.

#![allow(clippy::result_large_err)]


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

//#region 🌳️Modules
#[path = "."]
pub mod change_reference_hidden {
    #[path = "👀change-reference-hidden/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "👀change-reference-hidden/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "👀change-reference-hidden/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod change_reference_locked {
    #[path = "🗝️change-reference-locked/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🗝️change-reference-locked/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🗝️change-reference-locked/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod create_reference {
    #[path = "🖼️create-reference/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🖼️create-reference/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🖼️create-reference/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod delete_reference {
    #[path = "🚮delete-reference/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🚮delete-reference/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🚮delete-reference/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod move_reference {
    #[path = "🎯move-reference/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🎯move-reference/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🎯move-reference/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod replace_reference_source {
    #[path = "🖇️replace-reference-source/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🖇️replace-reference-source/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🖇️replace-reference-source/🦀️.rs"]
    pub mod mutation;
}
#[path = "."]
pub mod resize_reference {
    #[path = "📎resize-reference/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "📎resize-reference/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "📎resize-reference/🦀️.rs"]
    pub mod mutation;
}
//#endregion 🌳️Modules
